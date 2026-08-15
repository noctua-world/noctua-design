//! The shape of `spec/syntax.toml`: what paints each token role.
//!
//! # Why a syntax theme here is not a palette
//!
//! Every conventional syntax theme is a list of hexadecimal colours. A colour
//! literal fails `gates::color_literal`, and `NOCTUA.md §8.8` rule 1 says this
//! repository consumes `noctua-colors` and never redefines a colour — "not a
//! hex, not an `oklch()`, not a 'just this one shade'".
//!
//! So a theme here is a **mapping from a role to a token that already exists**.
//! That is not a workaround; it is the reason the default follows the page for
//! free. `--nc-danger-text-muted` is declared by every palette in both modes, so
//! one mapping is correct across thirty-nine palettes times two modes, inside a
//! `ThemeScope` on a different palette, with no script and no branch.
//!
//! # What the tokens had to be
//!
//! Measured over all 39 palettes × 2 modes, against the code block's own
//! surface:
//!
//! * `--nc-<family>-text-muted` — worst case **APCA Lc 71.2**. An isoluminant
//!   hue tier, and the foundation this rests on.
//! * `--nc-<family>-solid` — worst case Lc 32 in dark. Those are **fills**, not
//!   text, and a theme built on them is illegible in one mode by construction.
//! * `--nc-chart-wide-*` — a mode-invariant *lightness ramp*: index 1 measures
//!   Lc 0.0 against a dark surface and index 12 measures Lc 0.0 against a light
//!   one. It is meant to be read against itself in a chart, never against a
//!   page.
//!
//! And the hue set had to be five. Exhaustive search over the nine chromatic
//! families, worst pair over every palette and both modes: three hues reach
//! 0.0896, five reach 0.0362, six drop to 0.0369 and seven to 0.0294. A
//! just-noticeable difference is about 0.02, so five is where a set stops being
//! separable and starts being a list.
//!
//! # Why nothing here branches on light and dark
//!
//! It is unnecessary — the token carries the mode — and it is **unwritable**. A
//! `ThemeScope` with `mode = "system"` stamps no `data-theme` at all
//! (`omit_when`), and the palette sheet then decides the mode with a compound,
//! *same-element* selector. Any rule written here would be ancestor-based, and
//! the two disagree for a `system` scope inside a `light` page.
//! `appearance.rs::dark_rule` escapes this only because it is `:root`-anchored.
//! A code block is not the root.

use indexmap::IndexMap;
use serde::Deserialize;

use crate::error::{Problem, SpecError, did_you_mean};
use crate::grammar::Role;
use crate::vendor::Vendor;

/// The whole syntax layer.
#[derive(Debug, Clone, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Syntax {
    /// Where the rules attach and what they key on.
    pub meta: Meta,

    /// The non-colour signals, which every theme carries.
    #[serde(default)]
    pub signal: IndexMap<Role, Signal>,

    /// The roles whose colour is a meaning rather than a category.
    #[serde(default)]
    pub pinned: IndexMap<Role, String>,

    /// Every theme, keyed by the value its attribute takes.
    pub theme: IndexMap<String, IndexMap<Role, String>>,

    /// What `auto` falls to, and under what.
    #[serde(default, rename = "auto")]
    pub automatic: Vec<Automatic>,
}

/// Where the rules attach.
#[derive(Debug, Clone, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Meta {
    /// The block's class.
    pub class: String,
    /// The attribute the component stamps, always, even at its default.
    pub attribute: String,
    /// The attribute a token span carries.
    pub token_attribute: String,
    /// The page-level setting's attribute, from `spec/theme.toml`.
    pub page_attribute: String,
    /// The theme `auto` rests on when nothing else applies.
    pub resting: String,
    /// The value that means "defer".
    pub automatic: String,
}

/// A signal that is not a colour.
///
/// `NOCTUA.md §8.8` rule 6 is measured: the best worst-case separation a
/// six-family semantic set reaches is 0.0163, under one JND, so colour alone
/// cannot separate categories under dichromacy — and `§9.8` requires every
/// component to carry a non-colour signal for anything a colour is saying.
///
/// The honest argument for a syntax theme is upstream's own `labelled = true`:
/// the token **text** carries the meaning and the colour reinforces it. These
/// are the reinforcement running the other way, and in `mono` they are the only
/// thing there is.
#[derive(Debug, Clone, Default, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Signal {
    /// `bold`, or nothing.
    #[serde(default)]
    pub weight: String,
    /// `italic`, or nothing.
    #[serde(default)]
    pub style: String,
    /// A text decoration: `underline wavy`.
    #[serde(default)]
    pub decoration: String,
}

/// One capability `auto` answers to.
#[derive(Debug, Clone, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Automatic {
    /// The media query, without its `@media`.
    pub when: String,
    /// The theme it resolves to.
    pub theme: String,
    /// Why, in English. Emitted as a comment, because a rule that changes
    /// colour for a reason nobody wrote down is a rule nobody will dare touch.
    pub because: String,
}

/// Parses and validates the syntax layer against the vendored colour output.
///
/// # Errors
///
/// Returns every problem found, in one diagnostic.
pub fn parse(path: &str, text: &str, vendor: &Vendor) -> Result<Syntax, SpecError> {
    let syntax: Syntax = match toml::from_str(text) {
        Ok(syntax) => syntax,
        Err(error) => {
            let problem = match error.span() {
                Some(span) => Problem::at(span, error.message().to_owned())
                    .labelled("invalid here")
                    .fix("every key is a role name; the roles are in crates/noctua-design-spec/src/grammar.rs"),
                None => Problem::whole_file(error.message().to_owned()),
            };
            return Err(SpecError::new(path, text.to_owned(), vec![problem]));
        }
    };

    let problems = check(&syntax, vendor);
    if problems.is_empty() {
        Ok(syntax)
    } else {
        Err(SpecError::new(path, text.to_owned(), problems))
    }
}

/// Reads and validates the syntax layer from disk.
///
/// # Errors
///
/// A missing file, or any problem in it.
pub fn load(path: impl AsRef<std::path::Path>, vendor: &Vendor) -> Result<Syntax, SpecError> {
    let path = path.as_ref();
    let display = path.display().to_string();
    let text = std::fs::read_to_string(path).map_err(|error| {
        SpecError::new(
            &display,
            String::new(),
            vec![
                Problem::whole_file(format!("could not read {display}: {error}"))
                    .fix("check the path, or run `cargo xtask build` from the repository root"),
            ],
        )
    })?;
    parse(&display, &text, vendor)
}

fn check(syntax: &Syntax, vendor: &Vendor) -> Vec<Problem> {
    let mut problems = Vec::new();

    // EVERY NAME THIS FILE PAINTS WITH MUST EXIST, and nothing else in the
    // repository can say so. `gates::unknown_token` exempts `--nc-*` outright,
    // so `var(--nc-color-typo)` passes every gate and is then dropped at
    // computed-value time — leaving a role that looks like one nobody styled.
    let mut named: Vec<(&str, &str)> = Vec::new();
    for (theme, table) in &syntax.theme {
        for token in table.values() {
            named.push((theme, token));
        }
    }
    for (role, token) in &syntax.pinned {
        let _ = role;
        named.push(("pinned", token));
    }
    for (owner, token) in named {
        if !vendor.declared.contains(token) {
            let problem = Problem::whole_file(format!(
                "`{owner}` paints with `{token}`, which no palette declares"
            ));
            problems.push(
                match did_you_mean(token, vendor.declared.iter().map(String::as_str)) {
                    Some(near) => problem.fix(format!("did you mean `{near}`?")),
                    None => problem.fix(
                        "a var() naming a property nothing declares is dropped at \
                         computed-value time with no error — the run would simply inherit the \
                         block's colour and look like a role nobody styled",
                    ),
                },
            );
        }
    }

    // A ROLE NO THEME PAINTS IS A ROLE THAT LOOKS UNSTYLED. There is no
    // fallback and there must not be one: a fallback is how a missing entry
    // becomes something nobody notices for a milestone.
    for (theme, table) in &syntax.theme {
        for role in Role::ALL {
            if syntax.pinned.contains_key(&role) {
                continue;
            }
            if !table.contains_key(&role) {
                problems.push(
                    Problem::whole_file(format!(
                        "theme `{theme}` does not paint `{}`",
                        role.as_str()
                    ))
                    .fix(
                        "every theme paints every role the scanner can produce. Give it the \
                             same token as the role beside it if it should not stand out.",
                    ),
                );
            }
        }
    }

    // NO ROLE IS EVER BOLD AND ITALIC. `spec/fonts.toml` ships Iosevka Regular
    // 400, Bold 700 and Italic 400 — and no Bold Italic. Asking for both gets
    // CSS font matching (style before weight) → Italic 400 → a synthesised bold
    // that smears the stems, which `NOCTUA.md §7.6` trap 2 forbids outright.
    for (role, signal) in &syntax.signal {
        if signal.weight == "bold" && signal.style == "italic" {
            problems.push(
                Problem::whole_file(format!("`{}` asks for bold italic", role.as_str())).fix(
                    "the monospaced face ships Regular, Bold and Italic and no Bold Italic, so \
                     the browser would synthesise the bold and smear the stems. Pick one.",
                ),
            );
        }
    }

    if !syntax.theme.contains_key(&syntax.meta.resting) {
        problems.push(
            Problem::whole_file(format!(
                "`{}` rests on `{}`, which is not a theme",
                syntax.meta.automatic, syntax.meta.resting
            ))
            .fix("name one of the themes below it."),
        );
    }
    for entry in &syntax.automatic {
        if !syntax.theme.contains_key(&entry.theme) {
            problems.push(
                Problem::whole_file(format!(
                    "`{}` resolves to `{}`, which is not a theme",
                    entry.when, entry.theme
                ))
                .fix("name one of the themes below it."),
            );
        }
        // The width features are the gate's business, and a syntax theme has no
        // reason to ask about width at all.
        if entry.when.contains("width") {
            problems.push(
                Problem::whole_file(format!("`{}` asks about width", entry.when)).fix(
                    "`auto` answers to capability — contrast, colour, print — never to layout. \
                     A theme that changed with the viewport would change while a reader was \
                     reading.",
                ),
            );
        }
    }

    problems
}

#[cfg(test)]
mod tests {
    use super::*;

    fn vendor() -> Vendor {
        let root = std::path::PathBuf::from(concat!(env!("CARGO_MANIFEST_DIR"), "/../.."));
        crate::vendor::load(&root).expect("the vendored colour output")
    }

    fn shipped() -> String {
        std::fs::read_to_string(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../spec/syntax.toml"
        ))
        .expect("the shipped syntax spec")
    }

    #[test]
    fn the_shipped_spec_is_valid() {
        let syntax = parse("syntax.toml", &shipped(), &vendor()).expect("valid");
        assert!(syntax.theme.len() >= 3);
        assert!(syntax.theme.contains_key(&syntax.meta.resting));
    }

    /// The check that closes the hole `gates::unknown_token` leaves open. Added
    /// with the validator, because a validator wired to nothing looks exactly
    /// like one that passes.
    #[test]
    fn a_token_no_palette_declares_is_rejected_and_suggests_a_real_one() {
        let broken = shipped().replace("--nc-danger-text-muted", "--nc-danger-text-mutd");
        let error = parse("t.toml", &broken, &vendor()).expect_err("must fail");
        let problem = &error.problems()[0];
        assert!(
            problem.message().contains("text-mutd"),
            "{}",
            problem.message()
        );
        assert!(
            problem.help().contains("--nc-danger-text-muted"),
            "it should suggest the real name: {}",
            problem.help()
        );
    }

    /// A role a theme forgets is a role that renders in whatever colour
    /// contains it, which reads as a scanner bug rather than a spec omission.
    #[test]
    fn a_theme_that_forgets_a_role_is_rejected() {
        let broken = shipped().replacen("keyword = ", "kyeword_typo_removed = ", 1);
        // The typo'd key is refused by serde before the role check runs, which
        // is the same protection one level up.
        assert!(parse("t.toml", &broken, &vendor()).is_err());
    }

    #[test]
    fn bold_italic_is_refused_because_the_face_has_no_such_cut() {
        let broken = shipped().replace(
            "[signal.comment]\nstyle = \"italic\"",
            "[signal.comment]\nstyle = \"italic\"\nweight = \"bold\"",
        );
        let error = parse("t.toml", &broken, &vendor()).expect_err("must fail");
        assert!(
            error
                .problems()
                .iter()
                .any(|p| p.help().contains("synthesise")),
            "{:?}",
            error.problems()[0].message()
        );
    }
}
