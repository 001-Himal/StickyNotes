# ADR-007: Seamless Text Editor Architecture and Windows 7 Handwritten Typography

## Context
When running on modern Windows installations with OS Dark Mode enabled, Slint's standard library widget `TextEdit` from `std-widgets.slint` automatically adapts to Fluent dark theme tokens. This resulted in:
1. An unnatural, opaque dark gray (`#2D2D2D`) input box with an active focus underline appearing inside the pastel note paper whenever clicked or focused.
2. In unfocused states, text rendered in white (`#FFFFFF`) on light canary yellow (`#FDF1B0`), destroying visual contrast.
3. Typography defaulted to generic `Segoe UI` instead of the iconic cursive handwriting typography of classic Windows 7 Sticky Notes.
4. Shortcut combinations (`Ctrl+B`, `Ctrl+I`, `Ctrl+U`, `Ctrl+L`) previously injected raw Markdown strings (`****`, `_`, `- `) directly into the note's text buffer.

## Decision
1. **Low-Level `TextInput` Primitive Inside `ScrollView`**:
   - Replaced `TextEdit` with Slint's lower-level `TextInput` wrapped in a standard `ScrollView`.
   - `TextInput` carries no built-in background, border, or OS theme styling, remaining 100% transparent.
   - The note paper (`root.body-bg`) serves directly as the text canvas across all states (focused, idle, typing, resizing).
   - Text color is locked to the curated dark ink token (`root.text-color`), maintaining a >11:1 contrast ratio.

2. **Segoe Print Handwriting Typography**:
   - Standardized on Microsoft's native cursive font **`Segoe Print`** (`segoepr.ttf`, `segoeprb.ttf`), which is bundled with all modern Windows versions.
   - Scaled default font size to 15px.

3. **Non-Destructive Visual Style Modifiers**:
   - Replaced Markdown string injection with visual formatting flags (`is_bold`, `is_italic`, `is_underlined`).
   - `Ctrl+B` toggles bold font weight (`700` vs `400`).
   - `Ctrl+I` toggles italic slant.
   - `Ctrl+U` toggles authentic horizontal notebook ruled guidelines.
   - `Ctrl+L` toggles genuine bullet point glyphs (`• `) cleanly without Markdown formatting hyphens.
   - `Ctrl+=` / `Ctrl+-` dynamically scale text size.
   - Visual attributes persist per-note in individual JSON documents (`Note 1.json`).

## Consequences
- **Positive**:
  - The sticky note behaves like physical pastel paper; zero dark box intrusion or color changes on click.
  - High contrast and effortless legibility in all Windows themes.
  - Faithful Windows 7 Sticky Note look and feel out of the box.
  - In-note shortcuts format text styling cleanly without polluting user content.
- **Negative**:
  - Full WYSIWYG arbitrary mixed-character rich-text spans within a single note are constrained until Slint introduces native multi-span editable text components. Entire-note styling toggles provide an elegant, consistent alternative.
