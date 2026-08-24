//! `cargo xtask build` — compile `spec/` into every target.

use std::collections::{BTreeMap, BTreeSet};
use std::path::Path;

use noctua_design_emit::EmittedFile;
use noctua_design_spec::Spec;

use crate::ui;

/// A specification's problems, rendered so a person can act on them.
///
/// `format!("{:?}", miette::Report::new(error))` was here, and on a machine
/// without miette's `fancy` feature — which nothing in this workspace enables,
/// because it is a rendering dependency every consumer would inherit — it
/// prints the struct and a note about installing the feature. The message and
/// the fix, which the spec layer went to some trouble to produce, never reach
/// anyone. This prints them.
///
/// The byte span is deliberately not turned back into a line and column: every
/// problem the spec layer raises already names the thing that is wrong, and a
/// second locator would be a second thing to keep correct.
fn report(path: &str, error: &noctua_design_spec::SpecError) -> String {
    use std::fmt::Write as _;

    let mut out = format!("{path}: {} problem(s)", error.problems().len());
    for problem in error.problems() {
        let _ = write!(out, "\n  - {}", problem.message());
        if !problem.help().is_empty() {
            let _ = write!(out, "\n    fix: {}", problem.help());
        }
    }
    out
}

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
/// Every component's cases, by the component's name.
type CaseSets = Vec<(String, noctua_design_spec::conformance::Cases)>;

/// The components, and the message catalogue that half depends on them.
///
/// Together because the order is load-bearing: a component's contract carries
/// prose a reader sees — its guidance, every prop's documentation, the keyboard
/// contract, the accessibility notes — and the site rendered exactly those
/// strings on every page in every locale. Deriving them into the catalogue is
/// what makes the cross-locale check demand a Portuguese answer for each one,
/// and it cannot happen before the components are read.
fn words(
    root: &Path,
) -> Result<
    (
        Vec<noctua_design_spec::component::Component>,
        CaseSets,
        noctua_design_spec::i18n::Catalogue,
    ),
    String,
> {
    let components_path = root.join("spec/components");
    let components = noctua_design_spec::component::load_all(&components_path)
        .map_err(|error| report(&relative_to(root, &components_path), &error))?;
    ui::ok(&format!("compiled {}", relative_to(root, &components_path)));
    ui::detail(&format!(
        "{} components, {} props in total",
        components.len(),
        components.iter().map(|c| c.prop.len()).sum::<usize>()
    ));

    let cases = noctua_design_spec::conformance::load_all(&components_path, &components)
        .map_err(|error| report(&relative_to(root, &components_path), &error))?;
    // A CONTRACT THAT PROMISES BEHAVIOUR OWES THE CASES THAT HOLD IT TO IT.
    //
    // `NOCTUA.md §9.3` Leg 3. A `[keyboard]` block or a `[state]` block is a
    // promise to a reader, and a promise nothing checks is how `Switch` came to
    // ship with no way to switch it. A component that declares neither owes
    // nothing: there is nothing to drive.
    let missing: Vec<&str> = components
        .iter()
        .filter(|component| noctua_design_spec::conformance::owed(component))
        .filter(|component| {
            !cases
                .iter()
                .any(|(name, set)| name == &component.meta.name && !set.case.is_empty())
        })
        .map(|component| component.meta.name.as_str())
        .collect();
    if !missing.is_empty() {
        return Err(format!(
            "these declare a keyboard or a state and have no cases file: {}\n                      write spec/components/<name>.cases.toml, or take the promise out of the contract",
            missing.join(", ")
        ));
    }
    let owed = components
        .iter()
        .filter(|component| noctua_design_spec::conformance::owed(component))
        .count();
    ui::detail(&format!(
        "{} of them declare behaviour and answer to {} case(s) in {} file(s)",
        owed,
        cases.iter().map(|(_, set)| set.case.len()).sum::<usize>(),
        cases.len()
    ));

    let messages = root.join("spec/i18n");
    let derived = noctua_design_spec::component::derived_messages(&components).len();
    let catalogue = noctua_design_spec::i18n::load_for(&messages, &components)
        .map_err(|error| report(&relative_to(root, &messages), &error))?;
    ui::ok(&format!("compiled {}", relative_to(root, &messages)));
    ui::detail(&format!(
        "{} messages × {} locales ({}, one of them generated)",
        catalogue.keys.len(),
        catalogue.locales.len(),
        catalogue
            .locales
            .iter()
            .map(|locale| locale.tag.clone())
            .collect::<Vec<_>>()
            .join(", ")
    ));
    ui::detail(&format!(
        "{derived} of them derived from spec/components, and translated like any other"
    ));

    Ok((components, cases, catalogue))
}

// One long function on purpose: it is the list of what the build reads, in the
// order dependency forces, and every entry is four lines of the same shape.
// Splitting it would put half the list behind a call and teach a reader that
// the other half lives somewhere else.
#[allow(clippy::too_many_lines)]
/// The spec, the vendored color output, and every file they emit.
///
/// The vendor comes back with them because `gates::unknown_token` needs the
/// color names: it used to wave every `--nc-*` through, and a name that does
/// not exist is dropped at computed-value time with no error anywhere.
pub(crate) fn artifacts(
    root: &Path,
    spec_path: &Path,
) -> Result<(Spec, noctua_design_spec::Vendor, Vec<EmittedFile>), String> {
    let relative = relative_to(root, spec_path);

    let spec = noctua_design_spec::load(spec_path).map_err(|error| report(&relative, &error))?;

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
        .map_err(|error| report(&relative_to(root, &fonts_path), &error))?;
    ui::ok(&format!("compiled {}", relative_to(root, &fonts_path)));
    ui::detail(&format!(
        "{} faces, {} preloaded, {} stacks",
        fonts.face.len(),
        fonts.face.iter().filter(|f| f.preload).count(),
        fonts.stack.len()
    ));

    let sliced = load_slices(root, &fonts)?;

    let vendor = noctua_design_spec::vendor::load(root)
        .map_err(|error| report("packages/tokens/vendor", &error))?;
    ui::ok("read the vendored noctua-colors output");
    ui::detail(&format!(
        "{} semantic names, {} palettes ({} accents × {} saturations), default {}",
        vendor.contract.len(),
        vendor.themes.len(),
        vendor.accents.len(),
        vendor.saturations.len(),
        vendor.default_theme
    ));

    let theme_path = root.join("spec/theme.toml");
    let theme = noctua_design_spec::load_theme(&theme_path, &vendor)
        .map_err(|error| report(&relative_to(root, &theme_path), &error))?;
    ui::ok(&format!("compiled {}", relative_to(root, &theme_path)));
    ui::detail(&format!(
        "{} settings, {} scopable, {} global",
        theme.setting.len(),
        theme.scopable().count(),
        theme.global().count()
    ));

    let (components, cases, catalogue) = words(root)?;

    let icons_path = root.join("spec/icons.toml");
    let (icons, drawings) = noctua_design_spec::icons::load(&icons_path, root.join("assets/icons"))
        .map_err(|error| report(&relative_to(root, &icons_path), &error))?;
    ui::ok(&format!("compiled {}", relative_to(root, &icons_path)));
    ui::detail(&format!(
        "{} icons from {} source(s), vendored at a pinned tag",
        drawings.len(),
        icons.source.len()
    ));

    let syntax_path = root.join("spec/syntax.toml");
    let syntax = noctua_design_spec::syntax::load(&syntax_path, &vendor)
        .map_err(|error| report(&relative_to(root, &syntax_path), &error))?;
    ui::ok(&format!("compiled {}", relative_to(root, &syntax_path)));
    ui::detail(&format!(
        "{} syntax themes over {} roles, every token checked against the vendored palettes",
        syntax.theme.len(),
        noctua_design_spec::grammar::Role::ALL.len()
    ));

    let grammars_path = root.join("spec/grammars");
    let grammars = noctua_design_spec::grammar::load_all(&grammars_path)
        .map_err(|error| report(&relative_to(root, &grammars_path), &error))?;
    // THE JAVASCRIPT HALF IS VENDORED INPUT, read the way an icon's drawing is:
    // it is hand-written beside the Rust half and copied into the package
    // verbatim. An emitter that produced it would be a third implementation of
    // the scanner, and three that must agree is worse than two.
    let scanner_path = root.join("crates/noctua-design-grammars/src/scan.js");
    let scanner = std::fs::read_to_string(&scanner_path).map_err(|error| {
        format!(
            "could not read {}: {error}",
            relative_to(root, &scanner_path)
        )
    })?;
    ui::ok(&format!("compiled {}", relative_to(root, &grammars_path)));
    ui::detail(&format!(
        "{} languages, {} cases, one scanner in two languages",
        grammars.len(),
        grammars.iter().map(|g| g.cases.len()).sum::<usize>()
    ));

    let lineage_path = root.join("spec/lineage/cases.toml");
    let lineage = noctua_design_spec::lineage::load(&lineage_path)
        .map_err(|error| report(&relative_to(root, &lineage_path), &error))?;
    // The other vendored half, for the same reason the scanner is one.
    let layout_path = root.join("crates/noctua-design-lineage/src/layout.js");
    let layout = std::fs::read_to_string(&layout_path).map_err(|error| {
        format!(
            "could not read {}: {error}",
            relative_to(root, &layout_path)
        )
    })?;
    let runtime_path = root.join("crates/noctua-design-lineage/src/runtime.js");
    let runtime = std::fs::read_to_string(&runtime_path).map_err(|error| {
        format!(
            "could not read {}: {error}",
            relative_to(root, &runtime_path)
        )
    })?;
    let three_path = root.join("assets/vendor/three/three-css3d.js");
    let three = std::fs::read_to_string(&three_path)
        .map_err(|error| format!("could not read {}: {error}", relative_to(root, &three_path)))?;
    let licence_path = root.join("assets/vendor/three/LICENSE-three-MIT.txt");
    let three_licence = std::fs::read_to_string(&licence_path).map_err(|error| {
        format!(
            "could not read {}: {error}",
            relative_to(root, &licence_path)
        )
    })?;
    ui::ok(&format!("compiled {}", relative_to(root, &lineage_path)));
    ui::detail(&format!(
        "{} blessed layouts, one engine in two languages",
        lineage.case.len()
    ));

    let mut files = noctua_design_emit::emit_all(noctua_design_emit::Inputs {
        spec: &spec,
        fonts: &fonts,
        vendor: &vendor,
        theme: &theme,
        catalogue: &catalogue,
        components: &components,
        cases: &cases,
        sliced: &sliced,
        icons: &icons,
        drawings: &drawings,
        grammars: &grammars,
        syntax: &syntax,
        scanner: &scanner,
        lineage: &lineage,
        layout: &layout,
        runtime: &runtime,
        three: &three,
        three_licence: &three_licence,
    });

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

    Ok((spec, vendor, files))
}

/// Reads every sliced family's manifest and reports the total.
///
/// Separate from [`artifacts`] because a build step that is one `map` and one
/// `println` does not need to sit inside a function already near the length
/// limit — and because the reporting is the only part anyone reads.
fn load_slices(
    root: &Path,
    fonts: &noctua_design_spec::Fonts,
) -> Result<Vec<noctua_design_spec::Manifest>, String> {
    let manifests: Vec<_> = fonts
        .sliced
        .iter()
        .map(|family| family.load(&root.join("assets/fonts")))
        .collect::<Result<_, _>>()?;

    if !manifests.is_empty() {
        let slices: usize = manifests.iter().map(|m| m.slices.len()).sum();
        let bytes: u64 = manifests
            .iter()
            .flat_map(|m| &m.slices)
            .map(|s| s.bytes)
            .sum();
        ui::detail(&format!(
            "{} sliced families, {slices} slices, {} MB served on demand",
            manifests.len(),
            bytes / 1_000_000
        ));
    }
    Ok(manifests)
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
    let (spec, _vendor, files) = artifacts(root, spec_path)?;

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
    for line in listing(&files) {
        ui::detail(&line);
    }

    Ok(spec)
}

/// How many files one directory may contribute before it is summarised.
///
/// The icon layer emits two modules per icon, which is three and a half
/// thousand paths — a build log nobody reads is a build log that hides the one
/// line that mattered. A directory over this many becomes one line with a
/// count; everything else is still named.
const LISTED: usize = 8;

/// The emitted paths, with crowded directories collapsed to a count.
fn listing(files: &[EmittedFile]) -> Vec<String> {
    let mut counts: BTreeMap<&str, usize> = BTreeMap::new();
    for file in files {
        *counts
            .entry(file.path.rsplit_once('/').map_or("", |(head, _)| head))
            .or_default() += 1;
    }

    let mut lines = Vec::new();
    let mut summarised: BTreeSet<&str> = BTreeSet::new();
    for file in files {
        let directory = file.path.rsplit_once('/').map_or("", |(head, _)| head);
        let count = counts.get(directory).copied().unwrap_or_default();
        if count <= LISTED {
            lines.push(file.path.clone());
        } else if summarised.insert(directory) {
            lines.push(format!("{directory}/ — {count} file(s)"));
        }
    }
    lines
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
