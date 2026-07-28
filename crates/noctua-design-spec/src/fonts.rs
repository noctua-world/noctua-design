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
