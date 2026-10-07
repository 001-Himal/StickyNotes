# COMP-001: Note Window Shell (`NoteWindow.slint`)

## Purpose
The primary frameless container representing an individual sticky note on the OS desktop.

## Sizing & Dynamic Dimensions Inheritance
- **No Static Size Presets:** The app does not force fixed preset sizes.
- **Dynamic Inheritance:** Whenever a user resizes any note via the corner grip, that new dimension `(width, height)` is stored as the current default size in `config.json`. The next newly spawned note automatically adopts this exact size.
- **Individual Note Geometry:** Each note independently saves its own `(x, y, width, height)` in `Notes.json`.
- **Minimum Clamping:** `min-width: 180px`, `min-height: 120px`. Maximum bounded only by current display workspace.

## 6 Authentic Windows 7 Pastel Color Palettes
Each note features a darker shaded header at the top and a lighter shade for its main body:

| Palette | Darker Header Tone | Lighter Body Tone | Text Tone | Accent / Border |
|---|---|---|---|---|
| **1. Blue** | `#8FD1F4` (Light Blue) | `#C2E6F8` (Pale Blue) | `#1A334E` | `#70BCE6` |
| **2. Green** | `#B2E89D` (Soft Green) | `#D8F6C8` (Pale Green) | `#20382B` | `#93D67A` |
| **3. Pink** | `#F5ABC9` (Lavender/Pink) | `#FCD7E7` (Pale Pink) | `#4A1E24` | `#E893B5` |
| **4. Purple** | `#CEA8ED` (Purple) | `#EAD8FA` (Pale Purple) | `#392042` | `#B98DE0` |
| **5. White** | `#DCDCDC` (Light Gray) | `#FFFFFF` (Crisp White) | `#2B2B2B` | `#C5C5C5` |
| **6. Yellow (Default)** | `#F6E077` (Muted Yellow) | `#FDF1B0` (Light Yellow) | `#2B2B2B` | `#E2CA58` |

## Bottom-Right Corner: Curled / Bended Look vs. Normal Flat Look
Togglable via `corner_style` in `config.json`:
1. **Curled / Bended Look (Classic 3M / Win7):**
   - 22×22px folded dog-ear corner effect in the bottom-right corner.
   - Turned-over paper flap rendered with subtle diagonal gradient and under-curl drop shadow (`rgba(0, 0, 0, 0.18)`).
   - Serves as the tactile visual grip for dragging to resize.
2. **Normal / Flat Look (Modern Minimalist):**
   - Flat 4px rounded corner with 3 subtle diagonal grip lines that fade in on corner hover.

## Visual Shell Structure
- Outer container: Frameless, border-radius 4px, subtle ambient shadow.
- Header bar: Top 28px height (contains `+` quick-add, title label, and `X` close).
- Body editor: Flexible height fill.
- Corner resize handle: Bottom-right 22×22px interactive hit area.
