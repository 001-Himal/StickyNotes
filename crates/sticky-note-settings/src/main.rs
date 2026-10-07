slint::include_modules!();

use std::cell::RefCell;
use std::path::PathBuf;
use std::rc::Rc;
use sticky_note_core::{
    load_config_or_default, save_config, send_ipc_command, set_autostart_registry,
    CloseAction, CornerStyle, IpcCommand, NoteColor,
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config_path = PathBuf::from("config.json");
    let config = load_config_or_default(&config_path);
    let config_state = Rc::new(RefCell::new(config));
    let config_path_state = Rc::new(config_path);

    let window = SettingsWindow::new()?;

    // Populate initial state from config
    {
        let cfg = config_state.borrow();
        window.set_close_action(match cfg.behavior.close_action {
            CloseAction::Ask => "ask".into(),
            CloseAction::Close => "close".into(),
            CloseAction::Delete => "delete".into(),
        });
        window.set_corner_style(match cfg.note_appearance.corner_style {
            CornerStyle::Curled => "curled".into(),
            CornerStyle::Flat => "flat".into(),
        });
        window.set_default_color(match cfg.note_appearance.default_color {
            NoteColor::Blue => "blue".into(),
            NoteColor::Green => "green".into(),
            NoteColor::Pink => "pink".into(),
            NoteColor::Purple => "purple".into(),
            NoteColor::White => "white".into(),
            NoteColor::Yellow => "yellow".into(),
        });
        window.set_start_with_os(cfg.general.start_with_os);
        window.set_shortcut_new_note(cfg.shortcuts.new_note.clone().into());
        window.set_shortcut_settings(cfg.shortcuts.open_settings.clone().into());
        window.set_font_size_pt(cfg.note_appearance.font_size as i32);
    }

    // Helper to persist and notify running Sticky Note instance via IPC
    let persist_and_notify = {
        let config_state = config_state.clone();
        let config_path_state = config_path_state.clone();
        Rc::new(move || {
            let cfg = config_state.borrow();
            let _ = save_config(&config_path_state, &cfg);
            let _ = send_ipc_command(&IpcCommand::ReloadConfig);
        })
    };

    // Close action changed
    {
        let config_state = config_state.clone();
        let persist = persist_and_notify.clone();
        window.on_close_action_changed(move |action| {
            config_state.borrow_mut().behavior.close_action = match action.as_str() {
                "close" => CloseAction::Close,
                "delete" => CloseAction::Delete,
                _ => CloseAction::Ask,
            };
            persist();
        });
    }

    // Corner style changed
    {
        let config_state = config_state.clone();
        let persist = persist_and_notify.clone();
        window.on_corner_style_changed(move |style| {
            config_state.borrow_mut().note_appearance.corner_style = match style.as_str() {
                "flat" => CornerStyle::Flat,
                _ => CornerStyle::Curled,
            };
            persist();
        });
    }

    // Default color changed
    {
        let config_state = config_state.clone();
        let persist = persist_and_notify.clone();
        window.on_default_color_changed(move |color| {
            config_state.borrow_mut().note_appearance.default_color = match color.as_str() {
                "blue" => NoteColor::Blue,
                "green" => NoteColor::Green,
                "pink" => NoteColor::Pink,
                "purple" => NoteColor::Purple,
                "white" => NoteColor::White,
                _ => NoteColor::Yellow,
            };
            persist();
        });
    }

    // Start with OS toggled
    {
        let config_state = config_state.clone();
        let persist = persist_and_notify.clone();
        window.on_start_with_os_changed(move |enabled| {
            config_state.borrow_mut().general.start_with_os = enabled;
            let _ = set_autostart_registry(enabled, None);
            persist();
        });
    }

    // New Note shortcut changed
    {
        let config_state = config_state.clone();
        let persist = persist_and_notify.clone();
        window.on_shortcut_new_note_changed(move |sc| {
            let trimmed = sc.trim().to_string();
            if !trimmed.is_empty() {
                config_state.borrow_mut().shortcuts.new_note = trimmed;
                persist();
            }
        });
    }

    // Settings shortcut changed
    {
        let config_state = config_state.clone();
        let persist = persist_and_notify.clone();
        window.on_shortcut_settings_changed(move |sc| {
            let trimmed = sc.trim().to_string();
            if !trimmed.is_empty() {
                config_state.borrow_mut().shortcuts.open_settings = trimmed;
                persist();
            }
        });
    }

    // Font size changed
    {
        let config_state = config_state.clone();
        let persist = persist_and_notify.clone();
        window.on_font_size_changed(move |size| {
            if (10..=28).contains(&size) {
                config_state.borrow_mut().note_appearance.font_size = size as u32;
                persist();
            }
        });
    }

    // Window header dragging
    {
        let window_weak = window.as_weak();
        window.on_window_drag_delta(move |dx, dy| {
            if let Some(w) = window_weak.upgrade() {
                let current_pos = w.window().position();
                let new_x = current_pos.x + dx as i32;
                let new_y = current_pos.y + dy as i32;
                w.window().set_position(slint::PhysicalPosition::new(new_x, new_y));
            }
        });
    }

    // Close button clicked: exit immediately, freeing all memory
    {
        window.on_close_clicked(move || {
            let _ = slint::quit_event_loop();
        });
    }

    window.show()?;
    slint::run_event_loop()?;
    Ok(())
}
