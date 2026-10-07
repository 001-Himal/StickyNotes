# design-system.md

## Visual Metaphor: Physical Paper on Desktop
Inspired by the tactile warmth and minimalism of classic Windows 7 Sticky Notes and genuine 3M Post-it notes.

## Palette
- **Default Paper Tone:** `#FFF8D6` (Warm Canary Yellow)
- **Header Accent Tone:** `#F5E8A9` (Slightly deeper yellow for subtle header division)
- **Text Color:** `#2B2B2B` (Soft Charcoal, not harsh `#000000`)
- **Placeholder / Watermark:** `#8E8A75`
- **Hover Close Button Background:** `#E85D5D` (Subtle red badge) / White 'X' glyph
- **Resize Grip Color:** `#A8A38B`
- **Optional Pastel Palettes:**
  - Soft Blue: `#E3F2FD` (Header: `#BBDEFB`)
  - Mint Green: `#E8F5E9` (Header: `#C8E6C9`)
  - Lavender: `#F3E5F5` (Header: `#E1BEE7`)
  - Coral Pink: `#FFEBEE` (Header: `#FFCDD2`)

## Typography
- **Font Stack:** Native system sans-serif (Segoe UI on Windows, SF Pro on macOS, Roboto/Noto Sans on Linux).
- **Body Font Size:** 14px (configurable 12px–20px).
- **Header Title Font Size:** 12px semi-bold.

## Layout & Micro-Interactions
- **Header Bar:** 28px height. Drag region. Double click activates text input.
- **Close Button:** 18×18px circular or rounded square in top-right. Zero opacity by default; `opacity: 1.0` smoothly on header hover.
- **Resize Handle:** 16×16px hit area in bottom-right corner. Three diagonal grip lines fade in on corner hover.
- **Window Shadow:** Subtle ambient shadow (blur: 12px, alpha: 0.15) for desktop elevation.
