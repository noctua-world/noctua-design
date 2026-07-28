//! The font assets on disk, checked against what the spec claims.
//!
//! These are the one category of file in this repository that is vendored by
//! hand rather than generated, so nothing else verifies them. A licence that
//! stopped travelling with its font would be a licence violation that no
//! compiler and no lint would ever mention.

use std::collections::BTreeSet;
use std::path::{Path, PathBuf};

fn root() -> PathBuf {
    PathBuf::from(concat!(env!("CARGO_MANIFEST_DIR"), "/../.."))
}

fn assets() -> PathBuf {
    root().join("assets/fonts")
}

/// Case-insensitive extension test.
///
/// The filesystem this runs on is case-sensitive, but the assertion is about
/// what kind of file something is, not how it was typed.
fn has_extension(name: &str, extension: &str) -> bool {
    Path::new(name)
        .extension()
        .is_some_and(|e| e.eq_ignore_ascii_case(extension))
}

fn fonts() -> noctua_design_spec::Fonts {
    noctua_design_spec::load_fonts(root().join("spec/fonts.toml")).expect("the font spec")
}

/// Every font file present must be declared, and every declared file present.
///
/// A file on disk that nothing declares is dead payload nobody will remove; a
/// declaration with no file is a 404 the browser reports and the build does not.
#[test]
fn the_directory_and_the_spec_agree() {
    let declared: BTreeSet<String> = fonts().face.iter().map(|f| f.file.clone()).collect();

    let on_disk: BTreeSet<String> = std::fs::read_dir(assets())
        .expect("assets/fonts")
        .filter_map(Result::ok)
        .map(|e| e.file_name().to_string_lossy().into_owned())
        .filter(|name| has_extension(name, "woff2"))
        .collect();

    assert_eq!(
        declared, on_disk,
        "spec/fonts.toml and assets/fonts/ disagree about which files exist"
    );
}

/// Every family copied has its licence here, and no licence is here for a
/// family that was not copied.
///
/// `NOCTUA-PLAN.md` Stage 3, acceptance criterion 6 — and it is checked in
/// *both* directions deliberately. A missing licence is a licence violation. A
/// spare one is a claim that this repository ships a font it does not, which
/// is how an audit ends up looking for a file that was never here.
#[test]
fn every_copied_family_has_its_licence_and_no_others() {
    // The licence file each family's presence obliges.
    const OBLIGATIONS: [(&str, &str); 5] = [
        ("Noto Sans", "OFL-Noto_Sans.txt"),
        ("Noto Serif", "OFL-Noto_Serif.txt"),
        ("Noto Sans Display", "OFL-Noto_Sans_Display.txt"),
        ("Noctua Iosevka", "OFL-Iosevka.txt"),
        ("Symbols Nerd Font", "LICENSE-NerdFontsSymbolsOnly.txt"),
    ];

    let families: BTreeSet<String> = fonts().face.iter().map(|f| f.family.clone()).collect();

    let required: BTreeSet<&str> = OBLIGATIONS
        .iter()
        .filter(|(family, _)| {
            // "Symbols Nerd Font Mono" is the same MIT-licensed release.
            families.iter().any(|f| f.starts_with(family))
        })
        .map(|(_, licence)| *licence)
        .collect();

    let present: BTreeSet<String> = std::fs::read_dir(assets())
        .expect("assets/fonts")
        .filter_map(Result::ok)
        .map(|e| e.file_name().to_string_lossy().into_owned())
        .filter(|name| has_extension(name, "txt"))
        .collect();

    for licence in &required {
        assert!(
            present.contains(*licence),
            "{licence} is missing, and its family is shipped here. The licence text \
             must travel with the fonts (NOCTUA.md §7.8)"
        );
    }

    for licence in &present {
        assert!(
            required.contains(licence.as_str()),
            "{licence} is here but its family is not shipped. A spare licence claims \
             this repository distributes a font it does not"
        );
    }
}

/// Every family a font stack names must either be shipped here or be a real
/// platform fallback.
///
/// A stack naming a family that is neither is a silent no-op: the browser skips
/// it and moves on, so a typo in a family name costs nothing at build time and
/// everything at render time.
#[test]
fn every_stack_entry_is_shipped_or_is_a_real_fallback() {
    // Families that ship with some major operating system, per NOCTUA.md §7.4.
    // The point of the list is that every entry is *usable*, not decorative.
    const PLATFORM: [&str; 17] = [
        "ui-sans-serif",
        "ui-serif",
        "ui-monospace",
        "system-ui",
        "Segoe UI",
        "Roboto",
        "Helvetica Neue",
        "Arial",
        "Georgia",
        "Times New Roman",
        "SF Mono",
        "Cascadia Mono",
        "Roboto Mono",
        "Menlo",
        "Consolas",
        "Liberation Mono",
        "sans-serif",
    ];
    const GENERIC: [&str; 3] = ["serif", "monospace", "cursive"];

    let fonts = fonts();
    let shipped: BTreeSet<String> = fonts.face.iter().map(|f| f.family.clone()).collect();

    for (role, stack) in &fonts.stack {
        // `ui` is an indirection onto another stack, not a family list.
        if stack.trim_start().starts_with("var(") {
            continue;
        }
        for family in stack.split(',') {
            let family = family.trim().trim_matches('"');
            if family.is_empty() || family.starts_with("var(") {
                continue;
            }
            assert!(
                shipped.contains(family) || PLATFORM.contains(&family) || GENERIC.contains(&family),
                "the {role} stack names `{family}`, which is neither shipped here nor a \
                 platform fallback. A browser skips an unknown family silently"
            );
        }
    }
}

/// The attribution must record the exact command that reproduces the subsets.
///
/// A vendored binary whose provenance is "someone ran a tool once" is a
/// binary nobody can reproduce, verify, or safely update.
#[test]
fn the_attribution_is_reproducible() {
    let text = std::fs::read_to_string(assets().join("ATTRIBUTION.md")).expect("ATTRIBUTION.md");

    for required in [
        "pyftsubset",
        "varLib.instancer",
        "--unicodes=",
        "--layout-features=",
        "nix-shell",
    ] {
        assert!(
            text.contains(required),
            "ATTRIBUTION.md does not record `{required}`, so the subsets cannot be reproduced"
        );
    }

    // And the range it publishes must be the one the spec declares, or a
    // reader regenerating from the documented command produces files that do
    // not match the declarations.
    let declared = &fonts().range["latin"];
    for part in declared.split(',').map(str::trim) {
        assert!(
            text.contains(part),
            "the spec's latin range includes {part}, which ATTRIBUTION.md's invocation \
             does not — regenerating from the document would produce a different subset"
        );
    }
}

/// No CJK face may be here at all at this stage.
///
/// `NOCTUA-PLAN.md` Stage 3 says to copy only what is needed, and Group B is
/// 15.15 MiB of fonts nothing in this repository renders.
#[test]
fn no_cjk_face_was_copied() {
    for entry in std::fs::read_dir(assets()).expect("assets/fonts") {
        let name = entry
            .expect("entry")
            .file_name()
            .to_string_lossy()
            .into_owned();
        for cjk in ["JP", "KR", "SC", "TC", "CJK"] {
            assert!(
                !(has_extension(&name, "woff2") && name.contains(cjk)),
                "{name} is a CJK face. They arrive with content that renders them"
            );
        }
    }
}

/// Files vendored by hand must not look like build output, or someone will
/// eventually "clean" them.
#[test]
fn the_attribution_says_these_are_not_build_output() {
    let text = std::fs::read_to_string(assets().join("ATTRIBUTION.md")).expect("ATTRIBUTION.md");
    assert!(
        text.contains("not build output"),
        "ATTRIBUTION.md must say plainly that nothing regenerates these files"
    );
}

/// Sanity: the assets directory is where the emitter says it is.
#[test]
fn the_emitted_paths_resolve() {
    let css = {
        let spec = noctua_design_spec::load(root().join("spec/tokens.toml")).expect("tokens");
        noctua_design_emit::fonts::emit(&spec, &fonts())
            .remove(0)
            .contents
    };
    for face in &fonts().face {
        let url = format!("../assets/fonts/{}", face.file);
        assert!(css.contains(&url), "{url} is not in the emitted CSS");
        // dist/fonts.css resolves `../assets/fonts/...` against dist/.
        let resolved: PathBuf = root().join("dist").join(&url);
        assert!(
            Path::new(&resolved).exists(),
            "{url} does not resolve from dist/, so the browser would 404"
        );
    }
}
