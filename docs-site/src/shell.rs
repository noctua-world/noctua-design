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
use noctua_design::IconName;
use noctua_design::components::{
    Button, ButtonTone, ButtonVariant, Disclosure, Label, LabelKind, LabelSize, Search,
    SearchPresentation,
};
use noctua_design_i18n::{Key, Locale};

use crate::site::{self, Page};
use crate::theme_menu::ThemeMenu;

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

    /// The same, with one `{placeholder}` filled in.
    ///
    /// A message with a value in it is still ONE message. Joining a number to
    /// a translated word would put the number where English puts it and leave
    /// it there in every other language, which is the concatenation
    /// `a_translated_message_may_not_be_concatenated` refuses.
    #[must_use]
    pub(crate) fn fill(self, key: Key, name: &str, value: &str) -> String {
        noctua_design_i18n::fill(self.t(key), &[(name, value)])
    }
}

/// The sticky header. One line at 320 px, and the budget is written down.
///
/// It used to carry a mode control of its own, beside a panel that had a mode
/// row in it — two ways to do one thing, and a third of the 320px budget spent
/// on the copy. There is one theme menu now, behind the palette, and the header
/// holds the four controls that are not settings: the drawer, the repository,
/// the language and the menu itself.
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
            // A `Button` now, and the three hooks it needed are three props.
            // It used to be hand-written markup with `data-nd-menu`,
            // `data-nd-open` and `data-nd-close` invented on the spot, because
            // no component could say "this control swaps its own name as it
            // opens". `action` and `label_alt` are that, said once, for every
            // control that will ever need it.
            Button {
                // `nd-nav-toggle`, NOT `nd-menu`. That name belongs to the
                // `Menu` component now, and a shell class sharing it would put
                // a floating panel's rules on the header's hamburger.
                class: "nd-nav-toggle",
                action: "menu",
                variant: ButtonVariant::Transparent,
                tone: ButtonTone::Neutral,
                hide_label: true,
                icon: IconName::Menu,
                icon_alt: IconName::Close,
                label: context.t(Key::NavOpen),
                label_alt: context.t(Key::NavClose),
                expanded: false,
                controls: "nd-nav",
            }
            a { class: "nd-wordmark", href: "{home}", "{context.t(Key::AppName)}" }
            div { class: "nd-header-controls",
                // THE IMPLEMENTATION SWITCH, FOR THE WHOLE PAGE AT ONCE.
                //
                // It is here rather than in the theme menu because that panel
                // is titled *Theme* and holds the appearance of the page — its
                // mode, its accent, its corners. Which of two packages renders
                // the examples is not an appearance, and a row for it there
                // would be the one hand-written exception in a panel generated
                // from the theme model.
                //
                // An icon, like the three beside it, because the header's
                // budget is one line at 320 px and a two-stop strip reading
                // `Dioxus | React` does not fit in it. A toggle carries both
                // names, and `aria-pressed` says which one is showing.
                //
                // Absent entirely where the bundle was never built. See
                // `main::react_available`.
                if crate::react_available() {
                    Button {
                        class: "nd-impl-toggle",
                        action: "impl",
                        variant: ButtonVariant::Transparent,
                        tone: ButtonTone::Neutral,
                        hide_label: true,
                        icon: IconName::Code,
                        label: context.t(Key::ExampleImplPageReact),
                        label_alt: context.t(Key::ExampleImplPageDioxus),
                        // Server-rendered as not pressed, and that is honest
                        // rather than a guess: at the moment this markup is
                        // written, Dioxus IS what is rendered. The script moves
                        // it if the reader has asked for React before, which is
                        // a state change and not the correction of a lie.
                        pressed: false,
                    }
                }
                Button {
                    variant: ButtonVariant::Transparent,
                    tone: ButtonTone::Neutral,
                    hide_label: true,
                    icon: IconName::Github,
                    label: context.t(Key::AppRepository),
                    href: "https://github.com/noctua-world/noctua-design",
                    rel: "noreferrer",
                }
                LocaleSwitch { context }
                Settings { context }
            }
        }
    }
}

/// The language control: an icon, and a menu of three.
///
/// It was a link that toggled between two locales, showing the name of the one
/// you were **not** reading. That works for exactly two languages and for
/// nobody who wants the third answer — *whichever my browser asks for* — which
/// is the answer a reader who never thinks about languages actually wants, and
/// the one this site now defaults to.
///
/// Each locale is named **in its own language**: a switcher that lists
/// languages in the language you are trying to leave is useless to the person
/// who needs it.
///
/// Every entry is a real `<a href>` to this same page in that locale, so the
/// control works with the script blocked — the whole site is static files and
/// there is nothing here a script has to reconstruct. What the script adds is
/// the storage and the automatic resolution. `aria-current` is server-rendered
/// on the default, so nothing announces a wrong answer before it runs.
#[component]
fn LocaleSwitch(context: Context) -> Element {
    let automatic = crate::defaults::AUTOMATIC_LOCALE;
    rsx! {
        Disclosure {
            class: "nd-locale",
            name: "locale",
            icon: IconName::Languages,
            summary: context.t(Key::LocaleLabel),
            div { class: "nd-panel nd-panel--menu noctua-glass",
                div { class: "nd-panel-body",
                    // A `<ul>` is not labelable, so its name is a `<p>` it
                    // points at. `Label` derives the id from `for_control`.
                    Label {
                        kind: LabelKind::Name,
                        size: LabelSize::Small,
                        for_control: "nd-locale",
                        text: context.t(Key::LocaleLabel),
                    }
                    ul { class: "nd-locale-list", "aria-labelledby": "nd-locale-label",
                        li {
                            a {
                                class: "nd-locale-option",
                                // With no script there is no detection, so the
                                // honest destination is the source locale.
                                href: "{context.page.locale_href(Locale::DEFAULT)}",
                                "data-nd-locale": "{automatic}",
                                "aria-current": "true",
                                span { class: "nd-locale-name", "{context.t(Key::LocaleAutomatic)}" }
                                span { class: "nd-locale-note", "{context.t(Key::LocaleAutomaticNote)}" }
                            }
                        }
                        for locale in Locale::OFFERED {
                            li {
                                a {
                                    class: "nd-locale-option",
                                    href: "{context.page.locale_href(locale)}",
                                    lang: "{locale.bcp47()}",
                                    "data-nd-locale": "{locale.tag()}",
                                    // A language's own name is not a string to
                                    // translate. `.nd-identifier` says so, and
                                    // is what the pseudo-locale test skips.
                                    span { class: "nd-locale-name nd-identifier", "{locale.name()}" }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
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
        Disclosure {
            class: "nd-settings",
            // The hook `shell.js` binds to, for the lazy palette fetch behind
            // the accent swatches and for closing on a click outside. It used
            // to be a hand-written `data-nd-settings` on a hand-written
            // `<details>`; when this became the generated `Disclosure` the
            // attribute went with it, the script's `querySelector` returned
            // `null`, and `?.` swallowed it. Every swatch then painted the
            // page's accent and the panel would not close.
            name: "settings",
            // A palette, not a gear. The menu is the appearance of the page —
            // its mode, its accent, its corners — and a gear says "settings"
            // about a product that has no other kind.
            icon: IconName::Palette,
            summary: context.t(Key::SettingsLabel),
            div { class: "nd-panel noctua-glass",
                div { class: "nd-panel-body",
                    ThemeMenu { context, name: "page" }
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
/// The command palette: this site's own search, and the component's own dialog.
///
/// # Why the sidebar stopped being the search
///
/// It was a box that filtered the navigation and revealed three hundred and
/// thirteen hidden links under it. Two problems, and the second is the one that
/// decided this: filtering the navigation removes the rows a reader is
/// navigating *by*, and a two-hundred-and-forty-pixel column cannot show a
/// ranked result — its name, the letters that matched, and the page it leads
/// to. A search whose results cannot say where they lead sends the reader back
/// to the guess they started with.
///
/// # One dialog, one index, wherever it is opened from
///
/// The index is a file now (`index::json`), fetched the first time somebody
/// searches and cached for the rest of the visit, rather than re-sent inside
/// every document. That is also the honest half of `Search`'s contract: an
/// index at a URL is the form that stops working with the script blocked, and
/// the trigger ships hidden until the script reveals it — because a control
/// that opens a modal nothing can fill is a button that does nothing.
#[component]
pub(crate) fn Palette(context: Context) -> Element {
    rsx! {
        Search {
            name: "docs",
            presentation: SearchPresentation::Dialog,
            label: context.t(Key::SearchLabel),
            placeholder: context.t(Key::SearchPlaceholder),
            // Resolved by the browser against the document, which is why it is
            // one path for every page rather than one per depth.
            src: format!("{}assets/search-{}.json", site::BASE, context.locale.tag()),
            hotkey: true,
            cap: "12",
            recent: "noctua.docs.recent",
            status: context.t(Key::SearchResults),
            empty_label: context.t(Key::SearchEmpty),
            loading_label: context.t(Key::SearchLoading),
            error_label: context.t(Key::SearchError),
            cap_label: context.t(Key::SearchShowing),
            clear_label: context.t(Key::SearchClear),
            recent_label: context.t(Key::SearchRecent),
            forget_label: context.t(Key::SearchForget),
            close_label: context.t(Key::NavDismiss),
        }
    }
}

#[component]
pub(crate) fn Sidebar(context: Context) -> Element {
    // Sorted once. The rule between groups needs the row before this one, and
    // asking for the whole ordering again per row is thirty-one sorts to draw
    // thirty-one links.
    let pages = site::sorted(context.locale);
    rsx! {
        nav { id: "nd-nav", class: "nd-nav noctua-glass", "aria-label": context.t(Key::NavLabel),
            // The wordmark lives here below `sm`, where the header cannot afford
            // it. It is not a duplicate: exactly one of the two is ever shown.
            a { class: "nd-nav-wordmark", href: "{site::first(context.locale).href()}",
                "{context.t(Key::AppName)}"
            }
            Palette { context }
            // The introduction alone, then the foundations as a short block,
            // then every component alphabetically with nothing between them —
            // which is what `NOCTUA.md §9.10` closes on. A rule separates each
            // group; it is not a category heading, because a category heading
            // among the components is the thing that rule forbids.
            ul { class: "nd-nav-list",
                for (index, page) in pages.iter().copied().enumerate() {
                    li {
                        class: if site::starts_a_group(&pages, index) {
                            "nd-nav-item nd-nav-item--group"
                        } else {
                            "nd-nav-item"
                        },
                        a {
                            class: if page == context.page { "nd-nav-link nd-nav-link--current" } else { "nd-nav-link" },
                            href: "{page.href()}",
                            "aria-current": if page == context.page { "page" },
                            span {
                                // A component's title is its NAME, and a name
                                // is the same word in every locale.
                                class: if page.title_is_a_name() {
                                    "nd-nav-text nd-identifier"
                                } else {
                                    "nd-nav-text"
                                },
                                "{page.title(context.locale)}"
                            }
                        }
                    }
                }
            }
            // WHAT IS LEFT HERE IS THE NAVIGATION, WHOLE.
            //
            // The index used to be three hundred and thirteen hidden links in
            // every document of every locale, filtered by hiding the rows above
            // as well — and a filter that hides the rows you are navigating
            // destroys the navigation while you use it, which is why the old
            // handler needed two special cases for the empty query. `Palette`
            // shows results and leaves this alone.
            //
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

// THERE IS NO `Effects` COMPONENT ANY MORE, and its absence is the point.
//
// The pattern and the grain used to be fixed, inert boxes
// this site had to remember to render, because `dist/theme.css` bound each
// effect to a class — `.noctua-fx`, `.noctua-grain`. That is what made the
// three settings page-only: a payload on a descendant is settled by source
// order rather than by proximity, so a frame could not have its own and could
// not cancel the page's. It also made `ThemeScope { pattern }` a prop that
// compiled, stamped an attribute and painted nothing for any consumer of the
// package outside this repository.
//
// They are background layers on the element carrying the attribute now, so the
// page paints them through the same rule an ExampleFrame does — and a
// background is behind content by definition, which is why there is no
// `z-index` here any more, no `isolation`, and none of the stacking-order
// class of bug that cost the old layer three regressions.
