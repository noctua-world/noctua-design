//! `ThemeScope`, in both packages, from one table.
//!
//! A themed subtree is the one component the system cannot do without, because
//! everything else in the theme layer is a stylesheet and an attribute. It is
//! generated rather than written twice for the reason `NOCTUA.md §9.3` gives:
//! the two implementations must be *behaviourally identical*, and a prop
//! surface transcribed by hand agrees with its counterpart only until somebody
//! adds the eighth setting to one of them.
//!
//! # Why there is no `spec/components/theme-scope.toml`
//!
//! Every other component will have one. This one must not, and the reason is
//! worth stating: `ThemeScope`'s props **are** the scopable settings, exactly
//! and only. A second file listing them again could disagree with
//! `spec/theme.toml` — a setting marked `scope = true` there and missing here
//! would be silently unreachable, which is precisely the class of drift the
//! spec layer exists to make impossible. So the prop list is read from the
//! setting table, and `scope = true` is the single place a setting becomes
//! scopable.

use std::fmt::Write as _;

use noctua_design_spec::{Setting, Theme};

use crate::{CommentStyle, EmittedFile, THEME_SPEC, camel, header_from, identifier};

/// Emits the Dioxus component, the React component, and its types.
#[must_use]
pub fn emit(theme: &Theme) -> Vec<EmittedFile> {
    let scopable: Vec<_> = theme.scopable().collect();
    vec![
        EmittedFile::new(
            "crates/noctua-design/src/theme.rs",
            dioxus(theme, &scopable),
        ),
        EmittedFile::new("packages/react/src/theme.js", react(theme, &scopable)),
        EmittedFile::new("packages/react/src/theme.d.ts", react_types(&scopable)),
    ]
}

/// The doc comment both implementations carry, minus the comment markers.
///
/// Written once because it is the *contract*, and a contract stated twice is a
/// contract that will one day say two things.
const CONTRACT: &[&str] = &[
    "A subtree with a theme of its own.",
    "",
    "The page may be dark ochre while this is light amber, and every",
    "component inside it — including a nested scope — resolves against this",
    "rather than against the document.",
    "",
    "# Every attribute goes on ONE element",
    "",
    "The color system's forced-mode rule is compound and same-element:",
    "",
    // Fenced as `text`, and that is not cosmetic: an indented block in a Rust
    // doc comment is a *doctest*, and rustdoc would try to compile this CSS.
    "```text",
    "[data-palette=\"blue-vivid\"][data-theme=\"dark\"] { … }",
    "```",
    "",
    "Split the palette onto a wrapper and the mode onto a child and that",
    "selector matches nothing. The child falls through to the *default*",
    "palette's unanchored [data-theme=\"dark\"] and silently paints the wrong",
    "colors — silently, because every token still resolves to something.",
    "",
    "# A prop left unset inherits",
    "",
    "Inheriting is resolved here, not in CSS: the scope copies the value its",
    "surroundings resolved to. That is what makes a nested scope that only",
    "changes the mode keep the palette its parent scope chose.",
    "",
    "# A scope that changes nothing renders nothing",
    "",
    "No element, no attributes, no wrapper in the tree. The semantic contract",
    "is over 1,700 declarations, and re-resolving it for a subtree that asked",
    "for the theme it already had is pure cost.",
];

// --- Dioxus ----------------------------------------------------------------

fn dioxus(theme: &Theme, scopable: &[(&String, &Setting)]) -> String {
    let mut out = header_from(CommentStyle::Line("//!"), THEME_SPEC);
    out.push_str(
        "//!\n\
         //! The themed-subtree component, generated from the setting table so that\n\
         //! it and its React counterpart cannot declare different props.\n\
         \n\
         use dioxus::prelude::*;\n\
         \n",
    );

    writeln!(
        out,
        "pub use noctua_design_theme::{{Resolved, ScopeRequest, {}}};\n",
        scopable
            .iter()
            .map(|(name, _)| pascal(name))
            .collect::<Vec<_>>()
            .join(", ")
    )
    .expect("write");

    dioxus_hook(&mut out);
    dioxus_component(&mut out, theme, scopable);
    out
}

fn dioxus_hook(out: &mut String) {
    out.push_str(
        "/// The theme in force at this point in the tree.\n\
         ///\n\
         /// Outside any [`ThemeScope`] this is the document's theme, which is what\n\
         /// the bootstrap stamped on `<html>` before the first paint.\n\
         ///\n\
         /// # Why a signal and not a plain value\n\
         ///\n\
         /// A scope's theme can change while it is mounted — the demonstration page\n\
         /// has controls that do exactly that. A plain value put into context is\n\
         /// captured once, on first render, and every descendant would keep reading\n\
         /// the theme the scope started with.\n\
         #[must_use]\n\
         pub fn use_theme() -> Resolved {\n\
         \x20   match try_consume_context::<Signal<Resolved>>() {\n\
         \x20       Some(theme) => theme(),\n\
         \x20       None => Resolved::default(),\n\
         \x20   }\n\
         }\n\
         \n",
    );
}

fn dioxus_component(out: &mut String, theme: &Theme, scopable: &[(&String, &Setting)]) {
    for line in CONTRACT {
        if line.is_empty() {
            out.push_str("///\n");
        } else {
            writeln!(out, "/// {line}").expect("write");
        }
    }

    out.push_str("#[component]\npub fn ThemeScope(\n");
    for (name, _) in scopable {
        writeln!(
            out,
            "    /// The `{name}` this scope asks for. `None` inherits.\n    \
             {}: Option<{}>,",
            identifier(name),
            pascal(name)
        )
        .expect("write");
    }
    out.push_str(
        "    /// Classes for the scope element. Ignored when the scope is a no-op,\n\
         \x20   /// because then there is no element to put them on.\n\
         \x20   #[props(default)]\n\
         \x20   class: String,\n\
         \x20   /// What the scope wraps.\n\
         \x20   children: Element,\n\
         ) -> Element {\n",
    );

    writeln!(
        out,
        "    let inherited = use_theme();\n    \
         let request = ScopeRequest {{\n{}    }};\n    \
         let resolved = request.resolve(inherited);",
        scopable
            .iter()
            .map(|(name, _)| identifier(name))
            .fold(String::new(), |mut acc, name| {
                let _ = writeln!(acc, "        {name},");
                acc
            })
    )
    .expect("write");

    out.push_str(
        "\n\
         \x20   // Provided before the early return, because a hook that runs on some\n\
         \x20   // renders and not others is the one thing Dioxus cannot recover from.\n\
         \x20   let mut published = use_context_provider(|| Signal::new(resolved));\n\
         \x20   use_effect(use_reactive!(|resolved| published.set(resolved)));\n\
         \n\
         \x20   let Some(attributes) = noctua_design_theme::scope_attributes(request, inherited)\n\
         \x20   else {\n\
         \x20       // Nothing changed, so nothing is wrapped. `children` is returned as\n\
         \x20       // it is rather than inside a bare <div>, which would still cost a\n\
         \x20       // box in every layout it passed through.\n\
         \x20       return rsx! { {children} };\n\
         \x20   };\n\
         \n\
         \x20   rsx! {\n\
         \x20       div {\n\
         \x20           class: \"{class}\",\n",
    );

    // The attribute *names* are literals here rather than the constants the
    // model exports, because `rsx!` needs a literal. A test asserts the two
    // agree, so the duplication cannot rot.
    writeln!(out, "            \"{}\": \"\",", theme.meta.scope_attribute).expect("write");
    for (name, setting) in scopable {
        let Some(attribute) = &setting.attribute else {
            continue;
        };
        // A setting with an `omit_when` arrives as `Option<&str>` rather than
        // `&str`, and `rsx!` drops the attribute on `None`. That is what makes
        // `system` the *absence* of `data-theme` rather than a value of it.
        writeln!(
            out,
            "            \"{attribute}\": attributes.{},",
            identifier(name)
        )
        .expect("write");
    }
    out.push_str("            {children}\n        }\n    }\n}\n");
}

// --- React -----------------------------------------------------------------

fn react(theme: &Theme, scopable: &[(&String, &Setting)]) -> String {
    let mut out = header_from(CommentStyle::Line("//"), THEME_SPEC);
    writeln!(
        out,
        "\n// Generated from {THEME_SPEC}, the same table the Dioxus component comes\n\
         // from. Neither writes a prop name or a default down, so the two cannot\n\
         // declare different surfaces.\n\
         //\n\
         // No JSX and no build step: `createElement` is what this package ships, so\n\
         // a consumer needs no transpiler and this file is the file that runs.\n"
    )
    .expect("write");

    out.push_str(
        "import { createContext, createElement, useContext, useMemo } from \"react\";\n\
         import {\n\
         \x20 resolveScope,\n\
         \x20 scopeAttributes,\n",
    );
    for (name, _) in scopable {
        writeln!(out, "  {},", camel(&format!("{name}-values"))).expect("write");
    }
    out.push_str("} from \"@noctua-world/design-tokens/theme.js\";\n\n");

    writeln!(
        out,
        "/** The theme a subtree inherits when no scope encloses it. */\n\
         const documentTheme = {{\n{}}};\n",
        scopable.iter().fold(String::new(), |mut acc, (name, _)| {
            let _ = writeln!(
                acc,
                "  {}: {}.default,",
                camel(name),
                camel(&format!("{name}-values"))
            );
            acc
        })
    )
    .expect("write");

    out.push_str(
        "const ThemeContext = createContext(documentTheme);\n\n\
         /** The theme in force at this point in the tree. */\n\
         export function useTheme() {\n\
         \x20 return useContext(ThemeContext);\n\
         }\n\n",
    );

    out.push_str("/**\n");
    for line in CONTRACT {
        if line.is_empty() {
            out.push_str(" *\n");
        } else {
            writeln!(out, " * {line}").expect("write");
        }
    }
    out.push_str(" */\n");

    writeln!(
        out,
        "export function ThemeScope({{ children, className, {} }}) {{",
        scopable
            .iter()
            .map(|(name, _)| camel(name))
            .collect::<Vec<_>>()
            .join(", ")
    )
    .expect("write");

    writeln!(
        out,
        "  const inherited = useContext(ThemeContext);\n  \
         const request = {{ {} }};",
        scopable
            .iter()
            .map(|(name, _)| camel(name))
            .collect::<Vec<_>>()
            .join(", ")
    )
    .expect("write");

    writeln!(
        out,
        "  const resolved = useMemo(\n    \
         () => resolveScope(request, inherited),\n    \
         // Every prop, listed: a dependency array built from `request` would be a\n    \
         // new object on every render and memoize nothing.\n    \
         [inherited, {}],\n  );\n",
        scopable
            .iter()
            .map(|(name, _)| camel(name))
            .collect::<Vec<_>>()
            .join(", ")
    )
    .expect("write");

    writeln!(
        out,
        "  const attributes = scopeAttributes(request, inherited);\n  \
         const inner =\n    \
         attributes === null\n      \
         ? children\n      \
         : createElement(\"div\", {{ className, ...attributes }}, children);\n\n  \
         return createElement(ThemeContext.Provider, {{ value: resolved }}, inner);\n\
         }}\n"
    )
    .expect("write");

    let _ = theme;
    out
}

fn react_types(scopable: &[(&String, &Setting)]) -> String {
    let mut out = header_from(CommentStyle::Line("//"), THEME_SPEC);
    out.push_str("\nimport type { ReactNode } from \"react\";\nimport type {\n");
    for (name, _) in scopable {
        writeln!(out, "  {},", pascal(name)).expect("write");
    }
    out.push_str(
        "  ScopeSettings,\n} from \"@noctua-world/design-tokens/theme.js\";\n\n\
         /** The theme in force at this point in the tree. */\n\
         export declare function useTheme(): ScopeSettings;\n\n\
         /** What a themed subtree asks for. An omitted prop inherits. */\n\
         export interface ThemeScopeProps {\n\
         \x20 children?: ReactNode;\n\
         \x20 className?: string;\n",
    );
    for (name, _) in scopable {
        writeln!(out, "  {}?: {};", camel(name), pascal(name)).expect("write");
    }
    out.push_str(
        "}\n\n\
         /** A subtree with a theme of its own. See theme.js for the contract. */\n\
         export declare function ThemeScope(props: ThemeScopeProps): JSX.Element;\n",
    );
    out
}

/// `data-theme` → `DataTheme`; `saturation` → `Saturation`.
fn pascal(name: &str) -> String {
    let mut out = String::with_capacity(name.len());
    for word in name.split(['-', '_']) {
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

    fn model() -> Theme {
        let root = std::path::PathBuf::from(concat!(env!("CARGO_MANIFEST_DIR"), "/../.."));
        let vendor = noctua_design_spec::vendor::load(&root).expect("vendor");
        noctua_design_spec::load_theme(root.join("spec/theme.toml"), &vendor).expect("theme")
    }

    fn file(path: &str) -> String {
        emit(&model())
            .into_iter()
            .find(|f| f.path == path)
            .unwrap_or_else(|| panic!("{path} was not emitted"))
            .contents
    }

    /// The whole point of generating both: the same props, spelled each
    /// language's way.
    #[test]
    fn both_implementations_declare_the_same_props() {
        let theme = model();
        let rust = file("crates/noctua-design/src/theme.rs");
        let js = file("packages/react/src/theme.js");
        let dts = file("packages/react/src/theme.d.ts");

        for (name, _) in theme.scopable() {
            assert!(
                rust.contains(&format!("{}: Option<", identifier(name))),
                "the Dioxus component has no `{name}` prop"
            );
            assert!(
                js.contains(&camel(name)),
                "the React component has no `{name}` prop"
            );
            assert!(
                dts.contains(&format!("{}?:", camel(name))),
                "the React types have no `{name}` prop"
            );
        }
    }

    /// A global setting reaching the prop surface would be a prop that silently
    /// does nothing, because its CSS is bound to `:root`.
    #[test]
    fn a_global_setting_is_not_a_prop() {
        let theme = model();
        let rust = file("crates/noctua-design/src/theme.rs");
        for (name, _) in theme.global() {
            assert!(
                !rust.contains(&format!("{}: Option<", identifier(name))),
                "`{name}` is global but reached the ThemeScope prop surface"
            );
        }
    }

    /// `rsx!` needs a literal attribute name, so the generated component spells
    /// them out. This is what stops that copy from rotting.
    #[test]
    fn the_stamped_attributes_are_the_ones_the_model_declares() {
        let theme = model();
        let rust = file("crates/noctua-design/src/theme.rs");
        assert!(rust.contains(&format!("\"{}\": \"\"", theme.meta.scope_attribute)));
        for (name, setting) in theme.scopable() {
            let Some(attribute) = &setting.attribute else {
                continue;
            };
            assert!(
                rust.contains(&format!("\"{attribute}\": attributes.")),
                "`{name}` is scopable and has an attribute, but the component never stamps it"
            );
        }
    }

    /// Both halves must return the children untouched when nothing changed —
    /// this is the assertion that a wrapper `<div>` did not creep back in.
    #[test]
    fn a_no_op_scope_wraps_nothing_in_either_language() {
        assert!(
            file("crates/noctua-design/src/theme.rs").contains("return rsx! { {children} };"),
            "the Dioxus component wraps a no-op scope"
        );
        assert!(
            file("packages/react/src/theme.js").contains("? children"),
            "the React component wraps a no-op scope"
        );
    }

    #[test]
    fn names_become_type_names() {
        assert_eq!(pascal("saturation"), "Saturation");
        assert_eq!(pascal("mode"), "Mode");
    }
}
