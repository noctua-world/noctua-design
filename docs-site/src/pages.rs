//! The page bodies.
//!
//! Prose carries the measure, not the layout. `max-inline-size` lives on the
//! paragraph, never on `<main>` — the page is as wide as the screen and the
//! *line* is not (`NOCTUA.md §9.7`), which is also what leaves the empty column
//! the effect layers show through.

use dioxus::prelude::*;
use noctua_design::ThemeScope;
use noctua_design::components::{
    Code, CodeTone, Heading, HeadingLevel, HeadingSize, Kbd, List, ListItem, ListKind, ListSpacing,
    Paragraph, ParagraphTone, Text, TextSize, TextTone, TextWeight, VisuallyHidden,
};
use noctua_design::model::{Glass, Mode, Palette, Radius};
use noctua_design::{Icon, IconName, IconSize};
use noctua_design_i18n::Key;

use crate::shell::Context;

/// Dispatches to the body for this page.
#[component]
pub(crate) fn Body(context: Context) -> Element {
    rsx! {
        h1 { class: "nd-title", "{context.page.title(context.locale)}" }
        p { class: "nd-lead", "{context.t(context.page.summary)}" }
        match context.page.slug {
            "scoped-themes" => rsx! { ScopedThemes { context } },
            "colours" => rsx! { Colours { context } },
            "typography" => rsx! { Typography { context } },
            "spacing" => rsx! { Spacing { context } },
            "effects" => rsx! { Effects { context } },
            "icons" => rsx! { Icons { context } },
            // Every component, from its contract. Not a `match` arm each: the
            // page that listed components by hand is what this replaced.
            _ if context.page.component.is_some() => rsx! { ComponentPage { context } },
            // No catch-all body. A page with nothing to render is a build
            // failure, not a page that quietly shows something else: the site is
            // generated, so this stops the build and never reaches a reader.
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

        // `ThemeScope` is the one component with no `spec/components/*.toml`:
        // its props ARE `spec/theme.toml`'s scopable settings, and a second
        // file could disagree with the first. So its table lives here, on the
        // page about scoping, rather than on a page of its own.
        Heading { level: HeadingLevel::_2, size: HeadingSize::H3, "{context.t(Key::ComponentProps)}" }
        table { class: "nd-table",
            thead {
                tr {
                    th { "{context.t(Key::ComponentColumnName)}" }
                    th { "{context.t(Key::ComponentColumnType)}" }
                    th { "{context.t(Key::ComponentColumnDefault)}" }
                    th { "{context.t(Key::ComponentColumnEffect)}" }
                }
            }
            tbody {
                for prop in crate::props::SCOPABLE {
                    tr {
                        td { Code { "{prop.name}" } }
                        td { Code { class: "nd-type", "{prop.type_name}" } }
                        td { Code { "{prop.default}" } }
                        td { "{prop.values}" }
                    }
                }
            }
        }
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

// --- One page per component, from the contract ------------------------------

/// A component's page: what it is, every variant of it rendered live, every
/// prop, every state, its keyboard contract and its accessibility contract.
///
/// # Why the examples are markup and not the component
///
/// The page renders the DOM the component produces — its element, its class and
/// the attribute each prop becomes — rather than calling the Dioxus component.
/// That is not a shortcut: `props.rs` GENERATES the component's body from
/// exactly those three facts, so the two cannot disagree. What it buys is that
/// every value of every enum prop is shown by construction, with no example to
/// author and none to forget when a variant is added.
#[component]
fn ComponentPage(context: Context) -> Element {
    let component = context.page.component.expect("a component page");
    rsx! {
        if !component.guidance.is_empty() {
            Paragraph { "{component.guidance}" }
        }

        Heading { level: HeadingLevel::_2, size: HeadingSize::H3, "{context.t(Key::ComponentExamples)}" }
        for prop in component.props.iter().filter(|prop| prop.kind == "enum") {
            div { class: "nd-variant",
                Code { "{prop.name}" }
                div { class: "nd-variant-row",
                    for value in prop.values {
                        Specimen { component, prop: *prop, value: (*value).to_string() }
                    }
                }
            }
        }

        Heading { level: HeadingLevel::_2, size: HeadingSize::H3, "{context.t(Key::ComponentProps)}" }
        table { class: "nd-table",
            thead {
                tr {
                    th { "{context.t(Key::ComponentColumnName)}" }
                    th { "{context.t(Key::ComponentColumnType)}" }
                    th { "{context.t(Key::ComponentColumnDefault)}" }
                    th { "{context.t(Key::ComponentColumnEffect)}" }
                }
            }
            tbody {
                for prop in component.props {
                    tr {
                        td { Code { "{prop.name}" } }
                        td {
                            if prop.values.is_empty() {
                                Code { class: "nd-type", "{prop.kind}" }
                            } else {
                                for value in prop.values {
                                    Code { class: "nd-type", "{value}" }
                                }
                            }
                        }
                        td {
                            if prop.required {
                                Text { tone: TextTone::Accent, "{context.t(Key::ComponentRequired)}" }
                            } else if let Some(default) = prop.default {
                                Code { "{default}" }
                            } else {
                                "{context.t(Key::ComponentNoDefault)}"
                            }
                        }
                        td { "{prop.doc}" }
                    }
                }
            }
        }

        Heading { level: HeadingLevel::_2, size: HeadingSize::H3, "{context.t(Key::ComponentStates)}" }
        if component.states.is_empty() {
            Paragraph { tone: ParagraphTone::Muted, "{context.t(Key::ComponentStatesNone)}" }
        } else {
            table { class: "nd-table",
                thead {
                    tr {
                        th { "{context.t(Key::ComponentColumnName)}" }
                        th { "{context.t(Key::ComponentColumnAttribute)}" }
                        th { "{context.t(Key::ComponentColumnEffect)}" }
                    }
                }
                tbody {
                    for state in component.states {
                        tr {
                            td { Code { "{state.name}" } }
                            td { Code { "{state.attribute}" } }
                            td { "{state.doc}" }
                        }
                    }
                }
            }
        }

        Heading { level: HeadingLevel::_2, size: HeadingSize::H3, "{context.t(Key::ComponentKeyboard)}" }
        if component.keyboard.is_empty() {
            Paragraph { tone: ParagraphTone::Muted, "{context.t(Key::ComponentKeyboardNone)}" }
        } else {
            table { class: "nd-table",
                tbody {
                    for key in component.keyboard {
                        tr {
                            td { Kbd { "{key.key}" } }
                            td { "{key.action}" }
                        }
                    }
                }
            }
        }

        Heading { level: HeadingLevel::_2, size: HeadingSize::H3, "{context.t(Key::ComponentA11y)}" }
        if let Some(role) = component.role {
            Paragraph {
                "{context.t(Key::ComponentA11yRole)} "
                Code { "{role}" }
            }
        }
        if component.needs_name {
            Paragraph { "{context.t(Key::ComponentA11yName)}" }
        }
        if !component.a11y_notes.is_empty() {
            Paragraph { "{component.a11y_notes}" }
        }
        if component.role.is_none() && !component.needs_name && component.a11y_notes.is_empty() {
            Paragraph { tone: ParagraphTone::Muted, "{context.t(Key::ComponentA11yPlain)}" }
        }

        Heading { level: HeadingLevel::_2, size: HeadingSize::H3, "{context.t(Key::ComponentSource)}" }
        List { kind: ListKind::Unordered, spacing: ListSpacing::Tight,
            for (label, path) in source_paths(component.name) {
                ListItem {
                    "{context.t(label)} — "
                    Code { "{path}" }
                }
            }
        }
    }
}

/// One rendering of a component, with one enum prop set to one value.
///
/// The markup is built from the contract: the element, the root class, and the
/// attribute the prop becomes. `props.rs` generates the component's body from
/// the same three facts, so this shows what the component shows.
#[component]
fn Specimen(
    component: crate::generated::catalogue::DocComponent,
    prop: crate::generated::catalogue::DocProp,
    value: String,
) -> Element {
    let class = component.class.unwrap_or_default();
    let attribute = prop.attribute.unwrap_or(prop.name);
    rsx! {
        figure { class: "nd-variant-item",
            div {
                class: "nd-variant-stage",
                dangerous_inner_html: specimen_markup(component, attribute, &value),
            }
            figcaption { class: "nd-variant-label",
                Code { "{value}" }
                if class.is_empty() {
                    VisuallyHidden { "{component.element}" }
                }
            }
        }
    }
}

/// The element the component renders, carrying one attribute and some content.
fn specimen_markup(
    component: crate::generated::catalogue::DocComponent,
    attribute: &str,
    value: &str,
) -> String {
    // Block-level components read better with a sentence; an inline one reads
    // better with its own name, which is also what a reader is looking for.
    let block = matches!(
        component.element,
        "p" | "div" | "blockquote" | "ul" | "ol" | "dl" | "h1" | "h2" | "h3" | "h4" | "pre"
    );
    let content = if block { SPECIMEN } else { component.name };
    let class = component
        .class
        .map(|class| format!(" class=\"{class}\""))
        .unwrap_or_default();
    format!(
        "<{element}{class} data-nd-{attribute}=\"{value}\">{content}</{element}>",
        element = component.element,
    )
}

/// The specimen sentence. English on purpose: it is a pangram, and translating
/// a pangram gives a sentence that is no longer one.
const SPECIMEN: &str = "Sphinx of black quartz, judge my vow";

/// Where a component's four files live, for a reader who wants the truth.
fn source_paths(name: &str) -> [(Key, String); 3] {
    let stem = file_stem(name);
    [
        (
            Key::ComponentSourceSpec,
            format!("spec/components/{stem}.toml"),
        ),
        (
            Key::ComponentSourceDioxus,
            format!("crates/noctua-design/src/components/{stem}.rs"),
        ),
        (
            Key::ComponentSourceReact,
            format!("packages/react/src/components/{stem}.js"),
        ),
    ]
}

/// `CodeBlock` → `code_block`, the file the component came from.
fn file_stem(name: &str) -> String {
    let mut out = String::with_capacity(name.len() + 2);
    for (index, ch) in name.char_indices() {
        if ch.is_ascii_uppercase() && index > 0 {
            out.push('_');
        }
        out.extend(ch.to_lowercase());
    }
    out
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
                div { class: "nd-variant-item",
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
