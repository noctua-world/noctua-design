# @noctua-world/design-dist

What the [Noctua design system](https://github.com/noctua-world/noctua-design)
ships to a page: the stylesheets, the behaviour modules, the palettes and the
theme data they read.

**There is no build step.** No Tailwind, no bundler, no preprocessor — these are
the files, generated and committed upstream, and a page links them.

```sh
npm i @noctua-world/design-dist
cp -r node_modules/@noctua-world/design-dist/dist static/dist
```

Then, in `<head>` — **copy the `<link>`s from `static/dist/base.html`**, which is
that list, generated, in the one order that works:

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
`components.css` last, so your own stylesheet linked after it still wins. And
`bootstrap.html` has to run inline in `<head>`: it stamps the theme attributes
before the first paint, and without it the page has **no palette at all**,
because this system's default palette is scope-anchored. (Writing
`<body data-palette="blue-balanced" data-theme="dark">` yourself does the same
job for a page with no theme switcher.)

**The typefaces are the one thing not in here.** `dist/fonts.css` asks for them
at `../assets/fonts/`, beside the directory you copied — 16 MB of faces in every <!-- allow-reference: a URL relative to wherever the consumer put the sheet, not a path in this repository -->
install would be a poor trade. Without them the page uses your system font and
nothing breaks; to have them, take `assets/fonts/` from the repository or from
the release archive.

Each behaviour module is an ES module, exporting `enhanceX(root)` and
`releaseX(root)`:

```js
import { enhanceTabs, releaseTabs } from "@noctua-world/design-dist/tabs.js";
enhanceTabs();
```

Import the two or three your markup needs; nothing pulls in the rest. An app with
a client-side router calls `releaseX` before it replaces markup.

## With Tailwind

Nothing here needs Tailwind, and an application built on this system may use it.
`dist/tailwind/bridge.css` maps Tailwind v4's utilities onto the tokens: every
colour utility onto its `--nc-*` token, and the spacing scale, the radii, the type
sizes, line heights, weights, letter spacing and font roles, the shell's lengths,
the smallest target, the measure and the breakpoints onto their `--noctua-*`
tokens. It declares no colour and no length of its own. Link the sheets from
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
utility.** The bridge declares `--spacing-4` and never a bare `--spacing`, so
`gap-4` is `var(--noctua-space-4)` and `gap-5` emits nothing. Importing
Tailwind's own `theme.css` beside the bridge brings its multiplier and its
palette back, so import `tailwindcss/utilities.css` alone where the tokens are
meant to be the whole vocabulary.

## The components

This package is only what a page links. The React components are
`@noctua-world/design-react`; the Dioxus ones are the `noctua-design` crate, and
`noctua-design-dist` is this package's twin on crates.io — the same files, byte
for byte.

MIT OR Apache-2.0. Vendored terms are named in `LICENSE` upstream.
