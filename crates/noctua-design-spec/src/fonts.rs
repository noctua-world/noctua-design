//! The shape of `spec/fonts.toml`.
//!
//! Separate from the token spec because it is a different kind of thing: the
//! token spec describes values, this describes *files on disk and how a
//! browser should fetch them*. Merging them would mean a font change and a
//! spacing change touch the same file for no reason.

use indexmap::IndexMap;
use serde::Deserialize;

/// The whole font layer.
#[derive(Debug, Clone, Deserialize, Default)]
#[serde(deny_unknown_fields)]
pub struct Fonts {
    /// One font stack per role, keyed by role name.
    #[serde(default)]
    pub stack: IndexMap<String, String>,

    /// Named `unicode-range` values, referenced by faces.
    #[serde(default)]
    pub range: IndexMap<String, String>,

    /// Every `@font-face`, in declaration order.
    #[serde(default)]
    pub face: Vec<Face>,

    /// Families shipped as many range-limited slices rather than one file.
    #[serde(default)]
    pub sliced: Vec<Sliced>,
}

/// A family cut into many faces, each covering a narrow range.
///
/// # Why this is not just a lot of `[[face]]` entries
///
/// It would be: a sliced family expands to one `@font-face` per slice, and
/// those are ordinary faces in every way. But `Noto Sans SC` is 133 of them,
/// and three families come to nearly four hundred — four hundred entries a
/// human would have to keep in step with four hundred files, where a single
/// wrong `unicode-range` means a codepoint that silently renders as tofu.
///
/// So the subsetting writes a manifest beside the slices, and this declares the
/// family once. The ranges in the manifest are computed from the files
/// themselves, which is the only way they cannot disagree with them.
#[derive(Debug, Clone, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Sliced {
    /// The internal family name, shared by every slice.
    pub family: String,

    /// The manifest, relative to the font directory.
    pub manifest: String,

    /// Whether the slices declare a variable weight axis.
    #[serde(default)]
    pub axes: Vec<String>,

    /// `font-display` for every slice.
    pub display: String,

    /// Why this family is sliced, for whoever reads the spec.
    pub reason: String,
}

/// One slice, as the manifest records it.
#[derive(Debug, Clone, Deserialize)]
pub struct Slice {
    /// The file name, relative to the font directory.
    pub file: String,
    /// The `unicode-range` this slice covers, computed from the file.
    pub range: String,
    /// Its size, so the spec layer can report the total without stat-ing.
    pub bytes: u64,
}

/// A manifest, as the subsetting wrote it.
#[derive(Debug, Clone, Deserialize)]
pub struct Manifest {
    /// The family every slice belongs to.
    pub family: String,
    /// Every slice, in codepoint order.
    pub slices: Vec<Slice>,
}

impl Sliced {
    /// Reads this family's manifest.
    ///
    /// # Errors
    ///
    /// A missing or unparseable manifest, or one whose family disagrees with
    /// the spec — which would mean the slices on disk belong to a different
    /// font than the one declared here.
    pub fn load(&self, fonts_dir: &std::path::Path) -> Result<Manifest, String> {
        let path = fonts_dir.join(&self.manifest);
        let text = std::fs::read_to_string(&path)
            .map_err(|error| format!("could not read {}: {error}", path.display()))?;
        let manifest: Manifest = serde_json::from_str(&text)
            .map_err(|error| format!("{} does not parse: {error}", path.display()))?;
        if manifest.family != self.family {
            return Err(format!(
                "{} is a manifest for `{}`, but the spec declares `{}`",
                path.display(),
                manifest.family,
                self.family
            ));
        }
        Ok(manifest)
    }
}

/// One `@font-face`.
#[derive(Debug, Clone, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Face {
    /// The internal family name, exactly as the file declares it.
    pub family: String,

    /// The file, relative to `assets/fonts/`.
    pub file: String,

    /// `normal` or `italic`.
    pub style: String,

    /// The variable axes this face carries.
    ///
    /// **This is what drives both of §7.6's traps.** A face listing `wght`
    /// gets `font-weight: 100 900`; one listing `wdth` gets `font-stretch`.
    /// Getting either wrong is silent, so neither is written by hand.
    #[serde(default)]
    pub axes: Vec<String>,

    /// The single weight, for a static face. Ignored when `axes` lists `wght`.
    #[serde(default)]
    pub weight: Option<u32>,

    /// Which named [`Fonts::range`] this face covers.
    pub range: String,

    /// `optional`, `swap`, `block`, `fallback` or `auto`.
    pub display: String,

    /// Whether this face is in the default payload.
    #[serde(default)]
    pub preload: bool,
}

impl Face {
    /// Whether this face varies weight.
    #[must_use]
    pub fn is_variable_weight(&self) -> bool {
        self.axes.iter().any(|axis| axis == "wght")
    }

    /// Whether this face varies width.
    #[must_use]
    pub fn is_variable_width(&self) -> bool {
        self.axes.iter().any(|axis| axis == "wdth")
    }
}
