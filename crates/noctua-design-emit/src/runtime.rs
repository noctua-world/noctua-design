//! The browser behaviour that **ships**, copied into `dist/` and the packages.
//!
//! | Output | For |
//! |---|---|
//! | `dist/anchor.js` | any consumer with a floating panel — and this site's own shell |
//! | `dist/color.js` | the picker, and anything that needs a notation for a colour |
//! | `dist/color-picker.js` | `enhancePickers()` — everything a `ColorPicker` does |
//! | `dist/inputs.js`, `dist/messages.js` | `enhanceInputs()`, and the sentence helpers both use |
//! | `dist/select.js`, `dist/tag-picker.js`, `dist/tree.js`, `dist/tabs.js`, `dist/date-input.js` | the five that used to be enhanced from the documentation site alone |
//! | `dist/panels.js`, `dist/roving.js` | what those five share: one open list, and one arrow walk |
//! | `dist/palettes/*.json` | the Noctua tab, one file per palette, fetched on demand |
//! | `packages/tokens/color.js` + `.d.ts` | the JavaScript package, and the Node test that checks the arithmetic |
//!
//! # Nothing here is computed
//!
//! The files are `crates/noctua-design-runtime/src/*.js`, hand-written, and
//! this module copies them with a header saying where they really live —
//! exactly as `lineage` does for the layout and `grammar` does for the scanner.
//! `noctua_design_runtime::sources` hands them over as `include_str!`
//! constants, so a file renamed out from under this is a compile error naming
//! the file rather than an empty artifact nobody notices.
//!
//! # Why the colour arithmetic goes to the package as well as to `dist/`
//!
//! Because a test has to be able to import it in Node with no browser at all.
//! `tests/browser/color.spec.js` checks it against what `noctua-colors` already
//! published for every palette step — a second engine, in another language, in
//! another repository — and `packages/tokens/grammars.js` is checked the same
//! way by `scanner.spec.js` for the same reason.

use std::fmt::Write as _;

use noctua_design_runtime::{
    ANCHOR_SOURCE, CAROUSEL_SOURCE, COLOR_SOURCE, DATE_INPUT_SOURCE, DATE_PICKER_SOURCE,
    DIALOG_SOURCE, INFO_SOURCE, INPUTS_SOURCE, MENU_SOURCE, MESSAGE_BAR_SOURCE, MESSAGES_SOURCE,
    PANELS_SOURCE, PICKER_SOURCE, POPOVER_SOURCE, ROVING_SOURCE, SELECT_SOURCE, SUMMARY_SOURCE,
    TABLE_SOURCE, TABS_SOURCE, TAG_PICKER_SOURCE, TOAST_SOURCE, TOOLTIP_SOURCE, TREE_SOURCE,
    source,
};
use noctua_design_spec::Vendor;
use noctua_design_spec::vendor::VENDOR;

use crate::{CommentStyle, EmittedFile, header_from};

/// Emits every shipped-runtime artifact.
#[must_use]
pub fn emit(vendor: &Vendor) -> Vec<EmittedFile> {
    let mut files = vec![
        EmittedFile::new("dist/anchor.js", copied(ANCHOR_SOURCE)),
        EmittedFile::new("dist/color.js", copied(COLOR_SOURCE)),
        EmittedFile::new("dist/messages.js", copied(MESSAGES_SOURCE)),
        EmittedFile::new("dist/color-picker.js", copied(PICKER_SOURCE)),
        EmittedFile::new("dist/inputs.js", copied(INPUTS_SOURCE)),
        EmittedFile::new("dist/carousel.js", copied(CAROUSEL_SOURCE)),
        EmittedFile::new("dist/error-summary.js", copied(SUMMARY_SOURCE)),
        EmittedFile::new("dist/table.js", copied(TABLE_SOURCE)),
        EmittedFile::new("dist/info.js", copied(INFO_SOURCE)),
        EmittedFile::new("dist/menu.js", copied(MENU_SOURCE)),
        EmittedFile::new("dist/message-bar.js", copied(MESSAGE_BAR_SOURCE)),
        EmittedFile::new("dist/date-picker.js", copied(DATE_PICKER_SOURCE)),
        EmittedFile::new("dist/dialog.js", copied(DIALOG_SOURCE)),
        EmittedFile::new("dist/popover.js", copied(POPOVER_SOURCE)),
        EmittedFile::new("dist/tooltip.js", copied(TOOLTIP_SOURCE)),
        EmittedFile::new("dist/toast.js", copied(TOAST_SOURCE)),
        EmittedFile::new("dist/panels.js", copied(PANELS_SOURCE)),
        EmittedFile::new("dist/roving.js", copied(ROVING_SOURCE)),
        EmittedFile::new("dist/select.js", copied(SELECT_SOURCE)),
        EmittedFile::new("dist/tag-picker.js", copied(TAG_PICKER_SOURCE)),
        EmittedFile::new("dist/tree.js", copied(TREE_SOURCE)),
        EmittedFile::new("dist/tabs.js", copied(TABS_SOURCE)),
        EmittedFile::new("dist/date-input.js", copied(DATE_INPUT_SOURCE)),
        EmittedFile::new("packages/tokens/color.js", copied(COLOR_SOURCE)),
        EmittedFile::new("packages/tokens/color.d.ts", color_types()),
    ];
    files.extend(vendor.themes.iter().filter_map(|theme| {
        vendor.ramps.get(theme).map(|ramps| {
            EmittedFile::new(format!("dist/palettes/{theme}.json"), palette(theme, ramps))
        })
    }));
    files
}

/// One palette's steps, as `ColorPicker { palette }` fetches them.
///
/// # One file per palette, not one file for all of them
///
/// Thirty-nine palettes in both appearances is around four hundred kilobytes,
/// and a reader who opens the Noctua tab wants **one** of them — the one in
/// force. `noctua-colors` made this same call for its own stylesheets and its
/// `AGENTS.md` records the result: a self-contained file per palette is *a
/// twenty-fifth of the weight of all of them*. This is that decision applied to
/// the same data in a different shape.
///
/// Fetched on the first time the tab is opened and cached for the visit, which
/// is the trade `Search`'s index already makes on this site.
///
/// # `[token, hex]` pairs, in the vendor's order, ungrouped
///
/// The order **is** the ramp — light to dark — and grouping is left to the
/// picker, which splits at the first segment of each name. Nothing here infers
/// a family list: a list of families written down in this repository would be a
/// second copy of a fact that lives upstream, and it would go stale the first
/// time `noctua-colors` added one.
fn palette(theme: &str, ramps: &noctua_design_spec::vendor::Ramps) -> String {
    // JSON has no comments, so provenance is a key — the shape
    // `dist/lineage.cases.json` already uses.
    let mut out = String::from("{\n");
    let _ = writeln!(
        out,
        "  \"_generated\": \"Generated by noctua-design — do not edit. Source: {VENDOR}. \
         Regenerate: {}.\",",
        crate::REGENERATE
    );
    let _ = writeln!(out, "  \"theme\": \"{theme}\",");
    steps(&mut out, "light", &ramps.light);
    out.push_str(",\n");
    steps(&mut out, "dark", &ramps.dark);
    out.push_str("\n}\n");
    out
}

/// One appearance's steps, one pair per line so a diff names the step.
fn steps(out: &mut String, mode: &str, ramps: &[(String, String)]) {
    let _ = write!(out, "  \"{mode}\": [");
    for (index, (token, hex)) in ramps.iter().enumerate() {
        if index > 0 {
            out.push(',');
        }
        let _ = write!(out, "\n    [\"{token}\", \"{hex}\"]");
    }
    out.push_str("\n  ]");
}

/// One file, with a header saying where it really lives.
fn copied(path: &str) -> String {
    let mut out = header_from(CommentStyle::Block, path);
    out.push_str("\n// COPIED, NOT GENERATED. Edit the file named above.\n\n");
    out.push_str(source(path));
    out
}

/// The colour module's types.
///
/// Written here rather than derived, for `lineage`'s reason: these describe a
/// hand-written module, and a specification to generate them from would be a
/// specification with exactly one consumer.
fn color_types() -> String {
    let mut out = header_from(CommentStyle::Block, COLOR_SOURCE);
    out.push_str(
        r"
/** A colour, as every function here passes it around. */
export interface Rgb {
  /** Red, 0…255. */
  r: number;
  /** Green, 0…255. */
  g: number;
  /** Blue, 0…255. */
  b: number;
  /** Opacity, 0…1. Absent means opaque. */
  a?: number;
}

/** Hue in degrees, saturation and value as percentages. */
export interface Hsv {
  h: number;
  s: number;
  v: number;
}

/** Hue in degrees, saturation and lightness as percentages. */
export interface Hsl {
  h: number;
  s: number;
  l: number;
}

/** Lightness 0…1, chroma, hue in degrees. */
export interface Oklch {
  l: number;
  c: number;
  h: number;
}

/** One notation the codes panel can show. */
export interface Format {
  /** What the row is called, in every locale: `HEX`, `RGB`, `HSL`, `OKLCH`. */
  label: string;
  /** The string a reader copies. */
  of(color: Rgb, alpha: boolean): string;
}

export function rgbToHsv(color: Rgb): Hsv;
export function hsvToRgb(hsv: Hsv): Rgb;
export function rgbToHsl(color: Rgb): Hsl;
export function hslToRgb(hsl: Hsl): Rgb;
export function rgbToOklch(color: Rgb): Oklch;
export function oklchToRgb(oklch: Oklch): Rgb;
export function toHex(color: Rgb, options?: { alpha?: boolean }): string;

/** `null` for anything that is not one of the four accepted notations. */
export function parse(text: string): Rgb | null;

export const FORMATS: Record<string, Format>;

/** `null` for a notation name `FORMATS` does not carry. */
export function notation(
  color: Rgb,
  kind: string,
  options?: { alpha?: boolean },
): string | null;
",
    );
    out
}
