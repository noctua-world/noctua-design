//! The `noctua-design` token specification format.
//!
//! One TOML file describes every non-colour token. This crate turns it into a
//! validated [`Spec`], or into a diagnostic that says exactly where the file is
//! wrong and what to do about it.
//!
//! # Design
//!
//! **Every table has a default**, so the smallest useful spec is empty and
//! nobody edits configuration to get a working build:
//!
//! ```
//! # fn main() -> Result<(), Box<dyn std::error::Error>> {
//! let spec = noctua_design_spec::parse("example.toml", "")?;
//! assert_eq!(spec.meta.prefix, "noctua");
//! # Ok(()) }
//! ```
//!
//! **Unknown keys are errors.** A misspelled key that silently does nothing is
//! far worse than a build that stops: the token keeps its old value and the
//! diff looks correct.
//!
//! **Every problem is reported at once**, each with a fix.
//!
//! This crate knows nothing about output formats. That is `noctua-design-emit`.

pub mod component;
pub mod error;
pub mod fonts;
pub mod i18n;
pub mod model;
pub mod theme;
mod validate;
mod validate_fonts;
mod validate_theme;
pub mod vendor;

pub use component::Component;
pub use error::{Problem, SpecError};
pub use fonts::{Face, Fonts, Manifest, Slice, Sliced};
pub use i18n::{Catalogue, Locale};
pub use model::{Duration, Meta, Motion, Space, Spec, Typography};
pub use theme::{Setting, Theme};
pub use vendor::Vendor;

/// Parses and validates a token specification.
///
/// `path` is used only for diagnostics; nothing is read from disk.
///
/// # Errors
///
/// Returns every syntax and semantic problem found, in one diagnostic.
pub fn parse(path: &str, text: &str) -> Result<Spec, SpecError> {
    let spec: Spec = match toml::from_str(text) {
        Ok(spec) => spec,
        Err(error) => {
            let problem = match error.span() {
                Some(span) => Problem::at(span, error.message().to_owned())
                    .labelled("invalid here")
                    .fix(explain(error.message())),
                None => Problem::whole_file(error.message().to_owned()),
            };
            return Err(SpecError::new(path, text.to_owned(), vec![problem]));
        }
    };

    let problems = validate::check(&spec);
    if problems.is_empty() {
        Ok(spec)
    } else {
        Err(SpecError::new(path, text.to_owned(), problems))
    }
}

/// Turns a serde message into something a person can act on.
///
/// serde reports an unknown key by listing every key that *was* expected, which
/// is genuinely useful, and reports a type mismatch by naming a Rust type the
/// reader has never seen, which is not. Only the second needs help.
fn explain(message: &str) -> String {
    if message.contains("unknown field") {
        return "check the spelling against the key list above; unknown keys are \
                rejected because one that silently did nothing would leave the \
                token at its old value with a diff that looks correct"
            .to_owned();
    }
    if message.contains("invalid type") {
        return "check the value's type — a length is a quoted string with a unit \
                (\"0.5rem\"), a line height is a bare number (1.6), a weight is a \
                bare integer (400)"
            .to_owned();
    }
    "check the key name and the value's type against spec/tokens.toml's comments".to_owned()
}

/// Parses and validates the font layer.
///
/// # Errors
///
/// Returns every problem found, in one diagnostic.
pub fn parse_fonts(path: &str, text: &str) -> Result<Fonts, SpecError> {
    let fonts: Fonts = match toml::from_str(text) {
        Ok(fonts) => fonts,
        Err(error) => {
            let problem = match error.span() {
                Some(span) => Problem::at(span, error.message().to_owned())
                    .labelled("invalid here")
                    .fix(explain(error.message())),
                None => Problem::whole_file(error.message().to_owned()),
            };
            return Err(SpecError::new(path, text.to_owned(), vec![problem]));
        }
    };

    let problems = validate_fonts::check(&fonts);
    if problems.is_empty() {
        Ok(fonts)
    } else {
        Err(SpecError::new(path, text.to_owned(), problems))
    }
}

/// Reads and parses the font layer from disk.
///
/// # Errors
///
/// Returns a diagnostic if the file cannot be read, or if it is invalid.
pub fn load_fonts(path: impl AsRef<std::path::Path>) -> Result<Fonts, SpecError> {
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
    parse_fonts(&display, &text)
}

/// Parses and validates the theme model against the vendored axes.
///
/// # Errors
///
/// Returns every problem found, in one diagnostic.
pub fn parse_theme(path: &str, text: &str, vendor: &Vendor) -> Result<Theme, SpecError> {
    let theme: Theme = match toml::from_str(text) {
        Ok(theme) => theme,
        Err(error) => {
            let problem = match error.span() {
                Some(span) => Problem::at(span, error.message().to_owned())
                    .labelled("invalid here")
                    .fix(explain(error.message())),
                None => Problem::whole_file(error.message().to_owned()),
            };
            return Err(SpecError::new(path, text.to_owned(), vec![problem]));
        }
    };

    let problems = validate_theme::check(&theme, vendor);
    if problems.is_empty() {
        Ok(theme)
    } else {
        Err(SpecError::new(path, text.to_owned(), problems))
    }
}

/// Reads and parses the theme model from disk.
///
/// # Errors
///
/// Returns a diagnostic if the file cannot be read, or if it is invalid.
pub fn load_theme(path: impl AsRef<std::path::Path>, vendor: &Vendor) -> Result<Theme, SpecError> {
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
    parse_theme(&display, &text, vendor)
}

/// Reads and parses a token specification from disk.
///
/// # Errors
///
/// Returns a diagnostic if the file cannot be read, or if it is invalid.
pub fn load(path: impl AsRef<std::path::Path>) -> Result<Spec, SpecError> {
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

    parse(&display, &text)
}

#[cfg(test)]
mod tests {
    /// The shipped spec must parse. If it does not, nothing else in this
    /// repository can be trusted.
    #[test]
    fn the_shipped_spec_is_valid() {
        let path = concat!(env!("CARGO_MANIFEST_DIR"), "/../../spec/tokens.toml");
        let spec = super::load(path).expect("the shipped spec parses");
        assert_eq!(spec.meta.prefix, "noctua");
        assert!(!spec.space.steps.is_empty());
        assert!(spec.r#type.measure.is_some());
    }

    /// A misspelled key must stop the build rather than silently doing nothing.
    #[test]
    fn an_unknown_key_is_rejected_and_says_where() {
        let error = super::parse("t.toml", "[space]\nbaze = \"0.25rem\"\n").expect_err("must fail");
        let problem = &error.problems()[0];
        assert!(problem.message().contains("baze"), "{}", problem.message());
        assert!(
            problem.span().is_some(),
            "a parse error must point at the offending bytes"
        );
        assert!(!problem.help().is_empty(), "and must say what to do");
    }

    /// A type mismatch must not report a Rust type name to a spec author.
    #[test]
    fn a_type_mismatch_is_explained_in_terms_of_the_format() {
        let error =
            super::parse("t.toml", "[type.leading]\nbody = \"1.6\"\n").expect_err("must fail");
        let problem = &error.problems()[0];
        assert!(
            !problem.help().contains("f64"),
            "the help names a Rust type: {}",
            problem.help()
        );
        assert!(problem.help().contains("line height"), "{}", problem.help());
    }

    /// A length without a unit is invalid CSS that the browser drops silently.
    #[test]
    fn a_length_without_a_unit_is_rejected() {
        for bad in ["[radius]\ncontrol = \"8\"\n", "[space]\nbase = \"4\"\n"] {
            let error = super::parse("t.toml", bad).expect_err("must fail");
            assert!(
                error.problems().iter().any(|p| p.help().contains("unit")),
                "{bad:?} was rejected, but not for the reason we meant"
            );
        }
        // A bare zero is the one legal exception, and must survive.
        super::parse("t.toml", "[radius]\nnone = \"0\"\n").expect("bare zero is valid CSS");
    }

    /// A named purpose pointing at a step that does not exist would emit a
    /// token multiplied by nothing.
    #[test]
    fn a_named_space_must_point_at_a_declared_step() {
        let error = super::parse(
            "t.toml",
            "[space]\nsteps = [0, 1, 2, 4]\n[space.named]\ngap = 3\n",
        )
        .expect_err("must fail");
        let problem = &error.problems()[0];
        assert!(problem.message().contains("gap"), "{}", problem.message());
        assert!(problem.message().contains('3'), "{}", problem.message());
        assert!(
            problem.help().contains("nearest declared step is 4")
                || problem.help().contains("nearest declared step is 2"),
            "it should suggest a numerically near step: {}",
            problem.help()
        );
    }

    /// Sharing `noctua-colors`' namespace would let a design token shadow a
    /// colour token of the same name.
    #[test]
    fn the_colours_namespace_is_refused() {
        let error = super::parse("t.toml", "[meta]\nprefix = \"nc\"\n").expect_err("must fail");
        assert!(error.problems()[0].message().contains("noctua-colors"));
    }

    /// A duration without a unit is not a duration.
    #[test]
    fn a_duration_must_carry_a_unit() {
        let error = super::parse(
            "t.toml",
            "[motion.duration.base]\nnormal = \"180\"\nreduced = \"0ms\"\n",
        )
        .expect_err("must fail");
        assert!(error.problems()[0].message().contains("not a duration"));
    }

    /// Overlapping lines are legal CSS, so nothing downstream would object.
    #[test]
    fn a_line_height_below_one_is_rejected() {
        let error = super::parse("t.toml", "[type.leading]\nbody = 0.8\n").expect_err("must fail");
        assert!(error.problems()[0].help().contains("overlap"));
    }

    /// Every problem in one pass, not one per edit.
    #[test]
    fn every_problem_is_reported_at_once() {
        let error = super::parse(
            "t.toml",
            "[radius]\na = \"8\"\nb = \"9\"\n[type.leading]\nc = 0.5\n",
        )
        .expect_err("must fail");
        assert!(
            error.problems().len() >= 3,
            "expected every problem, got {}",
            error.problems().len()
        );
    }
}
