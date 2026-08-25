# noctua-design-dist

What [`noctua-design`](https://github.com/noctua-world/noctua-design) ships to a
page: the stylesheets, the behaviour modules, the palettes and the theme data
they read — embedded, with one call that writes them out.

```toml
[build-dependencies]
noctua-design-dist = "0.2.4"
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

**Copy the `<link>`s from `static/dist/base.html`** into your `<head>`. That file
is the list, generated, in the one order that works:

```html
<link rel="stylesheet" href="static/dist/tokens/css/ramp.css" />
<link rel="stylesheet" href="static/dist/scope.css" />
<link rel="stylesheet" href="static/dist/tokens/css/theme-blue-balanced.css"
      id="noctua-base-palette" />
<link rel="stylesheet" href="static/dist/tokens.css" />
<link rel="stylesheet" href="static/dist/tones.css" />
<link rel="stylesheet" href="static/dist/syntax.css" />
<link rel="stylesheet" href="static/dist/fonts.css" />
<link rel="stylesheet" href="static/dist/theme.css" />
<link rel="stylesheet" href="static/dist/components.css" />
<!-- then paste the contents of static/dist/bootstrap.html here -->
```

**Two things fail silently if you skip them.** The order is load-bearing —
`components.css` comes last so your own stylesheet, linked after it, still wins.
And `bootstrap.html` has to run inline in `<head>`: it stamps the theme
attributes before the first paint, and without it the page has **no palette at
all**, because this system's default palette is scope-anchored. (A page with no
theme switcher can write `<body data-palette="blue-balanced" data-theme="dark">`
instead.)

**The typefaces are the one thing not in here.** `dist/fonts.css` asks for them
at `../assets/fonts/`, beside the directory you wrote — 16 MB of faces in every <!-- allow-reference: a URL relative to wherever the consumer put the sheet, not a path in this repository -->
install would be a poor trade. Without them the page uses the system font and
nothing breaks; the faces and their licences are in the repository under
`assets/fonts/`.

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
