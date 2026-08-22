//! The reference gate: every path and every name a document cites must exist.
//!
//! # Why this exists
//!
//! The other gates read code. This one reads what the code is described BY, and
//! it exists because that half had no mechanism at all: `NOCTUA.md §0.5` is
//! prose, `AGENTS.md` is prose, and prose that nobody can fail is how a
//! documented path outlives the file it names.
//!
//! Every one of these was live in this repository, each for weeks, and not one
//! of them was found by a compiler, a test or a review:
//!
//! - `AGENTS.md` sent readers to `crates/noctua-design/src/icon.rs` and
//!   `packages/react/src/icon.js` after `Icon` moved into `components/`.
//! - `NOCTUA.md` sent them to `docs-site/assets/owl.js`, deleted when
//!   `model.js` superseded it.
//! - Three documents and one `ATTRIBUTION.md` still pointed at
//!   `docs-site/PROVISIONAL.md` after it was renamed.
//! - `AGENTS.md` named a guard called
//!   `every_tone_this_file_writes_is_one_the_palette_declares`. The test is
//!   `every_tone_a_tree_names_is_one_the_palette_declares`, so an agent
//!   grepping for the guard it was told to read would find nothing and conclude
//!   there was none.
//!
//! A stale reference is worse than a missing one. A missing one asks the reader
//! to look; a stale one answers, and the answer is wrong.
//!
//! # What it checks
//!
//! Two kinds of citation, both written the same way — inside backticks, or as a
//! Markdown link target:
//!
//! - **A path with a directory in it** is a precise claim and must resolve
//!   exactly — against the citing document's own directory, or against the
//!   repository root. Both are how a reader resolves one, so either counts.
//! - **A bare filename** is not a claim about location. Prose says `shell.js`
//!   and means the one file of that name, so a basename anywhere in the tree
//!   answers it. This is the difference between naming a file and pointing at
//!   one, and it is why the first version of this gate reported 60 findings and
//!   4 defects.
//! - **A name** — a `snake_case` identifier with at least two underscores, which
//!   is the shape of a test or a function this repository names in prose — must
//!   appear somewhere in the repository's own text.
//!
//! # What it deliberately does not check
//!
//! **A section reference** (`NOCTUA.md §19.5`). The document is authored in the
//! workspace, one directory above a repository that is a git repository of its
//! own, and CI clones this one alone. A gate that fails in CI for a file that
//! cannot be there is a gate somebody switches off.
//!
//! **Anything whose first segment is not a real top-level entry here.**
//! `google/fonts` and `notofonts/noto-cjk` are upstream repositories, written
//! with a slash because that is how upstream names them. This repository's own
//! paths all begin with one of its own directories, so that is the test.
//!
//! **A name with one underscore.** `error_slot`, `more_label` and `hide_label`
//! are props, and a prop is named in prose far more often than a function is.
//! Two underscores is where the false positives stop and the guard names start;
//! it is a heuristic, and it is stated here rather than hidden in a regex.
//!
//! **Anything with a placeholder in it** — `<name>`, `{en,pt-BR}`, a `*`. A
//! reader knows to substitute; a gate that guessed would be guessing.
//!
//! # The escape hatch
//!
//! `allow-reference: <reason>` on the same line, for the citation this gate
//! cannot be right about: a sentence whose subject is that a file does NOT
//! exist. `spec/components/theme-scope.toml` is the worked case — the argument
//! for `ThemeScope` having no contract is a sentence naming the contract it
//! does not have. Same-line, and the reason is mandatory, for the reasons
//! `gates.rs` gives about its own marker.

use std::collections::BTreeSet;
use std::path::{Path, PathBuf};

use crate::gates::Finding;

/// Directories this gate never reads and never indexes, each for a reason.
const SKIP: [&str; 7] = [
    ".git",
    "target",
    "node_modules",
    // Build output. Absent from a fresh clone, so a path into it is not a
    // claim this gate can check — and every byte of it came from a source file
    // that was checked.
    "docs-site/dist",
    // The staging area. Gitignored by design: `AGENTS.md` tells you to copy OUT
    // of it, and in CI there is nothing to copy.
    ".TEMP",
    // One module per drawing, 3,554 files, none of them prose. Indexing them
    // costs a second and answers nothing.
    "packages/tokens/icons",
    // Somebody else's minified React.
    "docs-site/assets/react",
];

/// Files whose text is read for citations.
const READ: [&str; 5] = ["md", "rs", "js", "css", "toml"];

/// The marker that excuses one line from this gate.
const ALLOW: &str = "allow-reference:";

/// A citation this gate cannot check, and why each one is here.
///
/// Every entry is a claim that something is out of scope, not that it is
/// correct. Kept as a list rather than a rule so that adding one is a visible
/// decision.
const UNCHECKABLE: [&str; 9] = [
    // Authored in the workspace, one directory above this repository. CI clones
    // this repository alone, so the file is legitimately absent.
    "NOCTUA.md",
    "NOCTUA-PLAN.md",
    "AGENTS.local.md",
    // Another repository's committed output and its own policy document, named
    // in this one's prose. Both are real; neither is here.
    "system/",
    "TOKEN-POLICY.md",
    // Build output and staging. Both are absent from a fresh clone, and this
    // gate runs before the build that would write the first.
    "docs-site/dist/",
    ".TEMP",
    // A running process, not a file on disk.
    "target/debug/xtask",
    // The name every built document has and no source file does. The URL
    // scheme is described in prose here; the files are written by the build.
    "index.html",
];

/// Runs the gate.
///
/// # Errors
///
/// Only a filesystem failure. A dangling reference is a returned value, so one
/// pass reports every one of them.
pub(crate) fn run(root: &Path) -> Result<Vec<Finding>, String> {
    let mut paths = Vec::new();
    let mut tree = Tree::default();
    index(root, root, &mut paths, &mut tree)?;
    paths.sort();
    tree.top_level = tree
        .present
        .iter()
        .filter(|entry| !entry.contains('/'))
        .cloned()
        .collect();

    // Every name the repository's CODE contains, which is what a cited name is
    // checked against. A spec file counts exactly as much as Rust — the
    // question is whether the thing exists, not what language it is in.
    //
    // A NAME COUNTS WHERE IT IS DECLARED, NEVER WHERE IT IS MENTIONED, and
    // this took two attempts to get right — both failures of exactly the kind
    // `NOCTUA.md §19.5` describes, in the gate written to enforce it.
    //
    // Built from every readable file, the set contained the DOCUMENTS doing the
    // citing, so a guard name invented in `AGENTS.md` was found in `AGENTS.md`
    // and the check passed on its own citation. Excluding Markdown was not
    // enough either: this module's own header names the defect it catches, and
    // a comment is prose wherever it sits. So comment lines are skipped, and
    // this file is skipped whole — its tests are made of counterexamples, and a
    // gate that indexes its own counterexamples can never fire.
    let mut names = BTreeSet::new();
    let mut documents = Vec::new();
    for path in &paths {
        let Ok(text) = std::fs::read_to_string(path) else {
            continue;
        };
        let relative = relative(root, path);
        let declaring =
            path.extension().is_some_and(|e| e != "md") && relative != "xtask/src/references.rs";
        if declaring {
            // Per language, because the marker is. `#` opens a comment in TOML
            // and opens an ATTRIBUTE in Rust — treating it as a comment there
            // hid every `#[serde(deny_unknown_fields)]` in the repository and
            // reported the attribute as a name that does not exist.
            let toml = path.extension().is_some_and(|e| e == "toml");
            for line in text.lines() {
                let trimmed = line.trim_start();
                let comment = if toml {
                    trimmed.starts_with('#')
                } else {
                    trimmed.starts_with("//")
                        || trimmed.starts_with('*')
                        || trimmed.starts_with("/*")
                };
                if comment {
                    continue;
                }
                for word in line.split(|c: char| !c.is_ascii_alphanumeric() && c != '_') {
                    if word.contains('_') {
                        names.insert(word.to_owned());
                    }
                }
            }
        }
        documents.push((relative, text));
    }

    let mut findings = Vec::new();
    for (file, text) in &documents {
        inspect(file, text, &tree, &names, &mut findings);
    }
    findings.sort_by(|a, b| (&a.file, a.line).cmp(&(&b.file, b.line)));
    Ok(findings)
}

/// What exists, in the three shapes a citation can be checked against.
#[derive(Debug, Default)]
struct Tree {
    /// Every repository-relative path, directories included, forward slashes.
    present: BTreeSet<String>,
    /// Every filename, wherever it sits. What a bare citation is answered by.
    basenames: BTreeSet<String>,
    /// The repository's own top-level entries, which is how a path of this
    /// repository is told from an upstream repository's name.
    top_level: BTreeSet<String>,
}

/// Collects every readable file, and every path that exists.
fn index(
    root: &Path,
    directory: &Path,
    files: &mut Vec<PathBuf>,
    tree: &mut Tree,
) -> Result<(), String> {
    let entries = std::fs::read_dir(directory)
        .map_err(|error| format!("could not read {}: {error}", directory.display()))?;
    let mut paths: Vec<PathBuf> = Vec::new();
    for entry in entries {
        paths.push(
            entry
                .map_err(|error| format!("could not read an entry: {error}"))?
                .path(),
        );
    }
    paths.sort();

    for path in paths {
        let rel = relative(root, &path);
        // The three bare names are matched at any depth, for the reason
        // `gates::walk` states: a second package in the pnpm workspace puts a
        // `node_modules` somewhere a root-anchored entry never sees.
        if rel
            .split('/')
            .any(|part| matches!(part, ".git" | "target" | "node_modules"))
        {
            continue;
        }
        if SKIP
            .iter()
            .any(|skip| rel == *skip || rel.starts_with(&format!("{skip}/")))
        {
            // Recorded, then not descended into. A skipped directory still
            // EXISTS, and `spec/components/icon.toml` cites one — the reason
            // it is skipped is cost, not absence.
            tree.present.insert(rel.clone());
            tree.present.insert(format!("{rel}/"));
            continue;
        }
        tree.present.insert(rel.clone());
        if let Some(name) = path.file_name() {
            tree.basenames.insert(name.to_string_lossy().into_owned());
        }
        if path.is_dir() {
            // Recorded with a trailing slash too, because a document that cites
            // a directory writes one: `assets/fonts/`.
            tree.present.insert(format!("{rel}/"));
            index(root, &path, files, tree)?;
        } else if path
            .extension()
            .is_some_and(|e| READ.iter().any(|w| e == *w))
        {
            files.push(path);
        }
    }
    Ok(())
}

/// A repository-relative path with forward slashes.
fn relative(root: &Path, path: &Path) -> String {
    path.strip_prefix(root)
        .unwrap_or(path)
        .to_string_lossy()
        .replace('\\', "/")
}

/// Reads one document for citations.
fn inspect(
    file: &str,
    text: &str,
    tree: &Tree,
    names: &BTreeSet<String>,
    findings: &mut Vec<Finding>,
) {
    // A generated file quotes whatever the spec told it to, and is verified
    // against that spec by the drift check.
    if text.contains("Generated by noctua-design — do not edit.") {
        return;
    }
    let directory = file.rsplit_once('/').map_or("", |(head, _)| head);
    // A document is written for a reader navigating this repository. A source
    // comment is written beside code, and cites runtime paths as freely as
    // repository ones — see [`check`].
    let prose = Path::new(file)
        .extension()
        .is_some_and(|e| e.eq_ignore_ascii_case("md"));

    for (number, line) in text.lines().enumerate() {
        if line.contains(ALLOW) || line.contains(crate::gates::ALLOW) {
            continue;
        }
        for citation in citations(line) {
            if let Some(message) = check(&citation, directory, prose, tree, names) {
                findings.push(Finding {
                    file: file.to_owned(),
                    line: number + 1,
                    message,
                });
            }
        }
    }
}

/// Every backticked run and Markdown link target on one line.
///
/// Backticks are how this repository writes a path or a name in prose, in
/// Markdown and in a doc comment alike, so one extractor covers both. A link
/// target is the other form a Markdown document uses, and it is the one a
/// reader clicks.
fn citations(line: &str) -> Vec<String> {
    let mut found = Vec::new();

    let mut rest = line;
    while let Some(open) = rest.find('`') {
        let after = &rest[open + 1..];
        let Some(close) = after.find('`') else { break };
        found.push(after[..close].to_owned());
        rest = &after[close + 1..];
    }

    // `[text](target)` — the target only. Written across two lines by a
    // formatter it is simply not seen, which is a gap and not a false answer.
    let mut rest = line;
    while let Some(open) = rest.find("](") {
        let after = &rest[open + 2..];
        let Some(close) = after.find(')') else { break };
        found.push(after[..close].to_owned());
        rest = &after[close + 1..];
    }

    found
}

/// What is wrong with one citation, if anything.
fn check(
    citation: &str,
    directory: &str,
    prose: bool,
    tree: &Tree,
    names: &BTreeSet<String>,
) -> Option<String> {
    let citation = citation.trim();
    if citation.is_empty() || citation.len() > 200 {
        return None;
    }
    // A placeholder is for a reader to substitute. A gate that resolved one
    // would be inventing the value.
    if citation.contains([
        '<', '>', '{', '}', '*', ' ', '"', '\'', '(', ')', '[', ']', '|', '$', '\\', '=', ',', ';',
        '…',
    ]) {
        return None;
    }
    if citation.starts_with("http")
        || citation.starts_with('#')
        || citation.starts_with('/')
        || citation.starts_with("../..")
        || citation.contains("://")
        // An extension on its own — `.d.ts`, `.rs` — names a KIND of file.
        || (citation.starts_with('.') && !citation.contains('/'))
    {
        return None;
    }
    if UNCHECKABLE
        .iter()
        .any(|known| citation == *known || citation.starts_with(known))
    {
        return None;
    }

    match shape(citation, tree) {
        // A DOCUMENT's path is a claim about where something is: it is written
        // for a reader navigating the repository, and it must resolve.
        Shape::Path if prose => check_path(citation, directory, tree),
        // A SOURCE COMMENT's is as often a runtime URL (`assets/model.js`, as
        // the built site serves it) or an import specifier (`./theme.js`,
        // resolved against `dist/`) as a repository path. Neither exists at the
        // cited location in this tree and both are correct, so what is checked
        // there is the weaker, still useful claim: the file is named `x`.
        Shape::Path => {
            let last = citation.trim_end_matches('/').rsplit('/').next()?;
            if EXTENSIONS.iter().any(|e| last.ends_with(e)) {
                check_filename(last, tree)
            } else {
                None
            }
        }
        Shape::Filename => check_filename(citation, tree),
        Shape::Name => check_name(citation, names),
        Shape::Neither => None,
    }
}

/// What a citation is, which is the whole of this gate's judgement.
#[derive(Debug, PartialEq, Eq)]
enum Shape {
    /// Carries a directory, and its first segment is one of this repository's
    /// own. A precise claim about where something is.
    Path,
    /// A bare filename with a real extension. Names a file without saying where.
    Filename,
    /// A `snake_case` identifier — a test or a function named in prose.
    Name,
    /// Everything else: an expression, an upstream repository, a word.
    Neither,
}

/// Extensions this repository's own files actually carry.
///
/// Not "anything with a dot": `Intl.formatToParts`, `element.offsetParent` and
/// `0.25rem` are all dotted and none of them is a file.
const EXTENSIONS: [&str; 14] = [
    ".rs", ".js", ".ts", ".css", ".md", ".toml", ".json", ".html", ".svg", ".glb", ".txt",
    ".woff2", ".jsonl", ".lock",
];

fn shape(citation: &str, tree: &Tree) -> Shape {
    if citation.contains("::") {
        return Shape::Neither;
    }
    // An extension on its own — `.rs`, `.d.ts` — is prose naming a KIND of
    // file. There has to be a name in front of it for the citation to be one.
    let extension = EXTENSIONS
        .iter()
        .find(|e| citation.ends_with(**e))
        .filter(|e| citation.len() > e.len());

    if let Some((first, _)) = citation.split_once('/') {
        // `google/fonts` and `notofonts/noto-cjk` are upstream repositories,
        // written with a slash because that is how upstream names them. A path
        // of THIS repository starts with one of its own entries — or with `.`,
        // which is a document pointing beside itself.
        return if first == ".." || first == "." || tree.top_level.contains(first) {
            Shape::Path
        } else {
            Shape::Neither
        };
    }
    if extension.is_some() {
        return Shape::Filename;
    }
    if citation
        .chars()
        .all(|c| c.is_ascii_lowercase() || c.is_ascii_digit() || c == '_')
        && citation.contains('_')
    {
        return Shape::Name;
    }
    Shape::Neither
}

/// A path resolves against the citing document, or against the repository root.
fn check_path(citation: &str, directory: &str, tree: &Tree) -> Option<String> {
    let bare = citation.strip_suffix('/').unwrap_or(citation);
    if tree.present.contains(bare) || tree.present.contains(citation) {
        return None;
    }
    // A reader of `docs-site/README.md` resolves `assets/shell.css` against
    // `docs-site/`, and `../AGENTS.md` against the root. Both are ordinary, so
    // both count.
    if let Some(resolved) = resolve(directory, bare) {
        if tree.present.contains(&resolved) {
            return None;
        }
    }
    Some(format!(
        "`{citation}` does not exist. A stale path answers the reader, and the answer is wrong — \
         fix it, or say `allow-reference: <reason>` on this line if the absence is the point"
    ))
}

/// A bare filename is answered by that name anywhere in the tree.
fn check_filename(citation: &str, tree: &Tree) -> Option<String> {
    if tree.basenames.contains(citation) {
        return None;
    }
    Some(format!(
        "no file in this repository is called `{citation}`. Naming one that is not here sends a \
         reader looking for it — fix it, or say `allow-reference: <reason>` on this line"
    ))
}

/// Joins a citation to the citing document's directory, resolving `..`.
fn resolve(directory: &str, citation: &str) -> Option<String> {
    let mut parts: Vec<&str> = if directory.is_empty() {
        Vec::new()
    } else {
        directory.split('/').collect()
    };
    for part in citation.split('/') {
        match part {
            "." | "" => {}
            ".." => {
                parts.pop()?;
            }
            other => parts.push(other),
        }
    }
    Some(parts.join("/"))
}

/// A name this repository writes in prose has to be a name it contains.
///
/// Two underscores is the floor, and [the module header](self) says why.
fn check_name(citation: &str, names: &BTreeSet<String>) -> Option<String> {
    let bare = citation.strip_suffix("()").unwrap_or(citation);
    if bare.matches('_').count() < 2 {
        return None;
    }
    if !bare
        .chars()
        .all(|c| c.is_ascii_lowercase() || c.is_ascii_digit() || c == '_')
    {
        return None;
    }
    if names.contains(bare) {
        return None;
    }
    Some(format!(
        "`{citation}` is named here and exists nowhere in this repository. An agent told to read \
         a guard by name finds none and concludes there is none"
    ))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn set(items: &[&str]) -> BTreeSet<String> {
        items.iter().map(|s| (*s).to_owned()).collect()
    }

    /// A tree with these paths in it, indexed the three ways [`Tree`] holds.
    fn tree(paths: &[&str]) -> Tree {
        let mut tree = Tree::default();
        for path in paths {
            tree.present.insert((*path).to_owned());
            if let Some(name) = path.rsplit('/').next() {
                tree.basenames.insert(name.to_owned());
            }
            tree.top_level.insert(
                path.split_once('/')
                    .map_or(*path, |(first, _)| first)
                    .to_owned(),
            );
        }
        tree
    }

    #[test]
    fn a_path_that_does_not_exist_is_reported() {
        let tree = tree(&["docs-site/README.md"]);
        assert!(
            check_path("docs-site/PROVISIONAL.md", "", &tree).is_some(),
            "the rename this gate was written for has to be caught"
        );
        assert!(check_path("docs-site/README.md", "", &tree).is_none());
    }

    #[test]
    fn a_moved_file_is_reported_at_the_path_it_left() {
        // `Icon` moved into `components/` and two documents were not told.
        let tree = tree(&[
            "crates/noctua-design/src/components/icon.rs",
            "packages/react/src/components/icon.js",
        ]);
        assert!(check_path("crates/noctua-design/src/icon.rs", "", &tree).is_some());
        assert!(check_path("packages/react/src/icon.js", "", &tree).is_some());
        assert!(check_path("crates/noctua-design/src/components/icon.rs", "", &tree).is_none());
    }

    #[test]
    fn a_path_resolves_against_the_document_that_cites_it() {
        let tree = tree(&["AGENTS.md", "docs-site/assets/shell.css"]);
        // Both of these are in `docs-site/README.md` today.
        assert!(check_path("../AGENTS.md", "docs-site", &tree).is_none());
        assert!(check_path("assets/shell.css", "docs-site", &tree).is_none());
        // And the same citation from the root is a different claim.
        assert!(check_path("assets/shell.css", "", &tree).is_some());
    }

    #[test]
    fn a_directory_may_be_cited_with_or_without_its_slash() {
        let tree = tree(&["assets/fonts", "assets/fonts/"]);
        assert!(check_path("assets/fonts/", "", &tree).is_none());
        assert!(check_path("assets/fonts", "", &tree).is_none());
    }

    #[test]
    fn a_bare_filename_is_answered_by_that_name_anywhere() {
        // Prose says `shell.js`, and means the one file of that name. Requiring
        // a full path here is what made the first version of this gate report
        // sixty findings and four defects.
        let tree = tree(&["docs-site/assets/shell.js"]);
        assert!(check_filename("shell.js", &tree).is_none());
        assert!(check_filename("owl.js", &tree).is_some());
    }

    #[test]
    fn a_name_that_exists_nowhere_is_reported() {
        let names = set(&["every_tone_a_tree_names_is_one_the_palette_declares"]);
        assert!(
            check_name(
                "every_tone_this_file_writes_is_one_the_palette_declares",
                &names
            )
            .is_some(),
            "the misremembered guard name has to be caught"
        );
        assert!(
            check_name(
                "every_tone_a_tree_names_is_one_the_palette_declares",
                &names
            )
            .is_none()
        );
    }

    #[test]
    fn a_source_comment_cites_runtime_paths_and_is_read_more_weakly() {
        // `assets/` has to be a real top-level entry for `assets/model.js` to
        // be read as a path of this repository at all.
        let tree = tree(&["docs-site/assets/model.js", "assets/fonts/ATTRIBUTION.md"]);
        let names = BTreeSet::new();
        // In a DOCUMENT this is a claim about where the file is, and it is
        // wrong: nothing sits at `assets/model.js` from the root.
        assert!(check("assets/model.js", "", true, &tree, &names).is_some());
        // In a SOURCE COMMENT the same string is the URL the built site serves,
        // and the checkable claim is only that a file of that name exists.
        assert!(check("assets/model.js", "", false, &tree, &names).is_none());
        assert!(check("assets/owl.js", "", false, &tree, &names).is_some());
    }

    #[test]
    fn a_prop_name_is_not_a_guard_name() {
        // One underscore, and no entry: still silent, because a prop is named
        // in prose far more often than a function is and this gate would
        // otherwise be a gate somebody switches off.
        assert!(check_name("error_slot", &BTreeSet::new()).is_none());
        assert!(check_name("more_label", &BTreeSet::new()).is_none());
    }

    #[test]
    fn a_placeholder_is_not_a_claim() {
        let tree = tree(&["spec/components/button.toml"]);
        let names = BTreeSet::new();
        for citation in [
            "spec/components/<name>.toml",
            "spec/i18n/{en,pt-BR}.toml",
            "dist/assets/search-<locale>.json",
            "https://example.invalid/x.md",
            "../../NOCTUA-PLAN.md",
            "NOCTUA.md",
            "/favicon.ico",
        ] {
            assert!(
                check(citation, "", true, &tree, &names).is_none(),
                "{citation} is not a claim this gate can check"
            );
        }
    }

    #[test]
    fn an_upstream_repository_is_not_a_path_here() {
        // These are in `assets/fonts/ATTRIBUTION.md`, written with a slash
        // because that is how upstream names them.
        let tree = tree(&["assets/fonts/ATTRIBUTION.md", "spec/tokens.toml"]);
        for citation in ["google/fonts", "notofonts/noto-cjk", "skfb.ly/pwnos"] {
            assert_eq!(shape(citation, &tree), Shape::Neither, "{citation}");
        }
        assert_eq!(shape("spec/tokens.toml", &tree), Shape::Path);
    }

    #[test]
    fn a_dotted_expression_is_not_a_file() {
        let tree = tree(&["docs-site/assets/shell.css"]);
        assert_eq!(shape("Intl.formatToParts", &tree), Shape::Neither);
        assert_eq!(shape("noctua_design::tones::ALL", &tree), Shape::Neither);
        assert_eq!(shape("0.25rem", &tree), Shape::Neither);
        assert_eq!(shape("shell.css", &tree), Shape::Filename);
        // An extension alone names a KIND of file, not one — rejected in
        // `check`, before a shape is asked for, because `.d.ts` is otherwise a
        // perfectly well-formed filename ending in `.ts`.
        let names = BTreeSet::new();
        assert!(check(".rs", "", true, &tree, &names).is_none());
        assert!(check(".d.ts", "", true, &tree, &names).is_none());
    }

    #[test]
    fn a_backticked_run_and_a_link_target_are_both_citations() {
        let found = citations("see [`the map`](docs-site/README.md) and `AGENTS.md`");
        assert!(found.contains(&"the map".to_owned()));
        assert!(found.contains(&"AGENTS.md".to_owned()));
        assert!(found.contains(&"docs-site/README.md".to_owned()));
    }

    #[test]
    fn the_repository_has_no_dangling_reference() {
        // The gate run against the repository it guards. Not a unit test of the
        // matcher: the thing this exists to prevent is a document in THIS tree
        // pointing at something that is not in it.
        let root = Path::new(env!("CARGO_MANIFEST_DIR"))
            .parent()
            .expect("xtask sits one directory below the repository root");
        let findings = run(root).expect("the repository can be read");
        assert!(
            findings.is_empty(),
            "dangling reference(s):\n{}",
            findings
                .iter()
                .map(|f| format!("  {}:{} — {}", f.file, f.line, f.message))
                .collect::<Vec<_>>()
                .join("\n")
        );
    }
}
