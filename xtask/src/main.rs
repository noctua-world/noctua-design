//! `cargo xtask` — the repository's whole command surface.
//!
//! **Five verbs, and it does not grow.** New capability goes behind an existing
//! verb or it does not ship. That is a real constraint and it is the reason
//! this file is the right place to look for "how do I…": there are five
//! answers, not thirty.
//!
//! Several verbs are stubs at this stage. A stub here is not a placeholder for
//! whoever notices — it is the verb surface being fixed *now*, while it is
//! cheap, so that a later stage adds behaviour behind an existing name rather
//! than adding a sixth verb. `NOCTUA-PLAN.md` Stage 3, scope 4.

mod build;
mod check;
mod gates;
mod ui;

use std::path::{Path, PathBuf};
use std::process::ExitCode;

use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(
    name = "xtask",
    about = "Build and check the Noctua design system",
    long_about = "Five verbs, and the surface does not grow: new capability \
                  goes behind an existing verb or it does not ship."
)]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    /// Compile spec/ into tokens, icons, props types and dist/.
    Build,

    /// The single gate: spec, source gates, drift, fmt, clippy, tests.
    ///
    /// This is exactly what CI runs — the same command, not a reimplementation
    /// in YAML, which is precisely how the two drift until "passes locally,
    /// fails in CI" becomes a normal sentence.
    Check {
        /// Skip formatting, lints and tests, for a fast loop while editing the
        /// spec. CI never passes this.
        #[arg(long)]
        spec_only: bool,
    },

    /// Watch spec/ and the components, rebuild, and serve the docs site.
    Dev {
        /// The port to serve on.
        #[arg(long, default_value_t = 8080)]
        port: u16,
    },

    /// Build the documentation site for both locales.
    Docs,

    /// Prepare a version. Writes it; committing is left to a human.
    Release {
        /// The version to release, such as `0.2.0`.
        version: String,

        /// Report what would happen without changing anything.
        #[arg(long)]
        dry_run: bool,
    },
}

fn main() -> ExitCode {
    let cli = Cli::parse();
    let root = repository_root();
    let spec = root.join("spec/tokens.toml");

    let outcome = match cli.command {
        Command::Build => build::run(&root, &spec).map(|_| ()),
        Command::Check { spec_only } => check::run(&root, &spec, spec_only),
        Command::Dev { port } => dev(&root, &spec, port),
        Command::Docs => {
            docs();
            Ok(())
        }
        Command::Release { version, dry_run } => release(&version, dry_run),
    };

    match outcome {
        Ok(()) => ExitCode::SUCCESS,
        Err(error) => {
            ui::failure(&error);
            ExitCode::FAILURE
        }
    }
}

/// `dev` — Stage 5 owns the docs site, so there is nothing to serve yet.
///
/// It builds and says so rather than starting a server on an empty directory:
/// a server serving nothing looks like a broken build, and a developer would
/// spend the next ten minutes finding out otherwise.
fn dev(root: &Path, spec: &Path, port: u16) -> Result<(), String> {
    build::run(root, spec)?;
    ui::gap();
    ui::warn("there is no documentation site yet — NOCTUA-PLAN.md Stage 5 owns it");
    ui::detail(&format!(
        "when it exists this watches spec/ and serves it on :{port} with live reload"
    ));
    Ok(())
}

/// `docs` — likewise Stage 5.
///
/// Returns nothing rather than `Result`: there is nothing here that can fail
/// yet, and a `Result` that is always `Ok` teaches a reader to stop checking.
fn docs() {
    ui::warn("there is no documentation site yet — NOCTUA-PLAN.md Stage 5 owns it");
    ui::detail("the verb exists now so that stage adds behaviour rather than a sixth verb");
}

/// `release` — deliberately refuses until there is something to release.
///
/// Not a stub that pretends to work. Writing a version onto a repository with
/// no published artifact produces a tag that names nothing, and the first real
/// release needs decisions — registry names, the publishing path — that belong
/// to the stage that makes them (`NOCTUA.md §5.3`).
///
/// The version is still validated, so the shape of the argument is settled now
/// and a later stage inherits it.
fn release(version: &str, _dry_run: bool) -> Result<(), String> {
    let parts: Vec<&str> = version.split('.').collect();
    if parts.len() != 3 || parts.iter().any(|p| p.parse::<u32>().is_err()) {
        return Err(format!(
            "`{version}` is not a version. Expected three numbers, such as 0.2.0"
        ));
    }
    Err(
        "nothing is published from this repository yet, so a version would name nothing. \
         The packages and their registries are decided in a later stage (NOCTUA.md §5.3); \
         this verb exists now so that stage adds behaviour rather than a sixth verb."
            .to_owned(),
    )
}

/// The repository root, found from this crate's location.
///
/// Not the current directory: `cargo xtask` runs from wherever the developer
/// happens to be, and every path this tool touches is repository-relative.
fn repository_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .expect("xtask has a parent")
        .to_path_buf()
}
