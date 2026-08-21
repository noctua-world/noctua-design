//! The theme menu: every appearance setting, in one component, used twice.
//!
//! # Why it is one component and not two panels
//!
//! There were two places to change the appearance and they did not know about
//! each other: a three-icon mode control in the header, and a panel behind a
//! gear that *also* had a mode row and nine more. A reader had two ways to do
//! one thing, and the header spent a third of its 320px budget on the copy.
//!
//! Now there is one menu, and both places open it the same way: a floating
//! panel hanging from the control that opened it. The same controls theme the
//! frame's stage instead of the document — because `shell.js` routes a chosen
//! value by *where the group sits*, not by which panel it was in.
//!
//! # Why it belongs to the site rather than to the packages
//!
//! Its labels are words: *Light*, *Glass*, *Corners*. A component published to
//! both packages may not carry words in any language (`NOCTUA.md §9.9`), and a
//! version that took thirty of them as props would be `Segmented` with extra
//! steps. What it is BUILT from is components, which is the part that matters.
//!
//! # What the panel is, as a design
//!
//! Eleven settings do not have eleven equal weights. Mode and accent are used
//! constantly; grain is used once. So the panel is sectioned, each row is a
//! label beside its control rather than a label above a full-width block, the
//! booleans are `Switch`es rather than two-button radio groups, and the reset
//! sits in the panel's own header where it can be reached without scrolling
//! past everything it undoes.

use dioxus::prelude::*;
use noctua_design::IconDrawing;
use noctua_design::components::{
    Button, ButtonSize, ButtonTone, ButtonVariant, Heading, HeadingLevel, HeadingSize, Label,
    LabelKind, LabelSize, Segmented, SegmentedAppearance, SegmentedFill, SegmentedItem,
    SegmentedSize, Swatch, SwatchSize, Switch, SwitchSize,
};
use noctua_design_i18n::Key;

use crate::pages::Message;
use crate::shell::Context;

/// How far this menu's settings reach.
///
/// It used to be called `Placement` and decided the surface as well: the
/// header's floated and a frame's was a band inside the frame. That band was
/// 457px tall, which covered the whole stage and — on a 172px frame — put every
/// one of its eight rows outside the frame's `overflow: clip`, unreachable.
/// Both float now, so what is left to decide is only this.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub(crate) enum Reach {
    /// The header's: every setting, including the four only a page can carry.
    #[default]
    Page,
    /// A frame's: only the settings a scope can carry, and no reset — the
    /// frame's toolbar already has one two inches away.
    Scope,
}

/// Every appearance setting, sectioned.
///
/// # The page-only section is absent inline, not disabled
///
/// `syntax` is the one setting left that a scope cannot carry, and it is
/// `scope = false` in `spec/theme.toml` for a reason the emitted selector
/// enforces: its payload is a colour on a descendant, so two of them on one
/// page are settled by source order rather than by proximity. Offering it on a
/// frame would be a control that does nothing, and a control offered where it
/// does nothing is a control that lies.
///
/// The backdrop, the pattern and the grain used to be in that
/// section with it and are not any more — each declares a custom property on
/// the element carrying its attribute, so a frame paints its own through the
/// same rule the page does. Their section is shown in both reaches.
/// # Every id here derives from `name`, and it did not
///
/// `Field` built `id="nd-label-{setting}"` from the setting alone, which is a
/// constant — so the header's menu and every frame's menu on the same page all
/// declared `nd-label-mode`, and each frame's `aria-labelledby` resolved to the
/// header's label rather than to its own. Six ids, twice over on most pages and
/// **seven times** on `/tables`, where there is one frame per component.
///
/// It is the same defect `Search` was rebuilt to fix — an id derived from
/// something that is not unique — and it has the same fix: one required name at
/// the top, every id below built from it, and no caller writing an id at all.
#[component]
pub(crate) fn ThemeMenu(
    context: Context,
    /// What this menu themes, and the root of every id in it: the page, or the
    /// frame it sits in. Required, because a default would be a value two
    /// menus could share, which is the whole defect.
    name: String,
    #[props(default)] reach: Reach,
) -> Element {
    let page = reach == Reach::Page;
    rsx! {
        div { class: "nd-theme-menu",
            // The title and the reset on one line. The reset used to be a
            // full-width button under eleven groups, which is the one place a
            // reader who wants to undo everything cannot see it from.
            div { class: "nd-theme-head",
                Heading { level: HeadingLevel::_2, size: HeadingSize::H4, class: "nd-theme-title",
                    "{context.t(Key::SettingsLabel)}"
                }
                if page {
                    Button {
                        action: "reset",
                        variant: ButtonVariant::Transparent,
                        tone: ButtonTone::Neutral,
                        size: ButtonSize::Small,
                        label: context.t(Key::SettingsReset),
                    }
                }
            }

            Section { context, title: Key::ThemeSectionAppearance,
                // The mode row keeps the icons the header control had. They
                // were its one real advantage: `☀ ☾ ▣` is scannable in a way
                // three words are not, and `SegmentedItem` takes both.
                Field { context, name: name.clone(), setting: "mode", label: Key::ThemeModeLabel,
                    Segmented {
                        name: "mode",
                        labelledby: "{name}-mode-label",
                        size: SegmentedSize::Small,
                        for (value, icon, text) in [
                            ("light", IconDrawing::SUN, Key::ThemeModeLight),
                            ("dark", IconDrawing::MOON, Key::ThemeModeDark),
                            ("system", IconDrawing::MONITOR, Key::ThemeModeSystem),
                        ] {
                            SegmentedItem {
                                value,
                                icon,
                                // Icon-only, as the header's control was. The
                                // row already says "Appearance", so three words
                                // beside three glyphs said it a second time —
                                // and "Match the system" is the widest string
                                // in the panel by half. The label stays the
                                // accessible name.
                                hide_label: true,
                                label: context.t(text),
                                selected: value == crate::defaults::of("mode"),
                            }
                        }
                    }
                }
                Accents { context, name: name.clone() }
                Choice {
                    context,
                    name: name.clone(),
                    setting: "saturation",
                    label: Key::ThemeSaturationLabel,
                    options: vec![
                        ("balanced", Key::ThemeSaturationBalanced),
                        ("vivid", Key::ThemeSaturationVivid),
                        ("sober", Key::ThemeSaturationSober),
                    ],
                }
            }

            Section { context, title: Key::ThemeSectionSurface,
                Choice {
                    context,
                    name: name.clone(),
                    setting: "radius",
                    label: Key::ThemeRadiusLabel,
                    options: vec![
                        ("square", Key::ThemeRadiusSquare),
                        ("soft", Key::ThemeRadiusSoft),
                        ("round", Key::ThemeRadiusRound),
                        ("circular", Key::ThemeRadiusCircular),
                    ],
                }
                Choice {
                    context,
                    name: name.clone(),
                    setting: "glass",
                    label: Key::ThemeGlassLabel,
                    options: vec![
                        ("off", Key::ThemeGlassOff),
                        ("subtle", Key::ThemeGlassSubtle),
                        ("full", Key::ThemeGlassFull),
                    ],
                }
                // WHY `Off` IS REFUSED, SAID WHERE IT IS REFUSED.
                //
                // The glass test needs the material on, so it turns it on and
                // holds it there — and a control that is marked disabled with
                // no reason given is a control that looks broken. Rendered
                // only in a frame's menu, because only a frame has a rig; and
                // shown only while that rig runs, by the stylesheet, because
                // whether it runs is not known when this is rendered.
                if !page {
                    p { class: "nd-note-line nd-glass-held",
                        // A `Message`, so the value it names is a code chip
                        // rather than a word with backticks around it — the
                        // treatment every other sentence on this site gives an
                        // identifier.
                        Message { context, message: Key::ExampleGlassHeld }
                    }
                }
                Choice {
                    context,
                    name: name.clone(),
                    setting: "font",
                    label: Key::ThemeFontLabel,
                    options: vec![("sans", Key::ThemeFontSans), ("mono", Key::ThemeFontMono)],
                }
            }

            // SHOWN IN BOTH REACHES, because all four are `scope = true` now.
            // They were page-only for one reason, and it was structural rather
            // than a judgement about what belongs to a page: each payload sat
            // on a DESCENDANT, so two of them could not be nested and `none`
            // was the absence of a rule rather than a value. Each declares a
            // custom property on the element carrying the attribute now, and a
            // frame paints its own through the same rule the page does.
            //
            // The title is its own key rather than `theme.section.page`, which
            // is the sentence a frame's menu could not have said.
            Section { context, title: Key::ThemeSectionBackground,
                // SIX BACKDROPS, NOT A BLACKOUT SWITCH. Five of them are
                // colours the colour system already solved in both modes,
                // so the row is a choice like every other and not a boolean
                // named after one of its own values.
                Choice {
                    context,
                    name: name.clone(),
                    setting: "backdrop",
                    label: Key::SettingsBackdropLabel,
                    options: vec![
                        ("plain", Key::SettingsBackdropPlain),
                        ("cool", Key::SettingsBackdropCool),
                        ("black", Key::SettingsBackdropBlack),
                        ("tinted", Key::SettingsBackdropTinted),
                        ("soft", Key::SettingsBackdropSoft),
                        ("softer", Key::SettingsBackdropSofter),
                    ],
                }
                Choice {
                    context,
                    name: name.clone(),
                    setting: "pattern",
                    label: Key::SettingsPatternLabel,
                    options: vec![
                        ("none", Key::SettingsPatternNone),
                        ("dots", Key::SettingsPatternDots),
                        ("mesh", Key::SettingsPatternMesh),
                        ("checks", Key::SettingsPatternChecks),
                        ("diagonals", Key::SettingsPatternDiagonals),
                    ],
                }
                Toggle {
                    context,
                    name: name.clone(),
                    setting: "grain",
                    label: Key::SettingsGrainLabel,
                    note: Key::SettingsGrainNote,
                }
            }

            if page {
                Section { context, title: Key::ThemeSectionPage,
                    // THE LAST PAGE-ONLY SETTING, and it is alone here for the
                    // reason its comment in `spec/theme.toml` gives: its
                    // payload is a colour on a DESCENDANT, which has no
                    // nearest-ancestor rule, so two scopes on one page would be
                    // settled by source order rather than by proximity. That
                    // was true of the four above it as well until each one was
                    // given a custom property to declare; this one cannot be,
                    // because seventeen token roles are not one value.
                    //
                    // A subtree that wants its own theme uses the component's
                    // `theme` prop, which lands on the block itself — a better
                    // answer than a scope, not a lesser one.
                    //
                    // NO PREVIEW CHIP. On any page with a code block the page
                    // IS the preview, and the change is instant because the
                    // attribute is on `:root` and the stylesheet is already
                    // loaded. On a page without one, five coloured dots would
                    // be a picture of a decision they cannot demonstrate.
                    Choice {
                        context,
                        name: name.clone(),
                        setting: "syntax",
                        label: Key::SettingsSyntaxLabel,
                        options: vec![
                            ("auto", Key::SettingsSyntaxAuto),
                            ("spectrum", Key::SettingsSyntaxSpectrum),
                            ("duotone", Key::SettingsSyntaxDuotone),
                            ("muted", Key::SettingsSyntaxMuted),
                            ("mono", Key::SettingsSyntaxMono),
                        ],
                    }
                }
            }
        }
    }
}

/// One named group of settings.
///
/// The panel used to be a flat list of eleven with a single `<hr>` in it, and
/// nothing said what the rule separated. It separated the settings a component
/// can carry from the ones only the page can; the heading says so.
#[component]
fn Section(context: Context, title: Key, children: Element) -> Element {
    rsx! {
        section { class: "nd-theme-section", "aria-label": context.t(title),
            p { class: "nd-theme-section-title", "{context.t(title)}" }
            // THE ROWS ARE WRAPPED SO THE TITLE CAN BE CLOSER THAN THEY ARE.
            //
            // With one flex gap for both, a section title sat exactly as far
            // from its first row as two unrelated rows sat from each other —
            // and both gaps were SMALLER than the gap inside a single row,
            // between its name and its own control. So `Corners` and `Glass`
            // read as one control with seven buttons.
            div { class: "nd-theme-rows", {children} }
        }
    }
}

/// One row: its name on one side, its control on the other.
#[component]
fn Field(
    context: Context,
    name: String,
    setting: String,
    label: Key,
    #[props(default)] note: Option<Key>,
    children: Element,
) -> Element {
    // THE ONE LABEL, and this row is what it was competing with. A `<p>` with
    // this class was the settings panel's private word for "the visible name
    // of a group"; it is `Label { kind: Name }` now, and the id it derives is
    // the one the `Segmented` beside it points `aria-labelledby` at.
    let named = format!("{name}-{setting}");
    rsx! {
        div { class: "nd-theme-field",
            div { class: "nd-theme-field-name",
                Label {
                    kind: LabelKind::Name,
                    size: LabelSize::Small,
                    for_control: named,
                    text: context.t(label),
                }
                if let Some(note) = note {
                    p { class: "nd-note-line", "{context.t(note)}" }
                }
            }
            div { class: "nd-theme-field-control", {children} }
        }
    }
}

/// A row whose control is a `Segmented` of two or more values.
#[component]
fn Choice(
    context: Context,
    name: String,
    setting: String,
    label: Key,
    options: Vec<(&'static str, Key)>,
) -> Element {
    let default = crate::defaults::of(&setting);
    let label_id = format!("{name}-{setting}-label");
    rsx! {
        Field { context, name: name.clone(), setting: setting.clone(), label,
            Segmented {
                name: "{setting}",
                labelledby: "{label_id}",
                appearance: SegmentedAppearance::Spread,
                size: SegmentedSize::Small,
                // HUG, NOT STRETCH, AND THE ROWS ARE WHY.
                //
                // `stretch` is a grid of equal columns, which is right for a
                // full-width row and wrong for one sharing its line with a
                // name: `auto-fit` fills the space with as many columns as its
                // minimum allows, so a wide panel gave fourteen 4rem columns
                // and clipped `Arredondados` in every one of them.
                //
                // It existed so "Match the system" would stop dropping onto a
                // ragged line of its own. The mode row is icons now, so the
                // longest string in the panel is gone and the reason with it.
                fill: SegmentedFill::Hug,
                for (value, text) in options {
                    SegmentedItem {
                        value,
                        label: context.t(text),
                        // Rendered from the default rather than left false and
                        // corrected by script: with JavaScript off, every radio
                        // in the panel announced itself as unchecked.
                        selected: value == default,
                    }
                }
            }
        }
    }
}

/// A row whose control is on or off.
///
/// These were `Segmented` groups of two reading "Off" and "On" — a radio group
/// asking a yes-or-no question, taking a full-width row to do it. The note
/// stays, and is now *pointed at* rather than merely adjacent, so it is part of
/// what the control announces.
#[component]
fn Toggle(context: Context, name: String, setting: String, label: Key, note: Key) -> Element {
    // Page-only settings today, so these never collided — and they would the
    // moment one became scopable. The rule is the same rule.
    let note_id = format!("nd-note-{name}-{setting}");
    rsx! {
        div { class: "nd-theme-field nd-theme-field--switch",
            Switch {
                name: "{setting}",
                on: "on",
                off: "off",
                size: SwitchSize::Small,
                describedby: "{note_id}",
                checked: crate::defaults::of(&setting) == "on",
                label: context.t(label),
            }
            p { class: "nd-note-line", id: "{note_id}", "{context.t(note)}" }
        }
    }
}

/// The accent row, with a real swatch of each accent's solid colour.
///
/// The swatch is painted by `--nc-accent-solid` read **inside a `data-palette`
/// scope**, which is the same mechanism `ThemeScope` uses: `dist/scope.css`
/// re-resolves the semantic contract on any element carrying the attribute, so
/// a swatch shows its own accent rather than the page's.
///
/// The stylesheets those scopes need are fetched by the script the first time
/// this panel is opened, and never on a page view that does not open it.
#[component]
fn Accents(context: Context, name: String) -> Element {
    let accents = [
        ("ochre", Key::ThemeAccentOchre),
        ("amber", Key::ThemeAccentAmber),
        ("lime", Key::ThemeAccentLime),
        ("jade", Key::ThemeAccentJade),
        ("teal", Key::ThemeAccentTeal),
        ("azure", Key::ThemeAccentAzure),
        ("blue", Key::ThemeAccentBlue),
        ("indigo", Key::ThemeAccentIndigo),
        ("violet", Key::ThemeAccentViolet),
        ("magenta", Key::ThemeAccentMagenta),
        ("rose", Key::ThemeAccentRose),
        ("clay", Key::ThemeAccentClay),
        ("umber", Key::ThemeAccentUmber),
    ];
    let default = crate::defaults::of("accent");
    let saturation = crate::defaults::of("saturation");
    rsx! {
        Field { context, name: name.clone(), setting: "accent", label: Key::ThemeAccentLabel,
            // NO LAYOUT OF ITS OWN, AND THAT IS THE FIX.
            //
            // This carried `.nd-accents`, a grid of
            // `minmax(var(--noctua-space-6), 1fr)` — a 24px minimum under a
            // chip that `[data-nd-hide-label]` makes a FIXED 32px. Every
            // swatch overhung its 25px track and landed 2.93px inside its
            // neighbour's border, thirteen times.
            //
            // `spread` is `flex-wrap: wrap` with a gap, and thirteen chips of
            // one width wrap into a grid on their own. A flex item cannot
            // overhang a sibling, so the whole failure mode is gone with the
            // rule that caused it.
            Segmented {
                name: "accent",
                labelledby: "{name}-accent-label",
                appearance: SegmentedAppearance::Spread,
                size: SegmentedSize::Small,
                for (value, text) in accents {
                    SegmentedItem {
                        value,
                        // THIRTEEN RADIOS THAT ALL SAID `aria-checked="false"`.
                        //
                        // This was hand-written markup and it hardcoded the
                        // state on every one of them, so with the script
                        // blocked the group announced that none of thirteen
                        // accents was in force. A component cannot make that
                        // mistake: the state is a prop.
                        selected: value == default,
                        hide_label: true,
                        label: context.t(text),
                        Swatch { palette: "{value}-{saturation}", size: SwatchSize::Small }
                    }
                }
            }
        }
    }
}
