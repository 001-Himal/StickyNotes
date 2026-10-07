slint::include_modules!();

#[cfg(windows)]
use sticky_note_core::{apply_stealth_window_styles, find_window_by_title, sink_to_desktop_layer};
use sticky_note_core::{apply_text_formatting, NoteColor};

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

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let window = NoteWindow::new()?;
    apply_theme(&window, NoteColor::Yellow);

    let window_weak = window.as_weak();
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
        }
    });

    let window_weak = window.as_weak();
    window.on_format_requested(move |fmt| {
        if let Some(w) = window_weak.upgrade() {
            let current = w.get_note_content();
            let formatted = apply_text_formatting(current.as_str(), fmt.as_str());
            w.set_note_content(formatted.into());
        }
    });

    let window_weak = window.as_weak();
    window.on_title_changed(move |new_title| {
        if let Some(w) = window_weak.upgrade() {
            w.set_note_title(new_title);
        }
    });

    let window_weak = window.as_weak();
    window.on_close_note_clicked(move || {
        if let Some(w) = window_weak.upgrade() {
            let _ = w.hide();
        }
    });

    window.on_add_note_clicked(move || {
        println!("Add note requested");
    });

    window.on_start_window_drag(move || {
        // Wired to native OS window drag in TASK-005
    });

    window.on_start_window_resize(move || {
        // Wired to native OS window resize in TASK-005
    });

    window.on_ask_action_selected(move |_action| {
        // Handled in TASK-006 / TASK-008
    });

    window.on_content_changed(move |_content| {
        // Handled via debounced autosave in TASK-005
    });

    #[cfg(windows)]
    {
        // Apply stealth desktop styling to note window
        unsafe {
            if let Some(hwnd) = find_window_by_title("Note 1") {
                apply_stealth_window_styles(hwnd);
                sink_to_desktop_layer(hwnd);
            }
        }
    }

    Ok(())
}
