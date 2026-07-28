//! Which typeface a run of text can actually be set in.
//!
//! `NOCTUA.md §7.7`: the reader may choose the monospaced interface font, and
//! Noctua Iosevka covers Latin, Greek and Cyrillic and nothing else. Every other
//! script has to fall back to Noto — **per component**, not per page.
//!
//! # Why per component, and why the whole component
//!
//! Rebinding at the page level would put every non-Latin page back on Noto even
//! where Iosevka would have been fine. Rebinding per *glyph* — which is what the
//! browser does on its own through the fallback chain — produces a line set in
//! two typefaces, with two different x-heights and two different weights on the
//! same baseline. That looks like a rendering bug and is one of the two things
//! §7.7 exists to prevent.
//!
//! So the decision is made once per component, over its whole content, and the
//! whole component moves together.
//!
//! # One implementation, and this is it
//!
//! Every text component calls [`dominant_script`]; none reimplements it. The
//! React package calls the same logic, generated from the same table, so the two
//! cannot disagree about whether a string is Devanagari.

/// The scripts this system distinguishes.
///
/// Not every script in Unicode — only the distinction that matters here, which
/// is "can Iosevka set this". Everything Iosevka cannot set is treated the same
/// way, so the enum names the *coverage* rather than cataloguing the world.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Script {
    /// Latin, Greek or Cyrillic — the three Iosevka covers.
    Monospaceable,
    /// Anything else: Arabic, Hebrew, Devanagari, Thai, CJK, and the rest.
    Proportional,
}

impl Script {
    /// The font stack a run of this script must use.
    #[must_use]
    pub const fn stack(self) -> &'static str {
        match self {
            // The reader's choice stands: Iosevka can set this.
            Self::Monospaceable => "var(--noctua-font-mono)",
            // It cannot, so the whole component moves to the proportional face
            // rather than letting the browser mix two faces on one line.
            Self::Proportional => "var(--noctua-font-sans)",
        }
    }
}

/// Whether a character is one Iosevka can set.
///
/// A table of ranges rather than a Unicode property lookup, for the reason
/// `NOCTUA.md §2.3` gives about dependencies: the question being asked is
/// narrow, the answer is a dozen ranges, and a crate to answer it would be
/// inherited by every consumer of this design system.
const fn is_monospaceable(ch: char) -> bool {
    matches!(ch,
        // Basic Latin and the Latin supplements, including every accented
        // letter Portuguese uses.
        '\u{0000}'..='\u{024F}'
        // Greek and Coptic, then Cyrillic and its supplement.
        | '\u{0370}'..='\u{03FF}'
        | '\u{0400}'..='\u{052F}'
        // Punctuation, currency and the symbol blocks a Latin page uses.
        | '\u{2000}'..='\u{206F}'
        | '\u{20A0}'..='\u{20CF}'
        | '\u{2100}'..='\u{214F}'
    )
}

/// The script a run of text is dominantly in.
///
/// **Dominant, not exclusive.** An English sentence containing one Japanese
/// name must not move the whole paragraph to the proportional face; a Japanese
/// sentence containing one English word must. Counting the letters and taking
/// the majority is what gets both cases right, and it is why this is a function
/// rather than a `contains` check.
///
/// Characters that belong to no script — spaces, digits, punctuation — are not
/// counted at all. Counting them would let a line of mostly punctuation decide
/// the typeface for the words around it.
#[must_use]
pub fn dominant_script(text: &str) -> Script {
    let mut monospaceable = 0_usize;
    let mut proportional = 0_usize;

    for ch in text.chars() {
        if !ch.is_alphabetic() {
            continue;
        }
        if is_monospaceable(ch) {
            monospaceable += 1;
        } else {
            proportional += 1;
        }
    }

    // Ties and empty strings go to the reader's choice: with nothing to weigh,
    // overriding what they picked would be the wrong default.
    if proportional > monospaceable {
        Script::Proportional
    } else {
        Script::Monospaceable
    }
}

/// The inline style a component's root needs, or `None` when it needs none.
///
/// `None` for the common case matters: emitting `font-family: var(--noctua-font-mono)`
/// on every component would mean every component overrides the reader's setting
/// with the same value the setting already had, and a page-level change would
/// stop reaching anything.
#[must_use]
pub fn font_override(text: &str) -> Option<&'static str> {
    match dominant_script(text) {
        Script::Monospaceable => None,
        Script::Proportional => Some(Script::Proportional.stack()),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn latin_greek_and_cyrillic_stay_monospaced() {
        for text in ["Hello", "Espaçamento não", "Ελληνικά", "Кириллица", "Ação"]
        {
            assert_eq!(dominant_script(text), Script::Monospaceable, "{text}");
            assert_eq!(font_override(text), None, "{text}");
        }
    }

    #[test]
    fn the_scripts_iosevka_lacks_fall_back() {
        for text in [
            "العربية",  // Arabic
            "עברית",    // Hebrew
            "देवनागरी",  // Devanagari
            "ไทย",      // Thai
            "አማርኛ",     // Ethiopic
            "ქართული",  // Georgian
            "Հայերեն",  // Armenian
            "日本語",   // Japanese
            "한국어",   // Korean
            "简体中文", // Simplified Chinese
            "ᠮᠣᠩᠭᠣᠯ",   // Mongolian
        ] {
            assert_eq!(dominant_script(text), Script::Proportional, "{text}");
            assert!(font_override(text).is_some(), "{text}");
        }
    }

    /// The case the whole design turns on: one foreign word must not move the
    /// paragraph, and one native word must not hold it back.
    #[test]
    fn the_majority_decides_not_the_presence() {
        assert_eq!(
            dominant_script("The city of 東京 is large"),
            Script::Monospaceable,
            "two Japanese characters moved an English sentence"
        );
        assert_eq!(
            dominant_script("東京都は日本の首都です and large"),
            Script::Proportional,
            "one English word held back a Japanese sentence"
        );
    }

    #[test]
    fn punctuation_and_digits_do_not_vote() {
        // Otherwise a line of mostly punctuation would pick the typeface for
        // the words around it.
        assert_eq!(dominant_script("日本 — 2026 (…)"), Script::Proportional);
        assert_eq!(dominant_script("!!! 123 ..."), Script::Monospaceable);
    }

    #[test]
    fn nothing_at_all_leaves_the_readers_choice_alone() {
        assert_eq!(font_override(""), None);
        assert_eq!(font_override("   "), None);
    }
}
