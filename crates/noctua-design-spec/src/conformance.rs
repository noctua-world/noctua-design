//! The behavioural cases both implementations answer to — `NOCTUA.md §9.3` Leg 3.
//!
//! # What a case is
//!
//! Given props and a sequence of interactions, what the DOM has to look like
//! afterwards. One file per component, `spec/components/<name>.cases.toml`, and
//! **one runner drives both implementations against it in a real browser**. Two
//! halves that agree with each other and disagree with the specification is the
//! failure that actually happens — which is why neither is checked against the
//! other alone.
//!
//! # Why the vocabulary is small on purpose
//!
//! `focus`, `click`, `key:` and `type:` and nothing else. A case language that
//! can express anything becomes a second test framework, written by people who
//! were not trying to write one, and the cases stop being readable as a
//! specification. Anything a case cannot say is a browser test in
//! `tests/browser/`, where saying it is somebody's job.

use std::collections::BTreeMap;

use serde::Deserialize;

use crate::error::{Problem, SpecError};

/// A prop value as a case writes it.
///
/// Re-exported so the emitters can read one without taking a TOML dependency
/// of their own — they format a validated model and parse nothing, which is
/// the rule `noctua-design-emit`'s own header states.
pub use toml::Value as PropValue;

/// Every case one component answers to.
#[derive(Debug, Clone, Default, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Cases {
    /// In file order, which is the order failures are reported in.
    #[serde(default)]
    pub case: Vec<Case>,
}

/// One arrangement, one sequence of interactions, one expectation.
#[derive(Debug, Clone, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Case {
    /// What it is called in a failure. A sentence stating the property.
    pub name: String,
    /// Why it is here. A case whose purpose nobody wrote down is a case nobody
    /// dares delete and nobody dares change — `spec/lineage/cases.toml` says
    /// the same thing about itself.
    pub note: String,
    /// What the component is called with. Every name has to be a prop the
    /// contract declares; a case cannot invent one.
    #[serde(default)]
    pub props: BTreeMap<String, toml::Value>,
    /// What is inside it, as plain text. For the components that take children.
    #[serde(default)]
    pub children: String,
    /// What is inside it, when what is inside is more components.
    ///
    /// Half the catalogue is a composition — a `Segmented` is its items, a
    /// `Tabs` is its tabs and panels — and a case that could only set props
    /// could say nothing about any of them. Written as `[[case.child]]`, which
    /// is TOML's own shape for a list of tables.
    #[serde(default)]
    pub child: Vec<Child>,
    /// What happens to it, in order. See [`STEPS`].
    #[serde(default)]
    pub steps: Vec<String>,
    /// What has to be true afterwards.
    #[serde(default)]
    pub expect: Expect,
}

/// One component inside a case.
#[derive(Debug, Clone, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Child {
    /// The catalogue's spelling: `SegmentedItem`, `Tab`, `TableCell`.
    pub component: String,
    /// What it is called with.
    #[serde(default)]
    pub props: BTreeMap<String, PropValue>,
    /// Plain text inside it.
    #[serde(default)]
    pub text: String,
    /// And components inside that, for the arrangements that go three deep.
    #[serde(default)]
    pub child: Vec<Child>,
}

/// What a case asserts once its steps have run.
#[derive(Debug, Clone, Default, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Expect {
    /// Where the focus is: `root`, `none`, or a selector inside the case.
    #[serde(default)]
    pub focus: String,
    /// Attributes on the root, by value. `""` means *present and empty*.
    #[serde(default)]
    pub attrs: BTreeMap<String, String>,
    /// Attributes that must NOT be on the root.
    ///
    /// A list rather than a `null` in `attrs`, because TOML has no null — and
    /// the assertion is worth its own name anyway: **a false ARIA state is
    /// written, not absent**, and an expectation that can only say *equals*
    /// cannot catch one.
    #[serde(default)]
    pub absent: Vec<String>,
    /// Selectors that must exist and be visible, relative to the case root.
    #[serde(default)]
    pub visible: Vec<String>,
    /// Selectors that must exist, whether or not they are rendered.
    ///
    /// Not the same as `visible`, and not decoration: a `Tooltip`'s bubble
    /// ships `hidden` and carries the id the control will be pointed at, so
    /// *it is there* is the whole assertion — and `hidden` alone would pass on
    /// a bubble that does not exist at all.
    #[serde(default)]
    pub present: Vec<String>,
    /// Selectors that must be absent, or present and not rendered.
    #[serde(default)]
    pub hidden: Vec<String>,
    /// The root's text content, with runs of whitespace collapsed.
    #[serde(default)]
    pub text: String,
    /// What the case's control holds — its `value` PROPERTY, not its attribute.
    ///
    /// The two part company the moment anything types or clears: the attribute
    /// is what the markup shipped and the property is what the reader has. A
    /// case about `Escape` emptying a box is a case about the property, and a
    /// selector like `input[value='']` would go on matching the old markup and
    /// pass while the box stayed full.
    #[serde(default)]
    pub value: Option<String>,
}

/// The whole step vocabulary, with what each one means.
pub const STEPS: &[(&str, &str)] = &[
    ("focus", "put the keyboard on the root"),
    ("focus:<selector>", "put it on something inside"),
    ("click", "press the root with a pointer"),
    ("click:<selector>", "press something inside"),
    (
        "key:<Key>",
        "a key press where the focus is, named as the browser names it: Space, Enter, Escape, Tab, ArrowDown, Home",
    ),
    ("type:<text>", "typing, one character at a time"),
    ("blur", "take the keyboard away"),
];

/// Reads one component's cases.
///
/// # Errors
///
/// A malformed file, a step outside the vocabulary, a case that asserts
/// nothing, or two cases with the same name.
pub fn parse(path: &str, text: &str) -> Result<Cases, SpecError> {
    let cases: Cases = toml::from_str(text).map_err(|error| {
        SpecError::new(
            path,
            text.to_owned(),
            vec![
                Problem::whole_file(format!("{path} does not parse: {error}")).fix(
                    "every case is a [[case]] table with name, note, and at least one of steps \
                     or expect.",
                ),
            ],
        )
    })?;
    let problems = check(path, &cases);
    if problems.is_empty() {
        Ok(cases)
    } else {
        Err(SpecError::new(path, text.to_owned(), problems))
    }
}

/// Everything that would stop a case failing for the right reason.
fn check(path: &str, cases: &Cases) -> Vec<Problem> {
    let mut problems = Vec::new();
    if cases.case.is_empty() {
        problems.push(
            Problem::whole_file(format!("{path} declares no case"))
                .fix("delete the file, or write the case it was going to hold."),
        );
    }
    let mut seen: Vec<&str> = Vec::new();
    for case in &cases.case {
        if seen.contains(&case.name.as_str()) {
            problems.push(
                Problem::whole_file(format!("two cases are called `{}`", case.name)).fix(
                    "a failure names the case, so two with one name is a failure that names \
                     neither.",
                ),
            );
        }
        seen.push(&case.name);

        if case.note.trim().is_empty() {
            problems.push(
                Problem::whole_file(format!("`{}` says nothing about why it exists", case.name))
                    .fix("write what breaking it would mean. `note` is not decoration."),
            );
        }

        if case.expect.focus.is_empty()
            && case.expect.attrs.is_empty()
            && case.expect.absent.is_empty()
            && case.expect.visible.is_empty()
            && case.expect.present.is_empty()
            && case.expect.hidden.is_empty()
            && case.expect.text.is_empty()
            && case.expect.value.is_none()
        {
            problems.push(
                Problem::whole_file(format!("`{}` asserts nothing", case.name)).fix(
                    "a case with no expectation passes whatever the component does, which is \
                     the vacuous assertion `NOCTUA.md §19.5` is about.",
                ),
            );
        }

        for step in &case.steps {
            if !understood(step) {
                let known = STEPS
                    .iter()
                    .map(|(shape, _)| *shape)
                    .collect::<Vec<_>>()
                    .join(", ");
                problems.push(
                    Problem::whole_file(format!("`{}`: no step is written `{step}`", case.name))
                        .fix(format!("the vocabulary is deliberately small: {known}.")),
                );
            }
        }
    }
    problems
}

/// Whether a step is one the runner can carry out.
#[must_use]
pub fn understood(step: &str) -> bool {
    match step.split_once(':') {
        None => matches!(step, "focus" | "click" | "blur"),
        Some((head, rest)) => {
            // Not trimmed: `type: ` types a space, which is a real thing to
            // type. A key, by contrast, is NAMED — `key:Space`, never `key: `.
            !rest.is_empty()
                && matches!(head, "focus" | "click" | "key" | "type")
                && (head != "key" || rest.chars().all(|c| c.is_ascii_alphanumeric()))
        }
    }
}

/// Reads every component's cases, by the file beside its contract.
///
/// A component with no cases file is not an error here — [`owed`] is what
/// decides whether it should have had one.
///
/// # Errors
///
/// The first file that does not read, with its own problems attached.
pub fn load_all(
    dir: &std::path::Path,
    components: &[crate::component::Component],
) -> Result<Vec<(String, Cases)>, SpecError> {
    let mut out = Vec::new();
    for component in components {
        let stem = crate::component::file_stem(&component.meta.name);
        let path = dir.join(format!("{stem}.cases.toml"));
        let Ok(text) = std::fs::read_to_string(&path) else {
            continue;
        };
        let shown = format!("spec/components/{stem}.cases.toml");
        let cases = parse(&shown, &text)?;
        let problems = against(component, &cases);
        if !problems.is_empty() {
            return Err(SpecError::new(&shown, text, problems));
        }
        out.push((component.meta.name.clone(), cases));
    }
    Ok(out)
}

/// Whether a component owes cases at all.
///
/// **A contract that declares a keyboard or a state is a contract that promises
/// behaviour**, and behaviour is what a case can hold to account. A `Divider`
/// declares neither and owes nothing: there is nothing to drive.
#[must_use]
pub fn owed(component: &crate::component::Component) -> bool {
    !component.keyboard.is_empty() || !component.state.is_empty()
}

/// Whether the cases only name props the contract declares.
fn against(component: &crate::component::Component, cases: &Cases) -> Vec<Problem> {
    let mut problems = Vec::new();
    for case in &cases.case {
        for name in case.props.keys() {
            if !component.prop.contains_key(name) {
                problems.push(
                    Problem::whole_file(format!(
                        "`{}` sets `{name}`, which {} does not declare",
                        case.name, component.meta.name
                    ))
                    .fix(
                        "a case calls the component the way a consumer does, so a prop that \
                         does not exist is a case that could never run.",
                    ),
                );
            }
        }
        if !case.children.is_empty()
            && !component.prop.values().any(|prop| {
                format!("{:?}", prop.kind)
                    .to_lowercase()
                    .contains("children")
            })
        {
            problems.push(
                Problem::whole_file(format!(
                    "`{}` puts children inside {}, which takes none",
                    case.name, component.meta.name
                ))
                .fix("the contract says what goes inside; a case cannot widen it."),
            );
        }
    }
    problems
}

#[cfg(test)]
mod tests {
    use super::*;

    const ONE: &str = r#"
[[case]]
name = "space activates when enabled"
note = "The key a button owes, which is the one an anchor does not."
props = { label = "Save" }
steps = ["focus", "key:Space"]
expect = { focus = "root", attrs = { "aria-disabled" = "false" } }
"#;

    #[test]
    fn a_case_reads_back_what_it_declares() {
        let cases = parse("x.toml", ONE).expect("parses");
        assert_eq!(cases.case.len(), 1);
        assert_eq!(cases.case[0].steps, ["focus", "key:Space"]);
        assert_eq!(
            cases.case[0].expect.attrs.get("aria-disabled"),
            Some(&"false".to_owned())
        );
    }

    /// A false ARIA state is written rather than absent, and an expectation
    /// that can only say "equals" cannot catch one.
    #[test]
    fn an_attribute_can_be_required_to_be_absent() {
        let text = "[[case]]\nname = \"n\"\nnote = \"w\"\n\
                    expect = { absent = [\"aria-checked\"] }\n";
        let cases = parse("x.toml", text).expect("parses");
        assert_eq!(cases.case[0].expect.absent, ["aria-checked"]);
    }

    /// A key is named, so `key: ` is a typo for `key:Space` rather than a way
    /// of pressing the space bar.
    #[test]
    fn a_key_is_named_rather_than_typed() {
        assert!(understood("key:Space"));
        assert!(!understood("key: "));
        assert!(understood("type: "));
    }

    #[test]
    fn a_case_that_asserts_nothing_is_refused() {
        let text = "[[case]]\nname = \"n\"\nnote = \"w\"\nsteps = [\"click\"]\n";
        let error = parse("x.toml", text).expect_err("refused");
        assert!(format!("{error:?}").contains("asserts nothing"));
    }

    #[test]
    fn a_step_outside_the_vocabulary_is_refused() {
        let text = "[[case]]\nname = \"n\"\nnote = \"w\"\nsteps = [\"scroll\"]\n\
                    expect = { focus = \"root\" }\n";
        let error = parse("x.toml", text).expect_err("refused");
        assert!(format!("{error:?}").contains("no step is written"));
    }

    #[test]
    fn two_cases_with_one_name_are_refused() {
        let text = "[[case]]\nname = \"n\"\nnote = \"w\"\nexpect = { focus = \"root\" }\n\
                    [[case]]\nname = \"n\"\nnote = \"w\"\nexpect = { focus = \"root\" }\n";
        let error = parse("x.toml", text).expect_err("refused");
        assert!(format!("{error:?}").contains("two cases are called"));
    }
}
