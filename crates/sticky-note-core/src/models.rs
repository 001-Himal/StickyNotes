//! Domain models for Sticky Note notes, index cache, and configuration.

use serde::{Deserialize, Serialize};

/// Supported authentic sticky note pastel palettes.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "lowercase")]
pub enum NoteColor {
    Blue,
    Green,
    Pink,
    Purple,
    White,
    #[default]
    Yellow,
}

impl NoteColor {
    /// Return the hex color codes for (header, body, text, accent).
    pub fn hex_codes(&self) -> (&'static str, &'static str, &'static str, &'static str) {
        match self {
            Self::Blue => ("#8FD1F4", "#C2E6F8", "#1A334E", "#70BCE6"),
            Self::Green => ("#B2E89D", "#D8F6C8", "#20382B", "#93D67A"),
            Self::Pink => ("#F5ABC9", "#FCD7E7", "#4A1E24", "#E893B5"),
            Self::Purple => ("#CEA8ED", "#EAD8FA", "#392042", "#B98DE0"),
            Self::White => ("#DCDCDC", "#FFFFFF", "#2B2B2B", "#C5C5C5"),
            Self::Yellow => ("#F6E077", "#FDF1B0", "#2B2B2B", "#E2CA58"),
        }
    }
}

/// Bottom-right paper corner visual style.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "lowercase")]
pub enum CornerStyle {
    #[default]
    Curled,
    Flat,
}

/// Action to take when user clicks the note's top-right 'X' button.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "lowercase")]
pub enum CloseAction {
    Delete,
    Close,
    #[default]
    Ask,
}

/// General preferences.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct GeneralConfig {
    pub start_with_os: bool,
}

/// Appearance preferences.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct NoteAppearanceConfig {
    pub last_used_width: u32,
    pub last_used_height: u32,
    pub corner_style: CornerStyle,
    pub font_family: String,
    pub font_size: u32,
    pub default_color: NoteColor,
}

impl Default for NoteAppearanceConfig {
    fn default() -> Self {
        Self {
            last_used_width: 300,
            last_used_height: 200,
            corner_style: CornerStyle::Curled,
            font_family: "Segoe Print".to_string(),
            font_size: 15,
            default_color: NoteColor::Yellow,
        }
    }
}

/// Interaction behavior preferences.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BehaviorConfig {
    pub close_action: CloseAction,
    pub autosave_debounce_ms: u64,
}

impl Default for BehaviorConfig {
    fn default() -> Self {
        Self {
            close_action: CloseAction::Ask,
            autosave_debounce_ms: 300,
        }
    }
}

/// Global shortcut key bindings.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ShortcutConfig {
    pub new_note: String,
    pub open_settings: String,
}

impl Default for ShortcutConfig {
    fn default() -> Self {
        Self {
            new_note: "Ctrl+Alt+N".to_string(),
            open_settings: "Ctrl+Alt+S".to_string(),
        }
    }
}

/// Application settings schema matching `config.json`.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Config {
    pub version: u32,
    pub general: GeneralConfig,
    pub note_appearance: NoteAppearanceConfig,
    pub behavior: BehaviorConfig,
    pub shortcuts: ShortcutConfig,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            version: 1,
            general: GeneralConfig::default(),
            note_appearance: NoteAppearanceConfig::default(),
            behavior: BehaviorConfig::default(),
            shortcuts: ShortcutConfig::default(),
        }
    }
}

/// Metadata summary of a note in the primary `Notes.json` index.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct NoteMetadata {
    pub id: String,
    pub filename: String,
    pub title: String,
    pub x: i32,
    pub y: i32,
    pub width: u32,
    pub height: u32,
    pub color: NoteColor,
    pub created_at: u64,
    pub updated_at: u64,
    pub is_closed: bool,
    #[serde(default)]
    pub is_bold: bool,
    #[serde(default)]
    pub is_italic: bool,
    #[serde(default)]
    pub is_underlined: bool,
}

/// Document schema for the primary `Notes.json` index file.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct NotesDocument {
    pub version: u32,
    pub last_updated: u64,
    pub notes: Vec<NoteMetadata>,
}

impl Default for NotesDocument {
    fn default() -> Self {
        Self {
            version: 1,
            last_updated: 0,
            notes: Vec::new(),
        }
    }
}

/// Full self-contained note document saved to an individual file (`Note 1.json`).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Note {
    pub version: u32,
    pub id: String,
    pub title: String,
    pub content: String,
    pub x: i32,
    pub y: i32,
    pub width: u32,
    pub height: u32,
    pub color: NoteColor,
    pub created_at: u64,
    pub updated_at: u64,
    pub is_closed: bool,
    #[serde(default)]
    pub is_bold: bool,
    #[serde(default)]
    pub is_italic: bool,
    #[serde(default)]
    pub is_underlined: bool,
}

impl Note {
    /// Convert full note to index metadata entry with the specified file name.
    pub fn to_metadata(&self, filename: String) -> NoteMetadata {
        NoteMetadata {
            id: self.id.clone(),
            filename,
            title: self.title.clone(),
            x: self.x,
            y: self.y,
            width: self.width,
            height: self.height,
            color: self.color,
            created_at: self.created_at,
            updated_at: self.updated_at,
            is_closed: self.is_closed,
            is_bold: self.is_bold,
            is_italic: self.is_italic,
            is_underlined: self.is_underlined,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_config_serialization_roundtrip() {
        let default_config = Config::default();
        let json = serde_json::to_string_pretty(&default_config).expect("Serialize config");
        let parsed: Config = serde_json::from_str(&json).expect("Deserialize config");
        assert_eq!(default_config, parsed);
    }

    #[test]
    fn test_note_and_document_serialization() {
        let note = Note {
            version: 1,
            id: "note-1".to_string(),
            title: "Note 1".to_string(),
            content: "Hello Slint".to_string(),
            x: 100,
            y: 200,
            width: 300,
            height: 220,
            color: NoteColor::Yellow,
            created_at: 1728280000,
            updated_at: 1728280050,
            is_closed: false,
            is_bold: false,
            is_italic: false,
            is_underlined: false,
        };

        let json = serde_json::to_string(&note).expect("Serialize note");
        let deserialized: Note = serde_json::from_str(&json).expect("Deserialize note");
        assert_eq!(note, deserialized);

        let meta = note.to_metadata("Note 1.json".to_string());
        let doc = NotesDocument {
            version: 1,
            last_updated: 1728280050,
            notes: vec![meta],
        };
        let doc_json = serde_json::to_string_pretty(&doc).expect("Serialize doc");
        let parsed_doc: NotesDocument = serde_json::from_str(&doc_json).expect("Deserialize doc");
        assert_eq!(doc, parsed_doc);
    }
}
