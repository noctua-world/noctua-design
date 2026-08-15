//! The shape of `spec/lineage/cases.toml`: the expectation **both** layout
//! engines are held to.
//!
//! # Why this is a file and not a test
//!
//! There are two implementations of the `Lineage` layout —
//! `crates/noctua-design-lineage/src/layout.rs`, which runs during
//! pre-rendering, and `layout.js` beside it, which runs in a browser for the
//! React package. They must agree, and the obvious way to check that — lay the
//! same graph out with both and compare — **passes whenever they are both
//! wrong in the same way**. That is the failure that actually happens: one
//! shared misreading, transcribed twice.
//!
//! So neither is compared to the other. Both are compared to this, which
//! `noctua-design-emit` also writes out as `dist/lineage.cases.json` so a
//! Playwright spec can read it without a TOML parser. A disagreement names
//! which half moved.
//!
//! # What a case looks like
//!
//! ```toml
//! [[case]]
//! name = "a couple and their two children"
//! note = "The bracket every family tree is drawn with."
//! graph = """
//! figure ada
//! figure bob
//! mark   u0 kind=union
//! bond   ada u0 kind=union
//! bond   bob u0 kind=union
//! bond   u0 cara
//! """
//! places = ["ada 0.5 0 0", "u0 2 0 0", "bob 3.5 0 0", "cara 2 1 0"]
//! crossings = 0
//! ```
//!
//! `places` is exhaustive and exact, because approximate agreement between two
//! implementations is not agreement. Coordinates are in cells; both checkers
//! round to three decimals before comparing, so neither language's float
//! formatting is part of the contract.

use serde::Deserialize;

use crate::error::{Problem, SpecError};

/// Every blessed case.
#[derive(Debug, Clone, Default, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Cases {
    /// In file order, which is the order both checkers report in.
    #[serde(default)]
    pub case: Vec<Case>,
}

/// One graph, and what both engines must make of it.
#[derive(Debug, Clone, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Case {
    /// What it is called in a failure.
    pub name: String,
    /// Why it is here. A case whose purpose nobody wrote down is a case nobody
    /// dares delete and nobody dares change.
    pub note: String,
    /// The graph, in the language `parse` reads.
    pub graph: String,
    /// `flat` or `deep`. Empty is `flat`.
    #[serde(default)]
    pub view: String,
    /// `elbow` or `taut`. Empty is `elbow`.
    #[serde(default)]
    pub route: String,
    /// `touch` or `shy`. Empty is `touch`.
    #[serde(default)]
    pub reach: String,
    /// `<id> <x> <y> <z>`, one per piece, in the order the graph declares them.
    pub places: Vec<String>,
    /// `<bond> <x> <y> <z> <len> <axis>`, when the routing is what the case is
    /// about. Empty means only the count below is checked.
    #[serde(default)]
    pub segments: Vec<String>,
    /// How many runs the scene draws.
    #[serde(default)]
    pub runs: usize,
    /// How many pairs of connections cross.
    #[serde(default)]
    pub crossings: usize,
    /// How many lines or attributes the parser did not understand.
    #[serde(default)]
    pub skipped: usize,
    /// How many bonds were dropped to break a cycle.
    #[serde(default)]
    pub broken: usize,
}

/// The views a case may ask for.
const VIEWS: [&str; 2] = ["flat", "deep"];
/// The routes a case may ask for.
const ROUTES: [&str; 2] = ["elbow", "taut"];
/// The axes a segment may travel along.
const AXES: [&str; 4] = ["x", "y", "z", "free"];

/// Reads the cases.
///
/// # Errors
///
/// A missing file, or any problem in it.
pub fn load(path: impl AsRef<std::path::Path>) -> Result<Cases, SpecError> {
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

/// Parses and validates the cases.
///
/// # Errors
///
/// A malformed file, or a case that claims something unreadable.
pub fn parse(path: &str, text: &str) -> Result<Cases, SpecError> {
    let cases: Cases = toml::from_str(text).map_err(|error| {
        SpecError::new(
            path,
            text.to_owned(),
            vec![
                Problem::whole_file(format!("{path} does not parse: {error}"))
                    .fix("every case is a [[case]] table with name, note, graph and places."),
            ],
        )
    })?;
    let problems = check(&cases);
    if problems.is_empty() {
        Ok(cases)
    } else {
        Err(SpecError::new(path, text.to_owned(), problems))
    }
}

/// Everything that would make a case unable to fail for the right reason.
fn check(cases: &Cases) -> Vec<Problem> {
    let mut problems = Vec::new();
    if cases.case.is_empty() {
        problems.push(
            Problem::whole_file("there are no cases")
                .fix("an empty expectation makes both engines vacuously correct."),
        );
    }
    for (index, case) in cases.case.iter().enumerate() {
        let name = &case.name;
        if cases.case[..index].iter().any(|other| &other.name == name) {
            problems.push(
                Problem::whole_file(format!("two cases are called `{name}`"))
                    .fix("a failure names the case, so two of a name is one nobody can find."),
            );
        }
        if case.note.trim().is_empty() {
            problems.push(
                Problem::whole_file(format!("`{name}` says nothing about why it exists"))
                    .fix("write what it catches. A case nobody can justify is a case nobody dares change."),
            );
        }
        if case.places.is_empty() {
            problems.push(
                Problem::whole_file(format!("`{name}` expects no placement at all")).fix(
                    "`places` is exhaustive: every piece the graph declares, with its coordinates.",
                ),
            );
        }
        word(&mut problems, name, "view", &case.view, &VIEWS);
        word(&mut problems, name, "route", &case.route, &ROUTES);
        for place in &case.places {
            fields(&mut problems, name, "place", place, 4);
        }
        for segment in &case.segments {
            let axis = segment.split_whitespace().nth(5).unwrap_or("");
            if !AXES.contains(&axis) {
                problems.push(
                    Problem::whole_file(format!("`{name}` expects a run along `{axis}`"))
                        .fix("an axis is x, y, z or free."),
                );
                continue;
            }
            // A free run carries the rotation that turns its own `+x` onto the
            // direction it travels, and that rotation is the only floating-point
            // arithmetic in the whole engine — so it is the part that most needs
            // pinning across two languages.
            let wanted = if axis == "free" { 15 } else { 6 };
            fields(&mut problems, name, "segment", segment, wanted);
        }
    }
    problems
}

/// One of a closed set, or empty for the default.
fn word(problems: &mut Vec<Problem>, case: &str, key: &str, value: &str, allowed: &[&str]) {
    if !value.is_empty() && !allowed.contains(&value) {
        problems.push(
            Problem::whole_file(format!("`{case}` asks for `{key} = \"{value}\"`"))
                .fix(format!("the choices are {}.", allowed.join(", "))),
        );
    }
}

/// A row of whitespace-separated fields, with the count it must have.
fn fields(problems: &mut Vec<Problem>, case: &str, what: &str, row: &str, wanted: usize) {
    let found = row.split_whitespace().count();
    if found != wanted {
        problems.push(
            Problem::whole_file(format!("`{case}` has a {what} of {found} fields: `{row}`"))
                .fix(format!("a {what} is {wanted} fields, separated by spaces.")),
        );
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn shipped() -> String {
        let root = concat!(env!("CARGO_MANIFEST_DIR"), "/../../spec/lineage/cases.toml");
        std::fs::read_to_string(root).expect("the shipped cases")
    }

    #[test]
    fn the_shipped_cases_parse() {
        parse("spec/lineage/cases.toml", &shipped()).expect("the shipped cases");
    }

    #[test]
    fn a_case_with_no_places_is_refused() {
        // The one shape that would make both engines vacuously correct.
        let text = "[[case]]\nname = \"x\"\nnote = \"y\"\ngraph = \"figure a\"\nplaces = []\n";
        assert!(parse("test", text).is_err());
    }

    #[test]
    fn a_malformed_row_is_refused() {
        let text =
            "[[case]]\nname = \"x\"\nnote = \"y\"\ngraph = \"figure a\"\nplaces = [\"a 0 0\"]\n";
        assert!(parse("test", text).is_err());
    }
}
