//! The icon set, in both languages, from one declaration and the drawings.
//!
//! | Output | For |
//! |---|---|
//! | `crates/noctua-design-icons/src/lib.rs` | the names and the drawings |
//! | `crates/noctua-design/src/icon.rs` | the Dioxus `Icon` |
//! | `packages/react/src/icon.js` + `.d.ts` | the React `Icon` |
//! | `packages/tokens/icons.js` + `.d.ts` | the names, for consumers with no framework |
//! | `dist/icons.json` | the manifest the documentation page reads |
//! | `assets/icons/ATTRIBUTION.md` | who drew them, and under what licence |
//!
//! # An enum, not a string, and that is the whole point
//!
//! The site shipped `Icon { name: "repository" }` against a `match` with a
//! `_ => ""` arm. It compiled, and it rendered `<path d="">` inside a 44×44
//! clickable box in the header of every page. Nothing could have caught it:
//! "an icon called repository" and "no icon" were the same value.
//!
//! Here the name is a generated enum in Rust and a generated union in
//! TypeScript, so that call is a type error in both languages. `NOCTUA.md §9.5`
//! asks for exactly this.
//!
//! # `aria-hidden` by default
//!
//! An icon is decoration until somebody says otherwise. `label: None` renders
//! `aria-hidden="true" focusable="false"`, which is right for the common case —
//! an icon beside a word. The case that needs a name is an icon-only control,
//! and that is `IconButton`'s job: its label is **required**, so a control with
//! no accessible name is not a thing a caller can construct.

use std::fmt::Write as _;

use noctua_design_spec::icons::{Icons, Paint, Resolved};

use crate::{CommentStyle, EmittedFile, header_from};

/// The icon specification path, as written in generated headers.
pub const ICON_SPEC: &str = "spec/icons.toml";

/// Emits the whole icon layer.
#[must_use]
pub fn emit(icons: &Icons, resolved: &[Resolved]) -> Vec<EmittedFile> {
    vec![
        EmittedFile::new("crates/noctua-design-icons/src/lib.rs", names(resolved)),
        EmittedFile::new("crates/noctua-design/src/icon.rs", dioxus(icons)),
        EmittedFile::new("packages/react/src/icon.js", react(icons)),
        EmittedFile::new("packages/react/src/icon.d.ts", react_types(resolved)),
        EmittedFile::new("packages/tokens/icons.js", tokens_js(resolved)),
        EmittedFile::new("packages/tokens/icons.d.ts", tokens_types(resolved)),
        EmittedFile::new("dist/icons.json", manifest(resolved)),
        EmittedFile::new("assets/icons/ATTRIBUTION.md", attribution(icons, resolved)),
    ]
}

// --- The names and the drawings --------------------------------------------

fn names(icons: &[Resolved]) -> String {
    let mut out = header_from(CommentStyle::Line("//!"), ICON_SPEC);
    out.push_str(
        "//!\n\
         //! Every icon the interface uses, as a name and a drawing.\n\
         //!\n\
         //! `no_std`: this is a table of string literals, and a consumer should not\n\
         //! take a standard library on to read one.\n\
         \n#![no_std]\n\n",
    );

    out.push_str("/// Every icon in the system.\n");
    out.push_str(
        "///\n\
         /// A generated enum rather than a name, so an icon that does not exist is a\n\
         /// compile error rather than an empty box on a page. There is deliberately\n\
         /// no `FromStr` and no `From<&str>`: a string that comes from anywhere but\n\
         /// this enum is exactly what shipped an invisible control.\n",
    );
    out.push_str("#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]\n");
    out.push_str("pub enum IconName {\n");
    for icon in icons {
        let _ = writeln!(out, "    /// {}\n    {},", icon.doc, icon.variant);
    }
    out.push_str("}\n\n");

    out.push_str("impl IconName {\n");
    let _ = writeln!(
        out,
        "    /// Every icon, in the order the spec declares them.\n    \
         pub const ALL: [Self; {}] = [",
        icons.len()
    );
    for icon in icons {
        let _ = writeln!(out, "        Self::{},", icon.variant);
    }
    out.push_str("    ];\n\n");

    out.push_str(
        "    /// Its name, as the spec spells it.\n    \
         #[must_use]\n    pub const fn as_str(self) -> &'static str {\n        match self {\n",
    );
    for icon in icons {
        let _ = writeln!(
            out,
            "            Self::{} => \"{}\",",
            icon.variant, icon.name
        );
    }
    out.push_str("        }\n    }\n\n");

    out.push_str(
        "    /// The drawing: the children of `<svg>`, and nothing else.\n    \
         #[must_use]\n    pub const fn body(self) -> &'static str {\n        match self {\n",
    );
    for icon in icons {
        let _ = writeln!(
            out,
            "            Self::{} => r#\"{}\"#,",
            icon.variant, icon.body
        );
    }
    out.push_str("        }\n    }\n\n");

    shared_tables(&mut out, icons);

    out.push_str(
        "    /// The icon of this name, for the documentation page and for tests.\n    \
         ///\n    /// Deliberately fallible and deliberately not `FromStr`: nothing in an\n    \
         /// interface should reach an icon by name.\n    \
         #[must_use]\n    pub fn parse(name: &str) -> Option<Self> {\n        \
         let mut index = 0;\n        while index < Self::ALL.len() {\n            \
         let icon = Self::ALL[index];\n            \
         if str_eq(icon.as_str(), name) {\n                return Some(icon);\n            }\n            \
         index += 1;\n        }\n        None\n    }\n}\n\n",
    );

    out.push_str(
        "/// `str::eq` without the standard library.\nfn str_eq(a: &str, b: &str) -> bool {\n    \
         a.as_bytes() == b.as_bytes()\n}\n",
    );
    out
}

/// The three tables where many icons legitimately agree.
///
/// One arm per icon even where the values repeat: collapsing the ones that
/// happen to agree today would make the next icon that disagrees a structural
/// change to this emitter rather than a line in the spec.
fn shared_tables(out: &mut String, icons: &[Resolved]) {
    const SAME_ARMS: &str = "    #[expect(\n        clippy::match_same_arms,\n        \
         reason = \"one arm per icon, generated from the spec\"\n    )]\n";

    let _ = write!(
        out,
        "    /// Its `viewBox`, taken from the drawing rather than assumed.\n\
         {SAME_ARMS}    #[must_use]\n    \
         pub const fn view_box(self) -> &'static str {{\n        match self {{\n"
    );
    for icon in icons {
        let _ = writeln!(
            out,
            "            Self::{} => \"{}\",",
            icon.variant, icon.view_box
        );
    }
    out.push_str("        }\n    }\n\n");

    let _ = write!(
        out,
        "    /// Whether it is drawn as an outline or as a solid shape.\n    \
         ///\n    /// Brand marks are solid; everything else is an outline.\n\
         {SAME_ARMS}    #[must_use]\n    \
         pub const fn filled(self) -> bool {{\n        match self {{\n"
    );
    for icon in icons {
        let _ = writeln!(
            out,
            "            Self::{} => {},",
            icon.variant,
            icon.paint == Paint::Fill
        );
    }
    out.push_str("        }\n    }\n\n");

    out.push_str(
        "    /// What it is for.\n    \
         #[must_use]\n    pub const fn doc(self) -> &'static str {\n        match self {\n",
    );
    for icon in icons {
        let _ = writeln!(
            out,
            "            Self::{} => \"{}\",",
            icon.variant,
            icon.doc.replace('"', "\\\"")
        );
    }
    out.push_str("        }\n    }\n\n");
}

// --- Dioxus ----------------------------------------------------------------

fn dioxus(icons: &Icons) -> String {
    let mut out = header_from(CommentStyle::Line("//!"), ICON_SPEC);
    let meta = &icons.meta;
    let _ = write!(
        out,
        "//!\n\
         //! The icon component.\n\
         //!\n\
         //! Generated from the same declaration as the React one, so the two cannot\n\
         //! disagree about a name, a size or what an unlabelled icon promises\n\
         //! assistive technology.\n\
         \n\
         use dioxus::prelude::*;\n\
         pub use noctua_design_icons::IconName;\n\
         \n\
         /// The rendered box.\n\
         #[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]\n\
         pub enum IconSize {{\n    \
         /// `{small}`\n    Small,\n    \
         #[default]\n    /// `{medium}`\n    Medium,\n    \
         /// `{large}`\n    Large,\n\
         }}\n\
         \n\
         impl IconSize {{\n    \
         /// Every value, in the order the spec declares them.\n    \
         pub const ALL: [Self; 3] = [Self::Small, Self::Medium, Self::Large];\n\
         \n    /// The value as it appears in the DOM.\n    \
         #[must_use]\n    pub const fn as_str(self) -> &'static str {{\n        \
         match self {{\n            Self::Small => \"small\",\n            \
         Self::Medium => \"medium\",\n            Self::Large => \"large\",\n        }}\n    }}\n\
         }}\n\n",
        small = meta.size_small,
        medium = meta.size_medium,
        large = meta.size_large,
    );

    let _ = write!(
        out,
        "/// An icon.\n\
         ///\n\
         /// `label` is what makes it available to a screen reader. Without one it is\n\
         /// `aria-hidden`, which is correct for an icon beside a word and wrong for an\n\
         /// icon on its own — so an icon-only control is `IconButton`, whose label is\n\
         /// required and cannot be forgotten.\n\
         #[component]\n\
         pub fn Icon(\n    \
         /// Which icon.\n    icon: IconName,\n    \
         /// Its accessible name. `None` hides it from assistive technology.\n    \
         #[props(default)]\n    label: Option<String>,\n    \
         /// The rendered box.\n    #[props(default)]\n    size: IconSize,\n    \
         /// Extra classes for the root element.\n    #[props(default)]\n    class: String,\n\
         ) -> Element {{\n    \
         let root = format!(\"nd-icon-glyph {{class}}\");\n    \
         let root = root.trim().to_owned();\n    \
         let named = label.is_some();\n\
         \n    rsx! {{\n        svg {{\n            \
         class: \"{{root}}\",\n            \
         \"data-nd-icon\": icon.as_str(),\n            \
         \"data-nd-size\": size.as_str(),\n            \
         view_box: icon.view_box(),\n            \
         width: \"1em\",\n            height: \"1em\",\n            \
         fill: if icon.filled() {{ \"currentColor\" }} else {{ \"none\" }},\n            \
         stroke: if icon.filled() {{ \"none\" }} else {{ \"currentColor\" }},\n            \
         stroke_width: \"{stroke}\",\n            \
         stroke_linecap: \"round\",\n            \
         stroke_linejoin: \"round\",\n            \
         // Quoted, not identifiers: `dioxus_elements::svg` has no typed\n            \
         // `role`, `aria-*` or `focusable`, and an `Option` renders nothing\n            \
         // when it is `None` — which is how an unlabelled icon carries\n            \
         // `aria-hidden` and a labelled one carries a name instead.\n            \
         \"role\": named.then_some(\"img\"),\n            \
         \"aria-label\": label,\n            \
         \"aria-hidden\": (!named).then_some(\"true\"),\n            \
         \"focusable\": \"false\",\n            \
         dangerous_inner_html: \"{{icon.body()}}\",\n        }}\n    }}\n}}\n",
        stroke = meta.stroke_width,
    );
    out
}

// --- React -----------------------------------------------------------------

fn react(icons: &Icons) -> String {
    let mut out = header_from(CommentStyle::Line("//"), ICON_SPEC);
    let meta = &icons.meta;
    let _ = write!(
        out,
        "\n// The icon component.\n\
         //\n\
         // Generated from the same declaration as the Dioxus one, so the two cannot\n\
         // disagree about a name, a size or what an unlabelled icon promises\n\
         // assistive technology.\n\
         \n\
         import {{ createElement as h }} from \"react\";\n\
         import {{ ICONS }} from \"@noctua/tokens/icons.js\";\n\
         \n\
         export function Icon({{ icon, label, size, className, ...rest }}) {{\n  \
         const drawing = ICONS[icon];\n  \
         // A name that is not an icon is a type error in TypeScript and a thrown\n  \
         // error here. It is never an empty box: that is what shipped once.\n  \
         if (!drawing) throw new Error(`no icon named ${{icon}}`);\n  \
         const size_ = size ?? \"medium\";\n  \
         const named = label !== undefined && label !== null;\n  \
         const root = [\"nd-icon-glyph\", className].filter(Boolean).join(\" \");\n  \
         return h(\"svg\", {{\n    \
         className: root,\n    \
         \"data-nd-icon\": icon,\n    \
         \"data-nd-size\": size_,\n    \
         viewBox: drawing.viewBox,\n    \
         width: \"1em\",\n    height: \"1em\",\n    \
         fill: drawing.filled ? \"currentColor\" : \"none\",\n    \
         stroke: drawing.filled ? \"none\" : \"currentColor\",\n    \
         strokeWidth: \"{stroke}\",\n    \
         strokeLinecap: \"round\",\n    \
         strokeLinejoin: \"round\",\n    \
         role: named ? \"img\" : undefined,\n    \
         \"aria-label\": named ? label : undefined,\n    \
         \"aria-hidden\": named ? undefined : \"true\",\n    \
         focusable: \"false\",\n    \
         dangerouslySetInnerHTML: {{ __html: drawing.body }},\n    \
         ...rest,\n  }});\n}}\n",
        stroke = meta.stroke_width,
    );
    out
}

fn react_types(icons: &[Resolved]) -> String {
    let mut out = header_from(CommentStyle::Line("//"), ICON_SPEC);
    out.push_str("\nimport type { ReactElement } from \"react\";\n");
    out.push_str("import type { IconName } from \"@noctua/tokens/icons.js\";\n\n");
    out.push_str("export type IconSize = \"small\" | \"medium\" | \"large\";\n\n");
    out.push_str("export interface IconProps {\n");
    out.push_str("  /** Which icon. */\n  icon: IconName;\n");
    out.push_str("  /** Its accessible name. Omit it and the icon is `aria-hidden`. */\n");
    out.push_str("  label?: string;\n");
    out.push_str("  /** The rendered box. */\n  size?: IconSize;\n");
    out.push_str("  className?: string;\n}\n\n");
    out.push_str("export declare function Icon(props: IconProps): ReactElement;\n");
    let _ = icons;
    out
}

// --- The names, framework-free ---------------------------------------------

fn tokens_js(icons: &[Resolved]) -> String {
    let mut out = header_from(CommentStyle::Line("//"), ICON_SPEC);
    out.push_str("\n/** Every icon: its drawing, its box, and how it is painted. */\n");
    out.push_str("export const ICONS = {\n");
    for icon in icons {
        let _ = writeln!(
            out,
            "  \"{}\": {{ viewBox: \"{}\", filled: {}, body: {} }},",
            icon.name,
            icon.view_box,
            icon.paint == Paint::Fill,
            json_string(&icon.body)
        );
    }
    out.push_str("};\n\n/** Every icon name, in the order the spec declares them. */\n");
    out.push_str("export const ICON_NAMES = [\n");
    for icon in icons {
        let _ = writeln!(out, "  \"{}\",", icon.name);
    }
    out.push_str("];\n");
    out
}

fn tokens_types(icons: &[Resolved]) -> String {
    let mut out = header_from(CommentStyle::Line("//"), ICON_SPEC);
    out.push_str("\n/** Every icon in the system. A name outside this union does not exist. */\n");
    out.push_str("export type IconName =\n");
    for (index, icon) in icons.iter().enumerate() {
        let end = if index + 1 == icons.len() { ";" } else { "" };
        let _ = writeln!(out, "  | \"{}\"{end}", icon.name);
    }
    out.push_str(
        "\nexport interface IconDrawing {\n  viewBox: string;\n  filled: boolean;\n  body: string;\n}\n\n",
    );
    out.push_str("export declare const ICONS: Record<IconName, IconDrawing>;\n");
    out.push_str("export declare const ICON_NAMES: readonly IconName[];\n");
    out
}

// --- The manifest and the attribution --------------------------------------

fn manifest(icons: &[Resolved]) -> String {
    let mut out = String::from("{\n");
    let _ = writeln!(
        out,
        "  \"_generated\": \"Generated by noctua-design — do not edit. Source: {ICON_SPEC}. Regenerate: {}.\",",
        crate::REGENERATE
    );
    out.push_str("  \"icons\": [\n");
    for (index, icon) in icons.iter().enumerate() {
        let comma = if index + 1 == icons.len() { "" } else { "," };
        let _ = writeln!(
            out,
            "    {{ \"name\": \"{}\", \"project\": \"{}\", \"viewBox\": \"{}\", \"filled\": {}, \"doc\": {}, \"body\": {} }}{comma}",
            icon.name,
            icon.project,
            icon.view_box,
            icon.paint == Paint::Fill,
            json_string(&icon.doc),
            json_string(&icon.body)
        );
    }
    out.push_str("  ]\n}\n");
    out
}

fn attribution(icons: &Icons, resolved: &[Resolved]) -> String {
    // Markdown has no line comment, so the header is one HTML comment rather
    // than `header_from`'s repeated-marker form.
    let mut out = String::from("<!--\n");
    for line in header_from(CommentStyle::Line(""), ICON_SPEC).lines() {
        let _ = writeln!(out, "{}", line.trim_start());
    }
    out.push_str("-->\n\n# Icons\n\n");
    out.push_str(
        "The drawings under this directory are **vendored by hand** at a pinned tag \
         and are not build output. What *is* generated from them is everything else: \
         the enum, both `Icon` implementations, the manifest, and this file.\n\n\
         Only icons the interface actually uses are here. `NOCTUA.md §9.5`: \
         *\"an unused icon is payload and review surface\"* — and the build enforces it \
         in both directions, so a drawing nothing declares fails `cargo xtask check`.\n\n",
    );

    for (key, source) in &icons.source {
        let mine: Vec<&Resolved> = resolved
            .iter()
            .filter(|icon| icon.project == source.project)
            .collect();
        let _ = write!(
            out,
            "## {}\n\n\
             - Source: <{}>\n\
             - Tag: `{}`\n\
             - Licence: {} — [`{}`]({})\n\n\
             | Icon | File | For |\n|---|---|---|\n",
            source.project,
            source.repository,
            source.tag,
            source.licence,
            source.licence_file,
            source.licence_file,
        );
        for icon in mine {
            let _ = writeln!(out, "| `{}` | `{}` | {} |", icon.name, key, icon.doc);
        }
        out.push('\n');
    }
    out
}

/// A JSON string literal, escaped.
fn json_string(text: &str) -> String {
    let mut out = String::with_capacity(text.len() + 2);
    out.push('"');
    for ch in text.chars() {
        match ch {
            '"' => out.push_str("\\\""),
            '\\' => out.push_str("\\\\"),
            '\n' => out.push_str("\\n"),
            c => out.push(c),
        }
    }
    out.push('"');
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    fn shipped() -> (Icons, Vec<Resolved>) {
        let root = concat!(env!("CARGO_MANIFEST_DIR"), "/../..");
        noctua_design_spec::icons::load(
            format!("{root}/spec/icons.toml"),
            format!("{root}/assets/icons"),
        )
        .expect("the shipped icon set")
    }

    #[test]
    fn every_icon_draws_something() {
        let (_, resolved) = shipped();
        assert!(!resolved.is_empty(), "the set is empty");
        for icon in &resolved {
            assert!(!icon.body.is_empty(), "{} draws nothing", icon.name);
            assert!(
                !icon.body.contains("d=\"\""),
                "{} has an empty path",
                icon.name
            );
        }
    }

    /// A drawing that names its own colour cannot follow a theme.
    #[test]
    fn no_drawing_carries_a_colour() {
        let (_, resolved) = shipped();
        for icon in &resolved {
            assert!(!icon.body.contains("fill="), "{}", icon.name);
            assert!(!icon.body.contains("stroke="), "{}", icon.name);
        }
    }

    /// An id inlined twice on one page is an id collision.
    #[test]
    fn no_drawing_carries_an_id() {
        let (_, resolved) = shipped();
        for icon in &resolved {
            assert!(!icon.body.contains("id="), "{}", icon.name);
        }
    }

    #[test]
    fn the_enum_has_a_variant_for_every_icon() {
        let (_, resolved) = shipped();
        let rust = names(&resolved);
        for icon in &resolved {
            assert!(
                rust.contains(&format!("Self::{} => \"{}\"", icon.variant, icon.name)),
                "{} is missing from the enum",
                icon.name
            );
        }
    }

    /// The two implementations must offer the same names, or "identical
    /// behaviour" is a claim nobody checks.
    #[test]
    fn both_languages_ship_the_same_names() {
        let (_, resolved) = shipped();
        let ts = tokens_types(&resolved);
        let rust = names(&resolved);
        for icon in &resolved {
            assert!(ts.contains(&format!("\"{}\"", icon.name)), "{}", icon.name);
            assert!(
                rust.contains(&format!("\"{}\"", icon.name)),
                "{}",
                icon.name
            );
        }
    }

    /// The header link that shipped invisible asked for this name.
    #[test]
    fn an_icon_that_does_not_exist_has_no_variant() {
        let (_, resolved) = shipped();
        assert!(
            !resolved.iter().any(|icon| icon.name == "repository"),
            "the test is stale"
        );
        let rust = names(&resolved);
        assert!(!rust.contains("Repository"));
    }
}
