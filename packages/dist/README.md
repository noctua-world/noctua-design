# @noctua-world/design-dist

What the [Noctua design system](https://github.com/noctua-world/noctua-design)
ships to a page: the stylesheets, the behaviour modules, the palettes and the
theme data they read.

**There is no build step.** No Tailwind, no bundler, no preprocessor — these are
the files, generated and committed upstream, and a page links them.

```sh
npm i @noctua-world/design-dist
```

```html
<!-- the list, and the order, are in dist/base.html -->
<link rel="stylesheet" href="/node_modules/@noctua-world/design-dist/dist/base.css" />
```

**Link `dist/base.html`'s list, in its order.** It is generated, and the order is
load-bearing: the default palette has to precede anything that could tie with it,
and the component rules come last so your own stylesheet — linked after them —
wins.

**Run `dist/bootstrap.html` too**, before the first paint. It stamps the theme
attributes; without it the page has no palette at all, because this system's
default palette is scope-anchored.

**Put the faces beside the directory.** `dist/fonts.css` reaches them at
`../assets/fonts/`, which is the one path in here that looks outside itself. <!-- allow-reference: a URL relative to wherever the consumer put the sheet, not a path in this repository -->

Each behaviour module is an ES module, exporting `enhanceX(root)` and
`releaseX(root)`:

```js
import { enhanceTabs, releaseTabs } from "@noctua-world/design-dist/tabs.js";
enhanceTabs();
```

Import the two or three your markup needs; nothing pulls in the rest. An app with
a client-side router calls `releaseX` before it replaces markup.

## The components

This package is only what a page links. The React components are
`@noctua-world/design-react`; the Dioxus ones are the `noctua-design` crate, and
`noctua-design-dist` is this package's twin on crates.io — the same files, byte
for byte.

MIT OR Apache-2.0. Vendored terms are named in `LICENSE` upstream.
