//! The shape of `spec/tokens.toml`.
//!
//! Every table has a default, so deleting one keeps the build working — the
//! same promise `noctua-colors`' spec makes, and for the same reason: a format
//! where every field is mandatory is a format nobody edits without the manual
//! open.
//!
//! **Unknown keys are errors.** A misspelled key that silently does nothing is
//! far worse than a build that stops, because the token it was meant to change
//! keeps its old value and the diff looks correct.

use indexmap::IndexMap;
use serde::Deserialize;

/// A whole non-colour token set.
#[derive(Debug, Clone, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Spec {
    /// Namespace and other emit-wide settings.
    #[serde(default)]
    pub meta: Meta,

    /// The spacing scale.
    #[serde(default)]
    pub space: Space,

    /// Corner radii.
    #[serde(default)]
    pub radius: IndexMap<String, String>,

    /// Everything typographic.
    #[serde(default)]
    pub r#type: Typography,

    /// Durations and easings.
    #[serde(default)]
    pub motion: Motion,

    /// Named z-indices.
    #[serde(default)]
    pub layer: IndexMap<String, i32>,

    /// Minimum-width breakpoints.
    #[serde(default)]
    pub breakpoint: IndexMap<String, String>,

    /// Focus-ring geometry.
    #[serde(default)]
    pub focus: IndexMap<String, String>,
}

/// Emit-wide settings.
#[derive(Debug, Clone, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Meta {
    /// Custom-property namespace, without dashes: `noctua` produces
    /// `--noctua-space-4`.
    #[serde(default = "default_prefix")]
    pub prefix: String,
}

impl Default for Meta {
    fn default() -> Self {
        Self {
            prefix: default_prefix(),
        }
    }
}

fn default_prefix() -> String {
    "noctua".to_owned()
}

/// The spacing scale: one base step, multiples of it, and named purposes.
///
/// [`Default`] is written out rather than derived, and that is not
/// bookkeeping: `#[serde(default)]` on the *table* hands over
/// `Space::default()` whole when `[space]` is absent, so a derived
/// implementation would produce an empty base and no steps — and the
/// "every table has a default" promise this format makes would be false for
/// the one table the whole scale rests on.
#[derive(Debug, Clone, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Space {
    /// The base step every other value is a multiple of.
    #[serde(default = "default_space_base")]
    pub base: String,

    /// Multipliers of [`Space::base`], in ascending order.
    #[serde(default = "default_space_steps")]
    pub steps: Vec<u32>,

    /// Purposes, each naming a step. `tight = 2` means "two base steps".
    ///
    /// A component asks for a purpose rather than a number, so changing what
    /// "the gap between paragraphs" means is one line here rather than a
    /// search across every component.
    #[serde(default)]
    pub named: IndexMap<String, u32>,
}

impl Default for Space {
    fn default() -> Self {
        Self {
            base: default_space_base(),
            steps: default_space_steps(),
            named: IndexMap::new(),
        }
    }
}

fn default_space_base() -> String {
    "0.25rem".to_owned()
}

fn default_space_steps() -> Vec<u32> {
    vec![0, 1, 2, 3, 4, 6, 8, 12, 16, 24, 32, 48, 64]
}

/// Sizes, line heights, weights, tracking, and the measure.
#[derive(Debug, Clone, Deserialize, Default)]
#[serde(deny_unknown_fields)]
pub struct Typography {
    /// The prose line-length cap, in `ch`.
    #[serde(default)]
    pub measure: Option<u32>,

    /// The narrower cap for text set at a larger size.
    #[serde(default)]
    pub measure_lead: Option<u32>,

    /// Font sizes, smallest first.
    #[serde(default)]
    pub size: IndexMap<String, String>,

    /// Line heights. Unitless, so they scale with the size applied to.
    #[serde(default)]
    pub leading: IndexMap<String, f64>,

    /// Font weights.
    #[serde(default)]
    pub weight: IndexMap<String, u32>,

    /// Letter spacing.
    #[serde(default)]
    pub tracking: IndexMap<String, String>,
}

/// Durations and easings.
#[derive(Debug, Clone, Deserialize, Default)]
#[serde(deny_unknown_fields)]
pub struct Motion {
    /// Each duration and what it collapses to under reduced motion.
    #[serde(default)]
    pub duration: IndexMap<String, Duration>,

    /// Easing curves.
    #[serde(default)]
    pub easing: IndexMap<String, String>,
}

/// One duration, with its reduced-motion collapse.
///
/// The pair is a single token rather than two, because that is what makes
/// honouring `prefers-reduced-motion` a property of the token instead of
/// something every component has to remember.
#[derive(Debug, Clone, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Duration {
    /// The ordinary value.
    pub normal: String,
    /// The value under `prefers-reduced-motion: reduce`.
    ///
    /// Not always `0ms`: a colour change at zero reads as a glitch, while a
    /// *movement* at zero is exactly what a motion-sensitive reader asked for.
    pub reduced: String,
}
