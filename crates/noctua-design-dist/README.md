# noctua-design-dist

What [`noctua-design`](https://github.com/noctua-world/noctua-design) ships to a
page: the stylesheets, the behaviour modules, the palettes and the theme data
they read — embedded, with one call that writes them out.

```toml
[build-dependencies]
noctua-design-dist = "0.3.1"
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

**The typefaces are crates of their own.** `dist/fonts.css` asks for each at
`../assets/fonts/<file>`, beside the directory you wrote, and <!-- allow-reference: a URL relative to wherever the consumer put the sheet, not a path in this repository -->
`noctua-design-fonts` is those files with their licences:
`noctua_design_fonts::write_to("static/assets/fonts")`. The three CJK families
are `noctua-design-fonts-jp`, `-kr` and `-sc`, because a registry refuses a crate
over 10 MB and the faces together are 17 MB. Without them the page uses the
system font and nothing breaks.

Each behaviour module is an ES module a page imports directly, exporting
`enhanceX(root)` and `releaseX(root)`:

```html
<script type="module">
  import { enhanceTabs } from "/static/dist/tabs.js";
  enhanceTabs();
</script>
```

Import the two or three your markup needs; nothing pulls in the rest.

## With Tailwind

Nothing here needs Tailwind, and an application built on this system may use it.
`dist/tailwind/bridge.css` maps Tailwind v4's utilities onto the tokens: every
colour utility onto its `--nc-*` token, and the spacing scale, the radii, the type
sizes, line heights, weights, letter spacing and font roles, the shell's lengths,
the smallest target, the measure, the easings and the breakpoints onto their
`--noctua-*` tokens. It declares no colour and no length of its own. Link the sheets from
`base.html` as above, then import the bridge after Tailwind in the entry
stylesheet:

```css
@import "tailwindcss";
@import "./static/dist/tailwind/bridge.css";
```

`bg-surface`, `text-fg` and `border-border` then follow the palette, the mode and
a theme scope exactly as the components do, and `gap-4`, `p-inset`, `min-w-0`,
`h-shell-line`, `rounded-control`, `text-body`, `leading-body`, `font-semibold`
and `font-ui` are the design tokens by name.

**A step of the spacing scale is a name, and a number between two steps is no
utility.** The bridge declares `--spacing-4` and sets the bare `--spacing` to
`initial`, so `gap-4` is `var(--noctua-space-4)` and `gap-5` emits nothing,
under either import. That takes out Tailwind's multiplier and nothing else:
`@import "tailwindcss"` still brings Tailwind's own type sizes, radii and
palette (`text-sm`, `rounded-md`). Import `tailwindcss/utilities.css` in its
place where the tokens are meant to be the whole vocabulary.

**A duration, a layer and a border width are utilities by the token's name.**
Tailwind has no theme variable for them, and its own are bare numbers
(`duration-150`, `z-10`, `border-2`), which no token stands behind. The bridge
adds `duration-fast`, `z-dialog`, `border-hairline` with its sides
(`border-b-hairline`), `outline-focus` and `outline-offset-focus`, and maps
`ease-out`, `ease-in-out` and `ease-linear`. A `border-hairline` and an
`outline-focus` each draw their line with no other class: the style comes with
the width. A `duration-fast` follows `prefers-reduced-motion`, as the token
does. The breakpoints are written as
lengths, and the scene tokens have no utility.

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
