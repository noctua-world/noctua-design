//! The typefaces, carried to where a registry can reach them.
//!
//! # Why this exists
//!
//! `dist/fonts.css` asks for every face at `../assets/fonts/<file>`, and
//! `assets/fonts/` sits at the repository root, above every package: neither
//! Cargo nor npm packages a file above the package directory. So a consumer who
//! installed from a registry had every `@font-face` and none of the faces, the
//! requests answered 404, and a native host drew its whole interface in the
//! system's fonts. That is `dist.rs`'s defect again, one directory further out.
//!
//! # Why they are not inside `dist/`, and why there is more than one crate
//!
//! Measured, not preferred. The faces come to 16.8 MB and are already
//! compressed, and crates.io refuses a package over 10 MB. The Simplified
//! Chinese slices alone are 8.7 MB. So no one crate can hold them all, with
//! `dist/` or without it, and the cut follows the one line `spec/fonts.toml`
//! already draws: every `[[face]]` in one crate, and each `[[sliced]]` family
//! in a crate of its own. A host that draws no Chinese embeds no Chinese.
//!
//! | Crate | What it carries |
//! |---|---|
//! | `noctua-design-fonts` | every `[[face]]`: the Latin faces, the code face, the symbols, and the other scripts |
//! | `noctua-design-fonts-<family>` | one `[[sliced]]` family's slices, named by the last word of its family in lower case |
//!
//! npm has no such ceiling, so `@noctua-world/design-fonts` is one package
//! holding all of them.
//!
//! # The path contract
//!
//! A page that links `<base>/dist/fonts.css` is asked by it for
//! `<base>/assets/fonts/<file>`. Every crate here writes its faces flat, by
//! the names the stylesheet uses, so a host calls `write_to` on each crate it
//! installed with one directory, `<base>/assets/fonts`, or serves each entry
//! of `FILES` at that path. The generated library says so, and
//! `examples/consumer-rust` proves it from outside: every `url()` the shipped
//! stylesheet writes resolves to a file the crates wrote.
//!
//! # Copied, never rewritten, and the licences travel
//!
//! A face is bytes, and [`crate::EmittedFile`] is text, so the copies are not
//! emitted: [`mirror`] says which file goes where, `cargo xtask build` copies
//! the bytes and `cargo xtask check` holds every copy to its source. Git
//! stores a blob once however many paths name it, so the copies cost a
//! checkout and no history.
//!
//! The SIL Open Font License requires its text to accompany the fonts, so
//! every package carries the licence of every family in it, and
//! `ATTRIBUTION.md`, which says where each symbol set's own terms are.

use std::fmt::Write as _;

use noctua_design_spec::{Fonts, Manifest};

use crate::{CommentStyle, EmittedFile, header_from};

/// Where the faces live in this repository.
pub const SOURCE: &str = "assets/fonts";

/// Where a page's stylesheet reaches them, relative to the directory that
/// holds `dist/`.
pub const SERVED: &str = "assets/fonts";

/// The directory inside a package that holds its faces and their licences.
pub const CARRIED: &str = "fonts";

/// The crate that carries every `[[face]]`.
pub const CRATE: &str = "noctua-design-fonts";

/// The one npm package, which carries every face.
pub const PACKAGE_DIR: &str = "packages/fonts";

/// The record of what is vendored, which travels with every package.
pub const ATTRIBUTION: &str = "ATTRIBUTION.md";

/// The most a crate's faces may weigh, in bytes.
///
/// crates.io refuses a package over 10 MB. A megabyte is left for the
/// licences, the manifest and what a family gains with its next slice, so the
/// failure is here, with a name, and not a `413` on the day of a release.
pub const CRATE_BUDGET: u64 = 9 * 1024 * 1024;

/// One crate of faces.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Carrier {
    /// The crate's name, which is also its directory under `crates/`.
    pub name: String,
    /// What it carries, as its library's first line says it.
    pub what: String,
    /// The faces, by file name, in the order the spec declares them.
    pub files: Vec<String>,
    /// The licence texts that cover them, by file name, and the attribution.
    pub licences: Vec<String>,
}

impl Carrier {
    /// The crate's directory, from the repository root.
    #[must_use]
    pub fn dir(&self) -> String {
        format!("crates/{}", self.name)
    }
}

/// The licence text that covers a family, by its file name in [`SOURCE`].
///
/// A table and not a field of the spec, because it is a fact about what was
/// vendored and `assets/fonts/ATTRIBUTION.md` is where that is recorded. A
/// test holds it to the directory: every name here is a file there, and every
/// licence there is one a package carries.
#[must_use]
pub fn licence_of(family: &str) -> &'static str {
    match family {
        "Noto Sans" => "OFL-Noto_Sans.txt",
        "Noto Serif" => "OFL-Noto_Serif.txt",
        "Noto Sans Display" => "OFL-Noto_Sans_Display.txt",
        "Noctua Iosevka" => "OFL-Iosevka.txt",
        "Symbols Nerd Font" | "Symbols Nerd Font Mono" => "LICENSE-NerdFontsSymbolsOnly.txt",
        // Every other family is a Noto face for one script, and one text
        // covers them all: `ATTRIBUTION.md`, "Licence obligations".
        _ => "OFL-Noto_Sans_scripts.txt",
    }
}

/// The crates the faces are carried in: one for every `[[face]]`, and one for
/// each `[[sliced]]` family whose manifest is among `sliced`.
///
/// A family with no manifest handed over is left out, as `fonts::emit` leaves
/// its faces out of the stylesheet: the build loads every manifest from the
/// spec's own declarations, and a test that passes none asks about none.
#[must_use]
pub fn carriers(fonts: &Fonts, sliced: &[Manifest]) -> Vec<Carrier> {
    let mut whole = Carrier {
        name: CRATE.to_owned(),
        what: "the faces that ship whole, which are the Latin faces, the code face, the \
                   symbols and the other scripts"
            .to_owned(),
        files: Vec::new(),
        licences: Vec::new(),
    };
    for face in &fonts.face {
        if !whole.files.contains(&face.file) {
            whole.files.push(face.file.clone());
        }
        let licence = licence_of(&face.family).to_owned();
        if !whole.licences.contains(&licence) {
            whole.licences.push(licence);
        }
    }
    whole.licences.push(ATTRIBUTION.to_owned());

    let mut out = vec![whole];
    for family in &fonts.sliced {
        let Some(manifest) = sliced
            .iter()
            .find(|manifest| manifest.family == family.family)
        else {
            continue;
        };
        let suffix = family
            .family
            .split_whitespace()
            .next_back()
            .unwrap_or_default()
            .to_ascii_lowercase();
        out.push(Carrier {
            name: format!("{CRATE}-{suffix}"),
            what: format!("{}, cut into slices by `unicode-range`", family.family),
            files: manifest
                .slices
                .iter()
                .map(|slice| slice.file.clone())
                .collect(),
            licences: vec![
                licence_of(&family.family).to_owned(),
                ATTRIBUTION.to_owned(),
            ],
        });
    }
    out
}

/// Every copy a build makes: the file in [`SOURCE`], and where it goes.
///
/// Each face and each licence goes into its crate and into the npm package,
/// under [`CARRIED`], by its own name.
#[must_use]
pub fn mirror(carriers: &[Carrier]) -> Vec<(String, String)> {
    let mut out = Vec::new();
    for carrier in carriers {
        for name in carrier.files.iter().chain(&carrier.licences) {
            let from = format!("{SOURCE}/{name}");
            out.push((from.clone(), format!("{}/{CARRIED}/{name}", carrier.dir())));
            let packaged = format!("{PACKAGE_DIR}/{CARRIED}/{name}");
            if !out.iter().any(|(_, to)| *to == packaged) {
                out.push((from, packaged));
            }
        }
    }
    out
}

/// Each crate's library, and the npm package's list of what it holds.
#[must_use]
pub fn emit(carriers: &[Carrier]) -> Vec<EmittedFile> {
    let mut out: Vec<EmittedFile> = carriers
        .iter()
        .map(|carrier| EmittedFile::new(format!("{}/src/lib.rs", carrier.dir()), library(carrier)))
        .collect();
    out.push(EmittedFile::new(
        format!("{PACKAGE_DIR}/fonts.json"),
        listing(carriers),
    ));
    out
}

/// The npm package's own record: every face it holds and the path a page
/// asks for it at, so a tool copies by a list and not by a glob.
fn listing(carriers: &[Carrier]) -> String {
    let mut out = String::from("{\n");
    let _ = writeln!(
        out,
        "  \"_generated\": \"Generated by noctua-design — do not edit. Source: spec/fonts.toml. \
         Regenerate: {}.\",",
        crate::REGENERATE
    );
    let _ = writeln!(out, "  \"served\": \"{SERVED}\",");
    out.push_str("  \"sets\": {");
    for (index, carrier) in carriers.iter().enumerate() {
        if index > 0 {
            out.push(',');
        }
        let _ = write!(out, "\n    \"{}\": [", carrier.name);
        for (at, file) in carrier.files.iter().enumerate() {
            if at > 0 {
                out.push(',');
            }
            let _ = write!(out, "\n      \"{file}\"");
        }
        out.push_str("\n    ]");
    }
    out.push_str("\n  }\n}\n");
    out
}

/// One crate's whole library: the faces, their licences, and the call that
/// writes them out.
fn library(carrier: &Carrier) -> String {
    let mut out = header_from(CommentStyle::Line("//!"), "spec/fonts.toml");
    let _ = write!(
        out,
        "\n//! Typefaces of the Noctua design system, embedded: {what}.\n\
         //!\n\
         //! `noctua-design-dist` carries `dist/fonts.css`, which declares every\n\
         //! face and asks for each at `../{SERVED}/<file>`: beside the directory\n\
         //! that holds `dist/`. This crate is the files it asks for. Cargo cannot\n\
         //! package a file above the package directory, and the faces are above\n\
         //! every package, so they are copied here byte for byte.\n\
         //!\n\
         //! ```no_run\n\
         //! // build.rs, beside `noctua_design_dist::write_to(\"static/dist\")`\n\
         //! fn main() -> std::io::Result<()> {{\n\
         //!     {ident}::write_to(\"static/{SERVED}\")\n\
         //! }}\n\
         //! ```\n\
         //!\n\
         //! A host that serves from memory serves each entry of [`FILES`] at\n\
         //! [`SERVED`]`/<name>` and has no directory at all.\n\
         //!\n\
         //! The faces keep their own terms, and this library's are not theirs:\n\
         //! [`LICENCES`] holds the text that covers every family here, and the\n\
         //! record of what was vendored. `write_to` writes them beside the faces,\n\
         //! because the SIL Open Font License requires its text to travel with\n\
         //! the fonts.\n\n\
         use std::path::Path;\n\n\
         /// Where a page's stylesheet reaches the faces, relative to the directory\n\
         /// that holds `dist/`.\n\
         pub const SERVED: &str = \"{SERVED}\";\n\n\
         /// Every face: the name `dist/fonts.css` asks for it by, and its bytes.\n\
         pub const FILES: &[(&str, &[u8])] = &[\n",
        what = carrier.what,
        ident = carrier.name.replace('-', "_"),
    );
    for file in &carrier.files {
        let _ = writeln!(
            out,
            "    ({file:?}, include_bytes!(concat!(\"../\", \"{CARRIED}/\", {file:?}))),"
        );
    }
    out.push_str(
        "];\n\n\
         /// The licence of every family here, and the record of what was vendored:\n\
         /// the file's name, and its text.\n\
         pub const LICENCES: &[(&str, &str)] = &[\n",
    );
    for licence in &carrier.licences {
        let _ = writeln!(
            out,
            "    ({licence:?}, include_str!(concat!(\"../\", \"{CARRIED}/\", {licence:?}))),"
        );
    }
    out.push_str(
        "];\n\n\
         /// Writes every face and every licence into `dir`, creating it if needed.\n\
         ///\n\
         /// Flat, by the names the stylesheet uses, so several crates of faces\n\
         /// write into one directory. A file already holding the same bytes is\n\
         /// left alone, so a build script that calls this on every build touches\n\
         /// nothing a file watcher is watching.\n\
         ///\n\
         /// # Errors\n\
         ///\n\
         /// Whatever the filesystem reports: a directory that cannot be created, a\n\
         /// file that cannot be written.\n\
         pub fn write_to(dir: impl AsRef<Path>) -> std::io::Result<()> {\n\
         \x20   let dir = dir.as_ref();\n\
         \x20   std::fs::create_dir_all(dir)?;\n\
         \x20   let texts = LICENCES.iter().map(|(name, text)| (*name, text.as_bytes()));\n\
         \x20   for (name, bytes) in FILES.iter().copied().chain(texts) {\n\
         \x20       let path = dir.join(name);\n\
         \x20       if std::fs::read(&path).is_ok_and(|old| old == bytes) {\n\
         \x20           continue;\n\
         \x20       }\n\
         \x20       std::fs::write(&path, bytes)?;\n\
         \x20   }\n\
         \x20   Ok(())\n\
         }\n",
    );
    out
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeSet;
    use std::path::PathBuf;

    use super::*;

    fn root() -> PathBuf {
        PathBuf::from(concat!(env!("CARGO_MANIFEST_DIR"), "/../.."))
    }

    fn shipped() -> (noctua_design_spec::Spec, Fonts, Vec<Manifest>) {
        let root = root();
        let spec = noctua_design_spec::load(root.join("spec/tokens.toml")).expect("tokens");
        let fonts = noctua_design_spec::load_fonts(root.join("spec/fonts.toml")).expect("fonts");
        let sliced = fonts
            .sliced
            .iter()
            .map(|family| family.load(&root.join(SOURCE)).expect("manifest"))
            .collect();
        (spec, fonts, sliced)
    }

    /// Every file a quoted `"../assets/fonts/<file>"` names in a sheet or a
    /// fragment.
    fn asked(text: &str) -> BTreeSet<String> {
        text.split(&format!("\"../{SERVED}/"))
            .skip(1)
            .filter_map(|rest| rest.split('"').next())
            .map(str::to_owned)
            .collect()
    }

    /// Every face the shipped stylesheet and the preload fragment ask for is
    /// carried by exactly one crate, and no crate carries a face neither asks
    /// for.
    ///
    /// The first consumer installed `dist/` from a registry and had every
    /// `@font-face` and no face: each request was answered 404 and the page
    /// was drawn in the system's fonts.
    #[test]
    fn every_face_the_stylesheet_asks_for_is_carried_once() {
        let (spec, fonts, sliced) = shipped();
        let sheet = crate::fonts::emit(&spec, &fonts, &sliced)
            .into_iter()
            .find(|file| file.path == "dist/fonts.css")
            .expect("the font sheet")
            .contents;
        let preload = crate::fonts::preload_fragment(&fonts).contents;
        let mut wanted = asked(&sheet);
        assert!(
            wanted.len() > 100,
            "the sheet asks for {} faces",
            wanted.len()
        );
        let preloaded = asked(&preload);
        assert!(!preloaded.is_empty(), "the fragment preloads nothing");
        wanted.extend(preloaded);

        let carriers = carriers(&fonts, &sliced);
        let mut carried = BTreeSet::new();
        for carrier in &carriers {
            assert!(
                !carrier.files.is_empty(),
                "{} carries nothing",
                carrier.name
            );
            for file in &carrier.files {
                assert!(
                    carried.insert(file.clone()),
                    "{file} is carried twice, the second time by {}",
                    carrier.name
                );
            }
        }
        for file in &wanted {
            assert!(
                carried.contains(file),
                "the stylesheet asks for {file} and no crate carries it"
            );
        }
        for file in &carried {
            assert!(
                wanted.contains(file),
                "{file} is carried and nothing asks for it"
            );
        }
    }

    /// Every crate fits a registry, measured on the files it would carry.
    #[test]
    fn every_crate_of_faces_fits_the_registry() {
        let (_, fonts, sliced) = shipped();
        let dir = root().join(SOURCE);
        let mut all = 0u64;
        for carrier in carriers(&fonts, &sliced) {
            let bytes: u64 = carrier
                .files
                .iter()
                .chain(&carrier.licences)
                .map(|name| {
                    std::fs::metadata(dir.join(name))
                        .unwrap_or_else(|error| panic!("{name}: {error}"))
                        .len()
                })
                .sum();
            assert!(
                bytes <= CRATE_BUDGET,
                "{} would carry {bytes} bytes, over the {CRATE_BUDGET} a crate may",
                carrier.name
            );
            all += bytes;
        }
        // The measurement the cut rests on: together they fit no one crate.
        assert!(
            all > CRATE_BUDGET,
            "the faces come to {all} bytes, which one crate could carry"
        );
    }

    /// A crate is named for what it holds, and no two share a name.
    #[test]
    fn a_sliced_family_has_a_crate_of_its_own() {
        let (_, fonts, sliced) = shipped();
        let names: Vec<String> = carriers(&fonts, &sliced)
            .into_iter()
            .map(|carrier| carrier.name)
            .collect();
        assert_eq!(names[0], CRATE);
        assert_eq!(names.len(), 1 + fonts.sliced.len());
        let unique: BTreeSet<&String> = names.iter().collect();
        assert_eq!(unique.len(), names.len(), "{names:?}");
        for name in &names[1..] {
            let suffix = name.strip_prefix(&format!("{CRATE}-")).expect("a suffix");
            assert!(
                !suffix.is_empty() && suffix.bytes().all(|byte| byte.is_ascii_lowercase()),
                "{name} is not named by one lower-case word"
            );
            assert!(
                root()
                    .join("crates")
                    .join(name)
                    .join("Cargo.toml")
                    .is_file(),
                "{name} carries a sliced family and has no manifest: a crate is created by \
                 hand, with its Cargo.toml and README.md"
            );
        }
    }

    /// The licences travel: every family's text is a file in the source
    /// directory, every package carries the texts of what it holds, and no
    /// licence in the directory is left behind.
    #[test]
    fn every_licence_travels_with_its_faces() {
        let (_, fonts, sliced) = shipped();
        let dir = root().join(SOURCE);
        let carriers = carriers(&fonts, &sliced);
        let mut travelling = BTreeSet::new();
        for carrier in &carriers {
            assert!(carrier.licences.contains(&ATTRIBUTION.to_owned()));
            assert!(
                carrier.licences.len() >= 2,
                "{} has no licence",
                carrier.name
            );
            for licence in &carrier.licences {
                assert!(dir.join(licence).is_file(), "{licence} is not in {SOURCE}");
                travelling.insert(licence.clone());
            }
        }
        for entry in std::fs::read_dir(&dir).expect("the fonts directory") {
            let name = entry
                .expect("an entry")
                .file_name()
                .to_string_lossy()
                .into_owned();
            if name.starts_with("OFL-") || name.starts_with("LICENSE") {
                assert!(
                    travelling.contains(&name),
                    "{name} is a licence in {SOURCE} that no package carries"
                );
            }
        }
    }

    /// The copies are one per crate and one in the npm package, by the file's
    /// own name, and the library names exactly what is copied beside it.
    #[test]
    fn the_copies_and_the_libraries_name_the_same_files() {
        let (_, fonts, sliced) = shipped();
        let carriers = carriers(&fonts, &sliced);
        let copies = mirror(&carriers);
        let targets: BTreeSet<&String> = copies.iter().map(|(_, to)| to).collect();
        assert_eq!(targets.len(), copies.len(), "a copy is made twice");
        let files = emit(&carriers);
        for carrier in &carriers {
            let library = &files
                .iter()
                .find(|file| file.path == format!("{}/src/lib.rs", carrier.dir()))
                .expect("the crate's library")
                .contents;
            for name in carrier.files.iter().chain(&carrier.licences) {
                assert!(
                    library.contains(&format!("({name:?}, include_")),
                    "{} does not embed {name}",
                    carrier.name
                );
                assert!(targets.contains(&format!("{}/{CARRIED}/{name}", carrier.dir())));
                assert!(targets.contains(&format!("{PACKAGE_DIR}/{CARRIED}/{name}")));
            }
        }
        let listing = &files
            .iter()
            .find(|file| file.path == format!("{PACKAGE_DIR}/fonts.json"))
            .expect("the package's listing")
            .contents;
        assert!(listing.contains(&format!("\"served\": \"{SERVED}\"")));
        assert!(listing.contains("\"NotoSans.woff2\""));
    }
}
