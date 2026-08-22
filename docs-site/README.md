# docs-site

The documentation site for this design system: one page per component, in three
locales, pre-rendered to static files.

**Read this before adding a page or changing the site's shape.** It is short on
purpose — it carries the site's own scope, its build and its gaps, and nothing else.
The rules for working in this repository are in [`../AGENTS.md`](../AGENTS.md); the
architecture is `NOCTUA.md §9.10`. A record of past work is not here and never will
be: that is what the git history is for (`NOCTUA.md §0.5`).

## What is real

**The shell.** Header, sidebar, mobile drawer, the partial scrim, the theme menu,
the language menu, the background layers, search. All of it measured rather than
asserted — including that each frosted surface actually *samples* rather than merely
declaring `backdrop-filter`.

**The theme runtime.** Every palette, three modes, two typefaces, the glass levels,
the corner presets, the background patterns and the grain. All generated from
`spec/theme.toml`; nothing on this site writes a setting name down.

**Every component has worked examples, and every example is data.** An example is a
tree in `docs-site/src/examples.rs`, interpreted into the real typed components, so
the **React** package renders the same examples the Dioxus one does — the two halves
of `NOCTUA.md §9.3` are demonstrated on the page rather than asserted beside it.
`every_component_has_examples` fails the build for a component with none.

**One frame per example.** A frame is a viewport, and the question it answers is
*does THIS survive a narrow column* — answering it for thirty examples at once
answers it for none of them. Its controls live on a rail down the trailing edge.

**One page shape, on every page.** A head opening with a complete working example,
an index, then **When to use it · Examples · Every variant · Reference** in that
order. The reference folds ship closed, and a fold carries a real heading inside its
`<summary>`, because closed the title is all that is left of the section.

**Three locales, and nothing untranslated.** English, Portuguese, and a generated
pseudo-locale at `/qps/` that brackets and lengthens every string.
`every_sentence_on_every_page_is_translated` renders every page in `qps` and asserts
every text run is bracketed, skipping only `<code>`, `<pre>`, `<kbd>`, `<time>`,
`<cite>` and anything carrying `.nd-identifier` — the class a page uses to say *this
is a name, not a sentence*. **Open `/qps/` at 320 px as well: anything that overflows
only fitted because English is short.**

**The site routes.** Following a sidebar link replaces `<main>` and keeps the rest of
the document, so the navigation does not reload and does not lose its scroll offset.
It is an interception of a real `<a href>` and nothing else — with the script blocked
every link still navigates. The consequence for anything you add here is in
`../AGENTS.md`: *the site routes, so an enhancer with no release is live rather than
latent.*

**Search covers the system's own names, and it is a palette.** Tokens, props, every
component, every page and every palette, each pointing at the page that explains it,
built from the generated crates so an upstream addition is searchable without an edit
here. **A name with no page is not listed**: a hit that lands somewhere with no answer
on it costs a reader more than no hit at all.

**The index is a file, and the sidebar is a trigger.** The palette reads
`dist/assets/search-<locale>.json` on the first search and caches it for the visit,
rather than shipping every entry hidden in every document of every locale. It is also
the honest half of `Search`'s contract — an index at a URL is the form that stops
working with the script blocked — so the trigger ships hidden until the script reveals
it, into a slot the stylesheet has already reserved.

## What is not real

Four gaps, each recorded rather than papered over. Verify one before relying on it;
the point of writing them down is that they are the ones a reader would otherwise
assume are closed.

**`dist/` does not carry the component styles.** It has the token layer, the font
faces, the theme and appearance layers and the scoped semantic contract — everything
a page needs to be *themed*. The rules that make an `.nd-heading` a heading live in
`docs-site/assets/shell.css`, so today the components are styled only inside this
site. `NOCTUA.md §9.4` describes the artifact this is meant to become.

**The shared behavioural cases are not written.** `<name>.cases.toml` — the one case
set run against both implementations — is `NOCTUA.md §9.3` Leg 3, and it does not
exist yet. Prop-surface parity is generated and enforced; behavioural parity is
demonstrated by the examples and not yet asserted from a case file.

**The React bundle is optional, and the site is honest when it is missing.**
`cargo xtask docs` notes its absence and carries on, because the Rust half of this
repository owes nothing to Node (`NOCTUA.md §9.11`). Without it the implementation
switch reports a failed load and the server-rendered Dioxus example keeps working.
The switch appears **only** on a frame that can honour it — one that offers it and
cannot is a control that lies.

**The owl model is NonCommercial.** `assets/vendor/owl/` is CC BY-NC 4.0. It lives in
the documentation site only, never in `dist/`, and the day anything documented here is
sold it is the first thing that has to go. `assets/vendor/owl/ATTRIBUTION.md` carries
the terms.

## How it is built

```sh
cargo xtask docs                                          # writes docs-site/dist/
pnpm --filter @noctua-world/docs-site run build:example    # rebuilds the React bundle
```

`cargo xtask docs` is an ordinary Rust binary. No `dx`, no wasm toolchain, no
JavaScript runtime — it renders `rsx!` to a string with `dioxus-ssr`. The output is
static files any server can serve, and `docs-site/dist/` is **not** committed: several
megabytes of copied palette stylesheets, all reproducible from sources that are.

**The page is correct before any script runs.** `assets/shell.js` adds behaviour to a
complete page; it never constructs one.

**Every document sits at the same depth** (`<locale>/<slug>/index.html`), which is
what makes `site::BASE` a constant and every relative href resolve identically after
the router swaps a page in. `site.rs` asserts it.

**Order matters when you are proving something.** `cargo xtask build` rebuilds
`dist/`; `cargo xtask docs` rebuilds the site around it. A proof run against a stale
bundle proves nothing — build, then docs, then prove (`NOCTUA.md §19.5`).

## How it is verified

```sh
cargo xtask docs --verify     # build the site, then run the whole suite
pnpm run test:browser         # the suite alone, and it prints its own count
```

Not a list of things somebody once checked: a suite that fails the build. **The count
is deliberately not written down here** — it was once, and it was wrong within a
stage. The command above prints it, and `tests/browser/` is the list.

It uses the system's Chrome (`NOCTUA_CHROME` overrides the path) rather than
Playwright's download, which cannot find its libraries on a Nix system and exits
before Playwright can say why. It needs `pnpm install` first.

**Every colour in the suite is read from screenshot pixels** decoded in a canvas,
never from `getComputedStyle` — the same rule the `color_read_from_dom` gate imposes
on the shell. The places that read a serialised colour do so only to paint it, and
carry an `allow-literal` saying so.

What it covers, by kind rather than by claim — each file in `tests/browser/` is named
after the property it holds:

| | |
|---|---|
| **Theme and scope** | a scope resolves the semantic contract against itself, a nested scope keeps its parent's palette, a frame is sealed from every scope but the one it sits in |
| **Colour, measured** | borders one step from their own surface, patterns against real glyphs at 4.5:1, every palette-and-appearance pair against `WCAG 1.4.11` |
| **Geometry** | overlap, overflow, clipping, touch targets, reachability and rhythm, at four widths in three locales |
| **No-script honesty** | the page whole with JavaScript blocked, the default palette applied, the drawer released by a media query alone |
| **Parity** | React's markup against what Dioxus server-rendered, and both layout engines against one blessed case file |
| **Payload** | zero React requests on a page view, one on the first toggle; the message catalogue held out of the bundle |
| **Fonts** | every script rendered with a loaded face, and a Latin page fetching none of the others |
| **Language** | every text run bracketed in `qps`, and the sidebar collated rather than sorted by codepoint |

**Before you add a guard here, read `NOCTUA.md §19.5.** It is the list of ways an
assertion in this suite has gone vacuous — and the rule that a new guard is confirmed
by restoring the defect it was written for, in the artefact that ships, and watching
it fail.

## Where the rest of this file went

This file used to carry 107 numbered entries — 3,100 lines narrating bugs that had
already been fixed. `NOCTUA.md §0.5` says why that is the wrong place for them and
where each kind of fact belongs instead:

- **the rules that still bind** moved to [`../AGENTS.md`](../AGENTS.md) under
  *Gotchas*, and the ones about how a test goes quiet moved to `NOCTUA.md §19.5`;
- **the evidence for an assertion** belongs in the comment beside that assertion, and
  the specs in `tests/browser/` carry it;
- **the narratives** were deleted. Each is in the commit that produced it, dated and
  attached to its own diff.

To read one:

```sh
git log -S'<term>' -- docs-site/README.md docs-site/PROVISIONAL.md
```
