//! System-wide global hotkey manager for Sticky Note.

use global_hotkey::{
    hotkey::{Code, HotKey, Modifiers},
    GlobalHotKeyEvent, GlobalHotKeyManager, HotKeyState,
};
use std::path::Path;
use std::thread;

pub struct HotkeyService {
    _manager: GlobalHotKeyManager,
    #[allow(dead_code)]
    pub new_note_id: u32,
    #[allow(dead_code)]
    pub settings_id: u32,
}

/// Parses a string representation of a shortcut (e.g., "Ctrl+Alt+N") into a `HotKey`.
pub fn parse_hotkey_str(s: &str) -> Option<HotKey> {
    let mut modifiers = Modifiers::empty();
    let mut key_code = None;

    for part in s.split('+') {
        let trimmed = part.trim();
        match trimmed.to_lowercase().as_str() {
            "ctrl" | "control" => modifiers |= Modifiers::CONTROL,
            "alt" => modifiers |= Modifiers::ALT,
            "shift" => modifiers |= Modifiers::SHIFT,
            "cmd" | "super" | "win" | "meta" => modifiers |= Modifiers::SUPER,
            _ => {
                let code = match trimmed.to_uppercase().as_str() {
                    "A" => Code::KeyA,
                    "B" => Code::KeyB,
                    "C" => Code::KeyC,
                    "D" => Code::KeyD,
                    "E" => Code::KeyE,
                    "F" => Code::KeyF,
                    "G" => Code::KeyG,
                    "H" => Code::KeyH,
                    "I" => Code::KeyI,
                    "J" => Code::KeyJ,
                    "K" => Code::KeyK,
                    "L" => Code::KeyL,
                    "M" => Code::KeyM,
                    "N" => Code::KeyN,
                    "O" => Code::KeyO,
                    "P" => Code::KeyP,
                    "Q" => Code::KeyQ,
                    "R" => Code::KeyR,
                    "S" => Code::KeyS,
                    "T" => Code::KeyT,
                    "U" => Code::KeyU,
                    "V" => Code::KeyV,
                    "W" => Code::KeyW,
                    "X" => Code::KeyX,
                    "Y" => Code::KeyY,
                    "Z" => Code::KeyZ,
                    "0" => Code::Digit0,
                    "1" => Code::Digit1,
                    "2" => Code::Digit2,
                    "3" => Code::Digit3,
                    "4" => Code::Digit4,
                    "5" => Code::Digit5,
                    "6" => Code::Digit6,
                    "7" => Code::Digit7,
                    "8" => Code::Digit8,
                    "9" => Code::Digit9,
                    _ => return None,
                };
                key_code = Some(code);
            }
        }
    }

    key_code.map(|code| {
        let mods = if modifiers.is_empty() {
            None
        } else {
            Some(modifiers)
        };
        HotKey::new(mods, code)
    })
}

/// Spawns the decoupled `Sticky Note Settings` utility executable.
pub fn launch_settings() {
    let current_exe = std::env::current_exe().unwrap_or_default();
    let dir = current_exe.parent().unwrap_or_else(|| Path::new("."));

    let candidates = [
        dir.join("Sticky Note Settings.exe"),
        dir.join("sticky-note-settings.exe"),
        dir.join("Sticky Note Settings"),
        dir.join("sticky-note-settings"),
    ];

    if let Some(bin_path) = candidates.iter().find(|p| p.exists()) {
        let _ = std::process::Command::new(bin_path).spawn();
    } else {
        let _ = std::process::Command::new("sticky-note-settings").spawn();
    }
}

impl HotkeyService {
    /// Initialize global hotkeys from config and start listening in a background thread.
    pub fn start<FNewNote, FSettings>(
        new_note_shortcut: &str,
        settings_shortcut: &str,
        on_new_note: FNewNote,
        on_settings: FSettings,
    ) -> Option<Self>
    where
        FNewNote: Fn() + Send + 'static,
        FSettings: Fn() + Send + 'static,
    {
        let manager = match GlobalHotKeyManager::new() {
            Ok(m) => m,
            Err(e) => {
                eprintln!("Warning: Failed to initialize GlobalHotKeyManager ({e})");
                return None;
            }
        };

        let new_note_hotkey = parse_hotkey_str(new_note_shortcut).unwrap_or_else(|| {
            HotKey::new(Some(Modifiers::CONTROL | Modifiers::ALT), Code::KeyN)
        });

        let settings_hotkey = parse_hotkey_str(settings_shortcut).unwrap_or_else(|| {
            HotKey::new(Some(Modifiers::CONTROL | Modifiers::ALT), Code::KeyS)
        });

        let new_note_id = new_note_hotkey.id();
        let settings_id = settings_hotkey.id();

        if let Err(e) = manager.register(new_note_hotkey) {
            eprintln!("Warning: Failed to register new note hotkey ({new_note_shortcut}): {e}");
        }

        if let Err(e) = manager.register(settings_hotkey) {
            eprintln!("Warning: Failed to register settings hotkey ({settings_shortcut}): {e}");
        }

        let receiver = GlobalHotKeyEvent::receiver();

        thread::spawn(move || {
            while let Ok(event) = receiver.recv() {
                if event.state == HotKeyState::Pressed {
                    if event.id == new_note_id {
                        on_new_note();
                    } else if event.id == settings_id {
                        on_settings();
                    }
                }
            }
        });

        Some(Self {
            _manager: manager,
            new_note_id,
            settings_id,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_hotkey_str() {
        let hk1 = parse_hotkey_str("Ctrl+Alt+N").expect("Parse Ctrl+Alt+N");
        assert_eq!(hk1.key, Code::KeyN);
        assert_eq!(hk1.mods, Modifiers::CONTROL | Modifiers::ALT);

        let hk2 = parse_hotkey_str("Ctrl+Alt+S").expect("Parse Ctrl+Alt+S");
        assert_eq!(hk2.key, Code::KeyS);
        assert_eq!(hk2.mods, Modifiers::CONTROL | Modifiers::ALT);

        let hk3 = parse_hotkey_str("Cmd+Shift+A").expect("Parse Cmd+Shift+A");
        assert_eq!(hk3.key, Code::KeyA);
        assert_eq!(hk3.mods, Modifiers::SUPER | Modifiers::SHIFT);

        assert!(parse_hotkey_str("InvalidKeyName").is_none());
    }
}
