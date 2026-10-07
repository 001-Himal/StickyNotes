# frontend.md (Slint Desktop UI)

- Adhere strictly to the Slint design system in `docs/product/design-system.md`.
- Micro-interactions: Controls ('X' close button, '+' add button, and resize grip) must reveal ONLY on hover over their respective zones.
- Keep the note surface pristine: Zero permanent toolbars, buttons, or menus on the note body.
- Typography: Use system fonts with high legibility and soft contrast (charcoal on warm canary yellow).
- Window behavior: Never force always-on-top; notes must reside quietly on the desktop layer beneath normal apps.
- User-centric copy: Never expose internal technical plumbing or errors in note text.
