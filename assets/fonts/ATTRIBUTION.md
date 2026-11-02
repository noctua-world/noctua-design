# Fonts

**These files are vendored by hand and are not build output.** Nothing
regenerates them, `cargo xtask build` does not touch them, and they must never
be deleted as stale artifacts. They are the one category of binary asset in
this repository that is copied rather than generated.

What *is* generated from them is `dist/fonts.css` — every `@font-face` and every
font-stack variable — compiled from [`spec/fonts.toml`](../../spec/fonts.toml).
See "Why the declarations are generated" below.

---

## What is here, and why exactly this

Three families, chosen for three different jobs (`NOCTUA.md §7.3`).

### Noto — the interface

| File | Family | Role |
|---|---|---|
| `NotoSans.woff2`, `NotoSans-Italic.woff2` | Noto Sans | body and interface text — the default for `--noctua-font-ui` |
| `NotoSerif.woff2`, `NotoSerif-Italic.woff2` | Noto Serif | long-form prose |
| `NotoSansDisplay.woff2`, `NotoSansDisplay-Italic.woff2` | Noto Sans Display | large headings, where Display's tighter fit reads better |

Licensed under the SIL Open Font License 1.1 — [`OFL-Noto_Sans.txt`](OFL-Noto_Sans.txt),
[`OFL-Noto_Serif.txt`](OFL-Noto_Serif.txt), [`OFL-Noto_Sans_Display.txt`](OFL-Noto_Sans_Display.txt).

### Noctua Iosevka — code and the optional interface font

| File | Weight |
|---|---|
| `NoctuaIosevka-Regular.woff2` | 400 |
| `NoctuaIosevka-Bold.woff2` | 700 |
| `NoctuaIosevka-Italic.woff2` | 400, italic |

A custom build of [Iosevka](https://typeof.net/Iosevka/) by Renzhi Li (Belleve
Invis), SIL OFL 1.1 — [`OFL-Iosevka.txt`](OFL-Iosevka.txt). The build plan lives
outside this repository, in `noctua-fonts`.

**Static faces, not variable** — no `fvar`. That is why each weight is its own
file, and why `spec/fonts.toml` gives each one an explicit `weight` rather than
a range. **Italic, not Oblique**: the true cursive italic with its own
letterforms, chosen deliberately over the slanted roman.

Bold is not optional: a synthesised bold on a monospaced face looks broken.

### Symbols Nerd Font — icons

| File | Family | Advance |
|---|---|---|
| `SymbolsNerdFont-Regular.woff2` | Symbols Nerd Font | proportional |
| `SymbolsNerdFontMono-Regular.woff2` | Symbols Nerd Font Mono | mono — every glyph exactly one em |

Nerd Fonts 3.4.0 Symbols-Only, **MIT** (not OFL) —
[`LICENSE-NerdFontsSymbolsOnly.txt`](LICENSE-NerdFontsSymbolsOnly.txt). That
is the only licence text Nerd Fonts ships with it, but the icon sets it
gathers keep their own upstream licences, and not all of them are MIT:
Codicons and Font Awesome are under CC BY 4.0, Material Design Icons under
Apache 2.0, Pomicons and Weather Icons under the SIL Open Font License 1.1,
and Font Logos under the Unlicense. The Nerd Fonts repository,
<https://github.com/ryanoasis/nerd-fonts>, lists every set with its source and
licence, in its license audit and in the README of its glyph sources.

Its language coverage is **empty**: icons, no letters. That is what makes it
safe as the last entry in every font stack — it can only ever supply a glyph
nothing before it had, and can never win a codepoint a text font should have
rendered.

**These two are not subset.** Subsetting them to the Latin repertoire produces a
400-byte file with no icons in it, which is a measurement rather than a guess.
They are fetched only when an icon codepoint actually renders, via
`unicode-range`, so their size never reaches a reader who has no icons on screen.

### What is deliberately absent

Every face a page does not render. The other-script faces and the CJK families
are here because the documentation site renders fourteen scripts
(`NOCTUA.md §7.3`): each is fetched through `unicode-range` only when a codepoint
inside its range is drawn, and none is preloaded — "Slicing the CJK families",
below, says how the three CJK families are cut. A family nothing renders is not
copied.

**No licence is present for a family that was not copied**, and that is checked
rather than trusted.

---

## Why they are subset

The upstream faces carry Greek, Cyrillic, Vietnamese, IPA and several thousand
glyphs this project does not render. Shipping them unsubset would dominate every
other performance decision made anywhere in the design system.

| File | Before | After | |
|---|---|---|---|
| `NotoSans.woff2` | 889,348 | 52,740 | 17× |
| `NotoSans-Italic.woff2` | 1,018,016 | 58,656 | 17× |
| `NotoSerif.woff2` | 796,044 | 52,828 | 15× |
| `NotoSerif-Italic.woff2` | 1,041,492 | 68,960 | 15× |
| `NotoSansDisplay.woff2` | 682,952 | 55,088 | 12× |
| `NotoSansDisplay-Italic.woff2` | 750,132 | 59,460 | 12× |
| `NoctuaIosevka-Regular.woff2` | 1,617,752 | 17,460 | 93× |
| `NoctuaIosevka-Bold.woff2` | 1,630,316 | 17,496 | 93× |
| `NoctuaIosevka-Italic.woff2` | 1,750,096 | 20,280 | 86× |
| **total** | **10,176,148** | **402,968** | **25×** |

### The `wdth` axis is pinned, and that is most of the saving

The six Noto files upstream are variable on **two** axes — `wght 100–900` and
`wdth 62.5–100`. Nothing in this design system uses width variation, and an axis
nobody uses is payload every visitor downloads and never sees.

Pinning `wdth=100` before subsetting took `NotoSans` from 87.4 KB to 51.5 KB — a
41% saving on the largest files in the payload, and the difference between the
default payload fitting its budget and not.

The consequence travels into `spec/fonts.toml`: **no face in this repository
carries a `wdth` axis**, so none declares `font-stretch`. That is trap 2 of
`NOCTUA.md §7.6` and it is derived rather than remembered — see below.

### The default payload

The faces preloaded at first paint have a budget of **200 KB**. The five marked `preload = true` in `spec/fonts.toml` come to
**162.8 KB**, and a test asserts the budget so it cannot drift.

| Preloaded | |
|---|---|
| `NotoSans.woff2` | 51.5 KB |
| `NotoSans-Italic.woff2` | 57.3 KB |
| `NoctuaIosevka-Regular.woff2` | 17.1 KB |
| `NoctuaIosevka-Bold.woff2` | 17.1 KB |
| `NoctuaIosevka-Italic.woff2` | 19.8 KB |
| **total** | **162.8 KB** |

Serif and Display are **not** preloaded, deliberately. Including them would put
the payload at 393.5 KB. Both fall back to a family that *is* preloaded and
whose metrics are close — Display's own stack names Noto Sans as its fallback —
so their `font-display: swap` costs a small, local reflow rather than a
page-wide one. That trade is the reason they carry `swap` and not `optional`:
`optional` on an un-preloaded face means the face never renders at all, and
`spec/fonts.toml`'s validation rejects that pairing.

---

## How to regenerate them

The inputs are the built faces in `noctua-fonts`. They are not in its git
history: they are published as release archives, and a clone fetches them with
its own archive tool, which writes each font only when its SHA-256 matches the
committed checksum list (its README says how). Run from the **workspace root**
with it in `repos/` and its fonts fetched, then offline, with `fonttools` and
`brotli` installed however you install Python tools.

```bash
set -e
F=repos/noctua-fonts
OUT=repos/noctua-design/assets/fonts
R="U+0020-007E,U+00A0-00FF,U+0100-017F,U+2010-2027,U+2030-205E,U+2070-209F,U+20A0-20BF,U+2122,U+2190-2199,U+21D0-21D2,U+2206,U+2212,U+2248,U+2260,U+2264-2265,U+25A0,U+25CF,U+2713,U+2717,U+FEFF,U+FFFD"
FLAGS="--flavor=woff2 --unicodes=$R --layout-features=kern,ccmp,mark,mkmk --desubroutinize --no-hinting --drop-tables+=DSIG"

# The six Noto faces: pin the unused wdth axis, then subset.
for f in NotoSans NotoSans-Italic NotoSerif NotoSerif-Italic \
         NotoSansDisplay NotoSansDisplay-Italic; do
  fonttools varLib.instancer "$F/noctua-noto/${f}-VariableFont_wdth,wght.woff2" \
    wdth=100 -o "/tmp/${f}.ttf"
  pyftsubset "/tmp/${f}.ttf" --output-file="$OUT/${f}.woff2" $FLAGS
  rm -f "/tmp/${f}.ttf"
done

# The three Iosevka faces. Static, so there is nothing to pin.
for f in Regular Bold Italic; do
  pyftsubset "$F/noctua-iosevka/WOFF2/NoctuaIosevka-${f}.woff2" \
    --output-file="$OUT/NoctuaIosevka-${f}.woff2" $FLAGS
done

# The two symbols faces are copied whole — see above.
cp "$F/symbols-nerd-font/SymbolsNerdFont-Regular.woff2" "$OUT/"
cp "$F/symbols-nerd-font/SymbolsNerdFontMono-Regular.woff2" "$OUT/"
```

### The repertoire, and why each part of it

| Range | Why |
|---|---|
| `U+0020-007E` | ASCII |
| `U+00A0-00FF` | Latin-1 Supplement — accented names such as Viénot and Björn |
| `U+0100-017F` | Latin Extended-A — Portuguese and the rest of European orthography |
| `U+2010-2027` | dashes, quotes, ellipsis, dagger |
| `U+2030-205E` | per mille, primes, guillemets, the fraction slash |
| `U+2070-209F` | super- and subscripts |
| `U+20A0-20BF` | currency symbols |
| `U+2122` | ™ |
| `U+2190-2199`, `U+21D0-21D2` | arrows, for diagrams |
| `U+2206`, `U+2212`, `U+2248`, `U+2260`, `U+2264-2265` | the maths relations that appear in measured tables |
| `U+25A0`, `U+25CF`, `U+2713`, `U+2717` | bullets and check marks |
| `U+FEFF`, `U+FFFD` | the byte-order mark and the replacement character |

**`spec/fonts.toml`'s `range.latin` must match this list.** A declared range
wider than the subset promises glyphs the file does not have, and the browser
renders tofu rather than falling through to the next family — which is silent,
and looks like a missing font rather than a wrong declaration.

---

## Why the declarations are generated

`dist/fonts.css` is compiled from `spec/fonts.toml` rather than written by hand,
because two of the three things that can go wrong in an `@font-face` are
**invisible in a browser**:

**`font-weight: 100 900` on every variable face.** Without it the browser
renders the file's *default instance*. `fc-query` reports `Thin,Regular` as the
first face for Noto JP, KR, SC and Hebrew — so the text comes out hairline, and
the browser then fabricates a fake bold on top of Thin. Nothing errors. It looks
like a design choice.

**`font-stretch` only on a face that carries `wdth`.** Declaring a width range a
face does not have makes the browser synthesize one, which is worse than not
asking for it.

Both are derived from the `axes` list in `spec/fonts.toml`, and both have a test
asserting the derivation — because a test is the only thing that will ever
notice. `NOCTUA.md §7.6`.

---

## Licence obligations

Every file above ships with its licence, in this directory, and the licence text
must travel wherever the fonts do. `NOCTUA.md §7.8`.

- **SIL OFL 1.1** — Noto Sans, Noto Serif, Noto Sans Display, Noctua Iosevka.
  The OFL requires the licence to accompany the fonts and forbids selling the
  fonts on their own.
- **MIT** — Symbols Nerd Font. Requires the copyright notice and permission
  notice to travel with it. Its icon sets keep their own upstream terms (see
  above); CC BY 4.0, for Codicons and Font Awesome, asks for attribution.

## The script faces

Eleven families covering the scripts the platform must render — Arabic, Hebrew,
Devanagari, Thai, Ethiopic, Georgian, Armenian, Mongolian, Japanese, Korean and
Simplified Chinese. All are Noto, all are SIL Open Font License 1.1, and the
licence text is byte-identical across them, so one copy covers all eleven
(`OFL-Noto_Sans_scripts.txt`).

**None is preloaded and every one carries a `unicode-range`.** Together they are
about 14 MB; a Latin page fetches none of them, and a Japanese page fetches one.
A test asserts both.

Sources: the variable fonts from `notofonts/notofonts.github.io` and
`google/fonts`, and the subset variable OTFs from `notofonts/noto-cjk`.

```sh
# The width axis is pinned before subsetting: a face that keeps `wdth` gets
# `font-stretch`, and a browser asked to synthesise a width the file does not
# have produces something worse than not asking (NOCTUA.md §7.6, trap 2).
FLAGS="--flavor=woff2 --layout-features=kern,ccmp,mark,mkmk,liga,rlig,init,medi,fina,isol,locl,akhn,rphf,blwf,half,pres,abvs,blws,psts --desubroutinize --no-hinting --drop-tables+=DSIG"

fonttools varLib.instancer "NotoSansArabic.ttf" wdth=100 -o "/tmp/pinned.ttf"
pyftsubset "/tmp/pinned.ttf" --output-file="NotoSansArabic.woff2" \
  --unicodes="U+0600-06FF,U+0750-077F,U+08A0-08FF,U+FB50-FDFF,U+FE70-FEFF,U+200C-200F" $FLAGS

# CJK needs no instancing — the subset OTFs vary on weight only — and a
# narrower feature set, since none of the Indic shaping features applies.
CJK_FLAGS="--flavor=woff2 --layout-features=kern,ccmp,mark,locl,vert --desubroutinize --no-hinting --drop-tables+=DSIG"
pyftsubset "NotoSansSC.otf" --output-file="NotoSansSC.woff2" \
  --unicodes="U+3000-303F,U+FF00-FFEF,U+4E00-9FFF,U+3400-4DBF" $CJK_FLAGS
```

Each `[range]` in `spec/fonts.toml` must match the `--unicodes` its face was
subset with. A range wider than the file promises glyphs it does not have, the
browser stops falling through to the next family, and the reader gets tofu.

### Slicing the CJK families

Whole, the three CJK families were 13.6 MB, and every reader of a page with one
Japanese character waited for all of it. Sliced, each file declares a narrow
`unicode-range` and the browser fetches only the slices holding codepoints
actually on the page.

```sh
# 400 codepoints per slice, cut on CODEPOINT ORDER rather than frequency.
# Frequency-ordered slices fetch marginally less for one page, but they make
# each range a scattered list — and a range that cannot be written as a handful
# of intervals is a range nobody can check against the file it describes.
python3 slice.py NotoSansSC.woff2 "Noto Sans SC" NotoSansSC slices/ 400
```

The script writes `<family>.json` beside the slices, recording each file's
`unicode-range` **computed from the file itself**. `spec/fonts.toml` declares
the family once with `[[sliced]]` and points at that manifest; the emitter
expands it into one `@font-face` per slice. Writing 134 ranges by hand is how
one ends up describing a file it does not match, and a codepoint inside a range
its file lacks renders as tofu with the right font sitting beside it.
