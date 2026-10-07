#![windows_subsystem = "windows"]
slint::include_modules!();

mod hotkeys;

#[cfg(windows)]
use sticky_note_core::{clamp_to_monitor_bounds, stealth_and_sink_all_process_windows};
use sticky_note_core::{
    apply_text_formatting, current_timestamp, generate_next_title, load_config_or_default,
    load_json, resolve_title, send_ipc_command, update_default_note_size, CloseAction, Config,
    CornerStyle, IpcCommand, IpcServer, Note, NoteColor, NotesDocument, StorageEngine,
};
use std::cell::RefCell;
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::rc::Rc;
use std::time::{SystemTime, UNIX_EPOCH};

fn hex_to_color(hex: &str) -> slint::Color {
    let hex = hex.trim_start_matches('#');
    if hex.len() == 6 {
        let r = u8::from_str_radix(&hex[0..2], 16).unwrap_or(0);
        let g = u8::from_str_radix(&hex[2..4], 16).unwrap_or(0);
        let b = u8::from_str_radix(&hex[4..6], 16).unwrap_or(0);
        slint::Color::from_rgb_u8(r, g, b)
    } else {
        slint::Color::from_rgb_u8(253, 241, 176)
    }
}

pub fn apply_theme(window: &NoteWindow, color: NoteColor) {
    let (header, body, text, accent) = color.hex_codes();
    window.set_header_bg(hex_to_color(header));
    window.set_body_bg(hex_to_color(body));
    window.set_text_color(hex_to_color(text));
    window.set_accent_color(hex_to_color(accent));
}

struct NoteSession {
    note: Note,
    window: NoteWindow,
    autosave_timer: slint::Timer,
}

struct AppContext {
    storage: StorageEngine,
    doc: NotesDocument,
    config: Config,
    config_path: PathBuf,
    sessions: HashMap<String, NoteSession>,
}

thread_local! {
    static APP: RefCell<Option<AppContext>> = const { RefCell::new(None) };
}

fn delete_note(app: &mut AppContext, note_id: &str) {
    let AppContext {
        ref storage,
        ref mut doc,
        ref mut sessions,
        ..
    } = *app;

    let _ = storage.delete_note(note_id, doc);
    if let Some(session) = sessions.remove(note_id) {
        let _ = session.window.hide();
    }
}

fn close_note(app: &mut AppContext, note_id: &str) {
    let AppContext {
        ref storage,
        ref mut doc,
        ref mut sessions,
        ..
    } = *app;

    let _ = storage.set_note_closed(note_id, true, doc);
    if let Some(session) = sessions.remove(note_id) {
        let _ = session.window.hide();
    }
}

fn create_and_show_note_window(
    app: &mut AppContext,
    note: Note,
    corner_style: CornerStyle,
) -> Result<(), Box<dyn std::error::Error>> {
    let window = NoteWindow::new()?;
    let note_id = note.id.clone();

    window.set_note_id(note_id.clone().into());
    window.set_note_title(note.title.clone().into());
    window.set_note_content(note.content.clone().into());
    window.set_corner_style(match corner_style {
        CornerStyle::Curled => "curled".into(),
        CornerStyle::Flat => "flat".into(),
    });
    apply_theme(&window, note.color);

    #[cfg(windows)]
    let (clamped_x, clamped_y) =
        unsafe { clamp_to_monitor_bounds(note.x, note.y, note.width, note.height) };
    #[cfg(not(windows))]
    let (clamped_x, clamped_y) = (note.x, note.y);

    window
        .window()
        .set_position(slint::PhysicalPosition::new(clamped_x, clamped_y));
    window
        .window()
        .set_size(slint::PhysicalSize::new(note.width, note.height));

    let debounce_ms = app.config.behavior.autosave_debounce_ms;

    let schedule_save = {
        let note_id_for_save = note_id.clone();
        Rc::new(move || {
            let note_id = note_id_for_save.clone();
            APP.with(|app_cell| {
                if let Some(ref mut app) = *app_cell.borrow_mut() {
                    if let Some(session) = app.sessions.get_mut(&note_id) {
                        session.autosave_timer.start(
                            slint::TimerMode::SingleShot,
                            std::time::Duration::from_millis(debounce_ms),
                            move || {
                                APP.with(|app_cell| {
                                    if let Some(ref mut app) = *app_cell.borrow_mut() {
                                        let AppContext {
                                            ref storage,
                                            ref mut doc,
                                            ref mut sessions,
                                            ..
                                        } = *app;
                                        if let Some(session) = sessions.get_mut(&note_id) {
                                            let _ = storage.save_note(&mut session.note, doc);
                                        }
                                    }
                                });
                            },
                        );
                    }
                }
            });
        })
    };

    // Quick-Add '+' button click
    {
        let note_id_for_add = note_id.clone();
        window.on_add_note_clicked(move || {
            APP.with(|app_cell| {
                if let Some(ref mut app) = *app_cell.borrow_mut() {
                    let _ = spawn_note(app, Some(&note_id_for_add));
                }
            });
        });
    }

    // Close 'X' button click (configurable: delete, close, ask)
    {
        let note_id_for_close = note_id.clone();
        let window_weak = window.as_weak();

        window.on_close_note_clicked(move || {
            let action = APP.with(|app_cell| {
                app_cell
                    .borrow()
                    .as_ref()
                    .map(|a| a.config.behavior.close_action)
                    .unwrap_or(CloseAction::Ask)
            });

            match action {
                CloseAction::Delete => {
                    APP.with(|app_cell| {
                        if let Some(ref mut app) = *app_cell.borrow_mut() {
                            delete_note(app, &note_id_for_close);
                        }
                    });
                }
                CloseAction::Close => {
                    APP.with(|app_cell| {
                        if let Some(ref mut app) = *app_cell.borrow_mut() {
                            close_note(app, &note_id_for_close);
                        }
                    });
                }
                CloseAction::Ask => {
                    if let Some(w) = window_weak.upgrade() {
                        w.set_show_ask_dialog(true);
                    }
                }
            }
        });
    }

    // Ask modal action selected ("delete", "close", "cancel")
    {
        let note_id_for_ask = note_id.clone();
        let window_weak = window.as_weak();

        window.on_ask_action_selected(move |action_str| match action_str.as_str() {
            "delete" => {
                APP.with(|app_cell| {
                    if let Some(ref mut app) = *app_cell.borrow_mut() {
                        delete_note(app, &note_id_for_ask);
                    }
                });
            }
            "close" => {
                APP.with(|app_cell| {
                    if let Some(ref mut app) = *app_cell.borrow_mut() {
                        close_note(app, &note_id_for_ask);
                    }
                });
            }
            _ => {
                if let Some(w) = window_weak.upgrade() {
                    w.set_show_ask_dialog(false);
                }
            }
        });
    }

    // Color palette selection
    {
        let window_weak = window.as_weak();
        let note_id_for_color = note_id.clone();
        let schedule_save = schedule_save.clone();

        window.on_color_selected(move |color_str| {
            if let Some(w) = window_weak.upgrade() {
                let color = match color_str.as_str() {
                    "blue" => NoteColor::Blue,
                    "green" => NoteColor::Green,
                    "pink" => NoteColor::Pink,
                    "purple" => NoteColor::Purple,
                    "white" => NoteColor::White,
                    _ => NoteColor::Yellow,
                };
                w.set_color_theme(color_str);
                apply_theme(&w, color);

                APP.with(|app_cell| {
                    if let Some(ref mut app) = *app_cell.borrow_mut() {
                        if let Some(session) = app.sessions.get_mut(&note_id_for_color) {
                            session.note.color = color;
                        }
                    }
                });
                schedule_save();
            }
        });
    }

    // Title editing and rename fallback
    {
        let window_weak = window.as_weak();
        let note_id_for_title = note_id.clone();

        window.on_title_changed(move |new_title| {
            APP.with(|app_cell| {
                if let Some(ref mut app) = *app_cell.borrow_mut() {
                    let AppContext {
                        ref storage,
                        ref mut doc,
                        ref mut sessions,
                        ..
                    } = *app;

                    if let Some(session) = sessions.get_mut(&note_id_for_title) {
                        let fallback = session.note.title.clone();
                        let safe_title = resolve_title(new_title.as_str(), &fallback);

                        if let Some(w) = window_weak.upgrade() {
                            w.set_note_title(safe_title.clone().into());
                        }

                        if session.note.title != safe_title {
                            session.note.title = safe_title;
                            let _ = storage.save_note(&mut session.note, doc);
                        }
                    }
                }
            });
        });
    }

    // Content editing auto-save
    {
        let note_id_for_content = note_id.clone();
        let schedule_save = schedule_save.clone();

        window.on_content_changed(move |content| {
            APP.with(|app_cell| {
                if let Some(ref mut app) = *app_cell.borrow_mut() {
                    if let Some(session) = app.sessions.get_mut(&note_id_for_content) {
                        session.note.content = content.to_string();
                    }
                }
            });
            schedule_save();
        });
    }

    // Keyboard formatting wrappers
    {
        let window_weak = window.as_weak();
        let note_id_for_format = note_id.clone();
        let schedule_save = schedule_save.clone();

        window.on_format_requested(move |fmt| {
            if let Some(w) = window_weak.upgrade() {
                let current = w.get_note_content();
                let formatted = apply_text_formatting(current.as_str(), fmt.as_str());
                w.set_note_content(formatted.clone().into());

                APP.with(|app_cell| {
                    if let Some(ref mut app) = *app_cell.borrow_mut() {
                        if let Some(session) = app.sessions.get_mut(&note_id_for_format) {
                            session.note.content = formatted;
                        }
                    }
                });
                schedule_save();
            }
        });
    }

    // Window header dragging
    {
        let window_weak = window.as_weak();
        let note_id_for_drag = note_id.clone();

        window.on_window_drag_delta(move |dx, dy| {
            if let Some(w) = window_weak.upgrade() {
                let current_pos = w.window().position();
                let new_x = current_pos.x + dx as i32;
                let new_y = current_pos.y + dy as i32;
                w.window()
                    .set_position(slint::PhysicalPosition::new(new_x, new_y));

                APP.with(|app_cell| {
                    if let Some(ref mut app) = *app_cell.borrow_mut() {
                        if let Some(session) = app.sessions.get_mut(&note_id_for_drag) {
                            session.note.x = new_x;
                            session.note.y = new_y;
                        }
                    }
                });
            }
        });
    }

    // Window drag completed: clamp and save geometry
    {
        let window_weak = window.as_weak();
        let note_id_for_moved = note_id.clone();
        let schedule_save = schedule_save.clone();

        window.on_window_moved_completed(move || {
            if let Some(w) = window_weak.upgrade() {
                let pos = w.window().position();
                APP.with(|app_cell| {
                    if let Some(ref mut app) = *app_cell.borrow_mut() {
                        if let Some(session) = app.sessions.get_mut(&note_id_for_moved) {
                            #[cfg(windows)]
                            let (clamped_x, clamped_y) = unsafe {
                                clamp_to_monitor_bounds(
                                    pos.x,
                                    pos.y,
                                    session.note.width,
                                    session.note.height,
                                )
                            };
                            #[cfg(not(windows))]
                            let (clamped_x, clamped_y) = (pos.x, pos.y);

                            w.window()
                                .set_position(slint::PhysicalPosition::new(clamped_x, clamped_y));
                            session.note.x = clamped_x;
                            session.note.y = clamped_y;

                            #[cfg(windows)]
                            unsafe {
                                stealth_and_sink_all_process_windows();
                            }
                        }
                    }
                });
                schedule_save();
            }
        });
    }

    // Single-direction bottom-right resizing (top-left anchored)
    {
        let window_weak = window.as_weak();
        let note_id_for_resize = note_id.clone();

        window.on_window_resize_delta(move |dw, dh| {
            if let Some(w) = window_weak.upgrade() {
                let current_size = w.window().size();
                let new_w = (current_size.width as i32 + dw as i32).max(180) as u32;
                let new_h = (current_size.height as i32 + dh as i32).max(120) as u32;
                w.window()
                    .set_size(slint::PhysicalSize::new(new_w, new_h));

                APP.with(|app_cell| {
                    if let Some(ref mut app) = *app_cell.borrow_mut() {
                        if let Some(session) = app.sessions.get_mut(&note_id_for_resize) {
                            session.note.width = new_w;
                            session.note.height = new_h;
                        }
                    }
                });
            }
        });
    }

    // Window resize completed: update config for dynamic size inheritance & save geometry
    {
        let window_weak = window.as_weak();
        let note_id_for_resized = note_id.clone();
        let schedule_save = schedule_save.clone();

        window.on_window_resize_completed(move || {
            if let Some(w) = window_weak.upgrade() {
                let size = w.window().size();
                APP.with(|app_cell| {
                    if let Some(ref mut app) = *app_cell.borrow_mut() {
                        if let Some(session) = app.sessions.get_mut(&note_id_for_resized) {
                            session.note.width = size.width;
                            session.note.height = size.height;
                        }
                        app.config.note_appearance.last_used_width = size.width;
                        app.config.note_appearance.last_used_height = size.height;
                        let _ = update_default_note_size(
                            &app.config_path,
                            size.width,
                            size.height,
                        );
                    }
                });
                schedule_save();
            }
        });
    }

    window.show()?;

    #[cfg(windows)]
    {
        unsafe {
            stealth_and_sink_all_process_windows();
        }
        slint::Timer::single_shot(std::time::Duration::from_millis(50), || {
            unsafe {
                stealth_and_sink_all_process_windows();
            }
        });
        slint::Timer::single_shot(std::time::Duration::from_millis(150), || {
            unsafe {
                stealth_and_sink_all_process_windows();
            }
        });
    }

    app.sessions.insert(
        note_id,
        NoteSession {
            note,
            window,
            autosave_timer: slint::Timer::default(),
        },
    );

    Ok(())
}

fn spawn_note(
    app: &mut AppContext,
    from_note_id: Option<&str>,
) -> Result<(), Box<dyn std::error::Error>> {
    let (parent_x, parent_y) = if let Some(id) = from_note_id {
        if let Some(session) = app.sessions.get(id) {
            (session.note.x, session.note.y)
        } else {
            (200, 200)
        }
    } else {
        (200, 200)
    };

    let last_w = app.config.note_appearance.last_used_width;
    let last_h = app.config.note_appearance.last_used_height;
    let default_color = app.config.note_appearance.default_color;
    let corner_style = app.config.note_appearance.corner_style;

    let existing_titles: Vec<String> = app.doc.notes.iter().map(|m| m.title.clone()).collect();
    let title_refs: Vec<&str> = existing_titles.iter().map(|s| s.as_str()).collect();
    let new_title = generate_next_title(&title_refs);

    let cascade_offset = 24;
    let new_x = parent_x + cascade_offset;
    let new_y = parent_y + cascade_offset;

    #[cfg(windows)]
    let (clamped_x, clamped_y) =
        unsafe { clamp_to_monitor_bounds(new_x, new_y, last_w, last_h) };
    #[cfg(not(windows))]
    let (clamped_x, clamped_y) = (new_x, new_y);

    let note_id = format!(
        "note-{}",
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_millis()
    );

    let mut new_note = Note {
        version: 1,
        id: note_id,
        title: new_title,
        content: String::new(),
        x: clamped_x,
        y: clamped_y,
        width: last_w,
        height: last_h,
        color: default_color,
        created_at: current_timestamp(),
        updated_at: current_timestamp(),
        is_closed: false,
    };

    {
        let AppContext {
            ref storage,
            ref mut doc,
            ..
        } = *app;
        storage.save_note(&mut new_note, doc)?;
    }

    create_and_show_note_window(app, new_note, corner_style)?;
    Ok(())
}

fn open_note_from_file(
    app: &mut AppContext,
    path_str: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let path = Path::new(path_str);
    if !path.exists() {
        return Ok(());
    }

    if let Ok(Some(mut note)) = load_json::<Note>(path) {
        if let Some(session) = app.sessions.get(&note.id) {
            let _ = session.window.show();
            return Ok(());
        }

        note.is_closed = false;
        let corner_style = app.config.note_appearance.corner_style;

        {
            let AppContext {
                ref storage,
                ref mut doc,
                ..
            } = *app;
            let _ = storage.set_note_closed(&note.id, false, doc);
        }

        create_and_show_note_window(app, note, corner_style)?;
    }

    Ok(())
}

fn reload_config(app: &mut AppContext) {
    let new_config = load_config_or_default(&app.config_path);
    app.config = new_config.clone();

    let corner_str = match new_config.note_appearance.corner_style {
        CornerStyle::Curled => "curled",
        CornerStyle::Flat => "flat",
    };

    for session in app.sessions.values() {
        session.window.set_corner_style(corner_str.into());
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = std::env::args().collect();
    let ipc_cmd = if args.len() > 1 && args[1].ends_with(".json") {
        IpcCommand::OpenFile(args[1].clone())
    } else {
        IpcCommand::NewNote
    };

    // Single-instance enforcement: if primary already running, deliver command and exit
    if let Ok(true) = send_ipc_command(&ipc_cmd) {
        return Ok(());
    }

    let storage = StorageEngine::default();
    let config_path = PathBuf::from("config.json");
    let config = load_config_or_default(&config_path);

    let (doc, notes) = storage.load_all()?;

    let mut app = AppContext {
        storage,
        doc,
        config: config.clone(),
        config_path,
        sessions: HashMap::new(),
    };

    // Bind IPC server and start listener background thread
    if let Ok(Some(server)) = IpcServer::bind() {
        std::thread::spawn(move || {
            let _ = server.accept_commands(move |cmd| {
                let _ = slint::invoke_from_event_loop(move || {
                    APP.with(|app_cell| {
                        if let Some(ref mut app) = *app_cell.borrow_mut() {
                            match cmd {
                                IpcCommand::NewNote => {
                                    let _ = spawn_note(app, None);
                                }
                                IpcCommand::OpenFile(path) => {
                                    let _ = open_note_from_file(app, &path);
                                }
                                IpcCommand::ReloadConfig => {
                                    reload_config(app);
                                }
                            }
                        }
                    });
                });
            });
        });
    }

    // Register system-wide global hotkeys (Ctrl+Alt+N for new note, Ctrl+Alt+S for settings)
    let _hotkey_service = hotkeys::HotkeyService::start(
        &config.shortcuts.new_note,
        &config.shortcuts.open_settings,
        || {
            let _ = slint::invoke_from_event_loop(|| {
                APP.with(|app_cell| {
                    if let Some(ref mut app) = *app_cell.borrow_mut() {
                        let _ = spawn_note(app, None);
                    }
                });
            });
        },
        || {
            hotkeys::launch_settings();
        },
    );

    let launched_file = if args.len() > 1 && args[1].ends_with(".json") {
        Some(args[1].clone())
    } else {
        None
    };

    if let Some(file_path) = launched_file {
        open_note_from_file(&mut app, &file_path)?;
    } else {
        let open_notes: Vec<Note> = notes.into_iter().filter(|n| !n.is_closed).collect();
        if open_notes.is_empty() {
            spawn_note(&mut app, None)?;
        } else {
            for note in open_notes {
                create_and_show_note_window(
                    &mut app,
                    note,
                    config.note_appearance.corner_style,
                )?;
            }
        }
    }

    APP.with(|app_cell| {
        *app_cell.borrow_mut() = Some(app);
    });

    slint::run_event_loop()?;
    Ok(())
}
