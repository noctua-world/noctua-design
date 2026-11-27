//! What the crate carries, as a host reads it.

use noctua_design_fonts::{FILES, LICENCES, SERVED, write_to};

/// Every entry is a WOFF2 face under the name the stylesheet asks for it by,
/// and no name is carried twice.
#[test]
fn every_file_is_a_face_under_its_own_name() {
    assert!(FILES.len() > 10, "only {} faces are carried", FILES.len());
    let mut names: Vec<&str> = FILES.iter().map(|(name, _)| *name).collect();
    names.sort_unstable();
    names.dedup();
    assert_eq!(names.len(), FILES.len(), "a face is carried twice");
    for (name, bytes) in FILES {
        assert!(
            std::path::Path::new(name)
                .extension()
                .is_some_and(|extension| extension == "woff2"),
            "{name}"
        );
        assert!(!name.contains('/'), "{name} is not written flat");
        assert!(bytes.starts_with(b"wOF2"), "{name} is not a WOFF2 face");
    }
    assert_eq!(SERVED, "assets/fonts");
}

/// The Open Font License travels with the faces, and so does the record of
/// what was vendored.
#[test]
fn the_licences_travel_with_the_faces() {
    assert!(
        LICENCES
            .iter()
            .any(|(_, text)| text.contains("SIL OPEN FONT LICENSE"))
    );
    assert!(LICENCES.iter().any(|(name, _)| *name == "ATTRIBUTION.md"));
    for (name, text) in LICENCES {
        assert!(!text.trim().is_empty(), "{name} is carried empty");
    }
}

/// One call writes every face and every licence flat into the directory it is
/// given, and a second call rewrites none of them.
#[test]
fn write_to_writes_them_flat_and_leaves_what_is_there() {
    let dir = std::env::temp_dir().join(format!("nd-fonts-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&dir);
    write_to(&dir).expect("the first write");
    let (name, bytes) = FILES[0];
    assert_eq!(std::fs::read(dir.join(name)).expect("a face"), bytes);
    for (licence, _) in LICENCES {
        assert!(dir.join(licence).is_file(), "{licence} was not written");
    }
    let written = std::fs::read_dir(&dir).expect("the directory").count();
    assert_eq!(written, FILES.len() + LICENCES.len());

    let before = std::fs::metadata(dir.join(name))
        .and_then(|meta| meta.modified())
        .expect("a time");
    write_to(&dir).expect("the second write");
    let after = std::fs::metadata(dir.join(name))
        .and_then(|meta| meta.modified())
        .expect("a time");
    assert_eq!(before, after, "a face that had not changed was rewritten");
    std::fs::remove_dir_all(&dir).expect("clean up");
}
