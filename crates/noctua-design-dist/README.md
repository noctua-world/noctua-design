# noctua-design-dist

What [`noctua-design`](https://github.com/noctua-world/noctua-design) ships to a
page: the stylesheets, the behaviour modules, the palettes and the theme data
they read — embedded, with one call that writes them out.

```toml
[build-dependencies]
noctua-design-dist = "0.1.0"
```

```rust
// build.rs
fn main() -> std::io::Result<()> {
    noctua_design_dist::write_to("static/dist")
}
```

`FILES` is the same set, as `(name, contents)`, if you would rather place them
yourself.

## What to do with them

**Link `dist/base.html`.** It is the list of stylesheets, generated, and the
order in it is load-bearing: the default palette has to precede anything that
could tie with it, and the component rules come last so your own stylesheet —
linked after them — wins.

**Run `dist/bootstrap.html` too.** It stamps the theme attributes before the
first paint. Link the sheets without it and the page has no palette at all,
because this system's default palette is scope-anchored.

**Put the faces beside the directory, not inside it.** `dist/fonts.css` reaches
them at `../assets/fonts/`, which is the one path in here that looks outside <!-- allow-reference: a URL relative to wherever the consumer put the sheet, not a path in this repository -->
itself. The faces are in the repository under `assets/fonts/`, with their
licences.

Each behaviour module is an ES module a page imports directly, exporting
`enhanceX(root)` and `releaseX(root)`:

```html
<script type="module">
  import { enhanceTabs } from "/static/dist/tabs.js";
  enhanceTabs();
</script>
```

Import the two or three your markup needs; nothing pulls in the rest.

## The components themselves

This crate is only what a page links. The Dioxus components are
[`noctua-design`](https://crates.io/crates/noctua-design); the React ones are
`@noctua-world/design-react`, and `@noctua-world/design-dist` is this crate's
twin on npm — the same files, byte for byte.

## Licence

MIT OR Apache-2.0, at your option. Four sets of vendored files carry their own
terms, named in `LICENSE` in the repository: the typefaces (SIL OFL 1.1, and MIT
for the symbols), the icon drawings (ISC and MIT), three.js (MIT), and a
photographed owl that is CC BY-NC 4.0 and reaches the documentation site rather
than this crate.
