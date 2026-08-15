//! The message catalogue, in both languages the system speaks.
//!
//! | Output | For |
//! |---|---|
//! | `crates/noctua-design-i18n/src/lib.rs` | the Dioxus half and the site build |
//! | `packages/tokens/i18n.js` + `.d.ts` | the React half |
//!
//! # A key is an enum, not a string
//!
//! The Rust side takes a [`Key`](../../noctua_design_i18n/enum.Key.html) rather
//! than a `&str`, so a typo is a compile error and a lookup can never miss. That
//! also removes the `Option` a string-keyed map would have to return, and with
//! it the temptation to answer a missing key with the key itself — which ships a
//! page with `nav.open` printed on a button and nothing to report it.
//!
//! TypeScript gets the same guarantee from a union type, which is as close as
//! that language comes to the same promise.

use std::fmt::Write as _;

use noctua_design_spec::i18n::{Catalogue, Locale, PSEUDO, SOURCE};

use crate::{CommentStyle, EmittedFile, MESSAGE_SPEC, header_from};

/// Emits the Rust catalogue and the JavaScript one.
#[must_use]
pub fn emit(catalogue: &Catalogue) -> Vec<EmittedFile> {
    vec![
        EmittedFile::new("crates/noctua-design-i18n/src/lib.rs", rust(catalogue)),
        EmittedFile::new("packages/tokens/i18n.js", js(catalogue)),
        EmittedFile::new("packages/tokens/i18n.d.ts", dts(catalogue)),
    ]
}

// --- Rust ------------------------------------------------------------------

fn rust(catalogue: &Catalogue) -> String {
    let mut out = header_from(CommentStyle::Line("//!"), MESSAGE_SPEC);
    out.push_str(
        "//!\n\
         //! Every message the interface can show, in every locale it speaks.\n\
         //!\n\
         //! Generated from `spec/i18n/`, where the two authored locales live. The\n\
         //! pseudo-locale is generated from English rather than written, so it cannot\n\
         //! fall behind the source it exists to test.\n\
         \n\
         #![no_std]\n\
         // Two allows, and both are about this being a *generated table* rather\n\
         // than code somebody wrote.\n\
         //\n\
         // Two locales may legitimately share a word — Jade, Magenta, Noctua —\n\
         // and merging those arms would stop the table listing every locale\n\
         // explicitly, which is the property that makes a missing translation\n\
         // impossible to express.\n\
         #![allow(clippy::match_same_arms)]\n\
         // The lookup is one arm per message per locale. Splitting it would move\n\
         // the same lines into more functions and make the generator harder to\n\
         // read, for no gain to anyone reading the output.\n\
         #![allow(clippy::too_many_lines)]\n\
         \n\
         // `fill` returns an owned String — a message with a hole in it is the one\n\
         // thing this crate cannot answer with a `&'static str`. Everything else is\n\
         // borrowed from the table, so `alloc` costs nothing that is not used.\n\
         extern crate alloc;\n\
         \n",
    );

    rust_locale(&mut out, catalogue);
    rust_key(&mut out, catalogue);
    rust_table(&mut out, catalogue);
    rust_fill(&mut out);
    out
}

fn rust_locale(out: &mut String, catalogue: &Catalogue) {
    out.push_str(
        "/// A locale this interface is available in.\n\
         #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]\n\
         #[non_exhaustive]\n\
         pub enum Locale {\n",
    );
    for locale in &catalogue.locales {
        writeln!(
            out,
            "    /// {} — `{}`.\n    {},",
            locale.meta.name,
            locale.tag,
            variant(&locale.tag)
        )
        .expect("write");
    }
    out.push_str("}\n\nimpl Locale {\n");

    writeln!(
        out,
        "    /// Every locale, source first.\n    \
         pub const ALL: [Self; {}] = [{}];\n",
        catalogue.locales.len(),
        catalogue
            .locales
            .iter()
            .map(|l| format!("Self::{}", variant(&l.tag)))
            .collect::<Vec<_>>()
            .join(", ")
    )
    .expect("write");

    writeln!(
        out,
        "    /// The locale a reader gets when nothing else is known.\n    \
         pub const DEFAULT: Self = Self::{};\n",
        variant(SOURCE)
    )
    .expect("write");

    writeln!(
        out,
        "    /// The locales offered to a reader.\n    \
         ///\n    \
         /// The pseudo-locale is deliberately not among them: it exists to break\n    \
         /// layouts during review, and offering it to a reader who picked it by\n    \
         /// accident would just look like a bug.\n    \
         pub const OFFERED: [Self; {}] = [{}];\n",
        catalogue.locales.len() - 1,
        catalogue
            .locales
            .iter()
            .filter(|l| l.tag != PSEUDO)
            .map(|l| format!("Self::{}", variant(&l.tag)))
            .collect::<Vec<_>>()
            .join(", ")
    )
    .expect("write");

    accessor(
        out,
        "tag",
        "The file stem and URL segment",
        catalogue,
        |l| l.tag.clone(),
    );
    accessor(
        out,
        "name",
        "This locale's name, written in this locale",
        catalogue,
        |l| l.meta.name.clone(),
    );
    accessor(out, "bcp47", "The tag for `<html lang>`", catalogue, |l| {
        l.meta.bcp47.clone()
    });
    accessor(out, "dir", "The tag for `<html dir>`", catalogue, |l| {
        l.meta.dir.clone()
    });

    out.push_str(
        "    /// Parses a stored or requested locale.\n\
         \x20   ///\n\
         \x20   /// `None` for anything unrecognised, and the caller falls back to\n\
         \x20   /// [`Locale::DEFAULT`]. Never guess: a half-matched locale shows a page\n\
         \x20   /// in a language nobody asked for.\n\
         \x20   #[must_use]\n\
         \x20   pub fn parse(value: &str) -> Option<Self> {\n\
         \x20       Self::ALL.into_iter().find(|l| l.tag() == value)\n\
         \x20   }\n\
         }\n\n\
         impl Default for Locale {\n\
         \x20   fn default() -> Self {\n\
         \x20       Self::DEFAULT\n\
         \x20   }\n\
         }\n\n",
    );
}

fn accessor(
    out: &mut String,
    method: &str,
    doc: &str,
    catalogue: &Catalogue,
    value: impl Fn(&Locale) -> String,
) {
    writeln!(
        out,
        "    /// {doc}.\n    #[must_use]\n    pub const fn {method}(self) -> &'static str {{\n        match self {{"
    )
    .expect("write");
    for locale in &catalogue.locales {
        writeln!(
            out,
            "            Self::{} => \"{}\",",
            variant(&locale.tag),
            value(locale)
        )
        .expect("write");
    }
    out.push_str("        }\n    }\n\n");
}

fn rust_key(out: &mut String, catalogue: &Catalogue) {
    out.push_str(
        "/// One message.\n\
         ///\n\
         /// An enum rather than a string, so a typo is a compile error and a lookup\n\
         /// cannot miss. That is also why [`Locale::message`] returns `&'static str`\n\
         /// rather than an `Option`: there is no missing case to answer.\n\
         #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]\n\
         #[non_exhaustive]\n\
         pub enum Key {\n",
    );
    for key in &catalogue.keys {
        writeln!(out, "    /// `{key}`\n    {},", variant(key)).expect("write");
    }
    out.push_str("}\n\nimpl Key {\n");

    writeln!(
        out,
        "    /// Every key, in the order the source locale declares them.\n    \
         pub const ALL: [Self; {}] = [{}];\n",
        catalogue.keys.len(),
        catalogue
            .keys
            .iter()
            .map(|k| format!("Self::{}", variant(k)))
            .collect::<Vec<_>>()
            .join(", ")
    )
    .expect("write");

    out.push_str(
        "    /// The dotted name, for diagnostics and for the search index.\n\
         \x20   #[must_use]\n\
         \x20   pub const fn id(self) -> &'static str {\n\
         \x20       match self {\n",
    );
    for key in &catalogue.keys {
        writeln!(out, "            Self::{} => \"{key}\",", variant(key)).expect("write");
    }
    out.push_str("        }\n    }\n}\n\n");
}

fn rust_table(out: &mut String, catalogue: &Catalogue) {
    out.push_str(
        "impl Locale {\n\
         \x20   /// This locale's text for a message.\n\
         \x20   #[must_use]\n\
         \x20   pub const fn message(self, key: Key) -> &'static str {\n\
         \x20       match self {\n",
    );
    for locale in &catalogue.locales {
        writeln!(
            out,
            "            Self::{} => match key {{",
            variant(&locale.tag)
        )
        .expect("write");
        for key in &catalogue.keys {
            writeln!(
                out,
                "                Key::{} => \"{}\",",
                variant(key),
                escape(&locale.message[key])
            )
            .expect("write");
        }
        out.push_str("            },\n");
    }
    out.push_str("        }\n    }\n}\n\n");
}

fn rust_fill(out: &mut String) {
    out.push_str(
        "/// Substitutes `{name}` placeholders in a message.\n\
         ///\n\
         /// # This is the only sanctioned way to build a message\n\
         ///\n\
         /// `NOCTUA.md §9.9` rule 1: never assemble a sentence from fragments. Word\n\
         /// order, gender and agreement all differ between languages, and two\n\
         /// concatenated lookups that read correctly in English can be ungrammatical\n\
         /// or meaningless once translated. A whole sentence with a hole in it\n\
         /// survives translation; two halves do not. A source gate enforces it.\n\
         ///\n\
         /// An unknown placeholder is left as it is, braces included, so a missing\n\
         /// argument is visible on the page rather than silently blank.\n\
         #[must_use]\n\
         pub fn fill(template: &str, arguments: &[(&str, &str)]) -> alloc::string::String {\n\
         \x20   use alloc::string::String;\n\
         \n\
         \x20   let mut out = String::with_capacity(template.len());\n\
         \x20   let mut rest = template;\n\
         \x20   while let Some(open) = rest.find('{') {\n\
         \x20       out.push_str(&rest[..open]);\n\
         \x20       rest = &rest[open + 1..];\n\
         \x20       let Some(close) = rest.find('}') else {\n\
         \x20           out.push('{');\n\
         \x20           break;\n\
         \x20       };\n\
         \x20       let name = &rest[..close];\n\
         \x20       if let Some((_, value)) = arguments.iter().find(|(key, _)| *key == name) {\n\
         \x20           out.push_str(value);\n\
         \x20       } else {\n\
         \x20           // Left as it is, braces included, so a missing argument is\n\
         \x20           // visible on the page rather than a silent gap in a sentence.\n\
         \x20           out.push('{');\n\
         \x20           out.push_str(name);\n\
         \x20           out.push('}');\n\
         \x20       }\n\
         \x20       rest = &rest[close + 1..];\n\
         \x20   }\n\
         \x20   out.push_str(rest);\n\
         \x20   out\n\
         }\n",
    );
}

// --- JavaScript ------------------------------------------------------------

fn js(catalogue: &Catalogue) -> String {
    let mut out = header_from(CommentStyle::Line("//"), MESSAGE_SPEC);
    out.push_str(
        "\n// Generated from spec/i18n/, the same files the Rust catalogue comes from.\n\
         // Neither writes a message down, so the two cannot disagree about one.\n\n\
         export const messages = {\n",
    );
    for locale in &catalogue.locales {
        writeln!(out, "  \"{}\": {{", locale.tag).expect("write");
        for key in &catalogue.keys {
            writeln!(out, "    \"{key}\": \"{}\",", escape(&locale.message[key])).expect("write");
        }
        out.push_str("  },\n");
    }
    out.push_str("};\n\nexport const localeMeta = {\n");
    for locale in &catalogue.locales {
        writeln!(
            out,
            "  \"{}\": {{ name: \"{}\", bcp47: \"{}\", dir: \"{}\" }},",
            locale.tag,
            escape(&locale.meta.name),
            locale.meta.bcp47,
            locale.meta.dir
        )
        .expect("write");
    }
    out.push_str("};\n\n");

    writeln!(out, "export const defaultLocale = \"{SOURCE}\";").expect("write");
    writeln!(
        out,
        "/** Offered to readers. The pseudo-locale is for review, not for reading. */\n\
         export const offeredLocales = [{}];\n",
        catalogue
            .locales
            .iter()
            .filter(|l| l.tag != PSEUDO)
            .map(|l| format!("\"{}\"", l.tag))
            .collect::<Vec<_>>()
            .join(", ")
    )
    .expect("write");

    out.push_str(
        "/**\n\
         \x20* One message, in one locale.\n\
         \x20*\n\
         \x20* Throws on an unknown key rather than returning it. A key rendered as\n\
         \x20* its own name puts `nav.open` on a button and reports nothing.\n\
         \x20*/\n\
         export function t(locale, key) {\n\
         \x20 const table = messages[locale] ?? messages[defaultLocale];\n\
         \x20 const message = table[key];\n\
         \x20 if (message === undefined) throw new Error(`no message for ${key}`);\n\
         \x20 return message;\n\
         }\n\n\
         /**\n\
         \x20* Substitutes `{name}` placeholders.\n\
         \x20*\n\
         \x20* The only sanctioned way to build a message: never concatenate two\n\
         \x20* lookups. Word order, gender and agreement differ between languages, so\n\
         \x20* a whole sentence with a hole in it survives translation and two halves\n\
         \x20* do not.\n\
         \x20*/\n\
         export function fill(template, args) {\n\
         \x20 return template.replace(/\\{(\\w+)\\}/g, (whole, name) =>\n\
         \x20   Object.hasOwn(args, name) ? args[name] : whole,\n\
         \x20 );\n\
         }\n",
    );
    out
}

fn dts(catalogue: &Catalogue) -> String {
    let mut out = header_from(CommentStyle::Line("//"), MESSAGE_SPEC);
    out.push('\n');

    writeln!(
        out,
        "export type LocaleTag = {};",
        catalogue
            .locales
            .iter()
            .map(|l| format!("\"{}\"", l.tag))
            .collect::<Vec<_>>()
            .join(" | ")
    )
    .expect("write");

    out.push_str("\n/** Every message. A typo is a type error. */\nexport type MessageKey =\n");
    for key in &catalogue.keys {
        writeln!(out, "  | \"{key}\"").expect("write");
    }
    out.push_str(
        "  ;\n\n\
         export declare const messages: Record<LocaleTag, Record<MessageKey, string>>;\n\
         export declare const localeMeta: Record<\n\
         \x20 LocaleTag,\n\
         \x20 { name: string; bcp47: string; dir: \"ltr\" | \"rtl\" }\n\
         >;\n\
         export declare const defaultLocale: LocaleTag;\n\
         export declare const offeredLocales: LocaleTag[];\n\
         export declare function t(locale: LocaleTag, key: MessageKey): string;\n\
         export declare function fill(\n\
         \x20 template: string,\n\
         \x20 args: Record<string, string>,\n\
         ): string;\n",
    );
    out
}

// --- Shared ----------------------------------------------------------------

/// `page.scoped-themes.title` → `PageScopedThemesTitle`; `pt-BR` → `PtBr`.
fn variant(name: &str) -> String {
    let mut out = String::with_capacity(name.len());
    let mut upper_next = true;
    for ch in name.chars() {
        if ch == '.' || ch == '-' || ch == '_' {
            upper_next = true;
        } else if upper_next {
            out.extend(ch.to_uppercase());
            upper_next = false;
        } else {
            out.extend(ch.to_lowercase());
        }
    }
    out
}

/// Escapes a message for a Rust or JavaScript double-quoted literal.
///
/// The two languages agree on `\"` and `\\`, and the catalogue has no other
/// character either of them treats specially — a newline in a message would be
/// a message that should have been two.
fn escape(message: &str) -> String {
    message.replace('\\', "\\\\").replace('"', "\\\"")
}

#[cfg(test)]
mod tests {
    use super::*;

    fn shipped() -> Catalogue {
        let root = std::path::PathBuf::from(concat!(env!("CARGO_MANIFEST_DIR"), "/../.."));
        noctua_design_spec::i18n::load_for(
            root.join("spec/i18n"),
            &noctua_design_spec::component::load_all(root.join("spec/components"))
                .expect("the shipped components"),
        )
        .expect("catalogue")
    }

    fn file(path: &str) -> String {
        emit(&shipped())
            .into_iter()
            .find(|f| f.path == path)
            .unwrap_or_else(|| panic!("{path} was not emitted"))
            .contents
    }

    /// The whole promise: a key exists in both languages or in neither.
    #[test]
    fn both_languages_carry_every_key() {
        let catalogue = shipped();
        let rust = file("crates/noctua-design-i18n/src/lib.rs");
        let js = file("packages/tokens/i18n.js");
        let dts = file("packages/tokens/i18n.d.ts");

        for key in &catalogue.keys {
            assert!(rust.contains(&variant(key)), "Rust is missing {key}");
            assert!(js.contains(&format!("\"{key}\"")), "JS is missing {key}");
            assert!(
                dts.contains(&format!("| \"{key}\"")),
                "the types miss {key}"
            );
        }
    }

    /// Every locale's text is present, or the table has a hole the enum cannot
    /// express and the build would not compile.
    #[test]
    fn every_locale_reaches_both_outputs() {
        let catalogue = shipped();
        let rust = file("crates/noctua-design-i18n/src/lib.rs");
        let js = file("packages/tokens/i18n.js");
        for locale in &catalogue.locales {
            assert!(rust.contains(&variant(&locale.tag)), "{}", locale.tag);
            assert!(
                js.contains(&format!("\"{}\":", locale.tag)),
                "{}",
                locale.tag
            );
        }
    }

    /// The pseudo-locale would look like a bug to a reader who picked it by
    /// accident, so it is built but never offered.
    #[test]
    fn the_pseudo_locale_is_built_but_not_offered() {
        let rust = file("crates/noctua-design-i18n/src/lib.rs");
        let offered = rust
            .split("pub const OFFERED")
            .nth(1)
            .and_then(|rest| rest.split(';').next())
            .expect("an OFFERED list");
        assert!(!offered.contains(&variant(PSEUDO)), "{offered}");
        assert!(rust.contains(&format!("Self::{}", variant(PSEUDO))));
    }

    #[test]
    fn names_become_variants() {
        assert_eq!(variant("page.scoped-themes.title"), "PageScopedThemesTitle");
        assert_eq!(variant("pt-BR"), "PtBr");
        assert_eq!(variant("en"), "En");
    }

    #[test]
    fn quotes_are_escaped() {
        assert_eq!(escape(r#"a "b" c"#), r#"a \"b\" c"#);
        assert_eq!(escape(r"a\b"), r"a\\b");
    }
}
