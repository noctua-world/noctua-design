# noctua-design-tokens

The [Noctua design system](https://github.com/noctua-world/noctua-design)'s
tokens — spacing, radii, type, motion — as Rust constants.

**Generated**, from `spec/` in that repository, and committed there. Nothing in
`src/` is written by hand, and a value edited here would be overwritten by the
next build rather than reaching anyone.

No dependencies, and there never will be: a consumer showing a button should not
inherit somebody else's crate to learn what a radius is.

The colours are not here. They come from
[`noctua-colors`](https://github.com/noctua-world/noctua-colors), which solves
them against a page rather than fixing them, and reach a page as CSS custom
properties in `noctua-design-dist`.

MIT OR Apache-2.0.
