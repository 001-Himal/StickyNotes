//! Atomic persistence and reconciliation engine for Sticky Note.

use crate::models::{Config, Note, NotesDocument};
use std::fs;
use std::io::{self, Write};
use std::path::{Path, PathBuf};
use std::time::{SystemTime, UNIX_EPOCH};

/// Returns the current Unix timestamp in seconds.
pub fn current_timestamp() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs()
}

/// Sanitize a string into a safe OS filename (stripping `\ / : * ? " < > |`).
pub fn sanitize_filename(title: &str) -> String {
    let sanitized: String = title
        .chars()
        .filter(|&c| !matches!(c, '\\' | '/' | ':' | '*' | '?' | '"' | '<' | '>' | '|' | '\0'..='\x1f'))
        .collect();

    let trimmed = sanitized.trim().trim_end_matches('.');
    if trimmed.is_empty() {
        "Note".to_string()
    } else {
        trimmed.to_string()
    }
}

/// Generates the next sequential default title, e.g. "Note 1", "Note 2", etc.
pub fn generate_next_title(existing_titles: &[&str]) -> String {
    let mut highest = 0;
    for title in existing_titles {
        let trimmed = title.trim();
        if let Some(rest) = trimmed.strip_prefix("Note ") {
            if let Ok(num) = rest.parse::<u32>() {
                if num > highest {
                    highest = num;
                }
            }
        }
    }
    format!("Note {}", highest + 1)
}

/// Atomically write data to a file by writing to a temporary file in the same
/// directory, flushing with `sync_all`, and renaming to the target path.
pub fn atomic_write_file(path: &Path, data: &[u8]) -> io::Result<()> {
    let parent = path.parent().unwrap_or_else(|| Path::new("."));
    fs::create_dir_all(parent)?;

    // Create a temp file in the same directory to ensure same filesystem for atomic rename
    let mut temp = tempfile::Builder::new()
        .prefix(".tmp_")
        .tempfile_in(parent)?;

    temp.write_all(data)?;
    temp.flush()?;
    temp.as_file().sync_all()?;

    // Atomically persist/rename over target
    temp.persist(path).map_err(|err| err.error)?;
    Ok(())
}

/// Atomically serialize and save JSON to disk.
pub fn atomic_save_json<T: serde::Serialize>(path: &Path, value: &T) -> io::Result<()> {
    let bytes = serde_json::to_vec_pretty(value)
        .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;
    atomic_write_file(path, &bytes)
}

/// Load and parse JSON file, returning None if the file doesn't exist.
pub fn load_json<T: serde::de::DeserializeOwned>(path: &Path) -> io::Result<Option<T>> {
    if !path.exists() {
        return Ok(None);
    }
    let data = fs::read(path)?;
    let parsed = serde_json::from_slice(&data)
        .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;
    Ok(Some(parsed))
}

/// Storage engine managing `Sticky Note Notes/` directory.
#[derive(Debug, Clone)]
pub struct StorageEngine {
    base_dir: PathBuf,
}

impl Default for StorageEngine {
    fn default() -> Self {
        Self {
            base_dir: PathBuf::from("Sticky Note Notes"),
        }
    }
}

impl StorageEngine {
    /// Initialize with a custom base directory (useful for unit tests).
    pub fn new(base_dir: impl Into<PathBuf>) -> Self {
        Self {
            base_dir: base_dir.into(),
        }
    }

    pub fn base_dir(&self) -> &Path {
        &self.base_dir
    }

    pub fn index_path(&self) -> PathBuf {
        self.base_dir.join("Notes.json")
    }

    pub fn note_path(&self, filename: &str) -> PathBuf {
        self.base_dir.join(filename)
    }

    /// Resolve a unique filename for a note title, handling collisions.
    pub fn resolve_unique_filename(&self, note_id: &str, title: &str) -> String {
        let base_name = sanitize_filename(title);
        let mut candidate = format!("{base_name}.json");
        let mut counter = 2;

        while self.is_filename_taken(&candidate, note_id) {
            candidate = format!("{base_name} ({counter}).json");
            counter += 1;
        }

        candidate
    }

    fn is_filename_taken(&self, filename: &str, current_note_id: &str) -> bool {
        let path = self.note_path(filename);
        if !path.exists() {
            return false;
        }
        // If the file exists, check if it belongs to the same note ID
        if let Ok(Some(existing_note)) = load_json::<Note>(&path) {
            existing_note.id != current_note_id
        } else {
            true
        }
    }

    /// Load or rebuild notes and the index cache.
    pub fn load_all(&self) -> io::Result<(NotesDocument, Vec<Note>)> {
        fs::create_dir_all(&self.base_dir)?;

        let index_path = self.index_path();
        let maybe_doc: Option<NotesDocument> = match load_json(&index_path) {
            Ok(doc) => doc,
            Err(e) => {
                eprintln!("Failed to parse Notes.json ({e}). Backing up and rebuilding.");
                let backup_path = self.base_dir.join("Notes.json.corrupted.bak");
                let _ = fs::rename(&index_path, backup_path);
                None
            }
        };

        if let Some(doc) = maybe_doc {
            let mut notes = Vec::new();
            let mut dirty = false;

            // Load notes listed in index
            for meta in &doc.notes {
                let note_file = self.note_path(&meta.filename);
                if note_file.exists() {
                    match load_json::<Note>(&note_file) {
                        Ok(Some(mut note)) => {
                            // Check external modification
                            if let Ok(file_meta) = fs::metadata(&note_file) {
                                if let Ok(mtime) = file_meta.modified() {
                                    let mtime_sec = mtime
                                        .duration_since(UNIX_EPOCH)
                                        .unwrap_or_default()
                                        .as_secs();
                                    if mtime_sec > note.updated_at {
                                        note.updated_at = mtime_sec;
                                        dirty = true;
                                    }
                                }
                            }
                            notes.push(note);
                        }
                        Ok(None) => {}
                        Err(_) => {
                            self.quarantine_corrupted_file(&note_file);
                            dirty = true;
                        }
                    }
                } else {
                    // Note file was deleted externally
                    dirty = true;
                }
            }

            if dirty {
                let rebuilt = self.rebuild_index()?;
                return self.load_all_from_index(rebuilt);
            }

            Ok((doc, notes))
        } else {
            // Index missing or corrupted - rebuild from disk files
            let doc = self.rebuild_index()?;
            self.load_all_from_index(doc)
        }
    }

    fn load_all_from_index(&self, doc: NotesDocument) -> io::Result<(NotesDocument, Vec<Note>)> {
        let mut notes = Vec::new();
        for meta in &doc.notes {
            let note_file = self.note_path(&meta.filename);
            if let Ok(Some(note)) = load_json::<Note>(&note_file) {
                notes.push(note);
            }
        }
        Ok((doc, notes))
    }

    fn quarantine_corrupted_file(&self, path: &Path) {
        let mut corrupted_name = path.as_os_str().to_os_string();
        corrupted_name.push(".corrupted.bak");
        let _ = fs::rename(path, PathBuf::from(corrupted_name));
    }

    /// Rebuild `Notes.json` index from all individual JSON files in the directory.
    pub fn rebuild_index(&self) -> io::Result<NotesDocument> {
        fs::create_dir_all(&self.base_dir)?;
        let mut notes_meta = Vec::new();

        if let Ok(entries) = fs::read_dir(&self.base_dir) {
            for entry in entries.flatten() {
                let path = entry.path();
                if path.is_file() {
                    let file_name = entry.file_name();
                    let name_str = file_name.to_string_lossy();
                    if name_str == "Notes.json" || !name_str.ends_with(".json") {
                        continue;
                    }

                    match load_json::<Note>(&path) {
                        Ok(Some(note)) => {
                            notes_meta.push(note.to_metadata(name_str.to_string()));
                        }
                        Ok(None) => {}
                        Err(_) => {
                            self.quarantine_corrupted_file(&path);
                        }
                    }
                }
            }
        }

        // Sort by created_at timestamp
        notes_meta.sort_by_key(|m| m.created_at);

        let doc = NotesDocument {
            version: 1,
            last_updated: current_timestamp(),
            notes: notes_meta,
        };

        atomic_save_json(&self.index_path(), &doc)?;
        Ok(doc)
    }

    /// Save a note, updating both its individual file and the `Notes.json` index.
    pub fn save_note(&self, note: &mut Note, doc: &mut NotesDocument) -> io::Result<String> {
        fs::create_dir_all(&self.base_dir)?;

        let old_filename = doc
            .notes
            .iter()
            .find(|m| m.id == note.id)
            .map(|m| m.filename.clone());

        let new_filename = self.resolve_unique_filename(&note.id, &note.title);

        // If renamed, remove old file if different
        if let Some(ref old) = old_filename {
            if old != &new_filename {
                let old_path = self.note_path(old);
                let _ = fs::remove_file(old_path);
            }
        }

        note.updated_at = current_timestamp();
        let note_path = self.note_path(&new_filename);
        atomic_save_json(&note_path, note)?;

        // Update or insert into index document
        let meta = note.to_metadata(new_filename.clone());
        if let Some(existing) = doc.notes.iter_mut().find(|m| m.id == note.id) {
            *existing = meta;
        } else {
            doc.notes.push(meta);
        }

        doc.last_updated = current_timestamp();
        atomic_save_json(&self.index_path(), doc)?;

        Ok(new_filename)
    }

    /// Delete a note permanently from disk and index.
    pub fn delete_note(&self, note_id: &str, doc: &mut NotesDocument) -> io::Result<()> {
        if let Some(pos) = doc.notes.iter().position(|m| m.id == note_id) {
            let meta = doc.notes.remove(pos);
            let path = self.note_path(&meta.filename);
            let _ = fs::remove_file(path);

            doc.last_updated = current_timestamp();
            atomic_save_json(&self.index_path(), doc)?;
        }
        Ok(())
    }

    /// Mark a note as closed (saved, but hidden).
    pub fn set_note_closed(&self, note_id: &str, is_closed: bool, doc: &mut NotesDocument) -> io::Result<()> {
        if let Some(meta) = doc.notes.iter_mut().find(|m| m.id == note_id) {
            meta.is_closed = is_closed;
            let note_path = self.note_path(&meta.filename);
            if let Ok(Some(mut note)) = load_json::<Note>(&note_path) {
                note.is_closed = is_closed;
                note.updated_at = current_timestamp();
                atomic_save_json(&note_path, &note)?;
            }
            doc.last_updated = current_timestamp();
            atomic_save_json(&self.index_path(), doc)?;
        }
        Ok(())
    }
}

/// Helper to load application configuration with fallback to defaults.
pub fn load_config_or_default(path: &Path) -> Config {
    match load_json::<Config>(path) {
        Ok(Some(config)) => config,
        _ => Config::default(),
    }
}

/// Helper to atomically save application configuration.
pub fn save_config(path: &Path, config: &Config) -> io::Result<()> {
    atomic_save_json(path, config)
}

/// Resolve a user-edited title: if empty or only whitespace, returns the fallback title.
pub fn resolve_title(input: &str, fallback: &str) -> String {
    let trimmed = input.trim();
    if trimmed.is_empty() {
        if fallback.trim().is_empty() {
            "Note 1".to_string()
        } else {
            fallback.trim().to_string()
        }
    } else {
        trimmed.to_string()
    }
}

/// Updates the default note width and height in configuration for dynamic size inheritance.
pub fn update_default_note_size(config_path: &Path, width: u32, height: u32) -> io::Result<Config> {
    let mut config = load_config_or_default(config_path);
    config.note_appearance.last_used_width = width.max(180);
    config.note_appearance.last_used_height = height.max(120);
    save_config(config_path, &config)?;
    Ok(config)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::NoteColor;

    #[test]
    fn test_sanitize_filename() {
        assert_eq!(sanitize_filename("Meeting / Work: Today?"), "Meeting  Work Today");
        assert_eq!(sanitize_filename("  Note...  "), "Note");
        assert_eq!(sanitize_filename("???"), "Note");
        assert_eq!(sanitize_filename("Normal Title"), "Normal Title");
    }

    #[test]
    fn test_generate_next_title() {
        let existing = vec!["Note 1", "Random", "Note 2", "Note 5"];
        assert_eq!(generate_next_title(&existing), "Note 6");

        let empty: Vec<&str> = vec![];
        assert_eq!(generate_next_title(&empty), "Note 1");
    }

    #[test]
    fn test_resolve_title_placeholder_fallback() {
        assert_eq!(resolve_title("   ", "Note 1"), "Note 1");
        assert_eq!(resolve_title("", "Note 3"), "Note 3");
        assert_eq!(resolve_title("  \t\n  ", "Note 2"), "Note 2");
        assert_eq!(resolve_title("My Custom Title", "Note 1"), "My Custom Title");
        assert_eq!(resolve_title("  Trimmed Title  ", "Note 1"), "Trimmed Title");
    }

    #[test]
    fn test_storage_engine_crud_and_reconciliation() {
        let temp_dir = tempfile::tempdir().expect("Create temp dir");
        let storage = StorageEngine::new(temp_dir.path());

        let mut doc = NotesDocument::default();
        let mut note = Note {
            version: 1,
            id: "note-1".to_string(),
            title: "Groceries".to_string(),
            content: "Milk, Bread, Coffee".to_string(),
            x: 100,
            y: 100,
            width: 300,
            height: 200,
            color: NoteColor::Yellow,
            created_at: 1000,
            updated_at: 1000,
            is_closed: false,
        };

        // 1. Save note
        let filename = storage.save_note(&mut note, &mut doc).expect("Save note");
        assert_eq!(filename, "Groceries.json");
        assert!(storage.note_path("Groceries.json").exists());
        assert!(storage.index_path().exists());

        // 2. Load all
        let (loaded_doc, loaded_notes) = storage.load_all().expect("Load all");
        assert_eq!(loaded_doc.notes.len(), 1);
        assert_eq!(loaded_notes.len(), 1);
        assert_eq!(loaded_notes[0].content, "Milk, Bread, Coffee");

        // 3. Rename note
        note.title = "Weekend Shopping".to_string();
        let new_filename = storage.save_note(&mut note, &mut doc).expect("Rename note");
        assert_eq!(new_filename, "Weekend Shopping.json");
        assert!(!storage.note_path("Groceries.json").exists());
        assert!(storage.note_path("Weekend Shopping.json").exists());

        // 4. Test missing index reconciliation: delete Notes.json
        fs::remove_file(storage.index_path()).expect("Delete index");
        let (rebuilt_doc, rebuilt_notes) = storage.load_all().expect("Load with rebuilt index");
        assert_eq!(rebuilt_doc.notes.len(), 1);
        assert_eq!(rebuilt_notes.len(), 1);
        assert_eq!(rebuilt_notes[0].title, "Weekend Shopping");

        // 5. Test set_note_closed (CloseAction::Close)
        storage.set_note_closed("note-1", true, &mut doc).expect("Close note");
        let (_, closed_notes) = storage.load_all().expect("Load closed note");
        assert_eq!(closed_notes.len(), 1);
        assert!(closed_notes[0].is_closed);
        assert!(doc.notes[0].is_closed);

        // 6. Delete note (CloseAction::Delete)
        storage.delete_note("note-1", &mut doc).expect("Delete note");
        assert!(!storage.note_path("Weekend Shopping.json").exists());
        assert_eq!(doc.notes.len(), 0);
    }

    #[test]
    fn test_dynamic_size_inheritance() {
        let temp_dir = tempfile::tempdir().expect("Create temp dir");
        let config_path = temp_dir.path().join("config.json");

        let updated = update_default_note_size(&config_path, 450, 350).expect("Update size");
        assert_eq!(updated.note_appearance.last_used_width, 450);
        assert_eq!(updated.note_appearance.last_used_height, 350);

        // Clamping to minimum 180x120
        let clamped = update_default_note_size(&config_path, 50, 50).expect("Clamp size");
        assert_eq!(clamped.note_appearance.last_used_width, 180);
        assert_eq!(clamped.note_appearance.last_used_height, 120);
    }
}

