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

    /// Named presets a reader chooses between.
    #[serde(default)]
    pub preset: Presets,

    /// Border widths.
    #[serde(default)]
    pub border: IndexMap<String, String>,

    /// The frosted-surface dimensions.
    #[serde(default)]
    pub glass: Glass,

    /// Background pattern and effect limits.
    #[serde(default)]
    pub effect: Effect,

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

/// The named presets a reader chooses between.
///
/// A preset overrides a whole group of tokens at once, keyed on an attribute,
/// so choosing one is a single DOM change and no component is involved.
///
/// Presets rather than a multiplier: `9999px * 0.5` is still a pill, and a
/// multiplier produces fractional radii nobody designed and that anti-alias
/// badly at small sizes. Token *names* never change either way, which is what
/// keeps every component out of it.
#[derive(Debug, Clone, Deserialize, Default)]
#[serde(deny_unknown_fields)]
pub struct Presets {
    /// Corner-radius presets, keyed by preset name then by radius name.
    #[serde(default)]
    pub radius: IndexMap<String, IndexMap<String, String>>,
}

/// The frosted surface.
///
/// Dimensionless numbers only. A pre-mixed `color-mix(…var(--nc-color-surface)…)`
/// declared at `:root` would freeze to the page's palette exactly as
/// `--nc-color-surface` itself does, and a themed component would show the
/// page's frost. The mix belongs where the token is used.
#[derive(Debug, Clone, Deserialize, Default)]
#[serde(deny_unknown_fields)]
pub struct Glass {
    /// Blur radius per level.
    #[serde(default)]
    pub blur: IndexMap<String, String>,
    /// How much of the surface colour survives, as a percentage.
    #[serde(default)]
    pub opacity: IndexMap<String, u32>,
    /// How much the backdrop's colour is concentrated. Dimensionless.
    #[serde(default)]
    pub saturate: IndexMap<String, String>,
    /// How much the backdrop is lifted. Dimensionless.
    #[serde(default)]
    pub brightness: IndexMap<String, String>,
    /// The lit top edge, as a percentage of the raised-surface colour.
    #[serde(default)]
    pub edge: IndexMap<String, u32>,
    /// The inner sheen below that edge, likewise a percentage.
    #[serde(default)]
    pub sheen: IndexMap<String, u32>,
}

/// Background patterns and ambient effects.
#[derive(Debug, Clone, Deserialize, Default)]
#[serde(deny_unknown_fields)]
pub struct Effect {
    /// Alpha ceilings — the most ink each effect may put over the page.
    ///
    /// Ceilings rather than values: they are what makes "a pattern never hurts
    /// readability" a bounded claim a test can measure.
    #[serde(default)]
    pub alpha: IndexMap<String, u32>,
    /// Blur radii.
    #[serde(default)]
    pub blur: IndexMap<String, String>,
    /// Ambient durations, in seconds.
    ///
    /// Deliberately not in `[motion.duration]`: those carry a reduced-motion
    /// collapse, and `animation-duration: 0ms` on an infinite animation is not
    /// "no motion" — it restarts every frame, and engines disagree about what
    /// renders. Reduced motion is handled by an explicit `animation: none`.
    #[serde(default)]
    pub duration: IndexMap<String, String>,
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
