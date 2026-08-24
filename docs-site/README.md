# docs-site

The documentation site for this design system: one page per component, in three
locales, pre-rendered to static files by an ordinary Rust binary.

**Read this before adding a page or changing the site's shape.** It carries the
site's scope, its build, and its open gaps — nothing else. How to work in this
repository is [`../AGENTS.md`](../AGENTS.md); the architecture is `NOCTUA.md §9.10`.
Past work is not recorded here and never will be; git holds it (`NOCTUA.md §0.5`).

## What is real

- **The shell** — header, sidebar, mobile drawer, scrim, theme and language menus,
  background layers, search.
- **The theme runtime** — every palette, three modes, two typefaces, glass levels,
  corner presets, patterns, grain. All generated from `spec/theme.toml`; nothing
  here writes a setting name down.
- **Examples are data.** An example is a tree in `src/examples.rs`, interpreted into
  the real typed components, so React renders what Dioxus renders (`NOCTUA.md §9.3`).
  `every_component_has_examples` fails the build for a component with none.
- **One frame per example.** A frame is a viewport, and the question is *does THIS
  survive a narrow column* — asking it of thirty examples at once answers it for none.
- **One page shape** — a working head example, an index, then *When to use it ·
  Examples · Every variant · Reference*. Reference folds ship closed, so every
  `<summary>` carries a real heading: closed, the title is all that is left.
- **Three locales, nothing untranslated** — English, Portuguese, and the generated
  pseudo-locale `/qps/`, which brackets and lengthens every string.
  `every_sentence_on_every_page_is_translated` requires every text run bracketed.
  **Open `/qps/` at 320 px as well: anything that overflows only fitted because
  English is short.**
- **The site routes.** A sidebar link replaces `<main>` and keeps the rest of the
  document. It intercepts a real `<a href>` and nothing more — with the script
  blocked every link still navigates.
- **Search is a palette over the system's own names** — tokens, props, components,
  pages, palettes, built from the generated crates, so an upstream addition is
  searchable without an edit here. A name with no page is not listed: a hit that
  lands somewhere with no answer costs a reader more than no hit. The index is
  `docs-site/dist/assets/search-<locale>.json`, fetched on first search and cached
  for the visit, so no page ships it hidden.

## What is not real

Three gaps, recorded because a reader would otherwise assume they are closed.
Verify one before relying on it. (Two different `dist/` directories appear below:
`../dist/` is what the packages ship, `docs-site/dist/` is this site's own output.)

- **The shared behavioural cases are unwritten.** `spec/components/<name>.cases.toml`,
  the one case set run against both implementations, is `NOCTUA.md §9.3` Leg 3 and
  does not exist yet. Prop-surface parity is generated and enforced; behavioural
  parity is demonstrated by the examples, not asserted from a case file.
- **The React bundle is optional.** `cargo xtask docs` reports its absence and
  carries on, because the Rust half of this repository owes nothing to Node
  (`NOCTUA.md §9.11`). Without it the implementation switch reports a failed load and
  the server-rendered Dioxus example keeps working — and the switch appears **only**
  on a frame that can honour it, since one that offers it and cannot is a control
  that lies.
- **The owl model is NonCommercial.** `../assets/vendor/owl/` is CC BY-NC 4.0. It is
  copied into `docs-site/dist/` and never into `../dist/`, so no consumer of the
  packages receives a byte of it — and the day anything documented here is sold, it
  is the first thing that has to go. Terms in `../assets/vendor/owl/ATTRIBUTION.md`.

## How it is built

```sh
cargo xtask docs                                         # writes docs-site/dist/
cargo xtask dev                                          # serve it and look at it
pnpm --filter @noctua-world/docs-site run build:example   # rebuild the React bundle
```

No `dx`, no wasm toolchain, no JavaScript runtime: `rsx!` rendered to a string with
`dioxus-ssr`. The output is static files any server can serve, and `docs-site/dist/`
is **not** committed — tens of megabytes of copied palettes, fonts and the owl, all
reproducible from sources that are.

**The page is correct before any script runs.** `assets/shell.js` adds behaviour to
a complete page; it never constructs one.

**Every document sits at the same depth** (`<locale>/<slug>/index.html`), which is
what makes `site::BASE` a constant and every relative href resolve identically after
the router swaps a page in. `site.rs` asserts it.

**Build before you prove.** `cargo xtask build` rebuilds `../dist/`;
`cargo xtask docs` rebuilds the site around it. A run against a stale bundle proves
nothing (`NOCTUA.md §19.5`).

## How it is verified

```sh
cargo xtask docs --verify     # build the site, then run the whole suite
pnpm run test:browser         # the suite alone; it prints its own count
```

The suite fails the build, and each file in `tests/browser/` is named after the
property it holds: theme and scope isolation, contrast held to 4.5:1 where ink meets
a pattern, geometry at four widths in three locales, the page whole with JavaScript
blocked, React's markup against Dioxus's, payload, fonts, language. **The count is
deliberately not written down here** — it was once, and it was wrong within a stage.

It uses the system's Chrome (`NOCTUA_CHROME` overrides the path) rather than
Playwright's download, which cannot find its libraries on a Nix system and exits
before Playwright can say why. Run `pnpm install` first.

**Every colour in the suite is read from screenshot pixels** decoded in a canvas,
never from `getComputedStyle` — the same rule `color_read_from_dom` imposes on the
shell. The few places that read a serialised colour do so only to paint it, and
carry an `allow-literal` saying so.

**Before adding a guard, read `NOCTUA.md §19.5`** — the ways an assertion in this
suite has gone vacuous, and the rule that a new guard is confirmed by restoring the
defect it was written for and watching it fail.
