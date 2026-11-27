# noctua-design-theme

The theme model of the
[Noctua design system](https://github.com/noctua-world/noctua-design): the
appearances, the palettes and the settings a scope carries.

**Generated** from `spec/theme.toml` in that repository, and committed there.
Nothing in `src/` is written by hand.

The values a page actually paints with are CSS custom properties, in
`noctua-design-dist`. This crate is what Rust code reads to know which settings
exist and what each one may be — a `ThemeScope` writing attributes, a server
deciding what to stamp before the first paint.

One colour is here as data: the page's. A native host paints its window before
its page has a first frame, with no stylesheet to ask, so
`Palette::page(mode, backdrop)` answers in sRGB bytes with what
`dist/theme.css` paints the page for that palette, appearance and backdrop.

```rust
use noctua_design_theme::{Backdrop, Mode, Palette};

// A light page is paper, whatever the palette.
assert_eq!(Palette::DEFAULT.page(Mode::Light, Backdrop::Plain), Some([255, 255, 255]));
// The system preference is the host's to read; ask again with what it resolved to.
assert_eq!(Palette::DEFAULT.page(Mode::System, Backdrop::Plain), None);
```

MIT OR Apache-2.0.
