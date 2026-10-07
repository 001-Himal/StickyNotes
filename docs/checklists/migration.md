# Checklist: storage migration
- [ ] Schema version incremented in `models.rs` (`version` field in `Notes.json` or `config.json`)
- [ ] Backward compatibility: Old JSON structure successfully deserializes into new models
- [ ] Automatic backup created (`.bak`) before modifying existing user files
- [ ] Unit tests verify migration from previous schema versions
