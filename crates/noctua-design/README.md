# noctua-design

The [Noctua design system](https://github.com/noctua-world/noctua-design) as
Dioxus components — the Rust half of two packages that behave identically, the
other being `@noctua-world/design-react`.

```toml
[dependencies]
noctua-design = "0.2.4"

[build-dependencies]
noctua-design-dist = "0.2.4"   # what the page links
```

```rust
use dioxus::prelude::*;
use noctua_design::{Button, ThemeScope};

fn app() -> Element {
    rsx! {
        ThemeScope { Button { "Save" } }
    }
}
```

## The page needs three things

**The stylesheets**, from [`noctua-design-dist`](https://crates.io/crates/noctua-design-dist):
call `noctua_design_dist::write_to("static/dist")` from `build.rs`. Components
are markup; nothing here writes a file, which is why the two crates are
separate.

**The links, in the order `dist/base.html` states.** It is generated, and the
order is load-bearing: the default palette has to precede anything that could
tie with it, and the component rules come last so your own stylesheet — linked
after them — wins.

**`dist/bootstrap.html`, before the first paint.** It stamps the theme
attributes. Link the sheets without it and the page has no palette at all,
because this system's default palette is scope-anchored.

## Behaviour is opt-in, and released

A component that owes a keyboard ships one, as an ES module a page imports
directly:

```html
<script type="module">
  import { enhanceTabs } from "/static/dist/tabs.js";
  enhanceTabs();
</script>
```

Import the two or three your markup needs. Each module exports `enhanceX(root)`
and `releaseX(root)`; an app with a client-side router calls the second before
it replaces markup.

## What holds the two implementations together

Both are generated from one contract per component, so a prop that exists in one
exists in the other or the build fails. What each *does* — the keys, the states,
the focus — is a third file, driven through both in a real browser by the
repository's conformance runner. That mechanism is `NOCTUA.md §9.3` there, and it
is why "behaves identically" is a claim with a test behind it.

`dioxus` is taken with `default-features = false`: the renderer stays your
decision, which is what lets these components be pre-rendered to static HTML and
hydrated later.

## Licence

MIT OR Apache-2.0, at your option. Vendored files carry their own terms, named in
`LICENSE` in the repository: the typefaces (SIL OFL 1.1, and MIT for the
symbols), the icon drawings (ISC and MIT), three.js (MIT), and a photographed owl
that is CC BY-NC 4.0 and reaches the documentation site rather than any package.
