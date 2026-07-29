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
    /// What the component applies when the caller says nothing.
    pub default: Option<&'static str>,
    /// The DOM attribute it becomes, when it becomes one.
    pub attribute: Option<&'static str>,
    /// Whether the caller must supply it.
    pub required: bool,
    /// What it is for.
    pub doc: &'static str,
}

/// One state, and what carries it.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct DocState {
    /// Its name.
    pub name: &'static str,
    /// The `data-` or `aria-` attribute a test can assert.
    pub attribute: &'static str,
    /// What it means.
    pub doc: &'static str,
}

/// One key, and what it does.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct DocKey {
    /// The key, as a reader would name it.
    pub key: &'static str,
    /// What pressing it does.
    pub action: &'static str,
}

/// One component's whole contract, flattened for rendering.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct DocComponent {
    /// Its name, which is an identifier and the same in every locale.
    pub name: &'static str,
    /// Its URL segment.
    pub slug: &'static str,
    /// The translated lead under the title.
    pub summary: Key,
    /// The element it renders by default.
    pub element: &'static str,
    /// The class its root carries.
    pub class: Option<&'static str>,
    /// When to reach for it, in English — API reference, like a doc comment.
    pub guidance: &'static str,
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
    pub a11y_notes: &'static str,
}

";

fn write_component(out: &mut String, component: &Component) {
    let name = &component.meta.name;
    let slug = slug(name);
    let _ = writeln!(out, "    DocComponent {{");
    let _ = writeln!(out, "        name: \"{name}\",");
    let _ = writeln!(out, "        slug: \"{slug}\",");
    let _ = writeln!(out, "        summary: Key::{},", summary_key(&slug));
    let _ = writeln!(out, "        element: \"{}\",", component.meta.element);
    let _ = writeln!(
        out,
        "        class: {},",
        option(component.meta.class.as_deref())
    );
    let _ = writeln!(
        out,
        "        guidance: \"{}\",",
        escape(&component.guidance)
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
        let _ = writeln!(
            out,
            "            DocProp {{ name: \"{prop_name}\", kind: \"{kind}\", values: &[{values}], \
             default: {}, attribute: {}, required: {}, doc: \"{}\" }},",
            option(prop.default.as_deref()),
            option(prop.attribute.as_deref()),
            prop.required,
            escape(&prop.doc)
        );
    }
    out.push_str("        ],\n");

    out.push_str("        states: &[\n");
    for (state_name, state) in &component.state {
        let _ = writeln!(
            out,
            "            DocState {{ name: \"{state_name}\", attribute: \"{}\", doc: \"{}\" }},",
            state.attribute,
            escape(&state.doc)
        );
    }
    out.push_str("        ],\n");

    out.push_str("        keyboard: &[\n");
    for (key, action) in &component.keyboard {
        let _ = writeln!(
            out,
            "            DocKey {{ key: \"{key}\", action: \"{}\" }},",
            escape(action)
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
        "        a11y_notes: \"{}\",",
        escape(&component.a11y.notes)
    );
    out.push_str("    },\n");
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
