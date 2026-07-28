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
are invisible, so an agent cannot read the shared colour contract, edit a
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
NOCTUA REPOSITORIES, SO I WILL NOT BE ABLE TO READ THE SHARED COLOUR
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
COLOUR TOKENS, THE DESIGN SYSTEM, THE MASTER TECHNICAL REFERENCE — IS
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

**It never defines a colour.** Not a hex, not an `oklch()`, not "just this one
shade". Colours come from `noctua-colors`, vendored into
`packages/tokens/vendor/`, and a missing colour is a one-line change *there*,
discussed first. `NOCTUA.md §8.8`. A source gate enforces this rather than
trusting it.

## Status

**Stages 4 and 5 — the theme runtime, the language layer and the site.**

What exists: the token compiler, the font layer, the source gates, the theme
model, the message catalogue, `ThemeScope` in both packages, and a documentation
site pre-rendered in three locales. `cargo xtask check` is still the single gate,
and `cargo xtask docs` builds the site.

**There is exactly one component, and it is not a widget.** `ThemeScope` is how
a theme reaches a subtree at all, and it is **generated** from `spec/theme.toml`
rather than written — a product of the machine rather than an exception to it.
Stage 6 adds the first real components; a Button built now would be rebuilt.

The site's own scope and limits are in `docs-site/PROVISIONAL.md`. Read that
before adding a page.

## Repository map

```
spec/                   the only files a developer edits
  tokens.toml           every non-colour token (Leg 1 of the parity mechanism)
  i18n/{en,pt-BR}.toml  every UI message, both locales
  components/           <name>.toml (contract) + <name>.cases.toml (behaviour)
crates/
  noctua-design/        the Dioxus package. src/theme.rs is GENERATED
  noctua-design-tokens/ GENERATED from spec/tokens.toml. COMMITTED
  noctua-design-theme/  GENERATED from spec/theme.toml. COMMITTED
  noctua-design-i18n/   GENERATED from spec/i18n/. COMMITTED
  noctua-design-icons/  GENERATED. COMMITTED
packages/
  react/                the React package. src/theme.js is GENERATED
  tokens/               GENERATED (TypeScript). COMMITTED
  tokens/vendor/        the colour tokens, vendored from noctua-colors
assets/fonts/           copied from .TEMP/, + ATTRIBUTION.md + licences
dist/                   GENERATED, COMMITTED: noctua-design.css and manifests
docs-site/              the docs app. src/ and assets/ are written; dist/ is
                        build output and is gitignored. Read PROVISIONAL.md
tests/conformance/      one case set, run against both implementations
xtask/                  the five verbs
.TEMP/                  staging area. Copy OUT of it; never modify it
```

**Generated directories are committed** so every consumer works with no build
step, exactly as `noctua-colors` commits `system/`. `cargo xtask check` fails if
one drifts from `spec/`.

## Commands

```bash
cargo xtask build     # compile spec/ into tokens, icons, props types and dist/
cargo xtask check     # THE gate: spec, source gates, drift, fmt, clippy, tests
cargo xtask dev       # watch spec/ and components, rebuild, serve the docs site
cargo xtask docs      # pre-render the site into docs-site/dist/, one build per locale
cargo xtask release   # prepare a version
```

**Five verbs, and the surface does not grow.** New capability goes behind an
existing verb or it does not ship. `cargo xtask check` is exactly what CI runs —
the same command, not a reimplementation in YAML, which is precisely how the two
drift until "passes locally, fails in CI" becomes a normal sentence.

`just <verb>` is a one-word alias for each and is never the only path.

## Invariants

Breaking any of these is a defect, not a tradeoff. Each is enforced by a gate,
because a convention nothing enforces is a convention that decays.

1. **No colour is defined here.** Every colour comes from a `--nc-*` token in the
   vendored `noctua-colors` output. A hex, an `oklch()` or a named colour in this
   repository's source fails `check`.
2. **No token literal.** No raw `px`, `rem` or `ms` in component source. Spacing,
   radii, type sizes, durations and easings come from the token layer, so a
   change to the scale reaches every component.
3. **No font family named directly.** Use `var(--noctua-font-*)`. A component
   that hardcodes `"Noto Sans"` cannot honour the interface-font setting and will
   not fall back correctly. `NOCTUA.md §7.4`.
4. **No Tailwind arbitrary value.** `p-[13px]` is a token literal wearing a
   different hat.
5. **Generated files are never hand-edited.** Change `spec/` and rebuild.
6. **The Rust side builds and tests with no Node installed.** `check` proves it
   rather than assuming it. Tailwind runs from its standalone binary so a
   Rust-only contributor never installs a JavaScript toolchain.
7. **Both implementations change together.** A component is four things — the
   spec, the Dioxus implementation, the React implementation, and the shared
   cases — and editing one implementation only is a compile error or a failing
   conformance test, by design. `NOCTUA.md §9.3`.

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

**A `var()` resolves where it is DECLARED, not where it is read.** The colour
system emits its whole semantic contract — `--nc-color-surface`, `-fg`,
`-border`, 1,767 names — once, at `:where(:root)`. Give a subtree
`data-palette="amber-vivid"` and it gets amber's `--nc-neutral-*`, but
`--nc-color-surface` still holds the **root's** value, because that is the
element the indirection was declared on. A themed component painted from the
semantic contract silently shows the page's colours, and every token still
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

### Trap 0.5 — media queries are minimum-width, and the gate now says so

A media query cannot read a custom property, so the token gate could never apply
to a prelude and preludes were simply unchecked. That hole let three `max-width`
queries into the shell's own stylesheet — written directly under a header saying
breakpoints are minimum-width only.

`gates::media_prelude` now rejects `max-width` outright and requires every
`min-width` to equal a breakpoint declared in `spec/tokens.toml`. Write the
narrow case unconditionally and override it upward.

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

## Maintaining this file

Update it on your own initiative whenever something here becomes wrong or
incomplete — in the same change as the code, never as a follow-up. Nobody should
have to ask.

Keep it dense: it loads every session, so every line costs context that could
have gone to the task. **Record the command, not the value** — a count written
here rots silently, and an agent that trusts a stale number errs with confidence.

Detail that belongs to the architecture belongs in `NOCTUA.md`; this file is the
operating manual for working *in this repository*.
