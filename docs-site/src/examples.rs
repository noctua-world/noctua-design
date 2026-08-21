//! Worked examples, one table per component.
//!
//! # Why these are written and the variant grid is generated
//!
//! `pages::ComponentPage` builds a grid showing every value of every enum prop,
//! derived from the contract, so no variant can be forgotten and none can
//! disagree with the component. What it cannot show is a component **used**:
//! `Segmented` needs `SegmentedItem` children, `List` needs items, `Prose` only
//! means anything with a heading and two paragraphs inside it, and thirteen
//! components have no enum prop at all — so their grid was empty, and their
//! page showed a prop table under a heading that said "Every variant".
//!
//! # What is here, and what moved
//!
//! The caption and the note. **What each example draws lives in
//! `docs-site/src/trees/`**, as data, because a Rust closure returning `rsx!`
//! is a thing only Dioxus can run — and this system ships two implementations
//! that `NOCTUA.md §9.3` requires to behave identically. The React package was
//! rendered nowhere on the site that documents it, and six of its modules were
//! broken for an unknown length of time under a green suite.
//!
//! The closures were kept beside the trees for exactly as long as it took to
//! prove the transcription, by rendering both halves and comparing them byte
//! for byte in every locale. 1,497 renderings, no difference. Then they went:
//! two descriptions of one example, only one of which ships, is the drift this
//! file's own comments are about.
//!
//! # The caption is the code
//!
//! Each example carries the Dioxus call that produced it, and that call is also
//! its caption. `Blockquote { tone: Accent, … }` says what an example shows
//! more precisely than a sentence would, in every language at once, and it is
//! the thing a reader came to copy. A `note` is for the few cases where the
//! call does not speak for itself, and a note is a message like every other
//! sentence on this site.

use noctua_design_i18n::Key;

/// One worked example.
#[derive(Clone, Copy)]
pub(crate) struct Example {
    /// The Dioxus call that produced it. Shown above the example, and its
    /// caption: an identifier rather than prose, and the same in every locale.
    pub source: &'static str,
    /// A sentence, for the few examples the call does not explain by itself.
    pub note: Option<Key>,
}

/// Declares one example without repeating the field names.
macro_rules! example {
    ($source:literal) => {
        Example {
            source: $source,
            note: None,
        }
    };
    ($source:literal, $note:expr) => {
        Example {
            source: $source,
            note: Some($note),
        }
    };
}

/// The one complete example a page opens with.
///
/// # Why a page needs one, and why it is not just the first example
///
/// Every page here used to open on a wall of prose and then the component at
/// its simplest — `Button { label: "…" }`, one grey rectangle — and a reader
/// who wanted to know what the thing *is* had to assemble it from twenty-two
/// separate examples further down. The examples below it each isolate one prop,
/// which is the right shape for an example and the wrong shape for an
/// introduction: they answer questions a reader cannot have yet.
///
/// So this is the component **used**, at the top, before anything is explained:
/// several props at once, real content, and on a collection page the parts
/// working together rather than one part alone. It is a model to copy, and it
/// is the only place on the site where showing off is the point.
///
/// KEYED BY PAGE, NOT BY COMPONENT. `/tables/` opens with one whole table built
/// from all six of its parts, which belongs to no single contract; and the
/// thirty-one components that share a page would otherwise each contribute an
/// overview to a page that can only have one.
type Overview = (&'static str, Example);

/// The overview for a page, or nothing if it has none.
fn overview(slug: &str) -> Option<Example> {
    OVERVIEWS
        .iter()
        .find(|(page, _)| *page == slug)
        .map(|(_, example)| *example)
}

/// The overview for a page, for `pages::DocHead`.
pub(crate) fn overview_for_page(slug: &str) -> Option<Example> {
    overview(slug)
}
/// The scenes, in the order the page tells them, which between them set
/// every prop the contract declares and show every feature it has.
const LINEAGE: &[Example] = &[
    example!(
        r#"Lineage { name: "doe", label: "…", graph: "figure gran … bond u0 ada" }"#,
        Key::ExampleLineageDoeNote
    ),
    example!(
        r#"Lineage { graph: "wobble", empty_label: "…", height: Short }"#,
        Key::ExampleLineageEmptyNote
    ),
    example!(
        r"Lineage { parallax: true, LineageFigure { parallax: Deep } } — the origin",
        Key::ExampleLineageSlotsNote
    ),
    example!(
        r#""bond ua kid kind=adoption": four true parents  ·  "kind=donor" and "kind=partner""#,
        Key::ExampleLineageKindsNote
    ),
    example!(
        r#""mark u1 kind=union" twice: one person, two unions  ·  "bond gran u0 kind=union" across generations"#
    ),
    example!(
        r"Lineage { route: Elbow, height: Short }  ·  Lineage { route: Taut }",
        Key::ExampleLineageRouteNote
    ),
    example!(r"Lineage { size: Small }  ·  Lineage { size: Large, tilt: Resting }"),
    example!(
        r"Lineage { line: Beam, tilt: Raked }  ·  { line: Ribbon }  ·  { thickness: None }",
        Key::ExampleLineageVolumeNote
    ),
    example!(
        r"Lineage { view: Deep, tilt: Resting, height: Full }",
        Key::ExampleLineageDeepNote
    ),
    example!(
        r#"Lineage { filter: "Doe", filter_mode: Isolate, matches_label, elided_label }"#,
        Key::ExampleLineageFilterNote
    ),
    example!(
        r#"Lineage { pick: Both, form: "ties" }  ·  { camera_label, zoom_in_label, zoom_out_label, fit_label }"#,
        Key::ExampleLineageActingNote
    ),
    example!(
        r#"Lineage { occasion: "christmas" } — every figure has a face for the day"#,
        Key::ExampleLineageSeasonNote
    ),
    example!(
        r#"Lineage { sets: "doe:local", LineageFigure { action, swaps: "doe:remote" } }"#,
        Key::ExampleLineageServerNote
    ),
    example!(
        r#"Lineage { graph: "…origin span=2" }  ·  "figure title rank=0 span=3 cluster=doe"  ·  three families"#,
        Key::ExampleLineageOutrightNote
    ),
    example!(
        r#"…"mark note kind=milestone slot" — a note standing on the structure"#,
        Key::ExampleLineageMarkNote
    ),
    example!(
        r#"Lineage { reach: Shy, lettering: Solid, graph: "…shape=bare…" }"#,
        Key::ExampleLineageWildNote
    ),
    example!(
        r#"div { "data-nd-model-size": "2", … }  Text { … }  Text { blurb }"#,
        Key::ExampleLineageQueenNote
    ),
];
/// The figures, in the order the page tells them, which between them set
/// every prop the contract declares.
const LINEAGE_FIGURE: &[Example] = &[
    example!(
        r#"LineageFigure { scene: "…", at: "ada", shape: Card, tone: Accent, parallax: Subtle }"#,
        Key::ExampleLineageFigureNote
    ),
    example!(r"LineageFigure { shape: Plaque, tone: Warning }  ·  { shape: Bare }"),
    example!(r#"LineageFigure { href: "…", label: "Ada Doe", occasion: "christmas" }"#),
    example!(
        r#"LineageFigure { action: "lineage-swap", swaps: "doe:remote", set: "doe:local" }"#,
        Key::ExampleLineageServerNote
    ),
];

/// The overview each page opens with, by page slug.
///
/// One per page and not one per component — see `Overview` above. The order
/// here is the sidebar's; nothing reads it in order, but a list of fifty that
/// is not sorted is a list nobody can check against the site.
const OVERVIEWS: &[Overview] = &[
    // --- Button ---
    (
        "button",
        example!(
            r#"Button { variant: Solid, tone: Accent, icon: "check", label: "Publish" }
Button { variant: Outline, label: "Save" }
Button { menu: true, controls: "overview-button-menu", label: "More actions" }"#
        ),
    ),
    (
        "avatars",
        example!(
            r#"Avatar { decorative: true, initials: "AL", src: "…", label: "…" }
AvatarGroup { label: "Reviewers", more: "+5", more_label: "…", more_href: "…" }"#
        ),
    ),
    (
        "badge",
        example!(
            r#"Badge { count: "3", label: "3 unread messages", Button { … } }
Badge { count: "128", max: "99", tone: "accent", … }
Badge { dot: true, tone: "neutral", … }"#
        ),
    ),
    (
        "blockquote",
        example!(
            r#"Blockquote { tone: Accent, cite: "…", attribution: "Edsger W. Dijkstra", Paragraph { … } }"#
        ),
    ),
    (
        "carousel",
        example!(
            r#"Carousel { name, label, per_view: "1", snap: Center, dots: true, counter: true }
CarouselSlide { label: "…" }"#
        ),
    ),
    (
        "checkbox",
        example!(
            r#"Fieldset { legend: "…", hint: "…" }
Checkbox { name: "apply", value: "work", label: "…", hint: "…", checked: true }
Checkbox { value: "none", divider: "or", exclusive: true }"#
        ),
    ),
    (
        "code-block",
        example!(
            r#"CodeBlock { code, language: "rust", filename: "scope.rs", numbered: true, mark_lines: "4-6", copy: "…", copied: "…", caption: "…" }"#
        ),
    ),
    (
        "color-picker",
        example!(
            r#"ColorPicker { name: "overview-label-colour", label: "…", hint: "…", value: "…", eyedropper: true, formats: "hex rgb oklch" }"#
        ),
    ),
    (
        "disclosure",
        example!(
            r#"Disclosure { summary: "…", heading: DisclosureHeading::H3, open: true }
Disclosure { summary: "…", heading: DisclosureHeading::H3 }"#
        ),
    ),
    (
        "error-summary",
        example!(
            r#"ErrorSummary { name: "…", title: "There is a problem", description: "…" }
ErrorSummaryItem { control: "nd-input-overview-name", "Enter your full name" }
ErrorSummaryItem { control: "nd-overview-terms-yes", "…" }"#,
            Key::ExampleOverviewErrorSummaryNote
        ),
    ),
    (
        "field",
        example!(
            r"Field { control, label, hint, required, required_label } / Field { kind: Group, label, hint }"
        ),
    ),
    (
        "fieldset",
        example!(
            r#"Fieldset { name: "overview-apply", legend: "…", hint: "…" }
Checkbox { name: "overview-apply", value: "study", label: "…", hint: "…" }"#
        ),
    ),
    (
        "heading",
        example!(
            r"Heading { level: _2, size: HeadingSize::Display, balance: true }
Heading { level: _3, size: HeadingSize::H3, tone: HeadingTone::Muted }"
        ),
    ),
    (
        "info",
        example!(
            r#"Info { name: "…", label: "What counts as a token", heading: "…", close_label: "…" }"#
        ),
    ),
    (
        "inline-text",
        example!(
            r#"Paragraph {
    Strong { … } Time { datetime: "2026-07-29", … } Code { … } Del { label: "removed:", … } Ins { label: "added:", … }
    Abbr { title: "…", … } Em { … } Mark { … } Kbd { … } Text { numeric: true, … }
}"#
        ),
    ),
    (
        "lineage",
        example!(
            r#"Lineage { name: "overview-doe", label: "…", graph: "figure gran … bond u1 cara", pick: Both, camera_label, zoom_in_label, zoom_out_label, fit_label }"#,
            Key::ExampleLineageActingNote
        ),
    ),
    (
        "list",
        example!(
            r#"List { kind: ListKind::Ordered } with ListItem { "…" }
List { kind: ListKind::Description, spacing: Tight } with ListItem { term: "…", "…" }"#
        ),
    ),
    (
        "menu",
        example!(
            r#"Button { menu: true, controls: "overview-menu-actions", label: "Actions" }
Menu { name: "overview-menu-actions", label: "…",
    MenuItem { icon: "copy", label: "…" }
    MenuItem { icon: "close", tone: "danger", label: "…", detail: "…" }
}"#
        ),
    ),
    (
        "meter",
        example!(
            r#"Meter { name: "overview-storage", label: "…", value: "212", max: "256", unit: "GB", low: "128", high: "204", optimum: "0", value_label: "…" }
Meter { name: "overview-seats", label: "…", value: "16", max: "20", value_label: "…" }"#
        ),
    ),
    (
        "pagination",
        example!(
            r#"Pagination { name, label, page: "3", pages: "4", href, previous_label,
next_label, page_label, current_label, summary }"#
        ),
    ),
    (
        "paragraph",
        example!(
            r"Paragraph { size: ParagraphSize::Lead, … }
Paragraph { … }
Paragraph { size: ParagraphSize::Small, tone: ParagraphTone::Muted, … }"
        ),
    ),
    (
        "progress",
        example!(
            r"Progress { name, label, value, max, value_label }
Progress { name, label, busy_label } — no value at all"
        ),
    ),
    (
        "prose",
        example!(
            r"Prose { Heading { … } Paragraph { size: Lead, … } Paragraph { … } Blockquote { … } }"
        ),
    ),
    (
        "search",
        example!(
            r#"Search { name, label, placeholder, status, empty_label, clear_label }
SearchGroup { label: "…" }
SearchResult { href: "…", label: "…", detail: "…", keywords: "…", current: true }"#
        ),
    ),
    (
        "segmented",
        example!(
            r#"Segmented { name: "…", label: "Mode", fill: SegmentedFill::Equal }
SegmentedItem { value: "light", icon: "sun", label: "Light" }
SegmentedItem { value: "system", icon: "monitor", label: "…", selected: true }"#
        ),
    ),
    (
        "select",
        example!(
            r#"Select { name: "overview-palette", label: "…", hint: "…", placeholder: "…", searchable: true, clearable: true }
SelectGroup { label: "…" }  ·  SelectOption { value: "ochre-balanced", label: "Ochre", swatch: "ochre-balanced", detail: "…" }"#
        ),
    ),
    (
        "slider",
        example!(
            r#"Slider { name: "…", label: "Volume", value: "35", min: "0", max: "100", step: "5" }
Slider { marks: "0 50 100", mark_labels: "Silent|Loud", output_label: "{value}%" }"#
        ),
    ),
    (
        "swatch",
        example!(
            r"SegmentedItem { value, label, Swatch { palette: value, size: SwatchSize::Small } }"
        ),
    ),
    (
        "switch",
        example!(
            r#"Switch { name: "overview-grain", on: "on", off: "off", checked: true, label: "…", describedby: "overview-switch-note" }
Switch { name: "overview-pattern", on: "on", off: "off", label: "…" }"#
        ),
    ),
    (
        "tag",
        example!(
            r#"Tag { icon: IconDrawing::CHECK, tone: "shipped", label: "Shipped" }
Tag { tone: "accent", variant: TagVariant::Solid, label: "In review" }
Tag { tone: "danger", variant: TagVariant::Outline, label: "Blocked" }"#
        ),
    ),
    (
        "tag-picker",
        example!(
            r#"TagPicker { name: "topics", label: "…", placeholder: "…", hint: "…", status, create_label, empty_label, remove_label, added_label, removed_label }
SelectGroup { label: "…", SelectOption { value: "docs", label: "docs" } }"#
        ),
    ),
    (
        "input",
        example!(
            r#"Input { name, label, kind: Email, placeholder: "…", hint, required, required_label, autocomplete }
Input { kind: Password, reveal_label, conceal_label }"#
        ),
    ),
    (
        "textarea",
        example!(
            r#"Textarea { name: "overview-note", label: "…", hint: "…", rows: "3", autosize: true, max_rows: "8", maxlength: "400", counter: true, counter_label: "…" }"#
        ),
    ),
    (
        "toolbar",
        example!(
            r#"Toolbar { label: "…" }
ToolbarGroup { … } / ToolbarGroup { align: ToolbarGroupAlign::End, … }"#
        ),
    ),
    (
        "toolbar-group",
        example!(
            r#"Toolbar { label: "…" }
ToolbarGroup { … }
ToolbarGroup { align: ToolbarGroupAlign::End, … }"#
        ),
    ),
    (
        "tree",
        example!(
            r#"Tree { name, label, guides: true, expand_label, collapse_label,
    TreeItem { label, icon, badge: "3", expanded: true, TreeItem { href, label } }
    TreeItem { label, icon, branch: true, href }
}"#
        ),
    ),
    (
        "truncate",
        example!(
            r#"Truncate { lines: TruncateLines::_2, title: "…", "…" }"#,
            Key::ExampleTruncateWhole
        ),
    ),
    (
        "button-group",
        example!(
            r#"ButtonGroup { label: "Publishing", … }
Button { label: "Publish" }
Button { menu: true, hide_label: true, controls: "overview-button-group-menu", label: "More actions" }"#
        ),
    ),
    (
        "date-input",
        example!(
            r"DateInput { name, legend, hint, day_label, month_label, year_label, pad: true, required: true, required_label, missing_label, number_label, invalid_label, year_digits_label, error_label }"
        ),
    ),
    (
        "file-input",
        example!(
            r#"FileInput { name, label, hint, browse_label, drop_label, multiple: true, max: "3", max_label, accept: ".pdf,image/*", accept_label, max_size, max_size_label, remove_label, empty_label, added_label, removed_label, status }"#
        ),
    ),
    (
        "link",
        example!(
            r#"Link { href: "…", external: true, rel: "noreferrer", "…" }
Link { href: "…", "…" }"#
        ),
    ),
    (
        "number-input",
        example!(
            r#"NumberInput { name, label, hint, value, min, max, step, big_step, prefix: "R$", group: true, increment_label, decrement_label, max_label }"#
        ),
    ),
    (
        "radio",
        example!(
            r#"Fieldset { name, legend, hint }
Radio { name: "overview-delivery", value, label, hint, checked, size: Medium }"#
        ),
    ),
    (
        "tables",
        example!(
            r"Table { name, caption, note, sort_ascending_label, sort_descending_label, sort_cleared_label }
TableColumn { name, width, sortable, sort_as }
TableSection { kind: Head } / TableSection / TableSection { kind: Foot }
TableRow / TableHeaderCell { scope: Row, sort } / TableCell { column, label, align: Numeric, sort_value }
TableRow { emphasis: Strong } / TableCell { aggregate: Sum }"
        ),
    ),
    (
        "time-input",
        example!(
            r"TimeInput { name, legend, hint, hour_label, minute_label, meridiem_label, am_label, pm_label, clock: Auto, value, min, max, step, pad, required, required_label, missing_label, invalid_label, min_label, max_label }"
        ),
    ),
    (
        "visually-hidden",
        example!(
            r#"Link { href: "…", "Read more" VisuallyHidden { " — Typography" } }"#,
            Key::ExampleOverviewVisuallyHiddenNote
        ),
    ),
    // --- The four that arrived together ---
    //
    // Each opens on the component USED rather than on its simplest form, which
    // is what an overview is for. See the type's own documentation.
    (
        "divider",
        example!(
            r#"Divider { spacing: Loose }
Divider { label: "or", spacing: Loose }"#
        ),
    ),
    (
        "label",
        example!(
            r"Label { text, for_control, required, required_label }
Label { kind: Name, for_control } / Segmented { labelledby }"
        ),
    ),
    (
        "image",
        example!(r"Image { src, alt, ratio: Landscape, caption, width, height }"),
    ),
    (
        "message-bar",
        example!(
            r#"MessageBar { tone: "warning", icon: IconDrawing::WARNING, title, dismissible: true }"#
        ),
    ),
    (
        "dialog",
        example!(r"Dialog { name, trigger_label, heading, description, close_label, … }"),
    ),
    (
        "popover",
        example!(r"Popover { name, trigger_label, heading, close_label, … Button { … } }"),
    ),
    (
        "tooltip",
        example!(r#"Tooltip { name, text: "…", Button { icon, label } }"#),
    ),
    (
        "toast",
        example!(r#"Toast { tone: "success", title, duration: "6000", dismiss_label }"#),
    ),
    (
        "icon",
        example!(r#"Icon { icon: "check", label: "…" } / Icon { icon: "star", size: Large }"#),
    ),
    (
        "rating",
        example!(r"Rating { readonly: true, value, max, value_label } / Rating { item_label, … }"),
    ),
    (
        "otp-field",
        example!(r"OtpField { name, legend, box_label, length, hint, invalid_label }"),
    ),
    (
        "tabs",
        example!(r"Tabs { name, label } + Tab { value, label, count } + TabPanel { value, … }"),
    ),
];

/// The examples for a component, by the name its page carries.
///
/// An empty slice is not a legal answer, and `every_component_has_examples`
/// below is what makes that true: a component added to `spec/components/`
/// fails `cargo xtask check` until somebody shows how it is used.
#[must_use]
pub(crate) fn for_component(name: &str) -> &'static [Example] {
    match name {
        "Abbr" => ABBR,
        "Blockquote" => BLOCKQUOTE,
        "Button" => BUTTON,
        "ButtonGroup" => BUTTON_GROUP,
        "Segmented" => SEGMENTED,
        "SegmentedItem" => SEGMENTED_ITEM,
        "Avatar" => AVATAR,
        "AvatarGroup" => AVATAR_GROUP,
        "Swatch" => SWATCH,
        "Tag" => TAG,
        "TagPicker" => TAG_PICKER,
        "ColorPicker" => COLOR_PICKER,
        "Field" => FIELD,
        "Input" => INPUT,
        "Textarea" => TEXTAREA,
        "NumberInput" => NUMBER_INPUT,
        "Slider" => SLIDER,
        "FileInput" => FILE_INPUT,
        "TimeInput" => TIME_INPUT,
        "Carousel" => CAROUSEL,
        "CarouselSlide" => CAROUSEL_SLIDE,
        "ErrorSummary" => ERROR_SUMMARY,
        "ErrorSummaryItem" => ERROR_SUMMARY_ITEM,
        "Badge" => BADGE,
        "Switch" => SWITCH,
        "Toolbar" => TOOLBAR,
        "ToolbarGroup" => TOOLBAR_GROUP,
        "Checkbox" => CHECKBOX,
        "DateInput" => DATE_INPUT,
        "Radio" => RADIO,
        "Fieldset" => FIELDSET,
        "Info" => INFO,
        "Divider" => DIVIDER,
        "Label" => LABEL,
        "Image" => IMAGE,
        "MessageBar" => MESSAGE_BAR,
        "Dialog" => DIALOG,
        "Popover" => POPOVER,
        "Tooltip" => TOOLTIP,
        "Toast" => TOAST,
        "Icon" => ICON,
        "Rating" => RATING,
        "OtpField" => OTP_FIELD,
        "Tabs" => TABS,
        "Tab" => TAB,
        "TabPanel" => TAB_PANEL,
        "Meter" => METER,
        "Progress" => PROGRESS,
        "Pagination" => PAGINATION,
        "Table" => TABLE,
        "TableColumn" => TABLE_COLUMN,
        "TableSection" => TABLE_SECTION,
        "TableRow" => TABLE_ROW,
        "TableCell" => TABLE_CELL,
        "TableHeaderCell" => TABLE_HEADER_CELL,
        "Select" => SELECT,
        "SelectGroup" => SELECT_GROUP,
        "SelectOption" => SELECT_OPTION,
        "Code" => CODE,
        "CodeBlock" => CODE_BLOCK,
        "Del" => DEL,
        "Disclosure" => DISCLOSURE,
        "Em" => EM,
        "Heading" => HEADING,
        "Ins" => INS,
        "Kbd" => KBD,
        "Link" => LINK,
        "List" => LIST,
        "ListItem" => LIST_ITEM,
        "Mark" => MARK,
        "Paragraph" => PARAGRAPH,
        "Prose" => PROSE,
        "Search" => SEARCH,
        "SearchGroup" => SEARCH_GROUP,
        "SearchResult" => SEARCH_RESULT,
        "Strong" => STRONG,
        "Text" => TEXT,
        "Time" => TIME,
        "Lineage" => LINEAGE,
        "LineageFigure" => LINEAGE_FIGURE,
        "Menu" => MENU,
        "MenuItem" => MENU_ITEM,
        "Tree" => TREE,
        "TreeItem" => TREE_ITEM,
        "Truncate" => TRUNCATE,
        "VisuallyHidden" => VISUALLY_HIDDEN,
        _ => &[],
    }
}

// --- Icon -------------------------------------------------------------------

const ICON: &[Example] = &[
    // THE COMMON CASE, AND THE DEFAULT. A drawing beside a word says what the
    // word says, so it is out of the accessibility tree.
    example!(r#"Icon { icon: "check" }"#, Key::ExampleIconNoteHidden),
    // THE ONE THAT NEEDS A NAME. Nothing beside it says what it means, so the
    // drawing becomes a `role="img"` carrying the caller's words.
    example!(
        r#"Icon { icon: "star", label: "…" }"#,
        Key::ExampleIconNoteNamed
    ),
    example!(
        r"Icon { size: Small | Medium | Large }",
        Key::ExampleIconNoteSize
    ),
    // The box is `1em`, which is the whole reason one class covers every
    // context: a heading and a caption size their own icon.
    example!(
        r"Icon { … } inside a Heading and inside a caption",
        Key::ExampleIconNoteInherit
    ),
];

// --- Rating -----------------------------------------------------------------

const RATING: &[Example] = &[
    // A READING, AND THE NUMBER IS WRITTEN OUT. The fifth star is filled three
    // tenths of the way, because rounding it would be a picture that disagrees
    // with the sentence beside it.
    example!(
        r#"Rating { name, label, readonly: true, value: "4.3", max: "5", value_label }"#,
        Key::ExampleRatingNoteReading
    ),
    // THE QUESTION. Real radios in a real fieldset, so it submits with no
    // script, and the number follows the reader's own choice.
    example!(
        r"Rating { item_label, value_label, hint, required: true, required_label }",
        Key::ExampleRatingNoteAsk
    ),
    example!(
        r#"Rating { name, label, readonly: true, value: "4", tone: "warning", size: Small | Medium | Large, value_label }"#,
        Key::ExampleRatingNoteSize
    ),
    // A REFUSED ANSWER MARKS EVERY STAR, because the score is one value. And a
    // rating nobody may change any more takes no focus at all.
    example!(
        r"Rating { name, label, value_label, item_label, error }
Rating { name, label, value, value_label, item_label, hint, disabled: true }",
        Key::ExampleRatingNoteRefused
    ),
];

// --- OtpField ---------------------------------------------------------------

const OTP_FIELD: &[Example] = &[
    // SIX BOXES, ONE VALUE. Every box posts under the same name, so the code
    // reaches a server whether or not the script arrived.
    example!(
        r"OtpField { name, legend, box_label, hint, invalid_label, required, required_label }",
        Key::ExampleOtpFieldNoteCode
    ),
    example!(
        r#"OtpField { kind: Alphanumeric, length: "4", value: "AB12" }"#,
        Key::ExampleOtpFieldNoteLetters
    ),
    example!(
        r"OtpField { size: Small | Medium | Large }",
        Key::ExampleOtpFieldNoteSize
    ),
    example!(
        r"OtpField { name, legend, box_label, length, value, error }
OtpField { name, legend, box_label, length, autocomplete: false, disabled: true }",
        Key::ExampleOtpFieldNoteRefused
    ),
];
// --- Tabs -------------------------------------------------------------------

const TABS: &[Example] = &[
    // ONE STRIP, THREE PANELS. With no script every panel is on the page and
    // each tab is a link down to the one it names.
    example!(
        r"Tabs { name, label } / Tab { selected: true } / TabPanel { … }",
        Key::ExampleTabsNoteStrip
    ),
    // MANUAL ACTIVATION, which is the one place a tab differs from a radio:
    // the arrows move the focus and wait to be told.
    example!(
        r"Tabs { activation: Manual, appearance: Enclosed, labelledby }",
        Key::ExampleTabsNoteManual
    ),
    example!(
        r"Tabs { orientation: Vertical, size: Small }",
        Key::ExampleTabsNoteVertical
    ),
    example!(r"Tabs { fill: Equal }", Key::ExampleTabsNoteFill),
];

// --- Tab --------------------------------------------------------------------

const TAB: &[Example] = &[
    example!(
        r"Tab { name, value, label, selected: true }",
        Key::ExampleTabNoteLabel
    ),
    example!(r#"Tab { icon: "settings" }"#, Key::ExampleTabNoteIcon),
    example!(r#"Tab { count: "4" }"#, Key::ExampleTabNoteCount),
    example!(r"Tab { disabled: true }", Key::ExampleTabNoteDisabled),
];

// --- TabPanel ---------------------------------------------------------------

const TAB_PANEL: &[Example] = &[
    example!(
        r"TabPanel { name, value, … } after the strip",
        Key::ExampleTabPanelNoteAfter
    ),
    example!(
        r"TabPanel { … } holding more than a sentence",
        Key::ExampleTabPanelNoteRich
    ),
    example!(
        // THE PANEL SHOWN IS THE ONE THE `value` NAMES, not the first one
        // written. This example used to be example 1 again with a different
        // `name`, under a caption claiming every panel is visible with no
        // script — which the rendered page, where the script runs, disproves
        // four inches below the sentence.
        r#"Tab { value: "delivery", selected: true } / TabPanel { name, value } three times"#,
        Key::ExampleTabPanelNoteChosen
    ),
    // A SET INSIDE A PANEL, which works because both ids come from a `name`
    // and this one differs from the set around it.
    example!(
        r"TabPanel { … Tabs { name: … } }",
        Key::ExampleTabPanelNoteNested
    ),
];

// --- Divider ----------------------------------------------------------------

const DIVIDER: &[Example] = &[
    // THE ONE CASE THIS COMPONENT IS FOR: two blocks that already have names,
    // with the rule drawing the boundary the names claim. The settings panel
    // reached the same arrangement from the other direction, and its own
    // comment in `shell.css` is the argument.
    example!(
        // The caption named `weight: Hairline`, which the body never set and
        // which is the default in any case, so it claimed two props where one
        // was passed. `weight` is shown at a value that differs below.
        r"Divider { spacing: DividerSpacing::Loose }",
        Key::ExampleDividerNoteNamed
    ),
    // A WORD ON THE RULE, which is a different element and a different
    // accessible name. `or` between two ways of doing the same thing.
    example!(r#"Divider { label: "or" }"#, Key::ExampleDividerNoteLabel),
    // SIDEWAYS, between two groups sharing a line. Its height comes from the
    // row, so it is right whatever is standing beside it.
    example!(
        r"Divider { orientation: Vertical, weight: Strong }",
        Key::ExampleDividerNoteVertical
    ),
    // THE THREE THICKNESSES, ITERATED RATHER THAN LISTED, so this cannot fall
    // out of step with the enum the day a fourth border token arrives.
    example!(
        r"Divider { weight, spacing: Tight } for weight in DividerWeight::ALL",
        Key::ExampleDividerNoteWeight
    ),
    // OUT OF THE TREE ALTOGETHER. The heading above it already draws the
    // boundary, so the rule repeats it for the eye and says nothing twice.
    example!(
        r"Divider { decorative: true, spacing: Tight }",
        Key::ExampleDividerNoteDecorative
    ),
];

// --- Label ------------------------------------------------------------------

const LABEL: &[Example] = &[
    // A LABEL NAMING A CONTROL IT DOES NOT DRAW. The `<input>` here is a bare
    // one on purpose: `Field` and `Input` already compose this component, and
    // an example of them would be an example of them.
    example!(
        r#"Label { text, for_control: "…" }"#,
        Key::ExampleLabelNoteFor
    ),
    // A GROUP'S NAME HAS TO BE A `<legend>`. Nothing else attaches a question
    // to every control under it, in every engine, with nothing to wire.
    example!(
        r"Label { kind: Legend, text, required, required_label }",
        Key::ExampleLabelNoteLegend
    ),
    // AND A GROUP THAT IS NOT A `<fieldset>` IS NAMED BY REFERENCE. A
    // `role="radiogroup"` is not labelable, so a `for` aimed at one names
    // nothing whatsoever.
    example!(
        r#"Label { kind: Name, for_control: "…" } / Segmented { labelledby }"#,
        Key::ExampleLabelNoteName
    ),
    // ONE WORD OR THE OTHER, NEVER BOTH AND NEVER AN ASTERISK.
    example!(
        r"Label { required, required_label } / Label { optional_label }",
        Key::ExampleLabelNoteMark
    ),
    // THE THREE SIZES, ITERATED, for the reason `Input` states: this cannot
    // fall out of step with the enum the day a fourth is added.
    example!(
        r"Label { size } for size in LabelSize::ALL",
        Key::ExampleLabelNoteSize
    ),
];

// --- Image ------------------------------------------------------------------

const IMAGE: &[Example] = &[
    // THE BOX BEFORE THE BYTES, which is the whole of this component. `width`
    // and `height` are the file's own numbers, handed to the browser before any
    // stylesheet has loaded.
    example!(
        r"Image { src, alt, ratio, width, height }",
        Key::ExampleImageNoteBox
    ),
    // A CAPTION MAKES IT A `<figure>`, which is what attaches the words to the
    // picture rather than leaving them under it.
    example!(
        r"Image { caption, ratio: Square }",
        Key::ExampleImageNoteFigure
    ),
    // AN EMPTY `alt` IS AN ANSWER, and it is the right one here: the sentence
    // beside the picture already says who this is.
    example!(
        r#"Image { alt: "", rounded: true, ratio: Square, loading: Eager }"#,
        Key::ExampleImageNoteEmpty
    ),
    // ROUNDED AND `contain`, for a drawing whose edges carry meaning. Cropping
    // a mark to fill a box is how a mark stops being the mark.
    example!(
        r"Image { rounded: true, fit: Contain, ratio: Wide }",
        Key::ExampleImageNoteRounded
    ),
    // THE FOUR SHAPES, ITERATED, over one file. What changes is the box.
    example!(
        r"Image { ratio } for ratio in ImageRatio::ALL",
        Key::ExampleImageNoteRatio
    ),
];

// --- MessageBar -------------------------------------------------------------

const MESSAGE_BAR: &[Example] = &[
    // THE DEFAULT, AND THE QUIET ONE. A status waits for a pause instead of
    // cutting a screen reader off, which is right about something that worked.
    example!(
        r#"MessageBar { tone: "success", icon: IconDrawing::CHECK, title, … }"#,
        Key::ExampleMessageBarNoteStatus
    ),
    // THE RUDE ONE, OPTED INTO. `alert` cuts in, and `live` says how loudly,
    // which is the one case worth overruling the role for.
    example!(
        r#"MessageBar { alert: true, live: Assertive, tone: "danger", icon: IconDrawing::ALERT }"#,
        Key::ExampleMessageBarNoteAlert
    ),
    // ONE A READER CAN TAKE AWAY. The control ships hidden and the script
    // reveals it, so a page with no script carries no button that closes
    // nothing.
    example!(
        r#"MessageBar { dismissible: true, dismiss_label, tone: "warning", icon: IconDrawing::WARNING }"#,
        Key::ExampleMessageBarNoteDismiss
    ),
    // NO TONE AT ALL, which is the absence of the attribute rather than a name
    // called `default`. The words do the work on their own.
    example!(r"MessageBar { … }", Key::ExampleMessageBarNotePlain),
];

// --- Dialog -----------------------------------------------------------------

/// Four dialogs, and every one of them is CLOSED on the page.
///
/// That is the component working rather than the examples falling short: a
/// `<dialog>` is `display: none` until it is opened, so what a reader sees here
/// is the control, and pressing it is the example. Four distinct `name` values,
/// because every id on this page comes from one of them.
const DIALOG: &[Example] = &[
    // THE ORDINARY ONE, AND EVERY NAMING PROP AT ONCE: a visible heading names
    // the surface, a sentence describes it, and the way out has a name.
    example!(
        r"Dialog { name, trigger_label, heading, description, close_label, … }",
        Key::ExampleDialogNoteModal
    ),
    // BESIDE THE PAGE RATHER THAN OVER IT. `show()` leaves the document
    // reachable, so the reader can keep reading the thing the notes are about.
    example!(
        r"Dialog { modal: false, size: DialogSize::Small, … }",
        Key::ExampleDialogNoteBeside
    ),
    example!(
        r"Dialog { size: DialogSize::Large, dismissible: false, … }",
        Key::ExampleDialogNoteWide
    ),
    // NAMED BY `label`, because there is no title to name it from.
    example!(r#"Dialog { label: "…", … }"#, Key::ExampleDialogNotePlain),
];

// --- Popover ----------------------------------------------------------------

const POPOVER: &[Example] = &[
    // CONTROLS IN IT, which is the difference between this and an `Info`.
    example!(
        r"Popover { name, trigger_label, heading, close_label, … }",
        Key::ExamplePopoverNoteControls
    ),
    example!(
        r"Popover { placement: PopoverPlacement::End, … }",
        Key::ExamplePopoverNoteEnd
    ),
    example!(
        r"Popover { dismissible: false, … }",
        Key::ExamplePopoverNoteFixed
    ),
    example!(r#"Popover { label: "…", … }"#, Key::ExamplePopoverNotePlain),
];

// --- Tooltip ----------------------------------------------------------------

const TOOLTIP: &[Example] = &[
    // THE HINT IS NEVER THE NAME. The button says `Copy` on it, and the hint
    // narrows what gets copied.
    example!(
        r#"Tooltip { name, text: "…", Button { … } }"#,
        Key::ExampleTooltipNoteHint
    ),
    example!(
        r"Tooltip { delay: TooltipDelay::Long, … }",
        Key::ExampleTooltipNoteLong
    ),
    example!(
        r"Tooltip { placement: TooltipPlacement::End, … }",
        Key::ExampleTooltipNoteEnd
    ),
    // AN ICON-ONLY CONTROL, which is where a tooltip is most tempting and most
    // dangerous: `label` is still the name, and the hint only describes.
    example!(
        r"Tooltip { delay: TooltipDelay::None, … Button { hide_label: true } }",
        Key::ExampleTooltipNoteNone
    ),
];

// --- Toast ------------------------------------------------------------------

/// Four messages, each pinned to a corner of its own example rather than to a
/// corner of the window.
///
/// `.nd-example-toasts` is `contain: layout`, which makes it the containing
/// block for the `position: fixed` message inside it. Without that, four toasts
/// would stack in one corner of the page and cover the documentation about
/// them. `shell.css` says the same thing beside the rule.
const TOAST: &[Example] = &[
    // NO TIMER, which is the default: the message stays until it is dismissed.
    example!(
        r"Toast { name, title, dismiss_label, … }",
        Key::ExampleToastNoteStays
    ),
    // The countdown is the component's and the replay control is this site's.
    // `spec/components/toast.toml` writes down which is which and why.
    example!(
        r#"Toast { tone: "success", duration: "6000", countdown_label, … } — with a control that runs it again"#,
        Key::ExampleToastNoteTimer
    ),
    example!(
        r#"Toast { tone: "danger", live: Assertive, placement: TopEnd }"#,
        Key::ExampleToastNoteLoud
    ),
    // A MESSAGE WITH SOMETHING TO DO IN IT, which `children`'s own contract
    // allows in as many words: "it may hold a `Link` to whatever a reader has
    // to do about it". Nothing demonstrated it, and two defects lived in that
    // gap — `toast.js` flattened the body to text on the one path that matters
    // and deleted the link, and its pause counter could never come back to
    // zero once a message held more than one focusable thing.
    example!(
        r#"Toast { tone: "danger", duration: "6000", … } — with a Link in it"#,
        Key::ExampleToastNoteAction
    ),
    example!(
        r"Toast { dismissible: false, placement: ToastPlacement::BottomStart }",
        Key::ExampleToastNoteFixed
    ),
];

// --- Abbr -------------------------------------------------------------------

const ABBR: &[Example] = &[
    example!(r#"Abbr { title: "Cascading Style Sheets", "CSS" }"#),
    example!(r#"Abbr { title: "…", "WCAG" }"#, Key::ExampleAbbrTouch),
    example!(r"Heading { … Abbr { … } }"),
    example!(r"Two in one sentence"),
];

// --- Blockquote -------------------------------------------------------------

const BLOCKQUOTE: &[Example] = &[
    example!(r"Blockquote { … }"),
    example!(r"Blockquote { tone: BlockquoteTone::Accent, … }"),
    example!(r#"Blockquote { attribution: "…", … }"#),
    example!(
        r#"Blockquote { cite: "…", attribution: "…", … }"#,
        Key::ExampleQuoteCite
    ),
    example!(r"Blockquote { … Link { … } }"),
    example!(r"Blockquote { glass: true, … }", Key::ExampleGlassBehind),
];

// --- Button -----------------------------------------------------------------

const BUTTON: &[Example] = &[
    example!(r#"Button { label: "…" }"#),
    example!(
        r"Button { variant: ButtonVariant::… }",
        Key::ExampleButtonVariants
    ),
    example!(r"Button { tone: ButtonTone::… }", Key::ExampleButtonTones),
    example!(r"Button { size: ButtonSize::… }"),
    example!(
        r"Button { shape: ButtonShape::… }",
        Key::ExampleButtonShapes
    ),
    example!(r"Button { icon: IconDrawing::CHECK, … }"),
    example!(
        // THE CAPTION NAMED AN ICON THE EXAMPLE DID NOT DRAW. It said
        // `ChevronDown` and rendered `ExternalLink`, which is a caption a reader
        // has no way to check and every reason to believe.
        r"Button { icon_end: IconDrawing::EXTERNAL_LINK | ::ChevronRight, … }",
        Key::ExampleButtonIconEnd
    ),
    example!(
        r"Button { hide_label: true, icon: …, … }",
        Key::ExampleButtonIconOnly
    ),
    example!(
        r#"Button { detail: "…", justify: ButtonJustify::Start, … }"#,
        Key::ExampleButtonCompound
    ),
    example!(
        r"Button { wrap: true, fill: true, … }",
        Key::ExampleButtonWrap
    ),
    example!(
        r#"Button { menu: true, expanded, controls: "…" }"#,
        Key::ExampleButtonMenu
    ),
    example!(
        r#"Button { href: "…", rel: "noreferrer", target: "_blank" }"#,
        Key::ExampleButtonLink
    ),
    example!(
        r#"Button { pressed: true, icon_alt: …, label_alt: "…", action: "…" }"#,
        Key::ExampleButtonPressed
    ),
    example!(r"Button { disabled: true, … }", Key::ExampleButtonDisabled),
    example!(
        r#"Button { submit: ButtonSubmit::Submit, name: "…", value: "…" }"#,
        Key::ExampleButtonSubmit
    ),
    example!(
        r"Button { variant: ButtonVariant::Nav, current: true, … }",
        Key::ExampleButtonNav
    ),
    example!(r"Button { glass: true, … }", Key::ExampleGlassBehind),
];

// --- ButtonGroup ------------------------------------------------------------

const BUTTON_GROUP: &[Example] = &[
    example!(
        r#"ButtonGroup { label: "…", Button { … } }"#,
        Key::ExampleGroupAttached
    ),
    example!(r"ButtonGroup { attached: false, … }"),
    example!(
        r"ButtonGroup { orientation: ButtonGroupOrientation::Vertical, … }",
        Key::ExampleGroupVertical
    ),
    example!(
        r"ButtonGroup { orientation: …Vertical, justify: ButtonGroupJustify::Start, … }",
        Key::ExampleGroupJustify
    ),
    example!(r"ButtonGroup { fill: true, … }", Key::ExampleGroupFill),
    example!(
        r#"ButtonGroup { labelledby: "…", … } — a split button"#,
        Key::ExampleGroupSplit
    ),
];

// --- Segmented and SegmentedItem ---------------------------------------------

const SEGMENTED: &[Example] = &[
    example!(
        r#"Segmented { name: "…", label: "…", SegmentedItem { … } }"#,
        Key::ExampleSegmentedTabStop
    ),
    example!(
        r"Segmented { appearance: SegmentedAppearance::Spread, … }",
        Key::ExampleSegmentedSpread
    ),
    example!(r"Segmented { size: SegmentedSize::… }"),
    example!(
        r"Segmented { shape: SegmentedShape::… }",
        Key::ExampleSegmentedShapes
    ),
    example!(
        r"Segmented { orientation: SegmentedOrientation::Vertical, … }",
        Key::ExampleSegmentedVertical
    ),
    example!(
        r"Segmented { name, label, fill: SegmentedFill::Equal }
Segmented { name, label, fill: SegmentedFill::Stretch }",
        Key::ExampleSegmentedEqual
    ),
    example!(
        r#"Segmented { labelledby: "…", … }"#,
        Key::ExampleSegmentedLabelledby
    ),
    example!(r"Segmented { glass: true, … }", Key::ExampleGlassBehind),
    example!(
        r"Segmented { selection: SegmentedSelection::Quiet, … }",
        Key::ExampleSegmentedQuiet
    ),
    // THE UNANSWERED CASE, WHICH THE PAGE HAD STOPPED SHOWING.
    //
    // `a group with nothing chosen is still one tab stop` needs a group with
    // nothing chosen to be true of, and the site had run out of them: the scope
    // controls were made to state their answer, and every example here chose
    // one. The guarantee is real, so the subject goes back rather than the test
    // being relaxed to whatever the page happens to contain.
    example!(
        r"Field { label, hint } / Segmented { name, label } with no selected item",
        Key::ExampleSegmentedUnanswered
    ),
];

const SEGMENTED_ITEM: &[Example] = &[
    example!(r#"SegmentedItem { value: "…", label: "…" }"#),
    example!(
        r"SegmentedItem { selected: true, … }",
        Key::ExampleItemSelected
    ),
    example!(
        r"SegmentedItem { icon: …, hide_label: true, … }",
        Key::ExampleItemIcons
    ),
    example!(
        r"SegmentedItem { disabled: true, … }",
        Key::ExampleItemDisabled
    ),
    example!(r"SegmentedItem { Swatch { … } }", Key::ExampleItemSwatch),
    example!(r"SegmentedItem { glass: true, … }", Key::ExampleGlassBehind),
    example!(
        r#"SegmentedItem { label: "320", detail: "px" }"#,
        Key::ExampleItemDetail
    ),
];

// --- Switch -------------------------------------------------------------------

const SWITCH: &[Example] = &[
    example!(
        r#"Switch { name: "…", on: "on", off: "off", label: "…" }"#,
        Key::ExampleSwitchValues
    ),
    example!(r"Switch { checked: true, … }", Key::ExampleSwitchState),
    example!(
        r#"Switch { describedby: "…", … }"#,
        Key::ExampleSwitchDescribed
    ),
    example!(r"Switch { size: SwitchSize::… }"),
    example!(r"Switch { disabled: true, … }", Key::ExampleSwitchRefused),
    example!(r"Switch { glass: true, … }", Key::ExampleGlassBehind),
];
const TOOLBAR: &[Example] = &[
    example!(
        r#"Toolbar { label: "…", ToolbarGroup { … } }"#,
        Key::ExampleToolbarStops
    ),
    example!(
        r"Toolbar { orientation: ToolbarOrientation::Vertical, … }",
        Key::ExampleToolbarVertical
    ),
    example!(
        r"Toolbar { density: ToolbarDensity::Compact, wrap: false, … }",
        Key::ExampleToolbarDensity
    ),
    example!(r#"Toolbar { labelledby: "…", … }"#),
    example!(r"Toolbar { glass: true, … }", Key::ExampleGlassBehind),
];

const TOOLBAR_GROUP: &[Example] = &[
    example!(
        r"ToolbarGroup { … }
ToolbarGroup { align: ToolbarGroupAlign::End, … }",
        Key::ExampleToolbarAlign
    ),
    example!(r"ToolbarGroup { … }"),
    example!(r#"ToolbarGroup { label: "…", … }"#),
    // THREE GROUPS, AND ONE `align` PLACES TWO OF THEM. Example 1 is two
    // groups at the two ends and this used to be the identical arrangement
    // with different buttons in it. The claim worth its own example is the
    // rest of the sentence in the contract: `end` moves this group **and
    // everything after it**.
    example!(
        r"ToolbarGroup { … } × 3, with align: ToolbarGroupAlign::End on the second",
        Key::ExampleToolbarAlignRest
    ),
];

// --- Swatch -----------------------------------------------------------------

// --- Swatch: never on its own ------------------------------------------------
//
// THE CHIP IS ALWAYS `aria-hidden`, so a bare one is a stage that is empty to a
// screen reader. The contract says so in writing: *never use it on its own,
// because the chip has no accessible name of its own.* Every example here
// therefore puts the chip beside the word that names it, which is also the
// only arrangement worth copying.

const SWATCH: &[Example] = &[
    example!(r#"Swatch { }  ·  Text { "…" }"#, Key::ExampleSwatchLive),
    example!(
        r#"Tag { swatch: "rose-vivid", label: "…" }"#,
        Key::ExampleSwatchNamed
    ),
    example!(r#"Swatch { size: SwatchSize::…, palette: "amber-vivid" }  ·  Code { "…" }"#),
    example!(
        r#"Swatch { shape: SwatchShape::…, size: SwatchSize::Large, palette: "teal-balanced" }  ·  Code { "…" }"#
    ),
];

// --- Checkbox, Radio and Fieldset -------------------------------------------

// EVERY GROUP ON THIS PAGE HAS A `name` OF ITS OWN, and that is not tidiness.
//
// Radios that share a `name` are ONE group as far as the browser is concerned,
// wherever they sit in the document — so two examples both calling their group
// `contact` would be six answers to one question, and choosing in the second
// example would silently clear the first. The ids follow from the name, so
// distinct names also mean distinct ids, which is what the labels point at.

/// Four fields, which between them set every prop the contract declares.
const DATE_INPUT: &[Example] = &[
    example!(
        r"DateInput { name, legend, hint, day_label, month_label, year_label, bday: true, required: true, required_label, missing_label, number_label, invalid_label, error_label }",
        Key::ExampleDateInputBirthNote
    ),
    example!(
        r"DateInput { min, max, min_label, max_label, year_digits_label, calendar: true, suggest_label, previous_label, next_label, Time { … } }",
        Key::ExampleDateInputVisitNote
    ),
    example!(
        r#"DateInput { value: "2026-08-07", order: Ymd, pad: true, expand: true }"#,
        Key::ExampleDateInputFilledNote
    ),
    example!(
        r"DateInput { size: Small }  ·  { size: Large, glass: true }  ·  { disabled: true }  ·  { error }",
        Key::ExampleDateInputFormsNote
    ),
];

const CHECKBOX: &[Example] = &[
    example!(r#"Fieldset { … } / Checkbox { name: "apply", value: "study", label: "…" }"#),
    // THE THREE SIZES AS REAL CONTROLS, not only as specimens in the variant
    // grid. The grid draws the box alone, which is what makes the prop legible
    // — but the claim the sizes exist for is that the *target* stays 44 pixels
    // while the box shrinks, and a drawn box with no input behind it cannot
    // show that. It is also what the browser suite measures.
    example!(
        r"Checkbox { size: CheckboxSize::Large | Medium | Small }",
        Key::ExampleFormsNoteSizes
    ),
    example!(
        r#"Checkbox { hint: "…", size: CheckboxSize::Small }"#,
        Key::ExampleFormsNoteHints
    ),
    example!(
        r#"Checkbox { divider: "…", exclusive: true }"#,
        Key::ExampleFormsNoteExclusive
    ),
    example!(r"Checkbox { reveal: true, … }", Key::ExampleFormsNoteReveal),
    example!(
        r"Checkbox { checked: true, disabled: true, invalid: true, hide_label }",
        Key::ExampleFormsNoteTarget
    ),
];

const RADIO: &[Example] = &[
    example!(
        r#"Fieldset { … } / Radio { name: "contact", value: "email", label: "…", checked: true }"#
    ),
    // The three sizes as real controls. See `Checkbox` above for why the
    // variant grid is not enough on its own.
    example!(
        r"Radio { size: RadioSize::Large | Medium | Small }",
        Key::ExampleFormsNoteSizes
    ),
    example!(
        r#"Radio { hint: "…", size: RadioSize::Medium }"#,
        Key::ExampleFormsNoteHints
    ),
    example!(
        r#"Radio { divider: "…", reveal: true, … }"#,
        Key::ExampleFormsNoteDivider
    ),
    example!(
        r"Radio { disabled: true, invalid: true, hide_label }",
        Key::ExampleFormsNoteTarget
    ),
];

const FIELDSET: &[Example] = &[
    example!(r#"Fieldset { name: "…", legend: "…", hint: "…" }"#),
    example!(
        r#"Fieldset { error: "…", error_label: "…" }"#,
        Key::ExampleFormsNoteError
    ),
    example!(r"Fieldset { inline: true, … }", Key::ExampleFormsNoteInline),
    example!(
        r"Fieldset { heading: true, … }",
        Key::ExampleFormsNoteHeading
    ),
    example!(
        r"Fieldset { disabled: true, … }",
        Key::ExampleFormsNoteDisabled
    ),
];

// --- Info -------------------------------------------------------------------

const INFO: &[Example] = &[
    example!(
        r"Info { name, label } beside the word it explains",
        Key::ExampleInfoDeadlineHint
    ),
    example!(r"Info { heading }", Key::ExampleInfoUsageHint),
    example!(r"Info { close_label }", Key::ExampleInfoCloseHint),
    example!(r"Info { placement: InfoPlacement::End } at the edge of a table"),
];

// --- Meter and Progress -----------------------------------------------------

// REAL READINGS. A disk that is 82% full, a token quota past its threshold, a
// team with four seats left. A bar of invented numbers is a bar nobody can tell
// is wrong, and this component's whole claim is that the number is written out.

const METER: &[Example] = &[
    example!(
        r"Meter { name, label, value, max, unit, value_label }",
        Key::ExampleMeterStorageNote
    ),
    example!(
        r"Meter { name, label, value, max, low, high, optimum, value_label }",
        Key::ExampleMeterTokensNote
    ),
    example!(
        r"Meter { name, label, value, min, max, unit, size, value_label }",
        Key::ExampleMeterFreezerNote
    ),
    example!(
        r#"Meter { name, label, value, tone: "…", size: MeterSize::Large, value_label }"#,
        Key::ExampleMeterShareNote
    ),
    example!(
        r"Meter { name, label, value, max, unit, hide_label: true, show_value: false, size, value_label }",
        Key::ExampleMeterHiddenNote
    ),
];

const PROGRESS: &[Example] = &[
    example!(
        r"Progress { name, label, value, max, value_label }",
        Key::ExampleProgressUploadNote
    ),
    example!(
        r"Progress { busy_label } — no value at all",
        Key::ExampleProgressImportNote
    ),
    example!(
        r#"Progress { tone: "danger" }"#,
        Key::ExampleProgressFailedNote
    ),
    example!(
        r"Progress { name, label, value, max, size: Large, value_label }
Progress { name, label, value, max, size: Small, hide_label: true, value_label }",
        Key::ExampleProgressStepsNote
    ),
];

// --- Pagination -------------------------------------------------------------

const PAGINATION: &[Example] = &[
    example!(
        r"Pagination { name, label, page, pages, href, previous_label, next_label, page_label, current_label }",
        Key::ExamplePaginationNoteLinks
    ),
    example!(
        r"Pagination { spread: PaginationSpread::Compact, more_label, previous_label, next_label }",
        Key::ExamplePaginationNoteWindow
    ),
    example!(r"Pagination { spread: PaginationSpread::Wide, size: PaginationSize::Small }"),
    example!(
        r"At the last page, with a summary",
        Key::ExamplePaginationNoteEnds
    ),
];
const TABLE_COLUMN: &[Example] = &[
    example!(
        r"TableColumn { low, high, tone_below, tone_within, tone_above }",
        Key::ExampleTableNoteStatus
    ),
    example!(
        r"TableColumn { name, width: TableColumnWidth::Fit | ::Wide }",
        Key::ExampleTableNoteColumnWidth
    ),
    example!(
        r"TableColumn { sortable, sort_as: ::Number | ::Date }",
        Key::ExampleTableNoteColumnSort
    ),
    example!(
        r"TableColumn { filterable, hideable, hidden }",
        Key::ExampleTableNoteColumnHide
    ),
    example!(
        r"TableColumn { resizable, pinned } and the three status tones",
        Key::ExampleTableNoteDeploys
    ),
];

const TABLE_SECTION: &[Example] = &[
    example!(
        r"TableSection { kind: TableSectionKind::Head }",
        Key::ExampleTableNoteSectionHead
    ),
    example!(
        r"TableSection { kind: TableSectionKind::Foot } with an aggregate",
        Key::ExampleTableNoteSectionFoot
    ),
    example!(
        r"TableSection { name, label, collapsible } — several groups, each folding",
        Key::ExampleTableNoteGroups
    ),
    example!(
        r"TableSection { children } — a table with no head at all",
        Key::ExampleTableNoteSectionNone
    ),
];

const TABLE: &[Example] = &[
    example!(
        r"Table { name, caption, note } with a Tag in a cell",
        Key::ExampleTableNoteInvoices
    ),
    example!(
        r#"TableColumn { low, tone_below, tone_within } and TableCell { tone: "…" }"#,
        Key::ExampleTableNoteStatus
    ),
    example!(
        r"Table { name, caption, search_label, search_placeholder, filtered_label }
TableRow { emphasis } and TableCell { aggregate }: subtotal, tax, total",
        Key::ExampleTableNoteTotals
    ),
    example!(
        r"A Meter, a Progress and an Info, all inside cells",
        Key::ExampleTableNoteUsage
    ),
    example!(
        r"Table { search_label, search_placeholder, filtered_label, summary_label }",
        Key::ExampleTableNoteSearch
    ),
    example!(
        r"Table { export, export_label, csv_label, json_label, print_label, exported_label, copy, copy_label, copied_label, selected_label }",
        Key::ExampleTableNoteExport
    ),
    example!(
        r"Table { more_src, more_label, more_status_label, total } — load more",
        Key::ExampleTableNoteMore
    ),
    example!(
        r"Table { density, rules, zebra, hover }",
        Key::ExampleTableNoteDensity
    ),
    example!(
        r"Table { sticky_header: true, height: TableHeight::Short }",
        Key::ExampleTableNoteSticky
    ),
    example!(
        r"Table { hide_caption: true, width: TableWidth::Auto, layout: TableLayout::Stack }",
        Key::ExampleTableNoteStack
    ),
    example!(
        r"Table { resize_label, resized_label, columns_label, column_shown_label, column_hidden_label }",
        Key::ExampleTableNoteColumns
    ),
    example!(
        r"Table { sort_ascending_label, sort_descending_label, sort_cleared_label }",
        Key::ExampleTableNoteSort
    ),
];

const TABLE_ROW: &[Example] = &[
    example!(
        r#"TableRow { tone: "…" } — a wash and a rule down the leading edge"#,
        Key::ExampleTableNoteRowTone
    ),
    example!(
        r"TableRow { name, expandable, expand_label } with a detail_for row",
        Key::ExampleTableNoteRowExpand
    ),
    example!(
        r"TableRow { select: TableRowSelect::One, select_name, select_label, selected, disabled }",
        Key::ExampleTableNoteRowSelect
    ),
    example!(
        r"TableRow { emphasis: ::Subtle | ::Strong } and TableCell { aggregate }",
        Key::ExampleTableNoteRowEmphasis
    ),
    example!(
        r"Table { name, caption, search_label, search_placeholder, filtered_label }
TableRow { empty: true }: the caller's own sentence",
        Key::ExampleTableNoteEmpty
    ),
];

const TABLE_CELL: &[Example] = &[
    example!(
        r#"TableCell { align: ::Numeric, sort_value: "…" }"#,
        Key::ExampleTableNoteCellNumeric
    ),
    example!(
        r"TableCell { wrap: ::Nowrap } beside TableCell { wrap: ::Truncate }",
        Key::ExampleTableNoteCellWrap
    ),
    example!(
        r#"TableCell { span: "2", rows: "2", column: "…" }"#,
        Key::ExampleTableNoteCellSpan
    ),
    example!(
        r#"TableCell { label, tone: "…" } — the stacked layout's own label"#,
        Key::ExampleTableNoteCellLabel
    ),
    example!(
        r"Table { name, caption, search_label, search_placeholder, filtered_label }
TableCell { aggregate, aggregate_over }: a total that recounts",
        Key::ExampleTableNoteCellAggregate
    ),
];

const TABLE_HEADER_CELL: &[Example] = &[
    example!(
        r"TableHeaderCell { scope: ::Col | ::Row }",
        Key::ExampleTableNoteHeaderScope
    ),
    example!(
        r"TableHeaderCell { column, sort: ::Ascending } — the order a server applied",
        Key::ExampleTableNoteHeaderSort
    ),
    example!(
        r#"TableHeaderCell { align, wrap, tone: "…" }"#,
        Key::ExampleTableNoteHeaderAlign
    ),
    example!(
        r#"TableHeaderCell { span: "2", rows: "2", scope: ::Colgroup }"#,
        Key::ExampleTableNoteHeaderGroup
    ),
];
const MENU: &[Example] = &[
    example!(
        r#"Menu { name, label } under a Button { menu: true, controls: "…" }"#,
        Key::ExampleMenuNoteBasic
    ),
    example!(
        r"Menu { placement: MenuPlacement::End }",
        Key::ExampleMenuNotePlacement
    ),
    example!(
        // A MENU LONG ENOUGH FOR THE WALK TO BE WORTH ANYTHING. This example
        // used to be example 1 with a fourth item, under a caption promising
        // *every key a menu owes* and markup showing none of them. Thirteen
        // items with thirteen different first letters is where Home, End and
        // the letter jump stop being claims and start being visible.
        r"Menu { name, label } with thirteen MenuItem { label }",
        Key::ExampleMenuNoteKeys
    ),
    example!(r"Menu, twice on one page", Key::ExampleMenuNoteOne),
];

const MENU_ITEM: &[Example] = &[
    example!(
        r#"MenuItem { icon, label, action: "…", value: "…" }"#,
        Key::ExampleMenuNoteIcon
    ),
    example!(
        r#"MenuItem { tone: "danger", detail: "…", action: "…" }"#,
        Key::ExampleMenuNoteItem
    ),
    example!(r"MenuItem { disabled: true }", Key::ExampleMenuNoteRefused),
    example!(
        r#"MenuItem { href: "…", rel, target }"#,
        Key::ExampleMenuNoteLinks
    ),
];
const SELECT: &[Example] = &[
    example!(
        r#"Select { name: "palette", label: "…", hint: "…", placeholder: "…" }"#,
        Key::ExampleSelectNoteNative
    ),
    example!(
        r"Select { searchable: true, search_label, empty_label, clearable: true, clear_label, size: SelectSize::Large }",
        Key::ExampleSelectNoteSearch
    ),
    example!(
        r"Select { multiple: true, required: true, selected_label }",
        Key::ExampleSelectNoteMultiple
    ),
    example!(
        r"Select { creatable: true, create_label }",
        Key::ExampleSelectNoteCreate
    ),
    example!(
        r"Select { required: true, … } inside a <form>",
        Key::ExampleSelectNoteForm
    ),
    example!(
        r#"Select { error: "…", error_label: "…" }"#,
        Key::ExampleSelectNoteError
    ),
    example!(
        r"Select { loading: true, loading_label, size: SelectSize::Small }",
        Key::ExampleSelectNoteLoading
    ),
    // NOT ALSO DISABLED, which the loading example used to be.
    //
    // A disabled control cannot be opened, so the one sentence the loading
    // state exists to say was in the page and unreachable — a demonstration of
    // a feature that demonstrated nothing. `disabled` gets an example of its
    // own instead, where it is the whole point.
    example!(r"Select { disabled: true }", Key::ExampleSelectNoteDisabled),
];

const SELECT_GROUP: &[Example] = &[
    example!(r#"SelectGroup { label: "…" }"#, Key::ExampleSelectNoteGroup),
    example!(r"Two runs, one list"),
    example!(
        r"SelectGroup { disabled: true, … }",
        Key::ExampleSelectNoteDisabledGroup
    ),
    example!(r"A run beside ungrouped options"),
];

const SELECT_OPTION: &[Example] = &[
    example!(
        r#"SelectOption { value: "…", label: "…" }"#,
        Key::ExampleSelectNotePlain
    ),
    example!(
        r#"SelectOption { detail: "…", swatch: "…" }"#,
        Key::ExampleSelectNoteRich
    ),
    example!(
        r"SelectOption { icon: IconDrawing::SUN, … }",
        Key::ExampleSelectNoteIcon
    ),
    example!(
        r"SelectOption { selected: true, disabled: true }",
        Key::ExampleSelectNoteSelected
    ),
];

// --- Code -----------------------------------------------------

const CODE: &[Example] = &[
    example!(r#"Code { "…" }"#),
    example!(r"Code { tone: CodeTone::Subtle, … }"),
    example!(r"Heading { … Code { … } }"),
    example!(r"A long name, in a narrow column", Key::ExampleCodeLong),
];

// --- CodeBlock --------------------------------------------------------------

const CODE_BLOCK: &[Example] = &[
    example!(r#"CodeBlock { code: "…", language: "rust" }"#),
    example!(
        r#"CodeBlock { language: "…" }, thirty-four of them"#,
        Key::ExampleCodeBlockLanguages
    ),
    example!(
        r#"CodeBlock { code: "…", language: "rust", theme: CodeBlockTheme::… }"#,
        Key::ExampleCodeBlockThemes
    ),
    example!(
        r#"CodeBlock { numbered: true, start_line: "…", mark_lines: "…", focus: true }"#,
        Key::ExampleCodeBlockLines
    ),
    example!(
        r#"CodeBlock { language: "diff", … }"#,
        Key::ExampleCodeBlockDiff
    ),
    example!(
        r#"CodeBlock { filename: "…", copy: "…", copied: "…", wrap_label: "…", caption: "…" }"#,
        Key::ExampleCodeBlockBar
    ),
    example!(r"CodeBlock { wrap: true, … }", Key::ExampleCodeBlockWrap),
    example!(
        r#"CodeBlock { max_lines: "…", expand_label: "…", collapse_label: "…" }"#,
        Key::ExampleCodeBlockCollapse
    ),
    example!(
        r"A language nothing recognises",
        Key::ExampleCodeBlockUnknown
    ),
    example!(r"CodeBlock { glass: true, … }", Key::ExampleGlassBehind),
    example!(r"Language::ALL", Key::ExampleCodeBlockEvery),
];

// --- Del and Ins ------------------------------------------------------------

const DEL: &[Example] = &[
    example!(r#"Del { "…" }"#),
    example!(r#"Del { label: "removed", … }"#, Key::ExampleEditLabel),
    example!(r#"Del { datetime: "2026-07-29", cite: "…", … }"#),
    example!(r"Del beside Ins", Key::ExampleEditPair),
];

const INS: &[Example] = &[
    example!(r#"Ins { "…" }"#),
    example!(r#"Ins { label: "added", … }"#, Key::ExampleEditLabel),
    example!(r#"Ins { datetime: "2026-07-29", cite: "…", … }"#),
    // NOT `Del` BESIDE `Ins` AGAIN. That example is on `Del`'s own table with
    // the same note key, and one question asked twice on one page is one of
    // the two examples doing nothing. This one is `Ins` around a whole item.
    example!(
        r"List { ListItem { Ins { label, … } } }",
        Key::ExampleEditBlock
    ),
];

// --- Disclosure -------------------------------------------------------------

const DISCLOSURE: &[Example] = &[
    example!(r#"Disclosure { summary: "…", … }"#),
    example!(r"Disclosure { open: true, … }", Key::ExampleDisclosureOpen),
    example!(r"Disclosure { marker: false, … }"),
    example!(
        r#"Disclosure { icon: "settings", … }"#,
        Key::ExampleDisclosureIcon
    ),
    example!(r#"Disclosure { name: "…", … }"#, Key::ExampleDisclosureName),
    example!(
        r"Disclosure { heading: DisclosureHeading::H3, … }",
        Key::ExampleDisclosureHeading
    ),
    example!(r"Disclosure { glass: true, … }", Key::ExampleGlassBehind),
];

// --- Em and Strong ----------------------------------------------------------

const EM: &[Example] = &[
    example!(r#"Em { "…" }"#),
    example!(r"Strong { Em { … } }", Key::ExampleEmNested),
    example!(r"Em inside a heading"),
    example!(r"Em beside Strong", Key::ExampleEmVersus),
];

const STRONG: &[Example] = &[
    example!(r#"Strong { "…" }"#),
    // NOT `Strong { Em { … } }` AGAIN. `Em`'s own table asks that question,
    // under this very note key. The question `Strong` alone can answer is what
    // a second `Strong` inside the first one means.
    example!(r"Strong { … Strong { … } }", Key::ExampleStrongDegrees),
    example!(r"Strong inside a list"),
    // `Em` beside `Strong` is on `Em`'s table already. What belongs here is
    // the case the element exists for: the part of a sentence that matters
    // most, which is usually the word a warning opens on.
    example!(r"Strong opening a warning", Key::ExampleStrongWarning),
];

// --- Heading ----------------------------------------------------------------

const HEADING: &[Example] = &[
    example!(r"An outline: levels 2, 3 and 4", Key::ExampleHeadingOutline),
    example!(
        r"Heading { level: _3, size: HeadingSize::Display, … }",
        Key::ExampleHeadingIndependent
    ),
    example!(r"Heading { tone: HeadingTone::Muted, … }"),
    example!(r"Heading { tone: HeadingTone::Accent, … }"),
    example!(r"Heading { balance: true, … }", Key::ExampleHeadingBalance),
];

// --- Kbd --------------------------------------------------------------------

const KBD: &[Example] = &[
    example!(r#"Kbd { "Esc" }"#),
    example!(r"A chord", Key::ExampleKbdChord),
    example!(r"Kbd { size: KbdSize::Body, … }"),
    example!(r"In a sentence"),
];

// --- Link -------------------------------------------------------------------

const LINK: &[Example] = &[
    example!(r#"Link { href: "…", "…" }"#),
    example!(
        r#"Link { external: true, rel: "noreferrer", … }"#,
        Key::ExampleLinkExternal
    ),
    example!(
        r"Link { tone: LinkTone::Inherit, … }",
        Key::ExampleLinkInherit
    ),
    example!(r"In a sentence"),
];

// --- List and ListItem ------------------------------------------------------

const LIST: &[Example] = &[
    // NO `kind` IN THE CALL AND NONE IN THE CAPTION. `unordered` is the
    // default, so the caption named a prop the body never passed. The two
    // examples below pass the two values that change the element.
    example!(r"List { ListItem { … } × 3 }"),
    example!(
        r"List { kind: ListKind::Ordered, … }",
        Key::ExampleListOrdered
    ),
    example!(
        r"List { kind: ListKind::Description, … }",
        Key::ExampleListDescription
    ),
    example!(
        r"List { marker: ListMarker::None, … }",
        Key::ExampleListMarker
    ),
    example!(r"List { spacing: ListSpacing::Loose, … }"),
    example!(
        r"Ordered inside unordered inside ordered, four deep",
        Key::ExampleListNested
    ),
    example!(
        r"List { levels: true } — a colour for each level",
        Key::ExampleListLevels
    ),
];

const LIST_ITEM: &[Example] = &[
    example!(r#"ListItem { "…" }"#),
    example!(r#"ListItem { term: "…", "…" }"#, Key::ExampleItemTerm),
    example!(r"An item holding a whole paragraph"),
    example!(r"An item holding a link"),
];

// --- Mark -------------------------------------------------------------------

const MARK: &[Example] = &[
    example!(r#"Mark { "…" }"#),
    example!(r"Two hits in one line", Key::ExampleMarkSearch),
    example!(r"Mark inside a heading"),
    example!(r"Mark inside a list"),
];

// --- Paragraph and Prose ----------------------------------------------------

const PARAGRAPH: &[Example] = &[
    example!(r"Paragraph { … }"),
    example!(
        r"Paragraph { size: ParagraphSize::Lead, … }",
        Key::ExampleParagraphLead
    ),
    example!(r"Paragraph { size: ParagraphSize::Small, … }"),
    example!(r"Paragraph { tone: ParagraphTone::Muted, … }"),
    example!(r"Two, one after the other"),
];

const PROSE: &[Example] = &[
    example!(r"Prose { … }", Key::ExampleProseMeasure),
    example!(r"Prose { measure: ProseMeasure::Lead, … }"),
    example!(
        r"Prose { measure: ProseMeasure::None, … }",
        Key::ExampleProseNone
    ),
    example!(r"A whole reading column"),
];
const SEARCH: &[Example] = &[
    example!(
        r#"Search { name: "…", label: "…", placeholder: "…", status: "one:… |other:…" }"#,
        Key::ExampleSearchChildren
    ),
    example!(
        r#"Search { presentation: SearchPresentation::Popover, value: "san", cap: "3", cap_label: "…" }"#,
        Key::ExampleSearchPopover
    ),
    example!(
        r#"Search { presentation: SearchPresentation::Dialog, src: "…", hotkey: true, recent: "…", status, empty_label, loading_label, error_label, cap_label, clear_label, recent_label, forget_label, close_label }"#,
        Key::ExampleSearchDialog
    ),
    example!(
        r#"Search { name, action: "…", query: "term", label, placeholder, clear_label }"#,
        Key::ExampleSearchForm
    ),
    example!(r"Search { filter: true }", Key::ExampleSearchFilter),
    example!(
        r"Search { size: SearchSize::Small } / Medium / Large",
        Key::ExampleSearchSizes
    ),
    example!(
        r"Search { loading: true } / Search { disabled: true } / Search { clearable: false }",
        Key::ExampleSearchStates
    ),
    example!(r"Search { glass: true }", Key::ExampleGlassBehind),
];

const SEARCH_GROUP: &[Example] = &[
    example!(r#"SearchGroup { label: "…" }"#, Key::ExampleSearchGroup),
    example!(r"Two runs, one list"),
    example!(
        r"A run beside ungrouped results",
        Key::ExampleSearchGroupMixed
    ),
    example!(
        r"An emptied group leaves nothing behind",
        Key::ExampleSearchGroupEmpty
    ),
];

const SEARCH_RESULT: &[Example] = &[
    example!(
        r#"SearchResult { href: "…", label: "…" }"#,
        Key::ExampleSearchResult
    ),
    example!(
        r#"SearchResult { detail: "…" }"#,
        Key::ExampleSearchResultDetail
    ),
    example!(
        r#"SearchResult { keywords: "heirloom slicing beefsteak" }"#,
        Key::ExampleSearchResultKeywords
    ),
    example!(
        r#"SearchResult { badge: "…" } / SearchResult { current: true }"#,
        Key::ExampleSearchCurrent
    ),
];

// --- ColorPicker ------------------------------------------------------------

// EVERY WORD IN THESE IS A PROP, which is the contract's own last paragraph
// made visible: the two tab names, the name of each track, the sentence that
// refuses a typed colour and the name of every copy control all arrive from
// the catalogue. A picker whose panel was written in English inside the
// component would ship that English into every locale a consumer has.
//
// The colours are hexes rather than tokens, and that is the component: what
// goes in and what comes out is the CONSUMER's data, which the palette has no
// opinion about. `palette` is the one example where the system's own colours
// appear, and they arrive from `noctua-colors` rather than from this file.

const COLOR_PICKER: &[Example] = &[
    example!(
        r"ColorPicker { name, label, value, trigger_label, area_label, hue_label, entry_label, entry_invalid_label, codes_label, copy_label, copied_label, close_label }",
        Key::ExampleColorPickerPlainNote
    ),
    example!(
        r"ColorPicker { size: Large, shape: Pill, alpha: true, alpha_label, eyedropper: true, eyedropper_label, recent, recent_label, forget_label }",
        Key::ExampleColorPickerExtrasNote
    ),
    example!(
        r"ColorPicker { multiple: true, max, max_label, add_label, remove_label, added_label, removed_label, status }",
        Key::ExampleColorPickerMultipleNote
    ),
    example!(
        r#"ColorPicker { palette: true, palette_label, spectrum_label, formats: "hex oklch token" }"#,
        Key::ExampleColorPickerPaletteNote
    ),
    example!(
        r"ColorPicker { hint, error, required: true, glass: true, entry: false, size: Small } / ColorPicker { disabled: true }",
        Key::ExampleColorPickerFormNote
    ),
];

// --- The input controls -----------------------------------------------------

// ONE FORM PER EXAMPLE, AND A REAL ONE. `novalidate` so the sentence a reader
// hears is the component's own rather than the browser bubble the no-script
// path already gives them — the same rig `DateInput`'s examples use, for the
// reason recorded there.

const FIELD: &[Example] = &[
    example!(r"Field { control, label, hint }", Key::ExampleFieldNote),
    example!(r"Field { required, required_label }"),
    example!(r"Field { error, error_label } / Field { error_slot } / Field { disabled }"),
    example!(r"Field { size }"),
    example!(r"Field { kind: Group } — a `<legend>`, because a `<label>` names one element"),
];

const INPUT: &[Example] = &[
    // ONE FIELD, IN A FORM THAT SUBMITS. Press it and the answer arrives in the
    // address bar, which is the only way to show that the box is a real
    // `<input>` with a real `name` rather than a picture of one.
    example!(
        r"Input { name, label, value, hint, required, required_label, autocomplete }",
        Key::ExampleInputNoteAutocomplete
    ),
    // THE KEYBOARD A PHONE OPENS. Three kinds, three different questions, and
    // three different labels — a row of boxes all called *Website* is the
    // defect this example replaced.
    example!(
        r"Input { kind, placeholder, pattern, spellcheck, prefix }",
        Key::ExampleInputNoteKind
    ),
    // A PASSWORD A READER CAN CHECK BEFORE SENDING IT. Both words ship, because
    // the button says two different things depending on which state it is in.
    example!(
        r"Input { kind: Password, reveal_label, conceal_label }",
        Key::ExampleInputNotePassword
    ),
    // WHAT THE BOX CARRIES BESIDE THE TEXT: ONE ORNAMENT AT EACH END, AND A
    // COUNT OF WHAT IS BETWEEN THEM. See the tree for why the `@` went.
    example!(
        r"Input { icon, suffix, value, clearable, clear_label, maxlength, counter, counter_label }",
        Key::ExampleInputNoteExtras
    ),
    // THE THREE SIZES, ITERATED RATHER THAN LISTED, so this cannot fall out of
    // step with the enum the day a fourth is added.
    example!(
        r"Input { size } for size in InputSize::ALL",
        Key::ExampleInputNoteSize
    ),
    // REFUSED, LOCKED AND UNAVAILABLE — three different states, each on a box
    // the state makes sense on. The error sits on the address that was refused,
    // not on a reference number nobody can edit.
    example!(
        r"Input { name, label, kind: Email, value, error, autocomplete }
Input { name, label, value, readonly }
Input { name, label, kind: Password, reveal_label, conceal_label, disabled, glass }",
        Key::ExampleInputNoteRefused
    ),
];

const TEXTAREA: &[Example] = &[
    example!(
        r"Textarea { name, label, rows, autosize, max_rows, placeholder, hint, required, required_label }",
        Key::ExampleTextareaNote
    ),
    example!(
        r"Textarea { name, label, rows, maxlength, counter, counter_label, spellcheck: false }"
    ),
    example!(r"Textarea { name, label, value, rows, resize: None, autosize, size, glass, error }"),
    example!(r"Textarea { readonly } / Textarea { disabled }"),
];

const NUMBER_INPUT: &[Example] = &[
    // ONE QUESTION, IN A FORM THAT SUBMITS. Press Save and the quantity
    // arrives in the address bar, which is the only way to show that this is a
    // real `<input>` with a real `name` rather than a picture of one.
    example!(
        r"NumberInput { name, label, value, min, max, step, big_step, increment_label, decrement_label, hint, required, required_label }",
        Key::ExampleNumberInputNoteForm
    ),
    // STEPPING IS THE ANSWER RATHER THAN A CORRECTION. `primary` is the shape
    // a quantity takes on a product page: one full control either side of the
    // value, each big enough to hit with a thumb.
    example!(
        r#"NumberInput { stepping: "primary", label, value, min, max, increment_label, decrement_label }"#,
        Key::ExampleNumberInputNoteStepping
    ),
    // REFUSED AND EXPLAINED, NEVER REWRITTEN. Type 40, leave the box, and the
    // sentence under the question is the caller's own. The number stays as it
    // was typed, so a reader can see what was wrong with it.
    example!(
        r"NumberInput { name, label, value, min, max, min_label, max_label, invalid_label, increment_label, decrement_label }",
        Key::ExampleNumberInputNoteBounds
    ),
    // MONEY, AND WHAT `group` PROMISES. The separators are the page's own and
    // they are drawn while nobody is typing. This one submits as well, because
    // "never submitted" is a claim only a form can settle.
    example!(
        r"NumberInput { name, label, value, min, max, step, prefix, group, placeholder, size, increment_label, decrement_label }",
        Key::ExampleNumberInputNoteMoney
    ),
    // A MEASUREMENT, WITH ITS UNIT INSIDE THE BOX. `step` carries the decimal
    // here: it sets what one press moves, how many decimals are drawn, and
    // which keyboard a phone opens.
    example!(
        r"NumberInput { name, label, value, min, max, step, suffix, increment_label, decrement_label }",
        Key::ExampleNumberInputNoteUnit
    ),
    // TOO LARGE TO STEP TO, which is the one reason to turn the two controls
    // off. The arrow keys and Page Up still move it, and `big_step` is what
    // makes crossing this range from the keyboard possible at all.
    example!(
        r"NumberInput { name, label, value, step, big_step, steppers, group }",
        Key::ExampleNumberInputNoteLarge
    ),
    // REFUSED, LOCKED AND UNAVAILABLE — three states, each on a question the
    // state makes sense on. The error sits on the number that was refused, and
    // the locked value carries none, because there is nothing to correct.
    example!(
        r"NumberInput { name, label, value, max, error, increment_label, decrement_label }
NumberInput { name, label, value, readonly, size, increment_label, decrement_label }
NumberInput { name, label, disabled, glass, increment_label, decrement_label }",
        Key::ExampleNumberInputNoteStates
    ),
];

const SLIDER: &[Example] = &[
    example!(
        r"Slider { name, label, value, min, max, step, output_label, hint, required, required_label }",
        Key::ExampleSliderNote
    ),
    example!(r"Slider { marks, mark_labels }"),
    example!(
        r"Slider { name, label, range, value, end, min, max, step, output_label, tone, size }"
    ),
    example!(
        r"Slider { name, label, value, output: false, tone, size, glass, error }
Slider { name, label, value, tone, disabled }"
    ),
];

const FILE_INPUT: &[Example] = &[
    example!(
        r"FileInput { name, label, browse_label, drop_label, empty_label, status, hint, required, required_label }",
        Key::ExampleFileInputNote
    ),
    example!(r"FileInput { multiple, max, max_label, remove_label, added_label, removed_label }"),
    example!(
        r"FileInput { accept, accept_label, max_size, max_size_label } — help, never security"
    ),
    example!(
        r"FileInput { name, label, browse_label, drop: false, size, glass, error }
FileInput { name, label, browse_label, disabled, size }"
    ),
];

const TIME_INPUT: &[Example] = &[
    example!(
        r"TimeInput { name, legend, hour_label, minute_label, value, hint, size }",
        Key::ExampleTimeInputNote
    ),
    example!(
        r"TimeInput { name, legend, hour_label, minute_label, clock: _12, meridiem_label, am_label, pm_label, value }"
    ),
    example!(
        r"TimeInput { min, max, step, pad: false, required, required_label, missing_label, invalid_label, min_label, max_label }"
    ),
    example!(r"TimeInput { seconds, second_label, glass, error } / TimeInput { disabled }"),
];
const CAROUSEL: &[Example] = &[
    example!(
        r"Carousel { name, label, snap: Center, previous_label, next_label, announce_label }",
        Key::ExampleCarouselGalleryNote
    ),
    example!(
        r#"Carousel { per_view: "auto" } — the shelf, which is the shape that survives a telephone"#
    ),
    example!(
        r"Carousel { dots, dot_label, counter, counter_label, glass }",
        Key::ExampleCarouselDotsNote
    ),
    example!(
        r"Carousel { autoplay, pause_label, play_label } — and the four rules it carries",
        Key::ExampleCarouselRotatingNote
    ),
    example!(
        r"Carousel { per_view, wrap, snap: None, counter, counter_label }
Carousel { orientation: Vertical, controls: false }",
        Key::ExampleCarouselWrapNote
    ),
];

const CAROUSEL_SLIDE: &[Example] = &[
    example!(r"CarouselSlide { label } — added to the position, never instead of it"),
    example!(r"CarouselSlide { children } — anything focusable in one stays in the tab order"),
    example!(r"CarouselSlide — with no label, the position alone is the name"),
    example!(r"CarouselSlide — in a vertical carousel, the same markup"),
];

// --- ErrorSummary -----------------------------------------------------------

// THE ONE PAGE ON THIS SITE WHOSE FORMS EXIST TO BE REFUSED. A summary with
// nothing in it is a box a reader learns to skip, so every example here is
// shown in the state the component is for — and the `collect` one is genuinely
// interactive: press its control with the boxes empty and the browser refuses,
// which is what fills it.

const ERROR_SUMMARY: &[Example] = &[
    example!(
        r"ErrorSummary { name, title, description, focus } with ErrorSummaryItem children",
        Key::ExampleSummaryRenderedNote
    ),
    example!(
        r"ErrorSummary { collect: true, form, title_prefix } — the browser's own refusal",
        Key::ExampleSummaryCollectNote
    ),
    example!(
        r"An entry reaching a control the script has HIDDEN — focus lands on the visible one",
        Key::ExampleSummaryLinkNote
    ),
    example!(
        r"ErrorSummary { focus: false } and no description",
        Key::ExampleSummaryPlainNote
    ),
];

const ERROR_SUMMARY_ITEM: &[Example] = &[
    example!(r"ErrorSummaryItem { control } — the id of the control, and it must exist"),
    example!(r"Several, in the order the form asks the questions"),
    example!(
        r"ErrorSummaryItem { control } for a question asked as several boxes",
        Key::ExampleSummaryItemGroupNote
    ),
    example!(
        r"ErrorSummaryItem { children } — the sentence, never the name of the field",
        Key::ExampleSummaryItemWordsNote
    ),
];

// --- Tag --------------------------------------------------------------------

// THE ROW IS THE POINT. A tag on its own says almost nothing about the
// component; tags come in rows, and every question worth asking about one — is
// the colour too loud at eleven, does the ellipsis land in the right place, do
// they wrap without tearing — is a question about the row.

const TAG: &[Example] = &[
    example!(
        r#"Tag { label: "…", tone: "…", variant: TagVariant::… }"#,
        Key::ExampleTagTones
    ),
    example!(
        r#"Tag { icon: IconDrawing::CHECK, swatch: "…", image: "…" }"#,
        Key::ExampleTagBeside
    ),
    example!(
        r#"Tag { href: "…" } / Tag { pressed: TagPressed::On } / Tag { disabled: true }"#,
        Key::ExampleTagKinds
    ),
    example!(
        r#"Tag { remove_label: "…", remove_name: "…", value: "…" } inside a <form>"#,
        Key::ExampleTagRemovable
    ),
    example!(
        r"Tag { size: TagSize::… } / Tag { shape: TagShape::… }",
        Key::ExampleTagSizes
    ),
    example!(r"Tag { glass: true }", Key::ExampleGlassBehind),
];

// --- Badge ------------------------------------------------------------------

const BADGE: &[Example] = &[
    example!(
        r#"Badge { count: "…", max: "99", label: "…" }"#,
        Key::ExampleBadgeCount
    ),
    example!(
        r#"Badge { dot: true } / Badge { count: "0" } / Badge { count: "0", show_zero: true }"#,
        Key::ExampleBadgeDot
    ),
    example!(r#"Badge { tone: "…" }"#, Key::ExampleBadgeTones),
    example!(
        r"Badge { placement: BadgePlacement::… } / Badge { overlap: BadgeOverlap::… }",
        Key::ExampleBadgePlacement
    ),
];
const AVATAR: &[Example] = &[
    example!(
        r#"Avatar { label: "…", src: "…" } / { initials: "…" } / { icon: IconDrawing::MONITOR }"#,
        Key::ExampleAvatarLayers
    ),
    example!(
        r"Avatar { size: AvatarSize::… } / Avatar { shape: AvatarShape::… }",
        Key::ExampleAvatarBox
    ),
    example!(r"Avatar { tone: AvatarTone::… }", Key::ExampleAvatarTone),
    example!(
        r#"Avatar { href: "…" } / Avatar { decorative: true }"#,
        Key::ExampleAvatarKinds
    ),
];

const AVATAR_GROUP: &[Example] = &[
    example!(
        r#"AvatarGroup { label: "…", more: "+5", more_label: "…", more_href: "…" }"#,
        Key::ExampleAvatarGroupCounter
    ),
    example!(
        r"AvatarGroup { overlap: AvatarGroupOverlap::… }",
        Key::ExampleAvatarGroupOverlap
    ),
    example!(
        r"AvatarGroup { size: AvatarGroupSize::…, shape: AvatarGroupShape::… }",
        Key::ExampleAvatarGroupBox
    ),
    example!(r"AvatarGroup { children }", Key::ExampleAvatarGroupPlain),
];
const TAG_PICKER: &[Example] = &[
    example!(
        r#"TagPicker { name: "…", label: "…", hint: "…", placeholder: "…", status, create_label, empty_label, remove_label, added_label, removed_label }"#,
        Key::ExamplePickerFree
    ),
    example!(
        r#"TagPicker { creatable: false, separators: ",;", max: "3", max_label: "…" }"#,
        Key::ExamplePickerBounded
    ),
    example!(
        r#"TagPicker { pattern: "…", invalid_label: "…", clearable: true, clear_label: "…" }
TagPicker { paste: false, duplicates: true, … }"#,
        Key::ExamplePickerRefusing
    ),
    example!(
        r#"TagPicker { required: true, error: "…" } inside a <form>"#,
        Key::ExamplePickerForm
    ),
    example!(
        r"TagPicker { size: TagPickerSize::… } / { loading: true, loading_label } / { disabled: true } / { glass: true }",
        Key::ExamplePickerStates
    ),
];

// --- Text -------------------------------------------------------------------

const TEXT: &[Example] = &[
    example!(r"Every size"),
    example!(r"Every weight"),
    example!(r"Every tone"),
    example!(r"Text { numeric: true, … }", Key::ExampleTextNumeric),
    example!(
        r"Text { size: Caption, weight: Semibold, tone: Muted }",
        Key::ExampleTextCombined
    ),
];

// --- Time -------------------------------------------------------------------

const TIME: &[Example] = &[
    example!(
        r#"Time { datetime: "2026-07-29", "…" }"#,
        Key::ExampleTimeMachine
    ),
    example!(r#"Time { datetime: "2026-07-29T14:30", … }"#),
    example!(r"In a sentence"),
    example!(r"In a list of dates", Key::ExampleTimeAligned),
];
const TREE: &[Example] = &[
    example!(
        r#"Tree { name: "…", label: "…", expand_label: "…", collapse_label: "…", empty_label: "…" }"#,
        Key::ExampleTreeWalk
    ),
    example!(
        r"Tree { guides: true, size: TreeSize::… }",
        Key::ExampleTreeGuides
    ),
    example!(
        r#"Tree { selection: TreeSelection::Multiple, form: "…" } inside a <form>"#,
        Key::ExampleTreeSelection
    ),
    example!(
        r"Tree { selection: TreeSelection::Single }",
        Key::ExampleTreeSingle
    ),
];

const TREE_ITEM: &[Example] = &[
    example!(
        r#"TreeItem { label: "…", href: "…", icon, icon_expanded, badge: "…" }"#,
        Key::ExampleTreeItemRow
    ),
    example!(
        r"TreeItem { expanded: true } / TreeItem { branch: true }",
        Key::ExampleTreeItemBranch
    ),
    example!(
        r"TreeItem { current: true } / TreeItem { disabled: true } / TreeItem { selected: true }",
        Key::ExampleTreeItemStates
    ),
    example!(r#"TreeItem { value: "…" }"#, Key::ExampleTreeItemValue),
];

// --- Truncate ---------------------------------------------------------------

const TRUNCATE: &[Example] = &[
    example!(r#"Truncate { title: "…", "…" }"#, Key::ExampleTruncateWhole),
    example!(r"Truncate { lines: TruncateLines::_2, … }"),
    example!(r"Truncate { lines: TruncateLines::_4, … }"),
    example!(r"Short enough not to clip", Key::ExampleTruncateShort),
];

// --- VisuallyHidden ---------------------------------------------------------

const VISUALLY_HIDDEN: &[Example] = &[
    example!(r#"VisuallyHidden { "…" }"#, Key::ExampleHiddenNothing),
    // Hand-written markup, deliberately, in the one example that is ABOUT
    // hand-written markup: this is the technique, and `Button { hide_label }`
    // is the technique already done for you. The note says so.
    example!(r"Naming an icon-only control", Key::ExampleHiddenNaming),
    example!(
        r"VisuallyHidden { focusable: true, … }",
        Key::ExampleHiddenFocusable
    ),
    example!(r"Extending a link's name", Key::ExampleHiddenContext),
];

#[cfg(test)]
mod tests {
    use super::*;
    use crate::generated::catalogue::COMPONENTS;

    #[test]
    fn every_component_has_examples() {
        for component in COMPONENTS {
            let examples = for_component(component.name);
            assert!(
                examples.len() >= 4,
                "{} has {} example(s); a component page needs several",
                component.name,
                examples.len()
            );
        }
    }

    /// AN OVERVIEW IS KEYED BY SLUG, AND A SLUG CAN STOP BEING A PAGE.
    ///
    /// Merging a component onto the page of the component it is part of leaves
    /// the overview it led behind, keyed to a slug `overview_for_page` will
    /// never be handed again. Nothing reports it, because the lookup that
    /// misses simply returns `None` and the page opens with no example at all.
    /// Four were orphaned that way — `ListItem`, `SegmentedItem`, `SelectGroup`
    /// and `SelectOption` — and `trees::complete` guards their trees the same
    /// way.
    #[test]
    fn every_overview_leads_a_page_that_exists() {
        for (slug, _) in OVERVIEWS {
            assert!(
                crate::site::by_slug(slug).is_some(),
                "an overview leads `{slug}`, which is not a page"
            );
        }
    }

    /// An example with no source is an example a reader cannot copy, and the
    /// source is also the caption — so an empty one is a blank caption too.
    #[test]
    fn every_example_carries_its_source() {
        for component in COMPONENTS {
            for example in for_component(component.name) {
                assert!(
                    !example.source.trim().is_empty(),
                    "{} has an example with no source",
                    component.name
                );
            }
        }
    }

    /// that has content demonstrates it.
    #[test]
    fn the_examples_reach_every_prop() {
        for component in COMPONENTS {
            let sources: String = for_component(component.name)
                .iter()
                .map(|example| example.source)
                .collect::<Vec<_>>()
                .join("\n");
            for prop in component.props {
                if prop.kind == "children" {
                    continue;
                }
                assert!(
                    sources.contains(prop.name),
                    "{}.{} appears in no example",
                    component.name,
                    prop.name
                );
            }
        }
    }
}
