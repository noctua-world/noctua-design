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

MIT OR Apache-2.0.
