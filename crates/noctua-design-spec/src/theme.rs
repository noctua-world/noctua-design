//! The shape of `spec/theme.toml`.
//!
//! One table per setting, and everything downstream is generated from it. That
//! is what makes the two packages' prop surfaces identical by construction
//! rather than by review: neither writes a value, a default or a key name down.

use indexmap::IndexMap;
use serde::Deserialize;

/// The whole theme model.
#[derive(Debug, Clone, Deserialize, Default)]
#[serde(deny_unknown_fields)]
pub struct Theme {
    /// Emit-wide settings.
    #[serde(default)]
    pub meta: Meta,

    /// Every setting, in declaration order.
    #[serde(default)]
    pub setting: IndexMap<String, Setting>,
}

impl Theme {
    /// The settings a `ThemeScope` may carry.
    pub fn scopable(&self) -> impl Iterator<Item = (&String, &Setting)> {
        self.setting.iter().filter(|(_, s)| s.scope)
    }

    /// The settings that only ever apply to the document.
    pub fn global(&self) -> impl Iterator<Item = (&String, &Setting)> {
        self.setting.iter().filter(|(_, s)| !s.scope)
    }
}

/// Emit-wide settings.
#[derive(Debug, Clone, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Meta {
    /// The attribute a scope stamps.
    #[serde(default = "default_scope_attribute")]
    pub scope_attribute: String,

    /// Where palette stylesheets are served from, relative to the page.
    #[serde(default = "default_stylesheet_base")]
    pub stylesheet_base: String,
}

impl Default for Meta {
    fn default() -> Self {
        Self {
            scope_attribute: default_scope_attribute(),
            stylesheet_base: default_stylesheet_base(),
        }
    }
}

fn default_scope_attribute() -> String {
    "data-noctua-scope".to_owned()
}

fn default_stylesheet_base() -> String {
    "tokens/css".to_owned()
}

/// One setting.
#[derive(Debug, Clone, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Setting {
    /// The values it may take, written out.
    #[serde(default)]
    pub values: Vec<String>,

    /// Or the vendored axis they come from — `axes.accents`, `axes.themes`.
    ///
    /// Naming an axis rather than listing its values is what keeps a palette
    /// added upstream from having to be remembered here.
    #[serde(default)]
    pub values_from: Option<String>,

    /// The value in force when nothing is stored.
    pub default: String,

    /// Whether it may be applied to a subtree.
    ///
    /// `false` is enforced by the emitted selector (`:root[…]`), not by
    /// documentation — so a global setting cannot be scoped by accident.
    #[serde(default)]
    pub scope: bool,

    /// The `localStorage` key.
    pub key: String,

    /// The DOM attribute, when the setting has one.
    ///
    /// `accent` and `saturation` have none: they are inputs to `palette`, which
    /// is what actually reaches the DOM.
    #[serde(default)]
    pub attribute: Option<String>,

    /// The one value that means "stamp no attribute at all".
    #[serde(default)]
    pub omit_when: Option<String>,

    /// The settings this one is computed from.
    #[serde(default)]
    pub derived_from: Vec<String>,
}

impl Setting {
    /// The values this setting may take, resolving a vendored axis.
    #[must_use]
    pub fn resolved_values(&self, vendor: &crate::Vendor) -> Vec<String> {
        match self.values_from.as_deref() {
            Some("axes.accents") => vendor.accents.clone(),
            Some("axes.saturations") => vendor.saturations.clone(),
            Some("axes.themes") => vendor.themes.clone(),
            _ => self.values.clone(),
        }
    }
}
