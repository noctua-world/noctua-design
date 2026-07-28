# docs-site — what is real, and what is a placeholder

This file exists so nobody has to guess. Every page in this site renders, every
control works, and every claim the shell makes about itself is enforced by a
test. But the *content* is thin on purpose: this stage built the frame, and the
stage that writes the documentation fills it.

Read it before adding a page.

## What is real

**The shell.** Header, sidebar, mobile drawer, the partial scrim, the settings
panel, the effect layers, search. All of it is measured rather than asserted —
see "Verified" below.

**The theme runtime.** All thirty-nine palettes, three modes, two typefaces,
three glass levels, four corner presets, four background patterns, the aurora,
the grain and ambient motion. Every one of them is generated from
`spec/theme.toml`; nothing on this site writes a setting name down.

**Scoped themes.** `/scoped-themes` is not a demonstration built for the page —
it is two real `ThemeScope` components with the same controls the page header
has. That page is the reason the rest of this stage exists.

**Both locales, plus a third.** English, Portuguese, and a generated
pseudo-locale at `/qps/` that brackets and lengthens every string. Open it at
320 px: anything that is not bracketed was hard-coded and never went through the
catalogue, and anything that overflows only fitted because English is short.

## What is a placeholder

**The prose.** Every page but `/scoped-themes` is a heading, a summary and a
demonstration of the thing it names. None of them explains anything yet. The
`provisional.*` messages in `spec/i18n/` exist for exactly this.

**The example frame's implementation switch.** The frame is real — width
controls, reset, and a stage that swaps in place. What is *not* wired is the
React path: mounting the React implementation next to the Dioxus one needs
`pnpm install` and a bundler, and `NOCTUA.md §9.11` bounds Node's role in this
repository to the React package and the conformance runner. Both implementations
of `ThemeScope` are generated and both are shipped; only the live side-by-side
mount is missing. Adding it is a task for the stage that stands up the Node
toolchain, not a rewrite of the frame.

**The component page template.** `/components` describes the contract and shows
the one component that exists. Generating a page from `spec/components/*.toml`
waits for the first component that has one — Stage 6.

**Search covers pages, not props.** The index is each link's collated title,
built at compile time by the same function that sorted the sidebar. Extending it
to token and prop names is a matter of emitting a second index; the search box
and its empty state already exist.

## How it is built

```sh
cargo xtask docs      # writes docs-site/dist/
```

An ordinary Rust binary. No `dx`, no wasm toolchain, no JavaScript runtime — it
renders `rsx!` to a string with `dioxus-ssr`. The output is static files that
any server can serve, and `docs-site/dist/` is **not** committed: it is about
5 MB of copied palette stylesheets, all of it reproducible from sources that are.

The page is correct before any script runs. `assets/shell.js` adds the settings
to a complete page; it never constructs one.

## Verified, in a real browser

Not asserted — measured. The numbers below are from the built site.

| | |
|---|---|
| A scope's `--nc-color-surface` equals **its own** `--nc-neutral-bg-app`, not the page's | ✓ |
| A nested scope keeps its parent's palette when only its mode changes | ✓ |
| Drawer `[0, 256]`, scrim `[256, 854]` — **seam 0.0000 px, intersection area 0** | ✓ |
| `elementFromPoint` inside the drawer never returns the scrim | ✓ |
| The drawer's `border-radius` is `0px`, ignoring the reader's corner preset | ✓ |
| Blackout paints `rgb(0, 0, 0)` and leaves `--nc-color-surface` byte-identical | ✓ |
| The header needs **212 px** of a 320 px budget | ✓ |
| No `box-shadow` on any element in the shell | ✓ |
| Every accent swatch shows its own accent, from its own `data-palette` scope | ✓ |
| pt-BR orders the sidebar in Portuguese, not in English | ✓ |

## Two bugs this stage found and fixed

Both were silent, which is why they are written down.

**Glass was never applied on a first visit.** Every glass block was a bare
attribute selector — which is what makes glass scopable — so until a reader
changed the setting, nothing matched, `--noctua-glass-blur` was undefined, and
`backdrop-filter: blur(var(--noctua-glass-blur))` was invalid at computed-value
time. An invalid `backdrop-filter` is `none`. Every frosted surface in the system
rendered flat and nothing reported it. The default level is now also declared at
`:where(:root)`, and a test asserts it.

**Three `max-width` media queries.** Written by the same file whose header says
breakpoints are minimum-width only. The gates did not catch it because media
preludes were unchecked. They are now: `max-width` is rejected outright, and a
`min-width` must equal a breakpoint declared in `spec/tokens.toml`.
