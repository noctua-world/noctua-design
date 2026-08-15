//! The bridge from an example's data to the real Dioxus components.
//!
//! # What this is for
//!
//! An example on the documentation site is a tree — see `docs-site/src/tree.rs`
//! for why. A tree names a component with a **string**, because the same tree
//! has to be readable by JavaScript, and that string has to become a typed
//! `rsx!` call somewhere.
//!
//! It becomes one here, generated from `spec/components/<name>.toml`, which is
//! the only place that knows every prop's name, kind, default and whether it is
//! required. Hand-writing this file would be 86 chances to set a prop from the
//! wrong key or forget a default, and it would go stale the first time a spec
//! changed — which is the whole argument `props.rs` already makes about the two
//! implementations, one layer up.
//!
//! # Why the locals carry a trailing underscore
//!
//! `TableHeaderCell` declares a prop named `scope`, and this file threads a
//! `Scope` through every call. `let scope = …` would shadow the parameter and
//! the next line would not compile — or worse, would compile against the wrong
//! thing. The React half solved the same collision the same way years of
//! `const size_ = size ?? "medium"` ago, so this follows it: every local is the
//! prop's name with an underscore after it, and no spec prop can end in one.

use std::collections::BTreeSet;
use std::fmt::Write as _;

use noctua_design_spec::Component;
use noctua_design_spec::component::Kind;

use crate::props::{file_stem, pascal};
use crate::{COMPONENT_SPEC, CommentStyle, EmittedFile, controls, header_from, identifier};

/// The interpreter, as one file.
#[must_use]
pub fn emit(components: &[Component]) -> Vec<EmittedFile> {
    vec![EmittedFile::new(
        "docs-site/src/generated/interpreter.rs",
        interpreter(components),
    )]
}

/// Clippy's ceiling for one function, in the dispatch's own arms.
const LONG: usize = 100;

/// How many props make a component's function long enough to need the allow.
const LONG_ENOUGH: usize = 20;

fn interpreter(components: &[Component]) -> String {
    let mut out = header_from(CommentStyle::Line("//!"), COMPONENT_SPEC);
    out.push_str(
        "//!\n\
         //! An example's tree, rendered with the real components.\n\
         //!\n\
         //! One function per component and a dispatch over the name. See\n\
         //! `crate::tree` for what a tree is and why an example is one.\n\n",
    );

    // THE LOCALS ARE THE CONTRACT'S PROP NAMES, and the contract has both
    // `remove_label` and `removed_label` on `ColorPicker`. `similar_names` is a
    // lint about naming taste, and these names are not this file's to choose:
    // renaming them here would put a second spelling beside the spec's.
    out.push_str(
        "#![allow(\n    clippy::similar_names,\n    reason = \"the locals are the \
         contract's own prop names\"\n)]\n\n",
    );
    out.push_str("use dioxus::prelude::*;\n");
    out.push_str(&imports(components));
    out.push_str("\nuse crate::tree::{Node, Scope, Value, render};\n\n");

    dispatch(&mut out, components);
    for component in components {
        out.push('\n');
        out.push_str(&body(component));
    }
    out
}

/// Everything the file names, in one `use`.
///
/// Explicit rather than a glob: `clippy::wildcard_imports` is pedantic and this
/// workspace runs pedantic as a warning with `-D warnings` over it, so a glob
/// would fail the build rather than merely read loosely.
fn imports(components: &[Component]) -> String {
    let mut names = BTreeSet::new();
    for component in components {
        names.insert(component.meta.name.clone());
        for (prop_name, prop) in component.attributes() {
            if prop.kind == Kind::Enum {
                names.insert(format!("{}{}", component.meta.name, pascal(prop_name)));
            }
        }
    }

    let mut out = String::from("use noctua_design::components::{\n");
    for name in names {
        let _ = writeln!(out, "    {name},");
    }
    out.push_str("};\n");
    out
}

/// The dispatch over a component's name.
fn dispatch(out: &mut String, components: &[Component]) {
    let mut arms = String::new();
    for component in components {
        let _ = writeln!(
            arms,
            "        \"{}\" => {}(props, children, scope),",
            component.meta.name,
            function(component)
        );
    }

    out.push_str("/// Renders one component of a tree.\n");
    if arms.lines().count() > LONG {
        out.push_str(
            "#[allow(\n    clippy::too_many_lines,\n    reason = \"one arm per component, \
             and the component set is the point\"\n)]\n",
        );
    }
    out.push_str(
        "pub(crate) fn component(\n    \
         name: &str,\n    \
         props: &'static [(&'static str, Value)],\n    \
         children: &'static [Node],\n    \
         scope: Scope,\n\
         ) -> Element {\n    \
         match name {\n",
    );
    out.push_str(&arms);
    out.push_str(
        "        // A name the catalogue does not carry draws nothing here, and\n        \
         // `every_component_a_tree_names_exists` refuses it at build time — so\n        \
         // this arm is what makes that gate the only way to reach a typo.\n        \
         _ => rsx! {},\n    }\n}\n",
    );
}

/// One component's function.
fn body(component: &Component) -> String {
    let name = &component.meta.name;
    let mut reads = String::new();
    let mut call = String::new();

    for (prop_name, prop) in component.attributes() {
        let field = identifier(prop_name);
        let local = format!("{field}_");
        let _ = writeln!(
            reads,
            "    let {local} = {};",
            read(component, prop_name, prop)
        );
        let _ = writeln!(call, "            {field}: {local},");
    }
    let _ = writeln!(
        reads,
        "    let class_ = scope.find(props, \"class\").unwrap_or_default();"
    );
    let _ = writeln!(call, "            class: class_,");

    let inside = if component.takes_children() {
        "            {render(children, scope)}\n"
    } else {
        ""
    };

    // WIDE COMPONENTS GET THE ALLOW, COUNTED IN PROPS RATHER THAN LINES.
    //
    // Estimating the formatted length was tried twice and got it wrong twice:
    // rustfmt breaks a method chain well before the column limit, so a written
    // 74 lines came back as 111. Props are the thing that is actually known
    // here, and the relationship is monotone. Twenty is under the narrowest
    // component that has ever crossed the ceiling, so the allow is present
    // wherever it could be needed — and an `#[allow]` that never fires is
    // silent, where a missing one fails the build.
    let allow = if component.attributes().count() > LONG_ENOUGH {
        "#[allow(\n    clippy::too_many_lines,\n    reason = \"one line per declared prop, \
         and the prop set is the contract\"\n)]\n"
    } else {
        ""
    };

    // A component taking no content never reads its children, and an unused
    // parameter is a warning this workspace treats as an error.
    let taken = if component.takes_children() {
        "children"
    } else {
        "_children"
    };

    format!(
        "/// `{name}`, from a tree.\n\
         {allow}\
         fn {}(\n    \
         props: &'static [(&'static str, Value)],\n    \
         {taken}: &'static [Node],\n    \
         scope: Scope,\n\
         ) -> Element {{\n\
         {reads}\
         \n    rsx! {{\n        \
         {name} {{\n\
         {call}\
         {inside}        \
         }}\n    }}\n}}\n",
        function(component)
    )
}

/// How one prop is read out of a tree.
///
/// The whole type surface, and it is closed: the spec's `type` is `text`,
/// `bool`, `enum` or `children`, and the only Rust type outside those four is
/// `IconName`, which eleven components take where the spec says `text`.
fn read(
    component: &Component,
    prop_name: &str,
    prop: &noctua_design_spec::component::Prop,
) -> String {
    let name = &component.meta.name;
    let required =
        format!("expect(\"{name}.{prop_name} is required and this tree does not set it\")");

    match prop.kind {
        Kind::Enum => {
            let ty = format!("{name}{}", pascal(prop_name));
            let pick = format!("scope.pick(props, \"{prop_name}\", &{ty}::ALL, {ty}::as_str)");
            // An enum with no declared default is `Option<T>` on the component
            // and has no `Default` to fall back to — the absence is itself an
            // answer there, which is what `Tag { pressed }` established.
            if prop.default.is_some() {
                format!("{pick}.unwrap_or_default()")
            } else {
                pick
            }
        }
        // A bool's default is a VALUE, not `Default::default()` — the drift
        // `props.rs` documents at length, where `Disclosure.marker` and
        // `CodeBlock.copy` default to `true` and the generator said `false`.
        Kind::Bool => format!(
            "scope.flag(props, \"{prop_name}\").unwrap_or({})",
            prop.default.as_deref().unwrap_or("false")
        ),
        Kind::Text if controls::dioxus_type(component, prop_name).is_some() => {
            let icon = format!("scope.icon(props, \"{prop_name}\")");
            if prop.required {
                format!("{icon}.{required}")
            } else {
                icon
            }
        }
        Kind::Text => {
            let find = format!("scope.find(props, \"{prop_name}\")");
            match (prop.default.as_deref(), prop.required) {
                // A text prop's default is a value too, for the same reason.
                (Some(default), _) => {
                    format!("{find}.unwrap_or_else(|| {default:?}.to_owned())")
                }
                (None, true) => format!("{find}.{required}"),
                (None, false) => find,
            }
        }
        // Filtered out by `attributes()`; the content is `children`.
        Kind::Children => unreachable!("children is not an attribute"),
    }
}

/// The function name for a component: `SelectOption` becomes `select_option`.
fn function(component: &Component) -> String {
    file_stem(&component.meta.name)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn shipped() -> Vec<Component> {
        let root = std::path::PathBuf::from(concat!(env!("CARGO_MANIFEST_DIR"), "/../.."));
        noctua_design_spec::component::load_all(root.join("spec/components")).expect("components")
    }

    #[test]
    fn no_prop_ends_in_the_underscore_the_locals_use() {
        // The locals are `<prop>_`, which is only unambiguous while no prop
        // ends in one. A prop that did would collide with the one beside it.
        for component in shipped() {
            for (prop_name, _) in component.attributes() {
                assert!(
                    !identifier(prop_name).ends_with('_'),
                    "{}.{prop_name} ends in an underscore, which the interpreter's \
                     locals use to stay clear of prop names like `scope`",
                    component.meta.name
                );
            }
        }
    }

    #[test]
    fn every_component_reaches_the_dispatch() {
        let components = shipped();
        let source = interpreter(&components);
        for component in &components {
            assert!(
                source.contains(&format!("\"{}\" => ", component.meta.name)),
                "{} has no arm in the interpreter's dispatch",
                component.meta.name
            );
            assert!(
                source.contains(&format!("fn {}(", function(component))),
                "{} has no function in the interpreter",
                component.meta.name
            );
        }
    }

    #[test]
    fn every_declared_prop_is_read_and_passed() {
        // The defect this exists for is a generator that reads a prop into a
        // local and never passes it: it compiles, it warns about nothing, and
        // the prop is inert on every example that sets it.
        for component in shipped() {
            let source = body(&component);
            for (prop_name, _) in component.attributes() {
                let field = identifier(prop_name);
                assert!(
                    source.contains(&format!("let {field}_ = ")),
                    "{}.{prop_name} is never read",
                    component.meta.name
                );
                assert!(
                    source.contains(&format!("{field}: {field}_,")),
                    "{}.{prop_name} is read and never passed",
                    component.meta.name
                );
            }
        }
    }

    #[test]
    fn a_declared_default_reaches_the_interpreter() {
        // Name-anchored, so another prop's fallback cannot satisfy it — the
        // rule `every_declared_default_reaches_the_generated_react` follows.
        for component in shipped() {
            let source = body(&component);
            for (prop_name, prop) in component.attributes() {
                let Some(default) = prop.default.as_deref() else {
                    continue;
                };
                let expected = match prop.kind {
                    Kind::Bool => format!("unwrap_or({default})"),
                    Kind::Text => format!("unwrap_or_else(|| {default:?}.to_owned())"),
                    _ => "unwrap_or_default()".to_owned(),
                };
                assert!(
                    source.contains(&expected),
                    "{}.{prop_name} defaults to `{default}` in the spec, and the \
                     interpreter never says `{expected}`",
                    component.meta.name
                );
            }
        }
    }
}
