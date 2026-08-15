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
    Docs {
        /// Also run the browser suite against the built site.
        ///
        /// Behind `docs` rather than a sixth verb, and opt-in rather than part
        /// of `check`, because it is the one thing here that needs a browser —
        /// and `NOCTUA.md §9.11` promises the Rust half builds and tests with
        /// no Node installed. CI runs `cargo xtask docs --verify`.
        #[arg(long)]
        verify: bool,
    },

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
        Command::Docs { verify } => docs(&root, verify),
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
    docs(root, false)?;

    ui::gap();
    ui::heading("serving");
    let dist = root.join("docs-site/dist");
    ui::ok(&format!("http://127.0.0.1:{port}/"));
    ui::detail("rebuilds when spec/, docs-site/src/ or docs-site/assets/ changes");
    ui::detail("stops rebuilding if an emitter changes — restart it then");
    ui::detail("Ctrl-C to stop");

    let watch_root = root.to_path_buf();
    let watch_spec = spec.to_path_buf();
    std::thread::spawn(move || watch(&watch_root, &watch_spec));

    serve(&dist, port)
}

/// Rebuilds whenever a source file's modification time moves.
///
/// Polling rather than an inotify crate, and that is a real tradeoff taken
/// deliberately: the watched set is a few dozen files, a poll costs one `stat`
/// each, and the alternative is a dependency with a platform backend for each
/// operating system — inherited by everyone who builds this repository, to save
/// a quarter-second of latency on a developer's own machine.
fn watch(root: &Path, spec: &Path) {
    let watched = [
        root.join("spec"),
        root.join("docs-site/src"),
        root.join("docs-site/assets"),
    ];
    // THIS PROCESS IS THE CODE IT STARTED WITH, and that is the trap.
    //
    // A long-running `target/debug/xtask` keeps the emitters it was compiled
    // with. Edit one, touch `spec/`, and this thread cheerfully regenerates
    // every committed artefact from the OLD compiler — silently reverting a fix
    // that was already made, already tested and already committed, and leaving
    // `cargo xtask check` reporting files out of sync with nothing to explain
    // it. That happened three times while this repository was being rebuilt.
    //
    // So the emitters are watched too, and a change to one stops the rebuilding
    // rather than doing it wrong. Refusing loudly is the whole feature.
    let emitters = [root.join("crates"), root.join("xtask/src")];
    let started_with = fingerprint(&emitters);
    let mut last = fingerprint(&watched);

    loop {
        std::thread::sleep(std::time::Duration::from_millis(400));

        if fingerprint(&emitters) != started_with {
            ui::gap();
            ui::heading("stale");
            ui::warn(
                "an emitter changed, and this process is still running the old one. \
                 Regenerating now would overwrite committed output with what the \
                 PREVIOUS build would have produced.",
            );
            ui::detail("nothing was rebuilt; stop this and run `cargo xtask dev` again");
            return;
        }

        let current = fingerprint(&watched);
        if current == last {
            continue;
        }
        last = current;

        ui::gap();
        ui::heading("rebuilding");
        // A failed rebuild must not kill the server: a spec with a typo in it is
        // the normal state of a file being edited, and losing the site every
        // time would mean restarting after every mistake.
        match build::run(root, spec).and_then(|_| docs(root, false)) {
            Ok(()) => ui::ok("reload the page"),
            Err(error) => ui::warn(&error),
        }
    }
}

/// The modification times of everything under the watched directories.
fn fingerprint(
    directories: &[std::path::PathBuf],
) -> Vec<(std::path::PathBuf, std::time::SystemTime)> {
    let mut out = Vec::new();
    for directory in directories {
        collect(directory, &mut out);
    }
    out.sort();
    out
}

fn collect(path: &Path, into: &mut Vec<(std::path::PathBuf, std::time::SystemTime)>) {
    let Ok(entries) = std::fs::read_dir(path) else {
        return;
    };
    for entry in entries.flatten() {
        let path = entry.path();
        if path.is_dir() {
            collect(&path, into);
        } else if let Ok(modified) = entry.metadata().and_then(|m| m.modified()) {
            into.push((path, modified));
        }
    }
}

/// A static file server, in the standard library and nothing else.
///
/// The site is static files by design, so serving them needs no framework — and
/// taking an HTTP dependency for `cargo xtask dev` would put it in the lockfile
/// of every consumer of this repository, to run a command none of them run.
fn serve(dist: &Path, port: u16) -> Result<(), String> {
    use std::io::{Read as _, Write as _};

    let listener = std::net::TcpListener::bind(("127.0.0.1", port))
        .map_err(|error| format!("could not listen on :{port}: {error}"))?;

    for stream in listener.incoming() {
        let Ok(mut stream) = stream else { continue };
        let mut buffer = [0_u8; 2048];
        let Ok(read) = stream.read(&mut buffer) else {
            continue;
        };

        let request = String::from_utf8_lossy(&buffer[..read]);
        let target = request
            .split_whitespace()
            .nth(1)
            .unwrap_or("/")
            .split('?')
            .next()
            .unwrap_or("/");

        // A directory asked for without its trailing slash has to REDIRECT, not
        // simply serve its index: every href on the page is document-relative,
        // so at `/en/colors` they all resolve one level too high and the page
        // arrives with no stylesheet and no working links. GitHub Pages and
        // `python3 -m http.server` both send this 301; a dev server that
        // quietly serves the file instead is a dev server that disagrees with
        // what ships, in the direction that hides the bug.
        if is_directory(dist, target) && !target.ends_with('/') {
            let header = format!(
                "HTTP/1.1 301 Moved Permanently\r\n\
                 Location: {target}/\r\n\
                 Content-Length: 0\r\n\
                 Connection: close\r\n\r\n"
            );
            let _ = stream.write_all(header.as_bytes());
            continue;
        }

        let (status, body, mime) = match resolve(dist, target) {
            Some((body, mime)) => ("200 OK", body, mime),
            None => (
                "404 Not Found",
                b"not found".to_vec(),
                "text/plain; charset=utf-8",
            ),
        };

        let header = format!(
            "HTTP/1.1 {status}\r\n\
             Content-Type: {mime}\r\n\
             Content-Length: {}\r\n\
             Cache-Control: no-store\r\n\
             Connection: close\r\n\r\n",
            body.len()
        );
        let _ = stream.write_all(header.as_bytes());
        let _ = stream.write_all(&body);
    }
    Ok(())
}

/// Whether a request names a directory inside the site.
///
/// Same traversal refusal as [`resolve`]: a path that escapes `dist` is not a
/// directory as far as this server is concerned, whatever the filesystem says.
fn is_directory(dist: &Path, target: &str) -> bool {
    if target.split('/').any(|segment| segment == "..") {
        return false;
    }
    let path = dist.join(target.trim_start_matches('/'));
    path.starts_with(dist) && path.is_dir()
}

/// Maps a request path to a file, refusing anything that escapes the site.
fn resolve(dist: &Path, target: &str) -> Option<(Vec<u8>, &'static str)> {
    // `..` in a request path is how a static server hands out /etc/passwd. The
    // whole segment is rejected rather than normalised, because normalising is
    // where the subtle bypasses live.
    if target.split('/').any(|segment| segment == "..") {
        return None;
    }

    let relative = target.trim_start_matches('/');
    let mut path = dist.join(relative);
    if path.is_dir() || relative.is_empty() {
        path = path.join("index.html");
    }
    if !path.starts_with(dist) {
        return None;
    }

    let mime = match path.extension().and_then(|e| e.to_str()) {
        Some("html") => "text/html; charset=utf-8",
        Some("css") => "text/css; charset=utf-8",
        Some("js") => "text/javascript; charset=utf-8",
        Some("json") => "application/json",
        Some("woff2") => "font/woff2",
        Some("svg") => "image/svg+xml",
        _ => "application/octet-stream",
    };
    std::fs::read(&path).ok().map(|body| (body, mime))
}

/// `docs` — pre-renders the documentation site, one build per locale.
///
/// The verb was reserved with no behaviour so that the stage which built the
/// site would add behaviour rather than a sixth verb (`NOCTUA.md §2.2`). This is
/// that stage, and the surface is still five.
///
/// It shells out to the `docs-site` binary rather than linking it, and that is
/// deliberate: `xtask` would otherwise take a dependency on Dioxus, and a
/// `cargo xtask check` on a machine that only wanted to validate the spec would
/// compile a UI framework to do it.
fn docs(root: &Path, verify: bool) -> Result<(), String> {
    ui::heading("documentation site");

    let output = std::process::Command::new(env!("CARGO"))
        .args(["run", "--quiet", "--package", "docs-site"])
        .current_dir(root)
        .output()
        .map_err(|error| format!("could not run the site build: {error}"))?;

    if !output.status.success() {
        return Err(format!(
            "the site build failed:\n{}",
            String::from_utf8_lossy(&output.stderr)
        ));
    }

    for line in String::from_utf8_lossy(&output.stdout).lines() {
        if let Some(detail) = line.strip_prefix("  ") {
            ui::detail(detail);
        } else {
            ui::ok(line);
        }
    }
    // Not "open the file": every URL is a directory now, and `file://` does not
    // serve a directory's index. `cargo xtask dev` is the way to look at it,
    // and it is the same static directory any host would serve.
    ui::detail("`cargo xtask dev` to look at it, or serve docs-site/dist");

    if verify {
        ui::gap();
        ui::heading("browser suite");
        let status = std::process::Command::new("pnpm")
            .args(["run", "test:browser"])
            .current_dir(root)
            .status()
            .map_err(|error| {
                format!(
                    "could not run the browser suite: {error}. It needs `pnpm install` \
                     and a Chrome; see docs-site/PROVISIONAL.md"
                )
            })?;
        if !status.success() {
            return Err("the browser suite failed".to_owned());
        }
        ui::ok("the browser suite passed");
    }

    Ok(())
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
