//! What `noctua-colors` vendored into this repository, read and validated.
//!
//! This module exists because of one property of the colour system that is
//! invisible until you try to theme a *subtree*.
//!
//! # The problem
//!
//! The semantic contract — `--nc-color-surface`, `-fg`, `-border`, and about
//! 1,767 more — is emitted **once**, at `:where(:root)`. Every declaration is a
//! pure indirection: `--nc-color-surface: var(--nc-neutral-bg-app)`.
//!
//! A `var()` is substituted at the element where the property is **declared**,
//! and the result inherits as a resolved value. So a subtree carrying
//! `data-palette="blue-vivid"` gets new `--nc-neutral-*` values — and keeps the
//! **root's** `--nc-color-surface`, because that is where the indirection was
//! resolved.
//!
//! A component painted from the semantic contract therefore ignores its own
//! theme, silently, and looks like a component whose setting does not work.
//!
//! # The fix, and why it is an extraction
//!
//! The contract's body is re-emitted under a selector list that also matches
//! scopes (see `noctua_design_emit::scope`). Extracting it rather than
//! restating it means there is exactly one source: a name added upstream
//! appears here on the next `cargo xtask export`, and a name that changes shape
//! fails the build rather than drifting.
//!
//! Everything below is therefore a **parser with opinions**. It refuses
//! anything it does not recognise, because the alternative — quietly emitting
//! whatever it found — would let a colour literal into a file this repository
//! generates, and this repository defines no colours.

use std::collections::BTreeMap;
use std::path::{Path, PathBuf};

use crate::error::{Problem, SpecError};

/// Where the vendored output lives, relative to the repository root.
pub const VENDOR: &str = "packages/tokens/vendor";

/// The vendored `noctua-colors` output, validated.
#[derive(Debug, Clone)]
pub struct Vendor {
    /// Every `--nc-color-*` declaration, in the order the vendor wrote them.
    ///
    /// Held as `(name, value)` rather than as raw text so the emitter formats
    /// them and a test can assert the values are all `var()` references.
    pub contract: Vec<(String, String)>,

    /// The accent hues offered, in spec order.
    pub accents: Vec<String>,

    /// The saturations offered, in spec order.
    pub saturations: Vec<String>,

    /// Every palette name, in spec order.
    pub themes: Vec<String>,

    /// The default palette — the one bound to `:root`.
    pub default_theme: String,

    /// `accent/saturation` to palette name.
    pub grid: BTreeMap<String, String>,
}

impl Vendor {
    /// Whether a name is a palette this vendor ships.
    #[must_use]
    pub fn has_theme(&self, name: &str) -> bool {
        self.themes.iter().any(|theme| theme == name)
    }

    /// The stylesheet that binds `name` to `[data-palette="name"]`.
    ///
    /// Every palette has one, including the default — `noctua-colors` 0.2.1
    /// added the last of them. Before that the default was `:root`-only, which
    /// made it the one palette of thirty-nine that could not be scoped.
    #[must_use]
    pub fn scopable_stylesheet(name: &str) -> String {
        format!("theme-{name}.css")
    }
}

/// Reads and validates everything this repository consumes from the vendor.
///
/// # Errors
///
/// Every problem found, in one diagnostic, each naming the file and the line.
pub fn load(root: &Path) -> Result<Vendor, SpecError> {
    let vendor = root.join(VENDOR);
    let mut problems = Vec::new();

    let contract = match contract(&vendor) {
        Ok(contract) => contract,
        Err(found) => {
            problems.extend(found);
            Vec::new()
        }
    };

    let axes = match axes(&vendor) {
        Ok(axes) => axes,
        Err(found) => {
            problems.extend(found);
            Axes::default()
        }
    };

    // Every palette must have a scopable stylesheet. This is the check that
    // found the missing thirty-ninth file: thirty-eight palettes worked on a
    // subtree and the default did not, which is exactly the shape of gap that
    // survives review.
    for theme in &axes.themes {
        let path = vendor.join("css").join(Vendor::scopable_stylesheet(theme));
        if !path.exists() {
            problems.push(
                Problem::whole_file(format!(
                    "the vendor has no {} for palette `{theme}`",
                    Vendor::scopable_stylesheet(theme)
                ))
                .fix(
                    "that palette cannot be applied to a subtree — the attribute would \
                     match nothing and the element would inherit the page's palette. \
                     Re-export from noctua-colors 0.2.1 or later",
                ),
            );
        }
    }

    if problems.is_empty() {
        Ok(Vendor {
            contract,
            accents: axes.accents,
            saturations: axes.saturations,
            themes: axes.themes,
            default_theme: axes.default_theme,
            grid: axes.grid,
        })
    } else {
        Err(SpecError::new(
            &format!("{VENDOR}/"),
            String::new(),
            problems,
        ))
    }
}

/// Extracts the semantic contract from the vendored `contexts.css`.
///
/// The shape it requires is asserted rather than assumed, and every failure
/// names the line, because a vendor refresh that changes the shape must produce
/// a diagnostic rather than a subtly wrong generated file.
fn contract(vendor: &Path) -> Result<Vec<(String, String)>, Vec<Problem>> {
    let path = vendor.join("css/contexts.css");
    let display = format!("{VENDOR}/css/contexts.css");

    let Ok(text) = std::fs::read_to_string(&path) else {
        return Err(vec![
            Problem::whole_file(format!("could not read {display}"))
                .fix("run `cargo xtask export` in noctua-colors to vendor the colour tokens"),
        ]);
    };

    let mut problems = Vec::new();
    let mut declarations = Vec::new();
    let mut inside = false;
    let mut heads = 0usize;

    for (index, line) in text.lines().enumerate() {
        let number = index + 1;
        let trimmed = line.trim();

        if !inside {
            if trimmed.ends_with('{') {
                heads += 1;
                // The one selector this extraction is built around. Anything
                // else means the vendor changed shape and the emitted file
                // would be wrong in a way nothing downstream would notice.
                if trimmed != ":where(:root) {" {
                    problems.push(
                        Problem::whole_file(format!(
                            "{display}:{number} opens `{trimmed}`, not `:where(:root) {{`"
                        ))
                        .fix(
                            "this repository re-emits that rule's body under a scope-aware \
                             selector, so its shape is load-bearing. If noctua-colors changed \
                             it deliberately, crates/noctua-design-spec/src/vendor.rs has to \
                             change with it",
                        ),
                    );
                }
                inside = true;
            }
            continue;
        }

        if trimmed == "}" {
            inside = false;
            continue;
        }
        if trimmed.is_empty() {
            continue;
        }

        match declaration(trimmed) {
            Some(pair) => declarations.push(pair),
            None => problems.push(
                Problem::whole_file(format!("{display}:{number} is `{trimmed}`")).fix(
                    "every line of the semantic contract must be \
                     `--nc-color-<name>: var(--nc-<target>);`. A value that is not a var() \
                     reference would put a literal colour into a file this repository \
                     generates, and this repository defines no colours",
                ),
            ),
        }
    }

    if heads != 1 {
        problems.push(
            Problem::whole_file(format!("{display} has {heads} rules, not 1"))
                .fix("the extraction assumes one rule whose body is the whole contract"),
        );
    }
    if declarations.is_empty() && problems.is_empty() {
        problems.push(
            Problem::whole_file(format!("{display} declares nothing"))
                .fix("an empty contract would emit an empty scope stylesheet, silently"),
        );
    }

    if problems.is_empty() {
        Ok(declarations)
    } else {
        Err(problems)
    }
}

/// Parses `  --nc-color-x: var(--nc-y);` into its two halves.
///
/// Deliberately strict: the value must be exactly one `var()` reference and
/// nothing else. A `var(--nc-x, #fff)` fallback would smuggle a literal past
/// the source gate, because the gate does not read generated files.
fn declaration(line: &str) -> Option<(String, String)> {
    let (name, value) = line.strip_suffix(';')?.split_once(':')?;
    let name = name.trim();
    let value = value.trim();

    if !name.starts_with("--nc-color-") || !usable(name.trim_start_matches("--")) {
        return None;
    }
    let inner = value.strip_prefix("var(")?.strip_suffix(')')?;
    if !inner.starts_with("--nc-") || !usable(inner.trim_start_matches("--")) {
        return None;
    }
    Some((name.to_owned(), value.to_owned()))
}

fn usable(name: &str) -> bool {
    !name.is_empty()
        && name
            .chars()
            .all(|c| c.is_ascii_lowercase() || c.is_ascii_digit() || c == '-')
}

/// The accent × saturation grid, read rather than restated.
#[derive(Debug, Default)]
struct Axes {
    accents: Vec<String>,
    saturations: Vec<String>,
    themes: Vec<String>,
    default_theme: String,
    grid: BTreeMap<String, String>,
}

/// Reads `axes.json`.
///
/// The palette list is **never** written down in this repository's own spec.
/// A count or a name copied here would rot the first time upstream added a
/// palette, and the failure would be a palette silently missing from a picker.
fn axes(vendor: &Path) -> Result<Axes, Vec<Problem>> {
    let display = format!("{VENDOR}/json/axes.json");
    let Ok(text) = std::fs::read_to_string(vendor.join("json/axes.json")) else {
        return Err(vec![
            Problem::whole_file(format!("could not read {display}"))
                .fix("run `cargo xtask export` in noctua-colors"),
        ]);
    };

    let parsed: serde_json::Value = match serde_json::from_str(&text) {
        Ok(value) => value,
        Err(error) => {
            return Err(vec![
                Problem::whole_file(format!("{display} is not valid JSON: {error}"))
                    .fix("it is generated by noctua-colors; re-export rather than editing it"),
            ]);
        }
    };

    let list = |key: &str| -> Vec<String> {
        parsed[key]
            .as_array()
            .map(|values| {
                values
                    .iter()
                    .filter_map(|v| v.as_str().map(str::to_owned))
                    .collect()
            })
            .unwrap_or_default()
    };

    let accents = list("accents");
    let saturations = list("saturations");

    // `themes` is the grid itself — an object keyed `accent/saturation`, not a
    // list. Read it as what it is, and derive the palette list from it in the
    // vendor's own order, so the two can never disagree about which palettes
    // exist.
    let mut grid = BTreeMap::new();
    let mut themes = Vec::new();
    if let Some(map) = parsed["themes"].as_object() {
        for (key, value) in map {
            if let Some(name) = value.as_str() {
                grid.insert(key.clone(), name.to_owned());
                if !themes.iter().any(|t| t == name) {
                    themes.push(name.to_owned());
                }
            }
        }
    }

    let mut problems = Vec::new();
    for (label, values) in [
        ("accents", &accents),
        ("saturations", &saturations),
        ("themes", &themes),
    ] {
        if values.is_empty() {
            problems.push(
                Problem::whole_file(format!("{display} has no `{label}`"))
                    .fix("the grid drives every palette control; an empty axis is unusable"),
            );
        }
    }

    // The default is the first accent at the first saturation — the pair
    // `noctua-colors` binds to `:root`. Derived from the axes rather than
    // written down: two places to state it is one place to get it wrong.
    //
    // A BTreeMap orders keys alphabetically, so `themes.first()` would be
    // `amber-balanced`, not the default. Look the pair up instead.
    let default_theme = accents
        .first()
        .zip(saturations.first())
        .and_then(|(a, s)| grid.get(&format!("{a}/{s}")))
        .cloned()
        .unwrap_or_default();

    if problems.is_empty() {
        Ok(Axes {
            accents,
            saturations,
            themes,
            default_theme,
            grid,
        })
    } else {
        Err(problems)
    }
}

/// The repository root, for callers that have a crate directory.
#[must_use]
pub fn repository_root() -> PathBuf {
    PathBuf::from(concat!(env!("CARGO_MANIFEST_DIR"), "/../.."))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn vendor() -> Vendor {
        load(&repository_root()).expect("the vendored output is valid")
    }

    #[test]
    fn the_shipped_vendor_is_valid() {
        let vendor = vendor();
        assert!(!vendor.contract.is_empty());
        assert_eq!(
            vendor.themes.len(),
            vendor.accents.len() * vendor.saturations.len()
        );
        assert_eq!(vendor.default_theme, "ochre-balanced");
    }

    /// Every extracted value is a `var()` reference — so no colour literal can
    /// reach a file this repository generates by this path.
    #[test]
    fn every_extracted_value_is_a_var_reference() {
        for (name, value) in vendor().contract {
            assert!(name.starts_with("--nc-color-"), "{name}");
            assert!(
                value.starts_with("var(--nc-") && value.ends_with(')'),
                "{name} is `{value}`, which is not a plain var() reference"
            );
        }
    }

    /// **The check that found the missing thirty-ninth file.**
    #[test]
    fn every_palette_has_a_scopable_stylesheet() {
        let vendor = vendor();
        let css = repository_root().join(VENDOR).join("css");
        for theme in &vendor.themes {
            let path = css.join(Vendor::scopable_stylesheet(theme));
            assert!(path.exists(), "{theme} cannot be applied to a subtree");
        }
    }

    /// The contract must be emitted once and only at `:where(:root)`. If a
    /// vendored theme file ever declares a `--nc-color-*`, our single-selector
    /// re-emission would out-specify it at a scope element.
    #[test]
    fn no_vendored_theme_declares_the_semantic_contract() {
        let css = repository_root().join(VENDOR).join("css");
        for entry in std::fs::read_dir(&css).expect("vendor css") {
            let path = entry.expect("entry").path();
            let name = path.file_name().unwrap_or_default().to_string_lossy();
            if !name.starts_with("theme-") {
                continue;
            }
            let text = std::fs::read_to_string(&path).expect("readable");
            assert!(
                !text.contains("--nc-color-"),
                "{name} declares a semantic token. The scope stylesheet re-emits the \
                 contract under one selector, which would now out-specify this file \
                 inside a scope"
            );
        }
    }

    /// A malformed contract must fail with the line, not silently truncate.
    #[test]
    fn a_value_that_is_not_a_var_reference_is_rejected() {
        assert!(declaration("  --nc-color-surface: var(--nc-neutral-bg-app);").is_some());
        for bad in [
            "  --nc-color-surface: #ffffff;",
            "  --nc-color-surface: var(--nc-neutral-bg-app, #fff);",
            "  --nc-color-surface: oklch(1 0 0);",
            "  --other: var(--nc-neutral-bg-app);",
        ] {
            assert!(declaration(bad).is_none(), "{bad} should be rejected");
        }
    }

    /// The grid must resolve every pair, or a picker can produce a name that
    /// has no stylesheet.
    #[test]
    fn the_grid_covers_every_pair() {
        let vendor = vendor();
        for accent in &vendor.accents {
            for saturation in &vendor.saturations {
                let key = format!("{accent}/{saturation}");
                let name = vendor
                    .grid
                    .get(&key)
                    .unwrap_or_else(|| panic!("{key} is not in the grid"));
                assert!(
                    vendor.has_theme(name),
                    "{key} maps to unknown palette {name}"
                );
            }
        }
    }
}
