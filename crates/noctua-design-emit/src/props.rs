//! Every component, in both languages, from one spec each.
//!
//! | Output | For |
//! |---|---|
//! | `crates/noctua-design/src/components/<name>.rs` | the Dioxus package |
//! | `packages/react/src/components/<name>.js` + `.d.ts` | the React package |
//!
//! # This is the parity mechanism, not a convenience
//!
//! `NOCTUA.md §9.3` Leg 2 requires the two packages to declare the same props
//! with the same defaults. Generating both from `spec/components/<name>.toml`
//! makes that true by construction: neither file names a prop, a default or a
//! value, so neither can disagree with the other about one. Adding a prop to one
//! implementation by hand is either overwritten by the next build or reported by
//! the drift check.

use std::fmt::Write as _;

use noctua_design_spec::component::{Component, Kind, Prop};

use crate::controls;
use crate::{COMPONENT_SPEC, CommentStyle, EmittedFile, camel, header_from, identifier};

/// Emits every component in both languages, plus each package's module index.
#[must_use]
pub fn emit(components: &[Component]) -> Vec<EmittedFile> {
    let mut files = Vec::new();
    for component in components {
        let file = file_stem(&component.meta.name);
        files.push(EmittedFile::new(
            format!("crates/noctua-design/src/components/{file}.rs"),
            dioxus(component),
        ));
        files.push(EmittedFile::new(
            format!("packages/react/src/components/{file}.js"),
            react(component),
        ));
        files.push(EmittedFile::new(
            format!("packages/react/src/components/{file}.d.ts"),
            react_types(component),
        ));
    }
    files.push(EmittedFile::new(
        "crates/noctua-design/src/components/mod.rs",
        rust_index(components),
    ));
    files.push(EmittedFile::new(
        "packages/react/src/components/index.js",
        js_index(components),
    ));
    files
}

// --- Dioxus ----------------------------------------------------------------

fn dioxus(component: &Component) -> String {
    let mut out = header_from(CommentStyle::Line("//!"), COMPONENT_SPEC);
    let name = &component.meta.name;
    writeln!(out, "//!\n//! {}\n", component.meta.summary).expect("write");
    out.push_str("use dioxus::prelude::*;\n");
    out.push_str(&controls::dioxus_imports(component));
    out.push('\n');

    for (prop_name, prop) in component.attributes() {
        if prop.kind == Kind::Enum {
            rust_enum(&mut out, name, prop_name, prop);
        }
    }

    writeln!(out, "/// {}", component.meta.summary).expect("write");
    if !component.a11y.notes.is_empty() {
        out.push_str("///\n/// # Accessibility\n///\n");
        for line in component.a11y.notes.lines() {
            writeln!(out, "/// {line}").expect("write");
        }
    }
    out.push_str("#[component]\n");
    writeln!(out, "pub fn {name}(").expect("write");

    for (prop_name, prop) in component.attributes() {
        writeln!(out, "    /// {}", prop.doc).expect("write");
        let ty = controls::dioxus_type(component, prop_name)
            .map_or_else(|| rust_type(name, prop_name, prop), str::to_owned);
        if prop.default.is_some() || prop.kind == Kind::Bool {
            writeln!(
                out,
                "    #[props(default)]\n    {}: {ty},",
                identifier(prop_name)
            )
            .expect("write");
        } else if prop.required {
            // Not `Option`. A required prop is the point of `required`: an
            // `IconButton` with no label must not be a thing a caller can
            // construct, and the site shipped one that was.
            writeln!(out, "    {}: {ty},", identifier(prop_name)).expect("write");
        } else {
            writeln!(out, "    {}: Option<{ty}>,", identifier(prop_name)).expect("write");
        }
    }
    out.push_str(
        "    /// Extra classes for the root element.\n    \
         #[props(default)]\n    class: String,\n",
    );
    if component.takes_children() {
        out.push_str("    /// The content.\n    children: Element,\n");
    }
    out.push_str(") -> Element {\n");

    if controls::is_custom(component) {
        controls::dioxus_body(&mut out, component);
        out.push_str("}\n");
    } else {
        rust_body(&mut out, component);
    }
    out
}

fn rust_enum(out: &mut String, component: &str, prop: &str, spec: &Prop) {
    let ty = format!("{component}{}", pascal(prop));
    writeln!(out, "/// {}", spec.doc).expect("write");
    out.push_str("#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]\npub enum ");
    writeln!(out, "{ty} {{").expect("write");
    for value in &spec.values {
        if spec.default.as_ref() == Some(value) {
            out.push_str("    #[default]\n");
        }
        writeln!(out, "    /// `{value}`\n    {},", pascal(value)).expect("write");
    }
    out.push_str("}\n\n");

    writeln!(out, "impl {ty} {{").expect("write");
    writeln!(
        out,
        "    /// Every value, in the order the spec declares them.\n    \
         pub const ALL: [Self; {}] = [{}];\n",
        spec.values.len(),
        spec.values
            .iter()
            .map(|v| format!("Self::{}", pascal(v)))
            .collect::<Vec<_>>()
            .join(", ")
    )
    .expect("write");
    out.push_str("    /// The value as it appears in the DOM.\n    #[must_use]\n    pub const fn as_str(self) -> &'static str {\n        match self {\n");
    for value in &spec.values {
        writeln!(out, "            Self::{} => \"{value}\",", pascal(value)).expect("write");
    }
    out.push_str("        }\n    }\n}\n\n");
}

fn rust_type(component: &str, prop: &str, spec: &Prop) -> String {
    match spec.kind {
        Kind::Enum => format!("{component}{}", pascal(prop)),
        Kind::Bool => "bool".to_owned(),
        Kind::Text => "String".to_owned(),
        Kind::Children => "Element".to_owned(),
    }
}

fn rust_body(out: &mut String, component: &Component) {
    let element = &component.meta.element;
    let class = component.meta.class.as_deref().unwrap_or("");

    writeln!(out, "    let root = format!(\"{class} {{class}}\");").expect("write");
    out.push_str("    let root = root.trim().to_owned();\n\n    rsx! {\n");
    writeln!(
        out,
        "        {element} {{\n            class: \"{{root}}\","
    )
    .expect("write");

    if let Some(role) = &component.a11y.role {
        writeln!(out, "            role: \"{role}\",").expect("write");
    }
    for (prop_name, prop) in component.attributes() {
        let Some(attribute) = &prop.attribute else {
            continue;
        };
        let field = identifier(prop_name);
        match prop.kind {
            Kind::Enum => {
                writeln!(out, "            \"{attribute}\": {field}.as_str(),").expect("write");
            }
            Kind::Bool => writeln!(
                out,
                "            \"{attribute}\": if {field} {{ \"true\" }} else {{ \"false\" }},"
            )
            .expect("write"),
            _ => writeln!(out, "            \"{attribute}\": {field},").expect("write"),
        }
    }
    if component.takes_children() {
        out.push_str("            {children}\n");
    }
    out.push_str("        }\n    }\n}\n");
}

fn rust_index(components: &[Component]) -> String {
    let mut out = header_from(CommentStyle::Line("//!"), COMPONENT_SPEC);
    out.push_str(
        "//!\n//! Every component, one module each, generated from its spec.\n\
         //!\n\
         //! Adding a file to `spec/components/` adds a module here. There is nothing\n\
         //! to remember and nothing to keep in step.\n\n",
    );
    for component in components {
        let file = file_stem(&component.meta.name);
        writeln!(out, "mod {file};").expect("write");
    }
    out.push('\n');
    for component in components {
        let file = file_stem(&component.meta.name);
        let name = &component.meta.name;
        let types: Vec<String> = component
            .attributes()
            .filter(|(_, p)| p.kind == Kind::Enum)
            .map(|(prop, _)| format!("{name}{}", pascal(prop)))
            .collect();
        if types.is_empty() {
            writeln!(out, "pub use {file}::{name};").expect("write");
        } else {
            writeln!(out, "pub use {file}::{{{name}, {}}};", types.join(", ")).expect("write");
        }
    }
    out
}

// --- React -----------------------------------------------------------------

fn react(component: &Component) -> String {
    let mut out = header_from(CommentStyle::Line("//"), COMPONENT_SPEC);
    let name = &component.meta.name;
    writeln!(
        out,
        "\n// {}\n//\n// Generated from the same spec as the Dioxus component, so the two\n\
         // cannot declare different props.\n\n\
         import {{ createElement as h }} from \"react\";\n",
        component.meta.summary
    )
    .expect("write");

    let defaults: Vec<String> = component
        .attributes()
        .filter_map(|(prop_name, prop)| {
            prop.default
                .as_ref()
                .map(|d| format!("  {}: \"{d}\",", camel(prop_name)))
        })
        .collect();
    if !defaults.is_empty() {
        writeln!(
            out,
            "/** Applied by the component, never by the caller — which is what keeps\n\
             \x20* the two implementations from disagreeing about a default. */\n\
             const defaults = {{\n{}\n}};\n",
            defaults.join("\n")
        )
        .expect("write");
    }

    let params: Vec<String> = component
        .attributes()
        .map(|(prop_name, _)| camel(prop_name))
        .collect();
    let mut signature = params.clone();
    signature.push("className".to_owned());
    if component.takes_children() {
        signature.push("children".to_owned());
    }
    signature.push("...rest".to_owned());

    writeln!(
        out,
        "export function {name}({{ {} }}) {{",
        signature.join(", ")
    )
    .expect("write");

    if controls::is_custom(component) {
        controls::react_body(&mut out, component);
        out.push_str("}\n");
        return out;
    }

    for (prop_name, prop) in component.attributes() {
        if let Some(default) = &prop.default {
            writeln!(
                out,
                "  const {0}_ = {0} ?? \"{default}\";",
                camel(prop_name)
            )
            .expect("write");
        }
    }

    let class = component.meta.class.as_deref().unwrap_or("");
    writeln!(
        out,
        "  const root = [\"{class}\", className].filter(Boolean).join(\" \");\n  \
         const props = {{ className: root, ...rest }};"
    )
    .expect("write");

    if let Some(role) = &component.a11y.role {
        writeln!(out, "  props.role = \"{role}\";").expect("write");
    }
    for (prop_name, prop) in component.attributes() {
        let Some(attribute) = &prop.attribute else {
            continue;
        };
        let value = if prop.default.is_some() {
            format!("{}_", camel(prop_name))
        } else {
            camel(prop_name)
        };
        writeln!(
            out,
            "  if ({value} !== undefined) props[\"{attribute}\"] = String({value});"
        )
        .expect("write");
    }

    if component.takes_children() {
        writeln!(
            out,
            "  return h(\"{}\", props, children);",
            component.meta.element
        )
        .expect("write");
    } else {
        writeln!(out, "  return h(\"{}\", props);", component.meta.element).expect("write");
    }
    out.push_str("}\n");
    out
}

fn react_types(component: &Component) -> String {
    let mut out = header_from(CommentStyle::Line("//"), COMPONENT_SPEC);
    let name = &component.meta.name;
    out.push_str("\nimport type { ReactNode } from \"react\";\n");
    out.push_str(&controls::typescript_imports(component));
    out.push('\n');

    for (prop_name, prop) in component.attributes() {
        if prop.kind != Kind::Enum {
            continue;
        }
        writeln!(
            out,
            "export type {name}{} = {};",
            pascal(prop_name),
            prop.values
                .iter()
                .map(|v| format!("\"{v}\""))
                .collect::<Vec<_>>()
                .join(" | ")
        )
        .expect("write");
    }

    writeln!(out, "\nexport interface {name}Props {{").expect("write");
    for (prop_name, prop) in component.attributes() {
        writeln!(out, "  /** {} */", prop.doc).expect("write");
        let ty = controls::typescript_type(component, prop_name).map_or_else(
            || match prop.kind {
                Kind::Enum => format!("{name}{}", pascal(prop_name)),
                Kind::Bool => "boolean".to_owned(),
                _ => "string".to_owned(),
            },
            str::to_owned,
        );
        // A required prop is not optional in TypeScript either, or the two
        // languages disagree about whether it can be left out.
        let optional = if prop.required { "" } else { "?" };
        writeln!(out, "  {}{optional}: {ty};", camel(prop_name)).expect("write");
    }
    out.push_str("  className?: string;\n");
    if component.takes_children() {
        out.push_str("  children?: ReactNode;\n");
    }
    out.push_str("}\n\n");
    writeln!(
        out,
        "/** {} */\nexport declare function {name}(props: {name}Props): JSX.Element;",
        component.meta.summary
    )
    .expect("write");
    out
}

fn js_index(components: &[Component]) -> String {
    let mut out = header_from(CommentStyle::Line("//"), COMPONENT_SPEC);
    out.push_str("\n// Every component, generated from spec/components/.\n\n");
    for component in components {
        writeln!(
            out,
            "export {{ {} }} from \"./{}.js\";",
            component.meta.name,
            file_stem(&component.meta.name)
        )
        .expect("write");
    }
    out
}

// --- Shared ----------------------------------------------------------------

/// `CodeBlock` → `code_block`; the file name in both packages.
fn file_stem(name: &str) -> String {
    let mut out = String::with_capacity(name.len() + 2);
    for (index, ch) in name.char_indices() {
        if ch.is_uppercase() && index > 0 {
            out.push('_');
        }
        out.extend(ch.to_lowercase());
    }
    out
}

/// `line-numbers` → `LineNumbers`; `2` → `_2`, because a Rust variant cannot
/// start with a digit and a heading level legitimately is one.
fn pascal(name: &str) -> String {
    let mut out = String::with_capacity(name.len());
    if name.chars().next().is_some_and(|c| c.is_ascii_digit()) {
        out.push('_');
    }
    for word in name.split(['-', '_', ' ']) {
        let mut chars = word.chars();
        if let Some(first) = chars.next() {
            out.extend(first.to_uppercase());
            out.push_str(chars.as_str());
        }
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    fn shipped() -> Vec<Component> {
        let root = std::path::PathBuf::from(concat!(env!("CARGO_MANIFEST_DIR"), "/../.."));
        noctua_design_spec::component::load_all(root.join("spec/components")).expect("components")
    }

    /// The whole point: one spec, two implementations, same props.
    #[test]
    fn both_languages_declare_the_same_props() {
        for component in shipped() {
            let files = emit(std::slice::from_ref(&component));
            let stem = file_stem(&component.meta.name);
            let rust = &files
                .iter()
                .find(|f| f.path.ends_with(&format!("{stem}.rs")))
                .expect("rust")
                .contents;
            let js = &files
                .iter()
                .find(|f| f.path.ends_with(&format!("{stem}.js")))
                .expect("js")
                .contents;

            for (prop, _) in component.attributes() {
                assert!(
                    rust.contains(&format!("{}: ", identifier(prop))),
                    "{} is missing {prop} in Rust",
                    component.meta.name
                );
                assert!(
                    js.contains(&camel(prop)),
                    "{} is missing {prop} in React",
                    component.meta.name
                );
            }
        }
    }

    #[test]
    fn every_component_reaches_both_indexes() {
        let components = shipped();
        let files = emit(&components);
        let rust = &files
            .iter()
            .find(|f| f.path.ends_with("components/mod.rs"))
            .expect("mod.rs")
            .contents;
        let js = &files
            .iter()
            .find(|f| f.path.ends_with("components/index.js"))
            .expect("index.js")
            .contents;
        for component in &components {
            assert!(
                rust.contains(&component.meta.name),
                "{}",
                component.meta.name
            );
            assert!(js.contains(&component.meta.name), "{}", component.meta.name);
        }
    }

    #[test]
    fn names_become_files_and_types() {
        assert_eq!(file_stem("CodeBlock"), "code_block");
        assert_eq!(file_stem("Kbd"), "kbd");
        assert_eq!(pascal("line-numbers"), "LineNumbers");
        // A heading level is a digit, and a Rust variant cannot start with one.
        assert_eq!(pascal("2"), "_2");
    }
}
