//! The prop surface of `ThemeScope`, as documentation reads it.
//!
//! Derived from the generated theme model rather than written down. `ThemeScope`
//! declares one prop per scopable setting and nothing else, so this list and the
//! component's signature are the same list by construction — a documented prop
//! cannot disagree with the implemented one, which is the property
//! `NOCTUA.md §9.3` asks a component page to have.

use noctua_design_i18n::Key;
use noctua_design_theme as theme;

/// One documented prop.
///
/// `PartialEq` because a Dioxus component takes it as a prop now: `WrittenProps`
/// renders both of these lists, and a component's props have to be comparable
/// for the framework to know when nothing has changed.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct Prop {
    /// The prop name, as both packages spell it.
    pub name: &'static str,
    /// The type, without the `Option` the component wraps it in.
    pub type_name: &'static str,
    /// What it accepts, as a sentence — and therefore a message.
    ///
    /// It was an English string sitting in a Rust array, printed verbatim into
    /// the Portuguese page. "thirty-nine, one per accent × saturation" is prose
    /// a reader reads, not an identifier, and prose a reader reads goes through
    /// the catalogue.
    pub values: Key,
    /// What it resolves to when the prop is omitted and nothing is inherited.
    pub default: &'static str,
}

/// Every scopable setting, in the order the model declares them.
///
/// The value lists are built from each enum's `ALL`, so a palette added upstream
/// in `noctua-colors` reaches this page on the next export.
pub(crate) const SCOPABLE: [Prop; 10] = [
    Prop {
        name: "mode",
        type_name: "Mode",
        values: Key::ScopePropMode,
        default: theme::Mode::DEFAULT.as_str(),
    },
    Prop {
        name: "accent",
        type_name: "Accent",
        values: Key::ScopePropAccent,
        default: theme::Accent::DEFAULT.as_str(),
    },
    Prop {
        name: "saturation",
        type_name: "Saturation",
        values: Key::ScopePropSaturation,
        default: theme::Saturation::DEFAULT.as_str(),
    },
    Prop {
        name: "palette",
        type_name: "Palette",
        values: Key::ScopePropPalette,
        default: theme::Palette::DEFAULT.as_str(),
    },
    Prop {
        name: "font",
        type_name: "Font",
        values: Key::ScopePropFont,
        default: theme::Font::DEFAULT.as_str(),
    },
    Prop {
        name: "glass",
        type_name: "Glass",
        values: Key::ScopePropGlass,
        default: theme::Glass::DEFAULT.as_str(),
    },
    Prop {
        name: "radius",
        type_name: "Radius",
        values: Key::ScopePropRadius,
        default: theme::Radius::DEFAULT.as_str(),
    },
    // THE FOUR THAT USED TO BE PAGE-ONLY. They were `scope = false` because of
    // how they were emitted — a payload on a descendant, which source order
    // settles rather than proximity — and each declares a custom property on
    // the element carrying its attribute now. Documented here because the
    // component really does accept them, which is the whole difference between
    // a prop and a prop that lies.
    Prop {
        name: "backdrop",
        type_name: "Backdrop",
        values: Key::ScopePropBackdrop,
        default: theme::Backdrop::DEFAULT.as_str(),
    },
    Prop {
        name: "pattern",
        type_name: "Pattern",
        values: Key::ScopePropPattern,
        default: theme::Pattern::DEFAULT.as_str(),
    },
    Prop {
        name: "grain",
        type_name: "Grain",
        values: Key::ScopePropGrain,
        default: theme::Grain::DEFAULT.as_str(),
    },
];

/// The example frame's own props, for the page that documents it.
///
/// Written rather than derived, and it is the same trade `ThemeScope` makes
/// from the other side: that component has no contract file because its props
/// ARE the theme model, and this one has none because it is not a component
/// the packages ship. Three props is a list short enough that writing it down
/// costs less than a mechanism to derive it, and
/// `the_documented_frame_props_are_the_real_ones` is what stops it going
/// stale.
pub(crate) const FRAME: [Prop; 3] = [
    Prop {
        name: "label",
        type_name: "Key",
        values: Key::FramePropLabel,
        default: "—",
    },
    Prop {
        name: "mountable",
        type_name: "bool",
        values: Key::FramePropMountable,
        default: "false",
    },
    Prop {
        name: "children",
        type_name: "Element",
        values: Key::FramePropChildren,
        default: "—",
    },
];

#[cfg(test)]
mod tests {
    use super::*;

    /// The list must cover every scopable setting and nothing else.
    ///
    /// A page-only setting documented as a prop would be a prop that silently
    /// does nothing, and a scopable setting left out is a prop the component
    /// accepts and the page never mentions.
    ///
    /// COMPARED AGAINST THE GENERATED LIST, not against a second one written
    /// out here. It used to be written out, which meant the two hand-written
    /// lists agreed with each other and neither had to agree with
    /// `spec/theme.toml` — so when four settings became scopable this test went
    /// on passing while the documented signature was four props short.
    #[test]
    fn the_documented_props_are_the_scopable_settings() {
        let documented: Vec<&str> = SCOPABLE.iter().map(|prop| prop.name).collect();
        assert_eq!(
            documented,
            theme::SCOPABLE_SETTINGS.to_vec(),
            "the prop table drifted from ThemeScope's signature"
        );
    }

    #[test]
    fn every_default_is_a_real_value() {
        assert!(theme::Mode::parse(SCOPABLE[0].default).is_some());
        assert!(theme::Palette::parse(SCOPABLE[3].default).is_some());
        assert!(theme::Radius::parse(SCOPABLE[6].default).is_some());
        assert!(theme::Backdrop::parse(SCOPABLE[7].default).is_some());
        assert!(theme::Grain::parse(SCOPABLE[9].default).is_some());
    }
}
