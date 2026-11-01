# noctua-design — Agent Guidelines

> ## Everything you write to disk is in English.
>
> Code, comments, identifiers, documentation, CLI help, error messages, test names, commit
> messages, the docs site — all of it, always, **whatever language the developer writes to you
> in.** The developer often writes Brazilian Portuguese. That changes nothing. You may *reply* in
> Portuguese; you may not *produce artifacts* in it.
>
> **UI strings are the one exception.** They are i18n resources and exist in **both** English and
> Brazilian Portuguese, in `spec/i18n/en.toml` and `spec/i18n/pt-BR.toml`. The keys and the code
> around them are English; the values exist in both, and a key missing from either locale fails
> the build (`NOCTUA.md §9.6`).

This file is the operating manual for working inside `noctua-design`: the guard, the map, the
commands, the invariants, and the rules every component, stylesheet and page of the
documentation site obeys, with the traps behind them. Every agent working here reads it at the
start of the session, with `NOCTUA.md` §0–§5 and the sections its §0.2 names for this repository
(§7, §8, §9, §19). `NOCTUA.md §9` states what the rest of the project relies on; this file holds
how it is carried out here.

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

    THE WORKSPACE ROOT — THE DIRECTORY HOLDING `.noctua-workspace`,
    WHICH THE PROBE ABOVE JUST FOUND AND NAMED

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

    THE `repos/` DIRECTORY OF YOUR NOCTUA WORKSPACE — THE ONE
    DIRECTORY THAT HOLDS THE `.noctua-workspace` MARKER

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

## Local notes

An **`AGENTS.local.md`** may exist beside this file: the developer's private, machine-specific
notes. **Look for one.** It is gitignored and never committed (`NOCTUA.md §3.4`).

## What this is

**The shared design system**: tokens and components, shipped as **two packages that stay
behaviourally identical** — a Dioxus crate (`crates/noctua-design`) and a React package
(`packages/react`) — plus a documentation site, which is the repository's primary artifact.

- **It is the only place a Noctua component lives.** `noctua-central` never writes a one-off
  component: one that is missing or inadequate is fixed here, properly, complete under
  `NOCTUA.md §9.12`, and every consumer gets the fix (`NOCTUA.md §9.2`).
- **The Dioxus package must never regress**, because every Noctua Central app is Dioxus. The React
  package exists for consumers outside Noctua, and must never diverge.
- **It never defines a colour.** Not a hex, not an `oklch()`, not "just this one shade". Colours
  come from `noctua-colors`, vendored into `packages/tokens/vendor/`, and a missing colour is a
  one-line change *there*, discussed first (`NOCTUA.md §8.8`). A source gate enforces this.

Where the project stands is `NOCTUA-PLAN.md`; it is not restated here. The documentation site's
own scope, build and gaps are in `docs-site/README.md` — read it before adding a page.

## Repository map

```
spec/                     the only files a developer edits
  tokens.toml             every non-colour token (Leg 1 of the parity mechanism)
  theme.toml              the theme model: every setting, and whether it scopes
  fonts.toml              the font layer (NOCTUA.md §7.4)
  syntax.toml, grammars/  the code blocks' syntax themes and grammars
  icons.toml              the icon sources, and the icons with an opinion
  i18n/                   every UI message, in en.toml and pt-BR.toml
  lineage/cases.toml      the blessed layouts both Lineage engines answer to
  components/             <name>.toml, the contract (Leg 2), and <name>.cases.toml, the
                          cases both implementations are driven through (Leg 3)
crates/
  noctua-design/          the Dioxus package. src/theme.rs is GENERATED
  noctua-design-styles/   the component rules. HAND-WRITTEN CSS; sources() is the cascade
  noctua-design-runtime/  the behaviour this system ships. HAND-WRITTEN JS
  noctua-design-lineage/  the Lineage engine: layout.rs, and layout.js beside it, the same
                          algorithm again. HAND-WRITTEN
  noctua-design-emit/     the emitters. HAND-WRITTEN
  noctua-design-spec/     the contract reader and the prose register
  noctua-design-dist/     dist/ COPIED, so crates.io can carry it. GENERATED
  noctua-design-{tokens,theme,i18n,icons,grammars}/   GENERATED, COMMITTED
packages/
  react/                  the React package. src/theme.js is GENERATED
  tokens/                 GENERATED JavaScript and type declarations, COMMITTED
    icons/<name>.js       one module per icon, so one icon costs one
    vendor/               the colour tokens, vendored from noctua-colors
  dist/                   dist/ COPIED again, so npm can carry it. GENERATED
dist/                     GENERATED, COMMITTED: the CSS layers, components.css, every shipped
                          behaviour module, the manifests, and tokens/css/ — the colour sheets
                          base.html links, without which a consumer's page has no palette
docs-site/                the documentation site. src/ and assets/ are written; dist/ is build
                          output and gitignored. Read its README.md
tests/browser/            the browser suite (Playwright): geometry, paint, focus, accessibility
tests/conformance/        the shared case runner (Leg 3)
examples/                 two consumers that live outside the workspace
assets/fonts/             copied from .TEMP/, with ATTRIBUTION.md and the licences
assets/icons/<source>/    vendored drawings, the upstream licence, SHA256SUMS
assets/vendor/            three.js and the owl model, each with ATTRIBUTION.md
xtask/                    the verbs
.TEMP/                    staging area. Copy OUT of it; never modify it
```

**Generated directories are committed**, as `noctua-colors` commits `system/`, so every consumer
works with no build step; `cargo xtask check` fails if one drifts from `spec/`.

**`build` writes changed files and never deletes stale ones.** Removing a component means deleting
its generated `.rs`, `.js` and `.d.ts` by hand as well as its `spec/components/<name>.toml`: the
drift check has nothing to say about a file nothing produces any more.

## Commands

```bash
cargo xtask build     # compile spec/ into tokens, icons, props types and dist/
cargo xtask check     # THE gate: spec, source gates, drift, fmt, clippy, tests, independence,
                      #   references, payload. ui::heading in xtask/src/check.rs is the live list
cargo xtask dev       # build, serve on :8080, rebuild on any change under spec/ or docs-site/
cargo xtask docs      # pre-render the site into docs-site/dist/, one build per locale
cargo xtask docs --verify   # …and run the browser suite against it
cargo xtask release   # prepare a version
```

**One verb per intent, and the surface does not grow**: new capability goes behind an existing
verb or it does not ship. `cargo xtask check` is exactly what CI runs — the same command, not a
reimplementation in YAML, which is how the two would drift apart. `just <verb>` is a one-word
alias for each, and never the only path.

> ⚠️ **`cargo xtask dev` runs the code it started with.** It is a long-running
> `target/debug/xtask`, so editing an emitter does not change the emitter it uses. It watches
> `crates/` and `xtask/src` and **stops rebuilding**, loudly, rather than rebuilding wrongly;
> restart it to pick up the change. If something "changes back on its own", an older binary
> without that guard is still running and reverting it: `pgrep -af "target/debug/xtask"`.

## Invariants

Breaking any of these is a defect, not a tradeoff. Each is enforced by a gate or a test, because
a convention nothing enforces is a convention that decays. Code cites them by number: keep the
numbers.

1. **No colour is defined here.** Every colour comes from a `--nc-*` token in the vendored
   `noctua-colors` output. A hex, an `oklch()` or a named colour in this repository's source fails
   `check`.
2. **No token literal.** No raw `px`, `rem` or `ms` in component source. Spacing, radii, type
   sizes, durations and easings come from the token layer, so a change to the scale reaches every
   component.
3. **No font family named directly.** Use `var(--noctua-font-*)`. A component that hardcodes
   `"Noto Sans"` cannot honour the interface-font setting and will not fall back correctly
   (`NOCTUA.md §7.4`).
4. **No arbitrary value.** `p-[13px]` is a token literal wearing a different hat.
5. **No colour token inside a custom property.** `--x: … var(--nc-…)` freezes to the palette of
   the element it is declared on, so anything reading it inside a `ThemeScope` paints the page's
   colours. Write the colour at the point of use.
6. **No colour read back out of the DOM.** `getComputedStyle(…).color` returns whatever the
   engine chose to serialise, so comparing two of those strings proves nothing about the pixels.
   Paint into a 1×1 canvas and read `getImageData`. Reading a *length* that way is fine.
7. **No translated sentence built from fragments.** Word order, gender and agreement differ
   between languages. One sentence is one key, with a `{placeholder}` where a value goes
   (`NOCTUA.md §9.9`, rule 1).
8. **Generated files are never hand-edited.** Change `spec/` and rebuild.
9. **The Rust side builds and tests with no Node installed.** `check` proves it rather than
   assuming it, so a Rust-only contributor never installs a JavaScript toolchain.
10. **Both implementations change together.** A component is four things — the contract, the
    Dioxus implementation, the React implementation and the shared cases — and editing one
    implementation alone is a compile error or a failing conformance case, by design
    (`NOCTUA.md §9.3`).
11. **No sentence reaches a reader untranslated.** The pseudo-locale brackets every message, and
    `every_sentence_on_every_page_is_translated` renders each page in `qps` and asserts every text
    run carries a bracket. Anything deliberately untranslated — a token name, a component name, a
    language's own name — is marked `.nd-identifier` and skipped: the page saying *this is a name,
    not a sentence*.
12. **Every component has worked examples.** `every_component_has_examples` fails the build for a
    component with none, and `the_examples_reach_every_prop` for a prop no example sets. A page
    that documents a component nobody has shown in use documents its prop table.
13. **A prop that does nothing is a defect, not a stub.** It is accepted, documented and dropped,
    and neither the compiler nor a review sees it. Check that a prop reaches the markup before
    shipping it, and check it *under the other props*: a value inert under another prop's default
    is the same defect one step further away.
14. **Every path and every name a document cites exists.** A stale reference is worse than a
    missing one: a missing one asks the reader to look, a stale one answers, wrongly.
    `xtask/src/references.rs` reads every backtick and every link target in this repository's
    documents and comments; a path must resolve, and a name with two underscores must be
    something the code declares. **A rename is not finished until this passes.**

**The escape hatch.** Every gate honours `// allow-literal: <reason>` on the same line, with a real
reason, so an exception lands in front of a reviewer instead of quietly disabling a gate. A marker
on the line above does not count, because `rustfmt` is free to move a comment away from the
literal it excuses. Where a file needs several, keep them in **one block per file, at the top**,
so the exceptions are read once rather than met scattered: `ColorPicker`'s hue track and
saturation field display the whole of sRGB so a reader can point at part of it, declare no
palette colour, and are the worked case (see "Component rules").

## Design rules that are not preferences

`NOCTUA.md §9.7` and §9.8 bind every component and every page; read them. What they come to here:

- **Very clean**: simple, objective, nothing decorative.
- **Mobile friendly from the first commit, and every window size**, down to a very small one: a
  component adapts rather than overflowing.
- **Text and components sit directly on the background. No cards**, and no shadow anywhere.
  `--nc-color-surface-raised` is for the rare case that genuinely needs separation, and reaching
  for it is a decision to justify.
- **Generous spacing, one scale.** Siblings share spacing, padding, radius and **height** — a row
  of controls that disagrees on any of the four reads as broken. Height is the one that goes
  unnoticed, because nothing declares it (see "The platform's silent failures").
- **Large default title and body sizes, and a narrow default measure** whatever the screen:
  `--noctua-measure` caps prose in `ch`. The page may be wide; the *line* may not.
- **No layout shift, no flickering.**
- **Every input and every seat.** Keyboard, pointer, touch and touchpad all work, and every
  interactive state is styled through the native pseudo-class **and** the seat attribute beside
  it, so a second seat's hover or focus can be shown.

## Where the area-specific traps live

A trap that binds one area is in the header of the module that owns it, so it loads when you open
the file rather than every session. **Read the header of the file you are about to change.** Each
of these carries measured rules, not description:

| If you are touching | Read the header of |
|---|---|
| a contract, a prop, either generated body | `crates/noctua-design-emit/src/props.rs`, `controls.rs` |
| a component that hosts an `AILabel`, or the mark itself | `crates/noctua-design-emit/src/controls.rs` (`graft`), `crates/noctua-design-spec/src/component.rs` (`AI_PROPS`) |
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
| a switch, a segmented choice — anything that answers a click by choosing | `crates/noctua-design-runtime/src/choice.js` |
| what both implementations must *do* | `spec/components/<name>.cases.toml`, and `crates/noctua-design-spec/src/conformance.rs` for the vocabulary a step may use |
| `Lineage`, or anything in three dimensions | `crates/noctua-design-lineage/src/lib.rs` |
| reader-facing prose, or the Portuguese | `crates/noctua-design-spec/src/prose.rs` |
| a source gate | `xtask/src/gates.rs` |
| a path or a name cited in prose | `xtask/src/references.rs` |
| a frame, a scope, the rail, a fold | `docs-site/assets/shell.js` |
| the router, or any enhancer that must release | `docs-site/assets/navigate.js` |
| a component's own rules | `crates/noctua-design-styles/src/` — one file per family, and `sources()` is the cascade |
| the shell's layout, paint order, or a radius | `docs-site/assets/shell.css` |
| the page shape or the URL scheme | `docs-site/src/pages.rs` |
| the site's scope, build, or its open gaps | `docs-site/README.md` |

## How a component is built

A component is four things that change together — its contract, the Dioxus implementation, the
React implementation and its cases — and this section is how the four are kept one
(`NOCTUA.md §9.3`).

**The contract** is `spec/components/<name>.toml`, the only place a prop is declared. Three of the
four parts are generated from it, so a prop that one implementation has and the other lacks
cannot be expressed. It declares the element and the root class, every prop with its type, values,
default, attribute and doc, the accessibility facts, any `[state]` the markup carries, and the
keyboard contract — declared rather than described, because the conformance suite cannot assert
a hope. `crates/noctua-design-spec/src/component.rs` is the full shape:

```toml
guidance = "When to use it, and what to use instead."

[meta]
name    = "Button"
element = "button"          # the DOM element, not negotiable
class   = "nd-button"       # the root class every rule hangs from
summary = "Let users carry out an action with one press."

[prop.variant]
type      = "enum"
values    = ["solid", "outline", "subtle", "transparent", "link", "nav"]
default   = "solid"
attribute = "data-nd-variant"
doc       = "How much of the page's attention the control asks for."

[a11y]
needs_name = true
notes      = "Facts about roles and attributes."

[keyboard]
Enter = "Activates the control."
Space = "Activates the control. The button form only."
```

**The cases** are `spec/components/<name>.cases.toml`: given props and a sequence of interactions,
what the DOM holds afterwards.

```toml
[[case]]
name   = "Space activates the button form"
note   = "Why this case exists. A case nobody wrote a reason for is one nobody dares change."
props  = { label = "Save", action = "case" }
steps  = ["focus", "key:Space"]
expect = { focus = "root", attrs = { "aria-disabled" = "false" }, absent = ["aria-pressed"] }
```

A contract that declares a `[keyboard]` or a `[state]` owes a cases file, and `cargo xtask check`
refuses one without it; a component that declares neither owes nothing, because there is nothing
to drive. `ls spec/components/*.cases.toml | wc -l` counts them. **The vocabulary is deliberately
small** — `focus`, `click`, `key:`, `type:`, `blur`, and expectations over attributes, absence,
presence, visibility, text, focus and a control's value — because a case language that can express
anything becomes a second test framework, and the cases stop reading as a specification. What a case cannot say is a browser test in
`tests/browser/`. `crates/noctua-design-spec/src/conformance.rs` holds the vocabulary.

**How both halves are run.** `cargo xtask docs` writes `docs-site/dist/conformance/index.html`: one
section per case, the Dioxus half server-rendered and the React half mounted from the same tree by
the same bundle the site's implementation switch uses. It is in no index and no search; it is
apparatus. `tests/conformance/cases.spec.js` drives **one case per page load**, because the
behaviours are delegated and several are document-wide, and on a page holding every case one
case's key press is another case's handler's business.

**Each half answers to the file, and then the two are diffed.** Two implementations that agree
with each other and disagree with the specification is the failure that actually happens, and a
diff between them cannot see it; the diff is the second question, and it catches what nobody
thought to write down. **It canonicalises two things and nothing else**: a boolean attribute's
value (`disabled=""` against `disabled="true"` is one state spelt by two serialisers) and a
`style` attribute's declarations (`--x:1` against `--x: 1;`).

**And a person looks.** The site renders every example in Dioxus and swaps it to React in place,
from the header or from the example's own frame: automated tests catch what they were told to
look for, and a person looking at two renderings of the same button catches what nobody thought to
assert.

**Neither implementation may do what the other cannot.** Dioxus receives `children` as an opaque
node with no length, so React does not count its children either. `dioxus-html` has no `<search>`
element, so `Search` renders `<div role="search">` in both. A component that needs to see its
whole structure — a graph, a block of code — takes it as a text prop and parses it, because a
Dioxus component cannot look inside its own children.

**A script hook is a prop.** `Button.action`, `Segmented.name`, `Disclosure.name` and
`Search.name` are the only way markup gets an attribute a script can find. There is no
arbitrary-attribute escape hatch, because an escape hatch on one component is one on all of them.

**A prop many components share is grafted, not written** (`NOCTUA.md §9.12.1`). `[meta] ai` in a
contract adds the props that forward an `AILabel`, with one doc each and **one** message key for
that doc — `component.ai.prop.*`, not one per host. A hand-written body marks where the mark goes
with a `// @ai` line, and the emitter fills it. The hosts are never listed by hand:
`grep -l '^ai *= ' spec/components/*.toml` answers.

**Behaviour ships when a component is nothing without it** (`NOCTUA.md §9.4`). Every component's
behaviour lives in `crates/noctua-design-runtime/`, is copied into `dist/` by an emitter exactly as
the stylesheets are, and is imported back by `docs-site/assets/shell.js` rather than kept twice. A
component whose markup writes a role that promises a keyboard — `listbox`, `tree`, `tablist` —
ships that keyboard: `Select`, `TagPicker`, `Tree`, `Tabs`, `DateInput`, `Search` and the rest.
What they share ships with them: `anchor.js` places a floating panel against its control,
`panels.js` decides which floating list is open, and `roving.js` is the one arrow walk for a radio
group, a toolbar and a tab list. Every module exports an `enhanceX(root)` and a `releaseX(root)`,
because a behaviour that cannot be taken back off leaks once per route. `Search` reads its index
from the path its consumer gives it (`[prop.src]` writes `data-nd-src`), and it matches and
colours by the fuzzy-finder doctrine (`NOCTUA.md §12.4`): every query word must match, and the
matched characters are coloured by the word that matched them.

## Styling

How the styling is built and how it reaches a consumer (`NOCTUA.md §9.4`, §9.11.1).

**No Tailwind here.** A component is a semantic class — `.nd-button`, `.nd-heading` — declared as
`[meta] class` in its contract and styled by hand-written CSS over the tokens. Nothing scans a
source file for class names, so a consumer has no `@source` path to configure. The gate against a
Tailwind arbitrary value stays, because such a class could still appear in a hand-written rule and
bypass the token layer.

**What `dist/` carries**: the token layer, the faces, the appearance layer, the scoped semantic
contract and `components.css`, plus every behaviour module and the palette and theme data they
read. `ls dist/` is the inventory, and `cargo xtask build` writes it.

**The component rules are hand-written and copied, never generated.**
`crates/noctua-design-styles/src/` holds one stylesheet per family, and **`sources()` there is the
order they load in, which is the cascade**: two rules that tie on specificity are decided by which
comes first, so a reordering has to be argued for. The emitter concatenates them into
`dist/components.css` with a generated header, as `runtime.rs` copies the behaviour modules, and
the sheet is linked **last** of the base set, after `theme.css`.

**A rule belongs to the documentation site only if its selector names a class no component can
emit.** Every other rule is a component rule and lives in `crates/noctua-design-styles/src/`, so a
consumer outside this repository gets styled components, not merely themed ones.
`docs-site/assets/shell.css` keeps the site's own chrome and nothing else, and it is linked after
`dist/components.css`, so documentation can override a component and never the reverse.

**Two token namespaces.** This repository emits `--noctua-*`; `noctua-colors` emits `--nc-*`. A
shared prefix would make "which repository owns this name" unanswerable and let a design token
shadow a colour token, so the spec rejects `nc` as a prefix.

**`--nd-*` is public surface, not a token.** These are the properties the component rules read —
a veil ratio, a tint, the height a fixed panel hangs from — declared in `base.css`, because a
`var()` naming a property nothing declares is dropped with no error.
`grep -- '--nd-' crates/noctua-design-styles/src/base.css` is the list. A consumer may set them,
and nothing checks or versions them. Promoting one to `--noctua-*` renames public surface, so it
is a change of its own, never made under cover of another.

**No component sets the page's typeface.** A design system does not restyle the page it is
installed into, so `--noctua-font-ui` and `--noctua-font-display` are handles the *page* reads —
`body { font-family: var(--noctua-font-ui); }` — and `dist/fonts.css` and the README say so. A
component names a face only where its content needs that one, as code needs the mono face, and
always through `var(--noctua-font-*)` (invariant 3).

**Every duration is a pair.** `spec/tokens.toml` declares each duration with its reduced-motion
collapse, and the emitter writes the value at `:root` and the collapse inside
`@media (prefers-reduced-motion: reduce)`, so forgetting the preference is impossible rather than
silent. Not every collapse is `0ms`: a colour or opacity change at zero reads as a glitch, while a
*movement* at zero is what a motion-sensitive reader asked for.

**A token that does not exist fails silently.** `var(--noctua-space-40)` is valid CSS with no such
token: the declaration is invalid at computed-value time and dropped with no error, and a grid
just becomes one column that looks deliberate. `gates::unknown_token` collects every `--name:`
declared anywhere and reports any `var()` outside that set.

**Media queries are minimum-width.** A media query cannot read a custom property, so the token
gate cannot reach a prelude; `gates::media_prelude` rejects `max-width` outright and requires
every `min-width` to equal a breakpoint declared in `spec/tokens.toml`. Write the narrow case
unconditionally and override it upward.

**The source gates do not walk the compiler crates.** They are about component and styling
source: a component hardcoding `13px` sits between two steps for ever, while a compiler that
formats `0.25rem` into CSS must contain the string `0.25rem`. The exclusion is a claim, and a test
asserts that component source is still covered. `xtask/src/gates.rs` is the list of gates.

**Generated Rust passes through `rustfmt` during the build.** The generated crates are workspace
members, so `cargo fmt --all` formats them; running `rustfmt` in the emitter is what stops `check`
reporting a file both out of sync and badly formatted, with no edit that fixes both.

## The scope model, and the rule everything here descends from

A theme applies to a page, to one component, or to a component inside that one
(`NOCTUA.md §9.11.2`). Every rule below follows from one fact.

**A `var()` resolves where it is DECLARED, not where it is read.** The colour system emits its
whole semantic contract once, at `:where(:root)`, each name an indirection onto a ramp token. Give
a subtree `data-palette="amber-vivid"` and it gets amber's `--nc-neutral-*`, but
`--nc-color-surface` still holds the **root's** value: a themed component painted from the
contract silently shows the page's colours, and every token still resolves to something, so
nothing looks broken.

`dist/scope.css` is the fix: the same contract, transcribed from the vendored output and
re-emitted under `:where(:root), [data-noctua-scope], [data-palette]:not(:root),
[data-theme]:not(:root)`. It carries no palette and no mode, because every declaration is an
indirection that resolves against whichever element matched. It **replaces** the vendored
`contexts.css` in the base link set; linking both doubles the cost for nothing.

**A colour token inside a custom property freezes, and that is a gate.** *Any* custom property
whose value contains `var(--nc-…)` freezes to the palette of its declaring element, and
`frozen_color_indirection` fails the build on one; `dist/scope.css` is the one legitimate case,
and it is generated. The mistake's usual shape is a border:
`:root { --nd-edge: 1px solid var(--nc-color-border) }` is readable and paints every border inside
every scope in the **page's** colour. Name the colour where you use it; the repetition is the
point. It is also why the glass tokens are dimensionless numbers and the `color-mix()` is written
at the point of use.

**A scope re-reads the ink as well as the contract.** Re-declaring the `--nc-color-*` names does
nothing for the properties that consume them: `body { color: var(--nc-color-fg) }` resolves at
`<body>` and every descendant inherits the computed value, so a themed subtree paints its own
surface with the page's ink. One zero-specificity rule at the end of `scope.css` fixes it, and
`readability.spec.js` measures it.

**A scope paints its own background, and that rule is generated.** Re-resolving the contract
changes what a subtree's colours mean and paints nothing, so a scope that sets ink and leaves the
surface transparent is a light island on a dark page. `dist/theme.css` paints
`:root, [data-noctua-scope]`, so consumers get it and the site keeps no copy. **Never the
`background` shorthand**: it resets `background-image`, so a frame asking for a pattern gets the
colour and nothing else. The backdrop is `--noctua-backdrop`, declared on the same selector list;
it moves one variable, so no measured contrast pair moves with it.

**Every theme attribute goes on ONE element.** A theme file's forced-mode rule is compound on one
element (`[data-palette="blue-vivid"][data-theme="dark"]`); split the palette onto a wrapper and the
mode onto a child and it matches nothing, so `scope_attributes` always emits the palette when it
emits anything. **`system` is the absence of `data-theme`, never a value of it**: the cascade
resolves the system preference before anything paints, and no stylesheet understands the word.

**Taking a scope OFF removes every attribute, from the published list.** `scopeAttributeNames`
(JavaScript) and `SCOPE_ATTRIBUTES` (Rust) publish it, beside `scopeAttributes`. Removing a subset
is worse than removing none: `scope.css` matches `[data-palette]:not(:root)`, so a leftover
`data-palette` keeps the whole contract resolving there, now against the system preference, and
the subtree can no longer be un-themed. A teardown uses the published list, never a hand-written
one.

**A scopable setting needs an unscoped default** wherever a property that can be invalid reads its
tokens. Scopability means a bare attribute selector — `[data-noctua-glass="subtle"]`, not
`:root[…]` — so until something stamps the attribute no block matches and the tokens are
undefined. `backdrop-filter: blur(var(--undefined))` is invalid at computed-value time, which
resolves to `none`: every frosted surface renders flat on a first visit, and nothing reports it.
The default is declared at `:where(:root)`, at zero specificity, so an explicit value still wins;
`appearance.rs` carries the worked case.

**Base link order is load-bearing, and fails silently.** In system mode the default palette's
`:root:not([data-theme="light"]):not(.light)` and a theme sheet's
`[data-palette="x"]:not(…):not(…)` both have specificity (0,3,0), so only source order separates
them, and the default is linked first. `base.html` is the order.

**Nothing a page is painted with moves.** There is no ambient animation, on the developer's
instruction, and the emitter's test fails on any `animation:` or `@keyframes` in the appearance
sheet, so whoever animates a background next answers the reduced-motion question deliberately.
The question is real: an infinite animation cannot reuse `[motion.duration]`, whose durations
collapse under reduced motion, and `animation-duration: 0ms` on an infinite animation is not "no
motion" — it completes and restarts every frame, and engines disagree about what renders.

**A ceiling is per pixel, so a sparse pattern needs its own.** `[effect.alpha] pattern = 4` is the
most any background pixel may move toward the text colour, right for `mesh`, `checks` and
`diagonals`, which cover most of their tile. `dots` covers about a fortieth of its own, so it has
`[effect.alpha] dots = 10`. The measurement that permits it is what holds it: every pattern, on
every backdrop, in both modes, against the darkest ink a heading paints, at 4.5:1.

## Component rules

What each component, or family of components, must do, and why. They derive from `NOCTUA.md`
§9.4, §9.7 and §9.8; the module headers in "Where the area-specific traps live" carry the measured
traps.

### Forms

**One component draws the question above every control: `Field`** — the label, the hint, the
sentence that went wrong, and the `<fieldset>`/`<legend>` a *set* of controls needs. **Two
components arriving independently at one arrangement is the criterion for extracting it into a
shared component**, and `Field` is that extraction. Its `error_slot` renders the error paragraph
even when there is no error, empty, so the browser's own complaint has somewhere to go when a
script has hidden the real control.

`Field` cannot write `aria-describedby` on its own child, so it derives `{control}-hint` and
`{control}-error` and each control computes the same two strings. That is a convention, and
conventions decay, so the browser suite asserts that *every describedby points at something, on
every page, in every locale*. **A group describes itself**: with `kind = "group"`, `Field` is the
`<fieldset>` and points `aria-describedby` at its own hint. `Checkbox` and `Radio` do not use
`Field`: their label sits in the row beside the box and is the box's own name.

**A wrong answer is marked once, and the same way everywhere**: a rule down the leading edge of the
**whole question** — the words, the hint, the sentence and the controls — plus the control's own
outline and the sentence itself. The rule is `--noctua-border-rule`, a third step beside
`hairline` and `strong`, thick enough to be noticed, because being noticed is its whole job. **The
question itself does not change colour**: a label coloured like the complaint reads as part of it.
The one mark answers a script's refusal and a server-rendered `error` alike.

**A refusal is said twice, in the two places a reader will be.** `ErrorSummary` and
`ErrorSummaryItem` list everything wrong at the top of the form, *beside* the field sentences:
the field says what is wrong where the answer is, and the summary says it where the reader is
after a submit. **It takes the focus, and that is the whole component**: `tabindex="-1"` on the box
and `role="alert"` on the content inside it, because focus alone is silent where a script could not
move it, and an alert alone leaves a reader at the bottom of a form being told about the top. It
is **not a landmark**: `role="region"` would put *there is a problem* in every landmark list for the
rest of the visit. Every entry is a real `<a href>` to a real id, so it works with the script
blocked. `dist/error-summary.js` decides where the focus **lands** — the first visible control in
the same field, never further. With `collect`, it catches the browser's own refusal, writes the
browser's `validationMessage` into the empty paragraph `Field.error_slot` leaves, and builds the
list from the fields: **no word of language is invented**, a sentence the caller wrote is never
overwritten, and the platform's bubble is refused because the same sentence is about to appear
where the reader looks. The controls that take a `required` keep that paragraph for themselves.
**It is never rendered empty**: a box saying *there is a problem* above a form with none teaches
readers to ignore it.

### Notes and menus

**A note opens on a press, never on a hover.** A bubble that appears only under a pointer does not
exist on a telephone or for a keyboard. `Info` is the small `i` beside a word: a real `<button>`
with `aria-expanded`; Escape closes it and returns the focus, and a click outside closes it. It is
`role="note"`, not a dialog: nothing behind it is inert and focus is not trapped. It calls
`anchor.js` and hangs from its *leading* edge, so the anchor writes both distances and the
stylesheet picks. **A note opened inside a `Dialog` covers the dialog's own text, and that is the
behaviour** (decided on 2026-08-26): every note is a `position: fixed` panel anchored to the
control that opened it, and teaching `anchor.js` about the surface would change its other callers
to solve a problem the reader does not have.

**A component may not announce what nothing can deliver.** `Menu` and `MenuItem` are *a button
that opens something living elsewhere in the document* — `Disclosure` is `<details>`, whose panel
is its own child — and `Button { menu }` names its `Menu` with `controls`. `dist/menu.js` carries
the keyboard a menu owes: a roving `tabindex` that wraps, Home and End, a printable key that jumps
and a run of them that spells a word, and Escape or a chosen item returning the focus to the
button. **A menu item is `aria-disabled`, never `disabled`**, because an item that leaves the arrow
walk is an action a reader concludes does not exist; and **`aria-expanded="false"` is true on a
menu button**, because a control that opens something opens it whether or not it is open now, so
the attribute's absence would be the false statement.

### Tones, bars, carousels, spectra

**A tone is a name, not a list of four.** The vendored palette declares hundreds of semantic
families — `danger` and `success`, and also `overdue`, `shipped`, `degraded`, `canary`,
`chargeback` — each with a foreground, a background, a border and a readable ink, and a component
offering a fixed handful would lose distinctions the colour system already draws. `dist/tones.css`
is generated from the vendor's own list, one block per family declaring `--nd-tone`,
`--nd-tone-bg`, `--nd-tone-border` and `--nd-tone-on`, so a component writes **one** rule and every
name works, including families added later. Where a component takes any family, `tone` is a `text`
prop checked against the generated `tones::ALL` at build time: a name no family declares fails the
build instead of quietly resolving to nothing. **No tone is the absence of the attribute**, never a
name for it, because the palette's own `default` family is an info blue.

**`Tag`'s tinted and outlined variants paint in the page's ink** and say their family through the
field and the hairline, as `AILabel` does; `tags.spec.js` measures every non-solid pill from
pixels. A solid fill and a pressed toggle are excluded there for the reason `ai-label.spec.js`
gives — `on-<family>` over `<family>` is the palette's own pair — and a refused tag because
WCAG 1.4.3 exempts an inactive control.

**A bar is two components, because a level is not a task.** `Meter` is a measurement in a range
known before anybody looked — a disk, a quota, a budget — and it can go down; `Progress` is a task
that starts at nothing, only advances, and finishes. Screen readers announce the two differently,
and the wrong one announces a disk as *loading*. Both are the real elements, so **neither has a
runtime**, and `Meter`'s colour comes from `low`/`high`/`optimum`, so the platform decides where
trouble starts. The number is always written out beside the bar. The indeterminate sweep is drawn
by the stylesheet, because `appearance: none` takes the browser's own animation away, and it stops
under `prefers-reduced-motion` because the stylesheet says so.

**A carousel is a scroll container, and autoplay is a contract.** `Carousel` puts content off
screen deliberately, which every other rule here refuses, so everything in it makes that honest:
the slides are a real list in a real `overflow` box with `scroll-snap`, and the runtime calls
`scrollTo`, so dragging, flicking, the trackpad, the scrollbar, Home and End and a reader's
reduced-motion setting all keep working. **Autoplay's four rules are enforced by the runtime**: it
refuses to start without a named pause control (WCAG 2.2.2, made a condition of starting); it never
starts under `prefers-reduced-motion`; it pauses under a pointer, under focus and in a hidden tab;
and it stops for good on the first manual interaction.

**A spectrum defines no colour.** `ColorPicker`'s hue track displays the whole of sRGB so a reader
can point at part of it, and its saturation field's axes end at white and at black because that is
where HSV's axes end. None of it is the palette, none of it follows a palette change, and what
comes out is the consumer's data. The lines marked `allow-literal` in
`crates/noctua-design-styles/src/color-picker.css`, and one block of test fixtures per spec, are
the whole of it — `grep -c allow-literal` counts them — and every other surface, border and ring in
the component is an `--nc-*` token.

### Tables

**A column is a thing, and that is what makes a table a grid.** Width, alignment and sortability
belong to the column, not to each heading cell: re-deriving a column from cell positions sorts
`10` before `9`, and sorts the wrong column under a heading that spans two. `TableColumn` is one
real `<colgroup>` per column, declared before any row, carrying `width`, `sortable`, `sort_as`,
`filterable`, `resizable`, `hideable`, `hidden` and `pinned`; several columns may pin. **The
comparator is declared, never sniffed.** Every table part restates its ARIA role in the markup, so
a layout change cannot strip the semantics.

**It is a table until something can be driven, and the switch is derived.** `role="table"` leaves
the arrow keys to a screen reader's own table navigation, which is better than anything a script
offers. Declare a sortable, filterable or resizable column, or a row that can be chosen or opened,
and the same markup becomes `role="grid"` with **one** tab stop and a roving `tabindex`. There is
no prop for it, so it can be neither set on a table with nothing to drive nor forgotten on one
that has. **One tab stop means one**: a widget inside a cell leaves the tab order and is reached by
entering the cell — `F2` always, `Enter` where `Enter` does not already choose, `Escape` back out —
and inside a cell the arrow keys belong to what is in it.

**A search is one box behind a magnifying glass**, not a box in every filterable heading. A column
may declare `low`/`high` and three tones, so **a status comes from the value**, recomputed when a
search hides a row. A `<tfoot>` may hold a subtotal, a tax line and a total, each at its own
weight, each recounting from the rows that are showing, and each keeping the shape the server
rendered: a currency total must not come back as a bare `6820`. Several `<tbody>` groups fold
independently; `more_src` fetches the next rows as `<tr>` markup, never JSON, because a schema
invented for it goes stale the first time a cell gains a `Tag`; and the export menu writes a CSV
file, a JSON file, the clipboard and the printer from one reading of what is showing.

**The narrow layout is a container query on the scroller**, never a media query on the window, so
a table in a narrow column stacks on a wide screen. `dist/table.js` adds three-state sorting,
filtering that hides rather than removes, a foot that recounts from the rows still showing,
resizing by pointer *and* by Alt with the arrows, column visibility, several pinned columns, and a
copy that writes tab-separated text and never reads the clipboard. Every sentence it says is the
caller's, with named holes; every number it writes is `Intl`'s, in the document's own locale.

**No prop value names a domain.** `TableRow.emphasis` is `subtle` or `strong` — **weights, not
meanings** — answering *where does this section end* and *which line is the answer*: a subtotal
and a total in a ledger, a regional count above a fleet-wide one, a baseline above the run it is
compared to. Values named `subtotal` and `total` would put accounting inside a component that has
nothing to do with accounting.

**Table layout rules, each a mechanism that fails silently:**

- **Nothing is ever hidden two ways.** A column hidden by `visibility: collapse` on its
  `<colgroup>` *and* by `hidden` on its cells makes automatic table layout take the next column
  with it.
- **A declared column width is a preference.** It must not switch the table to
  `table-layout: fixed`, where every column that declared no width takes an equal share and one
  drag resets all the others. The dragged column grows, and the rest share what is left.
- **The heading row matches the leading cells a row draws.** A row draws its own leading cells for
  a choice control or an expander; the runtime inserts the matching `<th>`, and a browser test
  walks every table checking that each heading starts where its own cell starts.
- **A stacked layout never sets `display: block` on the `<table>`**: its `<caption>`, a
  `display: table-caption` box, then has no table to caption and shrinks to its own `min-content`.
- **A variant stage disables three things before a specimen can size itself**: `container-type`
  (which is `contain: inline-size`, so the scroller's intrinsic width is zero), the scroller's
  overflow, and the table's own `inline-size: 100%`. Any one left in place turns every `width`
  variant into an empty box.
- **The un-stacking container query restores alignment** as well as `display`, `padding` and
  `position`. The stacked layout flattens every cell to `text-align: start`, and a `numeric` column
  left flattened is invisible in review: the words move to the end and the values under them do
  not.
- **A `colspan` is one child and several columns.** The runtime and its test count columns, never
  children; counting children reads a two-level heading as a head one cell short, and every word
  lands one column right of what it names.
- **A spanning heading has to be true.** A top level that describes none of the columns under it
  teaches a reader to ignore top levels.

### Specimens and variant grids

**A specimen is the component, not a bare element with one attribute.** Everything here is painted
by compound selectors — `.nd-button[data-nd-variant="solid"][data-nd-tone="danger"]` — so a
specimen carrying only the prop under test matches no rule and draws the browser's own grey
control. A specimen carries every enum prop at its default, then the one under test; bools are
excluded, because `aria-pressed="false"` is meaningful and would state the opposite of the
contract. **A void element cannot hold its own picture**: an `<input>` closes itself and the figure
lands beside it, so a rule written for a part inside the component's root misses on a form
control. A replaced `<meter>` never renders fallback content. **A rule enforced in one of three
call sites is not a rule.**

**No two values of one prop may draw the same picture**, asked of every component page. The
fingerprint reads the platform's own `::-webkit-*` parts, because `<meter>`, `<progress>` and a
range `<input>` are painted there and nowhere else. `variants = false` is how a prop says its
values cannot be told apart in a still picture (a comparator, a band, a scope, a computed total),
and a prop may carry its own `specimen` where the component's cannot show it: `align` needs a
value short enough to have room to move, `wrap` one long enough to break.

## Icons

How the icon set is built, and why its shape is what it is (`NOCTUA.md §9.5`).

- **The drawings are vendored by hand** at a pinned tag under `assets/icons/<source>/`, beside the
  upstream licence and a `SHA256SUMS` recorded when they were fetched. The build reads them from
  there — no network, no Node — normalises them (the `viewBox` taken off the wrapper, no `id`,
  `class` or `style`, `fill` and `stroke` only where the value is `currentColor` or `none`), and
  emits the constants, the modules, the manifest and `assets/icons/ATTRIBUTION.md`.
  `cargo xtask check` verifies every drawing against its checksum.
- **A source may be swept.** `spec/icons.toml` declares a `[source.*]` with `sweep = true`, and
  every drawing in its directory becomes an icon named after its file. An `[icon.<name>]` table is
  only for what a filename cannot say: a deliberate rename, a hand-written `doc`, or
  `paint = "fill"`. **A table claims its file**, so `close` ← `x.svg` produces `close` and no `x`.
  In a source that is not swept, a drawing nothing declares fails the build.
- **An icon carries upstream's name unless this system renamed it on purpose**, and every rename
  records its reason in the spec.
- **`Icon` takes a drawing.** In Rust it is an associated `const`, `IconDrawing::CHEVRON_DOWN`; in
  JavaScript it is one module per icon,
  `import { chevronDown } from "@noctua-world/design-tokens/icons/chevron-down.js"`. A typo is a
  compile error, and an import path that names no icon fails before a type checker is consulted.
- **The payload claim is measured.** `cargo xtask check` builds
  `crates/noctua-design-icons/examples/one_icon.rs` in release and reads the binary: the icon it
  draws is in it, and three it does not draw are not. `tests/browser/bundle.spec.js` does the same
  on a real esbuild bundle and caps its size. A third gate refuses any importable aggregate of
  drawings: the measurements prove the property, and that gate keeps the shape that makes it true.
  **Do not add a barrel module or a re-export of the whole set.**
- **`dist/icons.json` is the catalogue, and no page fetches it**; a browser test asserts so. It is
  what an icon picker or a tool reads at run time: lazy, cacheable, outside every bundle. In Rust
  the equivalent is `ALL` and `IconDrawing::parse`, behind the `catalogue` Cargo feature, which
  only the documentation site enables.
- **A script never carries a drawing.** It clones a `<template>` the component rendered, or the
  component renders both drawings and the state picks one.

## Theme and language

The details behind `NOCTUA.md §9.6`: the bootstrap, the message catalogue, and how the site
resolves a reader's language.

**The bootstrap** is emitted by `crates/noctua-design-emit/src/theme.rs` and carries all five of
the section's lessons. Lesson 3 is a test to write, not a hope: wherever a writer and a reader
share a storage key, assert that they agree on its name.

**A sentence a reader sees is a message even when it lives in a contract.** A contract carries
prose that becomes the doc comment in both languages — its guidance, each prop's `doc`, the
keyboard contract, the accessibility notes — and the site renders the same strings in every
locale. So `component::derived_messages` derives them into the catalogue, and the cross-locale
check demands a translation for each. English is written once, in the contract, where it is also
the doc comment; Portuguese is written once, in `spec/i18n/pt-BR.toml`.

**The pseudo-locale is a test, not a screenshot.** `qps` brackets and lengthens every message, so a
page rendered in it shows unbracketed text exactly where something bypassed the catalogue
(invariant 11).

**The site is built once per locale**, one directory each, at `<locale>/<slug>/index.html`, so every
asset path is one constant and no page is half-translated. The header's language control navigates
between the pre-built pages and offers the locales and **Automatic**, the default, which follows
`navigator.languages`. Anything a *script* writes takes its wording from a `data-` attribute, never
from a string in the script.

**A URL that names its language is a shared link, and it wins.** Automatic redirects only when the
reader followed a link inside the site, or when the document is a door (`/` and `/<locale>/`, which
exist only to send a reader somewhere). The test is a fresh `navigate` **and** a same-origin
referrer: a reload sets the referrer to the page itself, so the referrer alone would bounce a
reader out of the language they chose, and `back_forward` sets it to wherever they came from, so
Back would send them forward again.

**The resolver never builds a URL.** The destinations are written into the inline script at build
time, one per locale, and it picks from that table, so neither a stored value nor a
`navigator.languages` entry is ever concatenated into a path. Unrecognised input resolves to the
default, and an automatic match against nothing means *stay where you are*: both directions fail
closed (`NOCTUA.md §2.4`).

**A locale nobody is offered is not a reader's.** Automatic resolution applies only where the
document's locale is one the language menu offers. `qps` is not: it exists to break layouts
during review, is absent from the menu and from every link, and is reached only by typing it. It
is a routed locale like the other two, browsable link by link, and a row in the width sweep, which
is where a layout that fits only because English is short shows up.

## The documentation site

The rules of `docs-site/`, from `NOCTUA.md §9.10`. Its scope, build and open gaps are
`docs-site/README.md`.

**Every interactive thing on the site is a component from the catalogue, and there is one of
each.** A boolean is a `Switch` — `role="switch"` — never a `Segmented` of "Off" and "On": a radio
group asking a yes-or-no question tells a screen reader the wrong thing about what pressing it
means. A page that needs a control never gets a `<button>` with a class on it, nor a second
component for a control that exists: `Button { hide_label }` is the icon button.

**What a component owes its consumer ships from `dist/`, and `shell.js` imports it back.** What
stays in `docs-site/assets/shell.js` is the site's own furniture — the drawer, the settings panel,
the example frames with their viewport rig and implementation switch, the icon gallery — and it
ships nowhere. A countdown that a message owes every reader is the component's; a control that
raises the same message again is the page's (`spec/components/toast.toml` records the split).

**The header is one line at 320px.** It holds the controls that are not settings — the drawer,
the repository, the language, the implementation switch — and the one theme menu behind a palette
icon, which holds at least the mode (light, dark or system), the accent, the saturation and the
interface-font toggle (`NOCTUA.md §7.7`). They are icons rather than words, and every one has an
accessible name.

**No page may scroll sideways at 320px, and the components bend to that budget.** `Segmented`
makes an icon-only item square, which is right everywhere except the header, where square items
push the row past 320px; so the header's bar states the exception — narrower, never shorter,
because the height keeps every control in the row on one centre line.

**One theme menu, and nothing duplicates it.** The example frame opens the *same component* to
theme its own stage, on the same floating surface. `position: fixed` is what escapes the frame's
`overflow: clip`, and the frame's bar carries the stacking rank while the menu is open, because a
panel inside a `sticky` bar paints at the bar's layer whatever it asks for. That panel is opaque
where the header's is glass: what would read through it is the examples it has just re-themed. It
offers exactly the settings `spec/theme.toml` marks `scope = true` — every one but `syntax` —
because a control offered where it does nothing is a control that lies.

**The sidebar's foundation block is translated; the component block never is.** Identifiers stay
English (`NOCTUA.md §2.1`), and a sidebar row is how a reader finds the thing they will type, so
`Heading` is `Heading` in every locale. A page generated from `spec/components/` is titled with the
component's name; a page grouping several is titled with the name at its head (`Tree`, `Search`,
`Avatar`, `Table`, `Lineage`) or, where its members share no name, with an English label
(`Inline text`), never with a translated word that names no component anywhere. Two types carry
the two cases, so the rule is checked rather than remembered. **A page's URL does not move when its
title does**: `/avatars/` and `/tables/` are links a reader may have kept. A rule, never a heading,
separates the introduction, the foundations and the components, because a heading among the
components would be a category.

**The sidebar's first entry is the front door.** There is no landing page: an overview of the
other pages is the sidebar written twice. `<locale>/index.html` is a redirect — a door, not a room
— and it opens onto `site::first`, the top of the sidebar, so the two cannot disagree.

**Pages are static files, and a second one is adopted rather than loaded.** Following a link within
one locale replaces `<main>` in place, so the sidebar a reader is navigating by stays at the
offset they left it. Every destination is an `<a href>` the build wrote, to a file that exists, so
with the script blocked every link navigates. The router copies what differs outside `<main>` —
the title, the description, the language menu's links and the sidebar's current row — from the
fetched document, so there is no second renderer to disagree with the first. **A link that crosses
locales is a full load**, because a document in another language has a different `lang`, a
different `dir` and a different word in every control. The router rests on the release contract
every runtime file carries, and on the language being resolved exactly once, in `<head>`.

**Every page has one shape.** There is one `DocPage`: a head with the title, a one-line lead and
**one complete, working example**, then an index of the page, then **When to use it · Examples ·
Every variant · Reference**, always in that order, so a reader who learns where the props table
is on one page knows where it is on the next. The head example comes before the explanation, and
it is keyed by page rather than by component, because a page holding six parts opens with the six
working together.

**The apparatus may not outweigh what it frames.** The contract folds into four sections, each
saying how much is behind it, and a collection page folds one section per part. A fold takes a
**real heading inside its `<summary>`**: closed, the title is all that is left of a section, and a
section with no heading vanishes from the outline a screen reader navigates by. A count in a
fold's label goes **inside** the translated message, never joined to its end.

**The examples and the variants are two different things, and a page needs both.** The variant
grid is derived from the contract — every value of every enum prop — so none is forgotten and none
disagrees with the component, but it cannot show a component in use: `Prose` means nothing without
a heading and two paragraphs inside it, and a component with no enum prop has no grid at all. The
worked examples are written, in `docs-site/src/examples.rs`, rendering the real component with the
call that produced it as its caption, and invariant 12 fails the build for a component or a prop
none of them reaches.

**`ThemeScope` has a component page, written rather than derived.** It is the one component with
no contract, because its props *are* the scopable settings and a second list could only disagree
with the theme model. It sits in the component block, and it is the only page carrying a rendered
scope, so the browser suite measures the whole theme layer against it: removing the page removes
that coverage.

**Documentation says what a thing is and what it does**, and nothing else — not what it is like,
not what it used to be, not why the taste behind it is right. A component summary opens on a verb
and names what a reader can do with it; a prop doc names the prop, then the values, then the
default or the constraint; an accessibility note states facts about roles and attributes. ARIA
roles, WCAG numbers, attribute names and what works with the script blocked stay, because that is
what the thing does. `crates/noctua-design-spec/src/prose.rs` enforces it under
`cargo xtask check`, over every contract field and both catalogues: it refuses an em dash, a
semicolon, a sentence over twenty-five words, capitals used as emphasis, a list of banned
constructions per language, and anything over the ceiling for its kind.

**Every scene is measured, and also looked at.** A component that draws its own layout gets a
geometric audit: `tests/browser/lineage-audit.spec.js` walks every scene on the `Lineage` page at
four widths in two locales and fails on nine geometric claims — overlap, air, a loose connection
end, a label over a face, clipped content, anything beyond the pan's reach, type below twelve
pixels, a portrait below sixteen, a label further than ninety pixels from its own line — naming the
scene and the two things involved. Every rule is proven by restoring the defect it names and
watching it fail. The audit is necessary, not sufficient: a person still looks at every example,
because what no rule measures — a name ellipsised by a card three pixels too short — is a claim
nobody wrote down.

### The example frame

**One abstraction hosts every example** — the mount point, the implementation swap, the viewport
controls and the reset — and a 3D example is a frame whose content is a scene of real elements, the
answer `NOCTUA.md §4` gave in choosing `CSS3DRenderer` over rasterising content to a texture.
**The frame belongs to the site**, not to the packages, so it has no contract; it has its own page,
`/example-frame/`, in the component block beside `ThemeScope`'s, because a reader who wants to
know what the controls beside an example do has nowhere else to look.

**One frame per example, on every page.** A frame is a viewport, and the question it answers —
*does this survive a narrow column* — is about one example; asked of thirty at once it is answered
for none. Every example and every variant grid has its own frame, with its own width, appearance
and implementation controls, and **the implementation switch appears only on a frame that can
honour it**.

**The controls are a rail on the trailing edge, and their axis changes with the width.**
`.nd-frame` is a two-column grid with a vertical `Toolbar` in the second column, **never folded
behind another control**: pressing a control to reveal a control is worse than the control. Below
the `sm` step the grid collapses and the rail lies down into a row, so a 320px stage keeps its full
width. **The markup therefore cannot name the axis**: `aria-orientation` decides which arrows walk
a toolbar, so the markup ships the ARIA default, true at the narrow width, and the script upgrades
it where the stylesheet stands the rail up.

**There is no code panel.** Each example carries its call as its `<figcaption>`, always visible: a
reader who has to open a disclosure to see the line they came to copy copies the wrong thing.

**Every demonstration on the site is in a frame, and the frame is built from components.** The
typography specimens and the icon gallery are examples in frames like everything else, and the
frame's controls are a `Toolbar` of catalogue components — one tab stop, with arrow-key navigation,
placed by a component's contract rather than by a stylesheet rule.

**The frame's appearance toggle is scoped to the frame.** It stamps a theme scope on the stage at
run time, so `dist/scope.css` re-resolves the contract there and every example under it obeys while
the page does not move. The stage **paints** the contract's surface as well as resolving it, and
only while it is scoped, so a frame at rest still lets the page's pattern read through. With the
script blocked there is no toggle and no scope, which is correct. It is a toggle, not a cycle,
carrying both glyphs and both names, because a control that does not show its own state cannot be
used, and it un-stamps from the published list (see "The scope model, and the rule everything here
descends from").

## The scene doctrine

`Lineage` draws its own layout, which makes it the one component where the design system is also a
drawing program. These rules bind it and **any future component that places its own boxes**;
`NOCTUA.md §9.15` states them in brief. Beneath all of them: **no length in a scene is a fraction of
something the fit can shrink without bound** — tuning fractions against the scenes being looked at
only moves the defect to the next scene, which picks a different cell.

1. **Type is never smaller than readable.** The two pitches are fixed token lengths: `scene.cell`
   across, and the taller `scene.row` down, because the air between two rows holds the brackets,
   their labels and the stems into each card. The fit may compress the across-pitch to **0.85** of
   it and no further; the down-pitch never compresses. Text, portraits, paddings and line weights
   are real lengths that layout never scales — only the `size` prop multiplies them, by a factor
   whose floor keeps the smallest text at twelve pixels. A scene that cannot fit **pans** instead
   of shrinking: the platform's own scrolling with no script, the camera with one. The camera's
   zoom magnifies the finished picture — a `scale3d` at rest, the camera's distance once engaged —
   and the pitches never see it.
2. **The camera mounts on an act, never on a glance.** A press, a key or focus mounts it; a pointer
   passing over the page does not, because hovering an example must change nothing. It *engages* —
   clips the field and takes over placement — only at its first movement, and folds the reader's
   scroll position into the pan then, so the picture under the pointer does not move by a pixel.
   Until then a scripted page and a scriptless one are the same page.
3. **A filter marks and never moves — except `isolate`, which removes.** `highlight` dims the
   non-answers in place, for a reader who must not lose their place, and its layout is the
   unfiltered one. `isolate` lays out the answering subgraph fresh: each removed connected group
   becomes one dashed ring carrying the count of people it stands for, and every bond that crossed
   the cut is re-aimed at the ring, keeping its kind, so the ring stands where those people stood.
4. **A connection's label sits just above its own line**, over nothing — or just below, where its
   free stretch is shorter than a cell, because a short stretch has a union mark and two card
   edges crowding the air above it. Never on the line; the audit's distance rule makes the pairing
   mandatory.
5. **The two layout engines are held to the blessed cases, never to each other.** `layout.js` is a
   hand transcription of `layout.rs`, and comparing the twins passes whenever both are wrong the
   same way. Both run every case in `spec/lineage/cases.toml`, and a disagreement names which half
   moved.
6. **A pass that moves pieces must not feed back.** The failure is a ratchet: centring jumps a
   figure onto its own union mark while separation resolves overlaps rightward only, and the loop
   between them walks a family across the scene a little further each build. So centring counts
   partners as kin only across rows, separation restores the row's mean, and the pass that seats a
   leaf partner beside its union moves only figures that are nobody's kin.
7. **The contract names every behaviour-carrying word.** A value that places, sizes or restyles
   anything — a `kind` that ranks two figures together, a synthesised id suffix, an attribute
   vocabulary two layers key on — is written in the component's own spec, or it does not exist.
   No unwritten word may resize geometry, and a specimen never fakes differences with attributes no
   scene emits: its stand-ins carry `sample-` names and a comment saying so.
8. **A line never passes through a card that is not its destination.** It outranks the crossing
   count: `seat_marks` puts every union mark between its partners, `flank_marks` pulls the nearest
   partner on each side flush against it, whoever stood between slides outside, and crossings are
   recounted afterwards. The audit enforces it geometrically: a run that penetrates a card deeply
   while both its ends are outside it fails the build, and every legitimate stem has one end inside
   its own card, at the centre it is drawn to.

## Publishing

How a release reaches GitHub, crates.io and npm. `noctua-central` depends on this repository by
path, with the git-tag form in a comment above (`NOCTUA.md §9.14`).

**A tag publishes everything.** `git push --follow-tags` is the whole release.
`.github/workflows/release.yml` packs `dist/`, the fonts and the vendored colour CSS into a GitHub
Release with attestations, and every published name carries this repository as a trusted
publisher — crates.io against `release-crate.yml`, npm against `release-npm.yml`, both matched by
filename — so no token is stored anywhere. **Renaming either workflow file silently breaks
publishing**, and the error does not mention the filename. Both are idempotent: a run that dies
halfway is repeated with `workflow_dispatch` for the existing tag, because "already published" is
the one failure each tolerates. `cargo xtask release` prepares the version.

**What is published**: `noctua-design` and the crates it depends on — `noctua-design-tokens`,
`noctua-design-icons`, `noctua-design-grammars`, `noctua-design-theme` and `noctua-design-lineage`
— and `noctua-design-dist`; on npm, `@noctua-world/design-tokens`, `@noctua-world/design-dist` and
`@noctua-world/design-react`. **The compiler is never published**: `noctua-design-emit`, `-spec`,
`-styles`, `-runtime`, `-i18n` and `xtask` stay `publish = false`, because a consumer inherits what
the system *is*, never what builds it, which is also why the published support crates carry no
dependencies at all. `grep -L '^publish = false' crates/*/Cargo.toml` lists what publishes.

**`dist/` ships as its own package**, because neither Cargo nor npm packages a file above the
package directory, and `dist/` sits at the repository root, where a consumer copying it wants it.
`crates/noctua-design-emit/src/dist.rs` copies it into `crates/noctua-design-dist/` and
`packages/dist/` byte for byte, and the drift check proves the three agree; never hand-edit a
copy. A component crate does not depend on the assets crate: writing files is the application's
decision, and a component that wrote one would be making it for them.

**By hand, the order matters**: a crate cannot be published before the crates it depends on are on
the index.

```bash
cargo publish -p noctua-design-tokens     # then icons, grammars, theme, lineage
cargo publish -p noctua-design            # after its dependencies are on the index
cargo publish -p noctua-design-dist       # independent of the rest
pnpm --filter @noctua-world/design-tokens publish   # then design-dist, then design-react
```

**Pack with `pnpm`, upload with `npm`.** `pnpm` rewrites `workspace:*` to the real version and
`npm` does not, so a package packed by `npm` ships a dependency nobody can resolve. A relative path
given to `npm publish` needs a leading dot-slash: without it, `packed/x.tgz` is read as the GitHub
shorthand for a repository.

**What stops a release before it starts is checked on every push**: `pnpm-lock.yaml` must name
every package in the workspace (`cargo xtask check` and CI's `pnpm install --frozen-lockfile`), and
every install script pnpm runs must be approved under `allowBuilds` in `pnpm-workspace.yaml`,
because pnpm treats an unapproved one as an error.

**Registry behaviour worth knowing.** crates.io rate-limits *new* crate names to roughly one per
ten minutes; a `429` there is a queue, not a refusal, and the crate is sometimes created anyway, so
read the registry before retrying. An npm account with `two-factor auth: auth-and-writes` and a
security key refuses `--otp` codes, so a manual publish runs in a real terminal, where npm can open
the browser. **npm has no 0.2.2, 0.2.3 or 0.2.4**, which exist only on crates.io; do not publish
npm releases to fill the gap.

**A package is proven only from outside it.** `examples/consumer-rust` reads `base.html` and asserts
every sheet it links is beside it: inside this repository the site copies the colour sheets from
`packages/tokens/vendor/`, so every test here would pass while a consumer's page had no palette at
all. Anything the packages owe is checked from a project that has only the packages.

## The platform's silent failures

How browsers behave, each of which has cost real debugging time here, and none of which logs
anything.

- **An unregistered custom property reads back as its own source text.**
  `getPropertyValue("--nd-pitch")` answers `var(--noctua-scene-tilt)`, a string a script cannot
  compute with. Declare an `@property` for every custom property a script and the stylesheet
  share.
- **A syntax error anywhere in a `transform` list drops the whole declaration.**
  `translate3d(…) none` is not a list with a no-op in it: `none` is a value for the whole property.
  Assert that a computed `transform` exists at all.
- **`100cqw` inside a container's own declarations answers for the container above it.** An
  element that establishes a query container is not its own.
- **A rule inside `@media` or `@container` adds nothing to its specificity.** A rule that undoes a
  narrow-screen default has to win on order alone, so one extra `:not()` on the default makes the
  wide rule never apply, which looks like a query that does not match. Every restore pair is
  written at the specificity of the rule it restores.
- **Any child of a `display: table` box that is not a table part is wrapped in an anonymous table
  cell.** A `<caption>` given `display: block` becomes a cell one column wide, drawn under the
  heading row because a header group is laid out first — a stray cell rather than an obviously
  broken caption.
- **`[hidden] { display: none }` is a user-agent rule, so any author rule outranks it.**
  `[hidden] { display: none !important }` is the only `!important` in the stylesheet.
- **A `required` control the script has hidden makes the form silently unsubmittable.** The browser
  must focus an invalid control to anchor its bubble and cannot focus `display: none`, so an
  enhanced control puts the browser's own `validationMessage` on the element the reader sees.
- **`form.reset()` fires one event on the form and none on the controls inside it**, so an enhanced
  control keeps showing a value its native element no longer holds. And a script-created
  `<option>` has `defaultSelected === false`, so a reset deselects it and **leaves it in the list
  for ever**.
- **A rule written for a prop's default value is a rule on everything.** A
  `[data-nd-tone="neutral"]` rule added to make neutral "do something" repaints every figure on the
  site, including the ones whose shape deliberately removed it. The default is the resting state.
- **`overscroll-behavior: contain` on an axis the box does not scroll stops the page scrolling over
  it.** Name the axis.
- **With `box-sizing: border-box`, a box cannot be shorter than its own padding.** Blank space that
  has to be scrolled through is **content**, not padding; otherwise most of the travel is
  unreachable, and it still looks plausible in a screenshot.
- **Half of what a no-script page shows is a control some enhanced page hides**, and every sweep
  runs with a script. Ask again with the script blocked, as `material.spec.js` does.
- **`ch` is not a character.** It is the advance of `0`, so a fourteen-character word can be wider
  than `14ch`.
- **A control whose height is not declared is its font size, line height and padding added up**,
  so two controls side by side agree only by accident, and a screenshot of either one cannot show
  it. Declare the height on the row, and measure the row rather than the control.

## Two rules about proving something

**Geometry is asserted, not eyeballed — and not eyeballed once.** A control that overlaps its
neighbour, a name wider than its box, or a row that hit-tests to something else is a failing test,
not a matter of taste. Check across locales, because `qps` exists to make a fixed column overflow
and only finds it if something asks; and check the combination — a panel open inside a nested
frame, at 768px, in `pt-BR` — that a single wide English screenshot cannot show.

**A declaration that loses on specificity is indistinguishable from one that works** if what you
measure would have passed regardless. Assert the geometry — where a control's centre line is,
which pixel a corner paints — never the property.

**How a test here goes vacuous is `NOCTUA.md §19.5`.** Read it before adding a guard, and confirm
every new one by restoring the defect it was written for, in the artefact that actually ships, and
watching it fail. Under heavy machine load the browser suite can fail a spec that passes alone:
run that spec by itself before calling it a regression.

## No other design system is named as a source

`gates.rs::cited_sources` refuses it, and it is the one gate that reads the raw line rather than
the line with comments stripped, because comments are where such citations get written. Nothing
here was taken from another design system, so naming one claims a relationship that does not
exist. **Keep the fact, drop the name**: a WCAG number, a hit-target size or an accessibility
technique is a fact and stays. Vendored work is different: the attributions under `assets/` and
`packages/tokens/` are obligations.

## Maintaining this file

Update it on your own initiative whenever something here becomes wrong or incomplete, in the same
change as the code, never as a follow-up (`NOCTUA.md §0.5`). Nobody will ask.

**What goes where.** `NOCTUA.md §9` holds what the rest of the project relies on. This file holds
the design system's own rules and the rules that bind every file here. A trap that binds one module
lives in that module's header, named in "Where the area-specific traps live", because a header is
read by whoever opens the file.

**Record the command, not the value.** A count written here rots silently, and an agent that
trusts a stale number errs with confidence; a hand-written list of files, components or controls
rots the same way. If a gate or a generated file can answer, name that instead.

**A finished piece of work leaves a rule, never an account of itself:**

| What you have | Where it goes |
|---|---|
| a rule that still binds | this file, or the header of the module that owns the area |
| evidence for an assertion | the comment beside that assertion |
| the story of the change | its commit message, and nowhere else |

**A commit message names no agent**: no `Co-Authored-By` naming one, no session link, no mention in
the body, whatever an agent's own defaults say. The author is the person who asked for the work
(`NOCTUA.md §2.6`).
