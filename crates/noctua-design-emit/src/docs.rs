//! The documentation catalogue: one page per component, generated.
//!
//! # Why this exists
//!
//! The site had a page called "Components" — one page, with a hand-written
//! gallery of fifteen rows and a prop table for exactly one component. Adding a
//! component to `spec/components/` documented nothing; somebody had to remember
//! to add a row, and then to keep it true.
//!
//! `NOCTUA.md §9.10` is closed on the shape: the sidebar lists **all components
//! in alphabetical order**, and **every component's page** carries what it is,
//! every prop with type, default and effect, every variant and state, examples,
//! the keyboard contract and the accessibility contract. None of that is a
//! thing to write per component. It is a thing to derive from the contract that
//! already exists.
//!
//! # Why it is emitted rather than parsed at runtime
//!
//! `docs-site` could read `spec/components/*.toml` itself. It does not, for two
//! reasons: the drift check then covers the catalogue like every other
//! generated file, so a component whose page would not build fails
//! `cargo xtask check` rather than the site build; and the pages stay a `const`
//! array of `&'static str`, which is what lets a link be a `PageId` instead of
//! a string.

use std::fmt::Write as _;

use noctua_design_spec::component::{Component, Kind};

use crate::{COMPONENT_SPEC, CommentStyle, EmittedFile, header_from};

/// Emits the catalogue the documentation site is built from.
#[must_use]
pub fn emit(components: &[Component]) -> Vec<EmittedFile> {
    vec![EmittedFile::new(
        "docs-site/src/generated/catalogue.rs",
        catalogue(components),
    )]
}

/// `CodeBlock` → `code-block`. The URL segment, and the i18n key fragment.
///
/// The spec crate's, because the same transformation names the derived messages
/// it produces — and two spellings of one slug would be a page whose prose is
/// looked up under a key nothing wrote.
pub use noctua_design_spec::component::{slug, slugify};

/// `code-block` → `ComponentCodeBlockSummary`, the generated message key.
fn summary_key(slug: &str) -> String {
    let mut out = String::from("Component");
    for part in slug.split('-') {
        let mut chars = part.chars();
        if let Some(first) = chars.next() {
            out.extend(first.to_uppercase());
            out.push_str(chars.as_str());
        }
    }
    out.push_str("Summary");
    out
}

fn catalogue(components: &[Component]) -> String {
    let mut out = header_from(CommentStyle::Line("//!"), COMPONENT_SPEC);
    out.push_str(
        "//!\n\
         //! Every component, as the documentation site needs it: the contract, flat.\n\
         //!\n\
         //! Adding a component to `spec/components/` adds its page, its sidebar entry\n\
         //! and its search entries. There is no list to keep in step, which is the\n\
         //! whole point — the one that existed went stale the day it was written.\n\
         \n\
         use noctua_design_i18n::Key;\n\n",
    );

    out.push_str(TYPES);

    let _ = writeln!(
        out,
        "/// Every component, in the order `spec/components/` is read.\n\
         pub(crate) const COMPONENTS: [DocComponent; {}] = [",
        components.len()
    );
    for component in components {
        write_component(&mut out, component);
    }
    out.push_str("];\n");
    out
}

/// The shapes, written once here rather than derived — they are a data
/// structure, not a mapping of anything in the spec.
const TYPES: &str = r"/// One prop, as the page shows it.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct DocProp {
    /// Its name in both languages.
    pub name: &'static str,
    /// `enum`, `bool`, `text` or `children`.
    pub kind: &'static str,
    /// The values an enum accepts, in order.
    pub values: &'static [&'static str],
    /// The element each value renders, when the prop chooses the element.
    ///
    /// Empty for every other prop. A page that showed an ordered list as a
    /// `<ul>` carrying a `data-nd-kind` attribute would be documenting the
    /// defect rather than the component.
    pub elements: &'static [&'static str],
    /// What the component applies when the caller says nothing.
    pub default: Option<&'static str>,
    /// The DOM attribute it becomes, when it becomes one.
    pub attribute: Option<&'static str>,
    /// Whether the caller must supply it.
    pub required: bool,
    /// Whether the variant grid draws a picture of each value.
    ///
    /// `false` for an enum with no appearance — a comparator, a band, a scope.
    /// A grid of identical pictures under different captions reads as a
    /// component whose props do nothing.
    pub variants: bool,
    /// What the grid renders for this prop, when the component's own specimen
    /// cannot show it. Empty means the component's.
    pub specimen: Option<&'static str>,
    /// What it is for, as a message. A prop's documentation is prose a reader
    /// reads, and this is the layer that stops it being English everywhere.
    pub doc: Key,
}

/// One state, and what carries it.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct DocState {
    /// Its name.
    pub name: &'static str,
    /// The `data-` or `aria-` attribute a test can assert.
    pub attribute: &'static str,
    /// What it means, as a message.
    pub doc: Key,
}

/// One key, and what it does.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct DocKey {
    /// The key, as a reader would name it. A chord spelled out in words is a
    /// phrase rather than an identifier, so it is a message too.
    pub key: Key,
    /// What pressing it does.
    pub action: Key,
}

/// One component's whole contract, flattened for rendering.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct DocComponent {
    /// Its name, which is an identifier and the same in every locale.
    pub name: &'static str,
    /// Its URL segment, and its anchor on a collection page.
    pub slug: &'static str,
    /// The collection page it is documented on, or none for a page of its own.
    pub page: Option<&'static str>,
    /// The translated lead under the title.
    pub summary: Key,
    /// The element it renders by default.
    pub element: &'static str,
    /// The class its root carries.
    pub class: Option<&'static str>,
    /// When to reach for it. Empty for a component whose spec says nothing.
    pub guidance: Option<Key>,
    /// What a variant shows inside it, when the page's own choice is wrong.
    pub specimen: Option<&'static str>,
    /// Every prop, in declaration order.
    pub props: &'static [DocProp],
    /// Every state.
    pub states: &'static [DocState],
    /// The keyboard contract, which is empty for a component that takes no keys.
    pub keyboard: &'static [DocKey],
    /// The ARIA role its root carries beyond the element's own.
    pub role: Option<&'static str>,
    /// Whether it must have an accessible name.
    pub needs_name: bool,
    /// Anything a reviewer has to know.
    pub a11y_notes: Option<Key>,
}

";

fn write_component(out: &mut String, component: &Component) {
    let name = &component.meta.name;
    let slug = slug(name);
    let _ = writeln!(out, "    DocComponent {{");
    let _ = writeln!(out, "        name: \"{name}\",");
    let _ = writeln!(out, "        slug: \"{slug}\",");
    let _ = writeln!(
        out,
        "        page: {},",
        option(component.meta.page.as_deref())
    );
    let _ = writeln!(out, "        summary: Key::{},", summary_key(&slug));
    let _ = writeln!(out, "        element: \"{}\",", component.meta.element);
    let _ = writeln!(
        out,
        "        class: {},",
        option(component.meta.class.as_deref())
    );
    let _ = writeln!(
        out,
        "        guidance: {},",
        message(&slug, "guidance", &component.guidance)
    );
    let _ = writeln!(
        out,
        "        specimen: {},",
        option(component.meta.specimen.as_deref())
    );

    out.push_str("        props: &[\n");
    for (prop_name, prop) in &component.prop {
        let kind = match prop.kind {
            Kind::Enum => "enum",
            Kind::Bool => "bool",
            Kind::Text => "text",
            Kind::Children => "children",
        };
        let values = prop
            .values
            .iter()
            .map(|value| format!("\"{value}\""))
            .collect::<Vec<_>>()
            .join(", ");
        let elements = prop
            .elements
            .iter()
            .map(|element| format!("\"{element}\""))
            .collect::<Vec<_>>()
            .join(", ");
        let _ = writeln!(
            out,
            "            DocProp {{ name: \"{prop_name}\", kind: \"{kind}\", values: &[{values}], \
             elements: &[{elements}], default: {}, attribute: {}, required: {}, \
             variants: {}, specimen: {}, doc: {} }},",
            option(prop.default.as_deref()),
            option(prop.attribute.as_deref()),
            prop.required,
            prop.variants,
            option(prop.specimen.as_deref()),
            required_message(&slug, &format!("prop.{prop_name}.doc"), &prop.doc)
        );
    }
    out.push_str("        ],\n");

    out.push_str("        states: &[\n");
    for (state_name, state) in &component.state {
        let _ = writeln!(
            out,
            "            DocState {{ name: \"{state_name}\", attribute: \"{}\", doc: {} }},",
            state.attribute,
            required_message(&slug, &format!("state.{state_name}.doc"), &state.doc)
        );
    }
    out.push_str("        ],\n");

    out.push_str("        keyboard: &[\n");
    for (key, action) in &component.keyboard {
        let name = slugify(key);
        let _ = writeln!(
            out,
            "            DocKey {{ key: {}, action: {} }},",
            required_message(&slug, &format!("key.{name}.name"), key),
            required_message(&slug, &format!("key.{name}.action"), action)
        );
    }
    out.push_str("        ],\n");

    let _ = writeln!(
        out,
        "        role: {},",
        option(component.a11y.role.as_deref())
    );
    let _ = writeln!(out, "        needs_name: {},", component.a11y.needs_name);
    let _ = writeln!(
        out,
        "        a11y_notes: {},",
        message(&slug, "a11y-notes", &component.a11y.notes)
    );
    out.push_str("    },\n");
}

/// The `Key` variant a derived message becomes, or `None` when the spec is
/// silent — which is what the page renders nothing for.
fn message(slug: &str, suffix: &str, text: &str) -> String {
    if text.trim().is_empty() {
        return "None".to_owned();
    }
    format!("Some(Key::{})", key_variant(slug, suffix))
}

/// The same, for a field the page always renders and the spec always fills.
fn required_message(slug: &str, suffix: &str, text: &str) -> String {
    debug_assert!(
        !text.trim().is_empty(),
        "{slug}.{suffix} is empty; the spec validator should have refused it"
    );
    format!("Key::{}", key_variant(slug, suffix))
}

/// `heading` + `prop.level.doc` → `ComponentHeadingPropLevelDoc`.
///
/// The same transformation `i18n::variant` applies to an authored key, done
/// here so the catalogue names exactly what `derived_messages` inserted. Two
/// spellings of one key would be a compile error rather than a silent one,
/// which is the only reason this duplication is safe.
fn key_variant(slug: &str, suffix: &str) -> String {
    let mut out = String::from("Component");
    let mut upper = true;
    for ch in format!("{slug}.{suffix}").chars() {
        if ch == '.' || ch == '-' || ch == '_' {
            upper = true;
        } else if upper {
            out.extend(ch.to_uppercase());
            upper = false;
        } else {
            out.extend(ch.to_lowercase());
        }
    }
    out
}

fn option(value: Option<&str>) -> String {
    value.map_or_else(
        || "None".to_owned(),
        |value| format!("Some(\"{}\")", escape(value)),
    )
}

fn escape(text: &str) -> String {
    text.replace('\\', "\\\\").replace('"', "\\\"")
}

#[cfg(test)]
mod tests {
    use super::*;

    fn shipped() -> Vec<Component> {
        let root = concat!(env!("CARGO_MANIFEST_DIR"), "/../../spec/components");
        noctua_design_spec::component::load_all(root).expect("the shipped components")
    }

    #[test]
    fn names_become_slugs_and_keys() {
        assert_eq!(slug("CodeBlock"), "code-block");
        assert_eq!(slug("Kbd"), "kbd");
        assert_eq!(summary_key("code-block"), "ComponentCodeBlockSummary");
        assert_eq!(summary_key("kbd"), "ComponentKbdSummary");
    }

    /// The catalogue must name a real message key for every component, or the
    /// site does not compile — which is the point of generating it.
    #[test]
    fn every_component_has_a_translated_lead() {
        let source = catalogue(&shipped());
        for component in shipped() {
            let key = summary_key(&slug(&component.meta.name));
            assert!(
                source.contains(&format!("Key::{key}")),
                "{} has no lead",
                component.meta.name
            );
        }
    }

    #[test]
    fn every_component_is_in_the_catalogue() {
        let components = shipped();
        let source = catalogue(&components);
        assert!(source.contains(&format!("[DocComponent; {}]", components.len())));
        for component in &components {
            assert!(
                source.contains(&format!("name: \"{}\"", component.meta.name)),
                "{}",
                component.meta.name
            );
        }
    }

    /// Two components cannot share a URL.
    #[test]
    fn every_slug_is_distinct() {
        let mut slugs: Vec<String> = shipped()
            .iter()
            .map(|component| slug(&component.meta.name))
            .collect();
        let count = slugs.len();
        slugs.sort();
        slugs.dedup();
        assert_eq!(slugs.len(), count);
    }
}
