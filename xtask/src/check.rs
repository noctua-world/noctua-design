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

use crate::{build, gates, references, ui};

/// Runs every gate.
///
/// # Errors
///
/// A summary of what failed. Individual failures are printed as they are found,
/// so one run reports everything rather than stopping at the first.
pub(crate) fn run(root: &Path, spec_path: &Path, spec_only: bool) -> Result<(), String> {
    let mut failures: Vec<String> = Vec::new();

    ui::heading("specification");
    let (spec, vendor, files) = build::artifacts(root, spec_path)?;

    ui::gap();
    ui::heading("source gates");
    let findings = gates::run(root, &spec, &vendor)?;
    if findings.is_empty() {
        ui::ok("no color, token, font or arbitrary-value literal in source");
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
    if let Some(failure) = dangling_references(root)? {
        failures.push(failure);
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

    // The consumer outside the workspace, run rather than built: what it
    // asserts — that every root class it renders is painted by
    // `dist/components.css` — only happens when `main` does. `cargo test` in
    // this workspace can never reach it, because it is not a member.
    if let Err(error) = outside_consumer(root) {
        ui::failure(&error);
        failures.push("the external consumer".to_owned());
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

    payload(root, &mut failures);

    finish(&failures)
}

/// Every path and every name a document cites, checked against what is here.
///
/// The other gates read code; this one reads what the code is described BY, and
/// `references` says why that half needed a mechanism of its own.
///
/// # Errors
///
/// Only a filesystem failure. A dangling reference is reported and summarised,
/// like every other finding here.
fn dangling_references(root: &Path) -> Result<Option<String>, String> {
    ui::heading("references");
    let dangling = references::run(root)?;
    if dangling.is_empty() {
        ui::ok("every path and name a document cites exists");
        return Ok(None);
    }
    for finding in &dangling {
        ui::failure(&format!(
            "{}:{} — {}",
            finding.file, finding.line, finding.message
        ));
    }
    ui::detail("a renamed file leaves these behind; the rename is not done until they are");
    Ok(Some(format!("{} dangling reference(s)", dangling.len())))
}

/// The promise the icon set is arranged around, measured rather than argued.
///
/// `spec/icons.toml` ships every Lucide drawing, and what makes that acceptable
/// is that an application drawing one of them pays for one of them. That is a
/// claim about a linked binary and about a bundler's output; the bundler half
/// needs Node and lives in `tests/browser/bundle.spec.js`, and this is the half
/// the Rust build can make on its own. `NOCTUA.md §9.5`.
fn payload(root: &Path, failures: &mut Vec<String>) {
    ui::gap();
    ui::heading("payload");

    // THE NEEDLES AND THE EXPECTED MODULES COME OUT OF THE SPEC. Written into
    // these gates they would be copies of upstream geometry and of a file
    // listing, and the day a tag bump changed either the gate would stop
    // finding anything and go on reporting success. Upstream redrew `menu` from
    // three `<line>`s to three `<path>`s between the two tags this repository
    // has vendored, which is exactly that day.
    let drawings = match noctua_design_spec::icons::load(
        root.join("spec/icons.toml"),
        root.join("assets/icons"),
    ) {
        Ok((_, drawings)) => drawings,
        Err(error) => {
            ui::failure(&format!("{error:?}"));
            failures.push("the icon set does not load".to_owned());
            return;
        }
    };

    for (label, outcome) in [
        (
            "no aggregate of drawings is importable",
            no_icon_aggregate(root, &drawings),
        ),
        (
            "one icon in a release binary is one icon",
            one_icon_is_one_icon(root, &drawings),
        ),
    ] {
        match outcome {
            Ok(detail) => {
                ui::ok(label);
                ui::detail(&detail);
            }
            Err(problem) => {
                ui::failure(&problem);
                failures.push(label.to_owned());
            }
        }
    }
}

/// The icon a binary names is in it, and the ones it does not name are not.
///
/// # Why a binary and not a unit test
///
/// A unit test proves what a `const` evaluates to. It cannot prove what reaches
/// the linker, and that is the entire question: the previous shape put every
/// drawing in one `match` inside one reachable function, which no test would
/// have noticed and every consumer would have paid for.
///
/// So this builds the smallest real thing — an example that draws exactly one
/// icon — in release, and reads the bytes. The positive half matters as much as
/// the negative one: a needle nobody can find in a binary that genuinely
/// contains it means the search is broken, not that the payload is small.
fn one_icon_is_one_icon(
    root: &Path,
    drawings: &[noctua_design_spec::Icon],
) -> Result<String, String> {
    /// The icon `examples/one_icon.rs` draws. Change one, change the other.
    const DRAWN: &str = "menu";
    /// Three it does not, from three corners of the set.
    const ABSENT: [&str; 3] = ["dna", "vault", "flask-conical"];

    let body = |name: &str| {
        drawings
            .iter()
            .find(|icon| icon.name == name)
            .map(|icon| icon.body.clone())
            .ok_or_else(|| format!("this gate names `{name}`, which is not an icon"))
    };
    let drawn = body(DRAWN)?;

    cargo(
        root,
        &[
            "build",
            "--release",
            "--quiet",
            "-p",
            "noctua-design-icons",
            "--example",
            "one_icon",
        ],
    )?;

    let binary = root.join("target/release/examples/one_icon");
    let bytes = std::fs::read(&binary)
        .map_err(|error| format!("could not read {}: {error}", binary.display()))?;
    let holds = |needle: &str| {
        bytes
            .windows(needle.len())
            .any(|window| window == needle.as_bytes())
    };

    if !holds(&drawn) {
        return Err(format!(
            "{} does not contain `{DRAWN}`, which is the icon it draws. The search \
             is wrong, so the absences prove nothing",
            binary.display()
        ));
    }
    for name in ABSENT {
        if holds(&body(name)?) {
            return Err(format!(
                "{} draws one icon and contains `{name}` as well. Something reached \
                 the whole set — `ALL`, `parse`, or a table that came back",
                binary.display()
            ));
        }
    }
    Ok(format!(
        "{} KB of binary for {} icon(s) in the system: `{DRAWN}` present, {} absent",
        bytes.len() / 1024,
        drawings.len(),
        ABSENT.join(", ")
    ))
}

/// Nothing a consumer can import names more than one drawing.
///
/// The measurement above is the proof; this is what keeps the shape that makes
/// it true. A single module re-exporting every icon would pass every other gate
/// in this file and quietly undo the whole arrangement the first time somebody
/// imported it for convenience.
fn no_icon_aggregate(root: &Path, drawings: &[noctua_design_spec::Icon]) -> Result<String, String> {
    let shared = root.join("packages/tokens/icons.js");
    let text = std::fs::read_to_string(&shared)
        .map_err(|error| format!("could not read {}: {error}", shared.display()))?;
    // WHAT IT EXPORTS, not what it mentions: the file explains at length why
    // the `ICONS` table is gone, and a gate that read the explanation as the
    // offence would be a gate nobody could satisfy without deleting the reason.
    let exported: Vec<&str> = text
        .lines()
        .map(str::trim)
        .filter(|line| line.starts_with("export "))
        .collect();
    let only_the_weight = exported.len() == 1
        && exported[0].starts_with("export const ICON_STROKE_WIDTH = \"")
        && exported[0].ends_with("\";");
    if !only_the_weight {
        return Err(format!(
            "{} exports {exported:?}. It holds the stroke weight and nothing else; \
             a table here is a table every consumer of one icon imports",
            shared.display()
        ));
    }

    let directory = root.join("packages/tokens/icons");
    let entries = std::fs::read_dir(&directory)
        .map_err(|error| format!("could not read {}: {error}", directory.display()))?;
    let declared: std::collections::BTreeSet<&str> =
        drawings.iter().map(|icon| icon.name.as_str()).collect();
    let mut found: std::collections::BTreeSet<String> = std::collections::BTreeSet::new();

    for entry in entries.flatten() {
        let path = entry.path();
        if path.extension().is_none_or(|kind| kind != "js") {
            continue;
        }
        let Some(name) = path
            .file_stem()
            .map(|stem| stem.to_string_lossy().into_owned())
        else {
            continue;
        };
        found.insert(name);
        let text = std::fs::read_to_string(&path)
            .map_err(|error| format!("could not read {}: {error}", path.display()))?;
        if text.contains("icons/") {
            return Err(format!(
                "{} imports another icon. One module, one drawing: a module that \
                 reaches a second one is a bundle that carries a second one",
                path.display()
            ));
        }
        if text.matches("export const").count() != 1 {
            return Err(format!(
                "{} does not export exactly one drawing",
                path.display()
            ));
        }
    }

    // AND NOTHING STALE. `cargo xtask build` writes changed files and never
    // removes one nothing produces any more, so an icon deleted from the set
    // leaves its module behind — importable, unmentioned by the drift check,
    // and drawing something this system no longer has. The same hazard
    // `AGENTS.md` records for a deleted component, in a directory with 1,777
    // files where nobody would notice one.
    let stale: Vec<&String> = found
        .iter()
        .filter(|name| !declared.contains(name.as_str()))
        .collect();
    if !stale.is_empty() {
        return Err(format!(
            "{} holds {} module(s) no icon declares — {stale:?}. `build` never \
             removes a file, so these have to go by hand",
            directory.display(),
            stale.len()
        ));
    }
    if found.len() != declared.len() {
        return Err(format!(
            "{} holds {} module(s) for {} icon(s)",
            directory.display(),
            found.len(),
            declared.len()
        ));
    }
    if found.is_empty() {
        return Err(format!(
            "{} holds no modules, so this gate covered nothing",
            directory.display()
        ));
    }
    Ok(format!(
        "{} module(s), one drawing each, and none the spec does not declare",
        found.len()
    ))
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
        "crates/noctua-design-grammars/Cargo.toml",
        "crates/noctua-design-lineage/Cargo.toml",
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

/// Runs the example consumer, and reports what it said.
fn outside_consumer(root: &Path) -> Result<(), String> {
    let example = root.join("examples/consumer-rust");
    if !example.is_dir() {
        return Ok(());
    }
    ui::heading("the consumer outside this workspace");
    let line = consumer(&example, &root.join("target/examples"))?;
    ui::ok(&line);
    ui::gap();
    Ok(())
}

/// Builds and runs the example consumer, and returns what it reported.
///
/// Its own target directory: the example is a separate workspace, and letting
/// it write into this one's `target/` makes two builds fight over one lock for
/// no benefit.
fn consumer(dir: &std::path::Path, target: &std::path::Path) -> Result<String, String> {
    let output = std::process::Command::new("cargo")
        .args(["run", "--quiet"])
        .current_dir(dir)
        .env("CARGO_TARGET_DIR", target)
        .output()
        .map_err(|error| format!("could not run the example consumer: {error}"))?;
    if !output.status.success() {
        return Err(format!(
            "the example consumer failed:\n{}",
            String::from_utf8_lossy(&output.stderr)
        ));
    }
    let text = String::from_utf8_lossy(&output.stdout);
    Ok(text
        .lines()
        .rev()
        .find(|line| !line.trim().is_empty())
        .unwrap_or("it ran")
        .to_owned())
}
