//! The icon set: every name, every drawing, and the weight they are stroked at.
//!
//! | Output | For |
//! |---|---|
//! | `crates/noctua-design-icons/src/lib.rs` | one constant per icon, and `STROKE_WIDTH` |
//! | `packages/tokens/icons/<name>.js` + `.d.ts` | one module per icon |
//! | `packages/tokens/icons.js` + `.d.ts` | the stroke weight and the shape of a drawing |
//! | `dist/icons.json` | the manifest, for anything that wants the whole set |
//! | `assets/icons/ATTRIBUTION.md` | who drew them, and under what licence |
//!
//! # One constant, one module, and why the shape is the feature
//!
//! This emitted one `IconName` enum with a `match` per fact, and one `ICONS`
//! object read as `ICONS[name]`. Both are aggregates, and an aggregate is
//! indivisible: a `match` arm lives inside a function the linker keeps whole,
//! and a dynamic property read is a thing no bundler can trace. At forty icons
//! that cost a few kilobytes. At 1,777 it would cost every consumer the entire
//! set to draw one chevron.
//!
//! So a name is not a string that indexes a table any more — it is a
//! **constant that carries its own drawing**:
//!
//! - Rust: `IconDrawing::CHEVRON_DOWN`, an associated `const`. A `const` is
//!   materialised at its use sites and nowhere else, so an icon nothing names
//!   contributes no bytes at any optimisation level.
//! - JavaScript: `import { chevronDown } from ".../icons/chevron-down.js"`. One
//!   module per icon, one export each, so a bundler keeps exactly what was
//!   imported.
//!
//! `NOCTUA.md §9.5` asked for a name a typo could not survive. An import path
//! is that, harder: it fails before a type checker is consulted.
//!
//! `cargo xtask check` proves both — a release binary and a real esbuild
//! bundle, each scanned for drawings that must not be in it.
//!
//! # The `Icon` COMPONENT is not here
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

use std::fmt::Write as _;

use noctua_design_spec::icons::{Icons, Meta, Paint, Resolved};

use crate::{CommentStyle, EmittedFile, header_from};

/// The icon specification path, as written in generated headers.
pub const ICON_SPEC: &str = "spec/icons.toml";

/// Emits the whole icon layer.
#[must_use]
pub fn emit(icons: &Icons, resolved: &[Resolved]) -> Vec<EmittedFile> {
    let mut files = vec![
        EmittedFile::new(
            "crates/noctua-design-icons/src/lib.rs",
            names(&icons.meta, resolved),
        ),
        EmittedFile::new("packages/tokens/icons.js", tokens_js(&icons.meta)),
        EmittedFile::new("packages/tokens/icons.d.ts", tokens_types()),
        EmittedFile::new("dist/icons.json", manifest(resolved)),
        EmittedFile::new("assets/icons/ATTRIBUTION.md", attribution(icons, resolved)),
    ];
    for icon in resolved {
        files.push(EmittedFile::new(
            format!("packages/tokens/icons/{}.js", icon.name),
            module_js(icon),
        ));
        files.push(EmittedFile::new(
            format!("packages/tokens/icons/{}.d.ts", icon.name),
            module_types(icon),
        ));
    }
    files
}

// --- The names and the drawings --------------------------------------------

fn names(meta: &Meta, icons: &[Resolved]) -> String {
    let mut out = header_from(CommentStyle::Line("//!"), ICON_SPEC);
    out.push_str(
        "//!\n\
         //! Every icon in the system, as one constant each.\n\
         //!\n\
         //! `no_std`: this is a table of string literals, and a consumer should not\n\
         //! take a standard library on to read one.\n\
         //!\n\
         //! # One constant per icon, and nothing that touches all of them\n\
         //!\n\
         //! A `const` is materialised at its use sites and at no others, so an icon\n\
         //! nothing names reaches no binary. `ALL` and [`IconDrawing::parse`] are the\n\
         //! two things that touch every icon, and they are behind the `catalogue`\n\
         //! feature, which is off: an interface reaches an icon by naming it, and only\n\
         //! a gallery wants the set.\n\
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

    out.push_str(
        "/// One icon: its name, its geometry, and how it is painted.\n\
         ///\n\
         /// A value rather than a name, so the drawing travels with the reference to\n\
         /// it. That is what lets an application that draws one icon pay for one icon:\n\
         /// there is no table to look the name up in, so there is no table to keep.\n\
         ///\n\
         /// Constructed only by this module. A drawing assembled anywhere else has not\n\
         /// been through the validator in `spec/icons.toml`'s loader, and the whole\n\
         /// point of that validator is that a drawing carries geometry and nothing\n\
         /// else — which is why the fields are readable and the type is not.\n\
         #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]\n\
         #[non_exhaustive]\n\
         pub struct IconDrawing {\n\
         \x20   /// Its name, as `spec/icons.toml` spells it. Written to `data-nd-icon`.\n\
         \x20   pub name: &'static str,\n\
         \x20   /// Its `viewBox`, taken from the drawing rather than assumed.\n\
         \x20   pub view_box: &'static str,\n\
         \x20   /// Whether it is drawn as a solid shape rather than an outline.\n\
         \x20   pub filled: bool,\n\
         \x20   /// The drawing: the children of `<svg>`, and nothing else.\n\
         \x20   pub body: &'static str,\n\
         \x20   /// What it is for.\n\
         \x20   pub doc: &'static str,\n\
         }\n\n",
    );

    out.push_str("impl IconDrawing {\n");
    for (index, icon) in icons.iter().enumerate() {
        if index > 0 {
            out.push('\n');
        }
        let _ = writeln!(out, "    /// {}", doc_line(&icon.doc));
        let _ = writeln!(out, "    pub const {}: Self = Self {{", icon.constant);
        let _ = writeln!(out, "        name: \"{}\",", icon.name);
        let _ = writeln!(out, "        view_box: \"{}\",", icon.view_box);
        let _ = writeln!(out, "        filled: {},", icon.paint == Paint::Fill);
        let _ = writeln!(out, "        body: r#\"{}\"#,", icon.body);
        let _ = writeln!(out, "        doc: \"{}\",", escape_rust(&icon.doc));
        out.push_str("    };\n");
    }
    out.push_str("}\n\n");

    catalogue(&mut out, icons);
    out
}

/// `ALL` and `parse`: the two things that touch every icon.
///
/// Behind a feature because they are the two that would defeat the whole shape
/// — an array naming 1,777 constants materialises 1,777 constants. The
/// documentation site's gallery is what asks for them, and nothing else should.
fn catalogue(out: &mut String, icons: &[Resolved]) {
    out.push_str(
        "/// Every icon in the system, in name order.\n\
         ///\n\
         /// Behind the `catalogue` feature: referring to this array refers to every\n\
         /// icon in it, which is exactly the payload an application must not take on to\n\
         /// draw one. The gallery on the documentation site is what it is for.\n\
         ///\n\
         /// A `static` rather than a `const`, and clippy is right about why: a `const`\n\
         /// array is copied whole at every use site, so two readers of a thousand-entry\n\
         /// catalogue would be two thousand entries. A `static` is one instance, and one\n\
         /// nothing refers to is dropped at link time.\n\
         #[cfg(feature = \"catalogue\")]\n",
    );
    let _ = writeln!(out, "pub static ALL: [IconDrawing; {}] = [", icons.len());
    for icon in icons {
        let _ = writeln!(out, "    IconDrawing::{},", icon.constant);
    }
    out.push_str("];\n\n");

    out.push_str(
        "#[cfg(feature = \"catalogue\")]\n\
         impl IconDrawing {\n\
         \x20   /// The icon of this name, for the documentation page and for tests.\n\
         \x20   ///\n\
         \x20   /// Deliberately fallible and deliberately not `FromStr`: nothing in an\n\
         \x20   /// interface should reach an icon by name. A name that is a string cannot\n\
         \x20   /// tell \"this icon\" from \"no icon\", which is the defect the constants\n\
         \x20   /// above exist to make impossible.\n\
         \x20   #[must_use]\n\
         \x20   pub fn parse(name: &str) -> Option<Self> {\n\
         \x20       let mut index = 0;\n\
         \x20       while index < ALL.len() {\n\
         \x20           let icon = ALL[index];\n\
         \x20           if str_eq(icon.name, name) {\n\
         \x20               return Some(icon);\n\
         \x20           }\n\
         \x20           index += 1;\n\
         \x20       }\n\
         \x20       None\n\
         \x20   }\n\
         }\n\n\
         /// `str::eq` without the standard library.\n\
         #[cfg(feature = \"catalogue\")]\n\
         fn str_eq(a: &str, b: &str) -> bool {\n\
         \x20   a.as_bytes() == b.as_bytes()\n\
         }\n",
    );
}

/// One `doc` as a single Rust doc-comment line.
///
/// A derived doc is one sentence and a written one is often several, but a
/// `///` cannot carry a newline: the alternative is a multi-line comment whose
/// second line rustfmt is free to rewrap, and a generated file that rustfmt
/// changes fails the drift check.
fn doc_line(doc: &str) -> String {
    doc.split_whitespace().collect::<Vec<_>>().join(" ")
}

/// A Rust string literal's contents, escaped.
fn escape_rust(text: &str) -> String {
    doc_line(text).replace('\\', "\\\\").replace('"', "\\\"")
}

// --- One module per icon ----------------------------------------------------

fn module_js(icon: &Resolved) -> String {
    let mut out = header_from(CommentStyle::Line("//"), ICON_SPEC);
    let _ = write!(
        out,
        "\n/** {} */\nexport const {} = {{\n  \
         name: \"{}\",\n  viewBox: \"{}\",\n  filled: {},\n  body: {},\n}};\n",
        doc_line(&icon.doc),
        icon.export,
        icon.name,
        icon.view_box,
        icon.paint == Paint::Fill,
        json_string(&icon.body),
    );
    out
}

fn module_types(icon: &Resolved) -> String {
    let mut out = header_from(CommentStyle::Line("//"), ICON_SPEC);
    let _ = write!(
        out,
        "\nimport type {{ IconDrawing }} from \"../icons.js\";\n\n\
         /** {} */\nexport declare const {}: IconDrawing;\n",
        doc_line(&icon.doc),
        icon.export,
    );
    out
}

// --- What every icon shares -------------------------------------------------

fn tokens_js(meta: &Meta) -> String {
    let mut out = header_from(CommentStyle::Line("//"), ICON_SPEC);
    out.push_str(
        "\n// THERE IS NO TABLE OF ICONS HERE, AND THAT IS THE FEATURE.\n\
         //\n\
         // This module held an `ICONS` object with every drawing in it, and the React\n\
         // `Icon` read it as `ICONS[name]`. A dynamic property read is a thing no\n\
         // bundler can trace, so one icon meant shipping all of them. Each drawing is\n\
         // its own module now:\n\
         //\n\
         //     import { chevronDown } from \"@noctua-world/design-tokens/icons/chevron-down.js\";\n\
         //\n\
         // A consumer that wants the whole set — an icon picker — fetches\n\
         // `dist/icons.json` at run time, which is where a catalogue belongs: lazy,\n\
         // cacheable, and outside the bundle.\n\n",
    );
    out.push_str("/** The weight a stroked icon is drawn at. */\n");
    let _ = writeln!(
        out,
        "export const ICON_STROKE_WIDTH = \"{}\";",
        meta.stroke_width
    );
    out
}

fn tokens_types() -> String {
    let mut out = header_from(CommentStyle::Line("//"), ICON_SPEC);
    out.push_str(
        "\n/**\n * One icon: its name, its geometry, and how it is painted.\n *\n\
         * Reached by importing it, one module per icon:\n *\n\
         *     import { chevronDown } from \"@noctua-world/design-tokens/icons/chevron-down.js\";\n *\n\
         * A path that names no icon fails at build time, which is the guarantee the\n\
         * deleted `IconName` union used to give and gives sooner.\n */\n\
         export interface IconDrawing {\n  \
         /** Its name, as `spec/icons.toml` spells it. Written to `data-nd-icon`. */\n  \
         name: string;\n  \
         /** Its `viewBox`, taken from the drawing rather than assumed. */\n  \
         viewBox: string;\n  \
         /** Whether it is drawn as a solid shape rather than an outline. */\n  \
         filled: boolean;\n  \
         /** The drawing: the children of `<svg>`, and nothing else. */\n  \
         body: string;\n}\n\n",
    );
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
    // NOTHING ON A PAGE FETCHES THIS. It is the machine-readable catalogue —
    // for an icon picker, for tooling, for a search — and a browser test
    // asserts no rendered page requests it. A page that did would be shipping
    // every drawing to draw one.
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
            json_string(&doc_line(&icon.doc)),
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
    let _ = write!(
        out,
        "The drawings under this directory are **vendored by hand** at a pinned tag \
         and are not build output. What *is* generated from them is everything else: \
         the constants, both `Icon` implementations, the manifest, and this file.\n\n\
         Every drawing is checksummed. `cargo xtask check` verifies each one against \
         the `SHA256SUMS` recorded when it was fetched, and fails on a file that \
         changed, a file that is missing, and a file the list does not mention.\n\n\
         `NOCTUA.md §9.5` used to keep the set to what the interfaces used, because \
         *\"an unused icon is payload and review surface\"*. That is still true and it \
         is now enforced by measurement: an icon nothing names reaches no binary and \
         no bundle, and `cargo xtask check` proves it on a linked release binary and \
         on a real bundler's output.\n\n\
         **{} icons**, from {} source(s).\n\n",
        resolved.len(),
        icons.source.len(),
    );

    for (key, source) in &icons.source {
        let mine: Vec<&Resolved> = resolved.iter().filter(|icon| icon.source == *key).collect();
        let curated: Vec<&&Resolved> = mine.iter().filter(|icon| icon.curated).collect();
        let _ = write!(
            out,
            "## {}\n\n\
             - Source: <{}>\n\
             - Tag: `{}`\n\
             - Licence: {} — [`{}`]({}/{})\n\
             - Vendored: `{}/`, {} drawing(s)\n\n\
             Fetched with:\n\n```sh\n{}```\n\n",
            source.project,
            source.repository,
            source.tag,
            source.licence,
            source.licence_file,
            source.directory,
            source.licence_file,
            source.directory,
            mine.len(),
            source.fetch.trim_start(),
        );

        if !curated.is_empty() {
            out.push_str(
                "### What this system reaches for, and why\n\n\
                 Every other drawing from this source is swept in under its own name \
                 and is documented by that name alone.\n\n\
                 | Icon | File | For |\n|---|---|---|\n",
            );
            for icon in &curated {
                let _ = writeln!(
                    out,
                    "| `{}` | `{}` | {} |",
                    icon.name,
                    icon.file,
                    doc_line(&icon.doc)
                );
            }
            out.push('\n');
        }

        let swept: Vec<&str> = mine
            .iter()
            .filter(|icon| !icon.curated)
            .map(|icon| icon.name.as_str())
            .collect();
        if !swept.is_empty() {
            let _ = write!(
                out,
                "### The rest, in full\n\n\
                 {} drawing(s), each named after its file. Listed rather than \
                 summarised: an attribution that says \"and others\" attributes \
                 nothing.\n\n{}\n\n",
                swept.len(),
                swept.join(", ")
            );
        }
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

    /// A drawing that names its own color cannot follow a theme. `currentColor`
    /// is the one value that is not naming one.
    #[test]
    fn no_drawing_carries_a_fixed_color() {
        let (_, resolved) = shipped();
        for icon in &resolved {
            for painted in icon.body.split("fill=\"").skip(1) {
                let value = painted.split('"').next().unwrap_or_default();
                assert!(
                    matches!(value, "currentColor" | "none"),
                    "{} fills with {value}",
                    icon.name
                );
            }
            assert!(!icon.body.contains("stroke=\""), "{}", icon.name);
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
    fn every_icon_has_a_constant_and_a_module() {
        let (icons, resolved) = shipped();
        let rust = names(&icons.meta, &resolved);
        for icon in &resolved {
            assert!(
                rust.contains(&format!("pub const {}: Self = Self {{", icon.constant)),
                "{} is missing its constant",
                icon.name
            );
            assert!(
                module_js(icon).contains(&format!("export const {} = {{", icon.export)),
                "{} is missing its module",
                icon.name
            );
        }
    }

    /// The shape the whole change exists for: nothing a consumer imports may
    /// name more than the icon it asked for.
    #[test]
    fn no_emitted_module_mentions_a_second_icon() {
        let (icons, resolved) = shipped();
        let shared = tokens_js(&icons.meta);
        assert!(
            !shared.contains("body:"),
            "the shared module carries a drawing"
        );
        for icon in resolved.iter().take(200) {
            let module = module_js(icon);
            let mentions = resolved
                .iter()
                .filter(|other| module.contains(&format!("\"{}\"", other.name)))
                .count();
            assert_eq!(mentions, 1, "{} names another icon", icon.name);
        }
    }

    /// `ALL` is the one thing that names every icon, and a consumer must have
    /// to ask for it.
    #[test]
    fn the_catalogue_is_behind_a_feature() {
        let (icons, resolved) = shipped();
        let rust = names(&icons.meta, &resolved);
        let at = rust.find("pub static ALL").expect("the catalogue");
        assert!(
            rust[..at].ends_with("#[cfg(feature = \"catalogue\")]\n"),
            "ALL is not behind the feature"
        );
        assert!(rust.contains("#[cfg(feature = \"catalogue\")]\nimpl IconDrawing {"));
    }

    /// The header link that shipped invisible asked for this name.
    #[test]
    fn an_icon_that_does_not_exist_has_no_constant() {
        let (icons, resolved) = shipped();
        assert!(
            !resolved.iter().any(|icon| icon.name == "repository"),
            "the test is stale"
        );
        let rust = names(&icons.meta, &resolved);
        assert!(!rust.contains("pub const REPOSITORY"));
    }
}
