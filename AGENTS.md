# noctua-design — Agent Guidelines

> ## Everything you write to disk is in English.
>
> Code, comments, identifiers, documentation, CLI help, error messages, test
> names, commit messages, the docs site — all of it, always, whatever language the
> developer writes to you in. You may *reply* in Portuguese; you may not *produce
> artifacts* in it. This diverges from the rest of the `noctua-*` fleet, which is
> pt-BR. The divergence is deliberate; do not "correct" it.
>
> **UI strings are the one exception.** They are i18n resources and must exist in
> **both** English and Brazilian Portuguese, in `spec/i18n/{en,pt-BR}.toml`. The
> keys and the code around them are English; the values exist in both, and a key
> missing from either locale fails the build (`NOCTUA.md §9.6`).

## Before anything else: are you in the right place?

This repository lives inside **`noctua-workspace`**, and agents must be started at
the workspace root — not here. From inside this repository the other repositories
are invisible, so an agent cannot read the shared color contract, edit a
design-system component, or consult the master technical reference.

Run this **once, at the start of the session, before any other work.** Do not
re-run it per task.

```sh
d=$PWD; found=
while :; do
  [ -f "$d/.noctua-workspace" ] && { found=$d; break; }
  [ "$d" = / ] && break
  d=$(dirname "$d")
done
if [ -z "$found" ]; then
  echo "NOCTUA_WS=outside"
elif [ "$found" = "$PWD" ]; then
  echo "NOCTUA_WS=root"
else
  case $PWD in
    "$found"/repos/*) echo "NOCTUA_WS=inside-repo:$found" ;;
    *)                echo "NOCTUA_WS=inside-workspace:$found" ;;
  esac
fi
```

- **`root`** → proceed silently. Print nothing.
- **`inside-repo:…`** → print **ALARM A** below verbatim, stop, and ask whether to
  continue or abort. Touch nothing until answered.
- **`outside`** → print **ALARM B** below verbatim, and stop the same way.
- **`inside-workspace:…`** → not an alarm. Print the one-line correction, then
  continue, resolving paths against the workspace root.

**ALARM A — wrong starting directory:**

```
🚨🚨🚨 STOP — WRONG DIRECTORY 🚨🚨🚨

⛔ THIS AGENT WAS NOT STARTED FROM THE NOCTUA WORKSPACE. ⛔

YOU ARE INSIDE A SINGLE REPOSITORY. FROM HERE I CANNOT SEE THE OTHER
NOCTUA REPOSITORIES, SO I WILL NOT BE ABLE TO READ THE SHARED COLOR
TOKENS, EDIT A DESIGN-SYSTEM COMPONENT, OR CHECK THE MASTER TECHNICAL
REFERENCE. I WILL GUESS, AND MY GUESSES WILL BE WRONG. 🙈

✅ WHAT TO DO: STOP WHAT YOU ARE DOING, CLOSE THIS AGENT, AND REOPEN IT AT:

    /home/noctua/main/prjs/noctua-workspace

❓ DO YOU WANT ME TO CONTINUE ANYWAY, OR ABORT THE CURRENT WORK?
   I WILL NOT TOUCH ANYTHING UNTIL YOU ANSWER. 🛑
```

**ALARM B — repository outside the workspace:**

```
🚨🚨🚨 STOP — REPOSITORY IN THE WRONG PLACE 🚨🚨🚨

⛔ THIS REPOSITORY IS NOT INSIDE THE NOCTUA WORKSPACE. ⛔

I SEARCHED EVERY PARENT DIRECTORY AND FOUND NO `.noctua-workspace`
MARKER. THIS CLONE IS SOMEWHERE IT SHOULD NOT BE, SO NOTHING SHARED —
COLOR TOKENS, THE DESIGN SYSTEM, THE MASTER TECHNICAL REFERENCE — IS
REACHABLE FROM HERE. 🙈

✅ WHAT TO DO: STOP, AND RE-CLONE THIS REPOSITORY INSIDE:

    /home/noctua/main/prjs/noctua-workspace/repos/

   THEN REOPEN THE AGENT AT THE WORKSPACE ROOT, NOT INSIDE THE REPOSITORY.

❓ DO YOU WANT ME TO CONTINUE ANYWAY, OR ABORT THE CURRENT WORK?
   I WILL NOT TOUCH ANYTHING UNTIL YOU ANSWER. 🛑
```

**The correction (not an alarm):**

```
⚠️ Started in a workspace subdirectory, not at the workspace root.
   Working from <path> instead. Everything is reachable, so continuing.
```

These texts are authored once, in `NOCTUA.md §3.6`, and copied here verbatim so
they cannot drift. If one changes, change it there and propagate.
## What this is

**The shared design system**: tokens and components, shipped as **two packages
that must stay behaviourally identical** — a Dioxus crate (`crates/noctua-design`)
and a React package (`packages/react`) — plus a documentation site.

It is the **only** place a Noctua component may live. `noctua-central` never
writes a one-off component: if one is missing or inadequate, it is fixed here,
properly, and every consumer gets the fix. `NOCTUA.md §9.2`.

**It never defines a color.** Not a hex, not an `oklch()`, not "just this one
shade". Colors come from `noctua-colors`, vendored into
`packages/tokens/vendor/`, and a missing color is a one-line change *there*,
discussed first. `NOCTUA.md §8.8`. A source gate enforces this rather than
trusting it.

## Status

**Stages 4 and 5 — the theme runtime, the language layer and the site.**

What exists: the token compiler, the font layer, the source gates, the theme
model, the message catalogue, twenty-five components in both packages, and a
documentation site pre-rendered in three locales. `cargo xtask check` is still
the single gate, and `cargo xtask docs` builds the site.

**A component whose body the contract cannot describe gets its own emitter.**
`props.rs` derives a body from three facts — the element, the root class, and
the attribute each prop becomes — which covers every text component. A control
that renders another component inside itself, switches its own element, or
wires ARIA that is not a one-to-one mapping of a prop does not fit, and
`crates/noctua-design-emit/src/controls.rs` writes its body instead. The PROP
SURFACE is still generated from the spec, and both bodies are written in one
file side by side, so the two implementations still cannot drift.

**A prop may choose the ELEMENT, and that is declared.** `elements = [...]`
beside `values` in `spec/components/<name>.toml`, position by position. Two
props needed it and neither had it, so both were inert: `Heading.level`
rendered `<h2>` whatever it said, and `List.kind` rendered `<ul>` while its own
documentation promised otherwise. Declaring it reaches both implementations and
the documentation page at once. At most one prop per component may, and a prop
that switches the element carries no attribute.

**`required` on a prop is not documentation.** It removes the `Option` in Rust
and the `?` in TypeScript, which is how `IconButton` cannot be built without an
accessible name — the defect it exists to make unrepresentable.

**A `bool`'s default is a VALUE.** `#[props(default)]` is `false` however the
spec spells it, and it was emitted for every bool while React read the same
spec and wrote `marker ?? true`. `Disclosure`'s chevron was never drawn and
every code block shipped `data-nd-copy="false"`. Drift the *generator*
produced, which is why neither implementation could be reviewed into catching
it; `a_bool_default_reaches_both_languages` is the guard.

**Every example on a page is inside ONE frame.** `NOCTUA.md §9.10` calls for
one abstraction hosting every example; it existed and hosted exactly one on the
whole site, while a hundred and ten component examples sat in bare figures
beside it. A page has one frame now, and its width control resizes the whole
set — which is the question a reader has. The implementation switch appears
only on a frame that can honour it, which today is `ThemeScope`'s.

**One page per component, generated — and every one has worked examples.**
`crates/noctua-design-emit/src/docs.rs` compiles `spec/components/*.toml` into
`docs-site/src/generated/catalogue.rs`, and the site builds its pages, its
sidebar and its search index from that. The variant grid on each page is
derived from the contract, so every value of every enum prop appears by
construction. What a grid cannot show is a component *used* — thirteen
components have no enum prop at all — so `docs-site/src/examples.rs` carries a
hand-written table per component, rendering the real component with the call
that produced it as its caption. `every_component_has_examples` and
`the_examples_reach_every_prop` fail the build for a component or a prop that
has none.

**`ThemeScope` has a page and no contract file.** Its props ARE
`spec/theme.toml`'s scopable settings, so a `spec/components/theme-scope.toml`
could only disagree with the theme model. Its page is therefore written rather
than derived — the one exception — and it is in the component block of the
sidebar with the rest, because it is a component. It is also the only page with
a rendered scope on it, which makes it what the browser suite measures the
theme layer against.

**Reader-facing prose in a component's contract is DERIVED into the
catalogue.** Its guidance, every prop's `doc`, every state's, the keyboard
contract and the a11y notes are English in `spec/components/*.toml`, where they
also become the doc comment — and the site used to print those same strings on
the page in every locale. `component::derived_messages` turns them into
messages, `i18n::load_for` merges them, and the cross-locale check then demands
a Portuguese answer for each. English is written once; Portuguese is written
once, in `spec/i18n/pt-BR.toml`.

The site's own scope and limits are in `docs-site/PROVISIONAL.md`. Read that
before adding a page.

**One component is drawn in three dimensions, and one component ships its own
behaviour.** `Lineage` places figures and the relationships between them in real
space — perspective, a slab with a measurable thickness, planes that recede —
out of **elements rather than a canvas**, which is what `NOCTUA.md §4` chose
`CSS3DRenderer` for. Three things about it are worth carrying:

* **Its structure is a text prop.** A Dioxus component cannot look inside its
  own children, so a graph handed over as children could never be laid out —
  the wall `CodeBlock` hit over line numbers. `crates/noctua-design-lineage`
  parses it and places it, in Rust during pre-rendering and in `layout.js` for
  React, and **neither half is checked against the other**: both answer to
  `spec/lineage/cases.toml`, which is the first `.cases` file here.
* **The grid is isotropic, and that is a constraint.** A taut connection is
  drawn from a rotation computed in cell space; render that in a grid whose rows
  are a different length from its columns and the line stops reaching the two
  things it joins. One `--noctua-scene-cell` across, down and back.
* **`overflow`, `opacity` and `backdrop-filter` flatten a 3D context.** All
  three are kept off the scene and off a piece — the clipping is on the wrapper
  above, the fog on the leaves below, the glass on the card. Getting it wrong
  looks like a z-order bug and is not one.

`dist/lineage.js` is the first behaviour this repository ships, beside the
bootstrap it already shipped. three.js is **vendored** (`assets/vendor/three/`,
235 KB, 53 KB gzipped, MIT, attributed) because CI runs `cargo xtask check` with
no Node at all — and it is fetched on the first interaction rather than on load.

**Every demonstration on the site is inside an `ExampleFrame`, one per page**,
and the frame documents itself at `/example-frame/`. It is a docs-site
component, deliberately: a viewport rig and a lazily-imported React bundle are
documentation apparatus, not something a design system ships. What it is *built*
from is the components above.

**THAT LINE IS DRAWN PER FEATURE, NOT PER FILE, AND `Toast` IS THE WORKED
CASE.** Its timed example emptied itself after six seconds and needed two
things. The COUNTDOWN is the component's — `countdown_label`, drawn by
`dist/toast.js` — because a message with a deadline owes its reader the deadline
wherever it is used, and a docs-site copy is one no consumer could have. Raising
the same message again is the job of the page that raised it, so *show it again*
is a `Button { action }` answered in `shell.js` and ships nowhere. Ask which of
the two a consumer would want, not which file is easier to reach. The countdown
also shows the WHOLE wait again whenever the wait restarts, because "the wait
never shortens" is a guarantee a resuming number would contradict.

**AN EXAMPLE IS FICTIONAL, AND `Search`'S USED TO BE THIS SITE.** Every row of
every `Search` example was a real page or component of this documentation, so a
reader could not tell the demonstration from the application. They are an
invented seed catalogue now, in one fragment
(`docs-site/src/trees/search.rs::CATALOGUE`) that six examples and the hero
share. The varieties are proper names and cross untranslated under
`nd-identifier`, which is what lets a pre-filled query like `san` match in every
locale; every sentence in them is a message in both. `src` is the one prop still
pointed at this site and says why in place: its subject is FETCHING an index,
and a fictional file would be untranslated content with no catalogue behind it.

## Repository map

```
spec/                   the only files a developer edits
  tokens.toml           every non-color token (Leg 1 of the parity mechanism)
  i18n/{en,pt-BR}.toml  every UI message, both locales
  lineage/cases.toml    the blessed layouts BOTH engines are checked against
  components/           <name>.toml (contract). <name>.cases.toml — Leg 3 of the
                        parity mechanism — is NOT written yet, and neither is
                        tests/conformance/. What holds parity today is that both
                        implementations are generated from one file
crates/
  noctua-design/        the Dioxus package. src/theme.rs is GENERATED
  noctua-design-tokens/ GENERATED from spec/tokens.toml. COMMITTED
  noctua-design-theme/  GENERATED from spec/theme.toml. COMMITTED
  noctua-design-i18n/   GENERATED from spec/i18n/. COMMITTED
  noctua-design-lineage/ the `Lineage` engine. HAND-WRITTEN, and `layout.js`
                        beside `layout.rs` is the same algorithm again, with
                        `runtime.js` — the camera — beside both
  noctua-design-icons/  GENERATED. COMMITTED
packages/
  react/                the React package. src/theme.js is GENERATED
  tokens/               GENERATED (TypeScript). COMMITTED
  tokens/vendor/        the color tokens, vendored from noctua-colors
assets/fonts/           copied from .TEMP/, + ATTRIBUTION.md + licences
dist/                   GENERATED, COMMITTED: the token layer, the font faces,
                        the theme and appearance layers, the scoped semantic
                        contract, and `lineage.js` + `vendor/` — the one
                        behaviour this repository ships. NOT the component
                        styles: those live in docs-site/assets/shell.css and
                        ship nowhere yet
docs-site/              the docs app. src/ and assets/ are written; dist/ is
                        build output and is gitignored. Read PROVISIONAL.md
tests/conformance/      one case set, run against both implementations
xtask/                  the five verbs
.TEMP/                  staging area. Copy OUT of it; never modify it
```

**Generated directories are committed** so every consumer works with no build
step, exactly as `noctua-colors` commits `system/`. `cargo xtask check` fails if
one drifts from `spec/`.

**`build` writes changed files; it does not delete stale ones.** Removing a
component means deleting its generated `.rs`, `.js` and `.d.ts` by hand as well
as its `spec/components/<name>.toml`. The drift check compares what the emitter
produces against what is on disk and reports a file that is *missing* or
*different* — it has nothing to say about a file nothing produces any more.

**The controls are components, and there is one of each.** Every interactive
thing on the documentation site is `Button`, `Segmented` (+ `SegmentedItem`),
`Switch`, `Toolbar` (+ `ToolbarGroup`), `ButtonGroup`, `Swatch`, `Disclosure` or
`Search` (+ `SearchGroup`, `SearchResult`). A boolean is a `Switch` — `role="switch"` — never a `Segmented`
of "Off" and "On": a radio group asking a yes-or-no question tells a screen
reader the wrong thing about what pressing it means. `Choice`, `ChoiceOption` and `IconButton` were deleted:
`Button { hide_label }` is what `IconButton` existed for, and a second component
for one control is the duplication this repository is against. If a page needs a
control, the answer is never a `<button>` with a class on it.

**A script hook is a prop.** `Button.action`, `Segmented.name`,
`Disclosure.name` and `Search.name` are the only way markup gets an
attribute a script can find. There is no arbitrary-attribute escape hatch,
because an escape hatch on one component is an escape hatch on all of them.

## Commands

```bash
cargo xtask build     # compile spec/ into tokens, icons, props types and dist/
cargo xtask check     # THE gate: spec, source gates, drift, fmt, clippy, tests
cargo xtask dev       # build, serve on :8080, and rebuild on any change under spec/ or docs-site/
cargo xtask docs      # pre-render the site into docs-site/dist/, one build per locale
cargo xtask docs --verify   # …and run the browser suite against it
cargo xtask release   # prepare a version
```

> ⚠️ **`cargo xtask dev` is the code it started with.** It is a long-running
> `target/debug/xtask`, so editing an emitter does not change the emitter it
> uses. It used to regenerate `dist/` from that stale binary — silently
> reverting a fix already made, tested and committed, and leaving `check`
> reporting files out of sync with nothing to explain it. It happened three
> times while this repository was being rebuilt.
>
> It now watches `crates/` and `xtask/src` too, and **stops rebuilding** with a
> loud message rather than rebuilding wrongly. Restart it and it picks up the
> new emitter. If you ever see something "change back on its own", check
> `pgrep -af "target/debug/xtask"` anyway — an older binary predating this will
> still do it.

**Five verbs, and the surface does not grow.** New capability goes behind an
existing verb or it does not ship. `cargo xtask check` is exactly what CI runs —
the same command, not a reimplementation in YAML, which is precisely how the two
drift until "passes locally, fails in CI" becomes a normal sentence.

`just <verb>` is a one-word alias for each and is never the only path.

## Invariants

Breaking any of these is a defect, not a tradeoff. Each is enforced by a gate,
because a convention nothing enforces is a convention that decays.

1. **No color is defined here.** Every color comes from a `--nc-*` token in the
   vendored `noctua-colors` output. A hex, an `oklch()` or a named color in this
   repository's source fails `check`.
2. **No token literal.** No raw `px`, `rem` or `ms` in component source. Spacing,
   radii, type sizes, durations and easings come from the token layer, so a
   change to the scale reaches every component.
3. **No font family named directly.** Use `var(--noctua-font-*)`. A component
   that hardcodes `"Noto Sans"` cannot honour the interface-font setting and will
   not fall back correctly. `NOCTUA.md §7.4`.
4. **No Tailwind arbitrary value.** `p-[13px]` is a token literal wearing a
   different hat.
5. **No color token inside a custom property.** `--x: … var(--nc-…)` freezes to
   the palette of the element it is declared on, so anything reading it inside a
   `ThemeScope` paints the page's colors. Write the color at the point of use.
   See Trap 0.2 — this one was live in this repository.
6. **No color read back out of the DOM.** `getComputedStyle(…).color` returns
   whatever the engine chose to serialise; comparing two of those strings proves
   nothing about the pixels. Paint into a 1×1 canvas and read `getImageData`.
   Reading a *length* that way is fine.
7. **No translated sentence built from fragments.** Word order, gender and
   agreement differ between languages. One sentence is one key, with a
   `{placeholder}` where a value goes. `NOCTUA.md §9.9` rule 1.
8. **Generated files are never hand-edited.** Change `spec/` and rebuild.
9. **The Rust side builds and tests with no Node installed.** `check` proves it
   rather than assuming it. Tailwind runs from its standalone binary so a
   Rust-only contributor never installs a JavaScript toolchain.
10. **Both implementations change together.** A component is four things — the
   spec, the Dioxus implementation, the React implementation, and the shared
   cases — and editing one implementation only is a compile error or a failing
   conformance test, by design. `NOCTUA.md §9.3`.
11. **No sentence reaches a reader untranslated.** Not a gate: a test. The
   pseudo-locale brackets every message, so
   `every_sentence_on_every_page_is_translated` renders each page in `qps` and
   asserts every text run carries a bracket. Anything deliberately untranslated
   — a token name, a component name, a language's own name, a line of
   Devanagari — is marked `.nd-identifier` and skipped, which makes that class
   the page saying *this is a name, not a sentence*.
12. **Every component has worked examples.** `every_component_has_examples`
   fails the build for one that does not, and `the_examples_reach_every_prop`
   fails it for a prop no example sets. A page that documents a component
   nobody has shown in use documents its prop table.
13. **A prop that does nothing is a defect, not a stub.** Three shipped:
   `Heading.level`, `List.kind` and `CodeBlock.numbered`, each accepted,
   documented and dropped. Two are fixed by `elements` in the spec; the third
   was deleted, because CSS cannot number the lines of a text node and a prop
   that cannot work is worse than a missing one. `numbered` is back, and the
   note it left behind is why: it said re-adding it needed a `code` **text**
   prop first, "and that is a different component". The rewritten `CodeBlock`
   is that component — it reads its own content, so it can split it into lines
   and a counter has something to count.

   **`List.levels` is the same test applied before shipping.** It colours the
   depth of a nested list, and colouring the marker alone would have been inert
   under `marker: none` — a prop that does nothing under another prop. So it
   marks the marker *and* the rule down a nested list's inside edge. Two more
   things about it generalise: the depth comes from the SELECTOR
   (`.nd-list[data-nd-levels="true"] .nd-list .nd-list`) because a nested
   `List` is its own instance and carries none of the root's attributes, and
   the four colours alternate the palette's neutral and its accent, which are
   the only two hues thirty-nine palettes all have. **A bool has no variant
   grid** — `specimens::drawn` takes enum props only — so the example is the
   only place it is shown, and a `specimen` written on a bool prop is a
   picture nothing draws.

**The escape hatch.** Every gate honours `// allow-literal: <reason>` on the same
line, and the reason has to be a real one. It exists so an exception lands in
front of a reviewer instead of quietly disabling a gate.

## Design rules that are not preferences

From `NOCTUA.md §9.7`, and they bind every component and every page:

- **Very clean.** Simple, objective, nothing decorative.
- **Mobile friendly from the first commit**, not as a later pass.
- **Text and components sit directly on the background. Do not use cards.**
  `--nc-color-surface-raised` exists for the rare case that genuinely needs
  separation; reaching for it is a decision to justify, not a default.
- **Generous spacing, one scale**, with a stated base step. Siblings share
  spacing, padding and radius — a row of controls that disagrees on padding
  reads as broken.
- **Large default title and body sizes.**
- **A narrow default measure**, whatever the screen. `--noctua-measure` caps
  prose in `ch`. This is not a max-width on the page: the page may be wide, the
  *line* may not.
- **No layout shift, no flickering.**

## Gotchas

### Themes are scopable, and that is what shaped this repository

**A `var()` resolves where it is DECLARED, not where it is read.** The color
system emits its whole semantic contract — `--nc-color-surface`, `-fg`,
`-border`, 1,767 names — once, at `:where(:root)`. Give a subtree
`data-palette="amber-vivid"` and it gets amber's `--nc-neutral-*`, but
`--nc-color-surface` still holds the **root's** value, because that is the
element the indirection was declared on. A themed component painted from the
semantic contract silently shows the page's colors, and every token still
resolves to something so nothing looks broken.

`dist/scope.css` is the fix: the same contract, re-emitted under
`:where(:root), [data-noctua-scope], [data-palette]:not(:root),
[data-theme]:not(:root)`. It **replaces** the vendored `contexts.css` in the base
link set — linking both doubles the cost for nothing.

**Every theme attribute goes on ONE element.** A theme file's forced-mode rule is
compound and same-element:

```css
[data-palette="blue-vivid"][data-theme="dark"] { … }
```

Split the palette onto a wrapper and the mode onto a child and that selector
matches nothing: the child falls through to the *default* palette's unanchored
`[data-theme="dark"]`. Which is why `scope_attributes` always emits the palette
when it emits anything, even when the palette did not change.

**`system` is the absence of `data-theme`, never a value of it.** The cascade
resolves the system preference before anything paints; the word is something no
stylesheet understands.

**Taking a scope OFF means removing every attribute, and the list is
generated.** `scopeAttributeNames` (JavaScript) and `SCOPE_ATTRIBUTES` (Rust)
publish it, from the same table `scopeAttributes` iterates. Removing a subset is
not a partial un-theming — it is a different, worse bug: `dist/scope.css` matches
`[data-palette]:not(:root)`, so a leftover `data-palette` **on its own** keeps
the whole contract resolving at that element, now against the system preference
rather than the page, and the subtree cannot be un-themed at all. That is
exactly what a hand-written teardown of two attributes did to the example
frame's appearance toggle.

**A frame's parts are its own, and a nested frame is not part of its parent.**
`docs-site/assets/shell.js` resolves every one of a frame's controls through the
nearest `[data-nd-frame]`, because the frame's own documentation page puts one
inside another. A `frame.querySelectorAll(…)` there reaches into the nested one
— the outer frame owned ten width radios and two stages — and a control that
marks a stop it did not set is a control that lies.

**THE EXAMPLE FRAME'S CONTROLS ARE A RAIL, AND THEIR AXIS CHANGES.** They sit
in a `Toolbar.nd-frame-bar` that is an ordinary **static** grid item in column
two of `.nd-frame` — in flow, so the floating-surface allow-list does not apply
to it — and below the `sm` step the grid collapses to one column and the rail
lies down into a row under the stage. `Toolbar` writes `aria-orientation` and
`shell.js` reads it to choose the arrow keys, so **the markup cannot name the
axis**: it ships `horizontal`, the ARIA default and true at the narrow width a
script-free reader is likeliest to be at, and `railAxis()` in `shell.js`
upgrades it to `vertical` when the stylesheet stands the rail up. That function
reads the breakpoint off `--noctua-breakpoint-sm` rather than writing `30rem` a
second time. The frame's five width stops are one `Button { menu }` for the same
reason: five stacked stops are 150px, taller than half the frames on `/button/`.

**WRITE THE RAIL'S LAYOUT AS `.nd-toolbar.nd-frame-bar`, NOT `.nd-frame-bar`.**
`.nd-toolbar[aria-orientation="vertical"]` and
`.nd-toolbar[data-nd-density="compact"]` are each one class and one attribute,
so a rule written against the single class loses to them — and loses silently.
A `gap` and an `align-items` both did: the gap looked like it was working
because the case being measured fitted anyway, and the alignment left every
glyph hard against the rail's leading edge, which is the first thing a reader
said about it. **A declaration that loses on specificity is indistinguishable
from one that works, if the thing you measure would have passed regardless** —
so assert the geometry (where the control's centre line is) rather than the
property. And when a scripted edit changes a stylesheet, read the value back out
of `docs-site/dist/assets/shell.css`: a `replace` with a count of one matches the
first occurrence in the file, not the one you meant, and it moved a floating
panel's padding here.

**THE INSET INSIDE A FRAME IS ON `.nd-example`'S PARTS, NOT ON THE STAGE.**
`.nd-frame-stage` is a scroll container, so nothing can be pulled out to its
edges with a negative margin — content overflowing a scroller's start edge is
clipped and unreachable. The caption's header band needs those edges, so the
padding lives one level down, which works because `.nd-glass-scroll` is
`display: contents` at rest and each example is already a direct flex item of
the stage. **Under the glass rig that scroller becomes a real box**, and its own
`padding-inline` used to double the inset there; it is zero now, and adding one
back moves every example across the colour field.

**A PIXEL TEST WHOSE RESULT DEPENDS ON WHERE SOMETHING LANDS reports the next
unrelated layout change as a colour defect.** `glass.spec.js`'s "a frosted white
control must stay white" was sampling a control that happened to sit in the
rig's blank; the header band grew the content above it by a few pixels, nine
coloured shapes moved behind it, and a geometry change was reported as paint.
When the claim is about a control's own paint, take the backdrop out of the
picture rather than hoping it stays where it was.

**AN ICON IS FETCHED, NEVER DRAWN.** `spec/icons.toml` says to fetch the
upstream SVG at the pinned tag; do that, with `curl`, and diff it against what
is already vendored if you are replacing one. A glyph reproduced from memory is
a file that claims a provenance it does not have — `image.svg` was one, and it
was an older variant of the same drawing.

**AND `Icon` IS A COMPONENT IN THE CATALOGUE NOW, NOT A MODULE BESIDE IT.** It
was generated by `crates/noctua-design-emit/src/icons.rs` into
`crates/noctua-design/src/icon.rs` and `packages/react/src/icon.js`, which put
one component outside the catalogue: no props table, no variant grid, and
absent from a site whose search is by prop. It is `spec/components/icon.toml`
with its body in `controls.rs` now, and those two files are deleted.

**It was MOVED and not copied, and that is the whole of it.** A contract added
beside that emitter would have produced two components called `Icon` in one
crate — `components::Icon` and `icon::Icon` — with `lib.rs`'s explicit
re-export choosing one and nothing reporting the other. Not a compile error. A
silent divergence. `lib.rs` therefore has no `pub mod icon` at all, `Icon` and
`IconSize` arrive through `pub use components::*` like every other component,
and `IconName` comes from `noctua_design_icons`, where the drawings are.

What the icon emitter still owns is what is about the DRAWINGS rather than the
component: `IconName`, `dist/icons.json`, the attribution, `packages/tokens`,
and `STROKE_WIDTH` — read by both bodies, so `spec/icons.toml` decides the one
number either of them draws. A variant grid draws its icons through `Icon`
itself now, so there is no third reader to keep in step. `/icons/` remains the
gallery of drawings; `/icon/` is the component that draws them.

**AND THE GALLERY IS CALLED *Icons list*, because *Icons* was not a name it
could keep.** Two sidebar rows a letter apart, neither saying which was the
component and which the set of drawings. It is a foundation, so the title is a
sentence and both locales carry it — `Lista de ícones` in Portuguese. The slug
stays `icons`, for the reason `/avatars/` keeps its own: a URL a reader may
have kept is not worth a rename, and nothing derives the title from it.

**Its three size fields went with it, because they decided nothing.**
`size_small`, `size_medium` and `size_large` reached one doc comment and no
declaration, and had already drifted from the stylesheet, which sizes
`.nd-icon-glyph` from the type scale. `spec/icons.toml [meta]` is `stroke_width`
alone.

**A POSITIONED ELEMENT PAINTS ABOVE A STATIC SIBLING**, and `.nd-frame-glass` is
positioned. Anything a frame paints that must sit ON the frost — its bar, its
stage — needs a position of its own, unconditionally. Three separate regressions
came from giving it to one of them and not the other. And `relative` inherits
whatever `inset-*` was written for `sticky`: cancel it.

**FROST A LAYER, NEVER A BOX THAT CONTAINS A FLOATING PANEL.**
`backdrop-filter` makes an element the containing block for every `position:
fixed` descendant, so frosting a frame or a header re-anchors any menu inside it
— and if that box also clips, the menu opens where nobody can see it.
`.nd-header-glass` and `.nd-frame-glass` are layers for this reason and no
other.

**WHICH PAGE A COMPONENT IS DOCUMENTED ON IS IN ITS CONTRACT.** `[meta] page`
names a collection — several components on one page — and absent means a page of
its own. `site::pages` panics on a value no page declares, because the failure
mode is silent: a component documented nowhere looks exactly like one that
moved. Never keep a list of which components moved anywhere else.

**AND THREE THINGS MOVE WITH IT, EACH KEYED BY SLUG IN A DIFFERENT FILE.**
`examples::OVERVIEWS` holds the caption of the page's one hero, `trees::HEROES`
holds its tree, and `tests/browser/pages.js` holds its URL. A merge leaves an
entry in each pointing at a slug that is no longer a page, and nothing reported
it: the lookup that misses returns `None` and the page simply opens with no
example on it. `every_overview_leads_a_page_that_exists` and
`every_hero_leads_a_page_that_exists` are the two gates, added when `ListItem`,
`SegmentedItem`, `SelectGroup` and `SelectOption` moved onto the pages of the
components they are part of.

**A COLLECTION PAGE PUTS EVERY COMPONENT ON IT INTO THE SEARCH INDEX.**
`index::all` used to read `page.component`, which is `None` for a shared page —
so `TableColumn`, `SelectOption` and every prop either declares were absent
from the site's own search, and `site.rs` wrote that down as the price of
merging. It reads `site::documented_on` now and dedupes on name-and-page,
because two parts of one collection declare `children` and `label` and each
would otherwise be its own row pointing at the same place.

**A PICTURE OF A LIST CLAIMS NO ROLE, AND THAT IS THE RULE ABOUT PICTURES
AGAIN.** `audit.spec.js`'s list sweep asks every `ul.nd-list` to restate
`role="list"`, and a specimen whose contract carries MARKUP is the contract's
element with that markup inside — it deliberately carries no ARIA at all. The
sweep skips `.nd-variant-stage` for the same reason `segmented.spec.js` does.
Adding a page to `pages.js` is what surfaced it: five whole-site sweeps only
ever open the pages named there.

**A ROW OF BOXES HAS ONE WALK, AND IT IS `segmentWalk`.** Auto-advance when a
box fills, Backspace in an empty box going back, and a paste splitting itself
across the row are the same three gestures a `TimeInput` and an `OtpField`
need. There is one implementation, in
`crates/noctua-design-runtime/src/inputs.js`, and each control keeps only what
is not shared: the arrow step and the zero padding are a time's, and replacing
a full box is a code's.

**AND AN OTP FIELD POSTS UNDER ONE NAME.** Every box carries the same `name`, so
the code arrives as that many values in document order for a server to join. One
name per box makes a server take apart something it never asked to be split, and
a hidden field carrying the joined code posts nothing at all when the script does
not run — which is the failure `TagPicker` already chose against.

**THE BROWSER BEHAVIOUR THIS SYSTEM SHIPS LIVES IN
`crates/noctua-design-runtime/`.** Hand-written files — `anchor.js`,
`color.js`, `messages.js`, `color-picker.js`, `inputs.js`, `carousel.js`,
`error-summary.js`, `table.js`, `info.js` — copied
into `dist/` by
`noctua-design-emit`'s `runtime.rs`, exactly as `lineage` copies the layout.
They arrive there as `include_str!` constants rather than as a path `xtask`
reads, so a file renamed out from under the emitter is a compile error naming
the file instead of an artifact that emits empty. `shell.js` imports every one of them
back from `assets/`; it does not carry a copy of any of them. **`anchor.js`
moved out of `shell.js` and its three old call sites did not change** — a
shipped panel cannot import from the documentation site, and a second copy of
the off-screen test is what the note it carries warned about.

**A SPINBUTTON CARRIES ITS VALUE AT REST, AND WHAT THE SCRIPT DREW CANNOT BE
READ BACK OUT OF THE BOX.** `NumberInput` shipped with ten defects and five of
them generalise to any control this runtime enhances:

* **`aria-valuenow` belongs at enhance time**, not inside the move. Written
  only on move it left every number field on the site a spinbutton with no
  current value until a reader touched it. `aria-valuetext` goes beside it
  wherever the drawing says more than the digits do: a currency, a unit, the
  separators `group` writes.
* **Formatting that runs on focus and blur alone does nothing at rest**, so a
  grouped field rendered `1499.9` until somebody entered it.
* **`Number("1.499,90")` is `NaN`.** A value the script formatted has to be
  REMEMBERED, with the text it drew for it, and trusted only while the box
  still shows that text. A stepper that re-parsed the box fell back to `min`.
* **`step` sets the FEWEST decimals drawn, never the most.** Rounding to what
  `step` implies is a rewrite: 7.5 in a field that steps by one is 7.5, and
  the module's own header forbids changing what a reader typed.
* **A control the script owns ships `hidden`**, and `readonly` refuses a
  stepper exactly as `disabled` does — in the markup and in the handler, since
  a value a press can still change is a value that is still editable.
* **`stepping` IS AN ARRANGEMENT, NOT A SECOND CONTROL.** `primary` puts one
  full-size stepper either side of the value and `assist` keeps the two small
  ones at the end of the box, and the MARKUP is identical: the wrapper becomes
  `display: contents` and `order` moves the one that subtracts in front. So the
  runtime finds the same two buttons, the same reveal and the same refusal. The
  `:not([hidden])` on that rule is load-bearing — `[hidden]` is one attribute of
  specificity, so without it the stylesheet reveals a stepper the markup shipped
  hidden and a page with no script draws two controls that move nothing.

**THE READER'S LOCALE IS NOT THE PAGE'S LANGUAGE, AND `document.documentElement.lang`
IS THE PAGE'S.** The site is pre-rendered per locale, so that attribute is `en`,
`pt-BR` or `qps` — never a region. A bare `en` has no region, so CLDR resolves it
to the United States, and both places that asked it a question about the reader
got the American answer for every reader of the English build: `DateInput` re-laid
the row to month-day-year, and `TimeInput`'s `auto` put everyone on a twelve-hour
clock. Both were served the right thing and moved off it by the script.

Split the question, because it is two questions:

* **A WORD follows the page.** Month names, weekday initials and the bound
  inside `min_label` are written into a sentence the page is already written
  in. `dateWords()` in `shell.js`.
* **A CONVENTION follows the reader**, and the reader is `navigator.languages`.
  The row's order, the week's first day and the clock are conventions.
  `readerLocales()` in `shell.js` and `twelveHourReader()` in `inputs.js`, both
  read through `Intl` — `formatToParts` for the order, `resolvedOptions().hour12`
  for the clock. **Never a table of locales to formats**: a table is a copy of
  CLDR that stops being true the day CLDR changes.
* **An empty chain is passed as an empty array**, which `Intl` reads as *the
  default locale* — the reader's own by definition.
* **`playwright.config.js` pins `locale: "en-US"`** for the same reason it pins
  the colour scheme and the device scale: the suite asserted month-first and
  would have asserted it on a machine set to en-GB too, because the defect made
  the answer independent of the machine. A spec that wants another locale opens
  its own context, which is what the three forced-locale tests in
  `date-input.spec.js` do.

**`TimeInput` IS TWENTY-FOUR HOURS UNLESS ASKED, AND ITS MERIDIEM CONTROL IS A
REAL `Segmented`.** `clock` has three answers — `24` (the default), `auto` (ask
the reader's browser) and `12` — and the morning/afternoon control used to be
`createElement` markup imitating `Segmented`: no `appearance`, no `shape`, no
`fill`, no `aria-orientation`, so every compound rule that draws a joined
control matched nothing and the pair rendered as two detached boxes. It is the
component now, rendered by both bodies inside the `hidden` span, revealed by
`inputs.js` when the clock resolves to twelve. Four things came with it:

* **The server decides which half of the day is chosen**, from `value`, and an
  empty field chooses NEITHER. A control that ships with the morning marked has
  answered for the reader, and a twelve-hour reading whose half nobody has
  stated is half a time — `readAll` reports it missing rather than posting the
  morning.
* **The form still posts twenty-four hours**, which the contract promises in
  three places and the re-laid field broke: a box posts what it SHOWS, so
  `hour=02` left for the server when the reader meant 14:00. The entry is
  rewritten on `formdata`, exactly as `NumberInput` rewrites the separators
  `group` draws.
* **The value is RENDERED into the boxes.** It reached them from
  `data-nd-value` at enhance time and nowhere else, so a field given `14:05`
  was two empty boxes until a script ran.
* **A name and both words, or no control at all**, and no re-lay without one.
  The script used to fall back to the raw values and draw `am` and `pm` — two
  identifiers, untranslated, on the page. `{part}` in `missing_label` was the
  same failure: it was filled with the part's key, so the Portuguese page said
  *O minute ainda está vazio*.

**AND `DateInput`'S BEHAVIOUR DOES NOT SHIP.** It is `enhanceDateInput` in
`docs-site/assets/shell.js`, not in `crates/noctua-design-runtime/`, so a
consumer taking the packages gets the three boxes and none of the reorder,
stepping, paste, suggestions or month grid. Same recorded gap as `Tabs`,
`Segmented` and `Tree`, and the same fix when somebody takes it: move the
module, and let `shell.js` import it back.

**`Tabs` REVEALS A PANEL AND `Segmented` CHOOSES A VALUE**, and both contracts
say so. `Segmented` is a form control with a `name`, a `value` and
`role="radiogroup"`; `Tabs` submits nothing and changes which region of the page
is on screen. `segmented.toml` had already refused `role="tablist"` on the
grounds that it promises panels that change and there were none — this is the
other half of that sentence, and neither is a prop on the other.

**And it claims no role until a script can keep one**, which is `Tree`'s rule
applied a second time. With no script there is no `tablist`, no `tab` and no
`tabpanel`, every panel is on the page, and each tab is a **link** down to the
one it names — a working table of contents rather than a row of buttons that do
nothing. The ids and `aria-controls`/`aria-labelledby` are rendered, because
both come from `name` and `value` and cannot go stale.

**THE ARROW WALK IS `stops()` AND `roving()`, AND NOW SERVES THREE ROLES.** A
toolbar, a radio group and a tab list are one implementation in
`docs-site/assets/shell.js`, with Home, End and the orientation written once.
The only place a tab differs is that selection may not follow focus: `manual`
moves and waits for Space or Enter, which is one branch in the handler rather
than a second walk. Adding a fourth role is a selector and that branch.

**`Tabs`, like `Segmented` and `Tree`, is enhanced from the site's script rather
than from `dist/`, and that is a recorded gap**, not a decision: the walk it
needs lives in `shell.js`, and a consumer taking `dist/tabs.js` alone would get
the roles without the keyboard they oblige.

**AND THE RENDERED STOP IS ON THE GROUP, NOT ON THE CHOSEN ITEM.** It was on the
item — `tabindex="0"` when `selected` and `-1` otherwise — which is one stop for
a radio group that has an answer and **zero** for one that does not. Every
`Segmented` nothing had chosen was unreachable by keyboard with the script
blocked, including three on `/theme-scope/` and the first example on
`/segmented-item/`. An item cannot fix it: whether the group has an answer is a
fact about the group, and no rendering order lets the first item know that the
fourth is chosen. So `Segmented` writes `tabindex="0"` on itself, every
`SegmentedItem` writes `-1`, and `refreshRoving()` strips the group's own stop
before it places a real one — which it must, because `stops()` collects
`[tabindex]` and a group inside a toolbar would otherwise join the bar's walk.

**A DEFAULT VALUE IS THE ABSENCE OF ITS ATTRIBUTE, SO SELECT THE VALUE A THING
IS NOT.** A variant specimen writes every enum default EXCEPT an `aria-*` one
(`specimens.rs` skips those on purpose), and a runtime that builds a control by
hand writes the class alone — `inputs.js` does, for `TimeInput`'s AM/PM pair.
So `[data-nd-appearance="joined"]` and `[aria-orientation="horizontal"]` each
matched nothing for a reader who was looking at the default. `:not(spread)` and
`:not(vertical)` are the same two sets with the defaults included, and they
cannot both match, which is what lets one axis override the other without a
specificity fight. `Segmented`'s vertical grid lost every corner to this and
the AM/PM pair came apart into two boxes with a doubled edge between them.

**THE CORNERS OF AN ATTACHED ROW BELONG TO ITS TWO ENDS, AND THE AXIS DECIDES
WHICH TWO EACH END OWNS.** Zeroing the leading and trailing pair for the
horizontal case without restricting it to that axis leaves a vertical group
running both sets: one rounded corner at the top left, one at the bottom right,
diagonally opposite, and squares everywhere else. Write each axis's end corners
whole, from one named radius, and a lone member keeps all four because it is
both ends. `Segmented` is fixed; **`ButtonGroup` still has it**, in the same
shape, at `.nd-button-group[data-nd-attached="true"]`.

**`max-inline-size: 100%` IS A PERCENTAGE OF ZERO INSIDE A SHUT FOLD.** A
closed `<details>` is `content-visibility: hidden`, so every containing block
in the settings panel measures zero and a percentage cap against it collapses
the control — and takes the panel's own width down with it. `audit.spec.js`
reported fifty thousand controls under the minimum target from one such
declaration. `inline-size: fit-content` is the same intent with a floor: the
content when it fits, the container when it does not, never below
`min-content`. Which is also why nothing in that rule set may give a part
`min-inline-size: 0` — that puts the floor back at zero.

**A CENTRED FLEX LINE THAT DOES NOT FIT OVERFLOWS BOTH ENDS.** A stop divided
by count rather than by content can be narrower than its own icon and word
together, and `justify-content: center` then paints the icon over the label of
the stop before it. `flex-wrap: wrap` on the box puts the icon on a line of its
own instead, which costs nothing at any width where the two fit side by side.

**A DECLARED `attribute` IS A CLAIM ABOUT THE MARKUP, AND `rust_element` KEEPS
IT ONLY FOR A DERIVED BODY.** A component written by hand in `controls.rs`
never reaches that function, so thirty-one props across eleven components
declared a `data-nd-*` no body wrote and no script read — while the site's
generated markup snippet printed each one as though it were there. Nothing
reported it, because the values do reach the DOM: as labels, as tab names, as
text. `every_declared_attribute_is_written_by_both_bodies` in `props.rs` is the
gate now, and it reads the emitted source so it covers both kinds of body.

**A RATING WRITES ITS NUMBER OUT, AND SHIPS NO SCRIPT AT ALL.** The interactive
form is real radios in a `<fieldset>`, so the group, the arrows and the
announcement are the platform's. The number beside the stars follows a reader's
own choice with nothing running: `value_label` is filled once per step, every
copy ships, and `input:checked + label + .nd-rating-count` reveals exactly one.
A fraction in the read-only form is drawn by clipping a filled copy of the row,
so there is no `star-half` file and no rounding — and the sentence is always
there, because a shape a reader has to estimate from is the defect `Meter` and
`Progress` both record.

**A `tone` IS A NAME FROM `dist/tones.css`, NOT AN ENUM.** The emitter
(`crates/noctua-design-emit/src/tones.rs`) reads the vendor's 349 semantic
families and writes one indirection block each — `--nd-tone`, `--nd-tone-bg`,
`--nd-tone-border`, `--nd-tone-on` — so a component's stylesheet is one rule
rather than one per name. Three things follow:

* **A `tone` prop is `text`.** A typo is no longer a compile error, so
  `docs-site`'s `every_tone_this_file_writes_is_one_the_palette_declares` reads
  `noctua_design::tones::ALL` and fails the build instead. Add a tone to an
  example and it is checked; add one to a consumer's code and `tones::known`
  is there for the same purpose.
* **No tone is the ABSENCE of the attribute.** The palette declares a `default`
  family and it is an info blue — a component whose untoned state named it would
  tint every ordinary cell. `neutral` and `muted` are this repository's own two,
  emitted from the structural tokens because the palette has neither.
* **`gates.rs` reads `dist/tones.css`** for the `unknown_token` check, so a
  `var(--nd-tone-*)` is a declared name.

**`Meter` AND `Progress` HAVE NO RUNTIME, AND THAT IS THE ARGUMENT FOR THEM.**
They are the real elements; the bar, the range, the thresholds and the
announcement are the platform's. The stylesheet is the only cost, and it is a
real one: `<meter>` and `<progress>` have separate shadow trees and no engine
agrees with another about the pseudo-element names, so every selector is written
out — one wrong name invalidates the whole list it is in. `appearance: none`
takes the indeterminate animation away with the widget, so that sweep is drawn
and its reduced-motion guard is this stylesheet's to keep.

**A `Table` IS A `table` UNTIL A `TableColumn` GIVES IT SOMETHING TO DRIVE.**
The family was deleted and rebuilt around `TableColumn` — one real `<colgroup>`
per column, declared before the rows, carrying width, sortability and its
comparator, filterability, resizability, hideability and pinning. Three things
follow from that and are easy to get wrong:

* **`role="grid"` is derived, never declared.** It is true exactly when a column
  is sortable, filterable or resizable, or a row can be chosen or opened. There
  is no prop, so it cannot be set on a table with nothing to drive.
* **A `<colgroup>` counts positions, and a row draws its own leading cells.** A
  choice control and an expand control are cells `TableRow` added, and the
  declarations know nothing about them — so a table with either wants a leading
  `TableColumn` declared for it. The runtime inserts the missing groups where a
  caller did not, because the failure is silent and looks like a broken column
  width rather than like a missing declaration.
* **Never hide one thing two ways.** A column hidden by `visibility: collapse`
  on its `<colgroup>` *and* by `hidden` on its cells made automatic table layout
  take the next column away as well.
* **A `<col>` width must not switch the table to `table-layout: fixed`.** Under
  fixed layout every column with no declared width takes an equal share, so one
  resize reset all the others and the values printed through each other. A width
  is a preference, not a decree.
* **A row draws its own leading cells and the heading row does not.** The
  runtime inserts the missing `<th>`, and `table.spec.js` checks that every
  heading on the page starts where its own cell starts.
* **A variant stage disables three things** — `container-type` (which IS
  `contain: inline-size`, so the intrinsic width is zero), the scroller's
  overflow, and `.nd-table`'s `inline-size: 100%`. Any one left in place
  collapses the specimen to nothing, which is how five `width` variants came to
  be five empty boxes.
* **`variants = false` is how a prop says its values look identical**, and a
  prop may carry its own `specimen` where the component's cannot show it.
* **No prop value may name a domain.** `emphasis` was `subtotal | total`; it is
  `subtle | strong`, because a table of servers has no subtotal and the two
  weights are *where a section ends* and *which line is the answer*.
* **Count columns, not cells.** A `colspan` is one child and several columns —
  the runtime and `table.spec.js` both read a two-level heading as a head one
  cell short and inserted a blank `<th>` in front of it.
* **The un-stacking container query must restore alignment.** The stacked rule
  is one class and two attributes deep, so it outranked `[data-nd-align]` at
  every width and `numeric` did nothing anywhere — invisible, because a heading
  is a flex column that ends its own items and the words moved without the
  values.
* **`Menu` and `MenuItem` are the surface a button opens**, and `dist/menu.js` is
  the tenth shipped runtime file — `info.js`'s shape plus the keyboard a menu
  owes. `Button { menu: true }` had announced a menu with nothing behind it.
* **`aria-disabled` on a menu item, `disabled` on a button.** An item that leaves
  the arrow walk is an action a reader concludes does not exist.
* **`aria-expanded="false"` is TRUE on a menu button** — the one exception to
  `Button`'s write-it-only-when-true rule.
* **AND ESCAPE HAS TO BE HEARD ON THE BUTTON, NOT ONLY ON THE SURFACE.**
  Opening with a pointer leaves the focus on the button deliberately — a reader
  who clicked is reading — and the only Escape handler was on the menu, so that
  reader had no key that put it away. It is on both now, `stopPropagation` on
  each, so a menu inside a dialog closes itself rather than the dialog.
* **`Pagination`'s gap is the second menu trigger in the system.** `more_label`
  turns the `…` between two runs of numbers into a button opening a `Menu` of
  the pages it stands for, so page 7 of 40 is one press rather than five. It
  reuses `dist/menu.js` rather than growing a second menu, and it ships
  `hidden` exactly as `Button { menu }` does. The inert character stays in the
  markup and `.nd-pagination-gap:has(+ .nd-pagination-more:not([hidden]))` takes
  it away once the script has revealed the control, so a page with no script is
  what it always was. The items are page NUMBERS: identifiers, marked
  `nd-identifier` and never translated, while the menu's own name is a sentence
  and comes from the prop.
* **A MENU BUTTON DRAWS ONE CHEVRON, WHOEVER DREW IT.** `menu` draws the
  chevron and steps aside for an `icon` or an `icon_end` naming the same
  drawing. It only stepped aside for the trailing one, so a split button asking
  for `icon: "chevron-down"` beside `menu` shipped two of them side by side, on
  `/button-group/` and in that page's hero. `no button anywhere draws the same
  chevron twice` in `button.spec.js` is the guard.
* **A RULE THAT NAMES ONE AXIS HAS TO NAME BOTH.** `ButtonGroup`'s outer
  corners were two unconditional rules plus two for `vertical`, so a vertical
  group ran all four and kept two of its eight corners. Squaring an inner
  corner is `border-radius` for three shapes and the `clip-path` polygon for
  `bevel`, which is a cut rather than a radius and needs its own rules.
* **A `text` icon prop draws nothing when the name does not parse.** `x.svg` is
  the file; `close` is the icon's name. A test reads every icon an example names.
* **A SPECIMEN IS A GENERATED TREE, AND IT RENDERS THE REAL COMPONENT.** It was
  an HTML string `pages.rs` assembled from the contract, which made every grid a
  picture of what a string builder believed the component would render.
  `data-nd-data-nd-size`, four tones drawn as four grey buttons and
  `data-nd-kind="ordered"` on a `<ul>` all shipped that way, each one a grid
  faithfully documenting a bug in the grid.
  `crates/noctua-design-emit/src/specimens.rs` writes one tree per component,
  prop and value into `docs-site/src/generated/specimens.rs`, and both the page
  and the React mount draw it. The component writes its own attributes, its own
  defaults and its own bools.
* **A `specimen` THAT IS MARKUP IS THE FALLBACK, AND TWO COMPONENTS ARE LEFT ON
  IT.** Thirty-seven contracts used to carry a drawing rather than a use of the
  component, on the grounds that a live `<input>` in a grid is a control a
  reader can type into by mistake. That trade bought nothing and cost the grids
  their subject. `Meter` and `Progress` drew a `<span>` carrying the real
  element's class, which no `::-webkit-meter-*` rule paints, so six sizes were
  six grey stubs. `Badge` placed a badge against nothing, so all four
  placements landed in one spot. `List { kind: description }` put `<li>` inside
  a `<dl>`, which is markup no parser accepts. `Field` drew a large "Aa", a
  grey bar and an empty box — no label, no control, and nothing telling
  `single` from `group`.
  **`specimens::itself` is the reach**: what a component needs beside the prop
  under test in order to draw ITSELF — a control for `Field` to label,
  something for `Badge` to sit on, tabs for `Tabs`, rows for `Table`. It is a
  tree, so it lives beside the emitter that writes trees rather than in TOML,
  exactly as `controls.rs` holds a body a contract cannot describe. The two
  left on the fallback are `Lineage`, four of whose grids are about the WINDOW
  rather than the drawing, and `LineageFigure`, which is a piece with nowhere
  to stand outside a scene. **A picture carries every other enum prop at its
  default**, because one attribute alone matches no compound rule and compound
  is how every tone here is painted. **Never an `aria-*` one, though**: no rule
  in this system selects on ARIA, and what a specimen invented was
  `aria-sort="none"` on eight still `<th>` pictures — each announcing itself as
  a column somebody can sort by.
* **A GRID DRAWING THE REAL COMPONENT IS AN AUDIT OF THE COMPONENT, and it
  found four things nothing else had.** `Prose { measure: none }` was inert
  around every paragraph on the site, because `.nd-paragraph` caps itself and
  the absence of a rule left that cap in place. `AvatarGroup`'s overlap is
  `li + li`, so members handed over as bare avatars overlapped by nothing.
  A `Toolbar` holding one `ToolbarGroup` has one child, so `horizontal` and
  `vertical` were one picture. And a `ColorPicker` given no `area_label` or
  `hue_label` renders two nameless range inputs in its panel — on every page
  that has one, not only in a grid.
* **THE MARKER SAYING A CELL SHOWS NAMES IS ON THE STAGE.** `.nd-identifier` on
  the component call works for a component that draws its own content and fails
  for the six that wrap themselves in a `Field`: the class lands on
  `.nd-select`, and the `<label>` the field drew is its SIBLING. `/select/`
  shipped the word `Select` unbracketed until the class moved one level out.
* **AND A SPECIMEN'S `name` IS THE CELL'S, NOT THE COMPONENT'S.** Every other
  required text prop takes the component's own name — the same string in every
  cell, which is what lets `variants.spec.js` measure two values against each
  other — but `name` is the hook every control derives its ids from. One `name`
  across a page of grids is one `id` a dozen times over, and for a radio group
  it is worse: two groups sharing a name are one group.
* **A SPECIMEN STILL HAS A NAME, AND THE COMPONENT GIVES IT.** A picture points
  at its own caption with `aria-labelledby`, and a component call is passed the
  prop its contract requires for the purpose — filled with the component's own
  name, which is the same string for every value in the row. That last part is
  load-bearing: a row whose words change with the prop under test is a row where
  two values painted alike still measure apart, and `variants.spec.js` reads
  exactly that measurement. `every_specimen_of_a_named_control_has_a_name` is
  the gate, and `IconButton` shipping six nameless controls is why it exists.
* **A void or replaced element becomes a `<span>` in a picture.** An `<input>`
  cannot contain its own figure and a `<meter>` never renders fallback content,
  so `.nd-x[data-nd-size] .nd-x-part` matched nothing on six controls. A
  component CALL is exempt, because the component renders whatever it renders:
  `Divider`'s grid is a real `<hr>` again, self-closed and carrying the
  `role="separator"` the string never wrote.
* **`variants = false` is asked by `fn drawn`, and by everything that walks a
  grid.** Three renderers and the serialiser read it, and the emitter asks the
  same question of the spec. It used to be enforced on collection pages only, so
  the declaration silently did nothing on every single-component page.
* **A prop's own `specimen` beats the component's**, including when the
  component's is an icon name.
* **`variants.spec.js` ASKS THE FOUR QUESTIONS A GRID EXISTS TO ANSWER.** Two
  values of one prop drawing the same picture is the first, and its fingerprint
  reads the `::-webkit-*` parts too, because `<meter>`, `<progress>` and a range
  `<input>` are painted there and nowhere else. The other three: **every cell
  draws something** with a box bigger than nothing, **every cell carries the
  value its caption names** wherever the prop has an attribute at all, and **no
  cell overhangs a frame that cannot scroll** at 320 pixels. A picture whose far
  side is beyond anything a reader can do is not a picture.
* **A GRID REPEATS ITS OWN WORDS ON PURPOSE, and three sweeps had to be told.**
  Every cell of one row says the same thing, so that two values painted alike
  measure alike — which is the opposite of what `no two number fields ask the
  same question` and `every meter is a different reading` demand of the
  examples. Both read `:not(.nd-variant-stage *)` now. `no choice on the page is
  a specimen pretending to be a control` asserted the reverse of its own title
  and was rewritten: every choice on that page has a real `<input>` in it.
* **There is no `glow` and no aurora.** Removed on the developer's instruction,
  and with it the last animation on any page: `[effect.blur]`,
  `[effect.duration]`, the `drift` keyframes and the reduced-motion stop. An
  emitter test now fails if anything in the appearance sheet animates again.
* **`dots` has a ceiling of its own** (`[effect.alpha] dots = 10`). A ceiling is
  per-pixel, and a pattern covering a fortieth of its tile buys a fortieth of
  the texture at the same number — at 4% the dots were invisible on a light
  page.
* **An element screenshot ignores `clip`.** It returns the whole element, which
  is how the pattern-ceiling test came to measure the entire page and pass by
  reading furniture instead of ink.
* **A specimen cell carries `.nd-table-value`**, or every rule that acts on the
  value rather than the cell (`truncate`) does nothing in the grid.
* **The narrow layout is a container query on `.nd-table-scroll`**, not a media
  query. A props table inside a narrow example frame must stack on a wide
  window, and that is the case a media query cannot see — and the corollary bit
  once: a table left to shrink to its content inside a flex column became a box
  under the breakpoint and stacked itself, taking its heading row and every sort
  control with it. A table in a stack takes the width.
* **Anything that measures inside a `ResizeObserver` must not write what it
  measured back unchanged.** `paintPins` did, and the observer saw the style
  change, and the pair never settled — a core spent, and every element on the
  page permanently "unstable" to anything waiting for it.

**`ErrorSummary` IS THE ONLY COMPONENT THAT TAKES THE FOCUS UNASKED, AND THE
ONLY ONE THAT WRITES INTO ANOTHER COMPONENT.** With `collect` it catches the
form's `invalid` on the way down, puts the browser's own `validationMessage`
into the empty paragraph `Field.error_slot` leaves behind, marks the field, and
reads the list back out of every `[data-nd-field-error]` in document order. Two
rules make that safe and both are load-bearing: it **never overwrites a sentence
a caller wrote**, and it takes back only what it wrote — the marker is
`data-nd-said` on the slot. It invents no language of its own; every word comes
from the platform or from the caller. Five of the six controls that take a
`required` pass `error_slot: required` for it, which is why `Field.error_slot`'s
documentation names two shapes rather than one.

**A CONTROL WHOSE SENTENCES ARE ITS OWN PASSES `error_slot: true`, NOT
`required`.** `NumberInput` and `DateInput` write their refusals into that
paragraph themselves, so tying it to `required` means the slot is missing on
exactly the fields a caller bounded but did not require: `min_label`,
`max_label` and `invalid_label` were dead on every one of them, and the field
still set `aria-invalid`. Marked wrong and silent is worse than either. The
script marks its own sentence `data-nd-said`, as `ErrorSummary` does, so an
accepted value takes back what the script wrote and never what the caller did.

**`Field` DRAWS THE QUESTION, AND THE CONTROL OWNS THE `aria-describedby`.**
One component for the label, the hint, the error and the `<fieldset>`/`<legend>`
a *group* needs. It renders no control — the control is its children — so it
cannot write an attribute on one, and both sides derive `{control}-hint` and
`{control}-error` from the same prop. `every_describedby_points_at_something`
in `tests/browser/inputs.spec.js` is what holds them to it, and it has already
caught `TimeInput` pointing at ids the field never drew. **`Checkbox` and
`Radio` are not this shape** and are deliberately not migrated: their label is
in the row, beside the box.

**A PROP MAY NOT BE NAMED AFTER A RUST KEYWORD, and the spec refuses one.**
`loop` reached the generator and came back as *expected identifier, found
keyword* pointing at a generated file nobody wrote. Escaping it to `r#loop`
would work and would be one prop with two spellings across the two packages,
which is the thing they exist not to have — so it is `wrap`, and
`check_props` now names the offending word instead of leaving a rustfmt
diagnostic to be decoded. A prop doc may carry paragraphs now too; before
`autoplay` needed them, a newline in one produced a file that did not parse.

**THE FOUR THAT PREDATE `Field` USE IT NOW.** `Select`, `TagPicker`,
`DateInput` and `ColorPicker` drew their own label, hint and error with their
own class names; twelve rules and four copies of one arrangement are gone.
What the migration turned up is what `Field` gained doing it: **an
`error_slot`**, because `Select` and `DateInput` had each invented the same
thing — a paragraph rendered empty so the browser's own complaint has
somewhere to go, since a control the script has hidden cannot be reported; and
**a group that describes itself**, because `Field { kind = "group" }` IS the
`<fieldset>`, so a reader entering one hears the hint once instead of once per
box. `TimeInput` was written before that and is better for it.

Three things move with the markup, and a spec that selects on any of them has
to move too: the question is `.nd-field-label` and carries `{control}-label`,
the sentence is `[data-nd-field-error]`, and both are **siblings** of the
control's own root rather than children of it — `.nd-field:has(…)` is how a
test says *the field this control is in*.

**`Checkbox` and `Radio` are still not migrated, and still should not be.**

**`ColorPicker` IS THE ONE PLACE A COLOUR LITERAL IS NOT A PALETTE DECISION.**
A hue track and a saturation field declare no colour: they display sRGB so a
reader can point at one part of it, and what comes out is the consumer's data.
`NOCTUA.md §8.8` provides the marker and names tests explicitly; what this
repository adds is *where* — one block per file, at the top, so a reviewer
reads the exceptions once. Five lines in `shell.css`, one fixtures block in
each of the two colour specs, and nothing else anywhere.

**AND ITS ARITHMETIC IS CHECKED AGAINST `noctua-colors`, NOT AGAINST ITSELF.**
There is one implementation of the conversions, so there is no twin to compare
— and a thing compared to itself passes whenever it is wrong. The vendored
output publishes the hex of every palette step *and* the OKLCH it computed that
hex from, in another language, in another repository. `tests/browser/color.spec.js`
converts the first and is held to the second, five thousand times over.

**A COLLECTION PAGE'S TITLE IS A NAME, NEVER A MESSAGE.** `site::Title` has two
variants and the sidebar's two blocks are exactly those: a foundation is a
sentence a reader reads, a component page is an identifier a developer types.
The collections shipped as messages and were translated — `/tree/` was *Árvore*,
`/search/` *Busca*, `/avatars/` *Avatares*, `/tables/` *Tabelas* — so four rows
named components that exist under no such name anywhere. Title a collection with
the name at its head, or with an English label where its members share none
(`Inline text`), and leave the slug alone: `/avatars/` and `/tables/` are URLs.
`the_component_block_is_never_translated_and_the_foundations_always_are` is the
gate, and `example.spec.js` compares the two rendered sidebars.

**THE BACKDROP IS `--noctua-backdrop`, AND IT IS DECLARED WHEREVER A SCOPE CAN
BE.** It was `--noctua-page-background`, declared at `:root` and read by `html`
alone — and a `var()` resolves where it is DECLARED, so it was frozen to the
root's palette and mode. A frame that went dark kept the page's white under it:
dark ink on paper, inside a dark frame. It takes the same selector list
`dist/scope.css` uses, for the same reason that file exists, and `dist/theme.css`
paints it on `:root` and `[data-noctua-scope]` — the page, and every region that
asked to be a page in miniature.

What has not changed is why the backdrop is safe: it moves ONE variable, so
`--nc-color-surface` is untouched and not one measured contrast pair moves with
it. Five of the six name a token the colour system already solved in both modes,
so their rules carry no mode condition — writing one would be a second answer to
a question the token has already answered. Only `black` and the light default
are literals.

**A SCOPE RE-READS THE INK AS WELL AS THE CONTRACT.** `dist/scope.css`
re-declares 1,767 `--nc-color-*` names so a subtree resolves them against its
own palette, and that does nothing for the properties that CONSUME them.
`body { color: var(--nc-color-fg) }` resolves at `<body>`, against the page's
mode, and every descendant inherits the computed value — so a themed subtree
painted its own surface and kept the page's ink. Measured: near-black text on a
near-black surface, about 1.2:1. One zero-specificity rule at the end of
`scope.css` fixes it, and `readability.spec.js` measures it. That test had to be
written twice: a probe laid over a live page and screenshotted catches the
page's own furniture rather than its glyphs, and the first version passed with
the defect restored.

**A FROSTED CONTROL KEEPS ITS COLOUR IF IT HAD ONE, AND GAINS NONE IF IT DID
NOT.** The tint is for the variants that were FILLED — `solid` and `subtle` —
and tinting the others turned a white `outline` button grey. It is mixed from
the tone's solid step against the surface at `--nd-tint`, never the tone at
partial alpha (that keeps an `--nc-color-on-*` ink solved against an OPAQUE
fill, and measures 3.17:1 once a fifth of the backdrop shows through) and never
the tone's `-bg` step (0.975 lightness at zero chroma in this palette — a tint
nobody can see). The label is always `--nc-color-fg`; the tone's own colour as
ink is the 3.17:1 pair that `outline` lives with on an opaque page and glass
cannot. `glass.spec.js` measures every enabled frosted control from a
screenshot and `pixels.js` is where the colour maths lives — a second copy of a
contrast function is a second answer to the same question.

**GLASS IS A PROP, THERE IS ONE RECIPE, AND AN OVERRIDE WINS ON SPECIFICITY.**
A component that paints a fill declares `glass` (`data-nd-glass`), and the
generated `.noctua-glass` selector carries `[data-nd-glass="true"]` alongside
the class — a second `backdrop-filter` anywhere is a source-gate finding. The
per-component rule that replaces the fill carries **one more attribute than the
fill it replaces**; winning on source order is what put 32px over the settings
panel's title. The tint is written at each point of use, never held in a custom
property, because `--x: var(--nc-color-…)` freezes to the declaring element's
palette. And a bool prop always writes its attribute, so `[data-nd-glass]` IS
the set of controls that support it — never write that list down.

**A FRAME OWNS WHAT IS INSIDE IT.** A setting a frame STATES owns its whole
subtree — a nested frame, its bar, its border, its own menu panel. A setting
nobody stated comes from OUTSIDE, which on an ordinary page is the page and
inside another frame is that frame. Reset CLEARS the request; it does not state
the page.

**This replaces a rule that said the opposite**, and the opposite is why the
defect kept coming back: a frame used to default its request to the page's whole
theme, so the resolver stamped the page back on and cancelled the frame around
it. Every patch to the symptom was reverted by this paragraph and by two tests
named after it. `tests/browser/frame-isolation.spec.js` is named after the rule
so the next version cannot lose it, and it records the two traps that spec has
already fallen into.

Four things that make this look done when it is not:

* **A frame the resolver never visits cannot follow anything**, so every frame
  is registered at load whether or not it has been themed.
* **`inheritedAt` must walk REGISTERED scopes, not stamped ones.** Stamped means
  resolution depends on which ancestors happen to have been visited in the
  current pass — theme the inner frame first and it inherits the page.
* **A value stamped as the ABSENCE of its attribute (`omit_when`) means "the
  machine decides" AND "whatever is above me".** One spelling for two facts: a
  reader who picks `system` inside a dark frame asked for the machine, so that
  one value is written out concretely and nothing else is.
* **A no-op scope carries no attributes at all**, so anything that selects
  `[data-noctua-scope]` under a frame — a reset, an assertion — skips exactly the
  frames that stated nothing. Never write `stage.getAttribute(…)` in a test; ask
  the pixels, or ask what is in force.

**GEOMETRY IS ASSERTED, NOT EYEBALLED — AND NOT EYEBALLED ONCE.** A control
that overlaps the one beside it, a name wider than the box holding it, or a row
of an open panel that hit-tests to something else is a failing test, not a
matter of taste. `tests/browser/theme-menu.spec.js` holds the four, and they run
across every locale because `qps` exists to make a fixed column overflow and
only finds it if something asks. A panel is checked with a frame open INSIDE
another frame, at 768px, in `pt-BR` — the combination that broke, and the one a
single wide English screenshot cannot show.

**A panel cannot outrank its own ancestor's stacking context.** `.nd-frame-bar`
is `sticky` with a `z-index`, so a panel inside it paints at the BAR's layer
whatever the panel asks for. Raise the bar, not the panel.

**Where a settings value lands is decided by where the control SITS.** Inside a
themed scope it themes that scope; inside an example frame it themes that
frame's stage; a setting the model knows themes the document; anything else
moves and no more. `docs-site/src/theme_menu.rs` is one component used by the
header and by every frame, and that rule is why it needs no target prop.

**A scoped control inverts what is in force WHERE IT SITS, not what the page
is.** `settingAt(element, name)` walks every scope above the element and is the
only correct answer once anything can nest.

**A scope must paint its own background — and the rule that does it is
generated.** Re-resolving the contract changes what the subtree's colours
*mean*; it paints nothing. A scope that sets ink and borders and leaves the
surface transparent shows the page through itself, which is a light island on a
dark page. `.nd-scope` and `.nd-frame-stage` both learned this the same way, and
each carried its own rule for a while — the docs site's, so it worked here and
nowhere else. `dist/theme.css` paints `:root, [data-noctua-scope]` now, so any
consumer of the package gets it and this site's copies are gone. The one in
`shell.css` had to go for a second reason worth remembering: it used the
`background` SHORTHAND, which resets `background-image`, so a frame that asked
for a pattern would have got the colour and nothing else.

**The generalisation, and it is now a rule.** *Any* custom property whose value
contains `var(--nc-…)` freezes to the palette of the element it is declared on.
So the glass tokens are dimensionless numbers, and the `color-mix()` that turns
them into a surface is written **where the token is used**. A
`--noctua-glass-surface: color-mix(…var(--nc-color-surface)…)` at `:root` would
hand every themed component the page's frost.

### Trap 0 — a scopable setting still needs an unscoped default

Every glass block is a bare attribute selector, which is what makes glass reach a
subtree. But nothing stamps `data-noctua-glass` until a reader changes it, so on
a first visit **no block matched**, `--noctua-glass-blur` was undefined, and
`backdrop-filter: blur(var(--noctua-glass-blur))` was invalid at computed-value
time — which resolves to `none`. Every frosted surface in the system rendered
flat, silently, and the settings panel appeared to "turn glass on" when it was
only defining the tokens for the first time.

The default level is therefore also declared at `:where(:root)`, at zero
specificity so an explicit `off` still wins. A test asserts it. **Any scopable
setting whose tokens are read by a property that can be invalid needs the same
thing.**

### Trap 0.2 — never name a border once at the root

This is the same `var()` rule as above, and it is worth its own heading because
it is the shape the mistake actually takes:

```css
:root { --nd-edge: 1px solid var(--nc-color-border); }   /* WRONG */
```

Convenient, readable, and it paints every border inside every `ThemeScope` in the
**page's** color. It was in this repository's own shell stylesheet, below a
comment explaining the rule it broke. `frozen_color_indirection` now fails the
build on any custom property whose value contains `var(--nc-…)`. Name the color
where you use it; the repetition is the point.

### Trap 0.5 — media queries are minimum-width, and the gate now says so

A media query cannot read a custom property, so the token gate could never apply
to a prelude and preludes were simply unchecked. That hole let three `max-width`
queries into the shell's own stylesheet — written directly under a header saying
breakpoints are minimum-width only.

`gates::media_prelude` now rejects `max-width` outright and requires every
`min-width` to equal a breakpoint declared in `spec/tokens.toml`. Write the
narrow case unconditionally and override it upward.

### The URL scheme is a directory per page, and the depth is uniform

`dist/<locale>/<slug>/index.html`, for every page — foundations and components
alike. So `site::BASE` is the constant `../../` rather than a calculation, and
every href a reader can see is a directory ending in `/`. Nothing shows `.html`,
least of all `index.html`, which is the file name for *"this page has no name of
its own"* and was in every URL on the site.

Two consequences worth knowing before you fight them:

- **`file://` no longer works.** A browser will not serve a directory's
  `index.html` off the filesystem. `cargo xtask dev` is the way to look at the
  site, and it is the same static directory any host would serve.
- **A directory without its trailing slash must 301**, or every
  document-relative href on the page resolves one level too high. The dev
  server sends it, because GitHub Pages and `python3 -m http.server` both do,
  and a dev server that disagrees with production hides exactly the bugs it
  exists to surface.

`<locale>/index.html` is a redirect document to the first entry in that
locale's sidebar. It is a door, not a page: no title, no prose, no sidebar
entry, and `site::first` is what both it and the wordmark read.

### The language is resolved before the first paint, and never against the reader

`spec/i18n/` gives three locales; the site is pre-rendered in all of them, so
switching language is navigation rather than state. The control offers
**Automatic**, which is the default and which follows `navigator.languages`.

Two rules the inline resolver in `main::locale_script` keeps, and both matter:

- **It never builds a URL.** Every destination is written into the script at
  build time, one per locale; the script picks from that table. A stored value
  that is neither `auto` nor an offered tag becomes `auto`, and an `auto` that
  matches nothing means *stay where you are*. Both fail closed.
- **It never overrides a locale the reader asked for.** Only a fresh
  `navigate` with a same-origin referrer counts as *following a link inside the
  site*. A reload sets the referrer to the page itself and `back_forward` sets
  it to wherever the reader came from — treat either as internal and a reload
  bounces the reader out of the language they chose, or Back redirects them
  forward again and the button appears broken.

### The default palette is not the color system's default, and that costs an attribute

`noctua-colors` publishes exactly one palette anchored at `:root` — its own —
and thirty-nine anchored at `[data-palette="<name>"]`. `spec/theme.toml`'s
default is blue, so `dist/base.html` links `theme-blue-balanced.css` and the
attribute is what makes it apply.

Which is why the bootstrap stamps `data-palette` **even when nothing is
stored**, and why the documentation site server-renders it on `<html>`. Link
the base set without either and the page has no palette at all — not the wrong
colors, none — and it looks like a stylesheet that failed to load.

### Icons are declared, not drawn

`spec/icons.toml` names them; `assets/icons/` holds the upstream SVGs vendored
at a pinned tag beside their licences; everything else is generated — the
`IconName` enum, `STROKE_WIDTH`, `dist/icons.json`, the attribution and the
documentation page. Adding one is a table and a drawing; there is no list
anywhere to keep in step, including the page that shows them.

**Both `Icon` implementations come from the catalogue now**, not from this
emitter: `spec/components/icon.toml` and a body in `controls.rs`, exactly as
every other authored component. See the promotion note above for why moving it
was the only safe shape.

**Why an enum.** The shell hand-wrote an `Icon` that matched a `&str` against
three arms with `_ => ""`. `Icon { name: "repository" }` compiled and rendered
`<path d="">` — a 44×44 clickable void in the header of every page, in every
locale. A name that is a string cannot distinguish "this icon" from "no icon".
Two gates keep it that way: **`hand_drawn_icon`** (an `<svg>` or `<path>` in any
source outside the generated module) and the drawing validator, which refuses
`id`, `class`, `style`, `fill`, `stroke`, `mask`, `clip-path` and an empty path.

**A drawing may carry only geometry.** An `id` collides the second time the same
icon is inlined; a `fill` refuses `currentColor` and paints a fixed color
inside a themed subtree — the frozen-color trap wearing a different hat.

### Trap 0.8 — a token that does not exist fails silently, both ways

`var(--noctua-space-40)` is valid CSS. There is no such token, so the whole
declaration is invalid at computed-value time and the browser **drops it** with
no error: `grid-template-columns: repeat(auto-fill, minmax(var(--nope), 1fr))`
does not fail, the grid just becomes one column, and it looks like a decision
somebody made. This was written into this repository while adding a gate for a
different silent failure, and nothing caught it.

`gates::unknown_token` now collects every `--name:` declared anywhere — sources
plus the generated stylesheets — and reports any `var()` that names something
outside that set. `--nc-*` is exempt (1,767 vendored names, guarded elsewhere),
and a name ending in `-` is skipped because it is one an `rsx!` interpolation
was cut out of.

### One glass recipe, and three things that switch it off silently

`.noctua-glass` is emitted from `spec/tokens.toml` into `dist/theme.css`, and
`gates::hand_written_glass` fails the build on a `backdrop-filter` written
anywhere under `docs-site` or `assets`. There were four hand-written copies;
they drifted, and the drawer switched its glass off above `lg` — so the sidebar
and the settings panel were different materials on every desktop viewport.

Three ways a `backdrop-filter` does nothing, none of which reports anything:

1. **An ancestor has one.** It is then inert. This is why the header's frost is
   a *sibling* painted behind it, and why a scope is not glass: the nested
   scope's blur sampled its parent's and was switched off entirely.
2. **The element already has an opaque background.** There is nothing behind it
   left to sample.
3. **Its tokens are undefined**, which computes to `none` — see Trap 0.

Opacity is 78/58, not 92/82. At 92% there is nothing to see through: the blur
had nothing to do and the effect read as a tinted panel, which is precisely the
complaint. The floor is readability, measured by `readability.spec.js`.

### A scrollable element never carries a radius

And a radius element never scrolls. There is no source gate for it yet;
`material.spec.js` asserts it in the browser, over every element on the page.

A radius on a scroller has nothing to clip the bar against, so a classic
scrollbar paints square straight through the rounded corner. That is what a
reader sees the moment corners are set to `circular`. The fix is structural:
an outer element with the radius and `overflow: hidden`, an inner one with
`overflow-y: auto`. `.nd-panel` + `.nd-panel-body` is the shape, and so is
`.nd-codeblock` + `.nd-codeblock-code`, and so is `.nd-menu` +
`.nd-menu-scroll` — that last one is why `Menu` renders a `role="none"` box
around its items. A generic element between a `role="menu"` and its
`role="menuitem"` children is the `<ul role="menu"><li role="none">`
arrangement, and it is what a menu that has to scroll costs.

**A floating surface that cannot scroll is a floating surface with a height
nobody set.** `Menu` had neither: a list longer than the window ran off the
bottom of it with no bar and no flip. Every panel `anchor.js` positions bounds
itself with `calc(100vh - var(--nd-anchor-block) - …)`, because the script
writes that variable as the distance from whichever edge the panel hangs from —
so one expression is right above and below. Reach for that expression rather
than a number.

**It happened twice.** The code block declared both on one rule and survived
the browser assertion for a whole stage, because no page the suite looked at
had a block wide enough to actually scroll. An assertion over "every element on
the page" only covers the elements that are on it.

**`scrollbar-width` is not inherited.** Set on `:root` it styles the page's own
bar and nothing else — every panel and code block keeps the browser's default,
which is the bar a reader is actually looking at. The emitted rule uses
`:where(*)`.

**IT HAPPENED A THIRD TIME, AND THE COST WAS THE CORNER PRESET.** A
`<textarea>` IS a scroll container, so `.nd-textarea` — which was the
`<textarea>` — could not carry a radius and shipped `border-radius: 0`. It was
the one control on the site that ignored the reader's corner setting, and the
"no radius, because it scrolls" comment above it read as a decision rather than
as a missing box. `Textarea` is `.nd-textarea` + `.nd-textarea-box` now,
exactly as `Input` is `.nd-input` + `.nd-input-box`, and `inputs.spec.js`
asserts every field in the family goes round when the preset does. Two things
came out with it: **a bare `<textarea>` has no `::after`**, so the one glass
recipe's sheen had never drawn on a frosted one, and **`size` set no padding
anywhere**, because the only rules keyed on it needed a child the control did
not have.

**AND A FORM CONTROL'S `line-height` IS `normal`, whatever the page is set
in.** `Textarea`'s `max_rows` is worked out as `maxRows * lineHeight`;
`Number.parseFloat("normal")` is `NaN`, the ceiling fell back to `Infinity`,
and a box with `max_rows: 8` grew to 724px. The prop had never once applied,
and the test guarding it passed because it asserted `< 2000` rather than the
ceiling. Anything a script derives from a computed font metric on an `<input>`,
`<textarea>` or `<select>` needs that metric written down first.

### Trap 0.7 — a render-blocking stylesheet that 404s blocks nothing

The bootstrap injected the stored palette's sheet with a **bare relative href**.
Every page lives below the site root, so it resolved one directory too deep and
404ed on every load. `blocking="render"` was set correctly and bought nothing: a
sheet that never loads delays no paint. The page painted the default palette and
the deferred module snapped it over *after* first paint — the exact flash the
bootstrap exists to prevent, on every reload and every internal link.

Anything the bootstrap builds a URL from must resolve against
**`data-noctua-base`**, the attribute the document declares and `shell.js`
already reads. Hardcoding depth in an emitter is hardcoding the URL scheme.

**And the reason it survived a review and a test suite:** the only test asserted
that `blocking="render"` was set before insertion — reading back a property the
code under test had just set on an element it had just created. It never
resolved the URL and never looked at a color. *Assert the outcome.* The
replacements load the page with `shell.js` aborted and check `link.sheet !== null`
and that the painted accent differs from the default, plus one spec that fails on
any response ≥ 400 across every page in every locale.

**Trap 1 — Noto JP, KR, SC and Hebrew default to weight 100, not 400.**
`fc-query` reports `Thin,Regular` as their first face. Anything rendering the
default instance without pinning the axis gets **hairline text**, and the browser
then fabricates a fake bold on top of Thin. Every variable `@font-face` this
repository generates therefore declares `font-weight: 100 900`, and a test
asserts it — because the failure is silent and looks like a design choice.
`NOCTUA.md §7.6`.

**Trap 2 — `font-stretch` on a face with no `wdth` axis.** Declaring a range the
face does not have makes the browser synthesize, which is worse than not asking.
Only the Group A Noto faces carry `wdth`; Iosevka and the CJK faces do not. The
same test covers this.

**`font-display: optional` needs preloading, and needs *all* the faces
preloaded.** `optional` uses a face only if it is ready by first paint and never
swaps afterwards — which is what avoids the reflow `swap` causes. Preload only
the regular and bold and italic fall back inconsistently from one reload to the
next, which reads as the typeface flickering. This cost real time in
`noctua-colors`' docs site.

**Do not subset the on-demand script faces.** A user who needs Devanagari needs
all of it. Subsetting applies to the Latin default payload only.

### The Portuguese has a glossary, and nothing enforces it

`spec/i18n/pt-BR.toml` is written by whoever changes the English, and a term
translated two ways is the commonest defect in it. Settled: *a pessoa* for the
person using the interface, *quem chama* for the developer, *leitor de tela*,
*parada de tabulação*, *padrão* (never the English *default*), *linha* for a
table row, *rótulo* for visible label and *nome acessível* for the accessible
name, *traço* for a drawn rule, *alça* for a slider grip and *cursor* only for
the text caret, *região dinâmica*, *resumo* for an error summary, *marcação*,
*navegador*, *o build*.

The register check measures length, punctuation and banned constructions in
both languages. It cannot tell whether a sentence reads as Portuguese: *portão*
for a build gate, *compositor* for a composer input and *sumário* for an error
summary all passed it. That half is a reading, against the English, and it is
worth doing whenever the Portuguese is written in bulk.

### No other design system is named as a source

`xtask/src/gates.rs::cited_sources` refuses it, and it is the one gate that
reads the raw line rather than the line with its comments stripped, because
that is where the citations were. Nothing in this repository was taken from
another design system, so naming one is a claim about a relationship that does
not exist.

Keep the fact, drop the name: a WCAG number, a hit-target size, an accessibility
technique are all facts and stay written down.

This is not about vendored work. `assets/icons/ATTRIBUTION.md`,
`assets/vendor/`, `assets/fonts/` and the palette under `packages/tokens` carry
real licences, and those attributions are obligations.

### Reader-facing prose is checked, not merely reviewed

`crates/noctua-design-spec/src/prose.rs` holds the register and runs under
`cargo xtask check`, over `[meta] summary`, `guidance`, `[a11y] notes`, every
`doc`, every keyboard action, and both `spec/i18n/*.toml`. It refuses an em
dash, a semicolon, a sentence over twenty-five words, capitals used as
emphasis, a per-language list of banned constructions, and anything over the
ceiling for its kind (summary 100, keyboard action 120, note 140, the rest 250).

Say what the thing is and what it does. A summary opens on a verb and names
what a reader can do. A reason belongs in a doc only when it changes what the
caller writes.

Two things the checker knows that are easy to get wrong when extending it: a
backticked run is code, so `` `tones::ALL` `` is a name rather than a shout; and
whether an `example.*` key is a note or the content inside a picture is decided
by the macro in `docs-site/src/examples.rs`, which the test reads rather than
guesses.

To check one file while working, without a build:

    python3 <scratch>/register.py spec/components/<name>.toml

`spec/components/checkbox.toml` is the worked reference. Its Portuguese is in
`spec/i18n/pt-BR.toml` under `component.checkbox.*`.

### Every documentation page is one shape, and it is built once

`pages::DocPage` is the only page body. A head (title, one-line lead, and one
complete working example from `examples::OVERVIEWS`), a `DocIndex`, then the
sections `ComponentSections` emits: **when · examples · variants · reference**.
`Depth` is what a collection page changes and the only thing it changes — it
used to be a `bool` and two copies of a hundred lines, which is how the two
copies drifted apart. A page whose shape differs from this is a bug, and
`tests/browser/audit.spec.js` asserts it on all fifty-six.

### The folds ship closed, so a spec opens them first

The four reference sections, and every component on a collection page, are
`Disclosure`s rendered **without `name`** — `shell.js` closes any open
`[data-nd-disclosure]` the pointer lands outside of, which is right for the
header's panels and wrong for a section a reader opened on purpose.

In a browser spec, call `showParts(page)` from `tests/browser/pages.js` after
`goto` — a spec about what a component DOES is not a spec about the fold in
front of it. That the folds are shut, keep their headings, and open when a link
lands inside one is asserted on its own.

**A TEST THAT SKIPS WHAT IS HIDDEN IS A TEST A FOLD SWITCHES OFF.** The example
frame's controls were folded behind a gear for one stage, and two sweeps that
had looked green the whole time were doing nothing: `shell.spec.js`'s
floating-surface sweep skips on `offsetParent !== null` and `audit.spec.js`'s
320px sweep skips on `display: none`, so no control inside that fold had ever
been measured against 24x24, clipped text, or overflow. Before folding anything,
check what stops looking at it.

### A specimen with no children draws the pangram, and a grid of one picture

A component that takes `children` and declares no `[meta] specimen` renders the
placeholder line instead — so `Segmented`'s six grids were sixteen copies of one
line of text. Give it a `specimen` of real child markup, and mark the words in
it `.nd-identifier`: they are a picture, not prose, and the pseudo-locale test
reads any unmarked run as a sentence somebody forgot to translate.

Where a still picture genuinely cannot show the difference, say so with
`variants = false` and a comment giving the reason. `ToolbarGroup.align` is
`margin-inline-start: auto` and needs siblings to push against; `Info.placement`
is about a note that is not open; `Heading.level` is document structure.

`<textarea>`, like `<input>`, `<meter>` and `<progress>`, is handled as a
replaced element by the specimen emitter: its content is the **value**, so
injected markup is rendered to the reader as prose.

**AN EXAMPLE'S PICTURE IS CONTENT, AND `favicon.svg` IS NOT A PICTURE OF
ANYBODY.** Every `Avatar { src }` on the site pointed at the site's own mark,
which is a ring around a dot: under `object-fit: cover` each face drew a second
circle inside the avatar's own, and a reader reported two rings where a portrait
belongs. `docs-site/assets/portraits/*.svg` are the placeholder faces, the docs
build already copies them, and `Value::Asset("assets/portraits/….svg")` reaches
them. **And several faces at one `tone` are several identical discs** — an
`AvatarGroup` whose whole subject is the overlap cannot show which of four grey
discs is in front, so each member of an example group carries its own colour.

**ONE PERSON, ONE PORTRAIT — AND THE FIRST FIX POINTED THREE PEOPLE AT `ada`.**
Eight faces are vendored and the site was using one, so a group whose subject is
who stands in front of whom drew the same stranger at every position. Ana is
`ada`, Bruno is `bob`, Carla is `cara`, wherever any of them appears; Diego has
none on purpose, because he stands only where the subject is `tone` and a
picture there covers the thing being shown. The initials stay underneath every
portrait, so blocking the files leaves four people rather than four grey discs.
`no group draws one person's face twice` in `avatars.spec.js` compares the
PIXELS, not `src`: two files that happen to be one drawing fail the same way a
repeated URL does, and a reader cannot tell those apart either.

**A PROP EVERY CHILD REPEATS IS A PROP THE GROUP OWNS.** `ButtonGroup` is
`align-items: stretch`, so every button in a vertical one is as wide as the
widest and a short label had nowhere to sit but where `justify` put it — and
`justify` was `Button`'s, repeated on each child, said `start` three times in
one example, and left *Save* floating against the leading edge. It is
`ButtonGroup.justify` now, and it **overrides** each `Button`'s own, which is
the answer `AvatarGroup.size` and `.shape` already give: siblings share their
arrangement (`§9.7`), and there is no second mechanism to learn. The stylesheet
wins by being one class deeper (`.nd-button-group[data-nd-justify="…"] >
.nd-button`), and it writes the DEFAULT out too, because `Button` always writes
its own attribute and a group that said nothing would lose to it. Assert where
the words land, never `justify-content`: the two rules are one specificity step
apart and a rule that loses reads exactly like one that works.

**A GROUP'S OWN GRID NEEDS TWO THINGS IN EACH MEMBER.** `Button.justify`
already had an `itself()` arm giving its specimen an icon; the group's needs the
same, or `center`, `start` and `between` are one picture and `variants.spec.js`
says so. And a `specimen` in a contract whose component draws ITSELF is never
reached — put the arm in `specimens.rs`, not in the TOML.

**A CHIP THAT IS SQUARE BY DECLARATION LOOKS LIKE ONE THE PRESET MADE SQUARE.**
`ColorPicker.shape = square` resolves `--noctua-radius-none`, which is `0` under
all four corner presets — so the one chip a reader meets beside an error message
was the only surface on the site that did not answer *Corners*, and under the
circular preset it was a hard square inside a capsule. The example stopped
setting it; `shape` is shown where the subject is corners. **Sweep all four
presets when a corner is the claim**, `square` included, and read the corner
PIXEL rather than the radius: the length would prove the declaration is there
and nothing about which rule won.

**THE CHECKBOX TICK IS `stroke-width` ON A 24-UNIT GRID, AND IT HAS BEEN TOO
THIN TWICE.** The icon set's own 1.75, then the box's edge and a hairline (3),
now twice the edge and a hairline (5). One number covers all three sizes because
the grid does not scale with the glyph — which is also why a weight has to be
looked at at the smallest size, where it lands on fewest device pixels. The
ceiling is the drawing: two limbs at a right angle with the short one seven
units long, so past six the notch closes and the tick becomes a wedge.
`choices.spec.js` measures the painted share at every size, with a floor and a
ceiling, and records the table it chose them from.

## Maintaining this file

Update it on your own initiative whenever something here becomes wrong or
incomplete — in the same change as the code, never as a follow-up. Nobody should
have to ask.

Keep it dense: it loads every session, so every line costs context that could
have gone to the task. **Record the command, not the value** — a count written
here rots silently, and an agent that trusts a stale number errs with confidence.

Detail that belongs to the architecture belongs in `NOCTUA.md`; this file is the
operating manual for working *in this repository*.
