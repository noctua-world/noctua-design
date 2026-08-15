//! The icon set: every name, every drawing, and the weight they are stroked at.
//!
//! | Output | For |
//! |---|---|
//! | `crates/noctua-design-icons/src/lib.rs` | the names, the drawings and `STROKE_WIDTH` |
//! | `packages/tokens/icons.js` + `.d.ts` | the same, for consumers with no framework |
//! | `dist/icons.json` | the manifest the documentation page reads |
//! | `assets/icons/ATTRIBUTION.md` | who drew them, and under what licence |
//!
//! # The `Icon` COMPONENT is not here any more
//!
//! It was, in `crates/noctua-design/src/icon.rs` and `packages/react/src/icon.js`,
//! and that put one component outside the catalogue: no props table, no variant
//! grid, absent from search-by-prop. It is `spec/components/icon.toml` now, with
//! its body in `controls.rs` beside every other authored one.
//!
//! Moving it rather than adding a second contract is the whole of the fix. A
//! `spec/components/icon.toml` beside this emitter would have produced two
//! components called `Icon` in one crate — `components::Icon` and `icon::Icon` —
//! with the explicit re-export in `lib.rs` picking one and nothing reporting the
//! other. Not a compile error. A silent divergence.
//!
//! What stays here is what is about the DRAWINGS rather than about the
//! component: the names, the geometry, the manifest, the attribution, and the
//! stroke weight both bodies read from `noctua_design_icons::STROKE_WIDTH`.
//!
//! # An enum, not a string
//!
//! The site shipped `Icon { name: "repository" }` against a `match` with a
//! `_ => ""` arm. It compiled, and it rendered `<path d="">` inside a 44×44
//! clickable box in the header of every page. Nothing could have caught it:
//! "an icon called repository" and "no icon" were the same value.
//!
//! Here the name is a generated enum in Rust and a generated union in
//! TypeScript, so that call is a type error in both languages. `NOCTUA.md §9.5`
//! asks for exactly this.

use std::fmt::Write as _;

use noctua_design_spec::icons::{Icons, Meta, Paint, Resolved};

use crate::{CommentStyle, EmittedFile, header_from};

/// The icon specification path, as written in generated headers.
pub const ICON_SPEC: &str = "spec/icons.toml";

/// Emits the whole icon layer.
#[must_use]
pub fn emit(icons: &Icons, resolved: &[Resolved]) -> Vec<EmittedFile> {
    vec![
        EmittedFile::new(
            "crates/noctua-design-icons/src/lib.rs",
            names(&icons.meta, resolved),
        ),
        EmittedFile::new("packages/tokens/icons.js", tokens_js(&icons.meta, resolved)),
        EmittedFile::new("packages/tokens/icons.d.ts", tokens_types(resolved)),
        EmittedFile::new("dist/icons.json", manifest(resolved)),
        EmittedFile::new("assets/icons/ATTRIBUTION.md", attribution(icons, resolved)),
    ]
}

// --- The names and the drawings --------------------------------------------

fn names(meta: &Meta, icons: &[Resolved]) -> String {
    let mut out = header_from(CommentStyle::Line("//!"), ICON_SPEC);
    out.push_str(
        "//!\n\
         //! Every icon the interface uses, as a name and a drawing.\n\
         //!\n\
         //! `no_std`: this is a table of string literals, and a consumer should not\n\
         //! take a standard library on to read one.\n\
         \n#![no_std]\n\n",
    );

    // THE WEIGHT IS DECLARED ONCE AND READ TWICE. Both `Icon` bodies write it
    // on the `<svg>`, and the documentation page inlines the same wrapper for
    // its variant grid. A literal in any of the three would be a stroke weight
    // `spec/icons.toml` no longer decides.
    out.push_str(
        "/// The weight a stroked icon is drawn at, from `spec/icons.toml`.\n\
         ///\n\
         /// Read by both `Icon` implementations and by the documentation page's\n\
         /// variant grid, so the declaration has exactly one value in it.\n",
    );
    let _ = writeln!(
        out,
        "pub const STROKE_WIDTH: &str = \"{}\";\n",
        meta.stroke_width
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

// --- The names, framework-free ---------------------------------------------

fn tokens_js(meta: &Meta, icons: &[Resolved]) -> String {
    let mut out = header_from(CommentStyle::Line("//"), ICON_SPEC);
    // The React `Icon` reads it from here, as the Dioxus one reads
    // `noctua_design_icons::STROKE_WIDTH`. One declaration, two languages.
    out.push_str("\n/** The weight a stroked icon is drawn at. */\n");
    let _ = writeln!(
        out,
        "export const ICON_STROKE_WIDTH = \"{}\";\n",
        meta.stroke_width
    );
    out.push_str("/** Every icon: its drawing, its box, and how it is painted. */\n");
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
    out.push_str("export declare const ICON_STROKE_WIDTH: string;\n");
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

    /// A drawing that names its own color cannot follow a theme.
    #[test]
    fn no_drawing_carries_a_color() {
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
        let (icons, resolved) = shipped();
        let rust = names(&icons.meta, &resolved);
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
        let (icons, resolved) = shipped();
        let ts = tokens_types(&resolved);
        let rust = names(&icons.meta, &resolved);
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
        let (icons, resolved) = shipped();
        assert!(
            !resolved.iter().any(|icon| icon.name == "repository"),
            "the test is stale"
        );
        let rust = names(&icons.meta, &resolved);
        assert!(!rust.contains("Repository"));
    }
}
