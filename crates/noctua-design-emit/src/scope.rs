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
const SELECTOR: &str = ":where(:root),\n\
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

    vec![EmittedFile::new("dist/scope.css", out)]
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

    /// No colour literal can reach this file. The vendor reader already
    /// rejects a non-`var()` value; this asserts the emitted result, which is
    /// what actually ships.
    #[test]
    fn the_emitted_file_defines_no_colour() {
        let css = css();
        for needle in ["#", "rgb(", "hsl(", "oklch(", "oklab("] {
            assert!(
                !css.contains(needle),
                "the scope stylesheet contains {needle:?}; this repository defines no colours"
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

    /// One rule, so the whole contract shares one selector and there is no
    /// second copy to drift.
    #[test]
    fn it_is_exactly_one_rule() {
        let css = css();
        assert_eq!(css.matches('{').count(), css.matches('}').count());
        let bodies = css.matches('{').count();
        // The generated header is a `/*! … */` comment and contributes none.
        assert_eq!(bodies, 1, "expected one rule, found {bodies}");
    }
}
