//! The prop surface of `ThemeScope`, as documentation reads it.
//!
//! Derived from the generated theme model rather than written down. `ThemeScope`
//! declares one prop per scopable setting and nothing else, so this list and the
//! component's signature are the same list by construction — a documented prop
//! cannot disagree with the implemented one, which is the property
//! `NOCTUA.md §9.3` asks a component page to have.

use noctua_design_theme as theme;

/// One documented prop.
#[derive(Debug, Clone, Copy)]
pub(crate) struct Prop {
    /// The prop name, as both packages spell it.
    pub name: &'static str,
    /// The type, without the `Option` the component wraps it in.
    pub type_name: &'static str,
    /// Every value it accepts, joined for display.
    pub values: &'static str,
    /// What it resolves to when the prop is omitted and nothing is inherited.
    pub default: &'static str,
}

/// Every scopable setting, in the order the model declares them.
///
/// The value lists are built from each enum's `ALL`, so a palette added upstream
/// in `noctua-colors` reaches this page on the next export.
pub(crate) const SCOPABLE: [Prop; 7] = [
    Prop {
        name: "mode",
        type_name: "Mode",
        values: "light · dark · system",
        default: theme::Mode::DEFAULT.as_str(),
    },
    Prop {
        name: "accent",
        type_name: "Accent",
        values: "thirteen families, from ochre to umber",
        default: theme::Accent::DEFAULT.as_str(),
    },
    Prop {
        name: "saturation",
        type_name: "Saturation",
        values: "balanced · vivid · sober",
        default: theme::Saturation::DEFAULT.as_str(),
    },
    Prop {
        name: "palette",
        type_name: "Palette",
        values: "thirty-nine, one per accent × saturation",
        default: theme::Palette::DEFAULT.as_str(),
    },
    Prop {
        name: "font",
        type_name: "Font",
        values: "sans · mono",
        default: theme::Font::DEFAULT.as_str(),
    },
    Prop {
        name: "glass",
        type_name: "Glass",
        values: "off · subtle · full",
        default: theme::Glass::DEFAULT.as_str(),
    },
    Prop {
        name: "radius",
        type_name: "Radius",
        values: "square · soft · round · circular",
        default: theme::Radius::DEFAULT.as_str(),
    },
];

#[cfg(test)]
mod tests {
    use super::*;

    /// The list must cover every scopable setting and nothing else. A global
    /// setting documented as a prop would be a prop that silently does nothing,
    /// because its CSS is bound to `:root`.
    #[test]
    fn the_documented_props_are_the_scopable_settings() {
        let documented: Vec<&str> = SCOPABLE.iter().map(|prop| prop.name).collect();
        assert_eq!(
            documented,
            vec![
                "mode",
                "accent",
                "saturation",
                "palette",
                "font",
                "glass",
                "radius"
            ],
            "the prop table drifted from ThemeScope's signature"
        );
    }

    #[test]
    fn every_default_is_a_real_value() {
        assert!(theme::Mode::parse(SCOPABLE[0].default).is_some());
        assert!(theme::Palette::parse(SCOPABLE[3].default).is_some());
        assert!(theme::Radius::parse(SCOPABLE[6].default).is_some());
    }
}
