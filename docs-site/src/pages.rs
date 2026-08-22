//! The page bodies.
//!
//! Prose carries the measure, not the layout. `max-inline-size` lives on the
//! paragraph, never on `<main>` — the page is as wide as the screen and the
//! *line* is not (`NOCTUA.md §9.7`), which is also what leaves the empty column
//! the effect layers show through.
//!
//! # One page shape, and it is built once
//!
//! [`DocPage`] is the only page body: a head (title, one-line lead, and one
//! complete working example from `examples::OVERVIEWS`), a `DocIndex`, then the
//! sections `ComponentSections` emits — **when · examples · variants ·
//! reference**. `Depth` is what a collection page changes and the only thing it
//! changes; it used to be a `bool` and two copies of a hundred lines, which is
//! how the two copies drifted apart. A page whose shape differs from this is a
//! bug, and `tests/browser/audit.spec.js` asserts it on every page.
//!
//! The reference folds ship CLOSED, so every `<summary>` carries a real
//! heading: closed, the title is all that is left of the section. They are
//! rendered without a `name`, because `shell.js` closes any open *named*
//! disclosure the pointer lands outside of — right for the header's panels,
//! wrong for a section a reader opened on purpose.
//!
//! # Every document sits at the same depth
//!
//! `<locale>/<slug>/index.html`, for foundations and components alike, which is
//! what makes `site::BASE` a constant rather than a calculation and every
//! relative href resolve identically after the router swaps a page in.
//! `site.rs` asserts it. Nothing shows `.html`, so `file://` cannot serve this
//! site — `cargo xtask dev` is the way to look at it — and a directory reached
//! without its trailing slash must 301, or every document-relative href on the
//! page resolves one level too high.

use dioxus::prelude::*;
use noctua_design::ThemeScope;
use noctua_design::components::{
    Button, ButtonSize, ButtonTone, ButtonVariant, Code, CodeTone, Disclosure, DisclosureHeading,
    Em, Heading, HeadingLevel, HeadingSize, Input, Kbd, Label, LabelKind, LabelSize, Link, Menu,
    MenuItem, Paragraph, ParagraphSize, ParagraphTone, Prose, Segmented, SegmentedItem,
    SegmentedSize, Strong, Table, TableCell, TableCellAlign, TableColumn, TableColumnWidth,
    TableHeaderCell, TableHeaderCellAlign, TableHeaderCellScope, TableHeaderCellWrap, TableRow,
    TableSection, TableSectionKind, Text, TextSize, TextTone, TextWeight, Toolbar, ToolbarDensity,
};
use noctua_design::model::{Glass, Mode, Palette, Radius};
use noctua_design::{Icon, IconDrawing, IconSize};
use noctua_design_i18n::Key;
use noctua_design_tokens as tokens;

use crate::shell::Context;
use crate::site;
use crate::theme_menu::{Reach, ThemeMenu};
use crate::tree::Node;

/// Dispatches to the body for this page.
///
/// # One shape, on every page
///
/// Fifty-six pages used to be four different pages. `ComponentBody` had two
/// arms that were the same code with every heading pushed down one;
/// `CollectionPage` stacked whole references end to end; `LineagePage` told its
/// own story; and six foundation pages were each their own thing. A reader who
/// learned where the props table lived on `/button/` learned nothing about
/// `/tables/`, and `/tables/` was fifty-seven thousand pixels tall.
///
/// Everything below is now one skeleton — a head that opens with a complete
/// working example, an index of what is on the page, and sections in the same
/// order every time — and the pages differ in their CONTENT rather than in
/// their shape.
#[component]
pub(crate) fn Body(context: Context) -> Element {
    rsx! {
        article { class: "nd-doc",
            DocHead { context }
            match context.page.slug {
                "intro" => rsx! {
                    section { class: "nd-doc-section", id: "intro", Intro { context } }
                },
                // THE TWO FOUNDATION PAGES RENDER THEIR OWN SECTIONS, like the
                // lineage page below. Each documents three separate things —
                // a scale, a set of families, a set of scripts — and one
                // `<section>` around all three is a page whose outline is one
                // entry and whose jump index has nothing to point at.
                "typography" => rsx! { Typography { context } },
                "spacing" => rsx! { Spacing { context } },
                "icons" => rsx! {
                    section { class: "nd-doc-section", id: "icons", Icons { context } }
                },
                // The one component whose page is written rather than derived: its
                // props ARE the scopable settings, so it has no contract file and
                // nothing to derive a page from.
                "theme-scope" => rsx! {
                    section { class: "nd-doc-section", id: "theme-scope", ThemeScopePage { context } }
                },
                // The site's own apparatus, documented like everything else on it
                // — and the one page where a frame inside a frame is the subject
                // rather than a mistake.
                "example-frame" => rsx! {
                    section { class: "nd-doc-section", id: "example-frame", ExampleFramePage { context } }
                },
                // The one collection whose page is TOLD rather than stacked. Two
                // stacked references put the figure behind the scene's whole
                // contract, and twenty-nine frames in declaration order read as
                // repetition — the reader said so. The examples and the derived
                // variants and both contracts are all still here, in an order a
                // reader can follow.
                "lineage" => rsx! { LineagePage { context } },
                // A collection: several components documented together, because a
                // reader comparing them should open one page and not six. Asked of
                // the page rather than matched on its slug, so a third collection
                // is one entry in `site::COLLECTIONS` and nothing here.
                _ if site::collection(context.page).is_some() => rsx! { CollectionPage { context } },
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
}

/// The first thing on every page: what it is, in a line, and one working model.
///
/// # The model comes before the explanation
///
/// The hero is a complete, real use of the component — several props at once,
/// real content, and on a collection page the parts working together. It is
/// deliberately the FIRST thing under the title, before the guidance and before
/// the examples, because a reader who has seen the thing can read a sentence
/// about it and a reader who has not cannot.
///
/// A page with no overview renders the title and the lead and nothing else
/// here, which is what the foundation pages want: `/spacing/` documents a scale
/// and has no component to model.
#[component]
fn DocHead(context: Context) -> Element {
    let overview = crate::examples::overview_for_page(context.page.slug);
    rsx! {
        header { class: "nd-doc-head",
            h1 {
                // A component page is titled with the component's NAME, which is
                // the same word in every locale.
                class: if context.page.title_is_a_name() {
                    "nd-title nd-identifier"
                } else {
                    "nd-title"
                },
                "{context.page.title(context.locale)}"
            }
            p { class: "nd-lead", "{context.t(context.page.summary)}" }
            if let Some(example) = overview {
                div { class: "nd-doc-hero",
                    {framed(
                        context,
                        &format!("{}-overview", context.page.slug),
                        Key::ComponentOverview,
                        // A hero shows the page's own subject, so the React
                        // caption names the component the page is about. Where
                        // the page groups several, the first in the tree is the
                        // one the caption names, which is the same rule the
                        // hand-written Dioxus caption follows.
                        context.page.component.map_or("", |component| component.name),
                        &example,
                        crate::trees::for_page(context.page.slug).map_or(&[], |(_, _, nodes)| nodes),
                    )}
                }
            }
        }
    }
}

/// Where a page can be jumped to, on every page rather than on eleven of them.
///
/// The shape the collection pages already had — a wrapping row of links under a
/// label — pointed at whatever the page's own sections are. It is a plain
/// `<nav>` in the flow and not a sticky rail: `shell.spec.js` allows exactly
/// four kinds of floating surface on this site, and an outline that pins itself
/// to the viewport would be a fifth asking to be justified.
#[component]
fn DocIndex(context: Context, marks: Vec<Jump>) -> Element {
    if marks.is_empty() {
        return rsx! {};
    }
    rsx! {
        nav { class: "nd-doc-index", "aria-label": context.t(Key::ComponentOnThisPage),
            // The name of a `<nav>`, which is not labelable — see `Label`.
            Label {
                kind: LabelKind::Name,
                size: LabelSize::Small,
                text: context.t(Key::ComponentOnThisPage),
            }
            ul { class: "nd-doc-index-list",
                for mark in marks {
                    li {
                        Link {
                            // A COMPONENT'S NAME IS THE SAME WORD IN EVERY
                            // LANGUAGE, and the pseudo-locale test reads any
                            // unmarked run as a sentence somebody forgot to
                            // translate — which is exactly what it reported
                            // the moment these links lost the class.
                            class: if mark.name { "nd-identifier" } else { "" },
                            href: "#{mark.id}",
                            "{mark.label}"
                        }
                    }
                }
            }
        }
    }
}

/// One entry in a page's jump index.
#[derive(Clone, PartialEq, Eq)]
struct Jump {
    /// The id it lands on.
    id: String,
    /// What the link says.
    label: String,
    /// Whether that label is a name rather than a sentence.
    name: bool,
}

impl Jump {
    /// A section of the page, named by a translated heading.
    fn section(id: String, label: &str) -> Self {
        Self {
            id,
            label: label.to_owned(),
            name: false,
        }
    }

    /// A component, named by the one word it is called in every language.
    fn component(id: &str, label: &str) -> Self {
        Self {
            id: id.to_owned(),
            label: label.to_owned(),
            name: true,
        }
    }
}

// --- The parent every example sits in ---------------------------------------

/// Every viewport the width control offers, and what each one is.
///
/// The values ARE the system's own breakpoints (`spec/tokens.toml`), so the
/// control teaches the scale rather than inventing one beside it. `320` is not
/// a breakpoint and is deliberately first: `tokens.toml` calls it "the
/// narrowest screen still in real use and the floor every layout must
/// survive", and a rig that could not show you the floor would be missing the
/// one width that is not negotiable.
/// Every viewport the width control offers, and the length each one names.
///
/// The lengths are READ FROM THE TOKEN MODULE, not written here. A label that
/// said `30rem` beside a stylesheet that resized to `var(--noctua-breakpoint-sm)`
/// would agree today and quietly stop agreeing the day the scale moved — and a
/// viewport rig whose numbers lie is worse than one with no numbers at all.
/// `320px` is the one that is not a breakpoint, and it is the floor
/// `spec/tokens.toml` names in the same breath as saying so.
fn viewports() -> [(&'static str, Option<&'static str>); 5] {
    use noctua_design_tokens::breakpoint;
    [
        ("320", Some(FLOOR)),
        ("sm", Some(breakpoint::SM)),
        ("md", Some(breakpoint::MD)),
        ("lg", Some(breakpoint::LG)),
        ("full", None),
    ]
}

/// The 320px floor, as the one length on this page that is not a token.
///
/// `spec/tokens.toml` states it and states why it is not a breakpoint: it is
/// the default, the width every layout must survive, so there is nothing to
/// query above it. There is no token for "the floor", and inventing one to
/// avoid a literal here would put the same string in two files.
const FLOOR: &str = "320px"; // allow-literal: the viewport floor `spec/tokens.toml` names in prose — a label a reader reads, and the stylesheet spells the same width from the spacing scale

/// How many coloured shapes sit behind a stage under the glass test.
///
/// Twenty-four, because the material has to be judged against several colours
/// at once, against the EDGES between them — blur is only visible where there
/// is an edge to lose, and a single wash has none — and against several SIZES.
/// Half of them are small on purpose: a 192px block reads the same at `subtle`
/// as at `full`, because the blur radius is smaller than the shape and only its
/// edge moves, while a 16px dot is the size the radius actually acts on.
///
/// The last six are anchored to the field's CENTRE rather than to an edge. On a
/// narrow frame the edge-anchored ones overlap into one mass; on a wide one
/// they separate into two clusters with a blank corridor down the middle, and a
/// control parked in that corridor has nothing behind it to judge.
///
/// Each one takes its colour, its size and its place from `data-nd-shape`, in
/// the stylesheet, from the semantic palette — no shape here knows a colour.
const GLASS_SHAPES: usize = 30;

/// The frame that hosts every example on this site.
///
/// # One abstraction, and now it really is one
///
/// `NOCTUA.md §9.10` calls for exactly this: *one abstraction hosts every
/// example*, owning the mount point, the implementation swap, the viewport
/// controls and the reset. It existed and hosted **one** example — the
/// `ThemeScope` demonstration — while a hundred and ten component examples sat
/// in bare `<figure>`s beside it. Then it held them all, and the controls
/// themselves were still hand-assembled: two radio groups whose classes had no
/// CSS anywhere, a reset whose one distinguishing rule matched nothing, and a
/// bar that was a bare `<div>` claiming no role and taking three tab stops.
///
/// Now every part of it is a component. The bar is a `Toolbar`, so it is one
/// tab stop and the arrows walk it; the viewport control is a `Segmented` in a
/// `ToolbarGroup { align: End }`, so "on the right" is a contract rather than
/// a stylesheet rule against a class nothing carried; the two controls beside
/// it are `Button`s.
///
/// # Four controls, and what each one is for
///
/// **The viewport.** Five stops, from the 320px floor to full width, resizing
/// the whole set at once — *show me all of this at 320 px* is a question about
/// a component, not about one example of it. The caption reports the stage's
/// real width as it changes, so the number is measured rather than promised.
///
/// **The appearance toggle.** It flips the mode *inside the frame only*, by
/// stamping a scope on the stage: the examples under it obey, and the page
/// around it does not move. It is a toggle rather than a cycle, and it carries
/// both glyphs and both names so the state is visible before it is pressed —
/// `shell.rs` argues that case at length about the header's mode control, and
/// this is the same argument.
///
/// **The implementation switch**, on the one frame that can honour it. A
/// control offered where it does nothing is a control that lies.
///
/// **The reset**, which puts all three back.
///
/// # Why it is the site's and not the design system's
///
/// Everything it owns is documentation apparatus. Lazily importing a React
/// bundle to prove two packages agree is a thing a docs site does, not a thing
/// a design system ships to its consumers. What it is BUILT from is components,
/// and after this change that is *all* it is built from.
#[component]
fn ExampleFrame(
    context: Context,
    /// What this frame is, as an id root: the page it is on, or the component
    /// it demonstrates.
    ///
    /// Required for the same reason `Search { name }` is. The theme menu inside
    /// every frame built its label ids from the setting alone, so a page with
    /// one frame declared each of them twice and `/tables`, with one frame per
    /// component, declared each of them seven times — and every frame's
    /// `aria-labelledby` resolved to the header's label rather than to its own.
    name: String,
    /// What this frame is showing, for the toolbar's accessible name.
    ///
    /// A `role="toolbar"` with no name announces the word `toolbar` and
    /// nothing else, and a page with several frames would announce it several
    /// times identically.
    label: Key,
    /// Whether the React bundle can mount what is inside this frame.
    ///
    /// It used to be true of one frame in the whole site, and offering the
    /// switch where it does nothing is a control that lies. `NOCTUA.md §9.3`
    /// Leg 4 was therefore demonstrated in one place — which is exactly how six
    /// modules of the React package came to be broken unnoticed.
    ///
    /// Every example carries a tree now, so this is true wherever there is one
    /// to mount and the bundle was built. `framed` computes it; nothing sets it
    /// by hand except the `ThemeScope` frame, whose component is in the theme
    /// model rather than in `spec/components/`.
    #[props(default)]
    mountable: bool,
    children: Element,
) -> Element {
    rsx! {
        figure {
            class: "nd-frame",
            "data-nd-frame": "",
            // The loading message lives on the frame rather than on a control:
            // `Button` takes the props its contract declares and no others,
            // which is the point of it having a contract.
            "data-nd-loading": context.t(Key::ExampleImplLoading),
            // THE FROST IS A LAYER, NOT THE FRAME.
            //
            // `backdrop-filter` on an element makes it the containing block for
            // every `position: fixed` DESCENDANT — so frosting the frame itself
            // re-anchored the theme menu inside it to the frame, which is
            // `overflow: clip`, and the panel opened 1400px below the fold
            // where nothing could see it. The header solves this the same way
            // and for the same reason: `.nd-header-glass` exists so that the
            // header carries no filter and its own panel can stay fixed.
            //
            // Inert until the rig stamps it: no attribute, no material, no
            // cost, and the frame is the shape it always was.
            div { class: "nd-frame-glass", "aria-hidden": "true" }
            // THE CONTROLS ARE A RAIL DOWN THE FRAME'S TRAILING EDGE.
            //
            // They were a permanent horizontal bar, which cost more than it was
            // worth: measured on the built page at 1120px, `/button/` spent
            // 1,416px on twenty-two of those bars against 1,543px of actual
            // example. Then they were folded behind a gear, and pressing a
            // control to reveal the controls was worse than the bar had been.
            //
            // A frame is 900px wide and its widest example ends 487px in, so
            // every frame on this site has between 413 and 815px of unused
            // space on its right. The rail costs that space, which nothing was
            // using, instead of height, which every frame is short of.
            //
            // IN FLOW, NOT PINNED. It is a grid item with `position: static`,
            // so `shell.spec.js`'s floating-surface sweep — which only
            // considers `fixed`, `sticky` and `absolute` — does not see it and
            // needs no new entry. It is also why it cannot travel with the
            // page, which `frame.spec.js` asserts and the bar learned the hard
            // way.
            Toolbar {
                class: "nd-frame-bar",
                // THE AXIS IS NOT FIXED, SO THE ATTRIBUTE CANNOT BE EITHER.
                //
                // This strip is a rail down the trailing edge on a wide screen
                // and a row under the stage on a narrow one. `Toolbar` writes
                // `aria-orientation` and `shell.js` reads it to decide which
                // arrows walk the strip, so a value baked in here would be
                // wrong at one of the two widths — announcing an axis a reader
                // cannot walk is the thing this system refuses everywhere else.
                //
                // It ships `horizontal`, which is the ARIA default and is true
                // below the breakpoint, and `shell.js` upgrades it to vertical
                // when the stylesheet stands it up. With no script the layout
                // is the narrow one and the attribute is already right.
                density: ToolbarDensity::Compact,
                label: context.t(label),
                // THE WIDTH, AS ONE CONTROL THAT OPENS THE LIST.
                //
                // It was five stops side by side, which is the better shape and
                // does not fit: stacked they are 150px, and with the four icons
                // under them the rail reaches 278px — taller than half the
                // frames on this page.
                //
                // A `Menu` rather than a `Select` or a vertical `Segmented`,
                // and `spec/components/menu.toml` says a menu is actions rather
                // than values. That holds: pressing *320 px* resizes this
                // stage. The width is the stage's state, not something the menu
                // keeps, and no item carries a tick.
                //
                // THE TRIGGER SAYS THE WIDTH IN FORCE, in words rather than a
                // glyph. There is no icon for *width* in `spec/icons.toml`, and
                // reusing `columns` would give one drawing two meanings — which
                // that file refuses in so many words. The token is also the
                // thing a reader wants to see without opening anything.
                Button {
                    class: "nd-frame-width",
                    menu: true,
                    controls: "{name}-width",
                    variant: ButtonVariant::Transparent,
                    tone: ButtonTone::Neutral,
                    size: ButtonSize::Small,
                    label: context.t(Key::ExampleWidthFull),
                }
                Menu {
                    name: "{name}-width",
                    label: context.t(Key::ExampleWidthLabel),
                    for (value, width) in viewports() {
                        MenuItem {
                            value,
                            action: "frame-width",
                            // A LENGTH IS A NAME, NOT A SENTENCE. `320px` is the
                            // same string in every language, and the
                            // pseudo-locale scanner reads an unmarked run as
                            // one somebody forgot to translate. `Max` is prose
                            // and carries no mark, which is correct.
                            class: if width.is_some() { "nd-identifier" } else { "" },
                            label: match width {
                                Some(width) => width.to_owned(),
                                None => context.t(Key::ExampleWidthFull).to_owned(),
                            },
                        }
                    }
                }
                // THE SAME THEME MENU THE HEADER OPENS, ON THE SAME SURFACE,
                // THEMING THE STAGE.
                //
                // It shows only the settings a scope can carry. Blackout,
                // pattern and grain are `scope = false`, so they are absent
                // rather than present and dead.
                Disclosure {
                    class: "nd-frame-theme",
                    name: "frame-theme",
                    icon: IconDrawing::PALETTE,
                    summary: context.t(Key::SettingsLabel),
                    // NOT `noctua-glass`: what would read through this one is
                    // the examples it controls, in the appearance it has just
                    // given them. The material is 78% surface, so 22% of a
                    // stage the reader just turned dark would come through a
                    // panel that is still light.
                    div { class: "nd-panel",
                        div { class: "nd-panel-body",
                            ThemeMenu { context, name: "{name}", reach: Reach::Scope }
                        }
                    }
                }
                // THE GLASS TEST. The material is only visible as what is
                // behind it, and "behind it" on a documentation page is a flat
                // surface — so every glass example read as a tint and the five
                // tokens that tune it were untestable by eye. Pressing this
                // puts colour and hard edges behind the stage.
                Button {
                    action: "frame-glass",
                    variant: ButtonVariant::Transparent,
                    tone: ButtonTone::Neutral,
                    size: ButtonSize::Small,
                    hide_label: true,
                    icon: IconDrawing::FLASK_CONICAL,
                    label: context.t(Key::ExampleGlassTest),
                    label_alt: context.t(Key::ExampleGlassStop),
                }
                // The appearance toggle. Both glyphs and both names ship, and
                // the state picks one — see `Button.icon_alt`.
                Button {
                    action: "frame-mode",
                    variant: ButtonVariant::Transparent,
                    tone: ButtonTone::Neutral,
                    size: ButtonSize::Small,
                    hide_label: true,
                    icon: IconDrawing::MOON,
                    icon_alt: IconDrawing::SUN,
                    label: context.t(Key::ExampleModeInvert),
                    label_alt: context.t(Key::ExampleModeRestore),
                }
                // IT HAS A GLYPH NOW, and the rail is half the width it was.
                //
                // This kept its word for one stage because `spec/icons.toml`
                // declared nothing meaning *put it back*, and giving `close` or
                // `chevron-left` a second meaning is what that file refuses. So
                // the word set the rail's width — one 79px control among four
                // 32px ones, which is what made the strip look ragged and wide.
                // `rotate-ccw` was vendored for it: an arrow that comes back
                // round to where it started, which is the gesture, and which no
                // other icon in the set is already using.
                if mountable {
                    // AN ICON, NOT A TWO-STOP STRIP.
                    //
                    // It was a `Segmented` with `Dioxus` and `React` in it, and
                    // measured on the built page that cost **70px of height on
                    // every frame** — 260 against a median of 189. The rail
                    // exists precisely so the controls spend the width no frame
                    // was using rather than the height every frame is short of,
                    // and a two-row control in it spends the wrong one.
                    //
                    // A toggle carries both names and `aria-pressed` says which
                    // is showing, which is what the four buttons under it
                    // already do and what the page-wide control in the header
                    // does.
                    Button {
                        class: "nd-frame-impl",
                        action: "frame-impl",
                        variant: ButtonVariant::Transparent,
                        tone: ButtonTone::Neutral,
                        size: ButtonSize::Small,
                        hide_label: true,
                        // THE GLYPH IS THE DESTINATION, NOT THE STATE.
                        //
                        // A toggle that drew what it is showing would ask the
                        // reader to work out what pressing it does. `Button`
                        // ships both glyphs and lets the state pick one, so the
                        // control reads as "go to React" and then "come back to
                        // Dioxus" with no script involved.
                        //
                        // `frame-mode` beside it is the same shape, with a moon
                        // and a sun.
                        icon: IconDrawing::ATOM,
                        icon_alt: IconDrawing::DNA,
                        label: context.t(Key::ExampleImplFrameReact),
                        label_alt: context.t(Key::ExampleImplFrameDioxus),
                        pressed: false,
                    }
                }
                Button {
                    action: "frame-reset",
                    variant: ButtonVariant::Transparent,
                    tone: ButtonTone::Neutral,
                    size: ButtonSize::Small,
                    hide_label: true,
                    icon: IconDrawing::ROTATE_CCW,
                    label: context.t(Key::ExampleReset),
                }
            }
            div {
                class: "nd-frame-stage",
                "data-nd-frame-stage": "",
                "data-nd-width": "full",
                // THE COLOUR, AND IT DOES NOT SCROLL.
                //
                // It sits on the STAGE rather than inside the scroller, so the
                // examples travel across it. A backdrop that scrolls with the
                // content shows a reader nothing a still screenshot would not,
                // and the whole question about glass is what happens while
                // something moves behind it.
                //
                // Rendered always and `display: none` until asked. The site
                // works with the script blocked, and a wrapper that is only
                // sometimes in the markup is a layout that is only sometimes
                // tested.
                div { class: "nd-glass-field", "aria-hidden": "true",
                    for shade in 0..GLASS_SHAPES {
                        span { class: "nd-glass-shape", "data-nd-shape": "{shade}" }
                    }
                }
                // `display: contents` at rest, so this is not a box until it
                // has to be one: the stage keeps its own flex column, its own
                // gap and its own padding exactly as before.
                div { class: "nd-glass-scroll", {children} }
            }
        }
    }
}

/// One example inside its own frame.
///
/// A function rather than three lines of `rsx!`, because a frame now needs two
/// things computed from the same pair of facts — whether there is a tree, and
/// what its key is — and `rsx!` has nowhere to put a `let`.
///
/// **The tree is what decides whether the switch is offered.** Not a prop set
/// by hand, not a list: a frame whose example has no tree cannot be mounted, so
/// it shows no control. That is the rule this file already states twice, kept
/// by construction rather than by remembering.
fn framed(
    context: Context,
    key: &str,
    label: Key,
    subject: &str,
    example: &crate::examples::Example,
    nodes: &'static [Node],
) -> Element {
    let switchable = crate::react_available() && !nodes.is_empty();
    rsx! {
        ExampleFrame { context, name: "{key}", label, mountable: switchable,
            Example {
                context,
                source: example.source,
                note: example.note,
                tree: switchable.then(|| key.to_owned()),
                react: switchable
                    .then(|| crate::serialise::jsx(nodes, subject, context))
                    .flatten(),
                {crate::trees::draw(nodes, context)}
            }
        }
    }
}

/// One example inside the frame: the call, the render, and a note.
///
/// The call is also the caption. `Blockquote { tone: Accent, … }` says what an
/// example shows more precisely than a sentence would, in every language at
/// once, and it is the thing a reader came to copy — so it is `<figcaption>`,
/// first in the DOM as well as first on the page.

#[component]
fn Example(
    context: Context,
    // NO `title`. There was one, for an example whose subject needed naming,
    // and its two callers were the spacing and typography pages. Both are a
    // stack of `<section>`s with a real `<h2>` each now, which is the shape
    // every other page has and the shape the page outline is read from — so a
    // heading inside the figure would be a second name for the same thing,
    // one level too deep.
    /// The Dioxus call that produced it.
    source: String,
    /// The React call.
    ///
    /// Hand-written for `ThemeScope`, which is not in `spec/components/`, and
    /// derived from the tree for everything else — see `serialise::jsx`.
    #[props(default)]
    react: Option<String>,
    /// Which example this is, as the browser asks for it.
    ///
    /// The key into `assets/examples-<locale>.json`, and the same string the
    /// frame uses for its own ids. Present exactly when there is a tree to
    /// mount, so the presence of the attribute IS the answer to whether this
    /// example can be shown in React. Nothing else has to be kept in step.
    #[props(default)]
    tree: Option<String>,
    /// A sentence, for the few examples the call does not explain by itself.
    #[props(default)]
    note: Option<Key>,
    children: Element,
) -> Element {
    let react_hook = react.as_ref().map(|_| String::new());
    rsx! {
        figure { class: "nd-example",
            figcaption { class: "nd-example-source",
                // THE SWITCHABLE WRAPPER ONLY WHERE THERE IS SOMETHING TO
                // SWITCH TO.
                //
                // Every example used to carry `data-nd-impl-source="Dioxus"`
                // whether or not it had a React half — a hook on a hundred and
                // thirty captions that meant something on one. It was invisible
                // while a frame held one example and ambiguous the moment one
                // held two.
                if let Some(react) = react {
                    div { "data-nd-impl-source": "Dioxus",
                        Code { class: "nd-identifier", tone: CodeTone::Subtle, "{source}" }
                    }
                    div { "data-nd-impl-source": "React", hidden: true,
                        Code { class: "nd-identifier", tone: CodeTone::Subtle, "{react}" }
                    }
                } else {
                    Code { class: "nd-identifier", tone: CodeTone::Subtle, "{source}" }
                }
            }
            div {
                class: "nd-example-stage",
                // WHICH example the React bundle replaces, said out loud.
                //
                // The mount used to take the first scope in the stage, which
                // was unambiguous while the frame held one example. The
                // ThemeScope page now holds two scoped ones and the first is
                // the live demonstration — so React would have replaced the
                // wrong one. The example that HAS a React call is the one that
                // can be mounted; nothing else needs to be known.
                "data-nd-mountable": react_hook,
                // WHICH tree draws it, for the browser to look up.
                "data-nd-example": tree,
                {children}
            }
            if let Some(note) = note {
                p { class: "nd-example-note nd-prose",
                    Message { context, message: note }
                }
            }
        }
    }
}

/// The call that produced the live demonstration, nested scope and all.
const NESTED_SOURCE: &str = r"ThemeScope {
    palette: Palette::AzureVivid, mode: Mode::Light,
    ThemeScope { palette: Palette::RoseSober, mode: Mode::Dark, … }
}";

/// The Dioxus call that produced the `ThemeScope` example.
///
/// A literal, and deliberately: it is the source a reader would copy, and
/// deriving it from the props would produce something that renders the same
/// thing and reads like nothing anybody would write.
const DIOXUS_SOURCE: &str = r"ThemeScope {
    palette: Palette::AzureVivid,
    mode: Mode::Light,
    glass: Glass::Full,
    radius: Radius::Round,
    Sample {}
}";

// --- One message, with its identifiers set in code --------------------------

/// Renders a message, with its inline marks turned into real components.
///
/// | In the catalogue | On the page |
/// |---|---|
/// | `` `name` `` | `Code`, and an identifier |
/// | `**word**` | `Strong` |
/// | `*word*` | `Em` |
///
/// # This is not assembling a sentence from fragments
///
/// `NOCTUA.md §9.9` rule 1 forbids building one sentence out of several
/// lookups, because word order and agreement differ between languages. This is
/// the opposite: **one** message, one key, one translation — split for
/// rendering only, at marks the translator writes and controls.
///
/// Which is the whole argument for the marks existing. Where the emphasis falls
/// in a sentence is part of the sentence, and it moves when the sentence is
/// translated; a page that wrapped the third word in `Strong` because English
/// puts the important word third would emphasise the wrong word in Portuguese.
/// Handing the translator the marks hands them the decision.
///
/// A name inside backticks is an identifier, so it also gets `.nd-identifier`
/// and the pseudo-locale test skips it — the bracketed text around it is what
/// the test is looking at.
#[component]
pub(crate) fn Message(context: Context, message: Key) -> Element {
    let text = context.t(message);
    rsx! {
        // The wrapper is not decoration. Splitting at a mark leaves the
        // pseudo-locale's `⟦` on the first run and its `⟧` on the last, so a
        // run between two marks carries neither — and
        // `every_sentence_on_every_page_is_translated` would read it as text
        // that never went through the catalogue. This says *these runs are one
        // message*, and the test reads the subtree as one.
        span { class: "nd-message",
            for (index, run) in text.split('`').enumerate() {
                if index % 2 == 1 {
                    Code { class: "nd-identifier", tone: CodeTone::Subtle, "{run}" }
                } else {
                    Emphasis { run: run.to_owned() }
                }
            }
        }
    }
}

/// The `**` and `*` marks inside one run of a message.
///
/// Its own component because the two nest: `**` has to be found first, or the
/// first two asterisks of a bold word read as an empty italic one.
#[component]
fn Emphasis(run: String) -> Element {
    rsx! {
        for (bold, part) in marks(&run, "**") {
            if bold {
                Strong { "{part}" }
            } else {
                for (italic, piece) in marks(&part, "*") {
                    if italic {
                        Em { "{piece}" }
                    } else {
                        "{piece}"
                    }
                }
            }
        }
    }
}

/// Splits on a mark, saying of each piece whether it was inside one.
///
/// An unclosed mark leaves its text alone rather than swallowing the rest of
/// the sentence: a translator who typed one asterisk gets one asterisk, which
/// is visible and fixable, instead of a paragraph that silently went bold.
fn marks(text: &str, mark: &str) -> Vec<(bool, String)> {
    let parts: Vec<&str> = text.split(mark).collect();
    if parts.len() % 2 == 0 {
        return vec![(false, text.to_owned())];
    }
    parts
        .into_iter()
        .enumerate()
        .filter(|(_, part)| !part.is_empty())
        .map(|(index, part)| (index % 2 == 1, part.to_owned()))
        .collect()
}

// --- The front door ---------------------------------------------------------

/// The introduction, and the first entry in every locale's sidebar.
///
/// It is first because `Kind::Intro` sorts first, not because anything names
/// it: `site::first` reads the top of the sidebar, and the wordmark and the
/// root redirect both read `site::first`. So the front door and the top of the
/// navigation cannot disagree.
#[component]
fn Intro(context: Context) -> Element {
    rsx! {
        Prose {
            Paragraph { "{context.t(Key::PageIntroBody)}" }
        }
    }
}

// --- The one component whose page is written -------------------------------

/// `ThemeScope`, demonstrated rather than described.
///
/// # Why this page is written and every other component page is derived
///
/// Every other component has a `spec/components/<name>.toml` its page is built
/// from. This one cannot: its props **are** the scopable settings in
/// `spec/theme.toml`, and a second file listing them could only disagree with
/// the first. So the prop table is read from the theme model and the
/// demonstration is written.
///
/// It is also the only page with a rendered scope on it, which makes it what
/// the browser suite measures the whole theme layer against — a scope resolving
/// the semantic contract against itself, a nested scope keeping its parent's
/// palette, a border painting its own colour rather than the page's. Nothing
/// here is a screenshot: every box is the same markup as the page around it,
/// reading whichever theme reaches it.
#[component]
fn ThemeScopePage(context: Context) -> Element {
    rsx! {
        Paragraph { "{context.t(Key::ColorsScopeBody)}" }
        Paragraph { "{context.t(Key::ScopePageBody)}" }

        Heading { level: HeadingLevel::_2, size: HeadingSize::H2,
            "{context.t(Key::ColorsScopeTrapTitle)}"
        }
        Paragraph { "{context.t(Key::ColorsScopeTrapBody)}" }

        // ONE FRAME, NOT A BOX AND A FRAME SHOWING THE SAME THING.
        //
        // This page used to carry the demonstration twice: a hand-built
        // `.nd-scope-head` box with its own reset button near the top, and the
        // very same `azure-vivid / light / full / round` scope again in a real
        // frame further down. A reader who changed one wondered why the other
        // had not moved.
        //
        // It is one frame now, holding both examples — the live demonstration
        // with its nested scope, and the mountable one beside it. The frame's
        // own reset covers both, so the loose `scope-reset` button is gone too.
        Heading { level: HeadingLevel::_2, size: HeadingSize::H2,
            "{context.t(Key::ComponentExamples)}"
        }
        ExampleFrame {
            context,
            name: "scope",
            label: Key::ScopePageTitle,
            // The one hand-written switch left. `ThemeScope` comes from
            // `spec/theme.toml`, so it has no tree and no derived caption.
            // THE ONE FRAME THAT MOUNTED REACT, AND NOW ONE OF 596.
            //
            // `ThemeScope` comes from `spec/theme.toml` rather than
            // `spec/components/`, so the generated interpreter has no arm for
            // it and `tree::theme_scope` is written by hand. Without that this
            // frame lost its switch, and the React `ThemeScope` was rendered
            // nowhere — the exact condition that let six other React modules
            // ship broken under a green suite.
            mountable: crate::react_available(),
            // No heading on the figure: the scope inside it opens with an
            // `<h3>` carrying this very key, an inch below where the figure's
            // own would have been.
            Example {
                context,
                source: NESTED_SOURCE,
                note: Key::ScopeNote,
                ThemeScope {
                    class: "nd-scope",
                    palette: Palette::AzureVivid,
                    mode: Mode::Light,
                    glass: Glass::Full,
                    radius: Radius::Round,

                    div { class: "nd-scope-head",
                        h3 { class: "nd-scope-title", "{context.t(Key::ScopeComponentTitle)}" }
                        // What this scope declares, one line above.
                        ScopeControls { context, mode: "light", radius: "round", glass: "full" }
                    }
                    p { class: "nd-prose", "{context.t(Key::ScopeComponentBody)}" }
                    Sample { context }

                    ThemeScope {
                        class: "nd-scope nd-scope--nested",
                        palette: Palette::RoseSober,
                        mode: Mode::Dark,
                        radius: Radius::Circular,

                        div { class: "nd-scope-head",
                            h4 { class: "nd-scope-title", "{context.t(Key::ScopeNestedTitle)}" }
                            // What this one declares, and `full` because it
                            // declares no glass and inherits its parent's.
                            ScopeControls { context, mode: "dark", radius: "circular", glass: "full" }
                        }
                        p { class: "nd-prose", "{context.t(Key::ScopeNestedBody)}" }
                        Sample { context }
                    }
                }
            }
            // The one example the React bundle can mount. Two packages
            // generated from one table have identical prop surfaces by
            // construction; that they BEHAVE identically is a different claim,
            // and this is the only place it is shown rather than asserted
            // (`NOCTUA.md §9.3` Leg 4).
            Example {
                context,
                source: DIOXUS_SOURCE,
                tree: crate::react_available().then(|| "scope-0".to_owned()),
                react: crate::serialise::jsx(crate::trees::SCOPE, "ThemeScope", context),
                {crate::tree::render(crate::trees::SCOPE, crate::tree::Scope::new(context))}
            }
        }

        Heading { level: HeadingLevel::_2, size: HeadingSize::H2,
            "{context.t(Key::ComponentProps)}"
        }
        WrittenProps { context, name: "theme-scope", props: &crate::props::SCOPABLE }
    }
}

/// The controls a scope carries, which look exactly like the page's.
///
/// They are bound to the *scope element*, which the script finds by walking up
/// from the control. So the same markup drives the page, a component and a
/// component inside that one, with no per-instance wiring — and the rule it
/// relies on is one sentence: **a group inside a themed scope themes that
/// scope.** Nothing here says which scope; where it sits is the answer.
///
/// # These were four cycling buttons, and that was the bug
///
/// `shell.rs` argues at length, two hundred lines above the header, that a
/// cycling control has to show the *current* state to be usable — and then
/// this page carried four that showed a single static glyph each. A reader
/// could not tell which mode, accent, corner or glass level a scope was in;
/// pressing one moved it to the next value, whatever that was, with no way
/// back except the reset. They are `Segmented` groups now, so the state is
/// visible before anything is pressed, which is the whole reason the header
/// control is three radios rather than one cycling icon.
/// # And they state what is in force before any script runs
///
/// All three shipped with `aria-checked="false"` on every item, and `refresh()`
/// marked the right one once the module had loaded. That is exactly the defect
/// the accent swatches were fixed for: a radio group that announces nothing
/// chosen, to a reader with no script or to anything measuring the page before
/// it runs — while the scope around it plainly IS light, round and frosted.
/// The scope declares those three where it is built, so they are passed in and
/// the answer is rendered.
#[component]
fn ScopeControls(
    context: Context,
    /// What the scope this sits in has in force, and therefore which item is
    /// marked. A nested scope passes what it inherited where it declared
    /// nothing of its own.
    mode: &'static str,
    radius: &'static str,
    glass: &'static str,
) -> Element {
    rsx! {
        div { class: "nd-scope-controls",
            Segmented {
                name: "mode",
                size: SegmentedSize::Small,
                label: context.t(Key::ThemeModeLabel),
                for (value, icon, label) in [
                    ("light", IconDrawing::SUN, Key::ThemeModeLight),
                    ("dark", IconDrawing::MOON, Key::ThemeModeDark),
                    ("system", IconDrawing::MONITOR, Key::ThemeModeSystem),
                ] {
                    SegmentedItem {
                        value,
                        icon,
                        hide_label: true,
                        selected: value == mode,
                        label: context.t(label),
                    }
                }
            }
            Segmented {
                name: "radius",
                size: SegmentedSize::Small,
                label: context.t(Key::ThemeRadiusLabel),
                for (value, label) in [
                    ("square", Key::ThemeRadiusSquare),
                    ("soft", Key::ThemeRadiusSoft),
                    ("round", Key::ThemeRadiusRound),
                    ("circular", Key::ThemeRadiusCircular),
                ] {
                    SegmentedItem { value, selected: value == radius, label: context.t(label) }
                }
            }
            Segmented {
                name: "glass",
                size: SegmentedSize::Small,
                label: context.t(Key::ThemeGlassLabel),
                for (value, label) in [
                    ("off", Key::ThemeGlassOff),
                    ("subtle", Key::ThemeGlassSubtle),
                    ("full", Key::ThemeGlassFull),
                ] {
                    SegmentedItem { value, selected: value == glass, label: context.t(label) }
                }
            }
        }
    }
}

/// A heading, a paragraph, a link and two controls.
///
/// Every one of them is painted from the semantic contract — `--nc-color-fg`,
/// `--nc-color-accent`, `--nc-color-border` — so what it shows is whatever theme
/// resolved at the box it happens to be in. That is the assertion the page makes
/// visible: without `dist/scope.css` these would all be the page's colors.
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
                Button { label: context.t(Key::ScopeSampleAction) }
                Button {
                    variant: ButtonVariant::Outline,
                    tone: ButtonTone::Neutral,
                    label: context.t(Key::ScopeSampleQuiet),
                }
            }
        }
    }
}

// --- The frame, documenting itself ------------------------------------------

/// What the frame is, what it is made of, and what its four controls do.
///
/// Written rather than derived, like `ThemeScope`'s, and for the mirror-image
/// reason: that component has no contract file because its props ARE the theme
/// model, and this one has none because it is not a component the packages
/// ship. `NOCTUA.md §9.10` is explicit that the frame belongs to the site.
///
/// The demonstration is a real frame with a real example in it, so pressing
/// the appearance toggle on this page inverts that frame and leaves the
/// sentence above it alone — which is the claim, made rather than described.
#[component]
fn ExampleFramePage(context: Context) -> Element {
    rsx! {
        // ONE PARAGRAPH. This page had four headings, a four-item list and
        // three paragraphs explaining an apparatus a reader can simply press.
        // The controls document themselves — they carry names — and the frame
        // below is the demonstration.
        Paragraph {
            Message { context, message: Key::FramePageBody }
        }

        // A FRAME INSIDE A FRAME, and the one page where that is the subject.
        //
        // The outer one belongs to this page like every other; the inner one is
        // what the page is about. Pressing the outer toggle inverts both,
        // because the inner frame is inside the outer stage — which is the
        // isolation working, not failing.
        ExampleFrame { context, name: "frame", label: Key::ComponentExamples,
            Example { context, source: FRAME_SOURCE,
                // The inner one is what this page is about, and it needs its
                // own name for the same reason it needs its own everything.
                ExampleFrame { context, name: "frame-inner", label: Key::ComponentExamples,
                    Example { context, source: BUTTON_SOURCE,
                        div { class: "nd-example-row",
                            Button { label: context.t(Key::ExampleButtonSave) }
                            Button {
                                variant: ButtonVariant::Outline,
                                tone: ButtonTone::Neutral,
                                label: context.t(Key::ExampleButtonCancel),
                            }
                        }
                    }
                }
            }
        }

        Heading { level: HeadingLevel::_2, size: HeadingSize::H3,
            "{context.t(Key::FramePropsTitle)}"
        }
        WrittenProps { context, name: "example-frame", props: &crate::props::FRAME }

    }
}

/// The call inside the frame on this page.
///
/// A `const` rather than a literal in the `rsx!`, and that is not style: the
/// macro reads a string literal as a FORMAT string, so `{ label: … }` inside
/// one is an interpolation of a variable that does not exist.
const BUTTON_SOURCE: &str = r#"Button { label: "…" }"#;

/// The call that produced the frame on this page.
const FRAME_SOURCE: &str = r#"ExampleFrame {
    label: Key::ComponentExamples,
    Example { source: "…", Button { … } }
}"#;

// --- Spacing ----------------------------------------------------------------

/// The bar that shows a step, whose width IS the token it is labelled with.
///
/// `var(--noctua-space-{step})` rather than a computation that comes to the
/// same number: what the reader is looking at has to be the token, or the page
/// is a drawing of the scale rather than the scale. The interpolation is what
/// `gates::unknown_token` exempts by name — a `var()` whose property is
/// composed at runtime has nothing static to check, and its comment names this
/// page as the case.
#[component]
fn SpaceBar(step: String) -> Element {
    rsx! {
        span {
            class: "nd-bar",
            style: "inline-size: var(--noctua-space-{step})",
            "aria-hidden": "true",
        }
    }
}

/// One step's length, in the two units a reader needs.
///
/// Derived from `BASE` rather than written: the scale is stated once, in
/// `spec/tokens.toml`, and a page that repeated `0.25` would go on saying it
/// after somebody changed it. The pixel column assumes the browser's own
/// default root size, which is what this site runs at — nothing here sets
/// `font-size` on `html`, deliberately, so a reader who has enlarged their
/// text gets a larger scale rather than a broken one.
fn step_length(step: u32) -> (String, String) {
    let base: f64 = tokens::space::BASE
        .trim_end_matches("rem")
        .parse()
        .unwrap_or_default();
    let rem = base * f64::from(step);
    // Trailing zeros are noise in a column of numbers: `0.5rem`, not `0.50rem`.
    let trimmed = format!("{rem}");
    (format!("{trimmed}rem"), format!("{}", rem * ROOT_PX))
}

/// The root font size every browser ships with, and the one this site keeps.
///
/// A number rather than a token on purpose: it is a fact about browsers, not a
/// value this system chose, and putting it in `spec/tokens.toml` would claim
/// the opposite. It is only ever used to print a second column beside the rem
/// one — nothing is laid out from it.
const ROOT_PX: f64 = 16.0;

/// The three sections of the spacing page, and what goes in each.
///
/// One list, so the jump index above the page and the sections below it are
/// read from the same place and cannot fall out of step. The order is the
/// argument: the named gaps are what a component actually asks for, the raw
/// scale under them is the material, and the rhythm is the only claim of the
/// three that a table cannot make.
const SPACING_SECTIONS: [(&str, Key); 3] = [
    ("spacing-named", Key::SpacingNamedTitle),
    ("spacing-scale", Key::SpacingScaleTitle),
    ("spacing-rhythm", Key::SpacingRhythmTitle),
];

/// The named gaps, the raw scale, and the rhythm they make.
///
/// # Why the named gaps come first
///
/// They are what a component actually asks for. `space::GAP` says *between
/// sibling controls*; `--noctua-space-4` says *four*, and a reader who reaches
/// for the number has to decide what four means every time. The scale below
/// them is the raw material, and it is second because it is the answer to a
/// rarer question.
///
/// # The same shape the typography page has
///
/// A section, an `<h2>`, one frame, one demonstration, and the sentence about
/// it under the stage. The page was frameless for one revision and read as a
/// different site: the tables ran the full width of the column while every
/// other demonstration on the site sits in a stage with a rail beside it, and
/// nothing here could be looked at at 320px without resizing the window.
#[component]
fn Spacing(context: Context) -> Element {
    let marks = SPACING_SECTIONS
        .iter()
        .map(|(id, heading)| Jump::section((*id).to_owned(), context.t(*heading)))
        .collect();
    rsx! {
        DocIndex { context, marks }

        section { class: "nd-doc-section", id: "{SPACING_SECTIONS[0].0}",
            Heading { level: HeadingLevel::_2, size: HeadingSize::H2,
                "{context.t(Key::SpacingNamedTitle)}"
            }
            ExampleFrame { context, name: "spacing-named", label: Key::SpacingNamedTitle,
                Example { context, source: NAMED_SOURCE, note: Key::SpacingNamedBody,
                    NamedGaps { context }
                }
            }
        }

        section { class: "nd-doc-section", id: "{SPACING_SECTIONS[1].0}",
            Heading { level: HeadingLevel::_2, size: HeadingSize::H2,
                "{context.t(Key::SpacingScaleTitle)}"
            }
            ExampleFrame { context, name: "spacing-scale", label: Key::SpacingScaleTitle,
                Example { context, source: SCALE_STEP_SOURCE, note: Key::SpacingScaleBody,
                    RawScale { context }
                }
            }
        }

        section { class: "nd-doc-section", id: "{SPACING_SECTIONS[2].0}",
            Heading { level: HeadingLevel::_2, size: HeadingSize::H2,
                "{context.t(Key::SpacingRhythmTitle)}"
            }
            ExampleFrame { context, name: "spacing-rhythm", label: Key::SpacingRhythmTitle,
                Example { context, source: RHYTHM_SOURCE, note: Key::SpacingRhythmBody,
                    Rhythm { context }
                }
            }
        }
    }
}

/// The three calls the spacing page's demonstrations were produced by.
///
/// Each one names the generated list the demonstration above it LOOPS, rather
/// than a sample of what is in it. The tables read `tokens::space::NAMED` and
/// `tokens::space::STEPS`, so those are the calls, and a reader who follows
/// either reaches the whole scale rather than the one step a sample chose.
const NAMED_SOURCE: &str = "noctua_design_tokens::space::NAMED";
const SCALE_STEP_SOURCE: &str = "noctua_design_tokens::space::STEPS";
const RHYTHM_SOURCE: &str = "space::HEADING · space::PARAGRAPH · space::SECTION";

/// The seven gaps a component asks for by purpose.
#[component]
fn NamedGaps(context: Context) -> Element {
    rsx! {
        Table {
            name: "spacing-named",
            caption: context.t(Key::SpacingNamedTitle),
            hide_caption: true,
            // THE WIDTHS ARE THE COLUMNS' OWN. `narrow` is what the largest
            // named gap measures, so the bar never wraps; the sentence gets the
            // rest. They were five `width` props on five headings before, which
            // is the arrangement `TableColumn` exists to replace.
            TableColumn { name: "token" }
            TableColumn { name: "step", width: TableColumnWidth::Fit }
            TableColumn { name: "rem", width: TableColumnWidth::Fit }
            TableColumn { name: "size", width: TableColumnWidth::Narrow }
            TableColumn { name: "for", width: TableColumnWidth::Wide }
            TableSection { kind: TableSectionKind::Head,
                TableRow {
                    TableHeaderCell { "{context.t(Key::SpacingColumnToken)}" }
                    TableHeaderCell {
                        align: TableHeaderCellAlign::Numeric,
                        "{context.t(Key::SpacingColumnStep)}"
                    }
                    TableHeaderCell {
                        align: TableHeaderCellAlign::Numeric,
                        "{context.t(Key::SpacingColumnRem)}"
                    }
                    TableHeaderCell { column: "size", "{context.t(Key::SpacingColumnSize)}" }
                    TableHeaderCell { column: "for", "{context.t(Key::SpacingColumnFor)}" }
                }
            }
            TableSection {
                // READ FROM THE GENERATED SCALE, never listed here. The page
                // this replaces carried its own copy of these seven names and
                // of the twelve steps below, which is two second copies of
                // `spec/tokens.toml`.
                for (name, step) in tokens::space::NAMED {
                    TableRow {
                        TableHeaderCell {
                            scope: TableHeaderCellScope::Row,
                            wrap: TableHeaderCellWrap::Nowrap,
                            Code { class: "nd-identifier", "--noctua-space-{name}" }
                        }
                        TableCell {
                            align: TableCellAlign::Numeric,
                            label: context.t(Key::SpacingColumnStep),
                            Code { "{step}" }
                        }
                        TableCell {
                            align: TableCellAlign::Numeric,
                            label: context.t(Key::SpacingColumnRem),
                            // A LENGTH IS A VALUE, so it is set as one. It is
                            // also what keeps `0.5rem` out of the pseudo-locale
                            // scan: `<code>` is a run of identifier, not a
                            // sentence somebody forgot to translate.
                            Code { "{step_length(step).0}" }
                        }
                        TableCell { label: context.t(Key::SpacingColumnSize),
                            SpaceBar { step: name }
                        }
                        TableCell { label: context.t(Key::SpacingColumnFor),
                            "{context.t(named_purpose(name))}"
                        }
                    }
                }
            }
        }
    }
}

/// What each named gap is for, in the reader's language.
///
/// The one thing about this scale a machine cannot derive: `spec/tokens.toml`
/// carries the sentence as a comment beside each value, and a comment is not a
/// message. The `match` is exhaustive against the generated list — a name added
/// upstream with no sentence here is a panic at build time rather than a blank
/// cell on the page.
fn named_purpose(name: &str) -> Key {
    match name {
        "tight" => Key::SpacingTight,
        "inset" => Key::SpacingInset,
        "gap" => Key::SpacingGap,
        "paragraph" => Key::SpacingParagraph,
        "heading" => Key::SpacingHeading,
        "section" => Key::SpacingSection,
        "region" => Key::SpacingRegion,
        other => unreachable!("the spacing scale grew a gap with no sentence: {other}"),
    }
}

/// Every step, and what it comes to.
#[component]
fn RawScale(context: Context) -> Element {
    rsx! {
        Table {
            name: "spacing-scale",
            caption: context.t(Key::SpacingScaleTitle),
            hide_caption: true,
            TableColumn { name: "token" }
            TableColumn { name: "rem", width: TableColumnWidth::Fit }
            TableColumn { name: "pixels", width: TableColumnWidth::Fit }
            TableColumn { name: "size", width: TableColumnWidth::Wide }
            TableSection { kind: TableSectionKind::Head,
                TableRow {
                    TableHeaderCell { "{context.t(Key::SpacingColumnToken)}" }
                    TableHeaderCell {
                        align: TableHeaderCellAlign::Numeric,
                        "{context.t(Key::SpacingColumnRem)}"
                    }
                    TableHeaderCell {
                        align: TableHeaderCellAlign::Numeric,
                        "{context.t(Key::SpacingColumnPixels)}"
                    }
                    TableHeaderCell { column: "size", "{context.t(Key::SpacingColumnSize)}" }
                }
            }
            TableSection {
                for step in tokens::space::STEPS {
                    TableRow {
                        TableHeaderCell {
                            scope: TableHeaderCellScope::Row,
                            wrap: TableHeaderCellWrap::Nowrap,
                            Code { class: "nd-identifier", "--noctua-space-{step}" }
                        }
                        TableCell {
                            align: TableCellAlign::Numeric,
                            label: context.t(Key::SpacingColumnRem),
                            // A LENGTH IS A VALUE, so it is set as one. It is
                            // also what keeps `0.5rem` out of the pseudo-locale
                            // scan: `<code>` is a run of identifier, not a
                            // sentence somebody forgot to translate.
                            Code { "{step_length(step).0}" }
                        }
                        TableCell {
                            align: TableCellAlign::Numeric,
                            label: context.t(Key::SpacingColumnPixels),
                            Code { "{step_length(step).1}" }
                        }
                        TableCell { label: context.t(Key::SpacingColumnSize),
                            SpaceBar { step: step.to_string() }
                        }
                    }
                }
            }
        }
    }
}

/// The named gaps doing their job, with each one drawn where it falls.
///
/// A scale is a list of numbers until somebody stacks them. This is the page's
/// only claim that cannot be made in a table: that the gaps are *different
/// enough* to read as a hierarchy, and that a heading and its paragraph belong
/// together while two sections do not.
///
/// # Two columns, because that is how this site names a thing
///
/// It was one column of full-width bands with `--noctua-space-heading` set in
/// mono, right-aligned, floating on an accent wash — a way of labelling that
/// appears nowhere else here and read as a defect rather than as a
/// demonstration. The type scale beside it puts the name in its own track and
/// the specimen in the next, and so does every table on the page: a name
/// column and a value column. This is that grid.
///
/// The names are short for the same reason `.nd-scale-name` says `display`
/// rather than `--noctua-size-display`. The table above gives the custom
/// property in full, twice, and a 24-character name in a gutter would take a
/// third of a phone's width to repeat it.
#[component]
fn Rhythm(context: Context) -> Element {
    rsx! {
        div { class: "nd-rhythm",
            div { class: "nd-rhythm-block",
                Heading { level: HeadingLevel::_3, size: HeadingSize::H4,
                    "{context.t(Key::SpacingRhythmHeading)}"
                }
            }
            RhythmGap { name: "heading" }
            div { class: "nd-rhythm-block",
                Paragraph { size: ParagraphSize::Small, "{context.t(Key::SpacingRhythmFirst)}" }
            }
            RhythmGap { name: "paragraph" }
            div { class: "nd-rhythm-block",
                Paragraph { size: ParagraphSize::Small, "{context.t(Key::SpacingRhythmSecond)}" }
            }
            RhythmGap { name: "section" }
            div { class: "nd-rhythm-block",
                Heading { level: HeadingLevel::_3, size: HeadingSize::H4,
                    "{context.t(Key::SpacingRhythmNext)}"
                }
            }
            RhythmGap { name: "heading" }
            div { class: "nd-rhythm-block",
                Paragraph { size: ParagraphSize::Small, "{context.t(Key::SpacingRhythmThird)}" }
            }
        }
    }
}

/// One gap in the rhythm: its name in the first track, the gap itself in the
/// second, at the height the token really is.
///
/// TWO ROOTS, ON PURPOSE. They are direct children of the `.nd-rhythm` grid, so
/// the name lands in the name column and the band in the content column —
/// wrapping them in a box of their own would put both in one cell and undo the
/// alignment this exists for.
///
/// `Code`, like `.nd-scale-name` beside it, rather than the bare span this used
/// to be. The span carried `nd-identifier` by hand to tell the pseudo-locale
/// scan that a token name is not a sentence somebody forgot to translate, and
/// `<code>` says that already.
#[component]
fn RhythmGap(name: String) -> Element {
    rsx! {
        Code { class: "nd-rhythm-gap-name", "{name}" }
        div {
            class: "nd-rhythm-gap",
            style: "block-size: var(--noctua-space-{name})",
            "aria-hidden": "true",
        }
    }
}

// `inset` and `gap` drawn on two real `Button`s were a fourth demonstration
// here. The distinction they made — the space INSIDE a control against the
// space BETWEEN two — is a row each in the table above, in the same words, and
// a pair of buttons on a page about a scale is a picture of `ButtonGroup`.

// --- The contract tables, which four pages used to write out by hand ---------

// THIS IS THE COMPONENT'S FIRST CONSUMER, AND ITS REAL TEST.
//
// There were eight hand-written `<table class="nd-table">` blocks in this file
// and only three shapes among them — props, states, keyboard — with the whole
// triplet duplicated between `ComponentBody`'s two arms because a collection
// page nests one heading deeper. They are three components now, used seven
// times, and the site's own reference gains what it never had: a caption, a
// `scope` on every heading, the ARIA roles a stacked table needs, and a 320px
// reading that is a run of label-and-value pairs rather than a drag per row.

/// Every prop of a component, from its contract.
#[component]
fn ContractProps(
    context: Context,
    component: crate::generated::catalogue::DocComponent,
) -> Element {
    rsx! {
        Table {
            name: "{component.slug}-props",
            caption: context.t(Key::ComponentProps),
            // The heading above already says the word; the caption is the
            // table's NAME rather than a second copy of it.
            hide_caption: true,
            TableSection { kind: TableSectionKind::Head,
                TableRow {
                    TableHeaderCell { "{context.t(Key::ComponentColumnName)}" }
                    TableHeaderCell { "{context.t(Key::ComponentColumnType)}" }
                    TableHeaderCell { "{context.t(Key::ComponentColumnDefault)}" }
                    TableHeaderCell { "{context.t(Key::ComponentColumnEffect)}" }
                }
            }
            TableSection {
                for prop in component.props {
                    TableRow {
                        // The prop's name heads its own row, which is what
                        // makes each stacked record a titled run rather than
                        // four unlabelled values.
                        TableHeaderCell { scope: TableHeaderCellScope::Row,
                            Code { "{prop.name}" }
                        }
                        TableCell { label: context.t(Key::ComponentColumnType),
                            if prop.values.is_empty() {
                                Code { class: "nd-type", "{prop.kind}" }
                            } else {
                                for value in prop.values {
                                    Code { class: "nd-type", "{value}" }
                                }
                            }
                        }
                        TableCell { label: context.t(Key::ComponentColumnDefault),
                            if prop.required {
                                Text { tone: TextTone::Accent, "{context.t(Key::ComponentRequired)}" }
                            } else if let Some(default) = prop.default {
                                Code { "{default}" }
                            } else {
                                "{context.t(Key::ComponentNoDefault)}"
                            }
                        }
                        TableCell { label: context.t(Key::ComponentColumnEffect),
                            Message { context, message: prop.doc }
                        }
                    }
                }
            }
        }
    }
}

/// Every state a component can be in that is not a prop.
#[component]
fn ContractStates(
    context: Context,
    component: crate::generated::catalogue::DocComponent,
) -> Element {
    rsx! {
        Table {
            name: "{component.slug}-states",
            caption: context.t(Key::ComponentStates),
            hide_caption: true,
            TableSection { kind: TableSectionKind::Head,
                TableRow {
                    TableHeaderCell { "{context.t(Key::ComponentColumnName)}" }
                    TableHeaderCell { "{context.t(Key::ComponentColumnAttribute)}" }
                    TableHeaderCell { "{context.t(Key::ComponentColumnEffect)}" }
                }
            }
            TableSection {
                for state in component.states {
                    TableRow {
                        TableHeaderCell { scope: TableHeaderCellScope::Row,
                            Code { "{state.name}" }
                        }
                        TableCell { label: context.t(Key::ComponentColumnAttribute),
                            Code { "{state.attribute}" }
                        }
                        TableCell { label: context.t(Key::ComponentColumnEffect),
                            Message { context, message: state.doc }
                        }
                    }
                }
            }
        }
    }
}

/// What each key does. The one contract table with no heading row: both cells
/// are the answer, and a heading over `Enter` would be a word for the sake of
/// having one.
#[component]
fn ContractKeyboard(
    context: Context,
    component: crate::generated::catalogue::DocComponent,
) -> Element {
    rsx! {
        Table {
            name: "{component.slug}-keyboard",
            caption: context.t(Key::ComponentKeyboard),
            hide_caption: true,
            TableSection {
                for key in component.keyboard {
                    TableRow {
                        TableHeaderCell {
                            scope: TableHeaderCellScope::Row,
                            wrap: TableHeaderCellWrap::Nowrap,
                            Kbd { class: "nd-identifier", "{context.t(key.key)}" }
                        }
                        TableCell {
                            Message { context, message: key.action }
                        }
                    }
                }
            }
        }
    }
}

/// `ThemeScope`'s props and the example frame's, which are the same shape as
/// each other and not the same shape as a contract's.
#[component]
fn WrittenProps(context: Context, name: String, props: &'static [crate::props::Prop]) -> Element {
    rsx! {
        Table {
            name: "{name}-props",
            caption: context.t(Key::ComponentProps),
            hide_caption: true,
            TableSection { kind: TableSectionKind::Head,
                TableRow {
                    TableHeaderCell { "{context.t(Key::ComponentColumnName)}" }
                    TableHeaderCell { "{context.t(Key::ComponentColumnType)}" }
                    TableHeaderCell { "{context.t(Key::ComponentColumnDefault)}" }
                    TableHeaderCell { "{context.t(Key::ComponentColumnEffect)}" }
                }
            }
            TableSection {
                for prop in props {
                    TableRow {
                        TableHeaderCell { scope: TableHeaderCellScope::Row,
                            Code { class: "nd-identifier", "{prop.name}" }
                        }
                        TableCell { label: context.t(Key::ComponentColumnType),
                            Code { class: "nd-type nd-identifier", "{prop.type_name}" }
                        }
                        TableCell { label: context.t(Key::ComponentColumnDefault),
                            Code { class: "nd-identifier", "{prop.default}" }
                        }
                        TableCell { label: context.t(Key::ComponentColumnEffect),
                            "{context.t(prop.values)}"
                        }
                    }
                }
            }
        }
    }
}

// --- One page per component, from the contract ------------------------------

/// Every component documented on one page, with an index that jumps to them.
///
/// WHICH components those are is read from the catalogue — each contract says
/// `page = "inline-text"` — so an eleventh mark added to `spec/components/`
/// appears here, in the index and in the sidebar's count, with nothing here to
/// edit.
#[component]
fn CollectionPage(context: Context) -> Element {
    let components = site::documented_on(context.page);
    // The lead and the index heading are the COLLECTION's, read from the same
    // constant that says the page exists. A `match` on the slug here would be a
    // second list of collections, and two lists of one thing disagree.
    let collection = site::collection(context.page).expect("a collection page is a collection");
    // THE INDEX IS THE PARTS, NOT THEIR SECTIONS. Six components each carrying
    // four sections is twenty-four links under a heading called *On this page*,
    // which is not an outline of the page — it is the page again, in small.
    let marks: Vec<Jump> = components
        .iter()
        .map(|component| Jump::component(component.slug, component.name))
        .collect();
    rsx! {
        DocIndex { context, marks }

        Paragraph { class: "nd-prose",
            Message { context, message: collection.body }
        }

        // ONE FOLD PER PART, CLOSED. `/tables/` was fifty-seven thousand pixels
        // — sixty-three screens — because six whole references were stacked end
        // to end, and a reader who came for `TableCell` scrolled past five
        // other components' examples, grids and contracts to reach it. Folded,
        // the page is its hero and a list of its six parts, and the reference a
        // reader came for is one press away instead of forty scrolls.
        for component in components {
            section { class: "nd-doc-section", id: "{component.slug}",
                Disclosure {
                    class: "nd-doc-part",
                    // An `<h2>` inside the summary: closed, the name is the
                    // only thing left of this component, and a page whose
                    // outline has six sections must still have six headings.
                    heading: DisclosureHeading::H2,
                    summary: noctua_design_i18n::fill(
                        context.t(Key::ComponentPart),
                        &[
                            ("name", component.name),
                            (
                                "count",
                                &crate::examples::for_component(component.name).len().to_string(),
                            ),
                        ],
                    ),
                    Paragraph { class: "nd-prose", tone: ParagraphTone::Muted,
                        "{context.t(component.summary)}"
                    }
                    ComponentSections { context, component, depth: Depth::Part }
                }
            }
        }
    }
}

/// The page order of the lineage sections, and what each is called — one
/// list, so the jump nav and the sections below it can never disagree.
const LINEAGE_SECTIONS: &[(&str, Key)] = &[
    ("lineage-bonds", Key::PageLineageSectionBonds),
    ("lineage-space", Key::PageLineageSectionSpace),
    ("lineage-acting", Key::PageLineageSectionActing),
    ("lineage-live", Key::PageLineageSectionLive),
    ("lineage-placed", Key::PageLineageSectionPlaced),
    ("lineage-models", Key::PageLineageSectionModels),
    ("lineage-variants", Key::PageLineageSectionVariants),
    ("lineage-contracts", Key::PageLineageSectionContracts),
];

/// The frames for a RANGE of one component's examples.
///
/// The lineage page tells its examples in sections, so it pulls them by
/// range — and the array order in `examples.rs` IS the page order, which the
/// comment on each array states. The frame names keep the `{slug}-{index}`
/// shape every other page uses.
fn lineage_frames(
    context: Context,
    name: &'static str,
    slug: &'static str,
    range: std::ops::Range<usize>,
) -> Element {
    let all = crate::examples::for_component(name);
    rsx! {
        for index in range {
            {framed(
                context,
                &format!("{slug}-{index}"),
                Key::ComponentExamples,
                name,
                &all[index],
                crate::trees::nodes(name, index),
            )}
        }
    }
}

/// One component's contract — the tables without the examples, for the
/// lineage page's closing section.
///
/// It is `ReferenceBlock` like every other page's contract now, rather than a
/// third hand-written copy of the same four blocks. The name above it stays,
/// because this section holds two contracts and the folds inside them would
/// otherwise be eight controls in a row with nothing saying which is whose.
fn lineage_contract(
    context: Context,
    component: crate::generated::catalogue::DocComponent,
) -> Element {
    rsx! {
        Heading { level: HeadingLevel::_3, size: HeadingSize::H3, class: "nd-identifier", "{component.name}" }
        ReferenceBlock { context, component, depth: Depth::Part }
    }
}

/// The lineage collection, told as one story.
///
/// The generic collection page stacks two whole references, and here that
/// meant the figure — the half a consumer touches first — stood behind the
/// scene's twenty-three frames, eleven grids and its entire contract; and
/// frames in declaration order put near-identical scenes side by side, which
/// the reader called repetition. This page keeps every example, every derived
/// variant and both contracts, and changes only the ORDER: the scene, then
/// the figure while the first scene is still on screen, then the features in
/// themed sections, with the reference tables at the end where a reader who
/// wants tables looks for them.
#[component]
fn LineagePage(context: Context) -> Element {
    let components = site::documented_on(context.page);
    let scene = components[0];
    let figure = components[1];
    assert!(
        scene.slug == "lineage" && figure.slug == "lineage-figure",
        "the lineage page tells exactly these two components"
    );
    let collection = site::collection(context.page).expect("the lineage page is a collection");
    // The told order, as the same jump index every other page carries.
    let mut marks = vec![
        Jump::component("lineage", "Lineage"),
        Jump::component("lineage-figure", "LineageFigure"),
    ];
    marks.extend(
        LINEAGE_SECTIONS
            .iter()
            .map(|(id, heading)| Jump::section((*id).to_owned(), context.t(*heading))),
    );
    rsx! {
        DocIndex { context, marks }

        Paragraph { class: "nd-prose",
            Message { context, message: collection.body }
        }

        // The scene: a graph in, a drawing out — and what an empty graph says.
        section { class: "nd-doc-section", id: "lineage",
            Heading { level: HeadingLevel::_2, size: HeadingSize::H2, class: "nd-identifier", "Lineage" }
            // No summary paragraph: the page lead above says it, and saying
            // it twice in two inches was the wall the reader reported.
            if let Some(guidance) = scene.guidance {
                Paragraph {
                    Message { context, message: guidance }
                }
            }
            {lineage_frames(context, "Lineage", "lineage", 0..2)}
        }

        // The figure, while the first scene is still on screen — it is the
        // half a consumer touches first, and it used to stand behind the
        // scene's whole reference.
        section { class: "nd-doc-section", id: "lineage-figure",
            Heading { level: HeadingLevel::_2, size: HeadingSize::H2, class: "nd-identifier", "LineageFigure" }
            // No summary paragraph here either — the guidance opens with the
            // same sentence.
            if let Some(guidance) = figure.guidance {
                Paragraph {
                    Message { context, message: guidance }
                }
            }
            {lineage_frames(context, "LineageFigure", "lineage-figure", 0..4)}
            // The founders close the section: figures at every slot, and the
            // scene's own parallax over them.
            {lineage_frames(context, "Lineage", "lineage", 2..3)}
        }

        section { class: "nd-doc-section", id: "lineage-bonds",
            Heading { level: HeadingLevel::_2, size: HeadingSize::H3, "{context.t(Key::PageLineageSectionBonds)}" }
            {lineage_frames(context, "Lineage", "lineage", 3..5)}
        }

        section { class: "nd-doc-section", id: "lineage-space",
            Heading { level: HeadingLevel::_2, size: HeadingSize::H3, "{context.t(Key::PageLineageSectionSpace)}" }
            {lineage_frames(context, "Lineage", "lineage", 5..9)}
        }

        section { class: "nd-doc-section", id: "lineage-acting",
            Heading { level: HeadingLevel::_2, size: HeadingSize::H3, "{context.t(Key::PageLineageSectionActing)}" }
            {lineage_frames(context, "Lineage", "lineage", 9..11)}
        }

        section { class: "nd-doc-section", id: "lineage-live",
            Heading { level: HeadingLevel::_2, size: HeadingSize::H3, "{context.t(Key::PageLineageSectionLive)}" }
            {lineage_frames(context, "Lineage", "lineage", 11..13)}
        }

        section { class: "nd-doc-section", id: "lineage-placed",
            Heading { level: HeadingLevel::_2, size: HeadingSize::H3, "{context.t(Key::PageLineageSectionPlaced)}" }
            {lineage_frames(context, "Lineage", "lineage", 13..15)}
        }

        // The models: the two scenes the browser suite holds by name.
        section { class: "nd-doc-section", id: "lineage-models",
            Heading { level: HeadingLevel::_2, size: HeadingSize::H3, "{context.t(Key::PageLineageSectionModels)}" }
            {lineage_frames(context, "Lineage", "lineage", 15..17)}
        }

        // Every value of every enum prop, derived from both contracts — so a
        // variant cannot be forgotten and cannot disagree with the component.
        section { class: "nd-doc-section", id: "lineage-variants",
            Heading { level: HeadingLevel::_2, size: HeadingSize::H3, "{context.t(Key::PageLineageSectionVariants)}" }
            for component in [scene, figure] {
                for prop in component.props.iter().filter(|prop| drawn(prop)) {
                    ExampleFrame {
                        context,
                        name: "{component.slug}-{prop.name}",
                        label: Key::ComponentExamples,
                        // A grid is a tree now, so it can be mounted like any other.
                        mountable: crate::react_available(),
                        Example {
                            context,
                            source: "{prop.name}",
                            tree: crate::react_available()
                                .then(|| format!("{}-{}", component.slug, prop.name)),
                            div { class: "nd-variant-row",
                                for value in prop.values {
                                    Specimen { context, component, prop: *prop, value: (*value).to_string() }
                                }
                            }
                        }
                    }
                }
            }
        }

        section { class: "nd-doc-section", id: "lineage-contracts",
            Heading { level: HeadingLevel::_2, size: HeadingSize::H3, "{context.t(Key::PageLineageSectionContracts)}" }
            {lineage_contract(context, scene)}
            {lineage_contract(context, figure)}
        }
    }
}

/// How deep a component's sections sit on the page that holds them.
///
/// This used to be a `bool` called `nested` and two copies of a hundred lines
/// that differed only in their heading levels — so a change to the section
/// order had to be made twice, and twice is how the two arms drifted apart.
#[derive(Clone, Copy, PartialEq, Eq)]
enum Depth {
    /// The component has the page to itself: its sections are the page's.
    Page,
    /// The component is one of several on a collection page, under its own name.
    Part,
}

impl Depth {
    /// The heading element a section title is.
    const fn level(self) -> HeadingLevel {
        match self {
            Self::Page => HeadingLevel::_2,
            Self::Part => HeadingLevel::_3,
        }
    }

    /// How big that title is drawn.
    const fn size(self) -> HeadingSize {
        match self {
            Self::Page => HeadingSize::H2,
            Self::Part => HeadingSize::H3,
        }
    }

    /// The heading a folded reference block carries, one step under a section.
    const fn folded(self) -> DisclosureHeading {
        match self {
            Self::Page => DisclosureHeading::H3,
            Self::Part => DisclosureHeading::H4,
        }
    }

    /// The heading a whole folded SECTION carries, level with an open one.
    ///
    /// `folded()` is for the four reference blocks, which sit one step under
    /// the section that holds them. The variant grid is not under anything: it
    /// is a peer of *Examples* and *Reference*, and folding it must not move it
    /// down the page's outline. A reader navigating by heading finds the same
    /// three either way.
    const fn section_fold(self) -> DisclosureHeading {
        match self {
            Self::Page => DisclosureHeading::H2,
            Self::Part => DisclosureHeading::H3,
        }
    }
}

/// The id one of a component's sections answers to.
///
/// Bare on a page of its own, and prefixed with the component's slug on a
/// collection page — where six components each have a section called
/// `examples` and an id may appear once.
fn section_id(
    component: crate::generated::catalogue::DocComponent,
    depth: Depth,
    part: &str,
) -> String {
    match depth {
        Depth::Page => part.to_owned(),
        Depth::Part => format!("{}-{part}", component.slug),
    }
}

/// Whether a component has any variant grid to show.
fn has_variants(component: crate::generated::catalogue::DocComponent) -> bool {
    component.props.iter().any(drawn)
}

/// What a component's own sections are called, for the page's jump index.
fn component_marks(
    context: Context,
    component: crate::generated::catalogue::DocComponent,
    depth: Depth,
) -> Vec<Jump> {
    let mut marks = Vec::new();
    if component.guidance.is_some() {
        marks.push(Jump::section(
            section_id(component, depth, "when"),
            context.t(Key::ComponentWhen),
        ));
    }
    marks.push(Jump::section(
        section_id(component, depth, "examples"),
        context.t(Key::ComponentExamples),
    ));
    if has_variants(component) {
        marks.push(Jump::section(
            section_id(component, depth, "variants"),
            context.t(Key::ComponentVariants),
        ));
    }
    marks.push(Jump::section(
        section_id(component, depth, "reference"),
        context.t(Key::ComponentReference),
    ));
    marks
}

/// The contract, folded away: props, states, keyboard and accessibility.
///
/// # Why it is closed
///
/// These four tables were always open at the foot of every page, and on
/// `/tables/` that was six props tables — the longest thing on the site under
/// the least-read heading. A reader who came to look up a prop opens one fold;
/// a reader who came to see the component never scrolls past four tables to
/// leave. Each control says how many rows are behind it, because a fold that
/// hides an unknown quantity is a fold nobody presses.
///
/// Each is a real heading inside its `<summary>` (`Disclosure { heading }`), so
/// the page's outline still lists them while they are shut — closed, the title
/// is the only thing left of a section, and a screen reader navigating by
/// heading would otherwise find nothing here at all.
#[component]
fn ReferenceBlock(
    context: Context,
    component: crate::generated::catalogue::DocComponent,
    depth: Depth,
) -> Element {
    let heading = depth.folded();
    // The props a caller writes. `children` is not one of them: it is what goes
    // inside the component, and counting it would promise a row that the table
    // below does not have.
    let props = component
        .props
        .iter()
        .filter(|prop| prop.kind != "children")
        .count();
    rsx! {
        Disclosure {
            class: "nd-doc-fold",
            heading,
            summary: context.fill(Key::ComponentPropsCount, "count", &props.to_string()),
            ContractProps { context, component }
        }

        Disclosure {
            class: "nd-doc-fold",
            heading,
            summary: if component.states.is_empty() {
                context.t(Key::ComponentStates).to_owned()
            } else {
                context.fill(
                    Key::ComponentStatesCount,
                    "count",
                    &component.states.len().to_string(),
                )
            },
            if component.states.is_empty() {
                Paragraph { tone: ParagraphTone::Muted, "{context.t(Key::ComponentStatesNone)}" }
            } else {
                ContractStates { context, component }
            }
        }

        Disclosure {
            class: "nd-doc-fold",
            heading,
            summary: if component.keyboard.is_empty() {
                context.t(Key::ComponentKeyboard).to_owned()
            } else {
                context.fill(
                    Key::ComponentKeyboardCount,
                    "count",
                    &component.keyboard.len().to_string(),
                )
            },
            if component.keyboard.is_empty() {
                Paragraph { tone: ParagraphTone::Muted, "{context.t(Key::ComponentKeyboardNone)}" }
            } else {
                ContractKeyboard { context, component }
            }
        }

        Disclosure {
            class: "nd-doc-fold",
            heading,
            summary: context.t(Key::ComponentA11y).to_owned(),
            if let Some(role) = component.role {
                Paragraph {
                    "{context.t(Key::ComponentA11yRole)} "
                    Code { "{role}" }
                }
            }
            if component.needs_name {
                Paragraph { "{context.t(Key::ComponentA11yName)}" }
            }
            if let Some(notes) = component.a11y_notes {
                Paragraph {
                    Message { context, message: notes }
                }
            }
            if component.role.is_none() && !component.needs_name && component.a11y_notes.is_none() {
                Paragraph { tone: ParagraphTone::Muted, "{context.t(Key::ComponentA11yPlain)}" }
            }
        }
    }
}

/// One component's whole reference, at whatever depth the page reads it.
///
/// The same four sections either way — when to use it, the worked examples,
/// every variant, and the folded contract — in the same order on all fifty-six
/// pages. What a collection page changes is only how deep they sit, which is
/// `Depth` and nothing else.
#[component]
fn ComponentSections(
    context: Context,
    component: crate::generated::catalogue::DocComponent,
    depth: Depth,
) -> Element {
    let when_id = section_id(component, depth, "when");
    let examples_id = section_id(component, depth, "examples");
    let variants_id = section_id(component, depth, "variants");
    let reference_id = section_id(component, depth, "reference");
    rsx! {
        if let Some(guidance) = component.guidance {
            section { class: "nd-doc-section", id: "{when_id}",
                Heading { level: depth.level(), size: depth.size(),
                    "{context.t(Key::ComponentWhen)}"
                }
                Paragraph {
                    Message { context, message: guidance }
                }
            }
        }

        // ONE FRAME PER EXAMPLE, AND THAT IS A CORRECTION.
        //
        // Every example on a page used to sit inside ONE frame, on the argument
        // that its width control would then resize the whole set at once. What
        // that built was one frame holding thirty unrelated things — and a
        // frame is a viewport, so the question it answers is *does THIS survive
        // a narrow column*, which is a question about one example at a time.
        section { class: "nd-doc-section", id: "{examples_id}",
            Heading { level: depth.level(), size: depth.size(),
                "{context.t(Key::ComponentExamples)}"
            }
            for (index, example) in crate::examples::for_component(component.name).iter().enumerate() {
                {framed(
                    context,
                    &format!("{}-{index}", component.slug),
                    Key::ComponentExamples,
                    component.name,
                    example,
                    crate::trees::nodes(component.name, index),
                )}
            }
        }

        // Every value of every enum prop that HAS an appearance, derived from
        // the contract — so a variant cannot be forgotten and cannot disagree
        // with the component. It has a heading of its own again: the grids used
        // to run on under `Examples` with nothing naming them, and the key that
        // named them had been dead in the catalogue ever since.
        //
        // `variants = false` is how a prop says its values look identical: a
        // comparator, a band, a scope, a total a script computes. A grid of
        // three identical pictures under three different captions reads as a
        // component whose props do nothing, and a reader is right to read it
        // that way.
        //
        // IT SHIPS FOLDED. The examples above it already show every variant a
        // reader needs to choose between, in use and in context. This grid is
        // the exhaustive check — every value of every drawn prop, side by side
        // — which is what somebody comparing two values comes for and what
        // everybody else scrolls past. The fold keeps it one press away and
        // keeps its heading in the page's outline while it is shut.
        //
        // The id stays on the `<section>` rather than moving to the `<details>`,
        // so the jump index still lands on something a closed fold cannot hide,
        // and `revealTarget()` in `shell.js` opens it on arrival.
        if has_variants(component) {
            section { class: "nd-doc-section", id: "{variants_id}",
                Disclosure {
                    class: "nd-doc-fold",
                    heading: depth.section_fold(),
                    summary: context.t(Key::ComponentVariants).to_owned(),
                    for prop in component.props.iter().filter(|prop| drawn(prop)) {
                        ExampleFrame {
                            context,
                            name: "{component.slug}-{prop.name}",
                            label: Key::ComponentVariants,
                            // A grid is a tree now, so it can be mounted like any other.
                            mountable: crate::react_available(),
                            Example {
                                context,
                                source: "{prop.name}",
                                tree: crate::react_available()
                                    .then(|| format!("{}-{}", component.slug, prop.name)),
                                div { class: "nd-variant-row",
                                    for value in prop.values {
                                        Specimen { context, component, prop: *prop, value: (*value).to_string() }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }

        section { class: "nd-doc-section", id: "{reference_id}",
            Heading { level: depth.level(), size: depth.size(),
                "{context.t(Key::ComponentReference)}"
            }
            ReferenceBlock { context, component, depth }
        }
    }
}

/// A component's own page: its reference, and nothing else on it.
#[component]
fn ComponentPage(context: Context) -> Element {
    let component = context.page.component.expect("a component page");
    let marks = component_marks(context, component, Depth::Page);
    rsx! {
        DocIndex { context, marks }
        ComponentSections { context, component, depth: Depth::Page }
    }
}

/// Whether this prop gets a grid of pictures, one per value.
///
/// THREE PLACES RENDER A VARIANT GRID — a component page, a collection page and
/// the scene — and only one of them was asking this. So `variants = false` was
/// a declaration that worked on `/tables/` and did nothing at all on `/button/`,
/// where `submit` went on drawing three identical buttons under `button`,
/// `submit` and `reset` after the contract had said not to. A rule enforced in
/// one of three call sites is not a rule.
///
/// `serialise::json` is the fourth reader, for the same reason: a grid it wrote
/// and no page draws is a tree the browser would mount over nothing.
pub(crate) fn drawn(prop: &crate::generated::catalogue::DocProp) -> bool {
    prop.kind == "enum" && prop.variants
}

/// One rendering of a component, with one enum prop set to one value.
///
/// # Why this draws a tree rather than markup
///
/// It used to build an HTML **string** out of the contract: the element, the
/// root class, the prop's attribute, and every other enum prop at its default.
/// That made every grid on the site a picture of what a string builder believed
/// the component would render, and the two could disagree with nothing to
/// report it. `data-nd-data-nd-size` on fourteen pages, four tones drawn as four
/// untouched grey buttons, and `data-nd-kind="ordered"` on a `<ul>` all shipped
/// that way. Each was a grid faithfully documenting a bug in the grid.
///
/// The tree is generated from the same contract by
/// `crates/noctua-design-emit/src/specimens.rs` and rendered through the
/// interpreter, so the attributes are the component's own. Where a contract
/// carries a `specimen` that is a drawing, the tree carries that drawing, and
/// the generated file names every component in that position.
#[component]
fn Specimen(
    context: Context,
    component: crate::generated::catalogue::DocComponent,
    prop: crate::generated::catalogue::DocProp,
    value: String,
) -> Element {
    let nodes = crate::generated::specimens::nodes(component.name, prop.name, &value);
    rsx! {
        {crate::tree::render(nodes, crate::tree::Scope::new(context))}
    }
}

// --- The rest ---------------------------------------------------------------

/// Every icon in the system, listed by the system itself.
///
/// The page loops `noctua_design_icons::ALL`, so vendoring a drawing documents
/// it with no edit here. That is the user-facing half of the reason the set is
/// generated: a hand-maintained gallery is a list that goes stale, and a stale
/// gallery of icons is indistinguishable from a missing icon — which is exactly
/// the failure this whole layer replaced.
///
/// # The filter, and why it hides rather than removes
///
/// The set is over a thousand drawings, and a wall of a thousand pictures with
/// no way to narrow it is a gallery nobody finds anything in. The field above
/// the grid hides the cells whose name does not match, which is the rule
/// `Table` already keeps: what is filtered is still in the document, so the
/// count below stays true and the page a reader loads with no script working is
/// the whole set rather than an empty box.
#[component]
fn Icons(context: Context) -> Element {
    // Filled here rather than inside `rsx!`: the placeholder is literally
    // `{count}`, and the macro would read it as an interpolation of its own.
    //
    // Through `Context::fill`, which is the generated resolver, rather than a
    // `str::replace` written on the spot — a message with a value in it is one
    // message, and there is one function that knows how to finish one.
    let counted = context.fill(
        Key::IconsCount,
        "count",
        &noctua_design_icons::ALL.len().to_string(),
    );
    rsx! {
        // NO OPENING PARAGRAPH. It said the set was declared once, that there
        // is one constant per icon, and that an undrawn icon costs an
        // application nothing. All three are facts about how this repository
        // builds, which `NOCTUA.md §9.5` records and `xtask check` measures. A
        // reader here wants the drawing and its name.
        p { class: "nd-note nd-prose", "{counted}" }

        // The gallery is an EXAMPLE, in the frame every example sits in. It was
        // a bare `<ul>` — the last demonstration on the site with a container
        // of its own — and it wanted the frame's controls more than most: a
        // gallery of thirteen icons at 320px is the question "does this wrap
        // sensibly on a phone", and there was no way to ask it.
        ExampleFrame { context, name: "icons", label: Key::PageIconsTitle,
            Example { context, source: ICONS_SOURCE,
                div { class: "nd-icon-gallery", "data-nd-icon-gallery": "true",
                    // A REAL COMPONENT, because every interactive control in
                    // this system is one. The narrowing is the site's own
                    // behaviour and lives in `shell.js`; the field is `Input`.
                    Input {
                        name: "icon-filter",
                        label: context.t(Key::IconsFilterLabel),
                        icon: IconDrawing::SEARCH,
                        clearable: true,
                        clear_label: context.t(Key::IconsFilterClear),
                        autocomplete: "off",
                        spellcheck: false,
                    }

                    // Written server-side and hidden, so the script has the
                    // sentence in the reader's language without a catalogue of
                    // its own. `role="status"` because it appears in response
                    // to typing, which a screen reader is otherwise not told.
                    p {
                        class: "nd-icon-empty",
                        role: "status",
                        hidden: true,
                        "{context.t(Key::IconsFilterEmpty)}"
                    }

                    ul { class: "nd-icon-grid",
                        for icon in noctua_design_icons::ALL {
                            li { class: "nd-icon-cell", "data-nd-icon-name": icon.name,
                                // Unlabelled: the name beside it is the accessible
                                // name, and an icon that repeats the text next to it
                                // is noise to a screen reader rather than help.
                                Icon { icon, size: IconSize::Large }
                                Code { "{icon.name}" }
                            }
                        }
                    }
                }
            }
        }
    }
}

/// The call that produced the icon gallery.
const ICONS_SOURCE: &str =
    "for icon in noctua_design_icons::ALL { Icon { icon, size: IconSize::Large } }";

/// The three sections of the typography page: the scale, the families, the
/// scripts.
///
/// Read once, by the jump index and by the page under it, for the reason
/// `SPACING_SECTIONS` is.
const TYPE_SECTIONS: [(&str, Key); 3] = [
    ("type-scale", Key::TypeScaleTitle),
    ("type-families", Key::TypeFamiliesTitle),
    ("type-scripts", Key::TypeScriptsTitle),
];

/// The type scale, the four families, and the fourteen scripts.
///
/// The scripts are not decoration: `NOCTUA.md §7.3.1` is the reason Noto was
/// chosen over any other family, and this page is where that reason is either
/// visible or false. A browser spec asserts every one renders with no tofu.
///
/// # One frame per demonstration, and three demonstrations
///
/// The page held four inside a single frame, so one width control resized a
/// scale, a set of family specimens and fourteen writing systems together —
/// which is the arrangement `ComponentSections` records as a correction, for
/// the same reason. A frame is a viewport, and *does this survive a narrow
/// column* is a question about one specimen at a time.
///
/// The fourth was a whole page of `Prose`, `Blockquote`, `List`, `CodeBlock`
/// and the ten inline marks. Each of those has a page of its own with its own
/// examples, and none of them is the type scale, the families or the scripts.
#[component]
fn Typography(context: Context) -> Element {
    let marks = TYPE_SECTIONS
        .iter()
        .map(|(id, heading)| Jump::section((*id).to_owned(), context.t(*heading)))
        .collect();
    rsx! {
        DocIndex { context, marks }

        section { class: "nd-doc-section", id: "{TYPE_SECTIONS[0].0}",
            Heading { level: HeadingLevel::_2, size: HeadingSize::H2,
                "{context.t(Key::TypeScaleTitle)}"
            }
            ExampleFrame { context, name: "type-scale", label: Key::TypeScaleTitle,
                Example { context, source: SCALE_SOURCE, note: Key::TypeScaleBody,
                    Scale { context }
                }
            }
        }

        section { class: "nd-doc-section", id: "{TYPE_SECTIONS[1].0}",
            Heading { level: HeadingLevel::_2, size: HeadingSize::H2,
                "{context.t(Key::TypeFamiliesTitle)}"
            }
            // NO NOTE. The four families are named beside their own specimens
            // and the weights are named beside their own lines, so a sentence
            // saying there are four of them at every weight is the picture
            // again in words.
            ExampleFrame { context, name: "type-families", label: Key::TypeFamiliesTitle,
                Example { context, source: FAMILIES_SOURCE,
                    Families { context }
                }
            }
        }

        section { class: "nd-doc-section", id: "{TYPE_SECTIONS[2].0}",
            Heading { level: HeadingLevel::_2, size: HeadingSize::H2,
                "{context.t(Key::TypeScriptsTitle)}"
            }
            // Nor here, and for the same reason: every line below is visibly
            // in the script named above it.
            ExampleFrame { context, name: "type-scripts", label: Key::TypeScriptsTitle,
                Example { context, source: SCRIPTS_SOURCE,
                    Scripts { context }
                }
            }
        }
    }
}

/// The three calls the typography page's demonstrations were produced by.
///
/// A CALL, not a sentence about one. `source` is rendered as the figure's
/// caption inside `<code>`, and it is the thing a reader came to copy — two of
/// these were English prose sitting in that slot, which reads as a caption the
/// stylesheet set in the wrong face.
///
/// The interpunct separates alternatives, which is the convention
/// `examples.rs` already uses where one demonstration shows several calls.
const SCALE_SOURCE: &str = "noctua_design_tokens::size::*";
const FAMILIES_SOURCE: &str = "var(--noctua-font-sans) · var(--noctua-font-serif) · \
                               var(--noctua-font-display) · var(--noctua-font-mono)";
const SCRIPTS_SOURCE: &str = r#"p { lang: "ar", dir: "rtl", class: "nd-script-sample" }"#;

// A WHOLE PAGE OF EVERYTHING was a fourth demonstration here, and an index
// of every component page under it. The specimen drew `Prose`, `Blockquote`,
// two kinds of `List`, a `CodeBlock` and the ten inline marks, each of which
// has a page of its own; the index was the sidebar a second time, sixty
// links deep, on a page the sidebar is already open beside.

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
                // `.nd-specimen`, not `.nd-variant-item`. This borrowed the
                // frame's variant-grid class while sitting outside any grid,
                // so the separator `.nd-specimen` draws between families never
                // applied and the four ran together — a rule written, and a
                // class never carried, which is this stylesheet's oldest bug.
                div { class: "nd-specimen",
                    // A family name is a proper noun, and a weight name is a token.
                    Text { class: "nd-specimen-name nd-identifier", weight: TextWeight::Semibold,
                        "{family}"
                    }
                    for (label, weight) in weights {
                        p {
                            class: "nd-specimen-line",
                            style: "font-family: {stack}; font-weight: {weight}", // allow-literal: `stack` is one of the four font tokens in the table above — the gate cannot see through an interpolation, and a specimen page must set the family it is showing
                            Text { class: "nd-specimen-weight nd-identifier", size: TextSize::Caption,
                                tone: TextTone::Muted, "{label}"
                            }
                            "{context.t(Key::TypeSpecimenLine)}"
                        }
                    }
                    p {
                        class: "nd-specimen-line",
                        style: "font-family: {stack}; font-style: italic", // allow-literal: as above, the italic specimen
                        Text { class: "nd-specimen-weight nd-identifier", size: TextSize::Caption,
                            tone: TextTone::Muted, "italic"
                        }
                        "{context.t(Key::TypeSpecimenLine)}"
                    }
                }
            }
        }
    }
}

/// The fourteen scripts, each named and each set in real text.
///
/// Every row carries `lang` and `dir`, because a browser shapes Arabic
/// correctly only when it is told the text is Arabic — and because a test that
/// rendered the right glyphs from the wrong language tag would be proving
/// something other than what it claims.
///
/// # The name, and nothing under it
///
/// Each row carried a second line saying what that sample exercised: *the
/// fallback must not fire*, *conjunct consonants and marks above and below the
/// baseline*, *the base case, and every letter of the alphabet*. Those are the
/// claims `scripts.spec.js` makes, in the file that can check them, and on the
/// page they were fourteen notes to a maintainer in front of a reader who came
/// to look at the type.
#[component]
fn Scripts(context: Context) -> Element {
    rsx! {
        div { class: "nd-scripts", "data-nd-scripts": "",
            for sample in crate::scripts::SAMPLES {
                // The hook a spec reads is the script's ID, not its name: a
                // test that selected on a translated word would pass in one
                // locale and fail in the next.
                div { class: "nd-script", "data-nd-script": "{sample.id}",
                    div { class: "nd-script-meta",
                        Text { class: "nd-script-name", weight: TextWeight::Semibold,
                            "{context.t(sample.name)}"
                        }
                    }
                    // Real text in that script. Translating a Devanagari
                    // sample into Portuguese would stop it being a Devanagari
                    // sample, which is the only thing it is here to be.
                    p {
                        class: "nd-script-sample nd-identifier",
                        lang: "{sample.lang}",
                        dir: "{sample.dir}",
                        "data-nd-monospaceable": "{sample.monospaceable}",
                        // Declared rather than guessed from the name: a width
                        // check cannot tell an ideograph from a tofu box, and
                        // both are exactly one em.
                        "data-nd-ideographic": "{sample.ideographic}",
                        "{sample.text}"
                    }
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::generated::catalogue::COMPONENTS;
    use crate::tree::Node;

    /// The picture a stage holds, whatever wrapping it needed.
    ///
    /// A specimen is a figure, the stage inside it and the picture inside that,
    /// so a test about the picture has to walk two levels first.
    fn drawing(nodes: &'static [Node]) -> Option<&'static Node> {
        let [Node::Element { children, .. }] = nodes else {
            return None;
        };
        let [Node::Element { children, .. }, _] = children else {
            return None;
        };
        children.first()
    }

    /// Every value of every drawn prop has a picture.
    ///
    /// The lookup answers an unknown name with an empty tree, which draws an
    /// empty figure under a caption that still names the value. That reads as a
    /// component whose variants do nothing, which is the failure this whole
    /// layer exists under, so a missing specimen fails here instead.
    #[test]
    fn every_value_of_every_drawn_prop_is_drawn() {
        let mut found = 0usize;
        for component in COMPONENTS {
            for prop in component.props.iter().filter(|prop| drawn(prop)) {
                for value in prop.values {
                    found += 1;
                    let nodes =
                        crate::generated::specimens::nodes(component.name, prop.name, value);
                    assert!(
                        drawing(nodes).is_some(),
                        "{}.{} at {value} draws an empty figure",
                        component.name,
                        prop.name
                    );
                }
            }
        }
        // A walk that found nothing would pass for ever and prove nothing.
        assert!(found > 100, "only {found} specimens were read");
    }

    /// A specimen of a table part has to arrive inside a table.
    ///
    /// The browser parses a stage's content in the "in body" insertion mode,
    /// where every table-only start tag is a parse error that is ignored. Left
    /// bare, `Table`'s five parts would document eleven enum props with empty
    /// boxes — on a page that otherwise looks finished, which is why nothing
    /// would have caught it.
    #[test]
    fn every_specimen_of_a_table_part_is_drawn_inside_a_table() {
        let mut found = 0usize;
        for component in COMPONENTS {
            // A `Table` arrives inside its scroller instead, because `height`
            // caps the SCROLLER and four heights would otherwise be four
            // identical boxes.
            let wrapper = match component.element {
                "table" => "div",
                "caption" | "colgroup" | "thead" | "tbody" | "tfoot" | "tr" | "td" | "th" => {
                    "table"
                }
                _ => continue,
            };
            for prop in component.props.iter().filter(|prop| drawn(prop)) {
                for value in prop.values {
                    found += 1;
                    let nodes =
                        crate::generated::specimens::nodes(component.name, prop.name, value);
                    let Some(Node::Element { tag, .. }) = drawing(nodes) else {
                        panic!("{}.{} at {value} draws nothing", component.name, prop.name);
                    };
                    assert_eq!(
                        *tag, wrapper,
                        "{}.{} at {value} would be thrown away by the parser",
                        component.name, prop.name
                    );
                }
            }
        }
        assert!(found > 0, "no table part was read");
    }
}
