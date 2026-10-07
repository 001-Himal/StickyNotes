# structure.md

Cargo Workspace Layout for Sticky Note:

```
StickyNotes/
├── Cargo.toml                     # Workspace root definition
├── config.json                    # Shared application preferences
├── Sticky Note Notes/             # Data directory
│   ├── Notes.json                 # Index & metadata of notes
│   └── note-*.json                # Individual note files
├── crates/
│   ├── sticky-note-core/          # Shared models, persistence & platform abstractions
│   │   ├── Cargo.toml
│   │   └── src/
│   │       ├── lib.rs
│   │       ├── models.rs          # Note & Config structs
│   │       ├── storage.rs         # Atomic JSON reading/writing
│   │       └── platform/          # Win32, macOS, Linux window helpers
│   │           ├── mod.rs
│   │           ├── windows.rs
│   │           ├── macos.rs
│   │           └── linux.rs
│   ├── sticky-note/               # Primary desktop widget executable
│   │   ├── Cargo.toml
│   │   ├── build.rs               # Slint build integration
│   │   ├── ui/                    # Slint UI templates
│   │   │   └── note_window.slint
│   │   └── src/
│   │       ├── main.rs
│   │       ├── app.rs             # Note lifecycle & window manager
│   │       └── hotkeys.rs         # Global shortcut loop
│   └── sticky-note-settings/      # Auxiliary preferences executable
│       ├── Cargo.toml
│       ├── build.rs               # Slint build integration
│       ├── ui/
│       │   └── settings_window.slint
│       └── src/
│           └── main.rs
├── tests/                         # Integration & persistence tests
│   └── storage_tests.rs
└── docs/                          # Architecture & user guides
    ├── architecture/
    ├── development/
    ├── product/
    └── guides/
```
