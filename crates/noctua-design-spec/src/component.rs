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

    /// The prop that chooses which element is rendered, if one does.
    ///
    /// At most one may: two props both claiming the element would have no
    /// defined answer when they disagree, and `check_props` refuses it.
    #[must_use]
    pub fn element_switch(&self) -> Option<(&String, &Prop)> {
        self.attributes().find(|(_, p)| !p.elements.is_empty())
    }

    /// The element a value of the switching prop renders.
    ///
    /// Falls back to `meta.element`, which is what a component with no
    /// switching prop renders and what an unknown value would have rendered
    /// before this existed.
    #[must_use]
    pub fn element_for(&self, value: &str) -> &str {
        self.element_switch()
            .and_then(|(_, prop)| {
                prop.values
                    .iter()
                    .position(|candidate| candidate == value)
                    .and_then(|at| prop.elements.get(at))
            })
            .map_or(self.meta.element.as_str(), String::as_str)
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

    /// The documentation page this component is described on.
    ///
    /// Absent means what it has always meant: a page of its own, at the
    /// component's own slug. A value names a COLLECTION page — several
    /// components documented together, because a reader comparing them opens
    /// one page instead of six.
    ///
    /// It is in the contract rather than in a list beside the site for the
    /// reason every list in this repository has eventually earned: a list of
    /// which components moved is a second copy of a fact, and the two stop
    /// agreeing. `site::pages` refuses a value it does not recognise.
    #[serde(default)]
    pub page: Option<String>,

    /// One sentence for the documentation page, in English.
    ///
    /// Not an i18n key: this is developer-facing reference text about an API,
    /// which `NOCTUA.md §1` keeps in English along with every other identifier
    /// and doc comment. Reader-facing prose on the page around it is translated.
    pub summary: String,
}

/// `serde`'s default for a `bool` is `false`, and `variants` is `true`.
fn yes() -> bool {
    true
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

    /// The element each value renders, when the prop chooses the element.
    ///
    /// # Why this is declared rather than written into an implementation
    ///
    /// Two props needed it and neither had it, so both were **inert**.
    /// `Heading.level` documented itself as "the outline level" and rendered
    /// `<h2>` whatever it was set to; `List.kind` said "changes the element,
    /// not only the marker" and rendered `<ul data-nd-kind="ordered">`. Each
    /// compiled, each rendered, and each was a prop a caller could set with no
    /// effect whatsoever — the worst kind of defect, because the page looks
    /// almost right.
    ///
    /// Declaring it here reaches all three places that need to agree at once:
    /// both implementations switch on it, and the documentation page shows the
    /// element it actually produces. Same length as `values`, position by
    /// position; the validator refuses anything else.
    #[serde(default)]
    pub elements: Vec<String>,

    /// Whether the variant grid draws a picture of each value.
    ///
    /// # Why a prop would say no
    ///
    /// Some enums have no appearance. A comparator (`TableColumn.sort_as`), a
    /// band (`TableSection.kind` — a `<thead>` and a `<tbody>` look the same in
    /// isolation), a scope, a total that a script computes: every value renders
    /// the identical picture, and a grid of identical pictures under different
    /// captions is worse than no grid. It reads as a component whose props do
    /// nothing, which is exactly the report that produced this field.
    ///
    /// `true` by default: a prop that changes what a reader sees should be
    /// shown, and opting out has to be a decision somebody wrote down.
    #[serde(default = "yes")]
    pub variants: bool,

    /// What the variant grid renders for this prop, when the component's own
    /// specimen cannot show it.
    ///
    /// One component has one specimen, and two props can want opposite things
    /// from it: `TableCell.align` needs a value SHORT enough to leave room to
    /// move in, and `TableCell.wrap` needs one LONG enough to break. With one
    /// specimen for both, whichever prop lost was documented by four identical
    /// pictures.
    #[serde(default)]
    pub specimen: Option<String>,

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

/// `CodeBlock` → `code-block`. The URL segment, and the fragment every
/// message derived from this component is named after.
#[must_use]
pub fn slug(name: &str) -> String {
    let mut out = String::with_capacity(name.len() + 2);
    for (index, ch) in name.char_indices() {
        if ch.is_ascii_uppercase() && index > 0 {
            out.push('-');
        }
        out.extend(ch.to_lowercase());
    }
    out
}

/// `Space or Enter` → `space-or-enter`, so a key name becomes a message key.
///
/// A keyboard contract is keyed by the chord a reader presses, and that chord
/// is itself a phrase somebody has to translate. Slugging it gives a stable
/// name for both halves without an index, which would renumber every
/// translation the day a key is inserted in the middle.
#[must_use]
pub fn slugify(text: &str) -> String {
    let mut out = String::with_capacity(text.len());
    let mut dash = false;
    for ch in text.chars() {
        if ch.is_ascii_alphanumeric() {
            out.extend(ch.to_lowercase());
            dash = false;
        } else if !out.is_empty() && !dash {
            out.push('-');
            dash = true;
        }
    }
    out.trim_end_matches('-').to_owned()
}

/// Every sentence a component's contract carries that a **reader** sees.
///
/// # Why these become messages
///
/// `spec/components/<name>.toml` holds prose in two roles at once. Some of it
/// is developer-facing reference that becomes the doc comment in both
/// languages — and the site renders exactly the same sentences on the page.
/// The doc comment must be English (`NOCTUA.md §1`); the page must be the
/// reader's language (§9.9). Both were the same string, so the page lost.
///
/// So the English is derived from here into the catalogue, where the
/// cross-locale check demands a Portuguese answer for each one, and the
/// catalogue is what the page reads. The spec still owns the words; it simply
/// stops being the last word for a reader who does not speak them.
#[must_use]
pub fn derived_messages(components: &[Component]) -> Vec<(String, String)> {
    let mut out = Vec::new();
    for component in components {
        let slug = slug(&component.meta.name);
        let mut push = |suffix: String, text: &str| {
            if !text.trim().is_empty() {
                out.push((format!("component.{slug}.{suffix}"), collapse(text)));
            }
        };

        push("guidance".to_owned(), &component.guidance);
        push("a11y-notes".to_owned(), &component.a11y.notes);
        for (prop, spec) in &component.prop {
            push(format!("prop.{prop}.doc"), &spec.doc);
        }
        for (state, spec) in &component.state {
            push(format!("state.{state}.doc"), &spec.doc);
        }
        for (chord, action) in &component.keyboard {
            let name = slugify(chord);
            push(format!("key.{name}.name"), chord);
            push(format!("key.{name}.action"), action);
        }
    }
    out
}

/// A block of prose as one line.
///
/// A `"""…"""` note in TOML keeps its newlines, and a message with a newline in
/// it is a message that should have been two — the catalogue's own escaping
/// says so. Paragraph breaks survive as a full stop and a space, which is what
/// they already read as on the page.
fn collapse(text: &str) -> String {
    text.split_whitespace().collect::<Vec<_>>().join(" ")
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

    if component
        .prop
        .values()
        .filter(|p| !p.elements.is_empty())
        .count()
        > 1
    {
        problems.push(
            Problem::whole_file("two props both choose the element").fix(
                "Only one prop may. Two would have no defined answer the moment they \
                 disagreed, and the generator would silently pick whichever it read first.",
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
    /// Words a prop may not be called, because Rust has them.
    const KEYWORDS: [&str; 24] = [
        "as", "box", "break", "const", "continue", "crate", "else", "enum", "fn", "for", "if",
        "impl", "in", "let", "loop", "match", "mod", "move", "ref", "return", "static", "type",
        "use", "where",
    ];

    for (name, prop) in &component.prop {
        if prop.kind == Kind::Enum && prop.values.is_empty() {
            problems.push(
                Problem::whole_file(format!("`{name}` is an enum with no values")).fix(
                    "List the values it accepts. An enum with none generates a type \
                     nothing can construct.",
                ),
            );
        }

        if !prop.elements.is_empty() && prop.elements.len() != prop.values.len() {
            problems.push(
                Problem::whole_file(format!(
                    "`{name}` declares {} element(s) for {} value(s)",
                    prop.elements.len(),
                    prop.values.len()
                ))
                .fix(
                    "`elements` is read position by position against `values`. A short \
                     list would leave some values with no element and the generator \
                     silently falling back to the default one — which is the inert prop \
                     this field exists to make impossible.",
                ),
            );
        }

        if !prop.elements.is_empty() && prop.attribute.is_some() {
            problems.push(
                Problem::whole_file(format!(
                    "`{name}` both switches the element and becomes an attribute"
                ))
                .fix(
                    "Choose one. A prop that changes the element has already said what \
                     it does; repeating it as a `data-` attribute gives a stylesheet a \
                     second, redundant hook that can disagree with the first.",
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

        // A RUST KEYWORD IS NOT A PROP NAME, and the failure without this
        // check is a rustfmt diagnostic about a generated file nobody wrote:
        // `loop` reached the generator and came back as *expected identifier,
        // found keyword*, pointing at a line in `dist`. Escaping it to
        // `r#loop` would work in Rust and be one prop with two spellings
        // across the two packages, which is the thing they exist not to have.
        if KEYWORDS.contains(&name.as_str()) {
            problems.push(
                Problem::whole_file(format!("`{name}` is a Rust keyword")).fix(
                    "Pick another word. A prop a Rust consumer could only write as `r#name` \
                     would be one prop with two spellings across the two packages — `loop` \
                     became `wrap`, and `type` became `kind` before it.",
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
