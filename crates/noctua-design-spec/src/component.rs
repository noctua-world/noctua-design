//! The shape of `spec/components/<name>.toml`.
//!
//! One file per component, and it is the **only** place a prop is declared. Both
//! implementations are generated from it, so "the React one gained a prop the
//! Dioxus one has not" stops being a thing review has to catch and becomes a
//! thing that cannot be expressed.
//!
//! # Why the contract lives here and not in either implementation
//!
//! `NOCTUA.md §9.3`: a component is four things that change together — this
//! file, the two implementations, and the shared behavioural cases. Three of the
//! four are generated from this one. Editing an implementation by hand is either
//! overwritten by the next build or caught by the drift check, which is what
//! makes the parity claim mechanical rather than reviewed.

use indexmap::IndexMap;
use serde::Deserialize;

use crate::error::{Problem, SpecError};

/// One component's whole contract.
#[derive(Debug, Clone, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Component {
    /// Identity and defaults.
    pub meta: Meta,

    /// Every prop, in declaration order.
    #[serde(default)]
    pub prop: IndexMap<String, Prop>,

    /// What the component promises assistive technology.
    #[serde(default)]
    pub a11y: A11y,

    /// The states it can be in that are not props — what the DOM shows, and
    /// what a reader is meant to understand by it.
    ///
    /// A *variant* needs no table of its own: a variant IS an enum prop
    /// carrying an `attribute`, and inventing a second way to say that would
    /// give two places to look for the same fact.
    #[serde(default)]
    pub state: IndexMap<String, State>,

    /// What each key does, when the component answers the keyboard.
    ///
    /// `NOCTUA.md §9.3` Leg 2 requires this to be declared rather than
    /// described: a keyboard contract in prose is a hope, and the conformance
    /// suite cannot assert a hope.
    #[serde(default)]
    pub keyboard: IndexMap<String, String>,

    /// When to reach for it, and when not to — one paragraph, in English.
    ///
    /// Developer-facing reference about an API, like `meta.summary`. The
    /// reader-facing lead on the page around it is translated.
    #[serde(default)]
    pub guidance: String,
}

/// One state a component can be in.
#[derive(Debug, Clone, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct State {
    /// The DOM attribute that carries it.
    pub attribute: String,
    /// What it means, and what a reader sees.
    pub doc: String,
}

impl Component {
    /// The props that are not `children`, in declaration order.
    pub fn attributes(&self) -> impl Iterator<Item = (&String, &Prop)> {
        self.prop.iter().filter(|(_, p)| p.kind != Kind::Children)
    }

    /// Whether the component takes children.
    #[must_use]
    pub fn takes_children(&self) -> bool {
        self.prop.values().any(|p| p.kind == Kind::Children)
    }
}

/// Identity and rendering defaults.
#[derive(Debug, Clone, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Meta {
    /// The component's name, in `PascalCase`. Both packages export this.
    pub name: String,

    /// The HTML element it renders by default.
    pub element: String,

    /// The class its root carries, or none.
    #[serde(default)]
    pub class: Option<String>,

    /// What a component's page puts inside it, when a pangram would be wrong.
    ///
    /// A button holding "Sphinx of black quartz, judge my vow" demonstrates
    /// nothing about the button. Blank means the page picks: a sentence for
    /// block elements, the component's own name for inline ones.
    #[serde(default)]
    pub specimen: Option<String>,

    /// One sentence for the documentation page, in English.
    ///
    /// Not an i18n key: this is developer-facing reference text about an API,
    /// which `NOCTUA.md §1` keeps in English along with every other identifier
    /// and doc comment. Reader-facing prose on the page around it is translated.
    pub summary: String,
}

/// One prop.
#[derive(Debug, Clone, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Prop {
    /// What kind of value it takes.
    #[serde(rename = "type")]
    pub kind: Kind,

    /// The values an `enum` prop accepts, in order.
    #[serde(default)]
    pub values: Vec<String>,

    /// The value in force when the prop is omitted.
    ///
    /// `None` makes the prop optional in both languages. A prop with a default
    /// is still optional — the default is applied by the component, not by the
    /// caller, so the two implementations cannot disagree about it.
    #[serde(default)]
    pub default: Option<String>,

    /// The DOM attribute this prop becomes, when it becomes one.
    #[serde(default)]
    pub attribute: Option<String>,

    /// Whether a caller must supply it.
    ///
    /// The point is `IconButton`: its label is required, so a control made of
    /// nothing but an icon and carrying no accessible name is not something a
    /// caller can construct. A rule the type system holds beats a rule a
    /// reviewer holds.
    #[serde(default)]
    pub required: bool,

    /// What it is for. Becomes the doc comment in both languages.
    pub doc: String,
}

/// The kinds of value a prop can take.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Kind {
    /// One of a fixed list. Generates an enum in Rust, a union in TypeScript.
    Enum,
    /// True or false.
    Bool,
    /// Free text.
    Text,
    /// The component's content. At most one prop may be this.
    Children,
}

/// What the component promises assistive technology.
///
/// Declared rather than described, so the conformance suite can assert it. An
/// accessibility note in a doc comment is a hope; a declared role the test
/// checks is a contract.
#[derive(Debug, Clone, Deserialize, Default)]
#[serde(deny_unknown_fields)]
pub struct A11y {
    /// The ARIA role the root carries, when it needs one beyond the element's.
    #[serde(default)]
    pub role: Option<String>,

    /// Whether the component must have an accessible name.
    #[serde(default)]
    pub needs_name: bool,

    /// Anything a reviewer has to know, in prose.
    #[serde(default)]
    pub notes: String,
}

/// Parses one component file.
///
/// # Errors
///
/// A file that does not parse, a prop with an unusable name, an `enum` with no
/// values, a default outside its own value list, or more than one `children`.
pub fn parse(path: &str, text: &str) -> Result<Component, SpecError> {
    let component: Component = toml::from_str(text).map_err(|error| {
        SpecError::new(
            path,
            text.to_owned(),
            vec![
                Problem::whole_file(format!("{path} does not parse: {error}")).fix(
                    "Every component declares [meta] with a name, an element and a summary, \
                 then one [prop.<name>] table per prop.",
                ),
            ],
        )
    })?;

    let mut problems = Vec::new();

    if component
        .prop
        .values()
        .filter(|p| p.kind == Kind::Children)
        .count()
        > 1
    {
        problems.push(
            Problem::whole_file("more than one prop is declared as `children`").fix(
                "A component has one slot for its content. Two would be ambiguous in \
                 both languages and impossible in `rsx!`.",
            ),
        );
    }

    check_props(&component, &mut problems);

    for (name, state) in &component.state {
        if !state.attribute.starts_with("data-") && !state.attribute.starts_with("aria-") {
            problems.push(
                Problem::whole_file(format!(
                    "the state `{name}` is carried by `{}`",
                    state.attribute
                ))
                .fix(
                    "A state a test can assert is a `data-` or `aria-` attribute. A class \
                     is styling, and styling is not a contract.",
                ),
            );
        }
    }

    for (key, action) in &component.keyboard {
        if action.trim().is_empty() {
            problems.push(
                Problem::whole_file(format!("`{key}` is listed with no action")).fix(
                    "Say what the key does. A key in the contract that does nothing is \
                     worse than one that is missing.",
                ),
            );
        }
    }

    if problems.is_empty() {
        Ok(component)
    } else {
        Err(SpecError::new(path, text.to_owned(), problems))
    }
}

/// Everything that can be wrong with one prop.
///
/// Its own function because `parse` is a list of checks and a list that grows
/// is a function that stops being readable — not because these belong apart.
fn check_props(component: &Component, problems: &mut Vec<Problem>) {
    for (name, prop) in &component.prop {
        if prop.kind == Kind::Enum && prop.values.is_empty() {
            problems.push(
                Problem::whole_file(format!("`{name}` is an enum with no values")).fix(
                    "List the values it accepts. An enum with none generates a type \
                     nothing can construct.",
                ),
            );
        }

        if let Some(default) = &prop.default {
            let known = match prop.kind {
                Kind::Enum => prop.values.contains(default),
                Kind::Bool => default == "true" || default == "false",
                _ => true,
            };
            if !known {
                problems.push(
                    Problem::whole_file(format!(
                        "`{name}`'s default is `{default}`, which is not one of its values"
                    ))
                    .fix("A default outside the value list is a value no caller could pass."),
                );
            }
        }

        if prop.required && prop.default.is_some() {
            problems.push(
                Problem::whole_file(format!("`{name}` is required and also has a default")).fix(
                    "A default is what makes a prop optional — the component applies it, so \
                     the caller never has to. Required means the caller must decide.",
                ),
            );
        }

        if prop.required && prop.kind == Kind::Children {
            problems.push(
                Problem::whole_file(format!("`{name}` is `children` and marked required")).fix(
                    "Content is required by the element taking it, not by a flag. Both \
                     languages already refuse a component with no children where one is \
                     needed.",
                ),
            );
        }

        if name.chars().next().is_some_and(|c| c.is_ascii_digit()) {
            problems.push(
                Problem::whole_file(format!("`{name}` starts with a digit"))
                    .fix("A prop name becomes an identifier in two languages."),
            );
        }
    }
}

/// Loads every component under `spec/components/`, sorted by name.
///
/// # Errors
///
/// A missing directory, or any component that does not validate.
pub fn load_all(directory: impl AsRef<std::path::Path>) -> Result<Vec<Component>, SpecError> {
    let directory = directory.as_ref();
    let entries = std::fs::read_dir(directory).map_err(|error| {
        SpecError::new(
            &directory.display().to_string(),
            String::new(),
            vec![
                Problem::whole_file(format!("could not read {}: {error}", directory.display()))
                    .fix("Create spec/components/, one <name>.toml per component."),
            ],
        )
    })?;

    let mut paths: Vec<_> = entries
        .flatten()
        .map(|entry| entry.path())
        .filter(|path| path.extension().is_some_and(|e| e == "toml"))
        .filter(|path| {
            // `<name>.cases.toml` sits beside `<name>.toml` and is a different
            // format read by the conformance runner, not a component.
            !path
                .file_stem()
                .is_some_and(|s| s.to_string_lossy().ends_with(".cases"))
        })
        .collect();
    paths.sort();

    let mut components = Vec::new();
    for path in paths {
        let text = std::fs::read_to_string(&path).map_err(|error| {
            SpecError::new(
                &path.display().to_string(),
                String::new(),
                vec![Problem::whole_file(format!("could not read: {error}"))],
            )
        })?;
        components.push(parse(&path.display().to_string(), &text)?);
    }
    Ok(components)
}

#[cfg(test)]
mod tests {
    use super::*;

    const HEADING: &str = r#"
[meta]
name = "Heading"
element = "h2"
summary = "A heading."

[prop.level]
type = "enum"
values = ["1", "2", "3"]
default = "2"
doc = "The semantic level."

[prop.children]
type = "children"
doc = "The text."
"#;

    #[test]
    fn a_component_parses() {
        let component = parse("t.toml", HEADING).expect("valid");
        assert_eq!(component.meta.name, "Heading");
        assert!(component.takes_children());
        assert_eq!(component.attributes().count(), 1);
    }

    #[test]
    fn an_enum_with_no_values_is_rejected() {
        let text = "[meta]\nname=\"X\"\nelement=\"p\"\nsummary=\"x\"\n[prop.a]\ntype=\"enum\"\ndoc=\"a\"\n";
        let error = parse("t.toml", text).expect_err("rejected");
        assert!(error.problems()[0].message().contains("no values"));
    }

    #[test]
    fn a_default_outside_its_values_is_rejected() {
        let text = "[meta]\nname=\"X\"\nelement=\"p\"\nsummary=\"x\"\n[prop.a]\ntype=\"enum\"\nvalues=[\"a\"]\ndefault=\"b\"\ndoc=\"a\"\n";
        let error = parse("t.toml", text).expect_err("rejected");
        assert!(
            error.problems()[0]
                .message()
                .contains("not one of its values")
        );
    }

    #[test]
    fn two_children_props_are_rejected() {
        let text = "[meta]\nname=\"X\"\nelement=\"p\"\nsummary=\"x\"\n[prop.a]\ntype=\"children\"\ndoc=\"a\"\n[prop.b]\ntype=\"children\"\ndoc=\"b\"\n";
        let error = parse("t.toml", text).expect_err("rejected");
        assert!(error.problems()[0].message().contains("more than one"));
    }
}

// --- Behavioural cases and rendered examples --------------------------------

/// The contents of `spec/components/<name>.cases.toml`.
///
/// **One file, two consumers.** The conformance runner reads `[[case]]` and
/// runs it against both implementations; the documentation page reads
/// `[[example]]` and renders it. Keeping them in one file is what stops the
/// examples on the page from drifting away from the behaviour under test —
/// they are the same component, described once.
#[derive(Debug, Clone, Deserialize, Default)]
#[serde(deny_unknown_fields)]
pub struct Cases {
    /// Behaviour both implementations must agree on.
    #[serde(default)]
    pub case: Vec<Case>,

    /// What the documentation page shows.
    #[serde(default)]
    pub example: Vec<Example>,
}

/// One behaviour, asserted against both implementations.
#[derive(Debug, Clone, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Case {
    /// What is being asserted.
    pub name: String,
    /// The props to render with.
    #[serde(default)]
    pub props: IndexMap<String, String>,
    /// The content, if any.
    #[serde(default)]
    pub children: Option<String>,
    /// A CSS selector the rendered markup must match.
    #[serde(default)]
    pub matches: Option<String>,
}

/// One example, rendered on the component's page beside its code.
#[derive(Debug, Clone, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Example {
    /// A short title, in English — it names an API shape, not a sentence.
    pub name: String,

    /// The props to render with, as the spec spells their values.
    #[serde(default)]
    pub props: IndexMap<String, String>,

    /// Literal content. Use it for code, names and anything a translation
    /// would make less clear.
    #[serde(default)]
    pub children: Option<String>,

    /// A message key, for content a reader is meant to read.
    ///
    /// `NOCTUA.md §9.10` requires a component's page to show the pt-BR and
    /// English rendering of any text it contains, and a literal cannot do
    /// that. Mutually exclusive with `children`.
    #[serde(default)]
    pub children_key: Option<String>,

    /// One line about what the example is showing, if it needs one.
    #[serde(default)]
    pub note: String,
}

/// Parses one `<name>.cases.toml`.
///
/// # Errors
///
/// A file that does not parse, an example that declares both kinds of content,
/// or an example that declares neither for a component that takes children.
pub fn parse_cases(path: &str, text: &str, component: &Component) -> Result<Cases, SpecError> {
    let cases: Cases = toml::from_str(text).map_err(|error| {
        SpecError::new(
            path,
            text.to_owned(),
            vec![Problem::whole_file(error.message().to_owned()).fix(
                "Cases are `[[case]]` tables and examples are `[[example]]` tables, each \
                 with a `name` and optional `props`.",
            )],
        )
    })?;

    let mut problems = Vec::new();
    for example in &cases.example {
        if example.children.is_some() && example.children_key.is_some() {
            problems.push(
                Problem::whole_file(format!(
                    "the example `{}` declares both `children` and `children_key`",
                    example.name
                ))
                .fix("One or the other. A literal is for code and names; a key is for prose."),
            );
        }
        if component.takes_children()
            && example.children.is_none()
            && example.children_key.is_none()
        {
            problems.push(
                Problem::whole_file(format!(
                    "the example `{}` renders {} with no content",
                    example.name, component.meta.name
                ))
                .fix("A component that takes children needs some, or the example is an empty box."),
            );
        }
        for prop in example.props.keys() {
            if !component.prop.contains_key(prop) {
                problems.push(
                    Problem::whole_file(format!(
                        "the example `{}` sets `{prop}`, which {} does not have",
                        example.name, component.meta.name
                    ))
                    .fix("Check the prop names in the component's own spec file."),
                );
            }
        }
    }

    if problems.is_empty() {
        Ok(cases)
    } else {
        Err(SpecError::new(path, text.to_owned(), problems))
    }
}

/// Loads the cases beside a component, or an empty set if there are none yet.
///
/// # Errors
///
/// Anything [`parse_cases`] refuses.
pub fn load_cases(
    directory: impl AsRef<std::path::Path>,
    component: &Component,
) -> Result<Cases, SpecError> {
    let stem = file_stem(&component.meta.name);
    let path = directory.as_ref().join(format!("{stem}.cases.toml"));
    let Ok(text) = std::fs::read_to_string(&path) else {
        return Ok(Cases::default());
    };
    parse_cases(&path.display().to_string(), &text, component)
}

/// `CodeBlock` → `code_block`, matching the file it came from.
#[must_use]
pub fn file_stem(name: &str) -> String {
    let mut out = String::with_capacity(name.len() + 2);
    for (index, ch) in name.char_indices() {
        if ch.is_ascii_uppercase() && index > 0 {
            out.push('_');
        }
        out.extend(ch.to_lowercase());
    }
    out
}

#[cfg(test)]
mod grown_schema_tests {
    use super::*;

    const BASE: &str = r#"
[meta]
name = "Thing"
element = "div"
summary = "A thing."
[prop.children]
type = "children"
doc = "The content."
"#;

    fn parse_with(extra: &str) -> Result<Component, SpecError> {
        parse("t.toml", &format!("{BASE}{extra}"))
    }

    /// The twenty components that predate these tables must keep parsing, or
    /// growing the schema is a migration rather than an addition.
    #[test]
    fn the_shipped_components_still_parse() {
        let root = concat!(env!("CARGO_MANIFEST_DIR"), "/../../spec/components");
        let components = load_all(root).expect("the shipped components");
        assert!(components.len() >= 20, "{}", components.len());
    }

    #[test]
    fn a_required_prop_with_a_default_is_rejected() {
        let error = parse_with(
            "[prop.label]\ntype = \"text\"\nrequired = true\ndefault = \"x\"\ndoc = \"A label.\"\n",
        )
        .unwrap_err();
        assert!(format!("{error:?}").contains("required and also has a default"));
    }

    #[test]
    fn a_state_carried_by_a_class_is_rejected() {
        let error =
            parse_with("[state.open]\nattribute = \"is-open\"\ndoc = \"Open.\"\n").unwrap_err();
        assert!(format!("{error:?}").contains("is-open"));
    }

    #[test]
    fn a_state_on_a_data_attribute_is_fine() {
        let component = parse_with("[state.open]\nattribute = \"data-nd-open\"\ndoc = \"Open.\"\n")
            .expect("ok");
        assert_eq!(component.state.len(), 1);
    }

    #[test]
    fn a_key_with_no_action_is_rejected() {
        let error = parse_with("[keyboard]\nEnter = \"\"\n").unwrap_err();
        assert!(format!("{error:?}").contains("no action"));
    }

    #[test]
    fn an_example_may_not_declare_two_kinds_of_content() {
        let component = parse_with("").expect("ok");
        let error = parse_cases(
            "t.cases.toml",
            "[[example]]\nname = \"One\"\nchildren = \"a\"\nchildren_key = \"b\"\n",
            &component,
        )
        .unwrap_err();
        assert!(format!("{error:?}").contains("both"));
    }

    #[test]
    fn an_example_setting_an_unknown_prop_is_rejected() {
        let component = parse_with("").expect("ok");
        let error = parse_cases(
            "t.cases.toml",
            "[[example]]\nname = \"One\"\nchildren = \"a\"\nprops = { nope = \"1\" }\n",
            &component,
        )
        .unwrap_err();
        assert!(format!("{error:?}").contains("nope"));
    }

    #[test]
    fn a_component_name_becomes_its_file_stem() {
        assert_eq!(file_stem("CodeBlock"), "code_block");
        assert_eq!(file_stem("Kbd"), "kbd");
        assert_eq!(file_stem("VisuallyHidden"), "visually_hidden");
    }
}
