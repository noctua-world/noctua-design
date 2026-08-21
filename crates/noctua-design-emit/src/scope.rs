//! `dist/scope.css` — the semantic contract, made scopable.
//!
//! # What this file fixes
//!
//! `noctua-colors` emits the semantic contract once, at `:where(:root)`:
//!
//! ```css
//! :where(:root) {
//!   --nc-color-surface: var(--nc-neutral-bg-app);
//!   /* …1766 more */
//! }
//! ```
//!
//! A `var()` is substituted at the element where the property is **declared**,
//! and the result inherits as an already-resolved value. So a subtree carrying
//! `data-palette="blue-vivid"` gets blue's `--nc-neutral-*` and keeps the
//! **root's** `--nc-color-surface`. A component painted from the semantic
//! contract — which is what every component is told to do — silently ignores
//! its own theme.
//!
//! This file re-emits the same body under a selector list that also matches a
//! scope, so the indirection is resolved *again* at the scope element, against
//! that element's palette.
//!
//! # The selector, and why it carries no palette
//!
//! ```css
//! :where(:root),
//! [data-noctua-scope],
//! [data-palette]:not(:root),
//! [data-theme]:not(:root) { … }
//! ```
//!
//! Every declaration is a pure indirection, so it resolves against whatever
//! element matched — and that element already carries `data-palette` and
//! `data-theme`. It therefore picks up the right palette **and** the right
//! gamut layer for free, because the vendored `@supports` and `color-gamut`
//! blocks target the same `[data-palette="…"]` head.
//!
//! Compounding the palette into the selector would mean thirty-nine copies of
//! a 97 KB rule for no behavioural difference.
//!
//! # Specificity
//!
//! | Selector | | Against |
//! |---|---|---|
//! | `:where(:root)` | (0,0,0) | Zero on purpose, so a theme overriding a slot wins whatever the link order |
//! | `[data-noctua-scope]` | (0,1,0) | A harmless no-op on the root: same declarations, same values |
//! | `[data-palette]:not(:root)` | (0,2,0) | The safety net for hand-written subtrees. `:not(:root)` leaves the root governed by the zero-specificity rule |
//!
//! A page-level `[data-palette="x"]` never contends with any of these: it
//! declares `--nc-neutral-*`, this file declares `--nc-color-*`.
//!
//! **`.light` and `.dark` are deliberately absent.** They are extremely generic
//! class names, and claiming them would attach 1,767 declarations to any
//! consumer element that happens to use one.
//!
//! # This replaces `contexts.css` in the base link set
//!
//! Because the list includes `:where(:root)`, a page links this file *instead
//! of* the vendored one — never both, which would double the cost and raise a
//! question about which governs the root that has no interesting answer.

use std::fmt::Write as _;

use noctua_design_spec::Vendor;

use crate::{CommentStyle, EmittedFile, header};

/// The attribute a `ThemeScope` stamps.
pub const SCOPE_ATTRIBUTE: &str = "data-noctua-scope";

/// The selector list, as one string, so the emitter and its tests cannot
/// disagree about what was written.
///
/// `pub` because `appearance` needs it too: the backdrop is a property a scope
/// declares, and two files spelling "where a scope can be" separately is two
/// spellings that stop agreeing.
pub const SELECTOR: &str = ":where(:root),\n\
                           [data-noctua-scope],\n\
                           [data-palette]:not(:root),\n\
                           [data-theme]:not(:root)";

/// Emits `dist/scope.css`.
#[must_use]
pub fn emit(vendor: &Vendor) -> Vec<EmittedFile> {
    let mut out = header(CommentStyle::Block);

    writeln!(
        out,
        "\n/* The semantic contract — --nc-color-* — declared so that it also\n\
        \x20  resolves inside a themed subtree.\n\
        \x20\n\
        \x20  Derived from packages/tokens/vendor/css/contexts.css, which emits it\n\
        \x20  once at :where(:root). A var() resolves where it is DECLARED, so a\n\
        \x20  subtree carrying data-palette gets new --nc-neutral-* values and keeps\n\
        \x20  the root's --nc-color-surface. Re-declaring the same indirections here\n\
        \x20  resolves them again, at the scope element, against its palette.\n\
        \x20\n\
        \x20  Link this INSTEAD OF the vendored contexts.css — it includes\n\
        \x20  :where(:root), so it covers the page as well as every scope.\n\
        \x20\n\
        \x20  {} names, transcribed byte-for-byte. Regenerate rather than edit. */\n",
        vendor.contract.len()
    )
    .expect("write");

    writeln!(out, "{SELECTOR} {{").expect("write");
    for (name, value) in &vendor.contract {
        writeln!(out, "  {name}: {value};").expect("write");
    }
    writeln!(out, "}}").expect("write");

    ink(&mut out);

    vec![EmittedFile::new("dist/scope.css", out)]
}

/// The ink, re-read at the scope — the same bug as the contract, one level up.
///
/// # What was wrong
///
/// Everything above re-declares `--nc-color-*` so a subtree resolves them
/// against its own palette. It does nothing for the properties that CONSUME
/// them, and `color` is the one that matters: an application writes
/// `body { color: var(--nc-color-fg) }` once, that `var()` resolves at `<body>`
/// against the PAGE's mode, and every descendant inherits the **computed
/// value**. A scope below it re-resolves the contract, paints its own backdrop
/// — and its text is still the page's ink.
///
/// Measured, on a dark scope inside a light page: near-black text on a
/// near-black surface, a contrast ratio of about 1.2. Nothing reported it,
/// because every component that sets its own colour looked right; only prose
/// that inherited was wrong. `readability.spec.js` measures it now, and that
/// test had to be written twice — the first version screenshotted a probe over
/// a live page and caught the page's furniture rather than its own glyphs.
///
/// # Why here and not with the backdrop
///
/// This is the *semantic* half — the same sentence this file is named after,
/// applied to the property rather than to the custom property — so it takes the
/// same selector list. The backdrop is painted in `dist/theme.css` on `:root`
/// and `[data-noctua-scope]` only, because painting a surface is a decision and
/// re-resolving a colour is a correction.
///
/// # Why zero specificity
///
/// `:where()`, so anything at all can override it: a scope inside a callout
/// that deliberately sets its own ink still wins, and this only ever answers
/// where nothing else does. Inheritance loses to any matching rule, including a
/// (0,0,0) one, which is what makes this enough.
fn ink(out: &mut String) {
    writeln!(
        out,
        "\n/* The ink, re-read here. `color` is set once, high up, from\n\
        \x20  var(--nc-color-fg) — and a var() resolves where it is DECLARED, so\n\
        \x20  every descendant inherits the COMPUTED colour and a themed subtree\n\
        \x20  keeps the page's ink over its own surface. Measured before this rule\n\
        \x20  existed, on a dark scope inside a light page: near-black text on a\n\
        \x20  near-black surface, a contrast ratio of about 1.2.\n\
        \x20\n\
        \x20  Zero specificity, so any deliberate colour still wins. */\n\
        :where({}) {{\n\
        \x20 color: var(--nc-color-fg);\n\
        }}",
        SELECTOR.replace('\n', " ")
    )
    .expect("write");
}

#[cfg(test)]
mod tests {
    use super::*;

    fn vendor() -> Vendor {
        noctua_design_spec::vendor::load(&noctua_design_spec::vendor::repository_root())
            .expect("the vendored output")
    }

    fn css() -> String {
        emit(&vendor()).remove(0).contents
    }

    /// The contract is transcribed, not rewritten.
    ///
    /// Every name and every value, in the vendor's order. A reformatting
    /// emitter, a partial extraction or a dropped tail would all pass a
    /// "contains surface" check and fail this one.
    #[test]
    fn the_contract_is_transcribed_byte_for_byte() {
        let vendor = vendor();
        let css = css();
        let body = css.split_once('{').expect("a rule").1;

        let emitted: Vec<&str> = body
            .lines()
            .map(str::trim)
            .filter(|line| line.starts_with("--nc-color-"))
            .collect();

        assert_eq!(
            emitted.len(),
            vendor.contract.len(),
            "the emitted contract has a different number of names than the vendor's"
        );
        for (emitted, (name, value)) in emitted.iter().zip(&vendor.contract) {
            assert_eq!(*emitted, format!("{name}: {value};"));
        }
    }

    /// No color literal can reach this file. The vendor reader already
    /// rejects a non-`var()` value; this asserts the emitted result, which is
    /// what actually ships.
    #[test]
    fn the_emitted_file_defines_no_color() {
        let css = css();
        for needle in ["#", "rgb(", "hsl(", "oklch(", "oklab("] {
            assert!(
                !css.contains(needle),
                "the scope stylesheet contains {needle:?}; this repository defines no colors"
            );
        }
    }

    /// The selector must cover the root **and** every way a scope is marked.
    ///
    /// Written as four literal assertions rather than one string comparison so
    /// a failure says which head went missing.
    #[test]
    fn the_selector_covers_the_root_and_every_scope_form() {
        let css = css();
        for head in [
            ":where(:root)",
            "[data-noctua-scope]",
            "[data-palette]:not(:root)",
            "[data-theme]:not(:root)",
        ] {
            assert!(css.contains(head), "the selector list lost {head}");
        }
    }

    /// `.light` / `.dark` must never be claimed.
    ///
    /// They are generic class names, and attaching 1,767 declarations to them
    /// would reach into any consumer that happens to use one.
    #[test]
    fn the_generic_mode_classes_are_not_claimed() {
        let css = css();
        let selector = css.split_once('{').expect("a rule").0;
        assert!(!selector.contains(".light"), "{selector}");
        assert!(!selector.contains(".dark"), "{selector}");
    }

    /// The root keeps zero specificity, so a theme's override still wins.
    #[test]
    fn the_root_head_is_still_zero_specificity() {
        let css = css();
        let selector = css.split_once('{').expect("a rule").0;
        assert!(
            selector.contains(":where(:root)") && !selector.contains("\n:root"),
            "the root must be matched through :where(), or a theme override could lose"
        );
    }

    /// TWO RULES, AND ONLY TWO: the contract, and the ink that consumes it.
    ///
    /// The contract is one rule so it shares one selector and there is no
    /// second copy to drift. The ink is the other, and it is separate because
    /// it is the only declaration here that is not a `--nc-` name — a reader
    /// diffing this file against the vendor's must be able to see the whole
    /// contract as one block and the one addition as the other.
    #[test]
    fn it_is_the_contract_and_the_ink_and_nothing_else() {
        let css = css();
        assert_eq!(css.matches('{').count(), css.matches('}').count());
        let bodies = css.matches('{').count();
        // The generated header is a `/*! … */` comment and contributes none.
        assert_eq!(bodies, 2, "expected two rules, found {bodies}");

        // And the second one is the ink, at zero specificity so anything
        // deliberate still wins. A scope that could not be overridden would
        // reach into every consumer who ever set a colour on a themed subtree.
        let before = css.rsplit_once('{').expect("a second rule").0;
        let head = before
            .rsplit_once("*/")
            .map_or(before, |(_, after)| after)
            .trim();
        assert!(
            head.starts_with(":where("),
            "the ink rule is not zero-specificity: {head}"
        );
    }
}

#[cfg(test)]
mod contrast {
    /// Every colour the switch draws, named in `shell.css` and measured here.
    ///
    /// # Why this is arithmetic and not a browser test
    ///
    /// The knob is one colour in both states, which is what makes it read as
    /// one object rather than two of different sizes — that was reported twice.
    /// Whether it carries against the track is a fact about all thirty-nine
    /// palettes in two appearances, not about the one a page happens to show,
    /// and three browser probes failed to switch palette before this was
    /// written. The vendored ramps are on disk and the sums are four lines.
    ///
    /// # Why it READS the stylesheet rather than restating it
    ///
    /// The first version of this test carried a copy of what the CSS did: a
    /// `const ACCENT: f64 = 0.74` beside a rule that mixed the accent 74% into
    /// the surface. It passed, and the thing it was guarding was already
    /// broken — a switch showing three quarters of the theme's accent is
    /// showing a colour the theme does not contain, which is a defect a reader
    /// sees and a contrast sum never will.
    ///
    /// So the five colours come out of `shell.css`, each has to be a WHOLE
    /// `var(--nc-color-…)` with nothing wrapped around it, and the semantic
    /// name is resolved through the vendor's own `contexts.css` to the palette
    /// token behind it. A `color-mix` in any of the five fails at the first
    /// step, before a ratio is computed. Nothing here is a number this file
    /// chose, and there is no second copy of the design to drift from.
    ///
    /// Measured, at the time of writing: 78 pairs, worst 3.13:1 on
    /// `amber-sober` light, which is the palette's own solid-to-`bg-app`
    /// guarantee arriving intact.
    #[test]
    fn every_switch_track_carries_its_knob() {
        /// `WCAG 1.4.11`, for a component's own parts.
        const FLOOR: f64 = 3.0;

        let root = std::path::PathBuf::from(concat!(env!("CARGO_MANIFEST_DIR"), "/../.."));
        let css = std::fs::read_to_string(root.join("docs-site/assets/shell.css")).expect("shell");
        let contexts =
            std::fs::read_to_string(root.join("packages/tokens/vendor/css/contexts.css"))
                .expect("contexts");

        let knob = palette_token(&contexts, &whole_token(&css, ".nd-switch-track::after"));
        let grounds: Vec<(&str, String)> = [
            ("off", ".nd-switch-track"),
            ("on", ".nd-switch[aria-checked=\"true\"] > .nd-switch-track"),
            ("off hovered", ".nd-switch:hover > .nd-switch-track"),
            (
                "on hovered",
                ".nd-switch[aria-checked=\"true\"]:hover > .nd-switch-track",
            ),
        ]
        .into_iter()
        .map(|(state, selector)| {
            (
                state,
                palette_token(&contexts, &whole_token(&css, selector)),
            )
        })
        .collect();

        let mut checked = 0usize;
        let mut worst = (f64::MAX, String::new());

        for entry in std::fs::read_dir(root.join("dist/palettes")).expect("palettes") {
            let path = entry.expect("entry").path();
            if path.extension().is_none_or(|e| e != "json") {
                continue;
            }
            let text = std::fs::read_to_string(&path).expect("read");
            let name = path
                .file_stem()
                .unwrap_or_default()
                .to_string_lossy()
                .to_string();

            for mode in ["light", "dark"] {
                let Some(ink) = token(&text, mode, &knob) else {
                    continue;
                };
                for (state, ground) in &grounds {
                    let track = token(&text, mode, ground)
                        .unwrap_or_else(|| panic!("{ground} is missing from {name} {mode}"));
                    let ratio = contrast(luminance(track), luminance(ink));
                    if ratio < worst.0 {
                        worst = (ratio, format!("{name} {mode}, {state}"));
                    }
                    assert!(
                        ratio >= FLOOR,
                        "the switch's knob is {ratio:.2}:1 against its {state} track on \
                         {name} {mode}, under the {FLOOR}:1 a component's own parts need"
                    );
                }
                checked += 1;
            }
        }

        // A sweep that read nothing would pass for ever.
        assert!(
            checked >= 70,
            "only {checked} palette and appearance pairs were read"
        );
        println!(
            "{checked} pairs x {} grounds, worst {:.2}:1 on {}",
            grounds.len(),
            worst.0,
            worst.1
        );
    }

    /// The `background` a selector sets, which has to be one whole token.
    ///
    /// Anything wrapped around it — a `color-mix`, a `linear-gradient`, a
    /// second colour — leaves the value not starting `var(--nc-color-` and
    /// fails here, which is the point: a colour this stylesheet DERIVED is a
    /// colour no palette in the system contains.
    fn whole_token(css: &str, selector: &str) -> String {
        let head = format!("\n{selector} {{");
        let at = css
            .find(&head)
            .unwrap_or_else(|| panic!("no rule for `{selector}` in shell.css"));
        let body = css[at + head.len()..]
            .split_once('}')
            .unwrap_or_else(|| panic!("`{selector}` has no closing brace"))
            .0;
        let value = body
            .split("\n  background:")
            .nth(1)
            .unwrap_or_else(|| panic!("`{selector}` sets no background"))
            .split(';')
            .next()
            .unwrap_or_default()
            .trim();
        let name = value
            .strip_prefix("var(")
            .and_then(|rest| rest.strip_suffix(')'))
            .filter(|name| name.starts_with("--nc-color-"))
            .unwrap_or_else(|| {
                panic!(
                    "`{selector}` draws `{value}` rather than one whole \
                     `var(--nc-color-…)`, so it is a colour no palette holds"
                )
            });
        name.to_string()
    }

    /// The palette token a semantic name stands for, per the vendor's contract.
    ///
    /// Resolved from `contexts.css` rather than from a table here, so a rename
    /// in the colour system reaches this test without anybody remembering to
    /// bring it.
    fn palette_token(contexts: &str, name: &str) -> String {
        let at = contexts
            .find(&format!("\n  {name}: var("))
            .unwrap_or_else(|| panic!("`{name}` is not in the semantic contract"));
        contexts[at..]
            .split_once("var(")
            .expect("checked above")
            .1
            .split_once(')')
            .expect("a closed var()")
            .0
            .to_string()
    }

    /// One `#rrggbb` from a palette file, as linear light.
    fn token(text: &str, mode: &str, name: &str) -> Option<[f64; 3]> {
        let block = text.split(&format!("\"{mode}\"")).nth(1)?;
        let at = block.find(name)?;
        let hex = block[at..].split('#').nth(1)?;
        let hex = &hex[..6];
        Some(std::array::from_fn(|i| {
            let byte = u8::from_str_radix(&hex[i * 2..i * 2 + 2], 16).unwrap_or(0);
            let v = f64::from(byte) / 255.0;
            if v <= 0.039_28 {
                v / 12.92
            } else {
                ((v + 0.055) / 1.055).powf(2.4)
            }
        }))
    }

    fn luminance(c: [f64; 3]) -> f64 {
        0.2126f64.mul_add(c[0], 0.7152f64.mul_add(c[1], 0.0722 * c[2]))
    }

    fn contrast(a: f64, b: f64) -> f64 {
        (a.max(b) + 0.05) / (a.min(b) + 0.05)
    }
}
