//! `@font-face` declarations and the font-stack variables.
//!
//! Generated rather than hand-written because two of the three things that can
//! go wrong here are **silent in a browser**, and both are derivable from the
//! face's declared axes:
//!
//! **`font-weight: 100 900` on every variable face.** Without it the browser
//! renders the file's *default instance*. For Noto JP, KR, SC and Hebrew that
//! default is Thin (100), so the text comes out hairline and a fake bold is
//! fabricated on top of it. Nothing errors; it looks like a design choice.
//!
//! **`font-stretch` only where a `wdth` axis exists.** Declaring a width range
//! a face does not have makes the browser synthesize one, which is worse than
//! not asking.
//!
//! `NOCTUA.md §7.6`. A test asserts both derivations, because a test is the
//! only thing that will ever notice.

use std::fmt::Write as _;

use noctua_design_spec::{Fonts, Spec};

use crate::{CommentStyle, EmittedFile, header};

/// Where the faces are served from, relative to the stylesheet.
const ASSETS: &str = "../assets/fonts";

/// Emits `dist/fonts.css`.
#[must_use]
/// # Panics
///
/// If a manifest is passed for a family the spec does not declare as sliced.
/// The build loads manifests *from* those declarations, so the two cannot
/// disagree unless a caller assembles them by hand.
pub fn emit(
    spec: &Spec,
    fonts: &Fonts,
    sliced: &[noctua_design_spec::Manifest],
) -> Vec<EmittedFile> {
    let prefix = &spec.meta.prefix;
    let mut out = header(CommentStyle::Block);

    writeln!(
        out,
        "\n/* The font layer: one @font-face per file, and one variable per role.\n\
        \x20\n\
        \x20  Never name a family directly in a component. Use var(--{prefix}-font-*),\n\
        \x20  or the interface-font setting cannot reach it and the fallback chain is\n\
        \x20  lost. A source gate enforces this.\n\
        \x20\n\
        \x20  THE ROLES ARE FOR THE PAGE TO READ, NOT FOR IT TO REDEFINE.\n\
        \x20  No component sets a family: a design system does not restyle the page it\n\
        \x20  is installed into, so components inherit whatever the host set. That\n\
        \x20  makes `--{prefix}-font-ui` and `--{prefix}-font-display` the handles a\n\
        \x20  consumer *reads* — `body {{ font-family: var(--{prefix}-font-ui); }}` is\n\
        \x20  the one line that adopts this typography. Redefining one changes nothing\n\
        \x20  on its own, because nothing here reads it back.\n\
        \x20\n\
        \x20  `--{prefix}-font-mono` is the exception and is read by the components\n\
        \x20  that draw code, because a code block that inherited a proportional face\n\
        \x20  would stop being one. */\n"
    )
    .expect("write");

    for face in &fonts.face {
        writeln!(out, "@font-face {{").expect("write");
        writeln!(out, "  font-family: \"{}\";", face.family).expect("write");
        writeln!(
            out,
            "  src: url(\"{ASSETS}/{}\") format(\"woff2\");",
            face.file
        )
        .expect("write");
        writeln!(out, "  font-style: {};", face.style).expect("write");

        // Trap 1. A variable face gets the full range; a static one gets its
        // single declared weight.
        if face.is_variable_weight() {
            writeln!(out, "  font-weight: 100 900;").expect("write");
        } else {
            let weight = face.weight.unwrap_or(400);
            writeln!(out, "  font-weight: {weight};").expect("write");
        }

        // Trap 2. Only a face that actually carries `wdth`.
        if face.is_variable_width() {
            writeln!(out, "  font-stretch: 62.5% 100%;").expect("write");
        }

        writeln!(out, "  font-display: {};", face.display).expect("write");
        if let Some(range) = fonts.range.get(&face.range) {
            writeln!(out, "  unicode-range: {range};").expect("write");
        }
        writeln!(out, "}}\n").expect("write");
    }

    // The sliced families. One `@font-face` per slice, each with the range the
    // subsetting computed from the file itself.
    //
    // Emitted after the whole faces so that a codepoint covered by both — the
    // unified ideographs, which Japanese and Chinese share — resolves through
    // the stack's order rather than through where a rule happened to land.
    for manifest in sliced {
        let declaration = fonts
            .sliced
            .iter()
            .find(|s| s.family == manifest.family)
            .expect("a manifest is only loaded for a declared family");

        writeln!(
            out,
            "/* {} — {} slices. {} */",
            manifest.family,
            manifest.slices.len(),
            declaration.reason
        )
        .expect("write");

        for slice in &manifest.slices {
            writeln!(out, "@font-face {{").expect("write");
            writeln!(out, "  font-family: \"{}\";", manifest.family).expect("write");
            writeln!(
                out,
                "  src: url(\"{ASSETS}/{}\") format(\"woff2\");",
                slice.file
            )
            .expect("write");
            writeln!(out, "  font-style: normal;").expect("write");
            if declaration.axes.iter().any(|a| a == "wght") {
                writeln!(out, "  font-weight: 100 900;").expect("write");
            } else {
                writeln!(out, "  font-weight: 400;").expect("write");
            }
            writeln!(out, "  font-display: {};", declaration.display).expect("write");
            // Never absent. A slice with no range is fetched on every page,
            // which is the one thing slicing exists to prevent.
            writeln!(out, "  unicode-range: {};", slice.range).expect("write");
            writeln!(out, "}}\n").expect("write");
        }
    }

    writeln!(
        out,
        "/* The stacks. Every fallback named here ships with some major operating\n\
        \x20  system, so the interface is fully usable before our faces load and if\n\
        \x20  they never arrive at all.\n\
        \x20\n\
        \x20  \"Symbols Nerd Font\" is always last and is safe there: it holds no\n\
        \x20  letters, so it can only ever supply an icon glyph nothing before it\n\
        \x20  had. */"
    )
    .expect("write");
    writeln!(out, ":root {{").expect("write");
    for (role, stack) in &fonts.stack {
        writeln!(out, "  --{prefix}-font-{role}: {stack};").expect("write");
    }
    writeln!(out, "}}").expect("write");

    vec![EmittedFile::new("dist/fonts.css", out)]
}

/// The `<link rel="preload">` tags the default payload needs.
///
/// Emitted as a fragment rather than injected into a page, so a page — the
/// documentation site's, or a consumer's — includes a file rather than
/// reconstructing the list, and the list has exactly one source.
#[must_use]
pub fn preload_fragment(fonts: &Fonts) -> EmittedFile {
    let mut out = String::from(
        "<!-- Generated by noctua-design — do not edit.\n\
        \x20    Source: spec/fonts.toml\n\
        \x20    Regenerate: cargo xtask build\n\n\
        \x20    Every face in the default payload, not just the regular. `font-display:\n\
        \x20    optional` uses a face only if it is ready by the first paint and never\n\
        \x20    swaps afterwards, so a face that is not preloaded essentially never\n\
        \x20    renders — and preloading only the regular leaves bold and italic\n\
        \x20    falling back inconsistently from one reload to the next, which reads\n\
        \x20    as the typeface flickering. -->\n",
    );
    for face in fonts.face.iter().filter(|f| f.preload) {
        writeln!(
            out,
            "<link rel=\"preload\" as=\"font\" type=\"font/woff2\" crossorigin \
             href=\"{ASSETS}/{}\">",
            face.file
        )
        .expect("write");
    }
    EmittedFile::new("dist/preload.html", out)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn shipped() -> (Spec, Fonts) {
        let root = concat!(env!("CARGO_MANIFEST_DIR"), "/../..");
        (
            noctua_design_spec::load(format!("{root}/spec/tokens.toml")).expect("tokens"),
            noctua_design_spec::load_fonts(format!("{root}/spec/fonts.toml")).expect("fonts"),
        )
    }

    /// Every sliced family's manifest, read from beside its slices.
    fn manifests() -> Vec<noctua_design_spec::Manifest> {
        let root = std::path::PathBuf::from(concat!(env!("CARGO_MANIFEST_DIR"), "/../.."));
        let fonts = noctua_design_spec::load_fonts(root.join("spec/fonts.toml")).expect("fonts");
        fonts
            .sliced
            .iter()
            .map(|family| family.load(&root.join("assets/fonts")).expect("manifest"))
            .collect()
    }

    fn css() -> String {
        let (spec, fonts) = shipped();
        emit(&spec, &fonts, &manifests()).remove(0).contents
    }

    /// One block per face, so a face's declarations can be read together.
    fn blocks(css: &str) -> Vec<&str> {
        css.split("@font-face {").skip(1).collect()
    }

    /// **Trap 1.** Every variable face declares the full weight range.
    ///
    /// Without it the browser renders the default instance, which for four of
    /// the Noto families is Thin. This is the single most expensive silent
    /// mistake available in this file.
    #[test]
    fn every_variable_face_declares_the_full_weight_range() {
        let (_, fonts) = shipped();
        let css = css();
        let blocks = blocks(&css);
        // The whole faces come first, then the slices — so zipping against
        // `fonts.face` reads exactly the blocks this test is about.
        let slices: usize = manifests().iter().map(|m| m.slices.len()).sum();
        assert_eq!(
            blocks.len(),
            fonts.face.len() + slices,
            "one block per face and per slice"
        );

        for (face, block) in fonts.face.iter().zip(&blocks) {
            if face.is_variable_weight() {
                assert!(
                    block.contains("font-weight: 100 900;"),
                    "{} varies weight but does not declare the range — it will render \
                     at its default instance",
                    face.file
                );
            } else {
                assert!(
                    !block.contains("font-weight: 100 900;"),
                    "{} is static; a range would claim weights it does not have",
                    face.file
                );
            }
        }
    }

    /// **Trap 2.** `font-stretch` for exactly the faces with a `wdth` axis.
    ///
    /// Declared on a face without one, the browser synthesizes a width. Absent
    /// from one that has it, the axis is unreachable.
    #[test]
    fn font_stretch_is_declared_for_exactly_the_faces_with_a_width_axis() {
        let (_, fonts) = shipped();
        let css = css();
        for (face, block) in fonts.face.iter().zip(blocks(&css)) {
            assert_eq!(
                block.contains("font-stretch:"),
                face.is_variable_width(),
                "{} declares axes {:?}, so font-stretch should be {}",
                face.file,
                face.axes,
                if face.is_variable_width() {
                    "present"
                } else {
                    "absent"
                }
            );
        }
    }

    /// Every face must carry a range, or load-on-demand is not happening and
    /// the browser fetches everything.
    #[test]
    fn every_face_carries_a_unicode_range() {
        let css = css();
        for block in blocks(&css) {
            assert!(
                block.contains("unicode-range:"),
                "a face has no range: {block}"
            );
        }
    }

    /// Every file named must exist. A `src` pointing at nothing is a 404 the
    /// browser reports and the build does not.
    #[test]
    fn every_face_file_exists() {
        let (_, fonts) = shipped();
        let assets = concat!(env!("CARGO_MANIFEST_DIR"), "/../../assets/fonts");
        for face in &fonts.face {
            let path = std::path::Path::new(assets).join(&face.file);
            assert!(path.exists(), "{} does not exist", face.file);
        }
        // And every slice. A manifest listing a file that is not there is the
        // easy way to lose a range: the CSS declares it, the browser 404s, and
        // the codepoints in that range render as tofu.
        for manifest in manifests() {
            for slice in &manifest.slices {
                let path = std::path::Path::new(assets).join(&slice.file);
                assert!(path.exists(), "{} does not exist", slice.file);
            }
        }
    }

    /// A slice with no range is fetched on every page, which is the one thing
    /// slicing exists to prevent.
    #[test]
    fn every_slice_declares_a_range() {
        for manifest in manifests() {
            assert!(!manifest.slices.is_empty(), "{} has none", manifest.family);
            for slice in &manifest.slices {
                assert!(
                    slice.range.starts_with("U+"),
                    "{} has no unicode-range",
                    slice.file
                );
                assert!(slice.bytes > 0, "{} is empty", slice.file);
            }
        }
    }

    /// No two slices of one family may claim the same codepoint.
    ///
    /// They are cut on codepoint order, so an overlap would mean the cutting
    /// went wrong — and two files answering one codepoint is two downloads for
    /// one glyph.
    #[test]
    fn a_family_slices_without_overlapping_itself() {
        for manifest in manifests() {
            let mut seen: Vec<(u32, u32)> = Vec::new();
            for slice in &manifest.slices {
                for part in slice.range.split(',') {
                    let part = part.trim().trim_start_matches("U+");
                    let (lo, hi) = part.split_once('-').unwrap_or((part, part));
                    let lo = u32::from_str_radix(lo, 16).expect("hex");
                    let hi = u32::from_str_radix(hi, 16).expect("hex");
                    for (a, b) in &seen {
                        assert!(
                            hi < *a || lo > *b,
                            "{} overlaps an earlier slice of {}",
                            slice.file,
                            manifest.family
                        );
                    }
                    seen.push((lo, hi));
                }
            }
        }
    }

    /// The default payload's budget, so it cannot drift.
    ///
    /// `assets/fonts/ATTRIBUTION.md`, "The default payload". The number is the
    /// budget, not the current size: a build that comes in under it passes,
    /// and one that creeps over it fails and says by how much.
    #[test]
    fn the_default_payload_stays_within_its_budget() {
        const BUDGET: u64 = 200 * 1024;
        let (_, fonts) = shipped();
        let assets = concat!(env!("CARGO_MANIFEST_DIR"), "/../../assets/fonts");

        let mut total = 0;
        for face in fonts.face.iter().filter(|f| f.preload) {
            let path = std::path::Path::new(assets).join(&face.file);
            total += std::fs::metadata(&path)
                .unwrap_or_else(|e| panic!("{}: {e}", face.file))
                .len();
        }

        assert!(
            total <= BUDGET,
            "the preloaded payload is {} KB against a budget of {} KB",
            total / 1024,
            BUDGET / 1024
        );
        assert!(
            total > 0,
            "nothing is preloaded, so the budget proves nothing"
        );
    }

    /// No CJK face may be in the default payload. Group B is 15.15 MiB.
    #[test]
    fn no_cjk_face_is_preloaded() {
        let (_, fonts) = shipped();
        for face in fonts.face.iter().filter(|f| f.preload) {
            for cjk in ["JP", "KR", "SC", "TC"] {
                assert!(
                    !face.file.contains(cjk),
                    "{} is a CJK face and must never be in the default payload",
                    face.file
                );
            }
        }
    }

    /// The preload fragment lists every preloaded face and nothing else.
    #[test]
    fn the_fragment_lists_exactly_the_default_payload() {
        let (_, fonts) = shipped();
        let fragment = preload_fragment(&fonts).contents;
        for face in &fonts.face {
            assert_eq!(
                fragment.contains(&face.file),
                face.preload,
                "{} is preload={} but the fragment disagrees",
                face.file,
                face.preload
            );
        }
    }

    /// A stack variable per role, and no component may bypass them.
    #[test]
    fn every_stack_becomes_a_variable() {
        let (_, fonts) = shipped();
        let css = css();
        for role in fonts.stack.keys() {
            assert!(
                css.contains(&format!("--noctua-font-{role}:")),
                "the {role} stack is missing"
            );
        }
    }

    /// The icon face is last in the text stacks, where it can only ever supply
    /// a glyph nothing before it had.
    #[test]
    fn the_symbols_face_is_last_in_every_text_stack() {
        let (_, fonts) = shipped();
        for (role, stack) in &fonts.stack {
            if !stack.contains("Symbols Nerd Font") {
                continue;
            }
            // Split into families properly rather than on a substring:
            // "Symbols Nerd Font Mono" contains "Symbols Nerd Font", so a
            // substring split silently leaves `Mono"` behind and the assertion
            // measures the wrong thing.
            let families: Vec<String> = stack
                .split(',')
                .map(|f| f.trim().trim_matches('"').to_owned())
                .filter(|f| !f.is_empty())
                .collect();
            let symbols = families
                .iter()
                .position(|f| f.starts_with("Symbols Nerd Font"))
                .expect("the stack contains the symbols family");
            let after: Vec<&String> = families[symbols + 1..].iter().collect();

            assert!(
                after.iter().all(|f| {
                    ["sans-serif", "serif", "monospace", "cursive", "system-ui"]
                        .contains(&f.as_str())
                }),
                "in the {role} stack, {after:?} follow the symbols face; only a \
                 generic may, or the symbols face could win a codepoint a text font \
                 should have rendered"
            );
        }
    }

    #[test]
    fn braces_balance() {
        let css = css();
        assert_eq!(css.matches('{').count(), css.matches('}').count());
    }
}
