//! The message catalogue: `spec/i18n/<locale>.toml`.
//!
//! One file per locale, all with the same keys. English is the source: a key
//! exists because it was added there, and every other locale is checked against
//! it **in both directions**. A key missing from a translation and a key left
//! behind in one after English dropped it are both build failures naming the
//! key, because both ship a page with a hole in it.
//!
//! # The catalogue is a list of sentences, not of words
//!
//! `NOCTUA.md §9.9` rule 1: a message is never assembled from fragments. Word
//! order, gender and agreement differ between languages, and a fragment that
//! reads correctly in English can be ungrammatical once translated. The type
//! here cannot prevent a caller concatenating two lookups — a source gate does
//! that — but the *catalogue* is shaped so there is no reason to.

use std::collections::BTreeSet;
use std::path::{Path, PathBuf};

use indexmap::IndexMap;
use serde::Deserialize;

use crate::error::{Problem, SpecError};

/// The locale every other one is checked against.
pub const SOURCE: &str = "en";

/// The generated locale that exists to break layouts on purpose.
///
/// Not a language: it is English, marked and lengthened, so a screenshot at
/// 320 px shows which strings were hard-coded (they stay unmarked) and which
/// layouts only fit because English is short.
pub const PSEUDO: &str = "qps";

/// Every locale, loaded and cross-checked.
#[derive(Debug, Clone)]
pub struct Catalogue {
    /// Each locale, source first, in the order the files were read.
    pub locales: Vec<Locale>,
    /// Every key, in the order the source locale declares them.
    pub keys: Vec<String>,
}

impl Catalogue {
    /// The source locale.
    ///
    /// # Panics
    ///
    /// Never: [`load`] fails before constructing a catalogue without one.
    #[must_use]
    pub fn source(&self) -> &Locale {
        self.locales
            .iter()
            .find(|locale| locale.tag == SOURCE)
            .expect("the source locale is loaded first")
    }
}

/// One locale's messages.
#[derive(Debug, Clone)]
pub struct Locale {
    /// The file stem: `en`, `pt-BR`.
    pub tag: String,
    /// Metadata for `<html lang>` and the language switcher.
    pub meta: Meta,
    /// Key to message.
    pub message: IndexMap<String, String>,
}

/// One locale's metadata.
#[derive(Debug, Clone, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Meta {
    /// The locale's name, written **in that locale**.
    ///
    /// A language switcher that lists languages in the language you are trying
    /// to leave is useless to the person who needs it.
    pub name: String,
    /// The BCP 47 tag for `<html lang>`.
    pub bcp47: String,
    /// Writing direction, for `<html dir>`.
    pub dir: String,
}

/// The file as parsed, before cross-locale checking.
#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct File {
    meta: Meta,
    #[serde(default)]
    message: IndexMap<String, String>,
}

/// Builds a diagnostic over a locale file.
///
/// The problems this module finds are *cross-file* — a key here and not there —
/// so they have no single span to underline. What they do have is a key name and
/// a fix, which is what a translator actually needs.
fn problem(path: &str, text: &str, message: String, fix: &str) -> SpecError {
    SpecError::new(
        path,
        text.to_owned(),
        vec![Problem::whole_file(message).fix(fix.to_owned())],
    )
}

/// Loads every locale under `spec/i18n/` and cross-checks them.
///
/// # Errors
///
/// A missing directory, a file that does not parse, a key present in one locale
/// and absent from another, or a message whose placeholders do not match the
/// source's.
pub fn load(directory: impl AsRef<Path>) -> Result<Catalogue, SpecError> {
    load_with(directory, &[])
}

/// The catalogue this repository actually ships: authored **and** derived.
///
/// One call, because a caller who used [`load`] and forgot the derivation got a
/// catalogue missing a hundred and thirty keys and a build failure with no clue
/// in it. Every consumer takes the components anyway; taking them here removes
/// the way to get it wrong.
///
/// # Errors
///
/// As [`load_with`].
pub fn load_for(
    directory: impl AsRef<Path>,
    components: &[crate::component::Component],
) -> Result<Catalogue, SpecError> {
    load_with(directory, &crate::component::derived_messages(components))
}

/// The same, plus English messages **derived from another spec**.
///
/// # Why anything is derived rather than written here
///
/// A component's contract already carries prose a reader sees: what it is for,
/// what each prop does, what each key does, what a reviewer has to know. That
/// prose becomes the doc comment in both languages, so it has to live in
/// `spec/components/<name>.toml` — and it was ALSO printed verbatim onto the
/// Portuguese page, because the site had nowhere else to get it from. Every
/// component page in pt-BR carried English prop documentation.
///
/// Copying it into `en.toml` would give two places to edit and one of them
/// would rot. Deriving it means English is written once, where it also becomes
/// the doc comment; Portuguese is written once, here, in `pt-BR.toml`; and the
/// cross-locale check below fails the build naming any derived key a translator
/// has not answered yet. Nothing else changes: a derived key is an ordinary key
/// from the moment it is loaded.
///
/// # Errors
///
/// As [`load`], plus a derived key that collides with an authored one — which
/// would mean the catalogue silently had two answers for one name.
///
/// # Panics
///
/// Never: the check above returns before the source locale is looked up, so it
/// is always present by the time it is.
pub fn load_with(
    directory: impl AsRef<Path>,
    derived: &[(String, String)],
) -> Result<Catalogue, SpecError> {
    let directory = directory.as_ref();
    let mut paths = read_locale_paths(directory)?;

    // The source first, so its key order is the catalogue's order and its
    // placeholders are the ones everything else is measured against.
    paths.sort_by_key(|path| (stem(path) != SOURCE, stem(path)));

    let mut locales = Vec::new();
    for path in &paths {
        locales.push(read(path)?);
    }

    if !locales.iter().any(|locale| locale.tag == SOURCE) {
        return Err(problem(
            &directory.display().to_string(),
            "",
            format!("spec/i18n/ has no {SOURCE}.toml"),
            "English is the source locale — every other one is checked against it, \
             in both directions — so it has to exist.",
        ));
    }

    let source_path = directory.join(format!("{SOURCE}.toml"));
    let source = locales
        .iter_mut()
        .find(|locale| locale.tag == SOURCE)
        .expect("the source locale is present");
    for (key, message) in derived {
        if source.message.contains_key(key) {
            return Err(problem(
                &source_path.display().to_string(),
                "",
                format!("`{key}` is both written here and derived from another spec"),
                "A key has one answer. Delete the authored one: the derived text is the \
                 same words the doc comment uses, and two copies of one sentence is one \
                 copy that goes stale.",
            ));
        }
        source.message.insert(key.clone(), message.clone());
    }

    let keys: Vec<String> = locales[0].message.keys().cloned().collect();
    check(&locales, &keys)?;
    locales.push(pseudo(&locales[0]));

    Ok(Catalogue { locales, keys })
}

fn read_locale_paths(directory: &Path) -> Result<Vec<PathBuf>, SpecError> {
    let entries = std::fs::read_dir(directory).map_err(|error| {
        problem(
            &directory.display().to_string(),
            "",
            format!("could not read {}: {error}", directory.display()),
            "Create spec/i18n/ with at least en.toml in it.",
        )
    })?;

    let mut paths = Vec::new();
    for entry in entries {
        let path = entry
            .map_err(|error| {
                problem(
                    &directory.display().to_string(),
                    "",
                    format!("could not read a locale file: {error}"),
                    "Check the permissions on spec/i18n/.",
                )
            })?
            .path();
        if path.extension().is_some_and(|e| e == "toml") {
            paths.push(path);
        }
    }
    Ok(paths)
}

fn stem(path: &Path) -> String {
    path.file_stem()
        .unwrap_or_default()
        .to_string_lossy()
        .into()
}

fn read(path: &Path) -> Result<Locale, SpecError> {
    let name = path.display().to_string();
    let text = std::fs::read_to_string(path).map_err(|error| {
        problem(
            &name,
            "",
            format!("could not read {name}: {error}"),
            "Check the file exists and is readable.",
        )
    })?;
    let file: File = toml::from_str(&text).map_err(|error| {
        problem(
            &name,
            &text,
            format!("{name} does not parse: {error}"),
            "Keys are quoted and dotted — \"nav.open\" — so TOML reads them as \
             literal strings rather than nesting them into tables.",
        )
    })?;

    let tag = stem(path);
    if file.meta.bcp47 != tag {
        return Err(problem(
            &name,
            &text,
            format!(
                "{name} declares bcp47 = \"{}\" but is named {tag}.toml",
                file.meta.bcp47
            ),
            "The file name is what names the built output, so the two have to agree.",
        ));
    }
    if tag == PSEUDO {
        return Err(problem(
            &name,
            &text,
            format!("{PSEUDO} is generated, so {name} must not exist"),
            "Delete it. A hand-written pseudo-locale stops tracking the source the \
             first time a message changes, which is the one thing it exists to catch.",
        ));
    }

    Ok(Locale {
        tag,
        meta: file.meta,
        message: file.message,
    })
}

/// Every locale has every key, in both directions, with matching placeholders.
fn check(locales: &[Locale], keys: &[String]) -> Result<(), SpecError> {
    let source = &locales[0];
    let name = |tag: &str| format!("spec/i18n/{tag}.toml");

    for locale in &locales[1..] {
        // Missing: named individually, because "3 keys missing" sends a
        // translator hunting and "settings.glow.note is missing" does not.
        for key in keys {
            if !locale.message.contains_key(key) {
                return Err(problem(
                    &name(&locale.tag),
                    "",
                    format!("{} is missing \"{key}\"", name(&locale.tag)),
                    "Every locale carries every key. A lookup that quietly fell back \
                     to English would ship a page in two languages and report nothing.",
                ));
            }
        }

        // And the other direction, which is the one that gets forgotten: a key
        // deleted from English but left here is dead weight nobody will find.
        for key in locale.message.keys() {
            if !source.message.contains_key(key) {
                return Err(problem(
                    &name(&locale.tag),
                    "",
                    format!(
                        "{} has \"{key}\", which {SOURCE}.toml does not",
                        name(&locale.tag)
                    ),
                    "English is the source locale: add the key there, or delete it here.",
                ));
            }
        }

        for key in keys {
            let ours = placeholders(&locale.message[key]);
            let theirs = placeholders(&source.message[key]);
            if ours != theirs {
                return Err(problem(
                    &name(&locale.tag),
                    "",
                    format!(
                        "{}'s \"{key}\" uses {ours:?} where {SOURCE}.toml uses {theirs:?}",
                        name(&locale.tag)
                    ),
                    "A placeholder the caller does not supply is rendered literally, \
                     so the reader sees the braces.",
                ));
            }
        }
    }

    Ok(())
}

/// The `{name}` placeholders in a message.
fn placeholders(message: &str) -> BTreeSet<String> {
    let mut found = BTreeSet::new();
    let mut rest = message;
    while let Some(open) = rest.find('{') {
        rest = &rest[open + 1..];
        if let Some(close) = rest.find('}') {
            found.insert(rest[..close].to_owned());
            rest = &rest[close + 1..];
        } else {
            break;
        }
    }
    found
}

/// Builds the pseudo-locale from the source.
///
/// Two jobs, both of which a reviewer can check at a glance:
///
/// * **Marked** — every message is bracketed, so a string that appears
///   unbracketed on the page was hard-coded and never went through the
///   catalogue.
/// * **Longer** — vowels are doubled, which lengthens the text by roughly a
///   third. Most European languages run longer than English, and a header that
///   fits only because English is terse fails here rather than in production.
///
/// Placeholders pass through untouched, or the expansion would break the very
/// substitution it is meant to exercise.
fn pseudo(source: &Locale) -> Locale {
    let message = source
        .message
        .iter()
        .map(|(key, value)| (key.clone(), stretch(value)))
        .collect();

    Locale {
        tag: PSEUDO.to_owned(),
        meta: Meta {
            name: "Pseudo".to_owned(),
            bcp47: PSEUDO.to_owned(),
            dir: source.meta.dir.clone(),
        },
        message,
    }
}

fn stretch(message: &str) -> String {
    let mut out = String::with_capacity(message.len() * 2);
    out.push('⟦');

    let mut in_placeholder = false;
    for ch in message.chars() {
        match ch {
            '{' => {
                in_placeholder = true;
                out.push(ch);
            }
            '}' => {
                in_placeholder = false;
                out.push(ch);
            }
            _ => {
                out.push(ch);
                if !in_placeholder && "aeiouAEIOU".contains(ch) {
                    out.push(ch);
                }
            }
        }
    }

    out.push('⟧');
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    fn shipped() -> Catalogue {
        let root = std::path::PathBuf::from(concat!(env!("CARGO_MANIFEST_DIR"), "/../.."));
        // `load_for`, never `load`: the shipped catalogue is the authored keys
        // plus the ones derived from the component contracts, and half of it
        // does not exist without them.
        let components = crate::component::load_all(root.join("spec/components"))
            .expect("the shipped components");
        load_for(root.join("spec/i18n"), &components).expect("the shipped catalogue")
    }

    #[test]
    fn the_shipped_catalogue_loads() {
        let catalogue = shipped();
        assert!(catalogue.keys.len() > 50, "{}", catalogue.keys.len());
        assert_eq!(catalogue.source().tag, SOURCE);
    }

    #[test]
    fn every_locale_has_every_key() {
        let catalogue = shipped();
        for locale in &catalogue.locales {
            assert_eq!(
                locale.message.len(),
                catalogue.keys.len(),
                "{} has a different number of keys",
                locale.tag
            );
        }
    }

    /// The pseudo-locale is generated, not authored, so it cannot fall behind.
    #[test]
    fn the_pseudo_locale_is_longer_and_keeps_its_placeholders() {
        let catalogue = shipped();
        let source = catalogue.source().clone();
        let pseudo = catalogue
            .locales
            .iter()
            .find(|locale| locale.tag == PSEUDO)
            .expect("a pseudo-locale");

        for key in &catalogue.keys {
            let original = &source.message[key];
            let expanded = &pseudo.message[key];
            assert!(
                expanded.chars().count() > original.chars().count(),
                "{key} did not lengthen"
            );
            assert_eq!(
                placeholders(original),
                placeholders(expanded),
                "{key} lost or gained a placeholder"
            );
        }
    }

    #[test]
    fn placeholders_are_found() {
        assert!(placeholders("no braces here").is_empty());
        assert_eq!(placeholders("{count} results").len(), 1);
        assert_eq!(placeholders("{a} and {b}").len(), 2);
    }

    #[test]
    fn a_placeholder_survives_the_stretch() {
        let stretched = stretch("{count} results");
        assert!(stretched.contains("{count}"), "{stretched}");
        assert!(stretched.contains('⟦') && stretched.contains('⟧'));
    }

    #[test]
    fn a_missing_key_is_named() {
        let source = Locale {
            tag: SOURCE.to_owned(),
            meta: Meta {
                name: "English".into(),
                bcp47: SOURCE.into(),
                dir: "ltr".into(),
            },
            message: [("a.b".to_owned(), "x".to_owned())].into_iter().collect(),
        };
        let other = Locale {
            tag: "pt-BR".to_owned(),
            message: IndexMap::new(),
            ..source.clone()
        };
        let error = check(&[source, other], &["a.b".to_owned()]).expect_err("a failure");
        assert!(
            error.problems().iter().any(|p| p.message().contains("a.b")),
            "the missing key was not named"
        );
    }
}
