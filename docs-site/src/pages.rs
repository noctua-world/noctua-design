//! The page bodies.
//!
//! Prose carries the measure, not the layout. `max-inline-size` lives on the
//! paragraph, never on `<main>` — the page is as wide as the screen and the
//! *line* is not (`NOCTUA.md §9.7`), which is also what leaves the empty column
//! the effect layers show through.

use dioxus::prelude::*;
use noctua_design::ThemeScope;
use noctua_design::components::{
    Abbr, Blockquote, Code, CodeBlock, CodeTone, Del, Em, Heading, HeadingLevel, HeadingSize, Ins,
    Kbd, Link, List, ListItem, ListSpacing, Mark, Paragraph, ParagraphTone, Strong, Text, TextSize,
    TextTone, TextWeight, Time, Truncate, VisuallyHidden,
};
use noctua_design::model::{Glass, Mode, Palette, Radius};
use noctua_design::{Icon, IconName, IconSize};
use noctua_design_i18n::Key;

use crate::shell::Context;

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
            "icons" => rsx! { Icons { context } },
            "components" => rsx! { Components { context } },
            // No catch-all body. A page in `site::PAGES` with nothing to render
            // is a build failure, not a page that quietly shows something else:
            // the site is generated, so this panics during the build and never
            // reaches a reader.
            slug => unreachable!("no body for the page `{slug}`"),
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
        div { class: "nd-scope-head",
            Heading { level: HeadingLevel::_2, size: HeadingSize::H3, "{context.t(Key::ScopePageTitle)}" }
            button { class: "nd-option", r#type: "button", "data-nd-scope-reset": "",
                "{context.t(Key::ScopeReset)}"
            }
        }
        Paragraph { "{context.t(Key::ScopePageBody)}" }

        ThemeScope {
            class: "nd-scope",
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
                class: "nd-scope nd-scope--nested",
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
                noctua_design::Icon { icon: noctua_design::IconName::Monitor }
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

/// Every icon in the system, listed by the system itself.
///
/// The page loops `IconName::ALL`, so adding an icon to `spec/icons.toml`
/// documents it with no edit here. That is the user-facing half of the reason
/// the set is generated: a hand-maintained gallery is a list that goes stale,
/// and a stale gallery of icons is indistinguishable from a missing icon —
/// which is exactly the failure this whole layer replaced.
#[component]
fn Icons(context: Context) -> Element {
    // Filled here rather than inside `rsx!`: the placeholder is literally
    // `{count}`, and the macro would read it as an interpolation of its own.
    let counted = context
        .t(Key::IconsCount)
        .replace("{count}", &IconName::ALL.len().to_string());
    rsx! {
        Paragraph { "{context.t(Key::IconsBody)}" }

        p { class: "nd-note nd-prose", "{counted}" }

        ul { class: "nd-icon-grid",
            for icon in IconName::ALL {
                li { class: "nd-icon-cell",
                    // Unlabelled: the name beside it is the accessible name, and
                    // an icon that repeats the text next to it is noise to a
                    // screen reader rather than help.
                    Icon { icon, size: IconSize::Large }
                    Code { "{icon.as_str()}" }
                }
            }
        }
    }
}

/// Every palette, each swatch reading its own colours.
///
/// This is the one page that links all thirty-nine stylesheets — about 650 KB
/// gzipped, which is why no other page does. Each cell is a `data-palette`
/// scope, so `dist/scope.css` re-resolves the semantic contract inside it and
/// every chip shows that palette rather than the reader's.
#[component]
fn Colours(context: Context) -> Element {
    rsx! {
        Heading { level: HeadingLevel::_2, size: HeadingSize::H3, "{context.t(Key::ColoursRoles)}" }
        Paragraph { "{context.t(Key::ColoursRolesBody)}" }

        div { class: "nd-palette-grid", "data-nd-all-palettes": "",
            for palette in Palette::ALL {
                div { class: "nd-palette", "data-palette": palette.as_str(),
                    Code { class: "nd-palette-name", tone: CodeTone::Subtle, "{palette.as_str()}" }
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
        Paragraph { tone: ParagraphTone::Muted, "{context.t(Key::ColoursNote)}" }
    }
}

/// The type scale, the four families, and the fourteen scripts.
///
/// The scripts are not decoration: `NOCTUA.md §7.3.1` is the reason Noto was
/// chosen over any other family, and this page is where that reason is either
/// visible or false. A browser spec asserts every one renders with no tofu.
#[component]
fn Typography(context: Context) -> Element {
    rsx! {
        Heading { level: HeadingLevel::_2, size: HeadingSize::H2, "{context.t(Key::TypeScaleTitle)}" }
        Paragraph { class: "nd-prose", "{context.t(Key::TypeScaleBody)}" }
        Scale { context }

        Heading { level: HeadingLevel::_2, size: HeadingSize::H2, "{context.t(Key::TypeFamiliesTitle)}" }
        Paragraph { class: "nd-prose", "{context.t(Key::TypeFamiliesBody)}" }
        Families { context }

        Heading { level: HeadingLevel::_2, size: HeadingSize::H2, "{context.t(Key::TypeScriptsTitle)}" }
        Paragraph { class: "nd-prose", "{context.t(Key::TypeScriptsBody)}" }
        Scripts { context }
    }
}

/// Every size, at its real size, with its token name.
#[component]
fn Scale(context: Context) -> Element {
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
                    Code { class: "nd-scale-name", "{name}" }
                    p { class: "nd-scale-sample", style: "font-size: {reference}",
                        "{context.t(Key::TypeSpecimenLine)}"
                    }
                }
            }
        }
    }
}

/// One specimen per family, across the weights and styles that ship.
#[component]
fn Families(context: Context) -> Element {
    // The family NAMES are labels a reader reads, not font bindings — every
    // specimen below is set with the token beside it. A specimen page that
    // could not name the family it is showing would be useless.
    let families = [
        ("Noto Sans", "var(--noctua-font-sans)"), // allow-literal: a label, not a binding
        ("Noto Serif", "var(--noctua-font-serif)"), // allow-literal: as above
        ("Noto Sans Display", "var(--noctua-font-display)"), // allow-literal: as above
        ("Noctua Iosevka", "var(--noctua-font-mono)"), // allow-literal: as above
    ];
    let weights = [
        ("regular", "400"),
        ("medium", "500"),
        ("semibold", "600"),
        ("bold", "700"),
    ];
    rsx! {
        div { class: "nd-specimens",
            for (family, stack) in families {
                div { class: "nd-specimen",
                    Text { class: "nd-specimen-name", weight: TextWeight::Semibold, "{family}" }
                    for (label, weight) in weights {
                        p {
                            class: "nd-specimen-line",
                            style: "font-family: {stack}; font-weight: {weight}", // allow-literal: `stack` is one of the four font tokens in the table above — the gate cannot see through an interpolation, and a specimen page must set the family it is showing
                            Text { class: "nd-specimen-weight", size: TextSize::Caption,
                                tone: TextTone::Muted, "{label}"
                            }
                            "{context.t(Key::TypeSpecimenLine)}"
                        }
                    }
                    p {
                        class: "nd-specimen-line",
                        style: "font-family: {stack}; font-style: italic", // allow-literal: as above, the italic specimen
                        Text { class: "nd-specimen-weight", size: TextSize::Caption,
                            tone: TextTone::Muted, "italic"
                        }
                        "{context.t(Key::TypeSpecimenLine)}"
                    }
                }
            }
        }
    }
}

/// The fourteen scripts, each with what it proves.
///
/// Every row carries `lang` and `dir`, because a browser shapes Arabic
/// correctly only when it is told the text is Arabic — and because a test that
/// rendered the right glyphs from the wrong language tag would be proving
/// something other than what it claims.
#[component]
fn Scripts(context: Context) -> Element {
    let _ = context;
    rsx! {
        div { class: "nd-scripts", "data-nd-scripts": "",
            for sample in crate::scripts::SAMPLES {
                div { class: "nd-script", "data-nd-script": "{sample.name}",
                    div { class: "nd-script-meta",
                        Text { class: "nd-script-name", weight: TextWeight::Semibold, "{sample.name}" }
                        Text { size: TextSize::Caption, tone: TextTone::Muted, "{sample.proves}" }
                    }
                    p {
                        class: "nd-script-sample",
                        lang: "{sample.lang}",
                        dir: "{sample.dir}",
                        "data-nd-monospaceable": "{sample.monospaceable}",
                        "{sample.text}"
                    }
                }
            }
        }
    }
}

/// The named gaps, then the raw scale.
///
/// The named tokens come first on purpose: they are what a component actually
/// asks for, and the search index points every one of them at this page.
#[component]
fn Spacing(context: Context) -> Element {
    let named = [
        ("tight", Key::SpacingTight),
        ("inset", Key::SpacingInset),
        ("gap", Key::SpacingGap),
        ("paragraph", Key::SpacingParagraph),
        ("heading", Key::SpacingHeading),
        ("section", Key::SpacingSection),
        ("region", Key::SpacingRegion),
    ];
    let steps = [1_u32, 2, 3, 4, 6, 8, 12, 16, 24, 32, 48, 64];
    rsx! {
        Heading { level: HeadingLevel::_2, size: HeadingSize::H3, "{context.t(Key::SpacingNamedTitle)}" }
        Paragraph { "{context.t(Key::SpacingNamedBody)}" }
        div { class: "nd-scale",
            for (name, purpose) in named {
                div { class: "nd-scale-row",
                    Code { class: "nd-scale-name", "--noctua-space-{name}" }
                    span { class: "nd-bar", style: "inline-size: var(--noctua-space-{name})" }
                    Text { size: TextSize::Small, tone: TextTone::Muted, "{context.t(purpose)}" }
                }
            }
        }

        Heading { level: HeadingLevel::_2, size: HeadingSize::H3, "{context.t(Key::SpacingScaleTitle)}" }
        Paragraph { "{context.t(Key::SpacingScaleBody)}" }
        div { class: "nd-scale",
            for step in steps {
                div { class: "nd-scale-row",
                    Code { class: "nd-scale-name", "--noctua-space-{step}" }
                    span { class: "nd-bar", style: "inline-size: var(--noctua-space-{step})" }
                }
            }
        }
    }
}

/// The four patterns, drawn, plus the ambient layers and a way to try them.
///
/// The page that documents background effects has to show them. Each swatch
/// below carries a real `data-noctua-pattern` on a scoped element, so what a
/// reader sees is the shipped geometry rather than a picture of it.
#[component]
fn Effects(context: Context) -> Element {
    let patterns = [
        ("dots", Key::SettingsPatternDots),
        ("mesh", Key::SettingsPatternMesh),
        ("checks", Key::SettingsPatternChecks),
        ("diagonals", Key::SettingsPatternDiagonals),
    ];
    rsx! {
        Heading { level: HeadingLevel::_2, size: HeadingSize::H3, "{context.t(Key::EffectsPatternsTitle)}" }
        Paragraph { "{context.t(Key::EffectsPatternsBody)}" }

        div { class: "nd-pattern-grid",
            for (value, label) in patterns {
                figure { class: "nd-pattern",
                    // The real layer, on a real attribute. `.noctua-fx` is the
                    // colour layer's own class, so this swatch is drawn by the
                    // shipped CSS and cannot drift from it.
                    div { class: "nd-pattern-swatch", "data-nd-pattern-demo": "{value}",
                        div { class: "noctua-fx nd-pattern-fx" }
                    }
                    figcaption { class: "nd-pattern-name",
                        Text { size: TextSize::Small, weight: TextWeight::Medium, "{context.t(label)}" }
                    }
                }
            }
        }

        Heading { level: HeadingLevel::_2, size: HeadingSize::H3, "{context.t(Key::EffectsAmbientTitle)}" }
        Paragraph { "{context.t(Key::EffectsAmbientBody)}" }

        Heading { level: HeadingLevel::_3, size: HeadingSize::H4, "{context.t(Key::EffectsTry)}" }
        Paragraph { tone: ParagraphTone::Muted, "{context.t(Key::EffectsTryBody)}" }
        div { class: "nd-choice", "data-nd-setting": "pattern",
            p { class: "nd-choice-label", id: "nd-label-try-pattern", "{context.t(Key::SettingsPatternLabel)}" }
            div { class: "nd-options", role: "radiogroup", "aria-labelledby": "nd-label-try-pattern",
                for (value, label) in [
                    ("none", Key::SettingsPatternNone),
                    ("dots", Key::SettingsPatternDots),
                    ("mesh", Key::SettingsPatternMesh),
                    ("checks", Key::SettingsPatternChecks),
                    ("diagonals", Key::SettingsPatternDiagonals),
                ] {
                    button {
                        class: "nd-option",
                        r#type: "button",
                        role: "radio",
                        "aria-checked": "false",
                        "data-nd-value": "{value}",
                        "{context.t(label)}"
                    }
                }
            }
        }
        for (setting, label) in [
            ("glow", Key::SettingsGlowLabel),
            ("grain", Key::SettingsGrainLabel),
        ] {
            div { class: "nd-choice", "data-nd-setting": "{setting}",
                p { class: "nd-choice-label", id: "nd-label-try-{setting}", "{context.t(label)}" }
                div { class: "nd-options", role: "radiogroup", "aria-labelledby": "nd-label-try-{setting}",
                    for (value, text) in [("off", Key::SettingsOff), ("on", Key::SettingsOn)] {
                        button {
                            class: "nd-option",
                            r#type: "button",
                            role: "radio",
                            "aria-checked": "false",
                            "data-nd-value": "{value}",
                            "{context.t(text)}"
                        }
                    }
                }
            }
        }
    }
}

/// The component page: the contract, the whole set rendered, and the props.
#[component]
fn Components(context: Context) -> Element {
    let request = [("palette", "violet-vivid"), ("mode", "dark")];
    rsx! {
        Heading { level: HeadingLevel::_2, size: HeadingSize::H3, "{context.t(Key::ComponentsContractTitle)}" }
        Paragraph { "{context.t(Key::ComponentsContractBody)}" }

        Heading { level: HeadingLevel::_2, size: HeadingSize::H3, "{context.t(Key::ComponentsSet)}" }
        Gallery { context }

        Heading { level: HeadingLevel::_2, size: HeadingSize::H3, "ThemeScope" }
        PropTable { context }

        Frame {
            context,
            caption: Key::PageComponentsTitle,
            code: snippet(&request),
            ThemeScope {
                class: "nd-scope",
                palette: Palette::VioletVivid,
                mode: Mode::Dark,
                Sample { context }
            }
        }
    }
}

/// Every text component, rendered rather than listed.
///
/// The page cannot claim a component exists that does not compile, because each
/// row below *is* the component.
#[component]
fn Gallery(context: Context) -> Element {
    rsx! {
        div { class: "nd-gallery",
            GalleryRow { name: "Heading",
                Heading { level: HeadingLevel::_3, size: HeadingSize::H4, "{context.t(Key::ScopeSampleHeading)}" }
            }
            GalleryRow { name: "Text",
                Text { weight: TextWeight::Semibold, "Semibold" }
                Text { tone: TextTone::Muted, " muted " }
                Text { tone: TextTone::Accent, "accent" }
            }
            GalleryRow { name: "Link",
                Link { href: "#", "{context.t(Key::ScopeSampleLink)}" }
                Link { href: "https://example.org", external: true, class: "nd-gallery-gap", "external" }
            }
            GalleryRow { name: "Code", Code { "--noctua-space-gap" } }
            GalleryRow { name: "Kbd", Kbd { "Ctrl" } Kbd { class: "nd-gallery-gap", "K" } }
            GalleryRow { name: "Em, Strong",
                Em { "stress" }
                Strong { class: "nd-gallery-gap", "importance" }
            }
            GalleryRow { name: "Mark", Mark { "a search hit" } }
            GalleryRow { name: "Del, Ins",
                Del { "removed" }
                Ins { class: "nd-gallery-gap", "added" }
            }
            GalleryRow { name: "Abbr", Abbr { title: "Cascading Style Sheets", "CSS" } }
            GalleryRow { name: "Time", Time { datetime: "2026-07-28", "28 July 2026" } }
            GalleryRow { name: "Truncate",
                Truncate { title: "{context.t(Key::ScopeSampleBody)}", "{context.t(Key::ScopeSampleBody)}" }
            }
            GalleryRow { name: "List",
                List { spacing: ListSpacing::Tight,
                    ListItem { "One" }
                    ListItem { "Two" }
                }
            }
            GalleryRow { name: "Blockquote",
                Blockquote { Paragraph { "{context.t(Key::ScopeSampleBody)}" } }
            }
            GalleryRow { name: "CodeBlock",
                CodeBlock { language: "rust", "Heading level=3 size=h4" }
            }
            GalleryRow { name: "VisuallyHidden",
                VisuallyHidden { "announced, never shown" }
                Text { size: TextSize::Caption, tone: TextTone::Muted,
                    "there is hidden text here, for screen readers only"
                }
            }
        }
    }
}

#[component]
fn GalleryRow(name: String, children: Element) -> Element {
    rsx! {
        div { class: "nd-gallery-row",
            Code { class: "nd-gallery-name", tone: CodeTone::Subtle, "{name}" }
            div { class: "nd-gallery-demo", {children} }
        }
    }
}

/// The prop surface, straight out of the model.
#[component]
fn PropTable(context: Context) -> Element {
    let _ = context;
    rsx! {
        div { class: "nd-props",
            for prop in crate::props::SCOPABLE {
                div { class: "nd-prop",
                    code { class: "nd-prop-name", "{prop.name}" }
                    code { class: "nd-prop-type", "Option<" "{prop.type_name}" ">" }
                    p { class: "nd-prop-values", "{prop.values}" }
                    p { class: "nd-prop-default", "default " code { "{prop.default}" } }
                }
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
///
/// # The code shown is derived from the example shown
///
/// `code` is not a string somebody typed next to the example. It is produced by
/// [`snippet`] from the very `ScopeRequest` that built the component above it,
/// in both languages. A hand-written snippet drifts from the component it
/// documents on the first edit, and the reader has no way to tell which one
/// lied.
#[component]
fn Frame(
    context: Context,
    caption: Key,
    #[props(default)] code: Vec<(&'static str, String)>,
    children: Element,
) -> Element {
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
            if !code.is_empty() {
                details { class: "nd-code",
                    summary { class: "nd-code-summary",
                        span { class: "nd-code-show", "{context.t(Key::ExampleCodeShow)}" }
                        span { class: "nd-code-hide", "{context.t(Key::ExampleCodeHide)}" }
                    }
                    div { class: "nd-code-body",
                        div { class: "nd-options", role: "radiogroup",
                            "aria-label": context.t(Key::ExampleImplLabel),
                            for (index, (language, _)) in code.iter().enumerate() {
                                button {
                                    class: "nd-option",
                                    r#type: "button",
                                    role: "radio",
                                    "aria-checked": if index == 0 { "true" } else { "false" },
                                    "data-nd-impl": "{language}",
                                    "data-nd-loading": context.t(Key::ExampleImplLoading),
                                    "{language}"
                                }
                            }
                        }
                        for (index, (language, source)) in code.iter().enumerate() {
                            pre {
                                class: "nd-code-source",
                                "data-nd-impl-source": "{language}",
                                hidden: index != 0,
                                code { "{source}" }
                            }
                        }
                    }
                }
            }
            figcaption { class: "nd-frame-caption", "{context.t(caption)}" }
        }
    }
}

/// The same scope, written in both languages.
///
/// Generated from the request rather than transcribed, so the two snippets agree
/// with each other and with the component the reader is looking at. Prop names
/// are the ones the generated components declare — `identifier` in Rust,
/// `camelCase` in JavaScript — which is why adding a scopable setting to
/// `spec/theme.toml` reaches this without anyone editing it.
fn snippet(request: &[(&str, &str)]) -> Vec<(&'static str, String)> {
    use std::fmt::Write as _;

    let mut rust = String::new();
    let mut react = String::new();
    for (name, value) in request {
        let _ = writeln!(rust, "    {name}: {},", rust_variant(name, value));
        let _ = writeln!(react, "  {}={{\"{value}\"}}", camel(name));
    }

    vec![
        (
            "Dioxus",
            format!("rsx! {{\n  ThemeScope {{\n{rust}    // …\n  }}\n}}"),
        ),
        (
            "React",
            format!("<ThemeScope\n{react}>\n  {{/* … */}}\n</ThemeScope>"),
        ),
    ]
}

/// `palette`, `azure-vivid` → `Palette::AzureVivid`.
fn rust_variant(setting: &str, value: &str) -> String {
    let type_name = pascal(setting);
    let variant = pascal(value);
    format!("{type_name}::{variant}")
}

fn pascal(name: &str) -> String {
    name.split(['-', '_'])
        .filter(|word| !word.is_empty())
        .map(|word| {
            let mut chars = word.chars();
            chars.next().map_or_else(String::new, |first| {
                first.to_uppercase().chain(chars).collect()
            })
        })
        .collect()
}

fn camel(name: &str) -> String {
    let pascal = pascal(name);
    let mut chars = pascal.chars();
    chars.next().map_or_else(String::new, |first| {
        first.to_lowercase().chain(chars).collect()
    })
}
