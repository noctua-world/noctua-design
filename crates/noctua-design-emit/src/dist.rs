//! `dist/`, mirrored where a registry can reach it.
//!
//! # Why this exists
//!
//! Cargo and npm both refuse to package a file that sits above the package
//! directory. `dist/` sits at the repository root — which is right, because it
//! is what a consumer copies into their static directory — so neither registry
//! could carry it, and `noctua-design` on crates.io would have installed
//! components with no rules to paint them. That is the same defect
//! `styles.rs` exists to remove, arriving one step further out.
//!
//! So each registry gets a package whose entire content is that directory:
//! `noctua-design-dist` on crates.io, `@noctua-world/design-dist` on npm.
//!
//! # Copied, never rewritten
//!
//! Every file here is `contents.clone()` of a file already emitted. Nothing is
//! recomputed, nothing is filtered, and no path is renamed — so the three
//! copies cannot disagree about what shipped. The drift check in
//! `cargo xtask check` compares every emitted file against disk, which makes
//! "the packages are `dist/`, byte for byte" a gate nobody had to write.
//!
//! The Rust half embeds the bytes with `include_str!` rather than reading them
//! at run time, because a consumer calls it from `build.rs`, where the only
//! path that reliably exists is the one the compiler resolved.

use std::fmt::Write as _;

use crate::{CommentStyle, EmittedFile, header_from};

/// The crate that carries the shipped files to crates.io.
pub(crate) const CRATE_DIR: &str = "crates/noctua-design-dist";

/// The package that carries them to npm.
pub(crate) const PACKAGE_DIR: &str = "packages/dist";

/// The prefix that says a file is shipped rather than internal.
const SHIPPED: &str = "dist/";

/// Whether this path is a copy of a shipped file rather than a file of its own.
///
/// The gates that read emitted output — is this JavaScript, does this carry a
/// header — must ask it once, at the path the file is generated for. Asking
/// again here would judge the same bytes twice under a different name, and the
/// second answer can only ever be the first one or a bug.
#[must_use]
pub fn is_mirror(path: &str) -> bool {
    path.starts_with(CRATE_DIR) || path.starts_with(PACKAGE_DIR)
}

/// Mirrors every shipped file into both packages, and writes the crate's
/// library over what it just placed there.
///
/// Takes the emitted set rather than the inputs, because what ships is decided
/// by the other emitters and re-deciding it here is how the two answers drift.
#[must_use]
pub fn emit(files: &[EmittedFile]) -> Vec<EmittedFile> {
    let shipped: Vec<&EmittedFile> = files
        .iter()
        .filter(|file| file.path.starts_with(SHIPPED))
        .collect();

    let mut out = Vec::with_capacity(shipped.len() * 2 + 1);
    for file in &shipped {
        let rest = &file.path[SHIPPED.len()..];
        out.push(EmittedFile::new(
            format!("{CRATE_DIR}/{SHIPPED}{rest}"),
            file.contents.clone(),
        ));
        out.push(EmittedFile::new(
            format!("{PACKAGE_DIR}/{SHIPPED}{rest}"),
            file.contents.clone(),
        ));
    }
    out.push(EmittedFile::new(
        format!("{CRATE_DIR}/src/lib.rs"),
        library(&shipped),
    ));
    out
}

/// The crate's whole library: the files, and the call that writes them out.
fn library(shipped: &[&EmittedFile]) -> String {
    let mut out = header_from(CommentStyle::Line("//!"), SHIPPED);
    out.push_str(
        "\n//! What the Noctua design system ships, embedded.\n\
         //!\n\
         //! `dist/` is what a page links: the stylesheets in the order they resolve\n\
         //! in, the behaviour modules, the palettes and the theme data they read.\n\
         //! This crate is that directory, so a consumer who installs from crates.io\n\
         //! receives it — Cargo cannot package a file above the package directory,\n\
         //! and the directory is above every package that would carry it.\n\
         //!\n\
         //! ```no_run\n\
         //! // build.rs\n\
         //! fn main() -> std::io::Result<()> {\n\
         //!     noctua_design_dist::write_to(\"static/dist\")\n\
         //! }\n\
         //! ```\n\
         //!\n\
         //! `dist/base.html` is the list of links, generated, and the order in it is\n\
         //! load-bearing. `dist/bootstrap.html` has to run before the first paint or\n\
         //! the page has no palette at all.\n\n\
         use std::path::Path;\n\n\
         /// Every shipped file: the name to write it under, and its bytes.\n\
         ///\n\
         /// In the order the emitter produced them, which is neither alphabetical\n\
         /// nor the link order — `base.html` is what states the link order.\n\
         pub const FILES: &[(&str, &str)] = &[\n",
    );
    for file in shipped {
        let rest = &file.path[SHIPPED.len()..];
        let _ = writeln!(
            out,
            "    ({rest:?}, include_str!(concat!(\"../\", \"{SHIPPED}\", {rest:?}))),"
        );
    }
    out.push_str(
        "];\n\n\
         /// Writes every shipped file into `dir`, creating the directories it needs.\n\
         ///\n\
         /// Subdirectories are part of the name — `palettes/azure-vivid.json` — and\n\
         /// `dist/fonts.css` reaches the faces at `../assets/fonts/`, so whatever\n\
         /// `dir` is, the fonts belong beside it and not inside it.\n\
         ///\n\
         /// # Errors\n\
         ///\n\
         /// Whatever the filesystem returns: a directory that cannot be created, a\n\
         /// file that cannot be written.\n\
         pub fn write_to(dir: impl AsRef<Path>) -> std::io::Result<()> {\n    \
             let dir = dir.as_ref();\n    \
             for (name, contents) in FILES {\n        \
                 let path = dir.join(name);\n        \
                 if let Some(parent) = path.parent() {\n            \
                     std::fs::create_dir_all(parent)?;\n        \
                 }\n        \
                 std::fs::write(path, contents)?;\n    \
             }\n    \
             Ok(())\n\
         }\n",
    );
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    fn shipped(paths: &[&str]) -> Vec<EmittedFile> {
        paths
            .iter()
            .map(|path| EmittedFile::new(*path, format!("/* {path} */\n")))
            .collect()
    }

    /// Both packages carry the same files, under the same names.
    ///
    /// A registry consumer and a `git clone` consumer are reading the same
    /// documentation. The moment one package holds a file the other does not,
    /// that documentation is wrong for one of them and nothing says which.
    #[test]
    fn the_two_packages_carry_the_same_names() {
        let files = shipped(&["dist/base.css", "dist/palettes/azure-vivid.json"]);
        let out = emit(&files);
        let names = |dir: &str| -> Vec<String> {
            out.iter()
                .filter(|file| file.path.starts_with(dir))
                .filter(|file| !file.path.ends_with("src/lib.rs"))
                .map(|file| file.path[dir.len() + 1..].to_owned())
                .collect()
        };
        assert_eq!(names(CRATE_DIR), names(PACKAGE_DIR));
        assert_eq!(
            names(PACKAGE_DIR),
            vec![
                "dist/base.css".to_owned(),
                "dist/palettes/azure-vivid.json".to_owned()
            ]
        );
    }

    /// A mirrored file is the file, byte for byte.
    ///
    /// The whole reason this module copies rather than generates. A header, a
    /// trailing newline, a rewritten path — any of them would make the shipped
    /// stylesheet differ from the one this repository tests against.
    #[test]
    fn a_mirrored_file_is_the_file() {
        let files = shipped(&["dist/components.css"]);
        let out = emit(&files);
        for copy in out.iter().filter(|file| file.path.contains("/dist/")) {
            assert_eq!(copy.contents, files[0].contents, "{}", copy.path);
        }
    }

    /// Only what ships is mirrored.
    ///
    /// The emitter also writes generated Rust, the React package and the
    /// documentation site. A package that carried those would be shipping this
    /// repository's insides to people who asked for its stylesheets.
    #[test]
    fn nothing_that_is_not_shipped_is_carried() {
        let mut files = shipped(&["dist/base.css"]);
        files.push(EmittedFile::new(
            "packages/react/src/index.js",
            "export {};",
        ));
        files.push(EmittedFile::new("docs-site/js/registry.js", "export {};"));
        let out = emit(&files);
        assert!(!out.iter().any(|file| file.path.contains("react/src")));
        assert!(!out.iter().any(|file| file.path.contains("docs-site")));
        assert_eq!(out.len(), 3, "two copies and the library");
    }

    /// The library names every file it was handed, and nothing else.
    #[test]
    fn the_library_names_every_file() {
        let files = shipped(&["dist/base.css", "dist/palettes/azure-vivid.json"]);
        let out = emit(&files);
        let library = out
            .iter()
            .find(|file| file.path.ends_with("src/lib.rs"))
            .expect("the library");
        assert_eq!(
            library.contents.matches("include_str!").count(),
            files.len()
        );
        assert!(library.contents.contains("\"base.css\""));
        assert!(library.contents.contains("\"palettes/azure-vivid.json\""));
        assert!(!library.contents.contains("\"dist/base.css\""));
    }
}
