# Skill: storage migration

1. Inspect current `Notes.json` or `config.json` schema.
2. Plan exact schema changes, backward-compatibility logic, and default values.
3. Verify automatic creation of `.bak` files before in-place file rewrite.
4. Add unit test asserting older schema JSON deserializes cleanly into new struct models.
5. Verify zero data loss across existing notes.
