//! `cargo xtask check` — the single gate.
//!
//! CI runs exactly this and nothing else. Not a reimplementation of the step
//! list in YAML, which is precisely how the two drift until "passes locally,
//! fails in CI" becomes a normal sentence.
//!
//! # Two ways a gate can be quietly useless
//!
//! **A check that writes before it reads.** This one must never regenerate the
//! output before comparing against it: it would compare the output to itself
//! and report "in sync" immediately after a hand edit. [`crate::build`] keeps
//! the in-memory production separate from the write for this reason alone.
//!
//! **A check nothing calls.** Every gate below is invoked here, and the drift
//! check reports how many files it compared — a count of zero is a gate that
//! has silently stopped covering anything.

use std::path::Path;
use std::process::Command;

use crate::{build, gates, ui};

/// Runs every gate.
///
/// # Errors
///
/// A summary of what failed. Individual failures are printed as they are found,
/// so one run reports everything rather than stopping at the first.
pub(crate) fn run(root: &Path, spec_path: &Path, spec_only: bool) -> Result<(), String> {
    let mut failures: Vec<String> = Vec::new();

    ui::heading("specification");
    let (_spec, files) = build::artifacts(root, spec_path)?;

    ui::gap();
    ui::heading("source gates");
    let findings = gates::run(root)?;
    if findings.is_empty() {
        ui::ok("no colour, token, font or arbitrary-value literal in source");
    } else {
        for finding in &findings {
            ui::failure(&format!(
                "{}:{} — {}",
                finding.file, finding.line, finding.message
            ));
        }
        ui::detail(
            "if one of these is genuinely necessary, add `allow-literal: <reason>` \
             on the same line",
        );
        failures.push(format!("{} source-gate finding(s)", findings.len()));
    }

    ui::gap();
    ui::heading("generated output");
    let mut drifted = Vec::new();
    for file in &files {
        let path = root.join(&file.path);
        match std::fs::read_to_string(&path) {
            Ok(on_disk) if on_disk == file.contents => {}
            Ok(_) => drifted.push(format!("{} does not match the spec", file.path)),
            Err(_) => drifted.push(format!("{} is missing", file.path)),
        }
    }
    if drifted.is_empty() {
        ui::ok(&format!(
            "{} generated file(s) match the specification",
            files.len()
        ));
    } else {
        for problem in &drifted {
            ui::failure(problem);
        }
        ui::detail(
            "if you meant this, run `cargo xtask build`; if you did not, a generated \
             file was edited by hand",
        );
        failures.push(format!("{} generated file(s) out of sync", drifted.len()));
    }

    if spec_only {
        return finish(&failures);
    }

    ui::gap();
    ui::heading("workspace");

    // Formatting, lints and tests. Each is reported separately so a failing
    // run says which one, and all three run even if an earlier one failed —
    // fixing one problem only to be shown the next is a miserable loop.
    for (label, args) in [
        ("formatting", vec!["fmt", "--all", "--check"]),
        (
            "lints",
            vec![
                "clippy",
                "--workspace",
                "--all-targets",
                "--",
                "-D",
                "warnings",
            ],
        ),
        ("tests", vec!["test", "--workspace", "--quiet"]),
    ] {
        match cargo(root, &args) {
            Ok(()) => ui::ok(label),
            Err(error) => {
                ui::failure(&format!("{label}:"));
                anstream::eprintln!("{error}");
                failures.push(label.to_owned());
            }
        }
    }

    // The claim that this repository's Rust half needs no Node is an
    // acceptance criterion, so it is verified rather than asserted. Running
    // with an emptied PATH entry for node would be fragile; instead the check
    // is structural: nothing in the Rust build may reference a Node artifact.
    ui::gap();
    ui::heading("independence");
    match rust_needs_no_node(root) {
        Ok(()) => ui::ok("the Rust half references no Node artifact"),
        Err(problem) => {
            ui::failure(&problem);
            failures.push("the Rust half depends on Node".to_owned());
        }
    }

    finish(&failures)
}

fn finish(failures: &[String]) -> Result<(), String> {
    ui::gap();
    if failures.is_empty() {
        ui::heading("check passed");
        Ok(())
    } else {
        Err(format!("check failed: {}", failures.join(", ")))
    }
}

/// The Rust half must build and test with no Node installed.
///
/// `NOCTUA.md §9.11` makes this a promise to Rust-only contributors, and a
/// promise nothing checks is a promise that lapses the first time someone adds
/// a build script that shells out to `npx`.
///
/// Checked structurally rather than by manipulating `PATH`: a subprocess with a
/// doctored environment would test the machine rather than the repository, and
/// would pass on a machine that simply has no Node.
fn rust_needs_no_node(root: &Path) -> Result<(), String> {
    for manifest in [
        "Cargo.toml",
        "xtask/Cargo.toml",
        "crates/noctua-design/Cargo.toml",
        "crates/noctua-design-spec/Cargo.toml",
        "crates/noctua-design-emit/Cargo.toml",
        "crates/noctua-design-tokens/Cargo.toml",
        "crates/noctua-design-icons/Cargo.toml",
    ] {
        let path = root.join(manifest);
        let Ok(text) = std::fs::read_to_string(&path) else {
            continue;
        };
        for offender in ["npm", "npx", "pnpm", "node_modules"] {
            if text.contains(offender) {
                return Err(format!(
                    "{manifest} mentions `{offender}`. The Rust half must build and test \
                     with no Node installed (NOCTUA.md §9.11)"
                ));
            }
        }
    }

    // A build script is the usual way this promise gets broken.
    for script in ["crates/noctua-design/build.rs", "xtask/build.rs"] {
        if root.join(script).exists() {
            return Err(format!(
                "{script} exists. A build script is how a Node dependency usually \
                 arrives; if this one is genuinely necessary, this check needs updating \
                 deliberately"
            ));
        }
    }
    Ok(())
}

/// Runs a cargo subcommand, returning its combined output on failure.
fn cargo(root: &Path, args: &[&str]) -> Result<(), String> {
    let output = Command::new(std::env::var("CARGO").unwrap_or_else(|_| "cargo".to_owned()))
        .args(args)
        .current_dir(root)
        .output()
        .map_err(|error| format!("could not run cargo {}: {error}", args.join(" ")))?;

    if output.status.success() {
        Ok(())
    } else {
        let mut combined = String::from_utf8_lossy(&output.stdout).into_owned();
        combined.push_str(&String::from_utf8_lossy(&output.stderr));
        Err(combined)
    }
}
