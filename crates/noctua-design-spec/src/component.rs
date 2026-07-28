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

        if name.chars().next().is_some_and(|c| c.is_ascii_digit()) {
            problems.push(
                Problem::whole_file(format!("`{name}` starts with a digit"))
                    .fix("A prop name becomes an identifier in two languages."),
            );
        }
    }

    if problems.is_empty() {
        Ok(component)
    } else {
        Err(SpecError::new(path, text.to_owned(), problems))
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
