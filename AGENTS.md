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

These texts are authored once, in `NOCTUA.md §3.6`, and copied here verbatim
**because the case they fire in is the one where `NOCTUA.md` is unreachable**. If
one changes, change it there and propagate.

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

Where the project stands is `NOCTUA-PLAN.md`, and it is not restated here. The
site's own scope and limits are in `docs-site/README.md` — read that before
adding a page.

## Repository map

```
spec/                   the only files a developer edits
  tokens.toml           every non-color token (Leg 1 of the parity mechanism)
  theme.toml            the theme model: every scopable setting
  i18n/{en,pt-BR}.toml  every UI message, both locales
  icons.toml            the icon sources and the drawings this system renames
  lineage/cases.toml    the blessed layouts BOTH engines are checked against
  components/           <name>.toml (contract). <name>.cases.toml — Leg 3 of the
                        parity mechanism — is NOT written, and tests/conformance/
                        is empty. What holds parity today is that both
                        implementations are generated from one file
crates/
  noctua-design/        the Dioxus package. src/theme.rs is GENERATED
  noctua-design-runtime/ the browser behaviour this system SHIPS. HAND-WRITTEN
  noctua-design-lineage/ the `Lineage` engine. HAND-WRITTEN, and `layout.js`
                        beside `layout.rs` is the same algorithm again
  noctua-design-emit/   the emitters. HAND-WRITTEN
  noctua-design-spec/   the contract reader and the prose register
  noctua-design-{tokens,theme,i18n,icons,grammars}/  GENERATED. COMMITTED
packages/
  react/                the React package. src/theme.js is GENERATED
  tokens/               GENERATED (TypeScript). COMMITTED
  tokens/vendor/        the color tokens, vendored from noctua-colors
assets/fonts/           copied from .TEMP/, + ATTRIBUTION.md + licences
assets/icons/           upstream SVGs, pinned tag, licence, SHA256SUMS
assets/vendor/          three.js and the owl model, each with ATTRIBUTION.md
dist/                   GENERATED, COMMITTED: the CSS layers and every shipped
                        behaviour module. NOT the component styles: those live
                        in docs-site/assets/shell.css and ship nowhere yet
docs-site/              the docs app. src/ and assets/ are written; dist/ is
                        build output and is gitignored. Read its README.md
xtask/                  the five verbs
.TEMP/                  staging area. Copy OUT of it; never modify it
```

**Generated directories are committed** so every consumer works with no build
step, exactly as `noctua-colors` commits `system/`. `cargo xtask check` fails if
one drifts from `spec/`.

**`build` writes changed files; it does not delete stale ones.** Removing a
component means deleting its generated `.rs`, `.js` and `.d.ts` by hand as well
as its `spec/components/<name>.toml`. The drift check has nothing to say about a
file nothing produces any more.

**The controls are components, and there is one of each.** Every interactive
thing on the documentation site is a component from the catalogue. A boolean is a
`Switch` — `role="switch"` — never a `Segmented` of "Off" and "On": a radio group
asking a yes-or-no question tells a screen reader the wrong thing about what
pressing it means. If a page needs a control, the answer is never a `<button>`
with a class on it, and never a second component for a control that already
exists — `Button { hide_label }` is what an `IconButton` would have been.

**A script hook is a prop.** `Button.action`, `Segmented.name`, `Disclosure.name`
and `Search.name` are the only way markup gets an attribute a script can find.
There is no arbitrary-attribute escape hatch, because an escape hatch on one
component is an escape hatch on all of them.

## Commands

```bash
cargo xtask build     # compile spec/ into tokens, icons, props types and dist/
cargo xtask check     # THE gate: spec, source gates, drift, fmt, clippy, tests
cargo xtask dev       # build, serve on :8080, rebuild on any change under spec/ or docs-site/
cargo xtask docs      # pre-render the site into docs-site/dist/, one build per locale
cargo xtask docs --verify   # …and run the browser suite against it
cargo xtask release   # prepare a version
```

> ⚠️ **`cargo xtask dev` is the code it started with.** It is a long-running
> `target/debug/xtask`, so editing an emitter does not change the emitter it
> uses. It watches `crates/` and `xtask/src` and **stops rebuilding** with a loud
> message rather than rebuilding wrongly — restart it and it picks up the new
> emitter. If something ever "changes back on its own", check
> `pgrep -af "target/debug/xtask"`: an older binary predating that guard will
> still silently revert a committed fix.

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
4. **No arbitrary value.** `p-[13px]` is a token literal wearing a different hat.
5. **No color token inside a custom property.** `--x: … var(--nc-…)` freezes to
   the palette of the element it is declared on, so anything reading it inside a
   `ThemeScope` paints the page's colors. Write the color at the point of use.
6. **No color read back out of the DOM.** `getComputedStyle(…).color` returns
   whatever the engine chose to serialise; comparing two of those strings proves
   nothing about the pixels. Paint into a 1×1 canvas and read `getImageData`.
   Reading a *length* that way is fine.
7. **No translated sentence built from fragments.** Word order, gender and
   agreement differ between languages. One sentence is one key, with a
   `{placeholder}` where a value goes. `NOCTUA.md §9.9` rule 1.
8. **Generated files are never hand-edited.** Change `spec/` and rebuild.
9. **The Rust side builds and tests with no Node installed.** `check` proves it
   rather than assuming it, so a Rust-only contributor never installs a
   JavaScript toolchain.
10. **Both implementations change together.** A component is four things — the
   spec, the Dioxus implementation, the React implementation, and the shared
   cases — and editing one implementation only is a compile error or a failing
   conformance test, by design. `NOCTUA.md §9.3`.
11. **No sentence reaches a reader untranslated.** Not a gate: a test. The
   pseudo-locale brackets every message, so
   `every_sentence_on_every_page_is_translated` renders each page in `qps` and
   asserts every text run carries a bracket. Anything deliberately untranslated
   — a token name, a component name, a language's own name — is marked
   `.nd-identifier` and skipped, which makes that class the page saying *this is
   a name, not a sentence*.
12. **Every component has worked examples.** `every_component_has_examples` fails
   the build for one that does not, and `the_examples_reach_every_prop` fails it
   for a prop no example sets. A page that documents a component nobody has shown
   in use documents its prop table.
13. **A prop that does nothing is a defect, not a stub.** Not a compile error and
   not visible in review: it is accepted, documented, and dropped. Check that a
   prop reaches the markup before shipping it, and check it *under the other
   props* — a value inert under another prop's default is the same defect one
   step further away.
14. **Every path and every name a document cites exists.** A stale reference is
   worse than a missing one: a missing one asks the reader to look, a stale one
   answers and the answer is wrong. `xtask/src/references.rs` reads every
   backtick and every link target in this repository's documents and comments —
   a path must resolve, and a name with two underscores must be something the
   code actually declares. **A rename is not finished until this passes.**

**The escape hatch.** Every gate honours `// allow-literal: <reason>` on the same
line, and the reason has to be a real one. It exists so an exception lands in
front of a reviewer instead of quietly disabling a gate. Where a file has several
— a hue track and a saturation field display sRGB so a reader can point at part
of it, and declare no palette colour — keep them in **one block per file, at the
top**, so a reviewer reads the exceptions once rather than meeting them scattered.

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

## The scope model, and the rule everything here descends from

**A `var()` resolves where it is DECLARED, not where it is read.** The color
system emits its whole semantic contract once, at `:where(:root)`. Give a subtree
`data-palette="amber-vivid"` and it gets amber's `--nc-neutral-*`, but
`--nc-color-surface` still holds the **root's** value. A themed component painted
from the semantic contract silently shows the page's colors, and every token
still resolves to something so nothing looks broken.

`dist/scope.css` is the fix: the same contract, re-emitted under
`:where(:root), [data-noctua-scope], [data-palette]:not(:root),
[data-theme]:not(:root)`. It **replaces** the vendored `contexts.css` in the base
link set — linking both doubles the cost for nothing.

**The generalisation, and it is a gate.** *Any* custom property whose value
contains `var(--nc-…)` freezes to the palette of its declaring element.
`frozen_color_indirection` fails the build on one. The shape the mistake actually
takes is a border: `:root { --nd-edge: 1px solid var(--nc-color-border) }` is
convenient, readable, and paints every border inside every scope in the **page's**
colour. Name the colour where you use it; the repetition is the point.

**A scope re-reads the ink as well as the contract.** Re-declaring the
`--nc-color-*` names does nothing for the properties that CONSUME them:
`body { color: var(--nc-color-fg) }` resolves at `<body>` and every descendant
inherits the computed value, so a themed subtree paints its own surface and keeps
the page's ink — near-black on near-black. One zero-specificity rule at the end
of `scope.css` fixes it; `readability.spec.js` measures it.

**A scope must paint its own background, and that rule is generated.**
Re-resolving the contract changes what a subtree's colours *mean*; it paints
nothing, so a scope that sets ink and leaves the surface transparent is a light
island on a dark page. `dist/theme.css` paints `:root, [data-noctua-scope]`, so
consumers get it and this site keeps no copy. **Never the `background`
shorthand** — it resets `background-image`, so a frame asking for a pattern gets
the colour and nothing else. The backdrop is `--noctua-backdrop`, declared on the
same selector list, and it moves ONE variable so no measured contrast pair moves
with it.

**Every theme attribute goes on ONE element.** A theme file's forced-mode rule is
compound and same-element (`[data-palette="blue-vivid"][data-theme="dark"]`).
Split the palette onto a wrapper and the mode onto a child and that selector
matches nothing, so `scope_attributes` always emits the palette when it emits
anything. **`system` is the absence of `data-theme`, never a value of it** — the
cascade resolves the system preference before anything paints, and the word is
something no stylesheet understands.

**Taking a scope OFF means removing every attribute, and the list is generated.**
`scopeAttributeNames` (JavaScript) and `SCOPE_ATTRIBUTES` (Rust) publish it.
Removing a subset is not partial un-theming but a worse bug: `scope.css` matches
`[data-palette]:not(:root)`, so a leftover `data-palette` alone keeps the whole
contract resolving there, now against the system preference, and the subtree
cannot be un-themed at all.

**Any scopable setting whose tokens are read by a property that can be invalid
needs an unscoped default**, at `:where(:root)` so an explicit `off` still wins.
Nothing stamps the attribute until a reader changes it, and a property reading an
undefined custom property is invalid at computed-value time — which resolves to
`none`, silently. `appearance.rs` carries the worked case.

**Trap 0.8 — a token that does not exist fails silently, both ways.**
`var(--noctua-space-40)` is valid CSS with no such token, so the declaration is
invalid at computed-value time and the browser drops it with no error — a grid
just becomes one column and looks like a decision somebody made.
`gates::unknown_token` collects every `--name:` declared anywhere and reports any
`var()` outside that set.

**Trap 0.5 — media queries are minimum-width, and the gate says so.** A media
query cannot read a custom property, so preludes went unchecked and `max-width`
queries got in. `gates::media_prelude` rejects `max-width` outright and requires
every `min-width` to equal a breakpoint declared in `spec/tokens.toml`. Write the
narrow case unconditionally and override it upward.

## The platform's silent failures

Each of these cost real debugging time here, and not one of them logs anything.

**An unregistered custom property reads back as its own source text.**
`getPropertyValue("--nd-pitch")` answers `var(--noctua-scene-tilt)` — the text,
not a number — so a script reading one gets a string it cannot compute with.
`@property` is what gives a custom property a type; declare one for every custom
property a script and the stylesheet share.

**A syntax error anywhere in a `transform` list drops the whole declaration.**
`translate3d(…) none` is not a list containing a no-op: `none` is a value for the
whole property. Assert that a computed `transform` exists at all.

**`100cqw` inside a container's own declarations answers for the container above
it.** An element that establishes a query container is not its own.

**`[hidden] { display: none }` is a user-agent rule, so any author rule outranks
it.** `[hidden] { display: none !important }` is the only `!important` in the
stylesheet.

**A `required` control the script has hidden makes the form silently
unsubmittable.** The browser must focus an invalid control to anchor its bubble
and cannot focus `display: none`. An enhanced control puts the browser's own
`validationMessage` on the element the reader can actually see.

**`form.reset()` fires one event on the form and none on the controls inside
it**, so an enhanced control goes on showing a value its native element no longer
holds. Beside it: a script-created `<option>` has `defaultSelected === false`, so
a reset deselects it and **leaves it in the list for ever**.

**A rule written for a prop's default value is a rule on everything.** A
`[data-nd-tone="neutral"]` rule added to make neutral "do something" repaints
every figure on the site, including the ones whose shape deliberately removed it.
The default IS the resting state.

**`overscroll-behavior: contain` on an axis the box does not scroll stops the
page scrolling over it.** Declared on both axes of a horizontal-only scroller, a
vertical wheel meets a container that cannot move with chaining forbidden. Name
the axis.

**With `box-sizing: border-box`, a box cannot be shorter than its own padding.**
Blank space that has to be scrolled through is **content**, not padding —
otherwise most of the travel is unreachable and it still looks plausible in a
screenshot.

**Half of what a no-script page shows is a control some enhanced page hides.**
Every sweep runs with a script, so a native control the enhancer hides on sight
is invisible to every check that exists. Ask the question again with the script
blocked — `material.spec.js` does.

**`ch` is not a character.** It is the advance of `0`, so a fourteen-character
word can be wider than `14ch`.

## Two rules about proving something

**Geometry is asserted, not eyeballed — and not eyeballed once.** A control that
overlaps its neighbour, a name wider than its box, or a row that hit-tests to
something else is a failing test, not a matter of taste. Check across locales,
because `qps` exists to make a fixed column overflow and only finds it if
something asks — and check the combination (a panel open inside a nested frame,
at 768px, in `pt-BR`) that a single wide English screenshot cannot show.

**A declaration that loses on specificity is indistinguishable from one that
works**, if the thing you measure would have passed regardless. Assert the
geometry — where a control's centre line is, which pixel a corner paints — never
the property.

**How a test here goes vacuous is `NOCTUA.md §19.5`.** Read it before adding a
guard, and confirm every new one by restoring the defect it was written for, in
the artefact that actually ships, and watching it fail.

## No other design system is named as a source

`gates.rs::cited_sources` refuses it, and it is the one gate that reads the raw
line rather than the line with comments stripped, because that is where the
citations were. Nothing here was taken from another design system, so naming one
is a claim about a relationship that does not exist. **Keep the fact, drop the
name**: a WCAG number, a hit-target size, an accessibility technique are facts
and stay written down. This is not about vendored work — the attributions under
`assets/` and `packages/tokens/` are obligations.

## Where the area-specific traps live

They are in the module that owns them, in its header, so they load when you open
the file rather than every session. **Read the header of the file you are about
to change.** Each of these carries measured rules, not description:

| If you are touching | Read the header of |
|---|---|
| a contract, a prop, either generated body | `crates/noctua-design-emit/src/props.rs`, `controls.rs` |
| a variant grid or a specimen | `crates/noctua-design-emit/src/specimens.rs` |
| an icon, a source, the payload rules | `crates/noctua-design-emit/src/icons.rs` |
| a `tone` | `crates/noctua-design-emit/src/tones.rs` |
| glass, the backdrop, any appearance setting | `crates/noctua-design-emit/src/appearance.rs` |
| a font face or the preload set | `crates/noctua-design-emit/src/fonts.rs` |
| which page a component is on, a title, the search index | `crates/noctua-design-emit/src/docs.rs` |
| shipped behaviour — which file does what | `crates/noctua-design-runtime/src/lib.rs` |
| an enhanced form control, or anything locale-shaped | `crates/noctua-design-runtime/src/inputs.js` |
| a table | `crates/noctua-design-runtime/src/table.js` |
| a menu | `crates/noctua-design-runtime/src/menu.js` |
| `Lineage`, or anything in three dimensions | `crates/noctua-design-lineage/src/lib.rs` |
| reader-facing prose, or the Portuguese | `crates/noctua-design-spec/src/prose.rs` |
| a source gate | `xtask/src/gates.rs` |
| a path or a name cited in prose | `xtask/src/references.rs` |
| a frame, a scope, the rail, a fold | `docs-site/assets/shell.js` |
| the router, or any enhancer that must release | `docs-site/assets/navigate.js` |
| the shell's layout, paint order, or a radius | `docs-site/assets/shell.css` |
| the page shape or the URL scheme | `docs-site/src/pages.rs` |
| the site's scope, build, or its open gaps | `docs-site/README.md` |

Two recorded gaps, because a reader would otherwise assume they are closed.
`Tabs`, `Segmented`, `Tree` and `DateInput` are enhanced from the site's script
rather than from `dist/`, so a consumer taking the packages gets the roles
without the keyboard they oblige — the fix is to move the module and let
`shell.js` import it back. And `spec/components/<name>.cases.toml` is unwritten,
so behavioural parity is demonstrated by the examples rather than asserted.

## Maintaining this file

Update it on your own initiative whenever something here becomes wrong or
incomplete — in the same change as the code, never as a follow-up. Nobody should
have to ask.

**This file is what binds everywhere.** A rule that binds one area belongs in the
header of the module that owns it, named in the table above; a rule that binds
every file belongs here. That division is what keeps this file loadable every
session — and it is the only reason the traps are findable at all, because a
header is read by whoever opens the file, and this document is read by everyone
whether or not it applies to them.

**Record the command, not the value.** A count written here rots silently and an
agent that trusts a stale number errs with confidence — and a hand-written list
of files, components or controls rots the same way. If a gate or a generated file
can answer, name that instead.

**A finished piece of work leaves a rule, never an account of itself.**
`NOCTUA.md §0.5` is the full rule, and the three destinations are:

| What you have | Where it goes |
|---|---|
| a rule that still binds | this file, or the module header that owns the area |
| evidence for an assertion | the comment beside that assertion |
| the story of the change | its commit message, and nowhere else |

The third is the one that grows without bound, and git already holds it, dated
and attached to its own diff. Detail that belongs to the architecture belongs in
`NOCTUA.md`; this file is the operating manual for working *in this repository*.
