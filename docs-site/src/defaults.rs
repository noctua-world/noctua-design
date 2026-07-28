//! The value each setting has when nothing is stored.
//!
//! Read from the generated theme model, so the markup the server renders and
//! the state the script restores agree by construction.
//!
//! # Why this has to be server-rendered
//!
//! Every option in the settings panel shipped with `aria-checked="false"` and
//! was corrected by the script on load. With JavaScript off — or before it runs
//! — a screen reader was told that none of the forty-one options was selected,
//! which is worse than saying nothing: it is a confident wrong answer.

use noctua_design_theme as theme;

/// The default value for a setting, or `""` for one this page does not know.
///
/// `""` never equals a real value, so an unknown setting renders every option
/// unchecked — the old behaviour, and the correct one for something genuinely
/// unknown rather than something merely not looked up.
#[must_use]
pub(crate) fn of(setting: &str) -> &'static str {
    match setting {
        "mode" => theme::Mode::DEFAULT.as_str(),
        "accent" => theme::Accent::DEFAULT.as_str(),
        "saturation" => theme::Saturation::DEFAULT.as_str(),
        "palette" => theme::Palette::DEFAULT.as_str(),
        "font" => theme::Font::DEFAULT.as_str(),
        "glass" => theme::Glass::DEFAULT.as_str(),
        "radius" => theme::Radius::DEFAULT.as_str(),
        "blackout" => theme::Blackout::DEFAULT.as_str(),
        "pattern" => theme::Pattern::DEFAULT.as_str(),
        "glow" => theme::Glow::DEFAULT.as_str(),
        "grain" => theme::Grain::DEFAULT.as_str(),
        _ => "",
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Every setting the panel renders must be known here, or its options ship
    /// announcing that none of them is selected.
    #[test]
    fn every_settings_group_has_a_known_default() {
        for setting in [
            "mode",
            "accent",
            "saturation",
            "font",
            "glass",
            "radius",
            "blackout",
            "pattern",
            "glow",
            "grain",
        ] {
            assert!(!of(setting).is_empty(), "{setting} has no default here");
        }
    }

    #[test]
    fn an_unknown_setting_marks_nothing_checked() {
        assert_eq!(of("not-a-setting"), "");
    }
}
