# UI Depth

Use depth to communicate interaction and grouping, not decoration.

- Separate surfaces with background and border before relying on shadows.
- Use the existing daisyUI semantic surfaces and theme colors.
- Resting cards may use a subtle shadow; dialogs and sheets may use stronger
  elevation.
- Interactive controls should have clear hover, focus, pressed, and disabled
  states.
- Do not use shadow or color as the only indication of status or permission.
- Keep contrast readable in light and dark themes.
- Prefer Tailwind and daisyUI classes over inline styles or one-off CSS.
- Motion should be restrained, interruptible, and never hide a financial state
  transition or synchronization result.

## Native GPUI Multi-Style Theme System

The native GPUI desktop application (`app/src/theme/mod.rs`) implements an
extensible multi-style theme system driven by `StyleTokens` in GPUI global
state (`cx.set_global(...)`) and synced with `gpui-kit` and `rune-kit`:

- `UiStyle`: `CozyKawaii` (default), with architecture stubs for
  `NeoBrutalism` and `Claymorphism3D`.
- `CozyMood`: `Matcha` (default), `Peach`, and `Berry` across Dark Cozy and
  Light Cozy surfaces.
- Tactile sticker geometry and hard unblurred offset shadows (`radius_card:
  32px`, `radius_button: 22px`, `radius_input: 20px`, `radius_checkbox: 10px`,
  `2.5px`–`3.0px` outlines, `10px` card bottom shadow, `6px` button shadow
  compressing to `2px` on active press).

