//! The typefaces, copied to the packages that carry them to a registry.
//!
//! `crates/noctua-design-emit/src/faces.rs` decides which face goes in which
//! crate and says why there is more than one. A face is bytes and an emitted
//! file is text, so the copies are made here and held here: `build` writes a
//! copy whose bytes differ from its source, and `check` fails on one that
//! differs, on one that is missing, and on a file in a carried directory that
//! nothing copies there. A stale face would otherwise be published for ever,
//! because `build` never deletes.

use std::collections::BTreeSet;
use std::path::Path;

use noctua_design_emit::faces::{CARRIED, PACKAGE_DIR, SOURCE, carriers, mirror};

/// One copy: the file it is made from and where it goes, from the repository
/// root.
type Copy = (String, String);

/// Every copy to make, and every directory the copies are carried in.
fn plan(root: &Path) -> Result<(Vec<Copy>, BTreeSet<String>), String> {
    let path = root.join("spec/fonts.toml");
    let fonts = noctua_design_spec::load_fonts(&path)
        .map_err(|error| format!("could not read spec/fonts.toml: {error}"))?;
    let sliced = fonts
        .sliced
        .iter()
        .map(|family| family.load(&root.join(SOURCE)))
        .collect::<Result<Vec<_>, _>>()?;
    let carriers = carriers(&fonts, &sliced);
    let mut dirs: BTreeSet<String> = carriers
        .iter()
        .map(|carrier| format!("{}/{CARRIED}", carrier.dir()))
        .collect();
    dirs.insert(format!("{PACKAGE_DIR}/{CARRIED}"));
    for carrier in &carriers {
        if !root.join(carrier.dir()).join("Cargo.toml").is_file() {
            return Err(format!(
                "{} carries faces and has no Cargo.toml: a crate of faces is created by hand, \
                 with its manifest and its README.md, as the ones beside it were",
                carrier.dir()
            ));
        }
    }
    Ok((mirror(&carriers), dirs))
}

/// Copies every face and licence whose copy differs from its source, and
/// answers how many copies there are and how many were written.
///
/// # Errors
///
/// A source that cannot be read, a copy that cannot be written, or a crate of
/// faces with no manifest.
pub(crate) fn write(root: &Path) -> Result<(usize, usize), String> {
    let (copies, _) = plan(root)?;
    Ok((copies.len(), copy(root, &copies)?))
}

/// Makes the copies that differ from their sources, and answers how many.
fn copy(root: &Path, copies: &[(String, String)]) -> Result<usize, String> {
    let mut written = 0usize;
    for (from, to) in copies {
        let bytes = std::fs::read(root.join(from))
            .map_err(|error| format!("could not read {from}: {error}"))?;
        let target = root.join(to);
        // Only when the bytes differ, for the reason `build::run` gives: a
        // rewritten file has a new mtime, and a watcher rebuilds for nothing.
        if std::fs::read(&target).is_ok_and(|old| old == bytes) {
            continue;
        }
        if let Some(parent) = target.parent() {
            std::fs::create_dir_all(parent)
                .map_err(|error| format!("could not create {}: {error}", parent.display()))?;
        }
        std::fs::write(&target, &bytes)
            .map_err(|error| format!("could not write {to}: {error}"))?;
        written += 1;
    }
    Ok(written)
}

/// What is wrong with the copies, one line each, and how many were read.
///
/// # Errors
///
/// A source that cannot be read, or a crate of faces with no manifest.
pub(crate) fn drifted(root: &Path) -> Result<(usize, Vec<String>), String> {
    let (copies, dirs) = plan(root)?;
    Ok((copies.len(), compare(root, &copies, &dirs)?))
}

/// What is wrong with a set of copies and the directories they are carried in.
fn compare(
    root: &Path,
    copies: &[(String, String)],
    dirs: &BTreeSet<String>,
) -> Result<Vec<String>, String> {
    let mut problems = Vec::new();
    for (from, to) in copies {
        let bytes = std::fs::read(root.join(from))
            .map_err(|error| format!("could not read {from}: {error}"))?;
        match std::fs::read(root.join(to)) {
            Ok(copy) if copy == bytes => {}
            Ok(_) => problems.push(format!("{to} is not {from}, byte for byte")),
            Err(_) => problems.push(format!("{to} is missing")),
        }
    }
    let expected: BTreeSet<&str> = copies.iter().map(|(_, to)| to.as_str()).collect();
    for dir in dirs {
        let Ok(entries) = std::fs::read_dir(root.join(dir)) else {
            continue;
        };
        for entry in entries.filter_map(Result::ok) {
            let path = format!("{dir}/{}", entry.file_name().to_string_lossy());
            if !expected.contains(path.as_str()) {
                problems.push(format!(
                    "{path} is carried and nothing copies it there: delete it, because a \
                     build never does"
                ));
            }
        }
    }
    problems.sort();
    Ok(problems)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn root() -> std::path::PathBuf {
        std::path::PathBuf::from(concat!(env!("CARGO_MANIFEST_DIR"), "/.."))
    }

    /// The copies in this checkout are their sources, and nothing else is
    /// carried beside them.
    #[test]
    fn every_carried_face_is_its_source() {
        let (read, problems) = drifted(&root()).expect("the plan");
        assert!(read > 150, "only {read} copies were read");
        assert!(problems.is_empty(), "{problems:#?}");
    }

    /// The defects the check is for: a copy that differs, one that is gone,
    /// and a stale file nothing copies.
    #[test]
    fn a_copy_that_differs_is_missing_or_is_stale_is_named() {
        let scratch = std::env::temp_dir().join(format!("nd-faces-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&scratch);
        std::fs::create_dir_all(scratch.join("from")).expect("mkdir");
        std::fs::write(scratch.join("from/A.woff2"), b"a face").expect("a source");
        std::fs::write(scratch.join("from/B.woff2"), b"another").expect("a source");
        let copies = vec![
            ("from/A.woff2".to_owned(), "to/fonts/A.woff2".to_owned()),
            ("from/B.woff2".to_owned(), "to/fonts/B.woff2".to_owned()),
        ];
        let dirs = BTreeSet::from(["to/fonts".to_owned()]);

        assert_eq!(copy(&scratch, &copies), Ok(2));
        // A copy that is already its source is left alone.
        assert_eq!(copy(&scratch, &copies), Ok(0));
        assert_eq!(compare(&scratch, &copies, &dirs), Ok(Vec::new()));

        std::fs::write(scratch.join("to/fonts/A.woff2"), b"not a face").expect("edit");
        std::fs::remove_file(scratch.join("to/fonts/B.woff2")).expect("remove");
        std::fs::write(scratch.join("to/fonts/Gone.woff2"), b"stale").expect("a stale face");
        let problems = compare(&scratch, &copies, &dirs).expect("read");
        assert_eq!(problems.len(), 3, "{problems:#?}");
        assert!(problems[0].contains("to/fonts/A.woff2 is not from/A.woff2"));
        assert_eq!(problems[1], "to/fonts/B.woff2 is missing");
        assert!(
            problems[2].starts_with("to/fonts/Gone.woff2") && problems[2].contains("delete it")
        );
        std::fs::remove_dir_all(&scratch).expect("clean up");
    }
}
