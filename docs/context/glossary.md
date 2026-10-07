# glossary.md

- **Desktop Layer**: The OS window hierarchy level beneath regular application windows (e.g., above wallpaper, behind Chrome/VS Code).
- **Stealth Window**: A borderless window with `WS_EX_TOOLWINDOW` that omits taskbar and Alt+Tab representation.
- **Hover Controls**: UI elements (like the 'X' close button and resize grip) that remain completely invisible until the cursor moves over their specific interactive target zones.
- **Atomic Persistence**: Writing data to a temporary file before renaming it over the destination file, preventing corruption during system power loss or process kill.
- **Debounced Save**: Delaying disk write operations until a brief pause in user typing (e.g. 300ms) to eliminate wasteful disk I/O.
