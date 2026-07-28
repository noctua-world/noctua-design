//! The page bodies.
//!
//! Prose carries the measure, not the layout. `max-inline-size` lives on the
//! paragraph, never on `<main>` — the page is as wide as the screen and the
//! *line* is not (`NOCTUA.md §9.7`), which is also what leaves the empty column
//! the effect layers show through.

use dioxus::prelude::*;
use noctua_design::ThemeScope;
use noctua_design::model::{Glass, Mode, Palette, Radius};
use noctua_design_i18n::Key;

use crate::shell::Context;
use crate::site::PAGES;

/// Dispatches to the body for this page.
#[component]
pub(crate) fn Body(context: Context) -> Element {
    rsx! {
        h1 { class: "nd-title", "{context.t(context.page.title)}" }
        p { class: "nd-lead", "{context.t(context.page.summary)}" }
        match context.page.slug {
            "scoped-themes" => rsx! { ScopedThemes { context } },
            "colours" => rsx! { Colours { context } },
            "typography" => rsx! { Typography { context } },
            "spacing" => rsx! { Spacing { context } },
            "effects" => rsx! { Effects { context } },
            "components" => rsx! { Components { context } },
            _ => rsx! { Overview { context } },
        }
    }
}

// --- The page the whole design is for --------------------------------------

/// A page themed three ways at once.
///
/// The outer box and the box inside it each carry their own controls. Nothing
/// about them is a screenshot: they are the same markup as the page around them,
/// reading whichever theme reaches them.
#[component]
fn ScopedThemes(context: Context) -> Element {
    rsx! {
        p { class: "nd-prose", "{context.t(Key::ScopePageBody)}" }

        ThemeScope {
            class: "nd-scope nd-glass",
            palette: Palette::AzureVivid,
            mode: Mode::Light,
            glass: Glass::Full,
            radius: Radius::Round,

            div { class: "nd-scope-head",
                h2 { class: "nd-scope-title", "{context.t(Key::ScopeComponentTitle)}" }
                ScopeControls { context }
            }
            p { class: "nd-prose", "{context.t(Key::ScopeComponentBody)}" }
            Sample { context }

            ThemeScope {
                class: "nd-scope nd-scope--nested nd-glass",
                palette: Palette::RoseSober,
                mode: Mode::Dark,
                radius: Radius::Circular,

                div { class: "nd-scope-head",
                    h3 { class: "nd-scope-title", "{context.t(Key::ScopeNestedTitle)}" }
                    ScopeControls { context }
                }
                p { class: "nd-prose", "{context.t(Key::ScopeNestedBody)}" }
                Sample { context }
            }
        }

        p { class: "nd-prose nd-note", "{context.t(Key::ScopeNote)}" }
    }
}

/// The controls a scope carries, which look exactly like the page's.
///
/// They are bound to the *scope element*, which the script finds by walking up
/// from the button. So the same markup drives the page, a component and a
/// component inside that one, with no per-instance wiring.
#[component]
fn ScopeControls(context: Context) -> Element {
    rsx! {
        div { class: "nd-scope-controls",
            button {
                class: "nd-icon nd-icon--small",
                r#type: "button",
                "data-nd-scope-cycle": "mode",
                "aria-label": context.t(Key::ThemeModeLabel),
                title: "{context.t(Key::ThemeModeLabel)}",
                crate::shell::Icon { name: "mode" }
            }
            button {
                class: "nd-icon nd-icon--small",
                r#type: "button",
                "data-nd-scope-cycle": "accent",
                "aria-label": context.t(Key::ThemeAccentLabel),
                title: "{context.t(Key::ThemeAccentLabel)}",
                span { class: "nd-swatch nd-swatch--live", "aria-hidden": "true" }
            }
            button {
                class: "nd-icon nd-icon--small",
                r#type: "button",
                "data-nd-scope-cycle": "radius",
                "aria-label": context.t(Key::ThemeRadiusLabel),
                title: "{context.t(Key::ThemeRadiusLabel)}",
                span { class: "nd-radius-mark", "aria-hidden": "true" }
            }
            button {
                class: "nd-icon nd-icon--small",
                r#type: "button",
                "data-nd-scope-cycle": "glass",
                "aria-label": context.t(Key::ThemeGlassLabel),
                title: "{context.t(Key::ThemeGlassLabel)}",
                span { class: "nd-glass-mark", "aria-hidden": "true" }
            }
        }
    }
}

/// A heading, a paragraph, a link and two controls.
///
/// Every one of them is painted from the semantic contract — `--nc-color-fg`,
/// `--nc-color-accent`, `--nc-color-border` — so what it shows is whatever theme
/// resolved at the box it happens to be in. That is the assertion the page makes
/// visible: without `dist/scope.css` these would all be the page's colours.
#[component]
fn Sample(context: Context) -> Element {
    rsx! {
        div { class: "nd-sample",
            h4 { class: "nd-sample-heading", "{context.t(Key::ScopeSampleHeading)}" }
            p { class: "nd-prose",
                "{context.t(Key::ScopeSampleBody)} "
                a { class: "nd-link", href: "#", "{context.t(Key::ScopeSampleLink)}" }
            }
            div { class: "nd-sample-actions",
                button { class: "nd-button", r#type: "button", "{context.t(Key::ScopeSampleAction)}" }
                button { class: "nd-button nd-button--quiet", r#type: "button",
                    "{context.t(Key::ScopeSampleQuiet)}"
                }
            }
        }
    }
}

// --- The rest ---------------------------------------------------------------

#[component]
fn Overview(context: Context) -> Element {
    rsx! {
        p { class: "nd-prose", "{context.t(Key::ScopePageBody)}" }
        ul { class: "nd-summary",
            for page in PAGES.iter().skip(1) {
                li {
                    a { class: "nd-link", href: "{page.href()}", "{context.t(page.title)}" }
                    p { class: "nd-prose", "{context.t(page.summary)}" }
                }
            }
        }
        Frame { context, caption: Key::PageScopedThemesTitle,
            ThemeScope { class: "nd-scope nd-glass", palette: Palette::JadeBalanced,
                Sample { context }
            }
        }
    }
}

/// Every palette, each swatch showing its own colours.
///
/// This is the one page that legitimately links all thirty-nine stylesheets: it
/// is what the page is for. Everywhere else they are fetched on demand.
#[component]
fn Colours(context: Context) -> Element {
    rsx! {
        div { class: "nd-palette-grid", "data-nd-all-palettes": "",
            for palette in Palette::ALL {
                div { class: "nd-palette", "data-palette": palette.as_str(),
                    span { class: "nd-palette-name", "{palette.as_str()}" }
                    div { class: "nd-palette-bar",
                        span { class: "nd-chip nd-chip--bg" }
                        span { class: "nd-chip nd-chip--subtle" }
                        span { class: "nd-chip nd-chip--border" }
                        span { class: "nd-chip nd-chip--accent" }
                        span { class: "nd-chip nd-chip--fg" }
                    }
                }
            }
        }
        p { class: "nd-prose nd-note", "{context.t(Key::ScopeNote)}" }
    }
}

#[component]
fn Typography(context: Context) -> Element {
    // The token *references*, taken from the generated crate rather than a list
    // typed out here. A second list would carry the literal values — `3rem`,
    // `1.125rem` — which are exactly the numbers that go stale the first time
    // the scale moves, and which a developer copying from this page does not
    // want anyway: what they want is the name.
    use noctua_design_tokens::size;
    let sizes = [
        ("display", size::DISPLAY),
        ("h1", size::H1),
        ("h2", size::H2),
        ("h3", size::H3),
        ("h4", size::H4),
        ("lead", size::LEAD),
        ("body", size::BODY),
        ("small", size::SMALL),
        ("caption", size::CAPTION),
    ];
    rsx! {
        div { class: "nd-scale",
            for (name, reference) in sizes {
                div { class: "nd-scale-row",
                    span { class: "nd-scale-name", "{name}" }
                    span { class: "nd-scale-value", "{reference}" }
                    p { class: "nd-scale-sample", style: "font-size: {reference}",
                        "{context.t(Key::ScopeSampleHeading)}"
                    }
                }
            }
        }
        p { class: "nd-prose nd-lead-measure", "{context.t(Key::ScopeSampleBody)}" }
    }
}

#[component]
fn Spacing(context: Context) -> Element {
    let steps = [1_u32, 2, 3, 4, 6, 8, 12, 16, 24, 32];
    rsx! {
        div { class: "nd-scale",
            for step in steps {
                div { class: "nd-scale-row",
                    span { class: "nd-scale-name", "space-{step}" }
                    span { class: "nd-bar", style: "inline-size: var(--noctua-space-{step})" }
                }
            }
        }
        p { class: "nd-prose", "{context.t(Key::PageSpacingSummary)}" }
    }
}

#[component]
fn Effects(context: Context) -> Element {
    rsx! {
        p { class: "nd-prose", "{context.t(Key::SettingsGlowNote)}" }
        p { class: "nd-prose", "{context.t(Key::SettingsGrainNote)}" }
        p { class: "nd-prose", "{context.t(Key::SettingsAnimateNote)}" }
        Frame { context, caption: Key::PageEffectsTitle,
            div { class: "nd-glass nd-scope",
                Sample { context }
            }
        }
    }
}

#[component]
fn Components(context: Context) -> Element {
    rsx! {
        p { class: "nd-prose", "{context.t(Key::ProvisionalBody)}" }
        Frame { context, caption: Key::PageComponentsTitle,
            ThemeScope { class: "nd-scope nd-glass", palette: Palette::VioletVivid, mode: Mode::Dark,
                Sample { context }
            }
        }
    }
}

// --- The example frame ------------------------------------------------------

/// The container every example is shown in.
///
/// Viewport width, a code panel and a reset, built once so that a future example
/// — a 3D scene, a data grid — is a *use* of this rather than a new thing.
///
/// The width control resizes the mount point and nothing else, so an example can
/// be judged at 320 px without the reader resizing the browser.
#[component]
fn Frame(context: Context, caption: Key, children: Element) -> Element {
    rsx! {
        figure { class: "nd-frame", "data-nd-frame": "",
            div { class: "nd-frame-bar",
                span { class: "nd-frame-label", "{context.t(Key::ExampleLabel)}" }
                div { class: "nd-frame-controls",
                    for (value, key) in [
                        ("narrow", Key::ExampleWidthNarrow),
                        ("medium", Key::ExampleWidthMedium),
                        ("full", Key::ExampleWidthFull),
                    ] {
                        button {
                            class: "nd-option",
                            r#type: "button",
                            role: "radio",
                            "aria-checked": if value == "full" { "true" } else { "false" },
                            "data-nd-frame-width": "{value}",
                            "{context.t(key)}"
                        }
                    }
                    button { class: "nd-option", r#type: "button", "data-nd-frame-reset": "",
                        "{context.t(Key::ExampleReset)}"
                    }
                }
            }
            div { class: "nd-frame-stage", "data-nd-frame-stage": "", {children} }
            figcaption { class: "nd-frame-caption", "{context.t(caption)}" }
        }
    }
}
