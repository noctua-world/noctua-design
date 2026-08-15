//! The generated `noctua-design-grammars` crate, and its JavaScript twin.
//!
//! | Output | For |
//! |---|---|
//! | `crates/noctua-design-grammars/src/lib.rs` | the roles, the languages, the tables |
//! | `packages/tokens/grammars.js` + `.d.ts` | the same, for React and for anyone without a framework |
//! | `dist/grammars.json` | the manifest a documentation page reads |
//! | `dist/grammars.cases.json` | the blessed expectation **both** scanners are checked against |
//!
//! # What is generated and what is not
//!
//! The **tables** are generated, from `spec/grammars/`. The **algorithm** is
//! not: it is `crates/noctua-design-grammars/src/scan.rs` and its twin
//! `scan.js`, hand-written side by side, and this module copies the second into
//! the JavaScript package verbatim. Generating an algorithm from a spec would
//! mean inventing a language to describe it in, and that language would need a
//! compiler nobody asked for.
//!
//! # Why the cases go to `dist/` rather than into a test
//!
//! There are two scanners and they have to agree. Comparing them **to each
//! other** passes whenever they are both wrong in the same way, which is the
//! failure that actually happens — one shared misreading of the spec, written
//! down twice. So the expectation is a file: the Rust test asserts against it,
//! a Playwright test asserts against it, and a disagreement names which half
//! moved.

use std::fmt::Write as _;

use noctua_design_spec::grammar::{
    Comment, Context, Grammar, Identifier, Number, PropertyKey, Role, Sigil, StringForm, Symbols,
};

use crate::{CommentStyle, EmittedFile, header_from, identifier};

/// The grammar directory, as written in generated headers.
pub const GRAMMAR_SPEC: &str = "spec/grammars/";

/// Emits every grammar artifact.
///
/// `scanner` is the contents of `crates/noctua-design-grammars/src/scan.js`,
/// read from disk by `xtask` the way the icon drawings are — it is vendored
/// input to this emitter, not something this emitter can invent.
#[must_use]
pub fn emit(grammars: &[Grammar], scanner: &str) -> Vec<EmittedFile> {
    vec![
        EmittedFile::new(
            "crates/noctua-design-grammars/src/lib.rs",
            rust_crate(grammars),
        ),
        EmittedFile::new("packages/tokens/grammars.js", js(grammars, scanner)),
        EmittedFile::new("packages/tokens/grammars.d.ts", js_types(grammars)),
        EmittedFile::new("dist/grammars.json", manifest(grammars)),
        EmittedFile::new("dist/grammars.cases.json", cases(grammars)),
    ]
}

// --- The Rust crate ---------------------------------------------------------

fn rust_crate(grammars: &[Grammar]) -> String {
    let mut out = header_from(CommentStyle::Line("//!"), GRAMMAR_SPEC);
    out.push_str(
        "//!\n\
         //! # Every language, as a table\n\
         //!\n\
         //! The scanner that reads these is [`scan`], and it is hand-written —\n\
         //! see `scan.rs` beside this file, and `scan.js` beside that, which is\n\
         //! the same algorithm for the JavaScript package.\n\
         //!\n\
         //! Concatenating every token's text reproduces the input exactly. That\n\
         //! invariant is asserted for every case, every prefix of every case,\n\
         //! and every grammar against every other grammar's cases.\n\
         \n\
         #![no_std]\n\
         // A GENERATED TABLE, and both allows are about that rather than about\n\
         // code somebody wrote. Two languages legitimately agree about a word,\n\
         // and merging the arms that happen to agree today would make the next\n\
         // language that disagrees a structural change rather than a file.\n\
         #![allow(clippy::match_same_arms)]\n\
         #![allow(clippy::too_many_lines)]\n\
         \n\
         mod scan;\n\
         \n\
         pub use scan::{\n\
         \x20   Comment, Context, Grammar, Identifier, Mode, Number, PropertyKey, Sigil, StringForm,\n\
         \x20   Symbols, Token, lines, scan, scan_language,\n\
         };\n\n",
    );

    write_role(&mut out);
    write_language(&mut out, grammars);
    for grammar in grammars {
        write_grammar(&mut out, grammar);
    }
    write_cases(&mut out, grammars);
    write_tests(&mut out);
    out
}

fn write_role(out: &mut String) {
    out.push_str(
        "/// What a run of characters is, as far as a colour is concerned.\n\
         ///\n\
         /// Seventeen, and several share a paint: five hues is the measured\n\
         /// ceiling for a categorical set in this colour system, so a larger\n\
         /// role set buys expressiveness in the grammar rather than in the\n\
         /// theme. `spec/syntax.toml` must paint every one of them.\n\
         #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]\n\
         pub enum Role {\n",
    );
    for role in Role::ALL {
        let _ = writeln!(out, "    /// `{}`.", role.as_str());
        let _ = writeln!(out, "    {},", role.variant());
    }
    out.push_str("}\n\nimpl Role {\n");
    let _ = writeln!(
        out,
        "    /// Every role, in the order the scanner declares them.\n    \
         pub const ALL: [Self; {}] = [{}];\n",
        Role::ALL.len(),
        Role::ALL
            .iter()
            .map(|role| format!("Self::{}", role.variant()))
            .collect::<Vec<_>>()
            .join(", ")
    );
    out.push_str(
        "    /// Its `data-nd-tok` value, which is what the stylesheet keys on.\n    \
         #[must_use]\n    \
         pub const fn as_str(self) -> &'static str {\n        \
         match self {\n",
    );
    for role in Role::ALL {
        let _ = writeln!(
            out,
            "            Self::{} => \"{}\",",
            role.variant(),
            role.as_str()
        );
    }
    out.push_str("        }\n    }\n}\n\n");
}

fn write_language(out: &mut String, grammars: &[Grammar]) {
    out.push_str(
        "/// Every language a code block can be shown in.\n\
         ///\n\
         /// An enum rather than a string, for the reason `IconName` is one: a\n\
         /// typo is a compile error instead of an unhighlighted block nobody\n\
         /// notices. The runtime spelling is [`Language::parse`], for the case\n\
         /// where the name really does arrive as text — a markdown fence.\n\
         #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]\n\
         pub enum Language {\n",
    );
    for grammar in grammars {
        let _ = writeln!(out, "    /// {}.", grammar.meta.display);
        let _ = writeln!(out, "    {},", variant(&grammar.meta.name));
    }
    out.push_str("}\n\nimpl Language {\n");

    let _ = writeln!(
        out,
        "    /// Every language, in the order `spec/grammars/` lists them.\n    \
         pub const ALL: [Self; {}] = [{}];\n",
        grammars.len(),
        grammars
            .iter()
            .map(|g| format!("Self::{}", variant(&g.meta.name)))
            .collect::<Vec<_>>()
            .join(", ")
    );

    out.push_str(
        "    /// Its name, which is also its `data-nd-language` value.\n    \
         #[must_use]\n    pub const fn as_str(self) -> &'static str {\n        match self {\n",
    );
    for grammar in grammars {
        let _ = writeln!(
            out,
            "            Self::{} => \"{}\",",
            variant(&grammar.meta.name),
            grammar.meta.name
        );
    }
    out.push_str("        }\n    }\n\n");

    out.push_str(
        "    /// How it is written for a reader.\n    \
         #[must_use]\n    pub const fn display(self) -> &'static str {\n        match self {\n",
    );
    for grammar in grammars {
        let _ = writeln!(
            out,
            "            Self::{} => \"{}\",",
            variant(&grammar.meta.name),
            escape(&grammar.meta.display)
        );
    }
    out.push_str("        }\n    }\n\n");

    out.push_str(
        "    /// Its table.\n    #[must_use]\n    \
         pub const fn grammar(self) -> &'static Grammar {\n        match self {\n",
    );
    for grammar in grammars {
        let _ = writeln!(
            out,
            "            Self::{} => &{},",
            variant(&grammar.meta.name),
            constant(&grammar.meta.name)
        );
    }
    out.push_str("        }\n    }\n\n");

    // ONE NAME, ONE GRAMMAR — including the aliases, which `load_all` already
    // proved unique. A `match` rather than a table walk so the compiler builds
    // the jump for us and an added alias costs nothing at run time.
    out.push_str(
        "    /// The language a name or an alias resolves to.\n    ///\n    \
         /// `None` rather than a fallback: a block whose language nobody\n    \
         /// recognises is shown as plain text by the caller, because a wrong\n    \
         /// grammar is confidently wrong and no colour is merely quiet.\n    \
         #[must_use]\n    pub fn parse(name: &str) -> Option<Self> {\n        \
         match name {\n",
    );
    for grammar in grammars {
        let mut names = alloc_names(grammar);
        names.sort();
        names.dedup();
        let arms = names
            .iter()
            .map(|name| format!("\"{name}\""))
            .collect::<Vec<_>>()
            .join(" | ");
        let _ = writeln!(
            out,
            "            {arms} => Some(Self::{}),",
            variant(&grammar.meta.name)
        );
    }
    out.push_str("            _ => None,\n        }\n    }\n}\n\n");
}

fn alloc_names(grammar: &Grammar) -> Vec<String> {
    std::iter::once(grammar.meta.name.clone())
        .chain(grammar.meta.aliases.iter().cloned())
        .collect()
}

fn write_grammar(out: &mut String, grammar: &Grammar) {
    let name = &grammar.meta.name;
    let _ = writeln!(
        out,
        "/// The {} table. See `spec/grammars/{name}.toml`.",
        grammar.meta.display
    );
    let _ = writeln!(out, "pub static {}: Grammar = Grammar {{", constant(name));
    let _ = writeln!(out, "    name: \"{name}\",");
    let _ = writeln!(out, "    display: \"{}\",", escape(&grammar.meta.display));
    let _ = writeln!(out, "    aliases: &[{}],", list(&grammar.meta.aliases));
    let _ = writeln!(out, "    mode: Mode::{},", grammar.meta.mode.variant());
    write_identifier(out, &grammar.identifier);
    write_words(out, grammar);
    write_comments(out, &grammar.comments);
    write_strings(out, &grammar.strings);
    write_number(out, &grammar.number);
    write_symbols(out, &grammar.symbols);
    write_sigils(out, &grammar.sigils);
    write_context(out, &grammar.context);
    out.push_str("};\n\n");
}

fn write_identifier(out: &mut String, rules: &Identifier) {
    let _ = writeln!(
        out,
        "    identifier: Identifier {{ extra: \"{}\", unicode: {} }},",
        escape(&rules.extra),
        rules.unicode
    );
}

fn write_words(out: &mut String, grammar: &Grammar) {
    if grammar.words.is_empty() {
        out.push_str("    words: &[],\n");
        return;
    }
    out.push_str("    words: &[\n");
    for (role, list) in &grammar.words {
        let mut sorted = list.clone();
        sorted.sort();
        for word in sorted {
            let _ = writeln!(
                out,
                "        (\"{}\", Role::{}),",
                escape(&word),
                role.variant()
            );
        }
    }
    out.push_str("    ],\n");
}

fn write_comments(out: &mut String, forms: &[Comment]) {
    if forms.is_empty() {
        out.push_str("    comments: &[],\n");
        return;
    }
    out.push_str("    comments: &[\n");
    for form in forms {
        let _ = writeln!(
            out,
            "        Comment {{ open: \"{}\", close: \"{}\", nestable: {} }},",
            escape(&form.open),
            escape(&form.close),
            form.nestable
        );
    }
    out.push_str("    ],\n");
}

fn write_strings(out: &mut String, forms: &[StringForm]) {
    if forms.is_empty() {
        out.push_str("    strings: &[],\n");
        return;
    }
    out.push_str("    strings: &[\n");
    for form in forms {
        out.push_str("        StringForm {\n");
        let _ = writeln!(out, "            open: \"{}\",", escape(&form.open));
        let _ = writeln!(out, "            close: \"{}\",", escape(&form.close));
        let _ = writeln!(out, "            escape: \"{}\",", escape(&form.escape));
        let _ = writeln!(out, "            multiline: {},", form.multiline);
        let _ = writeln!(out, "            max: {},", form.max);
        let _ = writeln!(
            out,
            "            interpolate_open: \"{}\",",
            escape(&form.interpolate_open)
        );
        let _ = writeln!(
            out,
            "            interpolate_close: \"{}\",",
            escape(&form.interpolate_close)
        );
        let _ = writeln!(
            out,
            "            interpolate_sigil: \"{}\",",
            escape(&form.interpolate_sigil)
        );
        out.push_str("        },\n");
    }
    out.push_str("    ],\n");
}

fn write_number(out: &mut String, number: &Number) {
    out.push_str("    number: Number {\n");
    let _ = writeln!(out, "        prefixes: &[{}],", list(&number.prefixes));
    let _ = writeln!(out, "        separator: \"{}\",", escape(&number.separator));
    let _ = writeln!(out, "        exponent: \"{}\",", escape(&number.exponent));
    let _ = writeln!(out, "        suffix: {},", number.suffix);
    let _ = writeln!(out, "        leading_dot: {},", number.leading_dot);
    out.push_str("    },\n");
}

fn write_symbols(out: &mut String, symbols: &Symbols) {
    let _ = writeln!(
        out,
        "    symbols: Symbols {{ operator: \"{}\", punctuation: \"{}\" }},",
        escape(&symbols.operator),
        escape(&symbols.punctuation)
    );
}

fn write_sigils(out: &mut String, sigils: &[Sigil]) {
    if sigils.is_empty() {
        out.push_str("    sigils: &[],\n");
        return;
    }
    out.push_str("    sigils: &[\n");
    for sigil in sigils {
        let _ = writeln!(
            out,
            "        Sigil {{ open: \"{}\", role: Role::{}, line_start: {} }},",
            escape(&sigil.open),
            sigil.role.variant(),
            sigil.line_start
        );
    }
    out.push_str("    ],\n");
}

fn write_context(out: &mut String, context: &Context) {
    out.push_str("    context: Context {\n");
    let _ = writeln!(out, "        call_paren: {},", context.call_paren);
    let _ = writeln!(out, "        upper_type: {},", context.upper_type);
    let _ = writeln!(
        out,
        "        screaming_constant: {},",
        context.screaming_constant
    );
    let _ = writeln!(out, "        member_dot: {},", context.member_dot);
    let _ = writeln!(
        out,
        "        property_colon: PropertyKey::{},",
        match context.property_colon {
            PropertyKey::None => "None",
            PropertyKey::String => "String",
            PropertyKey::Any => "Any",
        }
    );
    let _ = writeln!(out, "        property_equals: {},", context.property_equals);
    let _ = writeln!(
        out,
        "        string_prefix: \"{}\",",
        escape(&context.string_prefix)
    );
    let _ = writeln!(
        out,
        "        name_after: &[{}],",
        context
            .name_after
            .iter()
            .map(|(word, role)| format!("(\"{}\", Role::{})", escape(word), role.variant()))
            .collect::<Vec<_>>()
            .join(", ")
    );
    let _ = writeln!(
        out,
        "        case_insensitive: {},",
        context.case_insensitive
    );
    out.push_str("    },\n");
}

/// Every case, as a table the test module reads directly.
///
/// Emitted as Rust **and** as `dist/grammars.cases.json` from this one
/// function, so the two scanners are held to the same expectation and neither
/// artifact needs a parser: the Rust half gets a `const`, the JavaScript half
/// gets the file, and a drift between them would be a drift inside a single
/// `for` loop.
fn write_cases(out: &mut String, grammars: &[Grammar]) {
    let total: usize = grammars.iter().map(|g| g.cases.len()).sum();
    out.push_str(
        "/// One case: the language, what the case is called, the code, what must\n\
         /// appear in the token stream and what must not.\n\
         pub type Case = (Language, &'static str, &'static str, &'static [&'static str], \
         &'static [&'static str]);\n\n\
         /// Every case every grammar declares: what it is called, what it is,\n\
         /// what must appear in its token stream and what must not.\n\
         ///\n\
         /// `expect` is a **subsequence** rather than the whole stream. Naming\n\
         /// every token of a realistic sample would be thousands of lines\n\
         /// nobody reviews, and the stream in full is already guarded by the\n\
         /// totality property. What belongs here is the trap.\n",
    );
    let _ = writeln!(out, "pub const CASES: [Case; {total}] = [");
    for grammar in grammars {
        for case in &grammar.cases {
            let _ = writeln!(
                out,
                "    (Language::{}, \"{}\", \"{}\", &[{}], &[{}]),",
                variant(&grammar.meta.name),
                escape(&case.name),
                escape(&case.code),
                list(&case.expect),
                list(&case.reject),
            );
        }
    }
    out.push_str("];\n\n");
}

/// The five properties every grammar is held to, generated so a new language
/// inherits them by existing.
fn write_tests(out: &mut String) {
    out.push_str(TESTS);
}

const TESTS: &str = r#"#[cfg(test)]
mod generated_tests {
    extern crate alloc;
    extern crate std;

    use alloc::string::String;
    use alloc::vec::Vec;

    use super::{CASES, Language, Role, scan};

    /// The tokens `language` makes of `code`, rebuilt into the input it came
    /// from. Every property below is some statement about this pair.
    fn rebuild(language: Language, code: &str) -> String {
        let mut rebuilt = String::new();
        for token in scan(language.grammar(), code) {
            rebuilt.push_str(token.text);
        }
        rebuilt
    }

    /// THE INVARIANT EVERYTHING ELSE RESTS ON. A highlighter that is imperfect
    /// and provably never destructive is a different kind of artifact from one
    /// that is better and might swallow a line.
    #[test]
    fn concatenating_the_tokens_reproduces_the_input() {
        for (language, name, code, _, _) in CASES {
            std::assert_eq!(rebuild(language, code), code, "{} lost bytes in `{name}`", language.as_str());
        }
    }

    /// A grammar pointed at another language's code must not panic, loop or
    /// drop anything. It is what a reader does the first time they mislabel a
    /// block, and it costs one loop to be sure of.
    #[test]
    fn every_grammar_survives_every_other_language() {
        for (_, name, code, _, _) in CASES {
            for language in Language::ALL {
                std::assert_eq!(
                    rebuild(language, code),
                    code,
                    "{} lost bytes reading `{name}`",
                    language.as_str()
                );
            }
        }
    }

    /// Every prefix of every case, which is the deterministic form of "what
    /// happens inside an unterminated construct" — no seed, no flake, and it
    /// covers the truncated string, the truncated comment and the truncated
    /// tag without anybody having to think of them.
    #[test]
    fn a_truncated_input_still_terminates_and_keeps_every_byte() {
        for (language, name, code, _, _) in CASES {
            for at in 0..=code.len() {
                if !code.is_char_boundary(at) {
                    continue;
                }
                let piece = &code[..at];
                std::assert_eq!(
                    rebuild(language, piece),
                    piece,
                    "{} lost bytes in `{name}` cut at {at}",
                    language.as_str()
                );
            }
        }
    }

    /// What each case claims, which is where a grammar's real traps live.
    #[test]
    fn every_case_gets_what_it_expects_and_nothing_it_refuses() {
        for (language, name, code, expect, reject) in CASES {
            let tokens = scan(language.grammar(), code);
            let pairs: Vec<(Role, &str)> = tokens.iter().map(|t| (t.role, t.text)).collect();

            let mut at = 0;
            for wanted in expect {
                let (role, text) = pair(wanted);
                match pairs[at..].iter().position(|(r, t)| *r == role && *t == text) {
                    Some(offset) => at += offset + 1,
                    None => std::panic!(
                        "{}: `{name}` expected `{wanted}`; the tokens were {pairs:?}",
                        language.as_str()
                    ),
                }
            }

            for refused in reject {
                let (role, text) = pair(refused);
                std::assert!(
                    !pairs.iter().any(|(r, t)| *r == role && *t == text),
                    "{}: `{name}` produced `{refused}`, which it refuses",
                    language.as_str()
                );
            }
        }
    }

    /// A grammar that produces two roles is a grammar whose table did not load.
    /// Cheap to notice here, expensive to find in a browser.
    #[test]
    fn every_grammar_produces_more_than_two_roles_across_its_own_cases() {
        for language in Language::ALL {
            if language == Language::Plain {
                continue;
            }
            let mut seen: Vec<Role> = Vec::new();
            for (owner, _, code, _, _) in CASES {
                if owner != language {
                    continue;
                }
                for token in scan(language.grammar(), code) {
                    if !seen.contains(&token.role) {
                        seen.push(token.role);
                    }
                }
            }
            std::assert!(
                seen.len() > 2,
                "{} produced only {seen:?} across its own cases",
                language.as_str()
            );
        }
    }

    /// `<role> <text>`, split at the FIRST space, so the text may contain
    /// spaces and may be a single space itself.
    fn pair(entry: &str) -> (Role, &str) {
        let (role, text) = entry.split_once(' ').expect("`<role> <text>`");
        let role = Role::ALL
            .into_iter()
            .find(|candidate| candidate.as_str() == role)
            .expect("a known role");
        (role, text)
    }
}
"#;

// --- The JavaScript half ----------------------------------------------------

fn js(grammars: &[Grammar], scanner: &str) -> String {
    let mut out = header_from(CommentStyle::Block, GRAMMAR_SPEC);
    out.push_str(
        "\n// The tables are generated; the scanner below them is not. It is a\n\
         // transcription of crates/noctua-design-grammars/src/scan.rs, kept in the\n\
         // same directory as its twin and changed in the same commit. Both are\n\
         // checked against dist/grammars.cases.json, never against each other.\n\n",
    );

    let _ = writeln!(
        out,
        "/** Every role, in the order the scanner declares them. */\nexport const ROLES = [{}];\n",
        Role::ALL
            .iter()
            .map(|role| format!("\"{}\"", role.as_str()))
            .collect::<Vec<_>>()
            .join(", ")
    );

    out.push_str("/** Every language, keyed by its name. */\nexport const GRAMMARS = {\n");
    for grammar in grammars {
        js_grammar(&mut out, grammar);
    }
    out.push_str("};\n\n");

    out.push_str(
        "/** Every alias, and the language it resolves to. */\nexport const ALIASES = {\n",
    );
    for grammar in grammars {
        for alias in &grammar.meta.aliases {
            let _ = writeln!(out, "  \"{alias}\": \"{}\",", grammar.meta.name);
        }
    }
    out.push_str("};\n\n");

    out.push_str(scanner.trim_start_matches('\n'));
    out
}

// One long function on purpose: it is a single object literal written
// field by field, and splitting it would put half an object in one place
// and half in another.
#[allow(clippy::too_many_lines)]
fn js_grammar(out: &mut String, grammar: &Grammar) {
    let _ = writeln!(out, "  \"{}\": {{", grammar.meta.name);
    let _ = writeln!(out, "    name: \"{}\",", grammar.meta.name);
    let _ = writeln!(out, "    display: \"{}\",", escape(&grammar.meta.display));
    let _ = writeln!(out, "    aliases: [{}],", list(&grammar.meta.aliases));
    let _ = writeln!(
        out,
        "    mode: \"{}\",",
        grammar.meta.mode.variant().to_lowercase()
    );
    let _ = writeln!(
        out,
        "    identifier: {{ extra: \"{}\", unicode: {} }},",
        escape(&grammar.identifier.extra),
        grammar.identifier.unicode
    );

    out.push_str("    words: [");
    let mut first = true;
    for (role, words) in &grammar.words {
        let mut sorted = words.clone();
        sorted.sort();
        for word in sorted {
            if !first {
                out.push_str(", ");
            }
            first = false;
            let _ = write!(out, "[\"{}\", \"{}\"]", escape(&word), role.as_str());
        }
    }
    out.push_str("],\n");

    out.push_str("    comments: [");
    for (at, form) in grammar.comments.iter().enumerate() {
        if at > 0 {
            out.push_str(", ");
        }
        let _ = write!(
            out,
            "{{ open: \"{}\", close: \"{}\", nestable: {} }}",
            escape(&form.open),
            escape(&form.close),
            form.nestable
        );
    }
    out.push_str("],\n");

    out.push_str("    strings: [");
    for (at, form) in grammar.strings.iter().enumerate() {
        if at > 0 {
            out.push_str(", ");
        }
        let _ = write!(
            out,
            "{{ open: \"{}\", close: \"{}\", escape: \"{}\", multiline: {}, max: {}, \
             interpolateOpen: \"{}\", interpolateClose: \"{}\", interpolateSigil: \"{}\" }}",
            escape(&form.open),
            escape(&form.close),
            escape(&form.escape),
            form.multiline,
            form.max,
            escape(&form.interpolate_open),
            escape(&form.interpolate_close),
            escape(&form.interpolate_sigil)
        );
    }
    out.push_str("],\n");

    let _ = writeln!(
        out,
        "    number: {{ prefixes: [{}], separator: \"{}\", exponent: \"{}\", suffix: {}, \
         leadingDot: {} }},",
        list(&grammar.number.prefixes),
        escape(&grammar.number.separator),
        escape(&grammar.number.exponent),
        grammar.number.suffix,
        grammar.number.leading_dot
    );
    let _ = writeln!(
        out,
        "    symbols: {{ operator: \"{}\", punctuation: \"{}\" }},",
        escape(&grammar.symbols.operator),
        escape(&grammar.symbols.punctuation)
    );

    out.push_str("    sigils: [");
    for (at, sigil) in grammar.sigils.iter().enumerate() {
        if at > 0 {
            out.push_str(", ");
        }
        let _ = write!(
            out,
            "{{ open: \"{}\", role: \"{}\", lineStart: {} }}",
            escape(&sigil.open),
            sigil.role.as_str(),
            sigil.line_start
        );
    }
    out.push_str("],\n");

    let context = &grammar.context;
    let _ = writeln!(
        out,
        "    context: {{ callParen: {}, upperType: {}, screamingConstant: {}, memberDot: {}, \
         propertyColon: \"{}\", propertyEquals: {}, stringPrefix: \"{}\", nameAfter: [{}], \
         caseInsensitive: {} }},",
        context.call_paren,
        context.upper_type,
        context.screaming_constant,
        context.member_dot,
        match context.property_colon {
            PropertyKey::None => "none",
            PropertyKey::String => "string",
            PropertyKey::Any => "any",
        },
        context.property_equals,
        escape(&context.string_prefix),
        context
            .name_after
            .iter()
            .map(|(word, role)| format!("[\"{}\", \"{}\"]", escape(word), role.as_str()))
            .collect::<Vec<_>>()
            .join(", "),
        context.case_insensitive
    );
    out.push_str("  },\n");
}

fn js_types(grammars: &[Grammar]) -> String {
    let mut out = header_from(CommentStyle::Block, GRAMMAR_SPEC);
    out.push('\n');
    let _ = writeln!(
        out,
        "/** Every role, which is also every `data-nd-tok` value. */\nexport type Role =\n{};\n",
        Role::ALL
            .iter()
            .map(|role| format!("  | \"{}\"", role.as_str()))
            .collect::<Vec<_>>()
            .join("\n")
    );
    let _ = writeln!(
        out,
        "/** Every language a code block can be shown in. */\nexport type LanguageName =\n{};\n",
        grammars
            .iter()
            .map(|g| format!("  | \"{}\"", g.meta.name))
            .collect::<Vec<_>>()
            .join("\n")
    );
    out.push_str(
        "/** A run of the input, and what it is. */\n\
         export interface Token {\n\
         \x20 role: Role;\n\
         \x20 text: string;\n\
         }\n\n\
         /** One language, as data. The shape is `scan.js`'s, not an API. */\n\
         export type Grammar = Record<string, unknown>;\n\n\
         export declare const ROLES: readonly Role[];\n\
         export declare const GRAMMARS: Record<LanguageName, Grammar>;\n\
         export declare const ALIASES: Record<string, LanguageName>;\n\n\
         /** The language a name or an alias resolves to, or `null`. */\n\
         export declare function language(name: string | null | undefined): Grammar | null;\n\n\
         /** Splits `source` into coloured runs. Concatenating every `text`\n\
         \x20* reproduces `source` exactly. */\n\
         export declare function scan(grammar: Grammar | null, source: string): Token[];\n",
    );
    out
}

// --- The manifests ----------------------------------------------------------

fn manifest(grammars: &[Grammar]) -> String {
    let mut out = String::from("{\n");
    out.push_str(&generated_key());
    out.push_str("  \"languages\": [\n");
    for (at, grammar) in grammars.iter().enumerate() {
        let comma = if at + 1 == grammars.len() { "" } else { "," };
        let _ = writeln!(
            out,
            "    {{ \"name\": \"{}\", \"display\": \"{}\", \"mode\": \"{}\", \"aliases\": [{}] }}{comma}",
            grammar.meta.name,
            escape(&grammar.meta.display),
            grammar.meta.mode.variant().to_lowercase(),
            list(&grammar.meta.aliases)
        );
    }
    out.push_str("  ],\n");
    let _ = writeln!(
        out,
        "  \"roles\": [{}]\n}}",
        Role::ALL
            .iter()
            .map(|role| format!("\"{}\"", role.as_str()))
            .collect::<Vec<_>>()
            .join(", ")
    );
    out
}

/// The header line a JSON file carries, since JSON has no comment syntax.
fn generated_key() -> String {
    format!(
        "  \"_generated\": \"Generated by noctua-design — do not edit. Source: {GRAMMAR_SPEC}. \
         Regenerate: {}.\",\n",
        crate::REGENERATE
    )
}

/// The one expectation both scanners are checked against.
fn cases(grammars: &[Grammar]) -> String {
    let mut out = String::from("{\n");
    out.push_str(&generated_key());
    out.push_str("  \"cases\": [\n");
    let total: usize = grammars.iter().map(|g| g.cases.len()).sum();
    let mut written = 0;
    for grammar in grammars {
        for case in &grammar.cases {
            written += 1;
            let comma = if written == total { "" } else { "," };
            let _ = write!(
                out,
                "    {{\n      \"language\": \"{}\",\n      \"name\": \"{}\",\n      \
                 \"code\": \"{}\",\n      \"expect\": [{}],\n      \"reject\": [{}]\n    }}{comma}\n",
                grammar.meta.name,
                json(&case.name),
                json(&case.code),
                case.expect
                    .iter()
                    .map(|entry| format!("\"{}\"", json(entry)))
                    .collect::<Vec<_>>()
                    .join(", "),
                case.reject
                    .iter()
                    .map(|entry| format!("\"{}\"", json(entry)))
                    .collect::<Vec<_>>()
                    .join(", "),
            );
        }
    }
    out.push_str("  ]\n}\n");
    out
}

// --- Spelling ---------------------------------------------------------------

/// `kebab-case` to `PascalCase`, for an enum variant.
fn variant(name: &str) -> String {
    let mut out = String::new();
    for part in name.split(['-', '_']) {
        let mut chars = part.chars();
        if let Some(first) = chars.next() {
            out.extend(first.to_uppercase());
            out.push_str(chars.as_str());
        }
    }
    out
}

/// `kebab-case` to `SCREAMING_SNAKE_CASE`, for a static.
fn constant(name: &str) -> String {
    identifier(&name.replace('-', "_")).to_uppercase()
}

/// A Rust or JavaScript string literal's body.
fn escape(text: &str) -> String {
    let mut out = String::new();
    for c in text.chars() {
        match c {
            '\\' => out.push_str("\\\\"),
            '"' => out.push_str("\\\""),
            '\n' => out.push_str("\\n"),
            '\t' => out.push_str("\\t"),
            '\r' => out.push_str("\\r"),
            other => out.push(other),
        }
    }
    out
}

/// A JSON string's body.
fn json(text: &str) -> String {
    let mut out = String::new();
    for c in text.chars() {
        match c {
            '\\' => out.push_str("\\\\"),
            '"' => out.push_str("\\\""),
            '\n' => out.push_str("\\n"),
            '\t' => out.push_str("\\t"),
            '\r' => out.push_str("\\r"),
            other if (other as u32) < 0x20 => {
                let _ = write!(out, "\\u{:04x}", other as u32);
            }
            other => out.push(other),
        }
    }
    out
}

fn list(items: &[String]) -> String {
    items
        .iter()
        .map(|item| format!("\"{}\"", escape(item)))
        .collect::<Vec<_>>()
        .join(", ")
}

#[cfg(test)]
mod tests {
    use noctua_design_spec::grammar::Mode;

    use super::*;

    fn shipped() -> Vec<Grammar> {
        let path = concat!(env!("CARGO_MANIFEST_DIR"), "/../../spec/grammars");
        noctua_design_spec::grammar::load_all(path).expect("the shipped grammars")
    }

    #[test]
    fn the_shipped_grammars_validate() {
        let grammars = shipped();
        assert!(grammars.iter().any(|g| g.meta.name == "rust"));
        assert!(grammars.iter().any(|g| g.meta.mode == Mode::Plain));
    }

    /// The two halves must offer the same languages under the same names, or a
    /// React consumer gets a blank block where the Rust one is highlighted.
    #[test]
    fn both_languages_ship_the_same_names() {
        let grammars = shipped();
        let rust = rust_crate(&grammars);
        let script = js(&grammars, "// scanner\n");
        for grammar in &grammars {
            assert!(
                rust.contains(&format!("Self::{}", variant(&grammar.meta.name))),
                "{} is missing from the Rust crate",
                grammar.meta.name
            );
            assert!(
                script.contains(&format!("\"{}\": {{", grammar.meta.name)),
                "{} is missing from the JavaScript table",
                grammar.meta.name
            );
        }
    }

    /// Every case reaches the fixture, because the fixture is what both
    /// scanners are held to and a case that never left the spec holds nobody.
    #[test]
    fn every_case_reaches_the_fixture() {
        let grammars = shipped();
        let fixture = cases(&grammars);
        let total: usize = grammars.iter().map(|g| g.cases.len()).sum();
        assert!(total > 0, "no grammar declares a case");
        assert_eq!(fixture.matches("\"language\"").count(), total);
    }

    #[test]
    fn a_name_becomes_a_variant_and_a_static() {
        assert_eq!(variant("objective-c"), "ObjectiveC");
        assert_eq!(constant("objective-c"), "OBJECTIVE_C");
    }

    /// A grammar whose words contain a quote or a backslash would otherwise
    /// emit source that does not compile.
    #[test]
    fn a_delimiter_that_is_a_backslash_survives_being_written_down() {
        assert_eq!(escape("\\"), "\\\\");
        assert_eq!(escape("\""), "\\\"");
        assert_eq!(json("a\nb"), "a\\nb");
    }
}
