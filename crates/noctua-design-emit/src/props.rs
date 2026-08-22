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
//!
//! # What a contract may claim, and what checks the claim
//!
//! **A declared `attribute` is a claim about the markup**, and [`rust_element`]
//! keeps it only for a DERIVED body. A component written by hand in
//! `controls.rs` never reaches that function, so a contract can declare a
//! `data-nd-*` that no body writes and no script reads while the site's
//! generated markup snippet prints it as though it were there. Nothing else
//! reports it, because the values do reach the DOM — as labels, as tab names,
//! as text. `every_declared_attribute_is_written_by_both_bodies` reads the
//! emitted source, so it covers both kinds of body.
//!
//! **A prop may choose the ELEMENT, and that is declared**: `elements = [...]`
//! beside `values`, position by position. At most one prop per component may,
//! and a prop that switches the element carries no attribute. Without the
//! declaration a prop that promises an element renders the default one, silently
//! — accepted, documented, and inert.
//!
//! **A prop that does nothing is a defect, not a stub.** It is not a compile
//! error and not visible in review, so check that a prop reaches the markup
//! before shipping it — and check it UNDER THE OTHER PROPS, because a value
//! inert under another prop's default is the same defect one step further away.
//!
//! **No prop value may name a domain.** `subtotal | total` is `subtle |
//! strong`: a table of servers has no subtotal, and the two weights are *where
//! a section ends* and *which line is the answer*.
//!
//! **A prop may not be named after a reserved word** in either language, and
//! the spec refuses one by name. Escaping to `r#loop` would work and would be
//! one prop with two spellings across the two packages, which is the thing they
//! exist not to have.

use std::fmt::Write as _;

use noctua_design_spec::component::{Component, Kind, Prop};
use noctua_design_spec::icons::Resolved as Icon;

use crate::controls;
use crate::{COMPONENT_SPEC, CommentStyle, EmittedFile, camel, header_from, identifier};

/// Emits every component in both languages, plus each package's module index.
#[must_use]
pub fn emit(components: &[Component], drawings: &[Icon]) -> Vec<EmittedFile> {
    let mut files = Vec::new();
    for component in components {
        let file = file_stem(&component.meta.name);
        files.push(EmittedFile::new(
            format!("crates/noctua-design/src/components/{file}.rs"),
            dioxus(component),
        ));
        files.push(EmittedFile::new(
            format!("packages/react/src/components/{file}.js"),
            react(component, drawings),
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
    out.push_str(&controls::allows(component));
    out.push_str("#[component]\n");
    writeln!(out, "pub fn {name}(").expect("write");

    for (prop_name, prop) in component.attributes() {
        // A PROP'S DOC MAY BE SEVERAL PARAGRAPHS, and one `///` in front of
        // the first line of several is a file that does not parse. `autoplay`
        // is the first prop long enough to need paragraphs; before it, this
        // interpolated whatever was in the spec and a newline in it produced a
        // generated file rustfmt rejected with a diagnostic about a backtick.
        for line in doc_lines(&prop.doc) {
            writeln!(out, "    ///{line}").expect("write");
        }
        let ty = controls::dioxus_type(component, prop_name)
            .map_or_else(|| rust_type(name, prop_name, prop), str::to_owned);
        if prop.kind == Kind::Bool {
            // A BOOL'S DEFAULT IS A VALUE, NOT `Default::default()`.
            //
            // `#[props(default)]` is `false` whatever the spec declares, and it
            // was emitted for every bool. React reads the same spec and writes
            // `marker ?? true`, so the two packages disagreed about
            // `Disclosure.marker` and `CodeBlock.copy` — the chevron was never
            // drawn and every code block shipped `data-nd-copy="false"`.
            //
            // That is precisely the drift `NOCTUA.md §9.3` exists to make
            // impossible, and it came from the generator rather than from
            // either implementation, which is why neither could be reviewed
            // into catching it.
            let default = prop.default.as_deref().unwrap_or("false");
            writeln!(
                out,
                "    #[props(default = {default})]\n    {}: {ty},",
                identifier(prop_name)
            )
            .expect("write");
        } else if prop.kind == Kind::Text && prop.default.is_some() {
            // A TEXT PROP'S DEFAULT IS ALSO A VALUE. `#[props(default)]` on a
            // `String` is the empty string whatever the spec declares — the
            // same defect the bool branch above documents, one type along, and
            // it survived because `Search.query` was the first text prop in the
            // repository to declare one. React reads `query ?? "q"` from the
            // same field, so the two packages would have disagreed about what
            // a search submits under.
            writeln!(
                out,
                "    #[props(default = {:?}.to_owned())]\n    {}: {ty},",
                prop.default.as_deref().unwrap_or_default(),
                identifier(prop_name)
            )
            .expect("write");
        } else if prop.default.is_some() {
            // An enum's default is the variant marked `#[default]`, written
            // from the same spec field, so `Default::default()` is the answer.
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
    // ONE LINE PER LINE, for the reason the struct field above learned first: a
    // `doc` of more than one paragraph written as a single `///` puts the second
    // paragraph into the source as code, and rustfmt rejects the whole file with
    // an error about an em dash. `Table.layout` is the prop that found it here.
    for line in doc_lines(&spec.doc) {
        writeln!(out, "///{line}").expect("write");
    }
    // `Default` ONLY WHEN THE SPEC NAMES ONE, and it never had to be said until
    // an enum prop declared no default: `#[derive(Default)]` on an enum with no
    // `#[default]` variant does not compile, so `Tag { pressed }` — where the
    // absence of the prop is itself an answer, and the prop is therefore
    // `Option<TagPressed>` — failed the build with an error about the derive
    // rather than about the spec. Every enum before it happened to have one.
    let derives = if spec.default.is_some() {
        "#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]\n"
    } else {
        "#[derive(Debug, Clone, Copy, PartialEq, Eq)]\n"
    };
    out.push_str(derives);
    out.push_str("pub enum ");
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

/// One doc comment's lines, each ready to sit behind a comment marker.
///
/// A leading space where there is text and nothing where there is not, because
/// `/// ` with a trailing space on a blank line is what `fmt --check` objects
/// to on the very next run.
fn doc_lines(doc: &str) -> impl Iterator<Item = String> + '_ {
    doc.trim().lines().map(|line| {
        let line = line.trim_end();
        if line.is_empty() {
            String::new()
        } else {
            format!(" {line}")
        }
    })
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
    let class = component.meta.class.as_deref().unwrap_or("");

    writeln!(out, "    let root = format!(\"{class} {{class}}\");").expect("write");
    out.push_str("    let root = root.trim().to_owned();\n\n");

    let Some((prop_name, prop)) = component.element_switch() else {
        out.push_str("    rsx! {\n");
        rust_element(out, component, &component.meta.element, 2);
        out.push_str("    }\n}\n");
        return;
    };

    // ONE ARM PER VALUE, because `rsx!` needs a literal element name.
    //
    // It is repetitive, and that costs nothing: this file is generated. What it
    // buys is a `level` prop that actually renders an `<h3>` — the alternative
    // was the prop being accepted, documented, and silently ignored, which is
    // what both props carrying `elements` used to do.
    out.push_str(
        "    // One arm per value: `rsx!` takes a literal element name, and this\n\
         \x20   // prop chooses the element rather than an attribute.\n\
         \x20   rsx! {\n",
    );
    writeln!(out, "        match {} {{", identifier(prop_name)).expect("write");
    for (value, element) in prop.values.iter().zip(&prop.elements) {
        writeln!(
            out,
            "            {}::{} => rsx! {{",
            format_args!("{}{}", component.meta.name, pascal(prop_name)),
            pascal(value)
        )
        .expect("write");
        rust_element(out, component, element, 4);
        out.push_str("            },\n");
    }
    out.push_str("        }\n    }\n}\n");
}

/// One element, with its class, its role and every attribute a prop becomes.
fn rust_element(out: &mut String, component: &Component, element: &str, depth: usize) {
    let pad = "    ".repeat(depth);
    writeln!(out, "{pad}{element} {{\n{pad}    class: \"{{root}}\",").expect("write");

    if let Some(role) = &component.a11y.role {
        writeln!(out, "{pad}    role: \"{role}\",").expect("write");
    }
    for (prop_name, prop) in component.attributes() {
        let Some(attribute) = &prop.attribute else {
            continue;
        };
        let field = identifier(prop_name);
        match prop.kind {
            Kind::Enum => {
                writeln!(out, "{pad}    \"{attribute}\": {field}.as_str(),").expect("write");
            }
            Kind::Bool => writeln!(
                out,
                "{pad}    \"{attribute}\": if {field} {{ \"true\" }} else {{ \"false\" }},"
            )
            .expect("write"),
            _ => writeln!(out, "{pad}    \"{attribute}\": {field},").expect("write"),
        }
    }
    if component.takes_children() {
        writeln!(out, "{pad}    {{children}}").expect("write");
    }
    // AFTER THE CONTENT, WHICH IS WHAT `inline` MEANS. A derived body is one
    // element holding its children, so the only room it has is the end of the
    // run — and that is where a mark on a heading or a paragraph belongs. It
    // costs `Heading` nothing: the six-arm element match is still generated,
    // and hand-writing it to add three lines would give up the
    // parity-by-construction the whole of this module is for.
    out.push_str(&controls::ai_block_dioxus(
        component,
        &format!("{pad}    "),
        "nd-ai-slot-inline",
    ));
    writeln!(out, "{pad}}}").expect("write");
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

/// Every word JavaScript refuses as a binding name inside a module.
///
/// A `.js` THIS EMITTER WRITES HAS TO PARSE, and `Table.export` is the prop
/// that proved nothing was checking. `export function Table({ …, export, … })`
/// is a `SyntaxError` on the destructure — `export` is a keyword, not an
/// identifier — so the whole module failed to load, and it had failed for an
/// unknown length of time with `cargo xtask check` green throughout.
///
/// The whole list rather than the one word that bit: the next spec to name a
/// prop `class`, `default`, `new`, `static` or `interface` would land in
/// exactly the same place, and a list of one is a list that gets extended one
/// production incident at a time.
///
/// Everything reserved in a module, which is strict-mode code: the keywords,
/// the future-reserved words strict mode adds, and `arguments` and `eval`,
/// which strict mode refuses as binding targets specifically.
const RESERVED: [&str; 48] = [
    "arguments",
    "await",
    "break",
    "case",
    "catch",
    "class",
    "const",
    "continue",
    "debugger",
    "default",
    "delete",
    "do",
    "else",
    "enum",
    "eval",
    "export",
    "extends",
    "false",
    "finally",
    "for",
    "function",
    "if",
    "implements",
    "import",
    "in",
    "instanceof",
    "interface",
    "let",
    "new",
    "null",
    "package",
    "private",
    "protected",
    "public",
    "return",
    "static",
    "super",
    "switch",
    "this",
    "throw",
    "true",
    "try",
    "typeof",
    "var",
    "void",
    "while",
    "with",
    "yield",
];

/// The identifier the generated function binds a prop to.
///
/// `camel` for every prop JavaScript will take as a name, and a leading
/// underscore for the ones it will not — the same escape `pascal` already makes
/// for a Rust variant that would otherwise start with a digit.
///
/// The PROP is not renamed, only the binding: a caller still writes
/// `<Table export />`, because a keyword is a perfectly ordinary property name
/// and only a *binding* is refused. Renaming the prop would have made the two
/// packages declare different surfaces, which is the one thing this file exists
/// to prevent.
#[must_use]
fn js_local(prop: &str) -> String {
    let name = camel(prop);
    if RESERVED.contains(&name.as_str()) {
        format!("_{name}")
    } else {
        name
    }
}

/// One entry in the destructuring parameter list.
///
/// `export: _export` where the prop's own name is a keyword, and the bare name
/// everywhere else — so the object the caller passes is unchanged and only the
/// local identifier moves.
#[must_use]
fn js_param(prop: &str) -> String {
    let name = camel(prop);
    let local = js_local(prop);
    if local == name {
        name
    } else {
        format!("{name}: {local}")
    }
}

/// The expression React is handed as the tag, and any statement it needs first.
///
/// A lookup rather than a switch: React takes the tag as a value, so the table
/// IS the code. Returns `"div"` — quoted, ready to interpolate — when no prop
/// chooses the element.
fn react_element(out: &mut String, component: &Component) -> String {
    let Some((prop_name, prop)) = component.element_switch() else {
        return format!("\"{}\"", component.meta.element);
    };
    let field = js_local(prop_name);
    let table = prop
        .values
        .iter()
        .zip(&prop.elements)
        .map(|(value, element)| format!("\"{value}\": \"{element}\""))
        .collect::<Vec<_>>()
        .join(", ");
    let default = prop.default.as_deref().map_or_else(
        || component.meta.element.clone(),
        |value| component.element_for(value).to_owned(),
    );
    writeln!(
        out,
        "  // This prop chooses the element rather than an attribute.\n  \
         const element = {{ {table} }}[{field}_] ?? \"{default}\";"
    )
    .expect("write");
    "element".to_owned()
}

fn react(component: &Component, drawings: &[Icon]) -> String {
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

    // AND WHATEVER THE CUSTOM BODY NEEDS. `controls::react_imports` has existed
    // since the first custom React body and was never called from anywhere —
    // so `button.js`, `disclosure.js` and `select_option.js` all shipped
    // `h(Icon, …)` with no `Icon` in scope. Nothing caught it because nothing
    // in this repository had ever rendered a React component outside a browser
    // page that fetched a pre-built bundle. `CodeBlock` is what finally did:
    // it reads its grammar tables at render time, so an absent import is not a
    // wrong icon, it is a `ReferenceError` on every block.
    out.push_str(&controls::react_imports(component));
    // And the one a host gains from `[meta] ai`, which its body did not ask
    // for and cannot: the fragment is written by `controls::ai_block_react`.
    out.push_str(&controls::ai_imports_react(component));
    // And the drawings the body reaches for, read out of the body itself.
    out.push_str(&controls::react_icon_imports(component, drawings));

    // NO `const defaults = {…}` OBJECT.
    //
    // One was emitted here for every component that had a default, and **not a
    // single generated file ever read it**: the generic path writes
    // `const x_ = x ?? "value"` from the same spec field, and a custom body in
    // `controls.rs` writes its own. Sixteen shipped React components carried it
    // as dead code, and — worse than dead — it was a second place one default
    // was written, with nothing asserting the two agreed.
    //
    // The default reaches the caller through the `??` below and through the
    // `.d.ts`. That is one place, which is the number this repository is built
    // around.

    let params: Vec<String> = component
        .attributes()
        .map(|(prop_name, _)| js_param(prop_name))
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
                js_local(prop_name)
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
            format!("{}_", js_local(prop_name))
        } else {
            js_local(prop_name)
        };
        writeln!(
            out,
            "  if ({value} !== undefined) props[\"{attribute}\"] = String({value});"
        )
        .expect("write");
    }

    let element = react_element(&mut out, component);

    // See the Dioxus half: the mark goes after the content, which is the only
    // room a derived body has.
    let mark = controls::ai_block_react(component, "    ", "nd-ai-slot-inline");
    let mark = if mark.is_empty() {
        String::new()
    } else {
        format!(",\n    {mark}")
    };

    if component.takes_children() {
        writeln!(out, "  return h({element}, props, children{mark});").expect("write");
    } else if mark.is_empty() {
        writeln!(out, "  return h({element}, props);").expect("write");
    } else {
        writeln!(out, "  return h({element}, props{mark});").expect("write");
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
        // The same, in the other language's comment syntax.
        let doc: Vec<String> = doc_lines(&prop.doc).collect();
        if doc.len() == 1 {
            writeln!(out, "  /** {} */", prop.doc.trim()).expect("write");
        } else {
            out.push_str("  /**\n");
            for line in doc {
                writeln!(out, "   *{line}").expect("write");
            }
            out.push_str("   */\n");
        }
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
///
/// The contract reader's own, rather than a second copy of the split. It was a
/// second copy, and the two agreed until a name arrived with two capitals in a
/// row — at which point one of them would have named a file the other could not
/// find.
pub(crate) fn file_stem(name: &str) -> String {
    noctua_design_spec::component::file_stem(name)
}

/// `line-numbers` → `LineNumbers`; `2` → `_2`, because a Rust variant cannot
/// start with a digit and a heading level legitimately is one.
pub(crate) fn pascal(name: &str) -> String {
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
    /// A DECLARED DEFAULT IS A VALUE, WHATEVER THE PROP'S TYPE IS.
    ///
    /// `#[props(default)]` is `Default::default()` — `false` for a bool and the
    /// empty string for a text prop — and it was emitted for both. The bool
    /// half of that shipped: `Disclosure.marker` and `CodeBlock.copy` disagreed
    /// between the two packages, because React reads `marker ?? true` from the
    /// same spec field. The text half survived until `Search.query` became the
    /// first text prop in the repository to declare one.
    ///
    /// This asks the emitted source rather than the branch, so it holds for a
    /// type nobody has added yet.
    #[test]
    fn a_declared_default_reaches_the_generated_rust() {
        let root = std::path::PathBuf::from(concat!(env!("CARGO_MANIFEST_DIR"), "/../.."));
        let components = noctua_design_spec::component::load_all(root.join("spec/components"))
            .expect("the shipped components");

        for component in &components {
            let source = dioxus(component);
            for (name, prop) in component.attributes() {
                let Some(default) = prop.default.as_deref() else {
                    continue;
                };
                if prop.kind == Kind::Enum {
                    // An enum's default is the variant carrying `#[default]`,
                    // which `Default::default()` genuinely resolves to.
                    continue;
                }
                // THE WHOLE LINE, NOT A SUBSTRING OF ONE. This searched for
                // `expanded: ` and found `icon_expanded: ` first, then read the
                // attribute above THAT — so a component declaring both failed
                // a test about a default it had written correctly. The same
                // class of defect as an index guard that matched `rem` inside
                // `remove_label`: a name is a whole token or it is a guess.
                let field = format!("{}: ", identifier(name));
                let at = source
                    .lines()
                    .scan(0usize, |offset, line| {
                        let here = *offset;
                        *offset += line.len() + 1;
                        Some((here, line))
                    })
                    .find(|(_, line)| line.trim_start().starts_with(&field))
                    .map(|(at, _)| at)
                    .expect("the prop is declared");
                let head = &source[..at];
                let attribute = head.rfind("#[props(").map(|at| &head[at..]);
                assert!(
                    attribute.is_some_and(|text| text.contains(default)),
                    "{}.{name} declares `{default}` and the generated Rust says `{}`",
                    component.meta.name,
                    attribute.unwrap_or("nothing").trim()
                );
            }
        }
    }

    /// EVERY NAME A REACT BODY USES IS A NAME IT IMPORTS.
    ///
    /// `controls::react_imports` existed from the first custom React body and
    /// was called from nowhere, so `button.js`, `disclosure.js` and
    /// `select_option.js` all shipped `h(Icon, …)` with no `Icon` in scope.
    /// Nothing caught it for three components and several milestones, because
    /// nothing in this repository had ever rendered a React component outside a
    /// browser page that fetched a pre-built bundle — and a bundler resolves
    /// what it can see, which is the import that was not there.
    ///
    /// This asks the emitted text instead: if a file mentions one of the names
    /// that can only arrive by import, it has to import it. A `ReferenceError`
    /// on first render is not a thing to find in somebody else's application.
    #[test]
    fn every_react_body_imports_what_it_names() {
        // Names that exist only because something imported them. A local
        // `const` shadowing one of these would make this test lie, so each is
        // a name no generated body ever declares.
        const EXTERNAL: [&str; 6] = ["Icon", "GRAMMARS", "ALIASES", "scan(", "lines(", "Checkbox"];

        let root = std::path::PathBuf::from(concat!(env!("CARGO_MANIFEST_DIR"), "/../.."));
        let components = noctua_design_spec::component::load_all(root.join("spec/components"))
            .expect("the shipped components");
        let (_, drawings) = noctua_design_spec::icons::load(
            root.join("spec/icons.toml"),
            root.join("assets/icons"),
        )
        .expect("the shipped icon set");

        for component in &components {
            let source = react(component, &drawings);
            let head = source
                .split("export function")
                .next()
                .expect("a file has a head");
            for name in EXTERNAL {
                // A component's own name is not an import. `Checkbox`'s body
                // opens `export function Checkbox(` and would otherwise
                // accuse itself.
                if name.trim_end_matches('(') == component.meta.name {
                    continue;
                }
                let used = source[head.len()..].contains(name);
                let imported = head.contains(name.trim_end_matches('('));
                assert!(
                    !used || imported,
                    "{}'s React body uses `{name}` and does not import it",
                    component.meta.name
                );
            }

            // AND EVERY DRAWING IT DRAWS. Each icon is its own module, so a
            // body that renders one names an identifier that only an import can
            // supply. The imports are derived from the body — this is what
            // checks the derivation actually ran.
            let body = &source[head.len()..];
            for at in body
                .match_indices("icon: ")
                .map(|(at, _)| at + "icon: ".len())
            {
                let identifier: String = body[at..]
                    .chars()
                    .take_while(char::is_ascii_alphanumeric)
                    .collect();
                let Some(icon) = drawings.iter().find(|icon| icon.export == identifier) else {
                    continue;
                };
                assert!(
                    head.contains(&format!("icons/{}.js", icon.name)),
                    "{}'s React body draws `{}` and does not import it",
                    component.meta.name,
                    icon.name
                );
            }
        }
    }

    use super::*;

    fn shipped() -> Vec<Component> {
        let root = std::path::PathBuf::from(concat!(env!("CARGO_MANIFEST_DIR"), "/../.."));
        noctua_design_spec::component::load_all(root.join("spec/components")).expect("components")
    }

    /// EVERY HOST ACTUALLY DRAWS ITS MARK.
    ///
    /// `[meta] ai` grafts props onto a contract and the reference table then
    /// documents them. A body written by hand in `controls.rs` never reaches
    /// [`rust_element`], so a host whose body forgot its `// @ai` marker takes
    /// all of them and renders nothing — accepted, documented, and inert, which
    /// is this module's own definition of a defect, one mechanism further away.
    ///
    /// Nothing else catches it: the props exist, and they declare no attribute
    /// for `every_declared_attribute_is_written_by_both_bodies` to look for.
    #[test]
    fn every_host_draws_the_mark_in_both_languages() {
        let mut missing = Vec::new();
        let mut hosts = 0usize;
        for component in shipped() {
            if component.meta.ai.is_none() {
                continue;
            }
            hosts += 1;
            let name = &component.meta.name;
            if !dioxus(&component).contains("crate::components::AILabel {") {
                missing.push(format!(
                    "{name} hosts a mark and the Dioxus half draws none"
                ));
            }
            if !react(&component, &[]).contains("h(AILabel, {") {
                missing.push(format!("{name} hosts a mark and the React half draws none"));
            }
        }
        assert!(missing.is_empty(), "{}", missing.join("\n"));
        assert!(
            hosts > 0,
            "no component hosts a mark, so this proves nothing"
        );
    }

    /// A host that cannot hold a control does not declare the prop for one.
    ///
    /// `ai_pressable = false` is the whole of the difference, and the failure
    /// without it is silent: the mark renders as a `<span>` either way, so a
    /// caller who set `ai_note` on a `MenuItem` would get no note, no error and
    /// no reason.
    #[test]
    fn a_host_that_cannot_be_pressed_takes_no_note() {
        let mut quiet = 0usize;
        for component in shipped() {
            if component.meta.ai_pressable != Some(false) {
                continue;
            }
            quiet += 1;
            assert!(
                !component.prop.contains_key("ai_note"),
                "{} cannot hold a control and declares `ai_note`",
                component.meta.name
            );
            assert!(
                component.prop.contains_key("ai_mark"),
                "{} declares no `ai_mark`",
                component.meta.name
            );
            assert!(
                !react(&component, &[]).contains("note: aiNote"),
                "{}'s React half forwards a note it does not take",
                component.meta.name
            );
        }
        assert!(quiet > 0, "no host is unpressable, so this proves nothing");
    }

    /// The whole point: one spec, two implementations, same props.
    #[test]
    fn both_languages_declare_the_same_props() {
        for component in shipped() {
            let files = emit(std::slice::from_ref(&component), &[]);
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

    /// A bool's declared default has to reach the code, in both languages.
    ///
    /// `#[props(default)]` is `false` however the spec spells it, so every bool
    /// that defaults to `true` was silently `false` in Dioxus while React read
    /// the same spec and got it right. `Disclosure`'s chevron and `CodeBlock`'s
    /// copy control both shipped that way.
    #[test]
    fn a_bool_default_reaches_both_languages() {
        let mut checked = 0usize;
        for component in shipped() {
            let files = emit(std::slice::from_ref(&component), &[]);
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

            for (prop_name, prop) in component.attributes() {
                if prop.kind != Kind::Bool {
                    continue;
                }
                let default = prop.default.as_deref().unwrap_or("false");
                assert!(
                    rust.contains(&format!("#[props(default = {default})]")),
                    "{}.{prop_name} defaults to {default} in the spec, and Rust does not say so",
                    component.meta.name
                );
                assert!(
                    js.contains(&camel(prop_name)),
                    "{}.{prop_name} is missing from React",
                    component.meta.name
                );
                checked += 1;
            }
        }
        assert!(
            checked > 0,
            "no bool prop was checked, so this proves nothing"
        );
    }

    /// A DECLARED DEFAULT REACHES REACT TOO, AND IT IS THE DECLARED ONE.
    ///
    /// [`a_declared_default_reaches_the_generated_rust`] asks the Rust half and
    /// skips enums by design; [`a_bool_default_reaches_both_languages`] asserts
    /// the camel name appears in the React source and never its value. So the
    /// whole enum half of the React side was outside the net, and
    /// `TableHeaderCell.wrap` shipped `?? "wrap"` against a spec that says
    /// `nowrap` — every column heading in a React table wrapping where a Dioxus
    /// one does not.
    ///
    /// The assertion is name-anchored: `<prop> ?? <default>`, quoted or not,
    /// because a body writing `?? false` for a bool and `?? "medium"` for an
    /// enum are the same statement in two types. Searching the file for the
    /// value alone would be satisfied by a *different* prop's fallback.
    ///
    /// # The one exemption, and why it is not a hole
    ///
    /// A bool that defaults to `false` is exempt. React has no way to say
    /// "absent" other than `undefined`, and `undefined` and `false` are the
    /// same answer to every consumer of a bool: the attribute is not written,
    /// the branch is not taken, the element is not disabled. A bool defaulting
    /// to `true` is NOT exempt, because there absent and default differ — which
    /// is the drift `Disclosure.marker` and `CodeBlock.copy` already shipped
    /// once, in the other direction.
    #[test]
    fn every_declared_default_reaches_the_generated_react() {
        let mut checked = 0usize;
        for component in shipped() {
            let source = react(&component, &[]);
            for (prop_name, prop) in component.attributes() {
                let Some(default) = prop.default.as_deref() else {
                    continue;
                };
                if prop.kind == Kind::Bool && default == "false" {
                    continue;
                }
                let name = js_local(prop_name);
                let quoted = format!("{name} ?? \"{default}\"");
                let bare = format!("{name} ?? {default}");
                assert!(
                    source.contains(&quoted) || source.contains(&bare),
                    "{}.{prop_name} defaults to `{default}` in the spec, and the \
                     generated React never says `{name} ?? {default}`",
                    component.meta.name
                );
                checked += 1;
            }
        }
        assert!(
            checked > 0,
            "no default was checked, so this proves nothing"
        );
    }

    /// EVERY NAME A REACT BODY READS IS A NAME IT HAS.
    ///
    /// Three shipped components failed this and nothing noticed:
    /// `lineage.js` named `filter_`, `sets_`, `view_`, `root` and a dozen more
    /// that the generator does not emit; `lineage_figure.js` named `root`,
    /// `shape_`, `tone_` and `parallax_`; `color_picker.js` used `control` as
    /// an id and handed it to `Field`. Two of the three parsed perfectly — a
    /// free variable is a runtime `ReferenceError`, not a syntax error — so
    /// [`crate::lib::tests::every_emitted_javascript_parses`] would not have
    /// found them either.
    ///
    /// A real scope check is hard; this is the cheap sufficient one. It gathers
    /// what a file declares — its imports, its parameters, its `const`s and
    /// `let`s, its arrow parameters — without regard to which block each is in,
    /// and reports what is read and never among them. Flat scoping can only
    /// miss a defect, never invent one, which is the direction a guard should
    /// fail in.
    #[test]
    fn no_react_body_names_something_it_does_not_have() {
        // WITH THE REAL DRAWINGS. Each icon is its own module, so a body that
        // renders one reads an identifier only its import can supply — and
        // `react_icon_imports` derives those imports from the body. Passing an
        // empty set here would make this test pass by asking a body that draws
        // nothing whether it declares what it reads.
        let root = std::path::PathBuf::from(concat!(env!("CARGO_MANIFEST_DIR"), "/../.."));
        let (_, drawings) = noctua_design_spec::icons::load(
            root.join("spec/icons.toml"),
            root.join("assets/icons"),
        )
        .expect("the shipped icon set");
        for component in shipped() {
            let source = react(&component, &drawings);
            let missing = crate::javascript::undeclared(&source).unwrap_or_else(|problem| {
                panic!("{} does not lex — {problem}", component.meta.name)
            });
            assert!(
                missing.is_empty(),
                "{}'s React body reads {missing:?}, and declares, takes or imports none of them",
                component.meta.name
            );
        }
    }

    #[test]
    fn every_component_reaches_both_indexes() {
        let components = shipped();
        let files = emit(&components, &[]);
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

    /// A DECLARED `attribute` IS A CLAIM ABOUT THE MARKUP, AND BOTH BODIES
    /// HAVE TO KEEP IT.
    ///
    /// `rust_element` turns `attribute` into a real attribute, and it runs for
    /// derived bodies only. A component whose body is hand-written in
    /// `controls.rs` never reaches it, so its declarations were documentation
    /// that documented nothing: eighteen props across `ColorPicker`,
    /// `FileInput`, `Field`, `Meter` and `Progress` declared a `data-nd-*` no
    /// body wrote and no script read, while the site's generated markup
    /// snippet printed each one as though it were there. Nothing reported it,
    /// because the values do reach the DOM — as labels, as tab names, as text.
    ///
    /// This asks the emitted source, so it covers both kinds of body and any
    /// component added after it.
    #[test]
    fn every_declared_attribute_is_written_by_both_bodies() {
        // The handful of HTML attributes React spells differently. Written out
        // rather than derived, because the list is the DOM's and not a rule.
        fn react_spelling(attribute: &str) -> &str {
            match attribute {
                "colspan" => "colSpan",
                "rowspan" => "rowSpan",
                "tabindex" => "tabIndex",
                "maxlength" => "maxLength",
                "minlength" => "minLength",
                "readonly" => "readOnly",
                "autocomplete" => "autoComplete",
                "autofocus" => "autoFocus",
                "spellcheck" => "spellCheck",
                "novalidate" => "noValidate",
                "datetime" => "dateTime",
                "crossorigin" => "crossOrigin",
                "inputmode" => "inputMode",
                "for" => "htmlFor",
                "class" => "className",
                other => other,
            }
        }
        // An attribute reaches the markup quoted (`"data-nd-size": …`), bare
        // when it is already an identifier (`scope: …`), or as a shorthand
        // where the value carries the same name (`src,`). All three count, and
        // the name has to stand on its own — `size` must not be answered by
        // `data-nd-size`.
        fn written(source: &str, spelling: &str) -> bool {
            if source.contains(&format!("\"{spelling}\"")) {
                return true;
            }
            let bytes = source.as_bytes();
            source.match_indices(spelling).any(|(at, _)| {
                let before = at
                    .checked_sub(1)
                    .is_none_or(|i| !bytes[i].is_ascii_alphanumeric() && bytes[i] != b'_');
                let after = bytes
                    .get(at + spelling.len())
                    .is_some_and(|byte| *byte == b':' || *byte == b',');
                before && after
            })
        }

        // COLLECTED, NOT ASSERTED ONE AT A TIME. The first version stopped at
        // the first prop and the fix was eighteen rebuilds long.
        let mut missing = Vec::new();
        for component in shipped() {
            let rust = dioxus(&component);
            let js = react(&component, &[]);
            for (name, prop) in component.attributes() {
                let Some(attribute) = prop.attribute.as_deref() else {
                    continue;
                };
                let owner = &component.meta.name;
                if !written(&rust, attribute) {
                    missing.push(format!(
                        "{owner}.{name} declares `{attribute}`, Dioxus writes it nowhere"
                    ));
                }
                if !written(&js, attribute) && !written(&js, react_spelling(attribute)) {
                    missing.push(format!(
                        "{owner}.{name} declares `{attribute}`, React writes it nowhere"
                    ));
                }
            }
        }
        assert!(missing.is_empty(), "{}", missing.join("\n"));
    }

    #[test]
    fn names_become_files_and_types() {
        assert_eq!(file_stem("CodeBlock"), "code_block");
        assert_eq!(file_stem("Kbd"), "kbd");
        assert_eq!(pascal("line-numbers"), "LineNumbers");
        // A heading level is a digit, and a Rust variant cannot start with one.
        assert_eq!(pascal("2"), "_2");
    }

    /// A RUN OF CAPITALS IS ONE WORD, and the split used to be one per capital.
    ///
    /// `AILabel` came out as `a_i_label` — allow-reference: the name that must
    /// NOT exist is what this test is about — which is a file name, a slug, a
    /// message key and a URL all naming something no reader would look up. Both
    /// halves of the split are asserted here: the acronym, and every shape that
    /// was already right and had to stay right.
    #[test]
    fn an_acronym_is_one_word_and_nothing_else_moved() {
        use noctua_design_spec::component::slug;

        assert_eq!(file_stem("AILabel"), "ai_label");
        assert_eq!(slug("AILabel"), "ai-label");

        // The whole catalogue as it stood before that name arrived.
        for (name, stem) in [
            ("CodeBlock", "code_block"),
            ("TableHeaderCell", "table_header_cell"),
            ("OtpField", "otp_field"),
            ("VisuallyHidden", "visually_hidden"),
            ("Tab", "tab"),
        ] {
            assert_eq!(file_stem(name), stem, "{name}");
            assert_eq!(slug(name), stem.replace('_', "-"), "{name}");
        }

        // AND THE TWO SPELLINGS OF ONE FACT AGREE. The site derives a file name
        // from the slug rather than splitting the name again, so this relation
        // is the one holding that derivation up.
        for component in shipped() {
            let name = &component.meta.name;
            assert_eq!(slug(name).replace('-', "_"), file_stem(name), "{name}");
        }
    }
}
