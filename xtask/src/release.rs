//! Preparing a version: writing it everywhere it appears.
//!
//! # What this does not do
//!
//! It does not commit and it does not tag. Every commit in this fleet is signed
//! with a hardware key that needs a physical touch, so a verb that tried would
//! hang on a machine nobody is sitting at. It prints the three commands
//! instead, in the order they have to run.
//!
//! # Why it runs the gate first
//!
//! Because a version is a promise about a state of the tree, and the cheapest
//! moment to discover the tree is not in that state is before the number is
//! written into every manifest that names one — `plan` is the list.
//! `noctua-colors` learned this in the same order.

use std::path::Path;

use crate::{check, ui};

/// Prepares `version` everywhere the repository names one.
///
/// # Errors
///
/// On a version that is not three integers, on a version already released, on a
/// failing gate, or on a manifest whose formatting moved out from under the
/// rewrite below.
pub(crate) fn run(root: &Path, spec: &Path, version: &str, dry_run: bool) -> Result<(), String> {
    let parts: Vec<&str> = version.split('.').collect();
    if parts.len() != 3 || parts.iter().any(|part| part.parse::<u32>().is_err()) {
        return Err(format!(
            "`{version}` is not a version. Expected three numbers, such as 0.2.0"
        ));
    }

    let manifest = root.join("Cargo.toml");
    let text = std::fs::read_to_string(&manifest)
        .map_err(|error| format!("could not read Cargo.toml: {error}"))?;
    let current = workspace_version(&text)?;
    if current == version {
        return Err(format!(
            "the version is already {version}. A release that changes nothing is a tag, and \
             `git tag` is what makes one"
        ));
    }

    ui::heading("release");
    ui::detail(&format!("{current} → {version}"));

    if dry_run {
        for (path, count) in plan(root, &current)? {
            ui::detail(&format!("would write {count} occurrence(s) in {path}"));
        }
        ui::ok("nothing was written");
        return Ok(());
    }

    // The gate before the writes, so a release cannot be prepared from a tree
    // that does not pass its own checks.
    check::run(root, spec, false)?;

    let mut written = 0usize;
    for (path, _) in plan(root, &current)? {
        let full = root.join(&path);
        let before = std::fs::read_to_string(&full)
            .map_err(|error| format!("could not read {path}: {error}"))?;
        let after = rewrite(&path, &before, &current, version)?;
        std::fs::write(&full, after).map_err(|error| format!("could not write {path}: {error}"))?;
        written += 1;
    }
    ui::ok(&format!("{written} file(s) now say {version}"));

    relock(root)?;
    ui::ok("Cargo.lock names the new version");

    ui::gap();
    ui::detail("Then, with the key in the machine:");
    ui::detail(&format!("    git commit -am \"release: {version}\""));
    ui::detail(&format!(
        "    git tag -a v{version} -m \"noctua-design {version}\""
    ));
    ui::detail("    git push --follow-tags");
    Ok(())
}

/// Every file that names the version, and how many times.
fn plan(root: &Path, current: &str) -> Result<Vec<(String, usize)>, String> {
    let mut out = Vec::new();
    for path in [
        "Cargo.toml",
        "packages/react/package.json",
        "packages/tokens/package.json",
        "packages/dist/package.json",
        "docs-site/package.json",
        // The install instructions. A reader copies these, so a stale one is an
        // error from a registry — the worst place to learn a document rotted.
        "README.md",
        "crates/noctua-design/README.md",
        "crates/noctua-design-dist/README.md",
    ] {
        let text = std::fs::read_to_string(root.join(path))
            .map_err(|error| format!("could not read {path}: {error}"))?;
        let count = text.matches(current).count();
        if count == 0 {
            return Err(format!(
                "{path} does not name {current}, so this release would leave it behind"
            ));
        }
        out.push((path.to_owned(), count));
    }
    Ok(out)
}

/// Brings `Cargo.lock` to the version just written.
///
/// **The gate runs before the writes**, so it reads the old version and cannot
/// see this: `v0.2.5` was tagged with a lockfile still naming `0.2.4`, and CI's
/// determinism job — which builds and then asks whether anything changed — is
/// what caught it, one push later. `cargo metadata` resolves the workspace and
/// rewrites the lockfile without compiling anything, which is the cheapest
/// thing that makes the tree consistent with itself.
///
/// # Errors
///
/// If cargo cannot be run, or refuses to resolve the workspace.
fn relock(root: &Path) -> Result<(), String> {
    let out = std::process::Command::new("cargo")
        .args(["metadata", "--format-version", "1", "--offline"])
        .current_dir(root)
        .output()
        .map_err(|error| format!("could not run cargo metadata: {error}"))?;
    if out.status.success() {
        return Ok(());
    }
    Err(format!(
        "cargo metadata refused to resolve the workspace, so Cargo.lock still names the old \
         version:\n{}",
        String::from_utf8_lossy(&out.stderr).trim()
    ))
}

/// The version inside `[workspace.package]`, which is the one that matters.
///
/// Read from the table rather than from the first `version = ` in the file: the
/// dependency pins below it carry the same number, and a scan that stopped at
/// the first line would keep working right up until they disagreed.
fn workspace_version(text: &str) -> Result<String, String> {
    let table = text
        .split_once("[workspace.package]")
        .ok_or_else(|| "Cargo.toml has no [workspace.package] table".to_owned())?
        .1;
    for line in table.lines() {
        if let Some(rest) = line.trim().strip_prefix("version = \"") {
            if let Some(end) = rest.find('"') {
                return Ok(rest[..end].to_owned());
            }
        }
        if line.trim_start().starts_with('[') {
            break;
        }
    }
    Err("[workspace.package] declares no version".to_owned())
}

/// One file's text with the version moved on.
///
/// The match is always the whole quoted string, never the bare number: `0.1.0`
/// appears inside a dependency's own version requirement and inside prose, and
/// a blind replace would rewrite both — including a sentence about what
/// happened at a version, which is history and must not move.
fn rewrite(path: &str, text: &str, current: &str, next: &str) -> Result<String, String> {
    // By the manifest this is, not by an extension: the four paths are literals
    // a few lines up, and asking the filesystem's opinion about their spelling
    // would be a question nobody asked.
    let spellings: Vec<(String, String)> = if path.contains("package.json") {
        vec![(
            format!("\"version\": \"{current}\""),
            format!("\"version\": \"{next}\""),
        )]
    } else if path.contains("README") {
        // Prose names a version three ways and all three are instructions a
        // reader copies: a Cargo requirement, an npm range, a git tag. Each is
        // matched **quoted**, so a sentence recalling that `v0.1.0` was tagged
        // on a Tuesday keeps saying so. By the file this is, not by its
        // extension, for the same reason the branch above says so.
        vec![
            (format!("\"{current}\""), format!("\"{next}\"")),
            (format!("\"^{current}\""), format!("\"^{next}\"")),
            (format!("\"v{current}\""), format!("\"v{next}\"")),
        ]
    } else {
        vec![(
            format!("version = \"{current}\""),
            format!("version = \"{next}\""),
        )]
    };
    if !spellings.iter().any(|(from, _)| text.contains(from)) {
        let from = &spellings[0].0;
        return Err(format!(
            "{path} does not contain `{from}` — its formatting moved, and this rewrite would \
             have silently done nothing"
        ));
    }
    let mut out = text.to_owned();
    for (from, to) in &spellings {
        out = out.replace(from, to);
    }
    Ok(out)
}

#[cfg(test)]
mod tests {
    use super::*;

    /// An install instruction moves; a sentence about the past does not.
    ///
    /// Both spellings sit in the same document, and telling them apart is the
    /// only reason this function knows what a README is.
    #[test]
    fn prose_moves_its_instructions_and_keeps_its_history() {
        let text = "noctua-design = \"0.1.0\"\n\
                    \"@noctua-world/design-dist\": \"^0.1.0\"\n\
                    tag = \"v0.1.0\"\n\
                    The gate opened at v0.1.0, on a Tuesday.\n";
        let out = rewrite("README.md", text, "0.1.0", "0.2.0").unwrap();
        assert!(out.contains("noctua-design = \"0.2.0\""));
        assert!(out.contains("\"^0.2.0\""));
        assert!(out.contains("tag = \"v0.2.0\""));
        assert!(
            out.contains("The gate opened at v0.1.0, on a Tuesday."),
            "history moved: {out}"
        );
    }

    #[test]
    fn the_workspace_version_is_read_from_its_own_table() {
        let text = "[workspace]\nmembers = []\n\n[workspace.package]\nversion = \"0.4.2\"\n";
        assert_eq!(workspace_version(text).unwrap(), "0.4.2");
    }

    /// The pins under `[workspace.dependencies]` carry the same number, and
    /// reading the first `version =` in the file would find one of them the day
    /// they stop agreeing.
    #[test]
    fn a_dependency_pin_is_not_mistaken_for_the_workspace_version() {
        let text = "[workspace.dependencies]\nfoo = { version = \"9.9.9\" }\n\n\
                    [workspace.package]\nversion = \"0.1.0\"\n";
        assert_eq!(workspace_version(text).unwrap(), "0.1.0");
    }

    #[test]
    fn a_manifest_whose_formatting_moved_is_an_error_rather_than_a_no_op() {
        let error = rewrite(
            "packages/react/package.json",
            "{\"version\":\"0.1.0\"}",
            "0.1.0",
            "0.2.0",
        )
        .unwrap_err();
        assert!(error.contains("silently done nothing"), "{error}");
    }

    #[test]
    fn every_quoted_version_in_a_manifest_moves() {
        let text = "version = \"0.1.0\"\nnoctua-design = { path = \"x\", version = \"0.1.0\" }\n";
        let out = rewrite("Cargo.toml", text, "0.1.0", "0.2.0").unwrap();
        assert_eq!(out.matches("0.2.0").count(), 2);
        assert!(!out.contains("0.1.0"));
    }
}
