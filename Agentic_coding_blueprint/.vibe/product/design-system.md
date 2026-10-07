# design-system.md

## Visual Metaphor: Physical Paper on Desktop
Inspired by the tactile warmth and minimalism of classic Windows 7 Sticky Notes and genuine 3M Post-it notes.

## 6 Pastel Palettes

| Theme Name | Paper Background | Header Bar | Charcoal Text | Accent / Hover |
|---|---|---|---|---|
| **Canary Yellow (Default)** | `#FFF8D6` | `#F5E8A9` | `#2B2B2B` | `#E0CF82` |
| **Mint Green** | `#E8F5E9` | `#C8E6C9` | `#20382B` | `#A5D6A7` |
| **Sky Blue** | `#E3F2FD` | `#BBDEFB` | `#1A334E` | `#90CAF9` |
| **Lavender** | `#F3E5F5` | `#E1BEE7` | `#392042` | `#CE93D8` |
| **Soft Pink** | `#FFEBEE` | `#FFCDD2` | `#4A1E24` | `#EF9A9A` |
| **Crisp White** | `#FFFFFF` | `#F0F0F0` | `#2B2B2B` | `#E0E0E0` |

## Settings Popup Theme
The `Sticky Note Settings` window intentionally shares this exact design language:
- Card-like warm paper aesthetic (`#FFF8D6` background, `#F5E8A9` header).
- Soft rounded edges (6px border radius).
- Same typography and charcoal text tones.
- Never rendered as a cold grey system dialog.

## Typography
- **Font Stack:** Native system sans-serif (`Segoe UI` on Windows, `SF Pro` on macOS, `Roboto`/`Noto Sans` on Linux).
- **Body Font Size:** 14px default (scalable in settings).
- **Header Title Font Size:** 12px semi-bold.

## Layout & Micro-Interactions
- **Header Bar:** 28px height. Drag region.
  - Left: `+` quick-add button (`18×18px`). Smoothly fades in on header hover (`opacity: 1.0`).
  - Right: `X` close button (`18×18px`). Smoothly fades in on header hover (`opacity: 1.0`).
  - Center: Title label. Double-click swaps to inline rename input.
- **Resize Handle:** 20×20px hit area in bottom-right corner. Three subtle diagonal grip lines fade in on corner hover.
- **Window Shadow:** Subtle ambient elevation (blur: 10px, alpha: 0.12).
