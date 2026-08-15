//! The register every reader-facing sentence is written in.
//!
//! # Why this is a check and not a style guide
//!
//! It was a style guide. The prose was rewritten once against a written rule —
//! shorter, one claim per sentence, every claim checkable — and it came back in
//! the same voice, because a house voice pulls harder than a paragraph of
//! instructions. The developer's verdict on the result was one word.
//!
//! So the rule is mechanical now. It does not make writing good; it makes a
//! specific, measured set of habits impossible, and those habits are what the
//! reader was complaining about:
//!
//! - **35 similes** across the contracts — *reads as a person*, *is furniture*,
//!   *is a promise the markup does not keep*.
//! - **101 fields** where a component *announces*, *refuses*, *insists* or
//!   *remembers*.
//! - **319 of 1,059 fields** carrying an em dash, most of them fastening a
//!   second subject to a sentence that had finished.
//! - Project history addressed to a reader who never saw it: *they were ten
//!   pages*, *an earlier draft*, *none of them was ever ours*.
//!
//! # What it does not do
//!
//! It does not check that a sentence is true, or useful, or that it names the
//! right prop. It cannot. Those stay a person's job, and this exists so the
//! person is reading for those things rather than for em dashes.

use crate::error::Problem;

/// Words that may be capitalised because they are names, not emphasis.
const ACRONYMS: [&str; 28] = [
    "ISO", "URLS", "ARIA", "WCAG", "HTML", "CSS", "DOM", "URL", "URI", "RTL", "LTR", "SVG", "JSON",
    "TOML", "API", "UI", "ID", "AA", "AAA", "CSV", "PNG", "JPG", "GIF", "PDF", "SI", "OK", "GOV",
    "NOCTUA",
];

/// Constructions this documentation does not use, in English.
///
/// Each one was found in the prose it replaced, and each marks a sentence that
/// is doing something other than saying what the thing is: a simile, a verdict,
/// a piece of the project's history, or an argument with a reader who has not
/// spoken.
const BANNED: [&str; 24] = [
    "reads as",
    "reads like",
    "would read",
    "looks like",
    "is furniture",
    "the whole point",
    "is the point",
    "and that is",
    "which is why",
    "used to be",
    "an earlier",
    "the version before",
    "is a promise",
    "is a lie",
    "that lies",
    "nobody",
    "somebody",
    "anybody",
    "for ever",
    "worth keeping",
    "worth having",
    "the truth",
    "honestly",
    "of course",
];

/// The same, in Portuguese.
const BANNED_PT: [&str; 16] = [
    "lê-se como",
    "soa como",
    "parece um",
    "é mobília",
    "a razão inteira",
    "o ponto inteiro",
    "e isso é",
    "por isso é que",
    "costumava ser",
    "uma versão anterior",
    "a versão antes",
    "é uma promessa",
    "que mente",
    "ninguém",
    "alguém",
    "para sempre",
];

/// How long one kind of string may be, in characters.
///
/// Ceilings rather than targets. A summary that says what a component is for in
/// forty characters is finished at forty.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Kind {
    /// The one line under a page title.
    Summary,
    /// When to reach for a component, and when not to.
    Guidance,
    /// What a component does for assistive technology.
    Accessibility,
    /// What one prop or one state does.
    Doc,
    /// What one key does.
    Key,
    /// The sentence under a worked example.
    Note,
}

impl Kind {
    /// The most characters this kind of string may run to.
    const fn ceiling(self) -> usize {
        match self {
            Self::Summary => 100,
            Self::Key => 120,
            Self::Note => 140,
            Self::Doc | Self::Guidance | Self::Accessibility => 250,
        }
    }

    /// What to call it in a message.
    const fn name(self) -> &'static str {
        match self {
            Self::Summary => "a summary",
            Self::Guidance => "guidance",
            Self::Accessibility => "an accessibility note",
            Self::Doc => "a prop or state doc",
            Self::Key => "a keyboard action",
            Self::Note => "an example note",
        }
    }
}

/// The most words one sentence may run to before it is two sentences.
const SENTENCE: usize = 25;

/// Every way one string breaks the register.
///
/// `where` names the string for a reader who has to find it: a file and a
/// field, or a message key.
#[must_use]
pub fn check(text: &str, kind: Kind, whose: &str, portuguese: bool) -> Vec<Problem> {
    let mut problems = Vec::new();
    let flat = text.split_whitespace().collect::<Vec<_>>().join(" ");
    let say = |what: String, fix: &str| Problem::whole_file(format!("{whose}: {what}")).fix(fix);

    if flat.chars().count() > kind.ceiling() {
        problems.push(say(
            format!(
                "{} runs to {} characters, over the {} this kind allows",
                kind.name(),
                flat.chars().count(),
                kind.ceiling()
            ),
            "Say what it is and what it does. A reason belongs here only when it \
             changes what the caller writes.",
        ));
    }

    // AN EM DASH FASTENS A SECOND SUBJECT TO A FINISHED SENTENCE, and that is
    // what it was being used for: 319 of 1,059 fields carried one. A semicolon
    // is the same move with different punctuation.
    if flat.contains('—') || flat.contains(';') {
        problems.push(say(
            "uses an em dash or a semicolon".to_owned(),
            "Write two sentences, or drop the second clause. Both marks were \
             joining a second subject to a sentence that had already finished.",
        ));
    }

    for sentence in flat.split(['.', '!', '?']) {
        let words = sentence.split_whitespace().count();
        if words > SENTENCE {
            problems.push(say(
                format!("has a sentence of {words} words"),
                "One idea per sentence. Over twenty-five words it is two ideas \
                 and a reader has to hold the first one to reach the second.",
            ));
        }
    }

    // Capitals were being used for emphasis inside reader-facing text. A name
    // is not emphasis, so the acronyms are allowed and nothing else is.
    //
    // A BACKTICKED RUN IS CODE AND IS SKIPPED. `tones::ALL` and `AA` are
    // identifiers a caller types, not a raised voice, and a rule that made
    // them unsayable would push a real constant out of the documentation.
    let mut prose = String::with_capacity(flat.len());
    let mut code = false;
    for character in flat.chars() {
        if character == '`' {
            code = !code;
        } else if !code {
            prose.push(character);
        }
    }
    for word in prose.split(|c: char| !c.is_ascii_alphabetic()) {
        let shouted = word.len() >= 3
            && word.chars().all(|c| c.is_ascii_uppercase())
            && !ACRONYMS.contains(&word);
        if shouted {
            problems.push(say(
                format!("shouts `{word}`"),
                "Capitals are not emphasis. Use `**bold**` if a claim needs \
                 weight, and usually it does not.",
            ));
        }
    }

    // A SUMMARY IS A SENTENCE ABOUT WHAT A READER CAN DO, NOT A DEFINITION.
    //
    // This is the rule the developer's own example turns on, and the only one
    // of theirs that nothing else here would have caught. What they rejected —
    // "One choice out of many, or one thing a reader agrees to, at a size a
    // thumb can find." — is inside every other limit on this page: no em dash,
    // no shout, no banned phrase, eighty-four characters, one sentence. It is
    // still wrong, because it is a noun phrase. What they asked for instead —
    // "Let users select one or more options by using the checkboxes component."
    // — opens on a verb and says what the component is FOR.
    //
    // So: a summary may not open on an article or a quantifier. That leaves a
    // verb, which is the whole of the fix.
    if kind == Kind::Summary && !portuguese {
        const NOUN_PHRASE: [&str; 12] = [
            "a", "an", "the", "one", "two", "three", "every", "each", "some", "any", "content",
            "text",
        ];
        let opening = flat
            .split_whitespace()
            .next()
            .unwrap_or_default()
            .to_lowercase();
        if NOUN_PHRASE.contains(&opening.as_str()) {
            problems.push(say(
                format!("opens on `{opening}`, so it is a definition rather than a sentence"),
                "Say what a reader can do with it, starting on a verb: \
                 `Let users select one or more options.`",
            ));
        }
    }

    let lower = flat.to_lowercase();
    let banned: &[&str] = if portuguese { &BANNED_PT } else { &BANNED };
    // ON WORD BOUNDARIES. `for ever` matched inside `for every`, which is a
    // phrase this documentation uses and needs.
    let bounded = format!(" {} ", lower.replace(['.', ',', ':', '`'], " "));
    for phrase in banned {
        if bounded.contains(&format!(" {phrase} ")) {
            problems.push(say(
                format!("uses `{phrase}`"),
                "This documentation says what a thing is and what it does. It \
                 does not compare it to something else, rank it, or tell the \
                 reader what it used to be.",
            ));
        }
    }

    problems
}

/// What kind of string a message key holds, or nothing if it is a label.
///
/// The catalogue is mostly labels — a column heading, a button's text, a row of
/// sample data — and a label is not prose. Only the keys whose shape says
/// "sentence" are checked, and the shape is the suffix.
///
/// `example.*` IS BOTH, AND THE FULL STOP TELLS THEM APART. That namespace
/// holds 245 sentences under pictures and 541 strings inside them — a column
/// heading, a button's text, a row of sample data. Which is which is decided by
/// a macro in `docs-site/src/examples.rs`, and this crate does not know about
/// that crate. What it can see is punctuation: a label does not end in a full
/// stop and a sentence does. The few content strings that do end in one are
/// error messages, and holding those to the same register is right anyway.
#[must_use]
pub fn kind_of(key: &str) -> Option<Kind> {
    if key.starts_with("example.") {
        return None;
    }
    // `clippy::case_sensitive_file_extension_comparisons` reads these as file
    // extensions. They are message-key suffixes, and a key is lowercase by
    // construction, so the case-insensitive form it suggests would be slower
    // and no more correct.
    #[allow(clippy::case_sensitive_file_extension_comparisons)]
    if key.ends_with(".guidance") {
        Some(Kind::Guidance)
    } else if key.ends_with(".a11y-notes") {
        Some(Kind::Accessibility)
    } else if key.ends_with(".doc") {
        Some(Kind::Doc)
    } else if key.ends_with(".action") {
        Some(Kind::Key)
    } else if key.ends_with(".summary") {
        Some(Kind::Summary)
    } else if key.ends_with(".body") {
        // A `.body` is a paragraph on a page — the intro, the colour rules,
        // the spacing scale — not a sentence under a picture, so it gets the
        // paragraph ceiling rather than the note one.
        Some(Kind::Guidance)
    } else {
        None
    }
}

/// Every `example.*` key that is a NOTE rather than content.
///
/// That namespace holds sentences under pictures and strings inside them: a
/// column heading, a button's text, a paragraph of sample prose shown to
/// demonstrate `Prose`. Only the first kind is documentation.
///
/// A heuristic was tried first — a label does not end in a full stop and a
/// sentence does — and it was wrong in the expensive direction: it held a
/// paragraph of demonstration text to the ceiling meant for a one-line note.
/// So the list is read from the file that decides it. This crate does not
/// depend on the docs site; it reads two of its files as text, under
/// `cfg(test)`, the same way it already reads `spec/`.
#[cfg(test)]
fn note_keys(root: &std::path::Path) -> std::collections::BTreeSet<String> {
    let examples = std::fs::read_to_string(root.join("docs-site/src/examples.rs"))
        .expect("the examples are readable");
    let catalogue = std::fs::read_to_string(root.join("crates/noctua-design-i18n/src/lib.rs"))
        .expect("the generated catalogue is readable");

    // `Self::ExampleButtonMenu => "example.button.menu",`
    let mut named = std::collections::BTreeMap::new();
    for line in catalogue.lines() {
        let Some((left, right)) = line.split_once("=>") else {
            continue;
        };
        let Some(variant) = left.trim().strip_prefix("Self::") else {
            continue;
        };
        let Some(open) = right.find('"') else {
            continue;
        };
        let rest = &right[open + 1..];
        if let Some(end) = rest.find('"') {
            named.insert(variant.trim().to_owned(), rest[..end].to_owned());
        }
    }

    // The three-argument arm of `example!` is `(source, note, render)`. A note
    // is the SECOND argument, so it sits alone on its own line at the macro's
    // own indent. A `Key::` inside a render closure is indented deeper.
    let mut keys = std::collections::BTreeSet::new();
    for line in examples.lines() {
        let Some(rest) = line.strip_prefix("        Key::") else {
            continue;
        };
        let variant = rest.trim().trim_end_matches(',');
        if let Some(key) = named.get(variant) {
            keys.insert(key.clone());
        }
    }
    keys
}

#[cfg(test)]
mod tests {
    use super::{Kind, check, kind_of, note_keys};

    #[test]
    fn the_register_the_developer_asked_for_passes() {
        // The sentence the developer wrote, verbatim, as the target.
        let wanted = "Let users select one or more options by using the checkboxes component.";
        assert!(check(wanted, Kind::Summary, "checkbox", false).is_empty());
    }

    #[test]
    fn the_register_the_developer_rejected_does_not() {
        // What that summary said before, verbatim.
        let was =
            "One choice out of many, or one thing a reader agrees to, at a size a thumb can find.";
        // It is inside the length ceiling, so length alone would have let it
        // through. What catches it has to be the shape.
        assert!(was.chars().count() < 100);
        assert!(!check(was, Kind::Summary, "checkbox", false).is_empty());
    }

    #[test]
    fn an_em_dash_is_caught_and_so_is_a_semicolon() {
        assert_eq!(
            check("A row of things — a real one.", Kind::Doc, "x", false).len(),
            1
        );
        assert_eq!(check("A row; a real one.", Kind::Doc, "x", false).len(), 1);
    }

    #[test]
    fn an_acronym_is_a_name_and_a_shout_is_not() {
        assert!(check("The role is ARIA `grid`.", Kind::Doc, "x", false).is_empty());
        assert_eq!(
            check("On the BUTTON, not here.", Kind::Doc, "x", false).len(),
            1
        );
    }

    #[test]
    fn a_long_sentence_is_two_ideas() {
        let long = "The size of everybody in the group stated once and it wins over an avatar's \
                    own size because siblings share their box and a row of faces would disagree.";
        assert!(!check(long, Kind::Doc, "x", false).is_empty());
    }

    #[test]
    fn portuguese_is_checked_against_its_own_list() {
        assert!(
            !check(
                "um título sobre uma frase é mobília.",
                Kind::Note,
                "x",
                true
            )
            .is_empty()
        );
        assert!(
            check(
                "A segunda linha é buscada junto com o nome.",
                Kind::Note,
                "x",
                true
            )
            .is_empty()
        );
    }

    /// A summary is written twice, and the two copies must say the same thing.
    ///
    /// `[meta] summary` in the contract becomes the Rust doc comment on the
    /// generated component; `component.<slug>.summary` in `en.toml` becomes the
    /// line under the page title. Nothing derives one from the other and
    /// nothing compared them, so they drifted: `ListItem` said "One item in a
    /// list, or one term and its description" in the contract and "One item in
    /// a list." on its own page.
    #[test]
    fn a_summary_says_the_same_thing_in_both_places() {
        let root = std::path::PathBuf::from(concat!(env!("CARGO_MANIFEST_DIR"), "/../.."));
        let text = std::fs::read_to_string(root.join("spec/i18n/en.toml"))
            .expect("the catalogue is readable");
        let file: toml::Value = toml::from_str(&text).expect("the catalogue parses");
        let messages = file
            .get("message")
            .and_then(toml::Value::as_table)
            .expect("the catalogue has messages");

        let mut apart = Vec::new();
        for component in
            crate::component::load_all(root.join("spec/components")).expect("the contracts parse")
        {
            let slug = crate::component::slug(&component.meta.name);
            let key = format!("component.{slug}.summary");
            let Some(said) = messages.get(&key).and_then(toml::Value::as_str) else {
                apart.push(format!("{key} is not in the catalogue"));
                continue;
            };
            if said != component.meta.summary {
                apart.push(format!(
                    "{slug}:\n  contract: {}\n  catalogue: {said}",
                    component.meta.summary
                ));
            }
        }
        assert!(
            apart.is_empty(),
            "{} summary/summaries disagree with themselves:\n{}",
            apart.len(),
            apart.join("\n")
        );
    }

    /// The gate itself, over everything the site shows.
    ///
    /// A TEST rather than a step in the load path, deliberately: wired into
    /// parsing it would stop `cargo xtask build` and `cargo xtask docs` while
    /// the prose is being rewritten, and the rewrite needs a site to look at.
    /// `cargo xtask check` runs the tests, so it is still a gate — it just
    /// holds at the end rather than in the middle.
    #[test]
    fn every_reader_facing_string_is_written_plainly() {
        let root = std::path::PathBuf::from(concat!(env!("CARGO_MANIFEST_DIR"), "/../.."));
        let mut found: Vec<String> = Vec::new();

        for component in
            crate::component::load_all(root.join("spec/components")).expect("the contracts parse")
        {
            let slug = crate::component::slug(&component.meta.name);
            let mut say = |text: &str, kind: Kind, what: &str| {
                for problem in check(text, kind, &format!("{slug}: {what}"), false) {
                    found.push(problem.message().to_owned());
                }
            };
            say(&component.meta.summary, Kind::Summary, "summary");
            if !component.guidance.is_empty() {
                say(&component.guidance, Kind::Guidance, "guidance");
            }
            if !component.a11y.notes.is_empty() {
                say(&component.a11y.notes, Kind::Accessibility, "a11y notes");
            }
            for (name, prop) in &component.prop {
                say(&prop.doc, Kind::Doc, &format!("prop.{name}"));
            }
            for (name, state) in &component.state {
                say(&state.doc, Kind::Doc, &format!("state.{name}"));
            }
            for (chord, action) in &component.keyboard {
                say(action, Kind::Key, &format!("key {chord}"));
            }
        }

        let notes = note_keys(&root);
        for tag in ["en", "pt-BR"] {
            let text = std::fs::read_to_string(root.join(format!("spec/i18n/{tag}.toml")))
                .expect("the catalogue is readable");
            let file: toml::Value = toml::from_str(&text).expect("the catalogue parses");
            let Some(messages) = file.get("message").and_then(toml::Value::as_table) else {
                continue;
            };
            for (key, value) in messages {
                let kind = kind_of(key).or_else(|| notes.contains(key).then_some(Kind::Note));
                let (Some(sentence), Some(kind)) = (value.as_str(), kind) else {
                    continue;
                };
                for problem in check(sentence, kind, &format!("{tag}.toml {key}"), tag != "en") {
                    found.push(problem.message().to_owned());
                }
            }
        }

        assert!(
            found.is_empty(),
            "{} string(s) are not in the register this documentation is written in:\n{}",
            found.len(),
            found.join("\n")
        );
    }
}
