//! The frame every page sits in: header, sidebar, scrim, settings, effects.
//!
//! # Three structural rules, each from something that breaks silently
//!
//! **The effect layers are siblings of the shell, never ancestors.**
//! `backdrop-filter` stops sampling the moment any *ancestor* has `filter`,
//! `opacity` below 1, `mask` or a `backdrop-filter` of its own. Wrapping the
//! shell in a faded layer would switch every frosted surface on the page to a
//! flat fill, with no error and no obvious cause.
//!
//! **The scrim never covers the drawer.** It is anchored by two insets rather
//! than given a width, so its inline start edge *is* the drawer's inline end
//! edge — the same custom property, resolved against the same containing block.
//! A `calc(100vw - …)` width rounds independently and leaves a one-pixel dim
//! line along the drawer's edge, which is exactly the artefact that ruins glass.
//!
//! **Nothing here is modal.** No focus trap, no `inert`, no `aria-modal`.
//! Widening the viewport past `lg` docks the sidebar, hides the scrim and
//! releases the scroll lock in CSS alone — there is no listener to miss and no
//! state to unwind.

use dioxus::prelude::*;
use noctua_design::{Icon, IconName};
use noctua_design_i18n::{Key, Locale};

use crate::site::{self, Page};

/// Everything a page needs to know about where it sits.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct Context {
    /// The locale being built.
    pub locale: Locale,
    /// The page being built.
    pub page: Page,
}

impl Context {
    /// This locale's text for a message.
    #[must_use]
    pub(crate) fn t(self, key: Key) -> &'static str {
        self.locale.message(key)
    }
}

/// The sticky header. One line at 320 px, and the budget is written down.
///
/// The mode control is three icons, not one that cycles. A cycling button has
/// to show the *current* state to be usable, and this one showed a single
/// static glyph that never changed — so a reader could not tell which of three
/// modes was in force, and one of the three transitions (dark → system on a
/// dark system) changes nothing on the page at all. Three radios always move
/// the indicator, whatever the pixels behind them do.
#[component]
pub(crate) fn Header(context: Context) -> Element {
    // The first entry in THIS locale's sidebar, not a fixed page: the order is
    // the reader's alphabet, so the front door differs by locale and reading it
    // from the same function the sidebar uses is what keeps the two agreeing.
    let home = site::first(context.locale).href();
    rsx! {
        // The header's frost, as a SIBLING painted behind it. Inside the header
        // it would be an ancestor of the settings panel, and a nested
        // `backdrop-filter` stops sampling — see shell.css.
        div { class: "nd-header-glass noctua-glass", "aria-hidden": "true" }
        header { class: "nd-header",
            button {
                class: "nd-icon nd-menu",
                r#type: "button",
                "aria-expanded": "false",
                "aria-controls": "nd-nav",
                "aria-label": context.t(Key::NavOpen),
                "data-nd-menu": "",
                "data-nd-open": context.t(Key::NavOpen),
                "data-nd-close": context.t(Key::NavClose),
                Icon { icon: IconName::Menu }
            }
            a { class: "nd-wordmark", href: "{home}", "{context.t(Key::AppName)}" }
            div { class: "nd-header-controls",
                ModeControl { context }
                a {
                    class: "nd-icon",
                    href: "https://github.com/noctua-world/noctua-design",
                    rel: "noreferrer",
                    "aria-label": context.t(Key::AppRepository),
                    title: "{context.t(Key::AppRepository)}",
                    Icon { icon: IconName::Github }
                }
                LocaleSwitch { context }
                Settings { context }
            }
        }
    }
}

/// Light, dark and system, side by side, one of them chosen.
///
/// A `role="radiogroup"` with the same `data-nd-setting` hook the settings
/// panel uses, so the existing roving-tabindex handling and the existing
/// persistence apply with no new script: one tab stop, arrows move within.
#[component]
fn ModeControl(context: Context) -> Element {
    let options = [
        ("light", IconName::Sun, Key::ThemeModeLight),
        ("dark", IconName::Moon, Key::ThemeModeDark),
        ("system", IconName::Monitor, Key::ThemeModeSystem),
    ];
    rsx! {
        div {
            class: "nd-segmented",
            "data-nd-setting": "mode",
            role: "radiogroup",
            "aria-label": context.t(Key::ThemeModeLabel),
            for (value, icon, label) in options {
                button {
                    class: "nd-segment",
                    r#type: "button",
                    role: "radio",
                    // Server-rendered from the known default, so the control is
                    // not announcing that none of three is selected before the
                    // script runs.
                    "aria-checked": if crate::defaults::of("mode") == value { "true" } else { "false" },
                    "data-nd-value": "{value}",
                    // The icon is decoration; the button carries the name.
                    "aria-label": context.t(label),
                    title: "{context.t(label)}",
                    Icon { icon }
                }
            }
        }
    }
}

/// The language control.
///
/// A link rather than a menu: with two locales offered, a control that toggles
/// is one tap instead of two, and it needs no floating surface. It is also a
/// real `<a href>`, so it works with JavaScript off — the whole site is static
/// files, and there is nothing here a script has to reconstruct.
#[component]
fn LocaleSwitch(context: Context) -> Element {
    let other = theme_other_locale(context.locale);
    let href = context.page.locale_href(other);
    rsx! {
        a {
            class: "nd-locale",
            href: "{href}",
            lang: "{other.bcp47()}",
            "aria-label": context.t(Key::LocaleLabel),
            title: "{context.t(Key::LocaleLabel)}",
            // Two spellings, one shown at a time. Below `sm` the header's width
            // budget does not stretch to "Português", and a truncated language
            // name is worse than a tag: the reader cannot tell what they would
            // be switching to.
            span { class: "nd-locale-full", "{other.name()}" }
            span { class: "nd-locale-short", "{other.tag().to_uppercase()}" }
        }
    }
}

/// The next locale a reader would switch to.
fn theme_other_locale(current: Locale) -> Locale {
    let offered = Locale::OFFERED;
    let index = offered.iter().position(|l| *l == current).unwrap_or(0);
    offered[(index + 1) % offered.len()]
}

/// The appearance panel: a disclosure, not a dialog.
///
/// `<details>`/`<summary>` rather than the `popover` attribute, because the top
/// layer is always `position: fixed` and cannot anchor to a sticky header
/// without being repositioned on every scroll frame. Focus is not moved on open
/// and nothing is trapped — a reader who tabs past it simply carries on into the
/// page, which is the correct behaviour for something that is not a dialog.
///
/// Its open state is deliberately never stored. Restoring it open would reflow
/// the top of the page on every single load.
#[component]
fn Settings(context: Context) -> Element {
    rsx! {
        details { class: "nd-settings", "data-nd-settings": "",
            summary {
                class: "nd-icon",
                "aria-label": context.t(Key::SettingsLabel),
                title: "{context.t(Key::SettingsLabel)}",
                Icon { icon: IconName::Settings }
            }
            div { class: "nd-panel noctua-glass",
              div { class: "nd-panel-body",

                Choice {
                    context,
                    setting: "mode",
                    label: Key::ThemeModeLabel,
                    options: vec![
                        ("light", Key::ThemeModeLight),
                        ("dark", Key::ThemeModeDark),
                        ("system", Key::ThemeModeSystem),
                    ],
                }
                Accents { context }
                Choice {
                    context,
                    setting: "saturation",
                    label: Key::ThemeSaturationLabel,
                    options: vec![
                        ("balanced", Key::ThemeSaturationBalanced),
                        ("vivid", Key::ThemeSaturationVivid),
                        ("sober", Key::ThemeSaturationSober),
                    ],
                }
                Choice {
                    context,
                    setting: "font",
                    label: Key::ThemeFontLabel,
                    options: vec![("sans", Key::ThemeFontSans), ("mono", Key::ThemeFontMono)],
                }
                Choice {
                    context,
                    setting: "glass",
                    label: Key::ThemeGlassLabel,
                    options: vec![
                        ("off", Key::ThemeGlassOff),
                        ("subtle", Key::ThemeGlassSubtle),
                        ("full", Key::ThemeGlassFull),
                    ],
                }
                Choice {
                    context,
                    setting: "radius",
                    label: Key::ThemeRadiusLabel,
                    options: vec![
                        ("square", Key::ThemeRadiusSquare),
                        ("soft", Key::ThemeRadiusSoft),
                        ("round", Key::ThemeRadiusRound),
                        ("circular", Key::ThemeRadiusCircular),
                    ],
                }

                hr { class: "nd-rule" }

                Choice {
                    context,
                    setting: "blackout",
                    label: Key::SettingsBlackoutLabel,
                    note: Key::SettingsBlackoutNote,
                    options: vec![("off", Key::SettingsOff), ("on", Key::SettingsOn)],
                }
                Choice {
                    context,
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
                Choice {
                    context,
                    setting: "glow",
                    label: Key::SettingsGlowLabel,
                    note: Key::SettingsGlowNote,
                    options: vec![("off", Key::SettingsOff), ("on", Key::SettingsOn)],
                }
                Choice {
                    context,
                    setting: "grain",
                    label: Key::SettingsGrainLabel,
                    note: Key::SettingsGrainNote,
                    options: vec![("off", Key::SettingsOff), ("on", Key::SettingsOn)],
                }

                button { class: "nd-reset", r#type: "button", "data-nd-reset": "",
                    "{context.t(Key::SettingsReset)}"
                }
              }
            }
        }
    }
}

/// One setting, as a row of buttons.
///
/// `role="radiogroup"` rather than real radios: the value is stored in
/// `localStorage` and applied as a DOM attribute, so there is no form to submit
/// and a `<fieldset>` of inputs would only add a name nothing reads.
///
/// `data-nd-setting` and `data-nd-value` are what the script binds to, so all
/// twelve settings are handled by one listener that knows nothing about any of
/// them beyond the generated model.
#[component]
fn Choice(
    context: Context,
    setting: String,
    label: Key,
    #[props(default)] note: Option<Key>,
    options: Vec<(&'static str, Key)>,
) -> Element {
    let default = crate::defaults::of(&setting);
    rsx! {
        div { class: "nd-choice", "data-nd-setting": "{setting}",
            p { class: "nd-choice-label", id: "nd-label-{setting}", "{context.t(label)}" }
            if let Some(note) = note {
                p { class: "nd-choice-note", "{context.t(note)}" }
            }
            div { class: "nd-options", role: "radiogroup", "aria-labelledby": "nd-label-{setting}",
                for (value, text) in options {
                    button {
                        class: "nd-option",
                        r#type: "button",
                        role: "radio",
                        // Rendered from the default rather than left false and
                        // corrected by script: with JavaScript off, every
                        // radio in the panel announced itself as unchecked.
                        "aria-checked": if value == default { "true" } else { "false" },
                        "data-nd-value": "{value}",
                        "{context.t(text)}"
                    }
                }
            }
        }
    }
}

/// The accent row, with a real swatch of each accent's solid colour.
///
/// The swatch is painted by `--nc-accent-solid` read **inside a `data-palette`
/// scope**, which is the same mechanism `ThemeScope` uses: `dist/scope.css`
/// re-resolves the semantic contract on any element carrying the attribute, so a
/// swatch shows its own accent rather than the page's.
///
/// The stylesheets those scopes need are fetched by the script the first time
/// this panel is opened, and never on a page view that does not open it.
#[component]
fn Accents(context: Context) -> Element {
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
    rsx! {
        div { class: "nd-choice", "data-nd-setting": "accent",
            p { class: "nd-choice-label", id: "nd-label-accent", "{context.t(Key::ThemeAccentLabel)}" }
            div { class: "nd-accents", role: "radiogroup", "aria-labelledby": "nd-label-accent",
                for (value, text) in accents {
                    button {
                        class: "nd-accent",
                        r#type: "button",
                        role: "radio",
                        "aria-checked": "false",
                        "data-nd-value": "{value}",
                        title: "{context.t(text)}",
                        span { class: "nd-swatch", "data-nd-swatch": "{value}", "aria-hidden": "true" }
                        span { class: "nd-accent-name", "{context.t(text)}" }
                    }
                }
            }
        }
    }
}

/// The sidebar, and the search that filters it.
///
/// Alphabetical by the **translated** title (see [`site::sorted`]), no
/// categories and no icons. The hover effect targets an inner `<span>` rather
/// than the row, which makes a row background structurally impossible rather
/// than merely discouraged, and only `color` and `text-decoration` change —
/// never `font-weight`, which reflows the line under the pointer.
#[component]
pub(crate) fn Sidebar(context: Context) -> Element {
    rsx! {
        nav { id: "nd-nav", class: "nd-nav noctua-glass", "aria-label": context.t(Key::NavLabel),
            // The wordmark lives here below `sm`, where the header cannot afford
            // it. It is not a duplicate: exactly one of the two is ever shown.
            a { class: "nd-nav-wordmark", href: "{site::first(context.locale).href()}",
                "{context.t(Key::AppName)}"
            }
            div { class: "nd-search",
                label { class: "nd-visually-hidden", r#for: "nd-search-input",
                    "{context.t(Key::SearchLabel)}"
                }
                input {
                    id: "nd-search-input",
                    class: "nd-search-input",
                    r#type: "search",
                    autocomplete: "off",
                    spellcheck: "false",
                    placeholder: "{context.t(Key::SearchPlaceholder)}",
                    "data-nd-search": "",
                }
            }
            ul { class: "nd-nav-list",
                for page in site::sorted(context.locale) {
                    li {
                        a {
                            class: if page == context.page { "nd-nav-link nd-nav-link--current" } else { "nd-nav-link" },
                            href: "{page.href()}",
                            "aria-current": if page == context.page { "page" },
                            "data-nd-index": "{site::collate(context.t(page.title))}",
                            span { class: "nd-nav-text", "{context.t(page.title)}" }
                        }
                    }
                }
            }
            // The system's own names, hidden until the reader types. An index
            // over pages alone answers "where is the colour page" and not "what
            // is --noctua-space-gap", which is the question a developer with a
            // half-remembered token name actually has.
            //
            // Built from the token crate and the theme model, so a token added
            // to spec/ is searchable without this list being edited.
            div { class: "nd-found", "data-nd-found": "", hidden: true,
                p { class: "nd-found-label", "{context.t(Key::SearchLabel)}" }
                for entry in crate::index::all() {
                    a {
                        class: "nd-found-entry",
                        href: "{entry.page.href()}",
                        "data-nd-index": "{entry.name}",
                        hidden: true,
                        "{entry.name}"
                    }
                }
            }
            p { class: "nd-search-empty", "data-nd-search-empty": "", hidden: true,
                "{context.t(Key::SearchEmpty)}"
            }
            // Announced, not merely shown. Without a live region a screen-reader
            // user types into the box and is told nothing at all.
            p {
                class: "nd-visually-hidden",
                "data-nd-search-status": "",
                "data-nd-template": context.t(Key::SearchResults),
                role: "status",
                "aria-live": "polite",
            }
            // No tagline. A sentence describing the system to a reader already
            // inside its documentation, repeated at the foot of every page, is
            // decoration — and `NOCTUA.md §9.7` says nothing decorative.
        }
    }
}

/// The dimming layer, which stops exactly where the drawer starts.
///
/// A plain `<button>`: it is a control that closes the menu, and making it one
/// means a keyboard reaches it, a screen reader announces it, and `Enter`
/// works — none of which is true of a `<div>` with a click handler.
///
/// It carried `tabindex="-1"`, which took all three away again while the
/// comment went on claiming them. It does not any more.
#[component]
pub(crate) fn Scrim(context: Context) -> Element {
    rsx! {
        button {
            class: "nd-scrim",
            r#type: "button",
            "aria-label": context.t(Key::NavDismiss),
            "data-nd-scrim": "",
        }
    }
}

/// The pattern, aurora and grain layers.
///
/// Siblings of everything, ancestors of nothing. Each is a fixed, inert box
/// that paints and takes no part in layout, which is what lets a frosted
/// surface above it keep sampling.
///
/// The class names are the colour layer's, not this site's: `dist/theme.css`
/// binds every pattern to `.noctua-fx` and the grain to `.noctua-grain`. Any
/// application gets the same effects by putting these two elements on the page
/// — there is nothing site-specific about them.
#[component]
pub(crate) fn Effects() -> Element {
    rsx! {
        div { class: "nd-effects", "aria-hidden": "true",
            div { class: "noctua-fx" }
            div { class: "noctua-grain" }
        }
    }
}
