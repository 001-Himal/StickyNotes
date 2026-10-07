# design-system.md

## Visual Metaphor: Physical Paper on Desktop
Inspired by the tactile warmth and minimalism of classic Windows 7 Sticky Notes and genuine 3M Post-it notes.

## 6 Authentic Sticky Note Palettes

Matched precisely to the classic Windows 7 sticky notes palette (darker header at top, lighter shade for main body):

| Note Palette | Header Color (Darker) | Main Body Color (Lighter) | Text Color | Icon / Border Accent |
|---|---|---|---|---|
| **1. Blue** | `#8FD1F4` (Light Blue) | `#C2E6F8` (Pale Blue) | `#1A334E` | `#70BCE6` |
| **2. Green** | `#B2E89D` (Soft Green) | `#D8F6C8` (Pale Green) | `#20382B` | `#93D67A` |
| **3. Pink** | `#F5ABC9` (Lavender/Pink) | `#FCD7E7` (Pale Pink) | `#4A1E24` | `#E893B5` |
| **4. Purple** | `#CEA8ED` (Purple) | `#EAD8FA` (Pale Purple) | `#392042` | `#B98DE0` |
| **5. White** | `#DCDCDC` (Light Gray) | `#FFFFFF` (Crisp White) | `#2B2B2B` | `#C5C5C5` |
| **6. Yellow (Default)** | `#F6E077` (Muted Yellow) | `#FDF1B0` (Light Yellow) | `#2B2B2B` | `#E2CA58` |

## Bottom-Right Corner: Curled / Bended vs. Normal Flat Look

The bottom-right corner serves as the visual anchor and resize zone. The user can toggle between two aesthetics in settings:

1. **Curled / Bended Look (Classic 3M / Win7 Aesthetic):**
   - The bottom-right corner features an authentic **bended dog-ear paper curl** effect, simulating paper lifting gently off the desktop.
   - Geometry: 22×22px triangular fold in the bottom-right corner.
   - Visual: A lighter turned-over paper flap with a soft drop-shadow underneath (`rgba(0, 0, 0, 0.18)`), creating tactile depth.
   - Dual function: Delivers the iconic paper aesthetic while intuitively inviting the user to grab and resize.
2. **Normal / Flat Look (Modern Minimalist):**
   - Clean, flush 4px border radius.
   - Three subtle diagonal grip tick-marks (`#A8A38B`) that smoothly fade in only when hovered.

## Settings Popup Theme
The `Sticky Note Settings` window intentionally shares this exact design language:
- Card-like warm paper aesthetic (`#FDF1B0` background, `#F6E077` header).
- Soft rounded edges (6px border radius).
- Same typography and charcoal text tones.
- Never rendered as a cold grey system dialog.

## Typography & Text Styling
- **Default Font:** Authentic Windows 7 cursive handwriting font **`Segoe Print`** (`segoepr.ttf`), shipped with Windows. Fallbacks: `Segoe Script`, `Comic Sans MS`, `Bradley Hand`, cursive.
- **Body Font Size:** 15px default (scalable via settings or in-editor `Ctrl + =` / `Ctrl + -`).
- **Header Title Font Size:** 12px semi-bold (`Segoe Print` / system sans-serif).
- **Seamless Canvas:** 100% transparent text area across all themes and focus states. The note body never shifts color or displays dark input boxes in dark mode.
- **Ruled Paper Guidelines:** Toggling underline mode (`Ctrl + U`) displays subtle, elegant horizontal notebook ruled lines (15% opacity ink tone) spaced at 24px intervals.

## Header Bar & Controls
- **Height:** 28px fixed.
- **Top-Left:** Small `+` (quick new note) icon (`18×18px`). Fades in on header hover (`opacity: 1.0`).
- **Center:** Note title. Double-click swaps to inline rename field.
- **Top-Right:** Small `X` (close/delete) icon (`18×18px`). Fades in on header hover (`opacity: 1.0`).
- **Shadow:** Subtle ambient elevation shadow (blur: 10px, alpha: 0.12).
