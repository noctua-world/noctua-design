//! `cargo xtask build` — compile `spec/` into every target.

use std::path::Path;

use noctua_design_emit::EmittedFile;
use noctua_design_spec::Spec;

use crate::ui;

/// Loads the spec and produces every artifact **in memory**, touching nothing.
///
/// Separate from [`run`] for one important reason: `check` must **not** rewrite
/// the generated files before inspecting them. A check that regenerates its own
/// input and then compares it against itself reports "in sync" immediately
/// after a hand edit — which is exactly the bug this separation exists to
/// prevent, and exactly the bug `noctua-colors` shipped for a milestone before
/// the same split was made there.
///
/// # Errors
///
/// Any problem in `spec/tokens.toml`.
pub(crate) fn artifacts(root: &Path, spec_path: &Path) -> Result<(Spec, Vec<EmittedFile>), String> {
    let relative = relative_to(root, spec_path);

    let spec = noctua_design_spec::load(spec_path).map_err(|error| {
        // The spec layer's diagnostics point at the exact byte range and end
        // with a fix; rendering them any other way would throw that away.
        format!("{:?}", miette::Report::new(error))
    })?;

    ui::ok(&format!("compiled {relative}"));
    ui::detail(&format!(
        "{} spacing steps, {} sizes, {} durations, {} layers, {} breakpoints",
        spec.space.steps.len(),
        spec.r#type.size.len(),
        spec.motion.duration.len(),
        spec.layer.len(),
        spec.breakpoint.len()
    ));

    let fonts_path = root.join("spec/fonts.toml");
    let fonts = noctua_design_spec::load_fonts(&fonts_path)
        .map_err(|error| format!("{:?}", miette::Report::new(error)))?;
    ui::ok(&format!("compiled {}", relative_to(root, &fonts_path)));
    ui::detail(&format!(
        "{} faces, {} preloaded, {} stacks",
        fonts.face.len(),
        fonts.face.iter().filter(|f| f.preload).count(),
        fonts.stack.len()
    ));

    let mut files = noctua_design_emit::emit_all(&spec, &fonts);

    // Generated Rust goes through rustfmt before anyone compares it.
    //
    // The generated token crate is a workspace member, so `cargo fmt --all`
    // formats it — which means the emitter's idea of the file and rustfmt's
    // idea of it must agree, or `check` reports the file both out of sync
    // *and* badly formatted, forever, with no edit that fixes both.
    //
    // Formatting here rather than teaching the emitter rustfmt's preferences
    // is the only version of this that stays true: rustfmt is the authority on
    // its own output, and guessing at it would be a maintenance debt with no
    // end. It happens in `artifacts` rather than in `run` so that `check`
    // compares formatted text against formatted text.
    for file in &mut files {
        if std::path::Path::new(&file.path)
            .extension()
            .is_some_and(|e| e.eq_ignore_ascii_case("rs"))
        {
            file.contents = rustfmt(&file.contents)?;
        }
    }

    Ok((spec, files))
}

/// Runs `rustfmt` over a string, returning the formatted text.
///
/// # Errors
///
/// If rustfmt cannot be run, or rejects the input — which would mean the
/// emitter produced source that does not parse, and is worth failing loudly.
fn rustfmt(source: &str) -> Result<String, String> {
    use std::io::Write as _;
    use std::process::{Command, Stdio};

    let mut child = Command::new("rustfmt")
        .args(["--edition", "2024", "--emit", "stdout", "--quiet"])
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .map_err(|error| {
            format!(
                "could not run rustfmt: {error}. It is declared in rust-toolchain.toml, \
                 so `rustup show` should install it"
            )
        })?;

    child
        .stdin
        .as_mut()
        .expect("stdin was piped")
        .write_all(source.as_bytes())
        .map_err(|error| format!("could not write to rustfmt: {error}"))?;

    let output = child
        .wait_with_output()
        .map_err(|error| format!("rustfmt did not finish: {error}"))?;

    if !output.status.success() {
        return Err(format!(
            "rustfmt rejected the generated source, which means it does not parse:\n{}",
            String::from_utf8_lossy(&output.stderr)
        ));
    }

    String::from_utf8(output.stdout)
        .map_err(|error| format!("rustfmt produced invalid UTF-8: {error}"))
}

/// Builds and writes every generated file.
///
/// # Errors
///
/// Any spec problem or filesystem failure.
pub(crate) fn run(root: &Path, spec_path: &Path) -> Result<Spec, String> {
    ui::heading("specification");
    let (spec, files) = artifacts(root, spec_path)?;

    ui::gap();
    ui::heading("generated output");

    let mut written = 0usize;
    for file in &files {
        let path = root.join(&file.path);
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent)
                .map_err(|error| format!("could not create {}: {error}", parent.display()))?;
        }

        // Only write when the contents actually differ. Rewriting an identical
        // file changes its mtime, which makes every file-watcher downstream
        // rebuild for nothing — and makes `dev` loop.
        let unchanged = std::fs::read_to_string(&path).is_ok_and(|old| old == file.contents);
        if !unchanged {
            std::fs::write(&path, &file.contents)
                .map_err(|error| format!("could not write {}: {error}", file.path))?;
            written += 1;
        }
    }

    ui::ok(&format!(
        "{} generated file(s), {written} rewritten",
        files.len()
    ));
    for file in &files {
        ui::detail(&file.path);
    }

    Ok(spec)
}

/// The spec path as written in generated headers.
///
/// Always repository-relative with forward slashes, so the generated output is
/// identical no matter where the build ran from or on which platform.
pub(crate) fn relative_to(root: &Path, path: &Path) -> String {
    path.strip_prefix(root)
        .unwrap_or(path)
        .to_string_lossy()
        .replace('\\', "/")
}
