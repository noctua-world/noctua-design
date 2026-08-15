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
//! These fill that gap, and they render the **real component** rather than a
//! string of markup. A page built from markup can show something the component
//! does not do; a page built from the component cannot.
//!
//! # The caption is the code
//!
//! Each example carries the Dioxus call that produced it, and that call is also
//! its caption. `Blockquote { tone: Accent, … }` says what an example shows
//! more precisely than a sentence would, in every language at once, and it is
//! the thing a reader came to copy. A `note` is for the few cases where the
//! call does not speak for itself, and a note is a message like every other
//! sentence on this site.

use dioxus::prelude::*;
use noctua_design::IconName;
use noctua_design::components::{
    Abbr, Avatar, AvatarGroup, AvatarGroupOverlap, AvatarGroupShape, AvatarGroupSize, AvatarShape,
    AvatarSize, AvatarTone, Badge, BadgeOverlap, BadgePlacement, Blockquote, BlockquoteTone,
    Button, ButtonGroup, ButtonGroupOrientation, ButtonJustify, ButtonShape, ButtonSize,
    ButtonSubmit, ButtonTone, ButtonVariant, Carousel, CarouselOrientation, CarouselSlide,
    CarouselSnap, Checkbox, CheckboxSize, Code, CodeBlock, CodeBlockTheme, CodeTone, ColorPicker,
    ColorPickerShape, ColorPickerSize, DateInput, DateInputOrder, DateInputSize, Del, Dialog,
    DialogSize, Disclosure, DisclosureHeading, Divider, DividerOrientation, DividerSpacing,
    DividerWeight, Em, ErrorSummary, ErrorSummaryItem, Fieldset, FileInput, FileInputSize, Heading,
    HeadingLevel, HeadingSize, HeadingTone, Icon, IconSize, Image, ImageFit, ImageLoading,
    ImageRatio, Info, InfoPlacement, Input, InputKind, InputSize, Ins, Kbd, KbdSize, Label,
    LabelKind, LabelSize, Link, LinkTone, List, ListItem, ListKind, ListMarker, ListSpacing, Mark,
    Menu, MenuItem, MenuPlacement, MessageBar, MessageBarLive, Meter, MeterSize, NumberInput,
    NumberInputSize, OtpField, OtpFieldKind, OtpFieldSize, Pagination, PaginationSize,
    PaginationSpread, Paragraph, ParagraphSize, ParagraphTone, Popover, PopoverPlacement, Progress,
    ProgressSize, Prose, ProseMeasure, Radio, RadioSize, Rating, RatingSize, Search, SearchGroup,
    SearchPresentation, SearchResult, SearchSize, Segmented, SegmentedAppearance, SegmentedFill,
    SegmentedItem, SegmentedOrientation, SegmentedSelection, SegmentedShape, SegmentedSize, Select,
    SelectGroup, SelectOption, SelectSize, Slider, SliderSize, SliderTone, Strong, Swatch,
    SwatchShape, SwatchSize, Switch, SwitchSize, Tab, TabPanel, Table, TableCell,
    TableCellAggregate, TableCellAggregateOver, TableCellAlign, TableCellWrap, TableColumn,
    TableColumnSortAs, TableColumnWidth, TableDensity, TableHeaderCell, TableHeaderCellAlign,
    TableHeaderCellScope, TableHeaderCellSort, TableHeaderCellWrap, TableHeight, TableLayout,
    TableRow, TableRowEmphasis, TableRowSelect, TableRules, TableSection, TableSectionKind,
    TableWidth, Tabs, TabsActivation, TabsAppearance, TabsFill, TabsOrientation, TabsSize, Tag,
    TagPicker, TagPickerSize, TagPressed, TagShape, TagSize, TagVariant, Text, TextSize, TextTone,
    TextWeight, Textarea, TextareaResize, TextareaSize, Time, TimeInput, TimeInputClock,
    TimeInputSize, Toast, ToastLive, ToastPlacement, Toolbar, ToolbarDensity, ToolbarGroup,
    ToolbarGroupAlign, ToolbarOrientation, Tooltip, TooltipDelay, TooltipPlacement, Tree, TreeItem,
    TreeSelection, TreeSize, Truncate, TruncateLines, VisuallyHidden,
};
use noctua_design::components::{
    Lineage, LineageFigure, LineageFigureParallax, LineageFigureShape, LineageFigureTone,
    LineageFilterMode, LineageHeight, LineageLettering, LineageLine, LineagePick, LineageReach,
    LineageRoute, LineageSize, LineageThickness, LineageTilt, LineageView,
};
use noctua_design_grammars::Language;
use noctua_design_i18n::Key;

use crate::shell::Context;

/// One worked example.
#[derive(Clone, Copy)]
pub(crate) struct Example {
    /// The Dioxus call that produced it. Shown above the example, and its
    /// caption: an identifier rather than prose, and the same in every locale.
    pub source: &'static str,
    /// A sentence, for the few examples the call does not explain by itself.
    pub note: Option<Key>,
    /// What to render. A function pointer, so the table stays a `const` and
    /// every example is the component rather than a picture of it.
    pub render: fn(Context) -> Element,
}

/// Declares one example without repeating the field names.
macro_rules! example {
    ($source:literal, $render:expr) => {
        Example {
            source: $source,
            note: None,
            render: $render,
        }
    };
    ($source:literal, $note:expr, $render:expr) => {
        Example {
            source: $source,
            note: Some($note),
            render: $render,
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

/// The graphs the scenes below are drawn from.
///
/// Written out rather than generated: a reader who wants to know what the
/// language looks like is looking at exactly what was passed in. Where a
/// connection carries a *word* rather than a name or a date, the graph is built
/// with `format!` around `context.t` — a bond's label is content a reader sees,
/// and content a reader sees is a message like every other one on this site.
///
/// The portraits are placeholders and nobody's likeness; the attribution beside
/// them says so. They are here because a scene of people reads differently from
/// a scene of initials, and this component is for scenes of people.
mod graphs {
    /// Where a portrait lives, from a page two levels below the site root.
    macro_rules! face {
        ($name:literal) => {
            concat!("image=../../assets/portraits/", $name, ".svg")
        };
    }

    /// Three generations, a union that carries children, and a second marriage.
    pub(super) const DOE: &str = concat!(
        "figure gran   label=\"Gran Doe\"   meta=\"1918–2004\" terms=\"doe\" ",
        face!("gran"),
        "\n",
        "figure gramps label=\"Gramps Doe\" meta=\"1915–1998\" terms=\"doe\" ",
        face!("gramps"),
        "\n",
        "mark   u0 kind=union\n",
        "bond   gran u0 kind=union\n",
        "bond   gramps u0 kind=union\n",
        "figure ada label=\"Ada Doe\" meta=\"1946\" terms=\"doe\" ",
        face!("ada"),
        "\n",
        "figure bob label=\"Bob Doe\" meta=\"1949\" terms=\"doe\" ",
        face!("bob"),
        "\n",
        "bond   u0 ada\n",
        "bond   u0 bob\n",
        "figure eve label=\"Eve Marsh\" meta=\"1948\" ",
        face!("eve"),
        "\n",
        "mark   u1 kind=union\n",
        "bond   ada u1 kind=union\n",
        "bond   eve u1 kind=union\n",
        "figure cara label=\"Cara Doe\" meta=\"1972\" terms=\"doe\" ",
        face!("cara"),
        "\n",
        "bond   u1 cara\n",
    );

    /// The adopted child, whose word and whose sentence are both messages —
    /// which is why this is a format string rather than a constant.
    pub(super) const ADOPTED: &str = concat!(
        "figure dan label=\"Dan Doe\" meta=\"1975\" terms=\"doe\" ",
        face!("dan"),
        "\n",
        "bond   u1 dan kind=adoption label=\"{word}\" note=\"{note}\"\n",
    );

    /// One person, two unions, children under each.
    pub(super) const PARTNERS: &str = concat!(
        "figure ada label=\"Ada Doe\" meta=\"1946\" ",
        face!("ada"),
        "\n",
        "figure bob label=\"Bob Marsh\" meta=\"1943\" ",
        face!("bob"),
        "\n",
        "figure eve label=\"Eve Okonkwo\" meta=\"1951\" ",
        face!("eve"),
        "\n",
        "mark   u0 kind=union\n",
        "mark   u1 kind=union\n",
        "bond   ada u0 kind=union\n",
        "bond   bob u0 kind=union\n",
        "bond   ada u1 kind=union\n",
        "bond   eve u1 kind=union\n",
        "figure cara label=\"Cara\" meta=\"1969\" ",
        face!("cara"),
        "\n",
        "figure dan  label=\"Dan\"  meta=\"1971\" ",
        face!("dan"),
        "\n",
        "figure fay  label=\"Fay\"  meta=\"1980\" ",
        face!("gran"),
        "\n",
        "bond   u0 cara\n",
        "bond   u0 dan\n",
        "bond   u1 fay\n",
    );

    /// The Doe family and the Marsh family, joined by one marriage.
    ///
    /// It exists for the filter: with `Doe` typed in, the whole Marsh side goes
    /// — four people, not one — and that is what a cut marker is for. On the
    /// plain family graph the filter removed a single figure, which is the one
    /// case where a gap needs no explaining.
    pub(super) const FILTERED: &str = concat!(
        "figure gran   label=\"Gran Doe\"   meta=\"1918–2004\" terms=\"doe\" ",
        face!("gran"),
        "\n",
        "figure gramps label=\"Gramps Doe\" meta=\"1915–1998\" terms=\"doe\" ",
        face!("gramps"),
        "\n",
        "mark   u0 kind=union\n",
        "bond   gran u0 kind=union\n",
        "bond   gramps u0 kind=union\n",
        "figure ada label=\"Ada Doe\" meta=\"1946\" terms=\"doe\" ",
        face!("ada"),
        "\n",
        "figure bob label=\"Bob Doe\" meta=\"1949\" terms=\"doe\" ",
        face!("bob"),
        "\n",
        "bond   u0 ada\n",
        "bond   u0 bob\n",
        "figure mgran label=\"Rui Marsh\"  meta=\"1920–2001\" ",
        face!("dan"),
        "\n",
        "figure mgram label=\"Fay Marsh\"  meta=\"1922–2010\" ",
        face!("gran"),
        "\n",
        "mark   um kind=union\n",
        "bond   mgran um kind=union\n",
        "bond   mgram um kind=union\n",
        "figure eve label=\"Eve Marsh\" meta=\"1948\" ",
        face!("eve"),
        "\n",
        "figure syb label=\"Syb Marsh\" meta=\"1951\" ",
        face!("cara"),
        "\n",
        "bond   um eve\n",
        "bond   um syb\n",
        "mark   u1 kind=union\n",
        "bond   ada u1 kind=union\n",
        "bond   eve u1 kind=union\n",
        "figure cara label=\"Cara Doe\" meta=\"1972\" terms=\"doe\" ",
        face!("cara"),
        "\n",
        "bond   u1 cara\n",
    );

    /// A branch on every side of the trunk, which is what the deep view is for.
    ///
    /// The scene it replaced was three children of one couple, and three
    /// children make one plane and two behind it — a demonstration of depth
    /// with almost no depth in it. This has four branches, two generations
    /// under two of them, and connections that leave the trunk in both
    /// directions.
    /// A branch on every side of the trunk, which is what the deep view is
    /// for. Each union here shows the one recorded parent — the mark itself
    /// says the other is unrecorded — because this example's whole subject is
    /// branches-as-planes, and a two-cell "parent unknown" card on every
    /// plane buried the depth it exists to show.
    pub(super) const DEEP: &str = concat!(
        "figure bob label=\"Bob Marsh\" meta=\"1943\" ",
        face!("bob"),
        "\n",
        "figure ada label=\"Ada Doe\" meta=\"1946\" ",
        face!("ada"),
        "\n",
        "mark   u0 kind=union\n",
        "bond   ada u0 kind=union\n",
        "bond   bob u0 kind=union\n",
        "figure cara label=\"Cara\" meta=\"1969\" ",
        face!("cara"),
        "\n",
        "figure dan  label=\"Dan\"  meta=\"1971\" ",
        face!("dan"),
        "\n",
        "figure fay  label=\"Fay\"  meta=\"1974\" ",
        face!("gran"),
        "\n",
        "bond   u0 cara\n",
        "bond   u0 dan\n",
        "bond   u0 fay\n",
        "figure kit label=\"Kit\" meta=\"1995\" ",
        face!("eve"),
        "\n",
        "figure lev label=\"Lev\" meta=\"1998\" ",
        face!("dan"),
        "\n",
        "figure mia label=\"Mia\" meta=\"2001\" ",
        face!("ada"),
        "\n",
        "mark   u1 kind=union\n",
        "bond   cara u1 kind=union\n",
        "bond   u1 kit\n",
        "bond   u1 lev\n",
        "mark   u2 kind=union\n",
        "bond   fay u2 kind=union\n",
        "bond   u2 mia\n",
    );

    pub(super) const SEASON: &str = concat!(
        "figure ada label=\"Ada Doe\" meta=\"1946\" slot\n",
        "figure eve label=\"Eve Marsh\" meta=\"1948\" slot\n",
        "mark   u0 kind=union\n",
        "bond   ada u0 kind=union\n",
        "bond   eve u0 kind=union\n",
        "figure cara label=\"Cara Doe\" meta=\"1972\" slot\n",
        "bond   u0 cara\n",
    );

    /// A grandparent married to a grandchild: the union spans two rows, and
    /// nothing about the descent moves to make it fit.
    ///
    /// EVERY CHILD HERE HAS TWO PARENTS, and two of the four are nobody. A
    /// person has two whether or not a family knows who they were, and a scene
    /// that hangs a child off one of them is drawing a claim rather than a
    /// gap — which is what a reader said about every example on this page. An
    /// unknown parent is a figure like any other, with an em dash for a name.
    pub(super) const SPANNING: &str = concat!(
        "figure gran label=\"Gran Doe\" meta=\"1918\" ",
        face!("gran"),
        "\n",
        "figure nan  kind=unknown span=1.5 label=\"{nobody}\"\n",
        "mark   u1 kind=union\n",
        "bond   gran u1 kind=union\n",
        "bond   nan u1 kind=union\n",
        "figure ada  label=\"Ada Doe\"  meta=\"1946\" ",
        face!("ada"),
        "\n",
        "bond   u1 ada\n",
        "figure pat  kind=unknown span=1.5 label=\"{nobody}\"\n",
        "mark   u2 kind=union\n",
        "bond   ada u2 kind=union\n",
        "bond   pat u2 kind=union\n",
        "figure cara label=\"Cara Doe\" meta=\"1972\" ",
        face!("cara"),
        "\n",
        "bond   u2 cara\n",
        "mark   u0 kind=union\n",
        "bond   gran u0 kind=union\n",
        "bond   cara u0 kind=union\n",
        "figure kid label=\"Dan Doe\" meta=\"1996\" ",
        face!("dan"),
        "\n",
        "bond   u0 kid\n",
    );

    /// A child with two biological parents nobody knows and two who raised
    /// them. Both couples are drawn, because both are true at once — which is
    /// the case a family tree with one parent per line cannot state at all.
    pub(super) const ADOPTION: &str = concat!(
        "figure bio1 kind=unknown span=1.5 label=\"{nobody}\"\n",
        "figure bio2 kind=unknown span=1.5 label=\"{nobody}\"\n",
        "mark   ub kind=union\n",
        "bond   bio1 ub kind=union\n",
        "bond   bio2 ub kind=union\n",
        "figure ada label=\"Ada Doe\"  meta=\"1946\" ",
        face!("ada"),
        "\n",
        "figure eve label=\"Eve Marsh\" meta=\"1948\" ",
        face!("eve"),
        "\n",
        "mark   ua kind=union\n",
        "bond   ada ua kind=union\n",
        "bond   eve ua kind=union\n",
        "figure kid label=\"Rui Doe\" meta=\"1998\" ",
        face!("dan"),
        "\n",
        "bond   ub kid\n",
        "bond   ua kid kind=adoption label=\"{word}\"\n",
    );

    /// A donor, the parent who carried the child, and the partner who is not a
    /// parent — three relationships that a tree of couples cannot tell apart
    /// and that this one draws in three different lines.
    pub(super) const DONOR: &str = concat!(
        "figure ada label=\"Ada Doe\" meta=\"1946\" ",
        face!("ada"),
        "\n",
        "figure donor kind=unknown span=1.5 label=\"{nobody}\"\n",
        "mark   u0 kind=union\n",
        "bond   ada u0 kind=union\n",
        "figure cara label=\"Cara Doe\" meta=\"1972\" ",
        face!("cara"),
        "\n",
        "bond   u0 cara\n",
        "figure eve label=\"Eve Marsh\" meta=\"1948\" ",
        face!("eve"),
        "\n",
        "bond   donor u0 kind=donor label=\"{donor}\"\n",
        "bond   ada eve kind=partner label=\"{partner}\"\n",
    );

    /// A company under an origin that stays at the top.
    pub(super) const COMPANY: &str = "\
figure aviary label=\"Aviary Ltd\" meta=\"Since 1948\" origin shape=plaque span=2 tone=accent\n\
figure chair label=\"Ada Doe\" meta=\"Chair\" kind=role\n\
figure ops   label=\"Bob Marsh\" meta=\"Operations\" kind=role\n\
figure build label=\"Eve Okonkwo\" meta=\"Building\" kind=role\n\
figure a1 label=\"Night shift\" kind=team\n\
figure a2 label=\"Day shift\" kind=team\n\
figure b1 label=\"Workshop\" kind=team\n\
bond   ops a1 kind=report\n\
bond   ops a2 kind=report\n\
bond   build b1 kind=report\n";

    /// Families with nothing between them.
    ///
    /// THREE, NOT FOUR, and each one a couple. Four families of one parent and
    /// one child fitted the window and said something false about all four;
    /// three families of three say the true thing in the same width.
    pub(super) const FOUR: &str = "\
figure a1 label=\"Ada Doe\" meta=\"1946\"\nfigure a2 label=\"Bob Doe\" meta=\"1944\"\n\
mark ua kind=union\nbond a1 ua kind=union\nbond a2 ua kind=union\n\
figure a3 label=\"Cara Doe\" meta=\"1972\"\nbond ua a3\n\
figure b1 label=\"Bob Marsh\" meta=\"1943\"\nfigure b2 label=\"Eve Marsh\" meta=\"1945\"\n\
mark ub kind=union\nbond b1 ub kind=union\nbond b2 ub kind=union\n\
figure b3 label=\"Fay Marsh\" meta=\"1969\"\nbond ub b3\n\
figure c1 label=\"Eve Okonkwo\" meta=\"1951\"\nfigure c2 label=\"Dan Okonkwo\" meta=\"1950\"\n\
mark uc kind=union\nbond c1 uc kind=union\nbond c2 uc kind=union\n\
figure c3 label=\"Ada Okonkwo\" meta=\"1980\"\nbond uc c3\n";

    /// A branch that can come from somewhere else, and the switch that brings
    /// it.
    ///
    /// THE SWITCH IS NOT IN THE TREE. It used to hang off Ada by a dotted line,
    /// which drew a server as a member of the family — it is a control, it
    /// belongs beside the scene, and `cluster` is how a graph says *this stands
    /// on its own*. Pressing it a second time puts the branch away again.
    pub(super) const SERVER: &str = concat!(
        "figure ada label=\"Ada Doe\" meta=\"1946\" ",
        face!("ada"),
        "\n",
        "figure bob label=\"Bob Doe\" meta=\"1944\" ",
        face!("bob"),
        "\n",
        "mark   u0 kind=union\n",
        "bond   ada u0 kind=union\n",
        "bond   bob u0 kind=union\n",
        "figure c1 label=\"Cara Doe\" meta=\"1972\" ",
        face!("cara"),
        "\n",
        "bond   u0 c1\n",
        "figure c2 label=\"Rui Doe\" meta=\"1975\" set=doe:remote ",
        face!("dan"),
        "\n",
        "figure c3 label=\"Fay Doe\" meta=\"1978\" set=doe:remote ",
        face!("gran"),
        "\n",
        "bond   u0 c2 set=doe:remote\n",
        "bond   u0 c3 set=doe:remote\n",
        "mark   srv kind=source slot span=2 cluster=server\n",
    );

    /// Two partners, their union and their child — the smallest scene that
    /// still has every kind of thing in it.
    ///
    /// It exists because the examples about SIZE and THICKNESS were drawn on a
    /// graph with a slot in it, and nothing filled the slot: each of them drew
    /// a dotted connection running into an empty box. A slot is a promise that
    /// a `LineageFigure` will arrive, and an example that does not keep it is
    /// an example of the component failing.
    pub(super) const PAIR: &str = concat!(
        "figure ada label=\"Ada Doe\" meta=\"1946\" ",
        face!("ada"),
        "\n",
        "figure eve label=\"Eve Marsh\" meta=\"1948\" ",
        face!("eve"),
        "\n",
        "mark   u0 kind=union\n",
        "bond   ada u0 kind=union\n",
        "bond   eve u0 kind=union\n",
        "figure cara label=\"Cara Doe\" meta=\"1972\" ",
        face!("cara"),
        "\n",
        "bond   u0 cara\n",
    );

    /// The owl and the founding couple the wild and queen scenes share — one
    /// text, so the two can never drift apart by accident.
    macro_rules! roost {
        () => {
            concat!(
                "figure noctua slot origin shape=bare span=2 reach=0.55 rank=0\n",
                "figure ada label=\"Ada Doe\" meta=\"1946\" shape=bare rank=2 ",
                face!("ada"),
                "\n",
                "figure eve label=\"Eve Marsh\" meta=\"1948\" shape=bare rank=2 ",
                face!("eve"),
                "\n",
                "mark   u0 kind=union\n",
                "bond   ada u0 kind=union\n",
                "bond   eve u0 kind=union\n",
            )
        };
    }

    /// The wild scene: nothing is in a box. Every figure is `shape=bare`,
    /// the origin is the photographed owl, and the example that uses it also
    /// sets `reach: Shy` and `lettering: Solid` — the three asks of one
    /// message.
    pub(super) const WILD: &str = concat!(
        roost!(),
        "figure cara label=\"Cara Doe\" meta=\"1972\" shape=bare ",
        face!("cara"),
        "\n",
        "figure dan label=\"Dan Doe\" meta=\"1975\" shape=bare ",
        face!("dan"),
        "\n",
        "bond   u0 cara\n",
        "bond   u0 dan\n",
    );

    /// The queen scene: the wild scene's roost with ONE child, so the doubled
    /// model is the only thing that changed between the two examples a
    /// reader compares — and the shorter family keeps a taller scene short.
    pub(super) const QUEEN: &str = concat!(
        roost!(),
        "figure cara label=\"Cara Doe\" meta=\"1972\" shape=bare ",
        face!("cara"),
        "\n",
        "bond   u0 cara\n",
    );

    /// And with the two founders left to fill, for the examples whose subject
    /// is what can go inside a figure. The origin draws itself here, because
    /// an example is about one thing at a time and every other slot in it has
    /// to be somebody's.
    pub(super) const SLOTS: &str = concat!(
        // THE ORIGIN IS A SLOT, AND IT IS WHERE A PLAQUE GOES. A plaque is two
        // cells wide, and the only row with two free cells on it is the one the
        // origin stands alone on — beside a union dot it reached across the
        // structure it was standing on.
        "figure noctua origin shape=plaque span=2 slot\n",
        "figure ada label=\"Ada Doe\" meta=\"1946\" slot\n",
        "figure eve label=\"Eve Marsh\" meta=\"1948\" slot\n",
        "mark   u0 kind=union\n",
        "bond   ada u0 kind=union\n",
        "bond   eve u0 kind=union\n",
        "figure cara label=\"Cara Doe\" meta=\"1972\" ",
        face!("cara"),
        "\n",
        "bond   u0 cara\n",
    );

    /// A mythic origin, and the family that says it came from her.
    pub(super) const OWL: &str = concat!(
        "figure noctua slot origin shape=round\n",
        "figure ada label=\"Ada Doe\" meta=\"1946\" slot\n",
        "figure eve label=\"Eve Marsh\" meta=\"1948\" slot\n",
        "mark   u0 kind=union\n",
        "bond   ada u0 kind=union\n",
        "bond   eve u0 kind=union\n",
        "figure cara label=\"Cara Doe\" meta=\"1972\" ",
        face!("cara"),
        "\n",
        "figure dan  label=\"Dan Doe\"  meta=\"1975\" ",
        face!("dan"),
        "\n",
        "bond   u0 cara\n",
        "bond   u0 dan\n",
    );

    /// Marks that are not unions: a milestone and a note, standing on the
    /// structure rather than in it.
    pub(super) const MARKED: &str = concat!(
        "figure ada label=\"Ada Doe\" meta=\"1946\" ",
        face!("ada"),
        "\n",
        "mark   note kind=milestone slot span=2\n",
        "bond   ada note kind=link\n",
        "figure eve label=\"Eve Marsh\" meta=\"1948\" ",
        face!("eve"),
        "\n",
        "mark   u0 kind=union\n",
        "bond   ada u0 kind=union\n",
        "bond   eve u0 kind=union\n",
        "figure cara label=\"Cara Doe\" meta=\"1972\" ",
        face!("cara"),
        "\n",
        "bond   u0 cara\n",
    );

    /// Ranks and widths said outright, for the cases inference cannot win.
    pub(super) const PLACED: &str = "\
figure title label=\"The Doe line\" rank=0 span=3 shape=plaque tone=accent cluster=doe\n\
figure a label=\"Ada Doe\"  meta=\"1946\" rank=1 cluster=doe\n\
figure b label=\"Bob Doe\"  meta=\"1949\" rank=1 cluster=doe\n\
figure c label=\"Cara Doe\" meta=\"1972\" rank=3 cluster=doe\n\
bond   a c\n\
bond   title a\n\
bond   title b\n";
}

/// The scenes, in the order the page tells them, which between them set
/// every prop the contract declares and show every feature it has.
const LINEAGE: &[Example] = &[
    example!(
        r#"Lineage { name: "doe", label: "…", graph: "figure gran … bond u0 ada" }"#,
        Key::ExampleLineageDoeNote,
        |context| rsx! {
            Lineage {
                name: "doe",
                label: context.t(Key::ExampleLineageDoe),
                graph: adopted(context),
            }
        }
    ),
    example!(
        r#"Lineage { graph: "wobble", empty_label: "…", height: Short }"#,
        Key::ExampleLineageEmptyNote,
        |context| rsx! {
            Lineage {
                name: "nothing",
                label: context.t(Key::ExampleLineageEmpty),
                graph: "wobble\n",
                empty_label: context.t(Key::ExampleLineageEmpty),
                height: LineageHeight::Short,
            }
        }
    ),
    example!(
        r"Lineage { parallax: true, LineageFigure { parallax: Deep } } — the origin",
        Key::ExampleLineageSlotsNote,
        |context| rsx! {
            Lineage {
                name: "founders",
                label: context.t(Key::ExampleLineageSlots),
                graph: graphs::OWL,
                LineageFigure {
                    scene: "founders",
                    at: "noctua",
                    shape: LineageFigureShape::Round,
                    tone: LineageFigureTone::Accent,
                    parallax: LineageFigureParallax::Deep,
                    // The sentence is the figure's accessible name rather than a
                    // third line: a round figure grows to whatever is in it, and
                    // three lines of it reached into the row below.
                    label: context.t(Key::ExampleLineageOwl),
                    Avatar {
                        label: "Noctua",
                        src: "../../assets/portraits/noctua.svg",
                        size: AvatarSize::Small,
                    }
                    Text { class: "nd-identifier", size: TextSize::Small, weight: TextWeight::Medium, "Noctua" }
                }
                LineageFigure {
                    scene: "founders",
                    at: "ada",
                    // SMALL, BECAUSE A FIGURE HAS ONE CELL. A portrait, a name
                    // and a line under it at the default size come to more than
                    // that, and the ceiling on a figure clips what is over it —
                    // which it has to, or the figure covers the connection that
                    // reaches it. See `.nd-lineage-figure` in `shell.css`.
                    Avatar { label: "Ada Doe", src: "../../assets/portraits/ada.svg",
                        size: AvatarSize::Small }
                    Text { class: "nd-identifier", size: TextSize::Small, weight: TextWeight::Medium, "Ada Doe" }
                    Text { size: TextSize::Caption, tone: TextTone::Muted,
                        "{context.t(Key::ExampleLineageFounder)}"
                    }
                }
                LineageFigure {
                    scene: "founders",
                    at: "eve",
                    Avatar { label: "Eve Marsh", src: "../../assets/portraits/eve.svg" }
                    Text { class: "nd-identifier", size: TextSize::Small, weight: TextWeight::Medium, "Eve Marsh" }
                }
            }
        }
    ),
    example!(
        r#""bond ua kid kind=adoption": four true parents  ·  "kind=donor" and "kind=partner""#,
        Key::ExampleLineageKindsNote,
        |context| rsx! {
            Lineage {
                name: "adoption",
                label: context.t(Key::ExampleLineageAdoption),
                graph: adoption(context),
            }
            Lineage {
                name: "donor",
                label: context.t(Key::ExampleLineageDonorScene),
                graph: donor(context),
            }
        }
    ),
    example!(
        r#""mark u1 kind=union" twice: one person, two unions  ·  "bond gran u0 kind=union" across generations"#,
        |context| rsx! {
            Lineage {
                name: "partners",
                label: context.t(Key::ExampleLineagePartners),
                graph: graphs::PARTNERS,
            }
            Lineage {
                name: "spanning",
                label: context.t(Key::ExampleLineageSpanning),
                graph: spanning(context),
            }
        }
    ),
    example!(
        r"Lineage { route: Elbow, height: Short }  ·  Lineage { route: Taut }",
        Key::ExampleLineageRouteNote,
        |context| rsx! {
            Lineage {
                name: "route-elbow",
                label: context.t(Key::ExampleLineageRouteElbow),
                graph: graphs::PARTNERS,
                height: LineageHeight::Short,
            }
            Lineage {
                name: "route-taut",
                label: context.t(Key::ExampleLineageRouteTaut),
                graph: graphs::PARTNERS,
                route: LineageRoute::Taut,
                height: LineageHeight::Short,
            }
        }
    ),
    example!(
        r"Lineage { size: Small }  ·  Lineage { size: Large, tilt: Resting }",
        |context| rsx! {
            Lineage {
                name: "size-small",
                label: context.t(Key::ExampleLineageSmall),
                graph: graphs::PAIR,
                size: LineageSize::Small,
                height: LineageHeight::Short,
            }
            Lineage {
                name: "size-large",
                label: context.t(Key::ExampleLineageLarge),
                graph: graphs::PAIR,
                size: LineageSize::Large,
                tilt: LineageTilt::Resting,
                height: LineageHeight::Short,
            }
        }
    ),
    example!(
        r"Lineage { line: Beam, tilt: Raked }  ·  { line: Ribbon }  ·  { thickness: None }",
        Key::ExampleLineageVolumeNote,
        |context| rsx! {
            Lineage {
                name: "beams",
                label: context.t(Key::ExampleLineageBeam),
                graph: graphs::PAIR,
                line: LineageLine::Beam,
                tilt: LineageTilt::Raked,
                height: LineageHeight::Short,
            }
            Lineage {
                name: "ribbons",
                label: context.t(Key::ExampleLineageRibbon),
                graph: graphs::PAIR,
                line: LineageLine::Ribbon,
                tilt: LineageTilt::Raked,
                height: LineageHeight::Short,
            }
            Lineage {
                name: "thin",
                label: context.t(Key::ExampleLineageFlat),
                graph: graphs::PAIR,
                thickness: LineageThickness::None,
                tilt: LineageTilt::Raked,
                height: LineageHeight::Short,
            }
        }
    ),
    example!(
        r"Lineage { view: Deep, tilt: Resting, height: Full }",
        Key::ExampleLineageDeepNote,
        |context| rsx! {
            Lineage {
                name: "deep",
                label: context.t(Key::ExampleLineageDeep),
                graph: graphs::DEEP,
                view: LineageView::Deep,
                // TURNED, BECAUSE THE CLAIM IS DEPTH. Straight on, four planes
                // one behind another are four rows of shrinking cards; a
                // resting pitch is the smallest camera that shows they are
                // planes at all, and it is the one place on this page where an
                // example opens at an angle on purpose.
                tilt: LineageTilt::Resting,
                height: LineageHeight::Full,
            }
        }
    ),
    example!(
        r#"Lineage { filter: "Doe", filter_mode: Isolate, matches_label, elided_label }"#,
        Key::ExampleLineageFilterNote,
        |context| rsx! {
            Lineage {
                name: "highlighted",
                label: context.t(Key::ExampleLineageFiltered),
                graph: graphs::FILTERED,
                filter: "Doe",
                matches_label: context.t(Key::ExampleLineageMatches),
                height: LineageHeight::Short,
            }
            Lineage {
                name: "isolated",
                label: context.t(Key::ExampleLineageFiltered),
                graph: graphs::FILTERED,
                filter: "Doe",
                filter_mode: LineageFilterMode::Isolate,
                matches_label: context.t(Key::ExampleLineageMatches),
                // WHAT THE GAP IS CALLED. `isolate` leaves a marker standing
                // where each removed group was; without this it is a numeral
                // with no name, and a screen reader reads a number on its own.
                elided_label: context.t(Key::ExampleLineageElided),
                height: LineageHeight::Short,
            }
        }
    ),
    example!(
        r#"Lineage { pick: Both, form: "ties" }  ·  { camera_label, zoom_in_label, zoom_out_label, fit_label }"#,
        Key::ExampleLineageActingNote,
        |context| rsx! {
            Lineage {
                name: "picked",
                label: context.t(Key::ExampleLineagePicked),
                graph: adopted(context),
                pick: LineagePick::Both,
                form: "ties",
            }
            Lineage {
                name: "camera",
                label: context.t(Key::ExampleLineageCamera),
                graph: graphs::DOE,
                camera_label: context.t(Key::ExampleLineageCamera),
                zoom_in_label: context.t(Key::ExampleLineageCloser),
                zoom_out_label: context.t(Key::ExampleLineageFurther),
                fit_label: context.t(Key::ExampleLineageFit),
            }
        }
    ),
    example!(
        r#"Lineage { occasion: "christmas" } — every figure has a face for the day"#,
        Key::ExampleLineageSeasonNote,
        |context| rsx! {
            Lineage {
                name: "season",
                label: context.t(Key::ExampleLineageSeason),
                graph: graphs::SEASON,
                occasion: "christmas",
                // THE WHOLE SCENE KEEPS THE DAY, and it is the only thing this
                // example shows. It used to share a scene with the origin and
                // with the parallax, so the one figure wearing a hat sat under
                // an owl and beside four people who were not — which is a
                // demonstration of nothing. Three people, six figures, one
                // occasion.
                LineageFigure { scene: "season", at: "ada",
                    Avatar { label: "Ada Doe", src: "../../assets/portraits/ada.svg",
                        size: AvatarSize::Small }
                    Text { class: "nd-identifier", size: TextSize::Small, weight: TextWeight::Medium, "Ada Doe" }
                }
                LineageFigure { scene: "season", at: "ada", occasion: "christmas",
                    Avatar { label: "Ada Doe", src: "../../assets/portraits/ada-christmas.svg",
                        size: AvatarSize::Small }
                    Text { class: "nd-identifier", size: TextSize::Small, weight: TextWeight::Medium, "Ada Doe" }
                }
                LineageFigure { scene: "season", at: "eve",
                    Avatar { label: "Eve Marsh", src: "../../assets/portraits/eve.svg",
                        size: AvatarSize::Small }
                    Text { class: "nd-identifier", size: TextSize::Small, weight: TextWeight::Medium, "Eve Marsh" }
                }
                LineageFigure { scene: "season", at: "eve", occasion: "christmas",
                    Avatar { label: "Eve Marsh", src: "../../assets/portraits/eve-christmas.svg",
                        size: AvatarSize::Small }
                    Text { class: "nd-identifier", size: TextSize::Small, weight: TextWeight::Medium, "Eve Marsh" }
                }
                LineageFigure { scene: "season", at: "cara",
                    Avatar { label: "Cara Doe", src: "../../assets/portraits/cara.svg",
                        size: AvatarSize::Small }
                    Text { class: "nd-identifier", size: TextSize::Small, weight: TextWeight::Medium, "Cara Doe" }
                }
                LineageFigure { scene: "season", at: "cara", occasion: "christmas",
                    Avatar { label: "Cara Doe", src: "../../assets/portraits/cara-christmas.svg",
                        size: AvatarSize::Small }
                    Text { class: "nd-identifier", size: TextSize::Small, weight: TextWeight::Medium, "Cara Doe" }
                }
            }
        }
    ),
    example!(
        r#"Lineage { sets: "doe:local", LineageFigure { action, swaps: "doe:remote" } }"#,
        Key::ExampleLineageServerNote,
        |context| rsx! {
            Lineage {
                name: "server",
                label: context.t(Key::ExampleLineageServer),
                graph: graphs::SERVER,
                height: LineageHeight::Short,
                LineageFigure {
                    scene: "server",
                    at: "srv",
                    tone: LineageFigureTone::Success,
                    action: "lineage-swap",
                    swaps: "doe:remote",
                    label: context.t(Key::ExampleLineageServer),
                    // A SERVER IS A CARD WITH A NAME AND A STATUS — the round
                    // version was reported as "ridiculously just a green
                    // circle with another green circle inside".
                    Text { class: "nd-identifier", size: TextSize::Small, weight: TextWeight::Medium, "Doe" }
                    Badge { label: context.t(Key::ExampleLineageOnline), dot: true,
                        tone: "success" }
                }
            }
        }
    ),
    example!(
        r#"Lineage { graph: "…origin span=2" }  ·  "figure title rank=0 span=3 cluster=doe"  ·  three families"#,
        Key::ExampleLineageOutrightNote,
        |context| rsx! {
            Lineage {
                name: "aviary",
                label: context.t(Key::ExampleLineageCompany),
                graph: graphs::COMPANY,
            }
            Lineage {
                name: "placed",
                label: context.t(Key::ExampleLineagePlaced),
                graph: graphs::PLACED,
                height: LineageHeight::Full,
            }
            Lineage {
                name: "four",
                label: context.t(Key::ExampleLineageBranches),
                graph: graphs::FOUR,
            }
        }
    ),
    example!(
        r#"…"mark note kind=milestone slot" — a note standing on the structure"#,
        Key::ExampleLineageMarkNote,
        |context| rsx! {
            Lineage {
                name: "marked",
                label: context.t(Key::ExampleLineageMark),
                graph: graphs::MARKED,
                height: LineageHeight::Short,
                LineageFigure {
                    scene: "marked",
                    at: "note",
                    shape: LineageFigureShape::Card,
                    tone: LineageFigureTone::Info,
                    Text { size: TextSize::Caption, weight: TextWeight::Medium,
                        "{context.t(Key::ExampleLineageMilestone)}"
                    }
                    Text { class: "nd-identifier", size: TextSize::Caption, tone: TextTone::Muted, "1970" }
                }
            }
        }
    ),
    example!(
        r#"Lineage { reach: Shy, lettering: Solid, graph: "…shape=bare…" }"#,
        Key::ExampleLineageWildNote,
        |context| rsx! {
            Lineage {
                name: "wild",
                label: context.t(Key::ExampleLineageWild),
                graph: graphs::WILD,
                reach: LineageReach::Shy,
                lettering: LineageLettering::Solid,
                tilt: LineageTilt::Resting,
                LineageFigure {
                    scene: "wild",
                    at: "noctua",
                    shape: LineageFigureShape::Bare,
                    label: context.t(Key::ExampleLineageOwl),
                    // THE MODEL STAGE, WITH THE FILE AS ITS INPUT. The
                    // feature knows nothing about owls: `model.js` takes any
                    // `.glb` through `data-nd-model` plus optional
                    // `data-nd-model-*` parameters, and this example sets
                    // NONE of them — the point is that the defaults carry a
                    // real scanned, rigged, animated model. Until the file
                    // exists (see `assets/vendor/owl/ATTRIBUTION.md`) the
                    // portrait below keeps the stage.
                    div { class: "nd-model-stage", "data-nd-model": "assets/vendor/owl.glb",
                        span { "data-nd-model-flat": "",
                            Avatar {
                                label: "Noctua",
                                src: "../../assets/portraits/noctua.svg",
                                size: AvatarSize::Small,
                            }
                        }
                    }
                    Text { class: "nd-identifier", size: TextSize::Small, weight: TextWeight::Medium, "Noctua" }
                }
            }
        }
    ),
    example!(
        r#"div { "data-nd-model-size": "2", … }  Text { … }  Text { blurb }"#,
        Key::ExampleLineageQueenNote,
        |context| rsx! {
            Lineage {
                name: "queen",
                label: context.t(Key::ExampleLineageQueen),
                graph: graphs::QUEEN,
                reach: LineageReach::Shy,
                lettering: LineageLettering::Solid,
                tilt: LineageTilt::Resting,
                // FULL, because the default window is a cap. `tall` holds
                // five pitches at most, and a doubled model's air makes
                // this drawing taller than that — inside a capped window
                // the family slid below the fold. A scene whose point is
                // one large figure shows all of itself.
                height: LineageHeight::Full,
                LineageFigure {
                    scene: "queen",
                    at: "noctua",
                    shape: LineageFigureShape::Bare,
                    label: context.t(Key::ExampleLineageOwl),
                    // THE SAME FILE, TWICE THE PRESENTATION: `size` is a
                    // multiplier on the fitted default, and the no-cut
                    // guarantee does not move — the canvas overdraws
                    // proportionally further instead.
                    div { class: "nd-model-stage",
                        "data-nd-model": "assets/vendor/owl.glb",
                        "data-nd-model-size": "2",
                        span { "data-nd-model-flat": "",
                            Avatar {
                                label: "Noctua",
                                src: "../../assets/portraits/noctua.svg",
                                size: AvatarSize::Small,
                            }
                        }
                    }
                    Text { class: "nd-identifier", size: TextSize::Small, weight: TextWeight::Medium, "Noctua" }
                    // ANY component, in order: the description below the
                    // name is nothing but one more child the figure was
                    // handed — the scene does not care what it is.
                    Text { class: "nd-lineage-blurb", size: TextSize::Small,
                        {context.t(Key::ExampleLineageQueenCaption)}
                    }
                }
            }
        }
    ),
];

/// The family with its adopted child, whose word and whose sentence are
/// messages — so the graph is built here rather than being a constant.
fn adopted(context: Context) -> String {
    format!(
        "{}{}",
        graphs::DOE,
        graphs::ADOPTED
            .replace("{word}", context.t(Key::ExampleLineageAdopted))
            .replace("{note}", context.t(Key::ExampleLineageAdoptedNote))
    )
}

/// The adoption scene, whose words are messages.
///
/// UNKNOWN IS A SENTENCE, NOT A DASH. A card with `—` on it says a field was
/// left empty; a card saying *biological father unknown* says what is true
/// about a person, which is what a reader asked for. The kind is what makes the
/// card two cells wide, so the sentence fits.
fn adoption(context: Context) -> String {
    graphs::ADOPTION
        .replace("{word}", context.t(Key::ExampleLineageAdopted))
        .replacen("{nobody}", context.t(Key::ExampleLineageNoFather), 1)
        .replace("{nobody}", context.t(Key::ExampleLineageNoMother))
}

/// The donor scene, whose words are messages.
fn donor(context: Context) -> String {
    graphs::DONOR
        .replace("{donor}", context.t(Key::ExampleLineageDonor))
        .replace("{partner}", context.t(Key::ExampleLineagePartner))
        .replace("{nobody}", context.t(Key::ExampleLineageNoParent))
}

/// The spanning scene, whose two unknown parents are messages.
fn spanning(context: Context) -> String {
    graphs::SPANNING
        .replacen("{nobody}", context.t(Key::ExampleLineageNoMother), 1)
        .replace("{nobody}", context.t(Key::ExampleLineageNoFather))
}

/// The figures, in the order the page tells them, which between them set
/// every prop the contract declares.
const LINEAGE_FIGURE: &[Example] = &[
    example!(
        r#"LineageFigure { scene: "…", at: "ada", shape: Card, tone: Accent, parallax: Subtle }"#,
        Key::ExampleLineageFigureNote,
        |context| rsx! {
            Lineage {
                name: "figure-basic",
                label: context.t(Key::ExampleLineageSlots),
                graph: graphs::SLOTS,
                parallax: true,
                // AND THE ORIGIN, because the graph declares it a slot and a
                // slot nobody fills is a connection running into an empty box.
                LineageFigure {
                    scene: "figure-basic",
                    at: "noctua",
                    shape: LineageFigureShape::Plaque,
                    Text { class: "nd-identifier", size: TextSize::Small, weight: TextWeight::Medium, "Noctua" }
                }
                LineageFigure {
                    scene: "figure-basic",
                    at: "ada",
                    shape: LineageFigureShape::Card,
                    tone: LineageFigureTone::Accent,
                    // And the accent card LEANS, so the figure's own
                    // `parallax` keeps a written example.
                    parallax: LineageFigureParallax::Subtle,
                    Avatar { label: "Ada Doe", src: "../../assets/portraits/ada.svg",
                        size: AvatarSize::Small }
                    Text { class: "nd-identifier", size: TextSize::Small, weight: TextWeight::Medium, "Ada Doe" }
                    Text { class: "nd-identifier", size: TextSize::Caption, tone: TextTone::Muted, "1946" }
                }
                // THE SECOND SLOT, AND IT IS NOT DECORATION. The graph declares
                // two, and a slot nobody fills draws an empty box with the
                // connections still running into it. It also puts the tone
                // beside its absence, which is what the pair is showing.
                LineageFigure {
                    scene: "figure-basic",
                    at: "eve",
                    shape: LineageFigureShape::Card,
                    Avatar { label: "Eve Marsh", src: "../../assets/portraits/eve.svg",
                        size: AvatarSize::Small }
                    Text { class: "nd-identifier", size: TextSize::Small, weight: TextWeight::Medium, "Eve Marsh" }
                    Text { class: "nd-identifier", size: TextSize::Caption, tone: TextTone::Muted, "1948" }
                }
            }
        }
    ),
    example!(
        r"LineageFigure { shape: Plaque, tone: Warning }  ·  { shape: Bare }",
        |context| rsx! {
            Lineage {
                name: "figure-shapes",
                label: context.t(Key::ExampleLineageSlots),
                graph: graphs::SLOTS,
                // A PLAQUE IS TWO CELLS WIDE, SO IT STANDS WHERE TWO CELLS
                // ARE FREE. At `ada` it reached across the union dot beside her
                // — a shape overlapping the structure it is standing on.
                LineageFigure {
                    scene: "figure-shapes",
                    at: "noctua",
                    shape: LineageFigureShape::Plaque,
                    tone: LineageFigureTone::Warning,
                    Text { class: "nd-identifier", size: TextSize::Small, weight: TextWeight::Medium, "Noctua" }
                }
                LineageFigure {
                    scene: "figure-shapes",
                    at: "ada",
                    Avatar { label: "Ada Doe", src: "../../assets/portraits/ada.svg",
                        size: AvatarSize::Small }
                    Text { class: "nd-identifier", size: TextSize::Small, weight: TextWeight::Medium, "Ada Doe" }
                }
                LineageFigure {
                    scene: "figure-shapes",
                    at: "eve",
                    shape: LineageFigureShape::Bare,
                    Avatar { label: "Eve Marsh", src: "../../assets/portraits/eve.svg",
                        size: AvatarSize::Large }
                }
            }
        }
    ),
    example!(
        r#"LineageFigure { href: "…", label: "Ada Doe", occasion: "christmas" }"#,
        |context| rsx! {
            Lineage {
                name: "figure-link",
                label: context.t(Key::ExampleLineageSlots),
                graph: graphs::SLOTS,
                occasion: "christmas",
                // AND THE ORIGIN, because the graph declares it a slot and a
                // slot nobody fills is a connection running into an empty box.
                LineageFigure {
                    scene: "figure-link",
                    at: "noctua",
                    shape: LineageFigureShape::Plaque,
                    Text { class: "nd-identifier", size: TextSize::Small, weight: TextWeight::Medium, "Noctua" }
                }
                LineageFigure {
                    scene: "figure-link",
                    at: "eve",
                    href: "../tree/",
                    Avatar { label: "Eve Marsh", src: "../../assets/portraits/eve.svg" }
                    Text { class: "nd-identifier", size: TextSize::Small, weight: TextWeight::Medium, "Eve Marsh" }
                }
                LineageFigure {
                    scene: "figure-link",
                    at: "ada",
                    occasion: "christmas",
                    tone: LineageFigureTone::Danger,
                    label: "Ada Doe",
                    Avatar { label: "Ada Doe", src: "../../assets/portraits/ada-christmas.svg",
                        size: AvatarSize::Small }
                    // The name FIRST — a card whose only text was "wearing a
                    // paper hat" never said whose head the hat was on.
                    Text { class: "nd-identifier", size: TextSize::Small, weight: TextWeight::Medium, "Ada Doe" }
                    Text { size: TextSize::Caption, tone: TextTone::Muted,
                        "{context.t(Key::ExampleLineageHat)}"
                    }
                }
            }
        }
    ),
    example!(
        r#"LineageFigure { action: "lineage-swap", swaps: "doe:remote", set: "doe:local" }"#,
        Key::ExampleLineageServerNote,
        |context| rsx! {
            Lineage {
                name: "figure-server",
                label: context.t(Key::ExampleLineageServer),
                graph: graphs::SERVER,
                sets: "doe:local",
                LineageFigure {
                    scene: "figure-server",
                    at: "srv",
                    shape: LineageFigureShape::Round,
                    tone: LineageFigureTone::Success,
                    action: "lineage-swap",
                    swaps: "doe:remote",
                    label: context.t(Key::ExampleLineageServer),
                    Text { class: "nd-identifier", size: TextSize::Caption, weight: TextWeight::Medium, "Doe" }
                }

            }
        }
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
Button { menu: true, controls: "overview-button-menu", label: "More actions" }"#,
            |context| rsx! {
                div { class: "nd-example-row",
                    Button {
                        variant: ButtonVariant::Solid,
                        tone: ButtonTone::Accent,
                        icon: IconName::Check,
                        label: context.t(Key::ExampleButtonPublish),
                    }
                    Button {
                        variant: ButtonVariant::Outline,
                        tone: ButtonTone::Neutral,
                        label: context.t(Key::ExampleButtonSave),
                    }
                    Button {
                        variant: ButtonVariant::Transparent,
                        tone: ButtonTone::Neutral,
                        icon: IconName::Copy,
                        label: context.t(Key::ExampleCopy),
                    }
                    Button {
                        menu: true,
                        controls: "overview-button-menu",
                        variant: ButtonVariant::Outline,
                        tone: ButtonTone::Neutral,
                        label: context.t(Key::ExampleButtonMore),
                    }
                    Menu {
                        name: "overview-button-menu",
                        label: context.t(Key::ExampleMenuLabel),
                        MenuItem { label: context.t(Key::ExampleMenuDuplicate) }
                        MenuItem { label: context.t(Key::ExampleMenuRename) }
                        MenuItem {
                            tone: "danger",
                            label: context.t(Key::ExampleMenuDelete),
                            detail: context.t(Key::ExampleMenuDeleteDetail),
                        }
                    }
                }
            }
        ),
    ),
    (
        "avatars",
        example!(
            r#"Avatar { decorative: true, initials: "AL", src: "…", label: "…" }
AvatarGroup { label: "Reviewers", more: "+5", more_label: "…", more_href: "…" }"#,
            |context| rsx! {
                div { class: "nd-example-stack",
                    // One person, beside the name that is already on the screen —
                    // which is the whole of `decorative`.
                    span { class: "nd-example-byline",
                        Avatar {
                            decorative: true,
                            initials: "AL",
                            src: format!("{}favicon.svg", crate::site::BASE),
                            label: context.t(Key::ExampleAvatarAna),
                        }
                        Text { "{context.t(Key::ExampleAvatarAna)}" }
                    }
                    AvatarGroup {
                        label: context.t(Key::ExampleAvatarReviewers),
                        more: "+5",
                        more_label: context.t(Key::ExampleAvatarMore),
                        more_href: "../avatars/",
                        li { Avatar { label: context.t(Key::ExampleAvatarBruno), initials: "BS" } }
                        li { Avatar { label: context.t(Key::ExampleAvatarCarla), initials: "CM" } }
                        li {
                            Avatar {
                                label: context.t(Key::ExampleAvatarPending),
                                icon: IconName::User,
                            }
                        }
                    }
                }
            }
        ),
    ),
    (
        "badge",
        example!(
            r#"Badge { count: "3", label: "3 unread messages", Button { … } }
Badge { count: "128", max: "99", tone: "accent", … }
Badge { dot: true, tone: "neutral", … }"#,
            |context| rsx! {
                // A ROOMIER ROW, for the reason the placement example below
                // gives: a badge draws OUTSIDE the box of what it marks, and
                // three of them in an ordinary row sit on their neighbours.
                div { class: "nd-example-row nd-example-row-airy",
                    Badge {
                        count: "3",
                        label: context.t(Key::ExampleBadgeUnread),
                        Button {
                            variant: ButtonVariant::Outline,
                            tone: ButtonTone::Neutral,
                            label: context.t(Key::ExampleBadgeInbox),
                        }
                    }
                    Badge {
                        count: "128",
                        max: "99",
                        tone: "accent",
                        label: context.t(Key::ExampleOverviewBadgeWaiting),
                        Button {
                            variant: ButtonVariant::Outline,
                            tone: ButtonTone::Neutral,
                            label: context.t(Key::ExampleOverviewBadgeReviews),
                        }
                    }
                    Badge {
                        dot: true,
                        tone: "neutral",
                        label: context.t(Key::ExampleBadgeChanged),
                        Button {
                            variant: ButtonVariant::Transparent,
                            tone: ButtonTone::Neutral,
                            hide_label: true,
                            icon: IconName::Settings,
                            label: context.t(Key::SettingsLabel),
                        }
                    }
                }
            }
        ),
    ),
    (
        "blockquote",
        example!(
            r#"Blockquote { tone: Accent, cite: "…", attribution: "Edsger W. Dijkstra", Paragraph { … } }"#,
            |context| rsx! {
                Blockquote {
                    tone: BlockquoteTone::Accent,
                    cite: "https://example.invalid/on-simplicity",
                    attribution: "Edsger W. Dijkstra",
                    Paragraph { "{context.t(Key::ExampleQuote)}" }
                }
            }
        ),
    ),
    (
        "carousel",
        example!(
            r#"Carousel { name, label, per_view: "1", snap: Center, dots: true, counter: true }
CarouselSlide { label: "…" }"#,
            |context| rsx! {
                Carousel {
                    name: "overview-carousel",
                    label: context.t(Key::ExampleCarouselLabel),
                    per_view: "1",
                    snap: CarouselSnap::Center,
                    dots: true,
                    dot_label: context.t(Key::ExampleCarouselDot),
                    counter: true,
                    counter_label: context.t(Key::ExampleCarouselCounter),
                    announce_label: context.t(Key::ExampleCarouselAnnounce),
                    previous_label: context.t(Key::ExampleCarouselPrevious),
                    next_label: context.t(Key::ExampleCarouselNext),
                    CarouselSlide { label: context.t(Key::ExampleCarouselSlideOne),
                        Paragraph { {context.t(Key::ExampleCarouselSlideOne)} }
                    }
                    CarouselSlide { label: context.t(Key::ExampleCarouselSlideTwo),
                        Paragraph { {context.t(Key::ExampleCarouselSlideTwo)} }
                    }
                    CarouselSlide { label: context.t(Key::ExampleCarouselSlideThree),
                        Paragraph { {context.t(Key::ExampleCarouselSlideThree)} }
                    }
                    CarouselSlide { label: context.t(Key::ExampleCarouselSlideFour),
                        Paragraph { {context.t(Key::ExampleCarouselSlideFour)} }
                    }
                }
            }
        ),
    ),
    (
        "checkbox",
        example!(
            r#"Fieldset { legend: "…", hint: "…" }
Checkbox { name: "apply", value: "work", label: "…", hint: "…", checked: true }
Checkbox { value: "none", divider: "or", exclusive: true }"#,
            |context| rsx! {
                Fieldset {
                    name: "overview-apply",
                    legend: context.t(Key::ExampleFormsApplyLegend),
                    hint: context.t(Key::ExampleFormsApplyHint),
                    Checkbox {
                        name: "overview-apply",
                        value: "study",
                        label: context.t(Key::ExampleFormsApplyStudy),
                        hint: context.t(Key::ExampleFormsApplyStudyHint),
                    }
                    Checkbox {
                        name: "overview-apply",
                        value: "work",
                        label: context.t(Key::ExampleFormsApplyWork),
                        hint: context.t(Key::ExampleFormsApplyWorkHint),
                        checked: true,
                    }
                    Checkbox {
                        name: "overview-apply",
                        value: "care",
                        label: context.t(Key::ExampleFormsApplyCare),
                    }
                    Checkbox {
                        name: "overview-apply",
                        value: "none",
                        label: context.t(Key::ExampleFormsApplyNone),
                        divider: context.t(Key::ExampleFormsOr),
                        exclusive: true,
                    }
                }
            }
        ),
    ),
    (
        "code-block",
        example!(
            r#"CodeBlock { code, language: "rust", filename: "scope.rs", numbered: true, mark_lines: "4-6", copy: "…", copied: "…", caption: "…" }"#,
            |context| rsx! {
                CodeBlock {
                    code: SAMPLE_RUST,
                    language: "rust",
                    filename: "scope.rs",
                    numbered: true,
                    mark_lines: "4-6",
                    copy: context.t(Key::ExampleCopy),
                    copied: context.t(Key::ExampleCopied),
                    caption: context.t(Key::ExampleOverviewCodeBlockCaption),
                }
            }
        ),
    ),
    (
        "color-picker",
        example!(
            r#"ColorPicker { name: "overview-label-colour", label: "…", hint: "…", value: "…", eyedropper: true, formats: "hex rgb oklch" }"#,
            |context| rsx! {
                ColorPicker {
                    name: "overview-label-colour",
                    label: context.t(Key::ExampleColorPickerLabel),
                    hint: context.t(Key::ExampleColorPickerHint),
                    value: "#7c3aed", // allow-literal: the colour a ColorPicker opens on IS a colour, and a picker whose starting value came from a palette token would follow the theme instead of showing the reader their own
                    trigger_label: context.t(Key::ExampleColorPickerTrigger),
                    area_label: context.t(Key::ExampleColorPickerArea),
                    hue_label: context.t(Key::ExampleColorPickerHue),
                    entry_label: context.t(Key::ExampleColorPickerEntry),
                    entry_invalid_label: context.t(Key::ExampleColorPickerEntryInvalid),
                    eyedropper: true,
                    eyedropper_label: context.t(Key::ExampleColorPickerEyedropper),
                    // THE NOTATIONS ARE FOLDED, AND THE COLOUR IS NOT. Three
                    // formats rather than four: an overview shows that the
                    // codes are there and what one looks like, and the full
                    // list is the example that is about `formats`.
                    codes_label: context.t(Key::ExampleColorPickerCodes),
                    formats: "hex rgb oklch",
                    copy_label: context.t(Key::ExampleColorPickerCopy),
                    copied_label: context.t(Key::ExampleColorPickerCopied),
                    close_label: context.t(Key::ExampleColorPickerClose),
                }
            }
        ),
    ),
    (
        "disclosure",
        example!(
            r#"Disclosure { summary: "…", heading: DisclosureHeading::H3, open: true }
Disclosure { summary: "…", heading: DisclosureHeading::H3 }"#,
            |context| rsx! {
                div { class: "nd-example-stack",
                    Disclosure {
                        summary: context.t(Key::ExampleDisclosureSummary),
                        heading: DisclosureHeading::H3,
                        open: true,
                        Paragraph { "{context.t(Key::ExampleDisclosureBody)}" }
                    }
                    Disclosure {
                        summary: context.t(Key::ExampleDisclosureSection),
                        heading: DisclosureHeading::H3,
                        Paragraph { "{context.t(Key::ExampleOverviewDisclosureA11y)}" }
                    }
                }
            }
        ),
    ),
    (
        "error-summary",
        example!(
            r#"ErrorSummary { name: "…", title: "There is a problem", description: "…" }
ErrorSummaryItem { control: "nd-input-overview-name", "Enter your full name" }
ErrorSummaryItem { control: "nd-overview-terms-yes", "…" }"#,
            Key::ExampleOverviewErrorSummaryNote,
            |context| rsx! {
                form { class: "nd-example-form", method: "get", action: "#", novalidate: true,
                    ErrorSummary {
                        name: "overview",
                        title: context.t(Key::ExampleSummaryTitle),
                        description: context.t(Key::ExampleSummaryDescription),
                        // Off, and only because this is a page of examples. A
                        // summary that is in the markup takes the focus as the
                        // page paints — which is the whole point of it on a
                        // form, and unreadable on a page of six other things.
                        focus: false,
                        ErrorSummaryItem {
                            control: "nd-input-overview-name",
                            {context.t(Key::ExampleSummaryNameError)}
                        }
                        ErrorSummaryItem {
                            control: "nd-input-overview-email",
                            {context.t(Key::ExampleSummaryEmailError)}
                        }
                        // ONE ENTRY FOR ONE QUESTION, whatever it is asked with:
                        // it points at the first box of the group, which is
                        // where a reader fixing it starts.
                        ErrorSummaryItem {
                            control: "nd-overview-terms-yes",
                            {context.t(Key::ExampleFormsTermsError)}
                        }
                    }
                    Input {
                        name: "overview-name",
                        label: context.t(Key::ExampleSummaryNameLabel),
                        error: context.t(Key::ExampleSummaryNameError),
                        autocomplete: "name",
                    }
                    Input {
                        name: "overview-email",
                        label: context.t(Key::ExampleSummaryEmailLabel),
                        kind: InputKind::Email,
                        error: context.t(Key::ExampleSummaryEmailError),
                        autocomplete: "email",
                    }
                    Fieldset {
                        name: "overview-terms",
                        legend: context.t(Key::ExampleFormsTermsLegend),
                        error: context.t(Key::ExampleFormsTermsError),
                        error_label: context.t(Key::ExampleFormsErrorWord),
                        Checkbox {
                            name: "overview-terms",
                            value: "yes",
                            label: context.t(Key::ExampleFormsTermsAgree),
                            invalid: true,
                        }
                    }
                    Button { submit: ButtonSubmit::Submit, label: context.t(Key::ExampleSummarySend) }
                }
            }
        ),
    ),
    (
        "field",
        example!(
            r"Field { control, label, hint, required, required_label } / Field { kind: Group, label, hint }",
            |context| rsx! {
                div { class: "nd-example-stack",
                    Input {
                        name: "overview-field-email",
                        label: context.t(Key::ExampleInputEmailLabel),
                        kind: InputKind::Email,
                        hint: context.t(Key::ExampleInputHint),
                        required: true,
                        required_label: context.t(Key::ExampleInputRequired),
                        autocomplete: "email",
                    }
                    // The other `kind`: a `<legend>` over three boxes, because a
                    // `<label>` names one element and a set of controls needs a
                    // name of its own.
                    TimeInput {
                        name: "overview-field-time",
                        legend: context.t(Key::ExampleInputTimeLegend),
                        hour_label: context.t(Key::ExampleInputHour),
                        minute_label: context.t(Key::ExampleInputMinute),
                        hint: context.t(Key::ExampleInputTimeHint),
                    }
                }
            }
        ),
    ),
    (
        "fieldset",
        example!(
            r#"Fieldset { name: "overview-apply", legend: "…", hint: "…" }
Checkbox { name: "overview-apply", value: "study", label: "…", hint: "…" }"#,
            |context| rsx! {
                Fieldset {
                    name: "overview-apply",
                    legend: context.t(Key::ExampleFormsApplyLegend),
                    hint: context.t(Key::ExampleFormsApplyHint),
                    Checkbox {
                        name: "overview-apply",
                        value: "study",
                        label: context.t(Key::ExampleFormsApplyStudy),
                        hint: context.t(Key::ExampleFormsApplyStudyHint),
                    }
                    Checkbox {
                        name: "overview-apply",
                        value: "work",
                        label: context.t(Key::ExampleFormsApplyWork),
                        hint: context.t(Key::ExampleFormsApplyWorkHint),
                        checked: true,
                    }
                    Checkbox {
                        name: "overview-apply",
                        value: "care",
                        label: context.t(Key::ExampleFormsApplyCare),
                    }
                }
            }
        ),
    ),
    (
        "heading",
        example!(
            r"Heading { level: _2, size: HeadingSize::Display, balance: true }
Heading { level: _3, size: HeadingSize::H3, tone: HeadingTone::Muted }",
            |context| rsx! {
                Prose { measure: ProseMeasure::Default,
                    Heading {
                        level: HeadingLevel::_2,
                        size: HeadingSize::Display,
                        balance: true,
                        "{context.t(Key::ExampleHeadingLong)}"
                    }
                    Paragraph { size: ParagraphSize::Lead, tone: ParagraphTone::Muted,
                        "{context.t(Key::ExampleListLong)}"
                    }
                    Heading {
                        level: HeadingLevel::_3,
                        size: HeadingSize::H3,
                        tone: HeadingTone::Muted,
                        "{context.t(Key::ExampleHeadingSection)}"
                    }
                }
            }
        ),
    ),
    (
        "info",
        example!(
            r#"Info { name: "…", label: "What counts as a token", heading: "…", close_label: "…" }"#,
            |context| rsx! {
                Text {
                    "{context.t(Key::ExampleInfoUsage)}"
                    Info {
                        name: "overview-usage",
                        label: context.t(Key::ExampleInfoUsageLabel),
                        heading: context.t(Key::ExampleInfoUsageHeading),
                        close_label: context.t(Key::ExampleInfoClose),
                        "{context.t(Key::ExampleInfoUsageNote)} {context.t(Key::ExampleLinkLead)} "
                        Link { href: "#", "{context.t(Key::ExampleLinkText)}" }
                        "."
                    }
                }
            }
        ),
    ),
    (
        "inline-text",
        example!(
            r#"Paragraph {
    Strong { … } Time { datetime: "2026-07-29", … } Code { … } Del { label: "removed:", … } Ins { label: "added:", … }
    Abbr { title: "…", … } Em { … } Mark { … } Kbd { … } Text { numeric: true, … }
}"#,
            |context| rsx! {
                Paragraph {
                    Strong { "{context.t(Key::ExampleOverviewInlineTextRelease)}" }
                    " {context.t(Key::ExampleOverviewInlineTextShipped)} "
                    Time { datetime: "2026-07-29", "{context.t(Key::ExampleTimeDate)}" }
                    "{context.t(Key::ExampleOverviewInlineTextInside)} "
                    Code { class: "nd-identifier", "ThemeScope" }
                    "{context.t(Key::ExampleOverviewInlineTextOld)} "
                    Del {
                        class: "nd-identifier",
                        label: context.t(Key::ExampleEditRemoved),
                        "--noctua-accent"
                    }
                    " {context.t(Key::ExampleOverviewInlineTextBecame)} "
                    Ins {
                        class: "nd-identifier",
                        label: context.t(Key::ExampleEditAdded),
                        "--nc-accent-solid"
                    }
                    "{context.t(Key::ExampleOverviewInlineTextFloor)} "
                    Abbr {
                        class: "nd-identifier",
                        title: "Web Content Accessibility Guidelines",
                        "WCAG"
                    }
                    " {context.t(Key::ExampleOverviewInlineTextFloorEnd)} "
                    Em { "{context.t(Key::ExampleOverviewInlineTextEvery)}" }
                    " {context.t(Key::ExampleOverviewInlineTextSearch)} "
                    Mark { "{context.t(Key::ExampleMarkWord)}" }
                    " {context.t(Key::ExampleOverviewInlineTextAgain)} "
                    Kbd { class: "nd-identifier", "Esc" }
                    " {context.t(Key::ExampleOverviewInlineTextCloses)} "
                    Text { class: "nd-identifier", numeric: true, "1 767" }
                    " {context.t(Key::ExampleOverviewInlineTextDeclarations)}"
                }
            }
        ),
    ),
    (
        "lineage",
        example!(
            r#"Lineage { name: "overview-doe", label: "…", graph: "figure gran … bond u1 cara", pick: Both, camera_label, zoom_in_label, zoom_out_label, fit_label }"#,
            Key::ExampleLineageActingNote,
            |context| rsx! {
                Lineage {
                    name: "overview-doe",
                    label: context.t(Key::ExampleLineageDoe),
                    // THREE GENERATIONS, WHICH IS THE SMALLEST SCENE THAT IS A
                    // FAMILY TREE. Two people and a child show the parts; a
                    // grandparents' union carrying two children, one of whom
                    // holds a union of their own, shows what the parts are for.
                    graph: graphs::DOE,
                    // The scene explains itself and submits nothing, so a pick
                    // is a thing on the screen and nowhere else — no `form`.
                    pick: LineagePick::Both,
                    camera_label: context.t(Key::ExampleLineageCamera),
                    zoom_in_label: context.t(Key::ExampleLineageCloser),
                    zoom_out_label: context.t(Key::ExampleLineageFurther),
                    fit_label: context.t(Key::ExampleLineageFit),
                }
            }
        ),
    ),
    (
        "list",
        example!(
            r#"List { kind: ListKind::Ordered } with ListItem { "…" }
List { kind: ListKind::Description, spacing: Tight } with ListItem { term: "…", "…" }"#,
            |context| rsx! {
                div { class: "nd-example-stack",
                    List { kind: ListKind::Ordered,
                        ListItem { "{context.t(Key::ExampleOverviewListDeclare)}" }
                        ListItem { "{context.t(Key::ExampleOverviewListBuild)}" }
                        ListItem { "{context.t(Key::ExampleOverviewListExamples)}" }
                    }
                    List { kind: ListKind::Description, spacing: ListSpacing::Tight,
                        ListItem {
                            term: context.t(Key::ThemeAccentLabel),
                            "{context.t(Key::ExampleTermAccent)}"
                        }
                        ListItem {
                            term: context.t(Key::ThemeGlassLabel),
                            "{context.t(Key::ExampleTermGlass)}"
                        }
                    }
                }
            }
        ),
    ),
    (
        "list-item",
        example!(
            r#"List { kind: ListKind::Description }
ListItem { term: "Accent", "…" }"#,
            |context| rsx! {
                List { kind: ListKind::Description,
                    ListItem {
                        term: context.t(Key::ThemeAccentLabel),
                        "{context.t(Key::ExampleTermAccent)}"
                    }
                    ListItem {
                        term: context.t(Key::ThemeGlassLabel),
                        "{context.t(Key::ExampleTermGlass)}"
                    }
                    ListItem {
                        term: context.t(Key::ThemeRadiusLabel),
                        "{context.t(Key::ExampleOverviewListItemCorners)}"
                    }
                }
            }
        ),
    ),
    (
        "menu",
        example!(
            r#"Button { menu: true, controls: "overview-menu-actions", label: "Actions" }
Menu { name: "overview-menu-actions", label: "…",
    MenuItem { icon: "copy", label: "…" }
    MenuItem { icon: "close", tone: "danger", label: "…", detail: "…" }
}"#,
            |context| rsx! {
                div {
                    Button {
                        menu: true,
                        controls: "overview-menu-actions",
                        variant: ButtonVariant::Outline,
                        tone: ButtonTone::Neutral,
                        label: context.t(Key::ExampleMenuOpen),
                    }
                    Menu {
                        name: "overview-menu-actions",
                        label: context.t(Key::ExampleMenuLabel),
                        MenuItem { icon: "copy", label: context.t(Key::ExampleMenuDuplicate) }
                        MenuItem { label: context.t(Key::ExampleMenuRename) }
                        MenuItem { icon: "download", label: context.t(Key::ExampleMenuArchive) }
                        MenuItem {
                            icon: "close",
                            tone: "danger",
                            label: context.t(Key::ExampleMenuDelete),
                            detail: context.t(Key::ExampleMenuDeleteDetail),
                        }
                    }
                }
            }
        ),
    ),
    (
        "meter",
        example!(
            r#"Meter { name: "overview-storage", label: "…", value: "212", max: "256", unit: "GB", low: "128", high: "204", optimum: "0", value_label: "…" }
Meter { name: "overview-seats", label: "…", value: "16", max: "20", value_label: "…" }"#,
            |context| rsx! {
                div { class: "nd-example-stack",
                    // A DISK AT 83%, PAST ITS OWN HIGH THRESHOLD. `optimum`
                    // sits under `low` because empty is the good end of a
                    // disk, so the platform paints this reading as trouble
                    // with nobody writing a condition.
                    //
                    // A DIFFERENT DISK FROM THE ONE THE EXAMPLES BELOW USE,
                    // so no two meters on this page are one reading twice.
                    Meter {
                        name: "overview-storage",
                        label: context.t(Key::ExampleMeterStorage),
                        value: "212",
                        max: "256",
                        unit: "GB",
                        low: "128",
                        high: "204",
                        optimum: "0",
                        value_label: context.t(Key::ExampleMeterStorageValue),
                    }
                    // And one with no thresholds beside it, which makes no
                    // judgement at all: sixteen seats of twenty is a fact.
                    Meter {
                        name: "overview-seats",
                        label: context.t(Key::ExampleMeterSeats),
                        value: "16",
                        max: "20",
                        value_label: context.t(Key::ExampleMeterSeatsValue),
                    }
                }
            }
        ),
    ),
    (
        "pagination",
        example!(
            r#"Pagination { name, label, page: "3", pages: "4", href, previous_label,
next_label, page_label, current_label, summary }"#,
            |context| rsx! {
                Pagination {
                    name: "overview-pagination",
                    label: context.t(Key::ExamplePaginationLabel),
                    // Ten palettes a page, thirty-nine of them, and the summary
                    // under the control says exactly that — a control whose
                    // numbers disagree with its own sentence is one nobody trusts.
                    page: "3",
                    pages: "4",
                    href: PAGE_HREF,
                    previous_label: context.t(Key::ExamplePaginationPrevious),
                    next_label: context.t(Key::ExamplePaginationNext),
                    page_label: context.t(Key::ExamplePaginationPage),
                    current_label: context.t(Key::ExamplePaginationCurrent),
                    summary: context.t(Key::ExamplePaginationSummary),
                }
            }
        ),
    ),
    (
        "paragraph",
        example!(
            r"Paragraph { size: ParagraphSize::Lead, … }
Paragraph { … }
Paragraph { size: ParagraphSize::Small, tone: ParagraphTone::Muted, … }",
            |context| rsx! {
                div {
                    Paragraph { size: ParagraphSize::Lead,
                        "{context.t(Key::ExampleOverviewParagraphLead)}"
                    }
                    Paragraph { "{context.t(Key::ExampleOverviewParagraphBody)}" }
                    Paragraph {
                        size: ParagraphSize::Small,
                        tone: ParagraphTone::Muted,
                        "{context.t(Key::ExampleOverviewParagraphNote)}"
                    }
                }
            }
        ),
    ),
    (
        "progress",
        example!(
            r"Progress { name, label, value, max, value_label }
Progress { name, label, busy_label } — no value at all",
            |context| rsx! {
                div { class: "nd-example-stack",
                    Progress {
                        name: "overview-upload",
                        label: context.t(Key::ExampleProgressUpload),
                        value: "7.4",
                        max: "12",
                        value_label: context.t(Key::ExampleProgressUploadValue),
                    }
                    Progress {
                        name: "overview-import",
                        label: context.t(Key::ExampleProgressImport),
                        busy_label: context.t(Key::ExampleProgressImportBusy),
                    }
                }
            }
        ),
    ),
    (
        "prose",
        example!(
            r"Prose { Heading { … } Paragraph { size: Lead, … } Paragraph { … } Blockquote { … } }",
            |context| rsx! {
                Prose {
                    Heading { level: HeadingLevel::_3, size: HeadingSize::H3,
                        "{context.t(Key::ExampleOverviewProseHeading)}"
                    }
                    Paragraph { size: ParagraphSize::Lead,
                        "{context.t(Key::ExampleOverviewProseLead)}"
                    }
                    Paragraph { "{context.t(Key::ExampleOverviewProseBody)}" }
                    Blockquote {
                        Paragraph { "{context.t(Key::ExampleQuoteShort)}" }
                    }
                }
            }
        ),
    ),
    (
        "search",
        example!(
            r#"Search { name, label, placeholder, status, empty_label, clear_label }
SearchGroup { label: "…" }
SearchResult { href: "…", label: "…", detail: "…", keywords: "…", current: true }"#,
            |context| rsx! {
                Search {
                    name: "overview-search",
                    label: context.t(Key::SearchLabel),
                    placeholder: context.t(Key::SearchPlaceholder),
                    status: context.t(Key::SearchResults),
                    empty_label: context.t(Key::SearchEmpty),
                    clear_label: context.t(Key::SearchClear),
                    SearchGroup { label: context.t(Key::SearchGroupPages),
                        SearchResult {
                            href: "../spacing/",
                            label: crate::site::SPACING.title(context.locale),
                            detail: context.t(Key::PageSpacingSummary),
                            // Letters a reader might type that the name does not
                            // contain: `gap` reaches this row and nothing else.
                            keywords: "gap inset step scale rhythm",
                        }
                        SearchResult {
                            href: "../typography/",
                            label: crate::site::TYPOGRAPHY.title(context.locale),
                            detail: context.t(Key::PageTypographySummary),
                        }
                    }
                    SearchGroup { label: context.t(Key::SearchGroupComponents),
                        SearchResult { class: "nd-identifier", href: "../button/", label: "Button" }
                        SearchResult { class: "nd-identifier", href: "../select/", label: "Select" }
                        // The page this is on, so `current` is a claim the
                        // document can back rather than a decoration.
                        SearchResult {
                            class: "nd-identifier",
                            href: "../search/",
                            label: crate::site::SEARCH.title(context.locale),
                            current: true,
                        }
                    }
                }
            }
        ),
    ),
    (
        "segmented",
        example!(
            r#"Segmented { name: "…", label: "Mode", fill: SegmentedFill::Equal }
SegmentedItem { value: "light", icon: "sun", label: "Light" }
SegmentedItem { value: "system", icon: "monitor", label: "…", selected: true }"#,
            |context| rsx! {
                Segmented {
                    name: "overview-mode",
                    label: context.t(Key::ThemeModeLabel),
                    fill: SegmentedFill::Equal,
                    for (value, icon, label, selected) in [
                        ("light", IconName::Sun, Key::ThemeModeLight, false),
                        ("dark", IconName::Moon, Key::ThemeModeDark, false),
                        ("system", IconName::Monitor, Key::ThemeModeSystem, true),
                    ] {
                        SegmentedItem { value, icon, label: context.t(label), selected }
                    }
                }
            }
        ),
    ),
    (
        "segmented-item",
        example!(
            r#"Segmented { name, label,
    SegmentedItem { value: "light", label: "…", icon: …, selected: true }
}"#,
            |context| rsx! {
                Segmented { name: "overview-segmented-item", label: context.t(Key::ThemeModeLabel),
                    for (value, icon, label, selected) in [
                        ("light", IconName::Sun, Key::ThemeModeLight, false),
                        ("dark", IconName::Moon, Key::ThemeModeDark, true),
                        ("system", IconName::Monitor, Key::ThemeModeSystem, false),
                    ] {
                        SegmentedItem { value, icon, label: context.t(label), selected }
                    }
                }
            }
        ),
    ),
    (
        "select",
        example!(
            r#"Select { name: "overview-palette", label: "…", hint: "…", placeholder: "…", searchable: true, clearable: true }
SelectGroup { label: "…" }  ·  SelectOption { value: "ochre-balanced", label: "Ochre", swatch: "ochre-balanced", detail: "…" }"#,
            |context| rsx! {
                Select {
                    name: "overview-palette",
                    label: context.t(Key::ExampleSelectPaletteLabel),
                    hint: context.t(Key::ExampleSelectPaletteHint),
                    placeholder: context.t(Key::ExampleSelectPalettePlaceholder),
                    searchable: true,
                    clearable: true,
                    search_label: context.t(Key::ExampleSelectSearchLabel),
                    empty_label: context.t(Key::ExampleSelectEmptyLabel),
                    clear_label: context.t(Key::ExampleSelectClearLabel),
                    SelectGroup { label: context.t(Key::ExampleSelectWarm),
                        SelectOption {
                            class: "nd-identifier",
                            value: "ochre-balanced",
                            label: "Ochre",
                            swatch: "ochre-balanced",
                            detail: context.t(Key::ExampleSelectDetailBalanced),
                        }
                        SelectOption {
                            class: "nd-identifier",
                            value: "amber-vivid",
                            label: "Amber",
                            swatch: "amber-vivid",
                            detail: context.t(Key::ExampleSelectDetailVivid),
                        }
                        SelectOption {
                            class: "nd-identifier",
                            value: "clay-sober",
                            label: "Clay",
                            swatch: "clay-sober",
                            detail: context.t(Key::ExampleSelectDetailSober),
                        }
                    }
                    SelectGroup { label: context.t(Key::ExampleSelectCool),
                        SelectOption {
                            class: "nd-identifier",
                            value: "teal-balanced",
                            label: "Teal",
                            swatch: "teal-balanced",
                            detail: context.t(Key::ExampleSelectDetailBalanced),
                        }
                        SelectOption {
                            class: "nd-identifier",
                            value: "azure-vivid",
                            label: "Azure",
                            swatch: "azure-vivid",
                            detail: context.t(Key::ExampleSelectDetailVivid),
                        }
                        SelectOption {
                            class: "nd-identifier",
                            value: "indigo-sober",
                            label: "Indigo",
                            swatch: "indigo-sober",
                            detail: context.t(Key::ExampleSelectDetailSober),
                        }
                    }
                }
            }
        ),
    ),
    (
        "select-option",
        example!(
            r#"SelectOption { value: "…", label: "…", swatch: "…", detail: "…", selected: true }"#,
            |context| rsx! {
                Select {
                    name: "overview-select-option",
                    label: context.t(Key::ExampleSelectPaletteLabel),
                    hint: context.t(Key::ExampleSelectPaletteHint),
                    SelectOption {
                        class: "nd-identifier",
                        value: "amber-vivid",
                        label: "Amber",
                        swatch: "amber-vivid",
                        detail: context.t(Key::ExampleSelectDetailVivid),
                    }
                    SelectOption {
                        class: "nd-identifier",
                        value: "ochre-balanced",
                        label: "Ochre",
                        swatch: "ochre-balanced",
                        detail: context.t(Key::ExampleSelectDetailBalanced),
                        selected: true,
                    }
                    SelectOption {
                        class: "nd-identifier",
                        value: "clay-sober",
                        label: "Clay",
                        swatch: "clay-sober",
                        detail: context.t(Key::ExampleSelectDetailSober),
                    }
                }
            }
        ),
    ),
    (
        "slider",
        example!(
            r#"Slider { name: "…", label: "Volume", value: "35", min: "0", max: "100", step: "5" }
Slider { marks: "0 50 100", mark_labels: "Silent|Loud", output_label: "{value}%" }"#,
            |context| rsx! {
                Slider {
                    name: "overview-volume",
                    label: context.t(Key::ExampleInputVolumeLabel),
                    value: "35",
                    min: "0",
                    max: "100",
                    step: "5",
                    marks: "0 50 100",
                    mark_labels: context.t(Key::ExampleInputVolumeMarks),
                    output_label: context.t(Key::ExampleInputPercentValue),
                }
            }
        ),
    ),
    (
        "swatch",
        example!(
            r"SegmentedItem { value, label, Swatch { palette: value, size: SwatchSize::Small } }",
            |context| rsx! {
                Segmented {
                    name: "overview-swatch-accent",
                    label: context.t(Key::ThemeAccentLabel),
                    appearance: SegmentedAppearance::Spread,
                    for (value, label, selected) in [
                        ("amber-vivid", Key::ThemeAccentAmber, false),
                        ("jade-balanced", Key::ThemeAccentJade, true),
                        ("teal-sober", Key::ThemeAccentTeal, false),
                        ("rose-vivid", Key::ThemeAccentRose, false),
                    ] {
                        // The chip carries the palette and takes its colour from
                        // it, so no colour is written here — and the label is
                        // what a reader who cannot see the chip chooses by.
                        SegmentedItem { value, label: context.t(label), selected,
                            Swatch { palette: value, size: SwatchSize::Small }
                        }
                    }
                }
            }
        ),
    ),
    (
        "switch",
        example!(
            r#"Switch { name: "overview-grain", on: "on", off: "off", checked: true, label: "…", describedby: "overview-switch-note" }
Switch { name: "overview-pattern", on: "on", off: "off", label: "…" }"#,
            |context| rsx! {
                div { class: "nd-example-stack",
                    Switch {
                        name: "overview-grain",
                        on: "on",
                        off: "off",
                        checked: true,
                        describedby: "overview-switch-note",
                        label: context.t(Key::SettingsGrainLabel),
                    }
                    // THE SENTENCE IS POINTED AT RATHER THAN MERELY NEAR. A
                    // note beside a control is a note a screen reader never
                    // reaches on its way through the setting.
                    p { class: "nd-note-line", id: "overview-switch-note",
                        "{context.t(Key::SettingsGrainNote)}"
                    }
                    Switch {
                        name: "overview-pattern",
                        on: "on",
                        off: "off",
                        label: context.t(Key::ExampleSwitchPattern),
                    }
                }
            }
        ),
    ),
    (
        "tag",
        example!(
            r#"Tag { icon: IconName::Check, tone: "shipped", label: "Shipped" }
Tag { tone: "accent", variant: TagVariant::Solid, label: "In review" }
Tag { tone: "danger", variant: TagVariant::Outline, label: "Blocked" }"#,
            |context| rsx! {
                div { class: "nd-example-row",
                    Tag {
                        icon: IconName::Check,
                        tone: "shipped",
                        label: context.t(Key::ExampleTagShipped),
                    }
                    Tag {
                        tone: "accent",
                        variant: TagVariant::Solid,
                        label: context.t(Key::ExampleTagReview),
                    }
                    Tag { label: context.t(Key::ExampleTagDraft) }
                    Tag {
                        tone: "danger",
                        variant: TagVariant::Outline,
                        label: context.t(Key::ExampleTagBlocked),
                    }
                }
            }
        ),
    ),
    (
        "tag-picker",
        example!(
            r#"TagPicker { name: "topics", label: "…", placeholder: "…", hint: "…", status, create_label, empty_label, remove_label, added_label, removed_label }
SelectGroup { label: "…", SelectOption { value: "docs", label: "docs" } }"#,
            |context| rsx! {
                div { class: "nd-example-stack nd-example-stack-wide",
                    TagPicker {
                        name: "overview-topics",
                        label: context.t(Key::ExamplePickerLabel),
                        placeholder: context.t(Key::ExamplePickerPlaceholder),
                        hint: context.t(Key::ExamplePickerHint),
                        status: context.t(Key::ExamplePickerStatus),
                        create_label: context.t(Key::ExamplePickerCreate),
                        empty_label: context.t(Key::ExamplePickerEmpty),
                        remove_label: context.t(Key::ExamplePickerRemove),
                        added_label: context.t(Key::ExamplePickerAdded),
                        removed_label: context.t(Key::ExamplePickerRemoved),
                        // A suggestion's `value` is its label, because a reader
                        // can produce the label by typing it and two ways to
                        // make one tag must not submit two different strings.
                        SelectGroup { label: context.t(Key::ExamplePickerWriting),
                            SelectOption { class: "nd-identifier", value: "docs", label: "docs" }
                            SelectOption { class: "nd-identifier", value: "spec", label: "spec" }
                        }
                        SelectGroup { label: context.t(Key::ExamplePickerBuilding),
                            SelectOption { class: "nd-identifier", value: "rust", label: "rust" }
                            SelectOption { class: "nd-identifier", value: "css", label: "css" }
                        }
                    }
                }
            }
        ),
    ),
    (
        "input",
        example!(
            r#"Input { name, label, kind: Email, placeholder: "…", hint, required, required_label, autocomplete }
Input { kind: Password, reveal_label, conceal_label }"#,
            |context| rsx! {
                form { class: "nd-example-form", method: "get", action: "#", novalidate: true,
                    Input {
                        name: "overview-email",
                        label: context.t(Key::ExampleInputEmailLabel),
                        kind: InputKind::Email,
                        placeholder: "ada@example.org",
                        hint: context.t(Key::ExampleInputHint),
                        required: true,
                        required_label: context.t(Key::ExampleInputRequired),
                        autocomplete: "email",
                    }
                    Input {
                        name: "overview-password",
                        label: context.t(Key::ExampleInputPasswordLabel),
                        kind: InputKind::Password,
                        hint: context.t(Key::ExampleInputPasswordHint),
                        required: true,
                        required_label: context.t(Key::ExampleInputRequired),
                        autocomplete: "new-password",
                        reveal_label: context.t(Key::ExampleInputReveal),
                        conceal_label: context.t(Key::ExampleInputConceal),
                    }
                    Button { submit: ButtonSubmit::Submit, label: context.t(Key::ExampleInputSend) }
                }
            }
        ),
    ),
    (
        "textarea",
        example!(
            r#"Textarea { name: "overview-note", label: "…", hint: "…", rows: "3", autosize: true, max_rows: "8", maxlength: "400", counter: true, counter_label: "…" }"#,
            |context| rsx! {
                // WRITE IN IT. Three lines to begin with, growing to eight and
                // then scrolling, and the count says how much room is left —
                // the box doing all three at once is the component.
                Textarea {
                    name: "overview-note",
                    label: context.t(Key::ExampleInputNoteLabel),
                    hint: context.t(Key::ExampleInputNoteHint),
                    rows: "3",
                    autosize: true,
                    max_rows: "8",
                    maxlength: "400",
                    counter: true,
                    counter_label: context.t(Key::ExampleInputCounter),
                }
            }
        ),
    ),
    (
        "toolbar",
        example!(
            r#"Toolbar { label: "…" }
ToolbarGroup { … } / ToolbarGroup { align: ToolbarGroupAlign::End, … }"#,
            |context| rsx! {
                Toolbar { label: context.t(Key::ExampleToolbarLabel),
                    ToolbarGroup {
                        Button {
                            variant: ButtonVariant::Transparent,
                            tone: ButtonTone::Neutral,
                            size: ButtonSize::Small,
                            hide_label: true,
                            icon: IconName::Search,
                            label: context.t(Key::SearchLabel),
                        }
                        Button {
                            variant: ButtonVariant::Transparent,
                            tone: ButtonTone::Neutral,
                            size: ButtonSize::Small,
                            hide_label: true,
                            icon: IconName::Copy,
                            label: context.t(Key::ExampleCopy),
                        }
                    }
                    ToolbarGroup { align: ToolbarGroupAlign::End,
                        Segmented {
                            name: "overview-toolbar-view",
                            size: SegmentedSize::Small,
                            label: context.t(Key::ExampleToolbarView),
                            for (value, label, selected) in [
                                ("light", Key::ThemeModeLight, true),
                                ("dark", Key::ThemeModeDark, false),
                            ] {
                                SegmentedItem { value, label: context.t(label), selected }
                            }
                        }
                        Button {
                            variant: ButtonVariant::Transparent,
                            tone: ButtonTone::Neutral,
                            size: ButtonSize::Small,
                            label: context.t(Key::ExampleReset),
                        }
                    }
                }
            }
        ),
    ),
    (
        "toolbar-group",
        example!(
            r#"Toolbar { label: "…" }
ToolbarGroup { … }
ToolbarGroup { align: ToolbarGroupAlign::End, … }"#,
            |context| rsx! {
                Toolbar { label: context.t(Key::ExampleToolbarLabel),
                    ToolbarGroup {
                        Button {
                            variant: ButtonVariant::Transparent,
                            tone: ButtonTone::Neutral,
                            size: ButtonSize::Small,
                            hide_label: true,
                            icon: IconName::Search,
                            label: context.t(Key::SearchLabel),
                        }
                        Button {
                            variant: ButtonVariant::Transparent,
                            tone: ButtonTone::Neutral,
                            size: ButtonSize::Small,
                            hide_label: true,
                            icon: IconName::Columns,
                            label: context.t(Key::ExampleTableColumns),
                        }
                        Button {
                            variant: ButtonVariant::Transparent,
                            tone: ButtonTone::Neutral,
                            size: ButtonSize::Small,
                            hide_label: true,
                            icon: IconName::Copy,
                            label: context.t(Key::ExampleCopy),
                        }
                    }
                    // NEITHER GROUP IS NAMED, which is the ordinary answer: the
                    // bar carries the name, and two named clusters inside it is
                    // a screen reader saying `group` twice on the way past.
                    ToolbarGroup { align: ToolbarGroupAlign::End,
                        Segmented {
                            name: "overview-toolbar-view",
                            size: SegmentedSize::Small,
                            selection: SegmentedSelection::Quiet,
                            label: context.t(Key::ExampleToolbarView),
                            for (value, label, selected) in [
                                ("light", Key::ThemeModeLight, true),
                                ("dark", Key::ThemeModeDark, false),
                            ] {
                                SegmentedItem { value, label: context.t(label), selected }
                            }
                        }
                        Button {
                            variant: ButtonVariant::Outline,
                            tone: ButtonTone::Neutral,
                            size: ButtonSize::Small,
                            label: context.t(Key::ExampleReset),
                        }
                    }
                }
            }
        ),
    ),
    (
        "tree",
        example!(
            r#"Tree { name, label, guides: true, expand_label, collapse_label,
    TreeItem { label, icon, badge: "3", expanded: true, TreeItem { href, label } }
    TreeItem { label, icon, branch: true, href }
}"#,
            |context| rsx! {
                Tree {
                    name: "overview-pages",
                    label: context.t(Key::ExampleTreeLabel),
                    guides: true,
                    expand_label: context.t(Key::ExampleTreeExpand),
                    collapse_label: context.t(Key::ExampleTreeCollapse),
                    TreeItem {
                        label: context.t(Key::ExampleTreeFoundations),
                        icon: IconName::Palette,
                        badge: "3",
                        expanded: true,
                        TreeItem { href: "../typography/", label: crate::site::TYPOGRAPHY.title(context.locale) }
                        TreeItem { href: "../spacing/", label: crate::site::SPACING.title(context.locale) }
                        TreeItem { href: "../icons/", label: crate::site::ICONS.title(context.locale) }
                    }
                    TreeItem {
                        label: context.t(Key::ExampleTreeControls),
                        icon: IconName::Settings,
                        expanded: true,
                        TreeItem { class: "nd-identifier", href: "../button/", label: "Button" }
                        TreeItem { class: "nd-identifier", href: "../tree/", label: "Tree", current: true }
                    }
                    // Children that were not passed: the lazily-loaded shape.
                    // Nothing is hidden, because nothing is there.
                    TreeItem {
                        label: context.t(Key::ExampleTreeEverything),
                        icon: IconName::Search,
                        branch: true,
                        href: "../search/",
                    }
                }
            }
        ),
    ),
    (
        "truncate",
        example!(
            r#"Truncate { lines: TruncateLines::_2, title: "…", "…" }"#,
            Key::ExampleTruncateWhole,
            |context| rsx! {
                // A COLUMN NARROW ENOUGH THAT THE CLIPPING HAPPENS. This
                // component demonstrates nothing in a column wide enough to
                // hold the whole string.
                div { class: "nd-example-narrow",
                    Truncate {
                        lines: TruncateLines::_2,
                        title: context.t(Key::ExampleListLong),
                        "{context.t(Key::ExampleListLong)}"
                    }
                }
            }
        ),
    ),
    (
        "button-group",
        example!(
            r#"ButtonGroup { label: "Publishing", … }
Button { label: "Publish" }
Button { menu: true, hide_label: true, icon: "chevron-down", controls: "overview-button-group-menu" }"#,
            |context| rsx! {
                div { class: "nd-example-row",
                    ButtonGroup { label: context.t(Key::ExampleGroupSplitLabel),
                        Button { label: context.t(Key::ExampleButtonPublish) }
                        Button {
                            menu: true,
                            hide_label: true,
                            icon: IconName::ChevronDown,
                            controls: "overview-button-group-menu",
                            label: context.t(Key::ExampleButtonMore),
                        }
                    }
                    Menu {
                        name: "overview-button-group-menu",
                        label: context.t(Key::ExampleButtonMore),
                        MenuItem { label: context.t(Key::ExampleOverviewButtonGroupSchedule) }
                        MenuItem { label: context.t(Key::ExampleOverviewButtonGroupDraft) }
                        MenuItem { label: context.t(Key::ExampleMenuArchive) }
                    }
                }
            }
        ),
    ),
    (
        "date-input",
        example!(
            r"DateInput { name, legend, hint, day_label, month_label, year_label, pad: true, required: true, required_label, missing_label, number_label, invalid_label, year_digits_label, error_label }",
            |context| rsx! {
                // `novalidate`, so an empty answer is refused in the
                // component's own sentence rather than the browser's bubble.
                form { class: "nd-example-form", method: "get", action: "#", novalidate: true,
                    DateInput {
                        name: "overview-expires",
                        legend: context.t(Key::ExampleOverviewDateInputLegend),
                        hint: context.t(Key::ExampleOverviewDateInputHint),
                        day_label: context.t(Key::ExampleDateInputDay),
                        month_label: context.t(Key::ExampleDateInputMonth),
                        year_label: context.t(Key::ExampleDateInputYear),
                        pad: true,
                        required: true,
                        error_label: context.t(Key::ExampleDateInputErrorWord),
                        required_label: context.t(Key::ExampleOverviewDateInputRequired),
                        missing_label: context.t(Key::ExampleDateInputMissing),
                        number_label: context.t(Key::ExampleDateInputNumber),
                        invalid_label: context.t(Key::ExampleDateInputReal),
                        year_digits_label: context.t(Key::ExampleDateInputYearDigits),
                    }
                    Button {
                        submit: ButtonSubmit::Submit,
                        label: context.t(Key::ExampleDateInputSend),
                    }
                }
            }
        ),
    ),
    (
        "file-input",
        example!(
            r#"FileInput { name, label, hint, browse_label, drop_label, multiple: true, max: "3", max_label, accept: ".pdf,image/*", accept_label, max_size, max_size_label, remove_label, empty_label, added_label, removed_label, status }"#,
            |context| rsx! {
                form { class: "nd-example-form", method: "get", action: "#", novalidate: true,
                    FileInput {
                        name: "overview-attachments",
                        label: context.t(Key::ExampleInputFilesLabel),
                        hint: context.t(Key::ExampleInputFilesHint),
                        browse_label: context.t(Key::ExampleInputBrowse),
                        drop_label: context.t(Key::ExampleInputDrop),
                        multiple: true,
                        max: "3",
                        max_label: context.t(Key::ExampleInputFileTooMany),
                        accept: ".pdf,image/*",
                        accept_label: context.t(Key::ExampleInputFileType),
                        max_size: "2097152",
                        max_size_label: context.t(Key::ExampleInputFileTooBig),
                        remove_label: context.t(Key::ExampleInputFileRemove),
                        empty_label: context.t(Key::ExampleInputFileEmpty),
                        added_label: context.t(Key::ExampleInputFileAdded),
                        removed_label: context.t(Key::ExampleInputFileRemoved),
                        status: context.t(Key::ExampleInputFileStatus),
                    }
                    Button {
                        submit: ButtonSubmit::Submit,
                        label: context.t(Key::ExampleInputSend),
                    }
                }
            }
        ),
    ),
    (
        "link",
        example!(
            r#"Link { href: "…", external: true, rel: "noreferrer", "…" }
Link { href: "…", "…" }"#,
            |context| rsx! {
                Paragraph {
                    "{context.t(Key::ExampleOverviewLinkLead)} "
                    Link {
                        href: "https://noctua-world.github.io/noctua-colors/",
                        external: true,
                        rel: "noreferrer",
                        "{context.t(Key::ExampleOverviewLinkColors)}"
                    }
                    ", {context.t(Key::ExampleOverviewLinkBetween)} "
                    Link { href: "../spacing/", "{context.t(Key::ExampleOverviewLinkSpacing)}" }
                    "."
                }
            }
        ),
    ),
    (
        "number-input",
        example!(
            r#"NumberInput { name, label, hint, value, min, max, step, big_step, prefix: "R$", group: true, increment_label, decrement_label, max_label }"#,
            |context| rsx! {
                NumberInput {
                    name: "overview-price",
                    label: context.t(Key::ExampleInputPriceLabel),
                    hint: context.t(Key::ExampleOverviewNumberInputHint),
                    value: "1499.9",
                    min: "0",
                    max: "9999.99",
                    step: "0.01",
                    big_step: "10",
                    prefix: "R$",
                    group: true,
                    increment_label: context.t(Key::ExampleOverviewNumberInputUp),
                    decrement_label: context.t(Key::ExampleOverviewNumberInputDown),
                    max_label: context.t(Key::ExampleOverviewNumberInputTooBig),
                }
            }
        ),
    ),
    (
        "radio",
        example!(
            r#"Fieldset { name, legend, hint }
Radio { name: "overview-delivery", value, label, hint, checked, size: Medium }"#,
            |context| rsx! {
                Fieldset {
                    name: "overview-delivery",
                    legend: context.t(Key::ExampleOverviewRadioLegend),
                    hint: context.t(Key::ExampleOverviewRadioHint),
                    Radio {
                        name: "overview-delivery",
                        value: "standard",
                        label: context.t(Key::ExampleOverviewRadioStandard),
                        hint: context.t(Key::ExampleOverviewRadioStandardHint),
                        size: RadioSize::Medium,
                        checked: true,
                    }
                    Radio {
                        name: "overview-delivery",
                        value: "express",
                        label: context.t(Key::ExampleOverviewRadioExpress),
                        hint: context.t(Key::ExampleOverviewRadioExpressHint),
                        size: RadioSize::Medium,
                    }
                    Radio {
                        name: "overview-delivery",
                        value: "collect",
                        label: context.t(Key::ExampleOverviewRadioCollect),
                        size: RadioSize::Medium,
                    }
                }
            }
        ),
    ),
    (
        "select-group",
        example!(
            r#"Select { name, label, hint, placeholder }
SelectGroup { label: "Warm" } / SelectGroup { label: "Cool" }
SelectOption { value, label, swatch }"#,
            |context| rsx! {
                Select {
                    name: "overview-palette",
                    label: context.t(Key::ExampleSelectPaletteLabel),
                    hint: context.t(Key::ExampleSelectPaletteHint),
                    placeholder: context.t(Key::ExampleSelectPalettePlaceholder),
                    SelectGroup { label: context.t(Key::ExampleSelectWarm),
                        SelectOption {
                            class: "nd-identifier",
                            value: "ochre-balanced",
                            label: "Ochre",
                            swatch: "ochre-balanced",
                        }
                        SelectOption {
                            class: "nd-identifier",
                            value: "amber-vivid",
                            label: "Amber",
                            swatch: "amber-vivid",
                        }
                        SelectOption {
                            class: "nd-identifier",
                            value: "clay-sober",
                            label: "Clay",
                            swatch: "clay-sober",
                        }
                    }
                    SelectGroup { label: context.t(Key::ExampleSelectCool),
                        SelectOption {
                            class: "nd-identifier",
                            value: "teal-balanced",
                            label: "Teal",
                            swatch: "teal-balanced",
                        }
                        SelectOption {
                            class: "nd-identifier",
                            value: "azure-vivid",
                            label: "Azure",
                            swatch: "azure-vivid",
                        }
                        SelectOption {
                            class: "nd-identifier",
                            value: "indigo-sober",
                            label: "Indigo",
                            swatch: "indigo-sober",
                        }
                    }
                }
            }
        ),
    ),
    (
        "tables",
        example!(
            r"Table { name, caption, note, sort_ascending_label, sort_descending_label, sort_cleared_label }
TableColumn { name, width, sortable, sort_as }
TableSection { kind: Head } / TableSection / TableSection { kind: Foot }
TableRow / TableHeaderCell { scope: Row, sort } / TableCell { column, label, align: Numeric, sort_value }
TableRow { emphasis: Strong } / TableCell { aggregate: Sum }",
            |context| rsx! {
                Table {
                    name: "overview-invoices",
                    caption: context.t(Key::ExampleTableInvoices),
                    note: context.t(Key::ExampleTableInvoicesNote),
                    sort_ascending_label: context.t(Key::ExampleTableSortAscending),
                    sort_descending_label: context.t(Key::ExampleTableSortDescending),
                    sort_cleared_label: context.t(Key::ExampleTableSortCleared),
                    // THE COLUMNS ARE DECLARED BEFORE ANY ROW, and everything a
                    // column decides is here: how wide it is, whether it sorts,
                    // and what two of its values mean to each other.
                    TableColumn { name: "invoice", width: TableColumnWidth::Fit }
                    TableColumn { name: "client" }
                    TableColumn {
                        name: "due",
                        width: TableColumnWidth::Fit,
                        sortable: true,
                        sort_as: TableColumnSortAs::Date,
                    }
                    TableColumn { name: "status", width: TableColumnWidth::Fit }
                    TableColumn {
                        name: "amount",
                        width: TableColumnWidth::Fit,
                        sortable: true,
                        sort_as: TableColumnSortAs::Number,
                    }
                    TableSection { kind: TableSectionKind::Head,
                        TableRow {
                            TableHeaderCell { column: "invoice", {context.t(Key::ExampleTableColInvoice)} }
                            TableHeaderCell { column: "client", {context.t(Key::ExampleTableColClient)} }
                            // The order the server already applied, drawn
                            // before any script runs.
                            TableHeaderCell {
                                column: "due",
                                sort: TableHeaderCellSort::Ascending,
                                {context.t(Key::ExampleTableColDue)}
                            }
                            TableHeaderCell { column: "status", {context.t(Key::ExampleTableColStatus)} }
                            TableHeaderCell {
                                column: "amount",
                                align: TableHeaderCellAlign::Numeric,
                                {context.t(Key::ExampleTableColAmount)}
                            }
                        }
                    }
                    TableSection {
                        TableRow {
                            TableHeaderCell {
                                scope: TableHeaderCellScope::Row,
                                column: "invoice",
                                class: "nd-identifier",
                                "INV-0043"
                            }
                            // EVERY CELL CARRIES ITS COLUMN'S NAME, for the
                            // stacked layout that has no heading row left.
                            TableCell {
                                column: "client",
                                class: "nd-identifier",
                                label: context.t(Key::ExampleTableColClient),
                                "Aurora Ltd"
                            }
                            TableCell {
                                column: "due",
                                class: "nd-identifier",
                                label: context.t(Key::ExampleTableColDue),
                                sort_value: "2026-03-03",
                                "3 Mar"
                            }
                            TableCell { column: "status", label: context.t(Key::ExampleTableColStatus),
                                Tag {
                                    label: context.t(Key::ExampleTableStatusPaid),
                                    tone: "paid",
                                    size: TagSize::Small,
                                }
                            }
                            TableCell {
                                column: "amount",
                                class: "nd-identifier",
                                label: context.t(Key::ExampleTableColAmount),
                                align: TableCellAlign::Numeric,
                                sort_value: "4900",
                                "R$ 4.900,00"
                            }
                        }
                        TableRow {
                            TableHeaderCell {
                                scope: TableHeaderCellScope::Row,
                                column: "invoice",
                                class: "nd-identifier",
                                "INV-0044"
                            }
                            TableCell {
                                column: "client",
                                class: "nd-identifier",
                                label: context.t(Key::ExampleTableColClient),
                                "Bracken & Co"
                            }
                            TableCell {
                                column: "due",
                                class: "nd-identifier",
                                label: context.t(Key::ExampleTableColDue),
                                sort_value: "2026-03-11",
                                "11 Mar"
                            }
                            TableCell { column: "status", label: context.t(Key::ExampleTableColStatus),
                                Tag {
                                    label: context.t(Key::ExampleTableStatusOverdue),
                                    tone: "overdue",
                                    size: TagSize::Small,
                                }
                            }
                            TableCell {
                                column: "amount",
                                class: "nd-identifier",
                                label: context.t(Key::ExampleTableColAmount),
                                align: TableCellAlign::Numeric,
                                sort_value: "1240",
                                "R$ 1.240,00"
                            }
                        }
                        TableRow {
                            TableHeaderCell {
                                scope: TableHeaderCellScope::Row,
                                column: "invoice",
                                class: "nd-identifier",
                                "INV-0045"
                            }
                            TableCell {
                                column: "client",
                                class: "nd-identifier",
                                label: context.t(Key::ExampleTableColClient),
                                "Cintra Studio"
                            }
                            TableCell {
                                column: "due",
                                class: "nd-identifier",
                                label: context.t(Key::ExampleTableColDue),
                                sort_value: "2026-03-24",
                                "24 Mar"
                            }
                            TableCell { column: "status", label: context.t(Key::ExampleTableColStatus),
                                Tag {
                                    label: context.t(Key::ExampleTableStatusPending),
                                    tone: "pending",
                                    size: TagSize::Small,
                                }
                            }
                            TableCell {
                                column: "amount",
                                class: "nd-identifier",
                                label: context.t(Key::ExampleTableColAmount),
                                align: TableCellAlign::Numeric,
                                sort_value: "680",
                                "R$ 680,00"
                            }
                        }
                        TableRow {
                            TableHeaderCell {
                                scope: TableHeaderCellScope::Row,
                                column: "invoice",
                                class: "nd-identifier",
                                "INV-0046"
                            }
                            TableCell {
                                column: "client",
                                class: "nd-identifier",
                                label: context.t(Key::ExampleTableColClient),
                                "Delgado SA"
                            }
                            TableCell {
                                column: "due",
                                class: "nd-identifier",
                                label: context.t(Key::ExampleTableColDue),
                                sort_value: "2026-03-28",
                                "28 Mar"
                            }
                            TableCell { column: "status", label: context.t(Key::ExampleTableColStatus),
                                Tag {
                                    label: context.t(Key::ExampleTableStatusPaid),
                                    tone: "paid",
                                    size: TagSize::Small,
                                }
                            }
                            TableCell {
                                column: "amount",
                                class: "nd-identifier",
                                label: context.t(Key::ExampleTableColAmount),
                                align: TableCellAlign::Numeric,
                                sort_value: "2310",
                                "R$ 2.310,00"
                            }
                        }
                    }
                    // A `<tfoot>` is what makes a total a total rather than one
                    // more row of data, and it is the only band whose cells may
                    // carry an `aggregate` — recounted from the rows showing.
                    TableSection { kind: TableSectionKind::Foot,
                        TableRow { emphasis: TableRowEmphasis::Strong,
                            TableHeaderCell {
                                scope: TableHeaderCellScope::Row,
                                column: "invoice",
                                span: "4",
                                {context.t(Key::ExampleTableTotal)}
                            }
                            TableCell {
                                column: "amount",
                                class: "nd-identifier",
                                align: TableCellAlign::Numeric,
                                aggregate: TableCellAggregate::Sum,
                                "R$ 9.130,00"
                            }
                        }
                    }
                }
            }
        ),
    ),
    (
        "time-input",
        example!(
            r"TimeInput { name, legend, hint, hour_label, minute_label, meridiem_label, am_label, pm_label, value, min, max, step, pad, now_label, required, required_label, missing_label, invalid_label, min_label, max_label }",
            |context| rsx! {
                form { class: "nd-example-form", method: "get", action: "#", novalidate: true,
                    TimeInput {
                        name: "overview-slot",
                        legend: context.t(Key::ExampleInputTimeLegend),
                        hint: context.t(Key::ExampleInputTimeHint),
                        hour_label: context.t(Key::ExampleInputHour),
                        minute_label: context.t(Key::ExampleInputMinute),
                        meridiem_label: context.t(Key::ExampleInputMeridiem),
                        am_label: context.t(Key::ExampleInputAm),
                        pm_label: context.t(Key::ExampleInputPm),
                        value: "09:30",
                        min: "08:00",
                        max: "18:00",
                        step: "15",
                        pad: true,
                        now_label: context.t(Key::ExampleInputNow),
                        required: true,
                        required_label: context.t(Key::ExampleInputTimeNeeded),
                        missing_label: context.t(Key::ExampleInputTimeMissing),
                        invalid_label: context.t(Key::ExampleInputTimeInvalid),
                        min_label: context.t(Key::ExampleInputTimeEarly),
                        max_label: context.t(Key::ExampleInputTimeLate),
                    }
                    Button {
                        submit: ButtonSubmit::Submit,
                        label: context.t(Key::ExampleInputSend),
                    }
                }
            }
        ),
    ),
    (
        "visually-hidden",
        example!(
            r#"Link { href: "…", "Read more" VisuallyHidden { " — Typography" } }"#,
            Key::ExampleOverviewVisuallyHiddenNote,
            |context| rsx! {
                // THREE LINKS THAT READ THE SAME AND ANNOUNCE DIFFERENTLY. The
                // term beside each one is what a sighted reader has and a list
                // of links, read out of context, does not.
                List { kind: ListKind::Description,
                    ListItem { term: context.t(Key::PageTypographyTitle),
                        Link { href: "../typography/",
                            "{context.t(Key::ExampleOverviewVisuallyHiddenMore)}"
                            VisuallyHidden { " — {context.t(Key::PageTypographyTitle)}" }
                        }
                    }
                    ListItem { term: context.t(Key::PageSpacingTitle),
                        Link { href: "../spacing/",
                            "{context.t(Key::ExampleOverviewVisuallyHiddenMore)}"
                            VisuallyHidden { " — {context.t(Key::PageSpacingTitle)}" }
                        }
                    }
                    ListItem { term: context.t(Key::PageIconsTitle),
                        Link { href: "../icons/",
                            "{context.t(Key::ExampleOverviewVisuallyHiddenMore)}"
                            VisuallyHidden { " — {context.t(Key::PageIconsTitle)}" }
                        }
                    }
                }
            }
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
Divider { label: "or", spacing: Loose }"#,
            |context| rsx! {
                div {
                    Heading { level: HeadingLevel::_3, size: HeadingSize::H4,
                        "{context.t(Key::ExampleOverviewDividerPay)}"
                    }
                    Paragraph { "{context.t(Key::ExampleOverviewDividerPayBody)}" }
                    // A WORD ON THE RULE, because these two are alternatives
                    // rather than one section after another.
                    Divider {
                        label: context.t(Key::ExampleDividerOr),
                        spacing: DividerSpacing::Loose,
                    }
                    Heading { level: HeadingLevel::_3, size: HeadingSize::H4,
                        "{context.t(Key::ExampleOverviewDividerInvoice)}"
                    }
                    Paragraph { "{context.t(Key::ExampleOverviewDividerInvoiceBody)}" }
                }
            }
        ),
    ),
    (
        "label",
        example!(
            r"Label { text, for_control, required, required_label }
Label { kind: Name, for_control } / Segmented { labelledby }",
            |context| rsx! {
                div { class: "nd-example-stack",
                    Label {
                        text: context.t(Key::ExampleOverviewLabelEmail),
                        for_control: "overview-label-email",
                        required: true,
                        required_label: context.t(Key::ExampleInputRequired),
                    }
                    input {
                        class: "nd-input-box",
                        id: "overview-label-email",
                        r#type: "email",
                        name: "overview-label-email",
                        autocomplete: "email",
                        "aria-describedby": "overview-label-email-hint",
                    }
                    p { id: "overview-label-email-hint",
                        Text { size: TextSize::Small, tone: TextTone::Muted,
                            "{context.t(Key::ExampleOverviewLabelHint)}"
                        }
                    }
                    Divider { spacing: DividerSpacing::Tight }
                    // AND THE ONE A `for` CANNOT REACH. A radiogroup is not
                    // labelable, so its name is a `<p>` it points at.
                    Label {
                        kind: LabelKind::Name,
                        text: context.t(Key::ExampleLabelDensity),
                        for_control: "overview-label-density",
                    }
                    Segmented {
                        name: "overview-label-density",
                        labelledby: "overview-label-density-label",
                        for (value, label, selected) in [
                            ("compact", Key::ExampleLabelCompact, false),
                            ("cosy", Key::ExampleLabelCosy, true),
                            ("roomy", Key::ExampleLabelRoomy, false),
                        ] {
                            SegmentedItem { value, label: context.t(label), selected }
                        }
                    }
                }
            }
        ),
    ),
    (
        "image",
        example!(
            r"Image { src, alt, ratio: Landscape, caption, width, height }",
            |context| rsx! {
                div { class: "nd-example-picture",
                    Image {
                        src: format!("{}assets/portraits/gran.svg", crate::site::BASE),
                        alt: context.t(Key::ExampleImageAlt),
                        caption: context.t(Key::ExampleOverviewImageCaption),
                        ratio: ImageRatio::Landscape,
                        width: "480",
                        height: "360",
                    }
                }
            }
        ),
    ),
    (
        "message-bar",
        example!(
            r#"MessageBar { tone: "warning", icon: IconName::Warning, title, dismissible: true }"#,
            |context| rsx! {
                MessageBar {
                    tone: "warning",
                    icon: IconName::Warning,
                    title: context.t(Key::ExampleOverviewMessageBarTitle),
                    dismissible: true,
                    dismiss_label: context.t(Key::ExampleMessageBarDismiss),
                    "{context.t(Key::ExampleOverviewMessageBarBody)}"
                }
            }
        ),
    ),
    (
        "dialog",
        example!(
            r"Dialog { name, trigger_label, heading, description, close_label, … }",
            |context| rsx! {
                Dialog {
                    name: "overview-dialog",
                    trigger_label: context.t(Key::ExampleOverviewDialogOpen),
                    heading: context.t(Key::ExampleOverviewDialogHeading),
                    description: context.t(Key::ExampleOverviewDialogBody),
                    close_label: context.t(Key::ExampleDialogClose),
                    Paragraph { "{context.t(Key::ExampleOverviewDialogBody)}" }
                    div { class: "nd-example-row",
                        Button {
                            variant: ButtonVariant::Outline,
                            tone: ButtonTone::Neutral,
                            label: context.t(Key::ExampleDialogCancel),
                        }
                        Button {
                            variant: ButtonVariant::Solid,
                            tone: ButtonTone::Accent,
                            label: context.t(Key::ExampleOverviewDialogConfirm),
                        }
                    }
                }
            }
        ),
    ),
    (
        "popover",
        example!(
            r"Popover { name, trigger_label, heading, close_label, … Button { … } }",
            |context| rsx! {
                Popover {
                    name: "overview-popover",
                    trigger_label: context.t(Key::ExamplePopoverShareOpen),
                    heading: context.t(Key::ExamplePopoverShareHeading),
                    close_label: context.t(Key::ExamplePopoverClose),
                    Paragraph { "{context.t(Key::ExamplePopoverShareBody)}" }
                    Button {
                        variant: ButtonVariant::Outline,
                        tone: ButtonTone::Neutral,
                        icon: IconName::Copy,
                        label: context.t(Key::ExamplePopoverCopy),
                    }
                }
            }
        ),
    ),
    (
        "tooltip",
        example!(
            r#"Tooltip { name, text: "…", Button { icon, label } }"#,
            |context| rsx! {
                div { class: "nd-example-row",
                    Tooltip {
                        name: "overview-tooltip-copy",
                        text: context.t(Key::ExampleTooltipCopyHint),
                        Button {
                            variant: ButtonVariant::Outline,
                            tone: ButtonTone::Neutral,
                            icon: IconName::Copy,
                            label: context.t(Key::ExampleTooltipCopy),
                        }
                    }
                    Tooltip {
                        name: "overview-tooltip-export",
                        text: context.t(Key::ExampleTooltipExportHint),
                        Button {
                            variant: ButtonVariant::Outline,
                            tone: ButtonTone::Neutral,
                            icon: IconName::Download,
                            label: context.t(Key::ExampleTooltipExport),
                        }
                    }
                }
            }
        ),
    ),
    (
        "toast",
        example!(
            r#"Toast { tone: "success", title, duration: "6000", dismiss_label }"#,
            |context| rsx! {
                div { class: "nd-example-toasts",
                    Toast {
                        name: "overview-toast",
                        tone: "success",
                        duration: "6000",
                        title: context.t(Key::ExampleOverviewToastTitle),
                        dismiss_label: context.t(Key::ExampleToastDismiss),
                        "{context.t(Key::ExampleOverviewToastBody)}"
                    }
                }
            }
        ),
    ),
    (
        "icon",
        example!(
            r#"Icon { icon: "check", label: "…" } / Icon { icon: "star", size: Large }"#,
            |context| rsx! {
                div { class: "nd-example-row",
                    // Named, because the drawing is the whole message here.
                    Icon { icon: IconName::Check, label: context.t(Key::ExampleOverviewIconStatus) }
                    Text { "{context.t(Key::ExampleOverviewIconStatus)}" }
                    Icon { icon: IconName::Star, size: IconSize::Small }
                    Icon { icon: IconName::Star }
                    Icon { icon: IconName::Star, size: IconSize::Large }
                }
            }
        ),
    ),
    (
        "rating",
        example!(
            r"Rating { readonly: true, value, max, value_label } / Rating { item_label, … }",
            |context| rsx! {
                div { class: "nd-example-stack",
                    Rating {
                        name: "overview-rating-average",
                        label: context.t(Key::ExampleOverviewRatingAverage),
                        readonly: true,
                        value: "4.3",
                        max: "5",
                        value_label: context.t(Key::ExampleRatingValue),
                    }
                    Rating {
                        name: "overview-rating-stay",
                        label: context.t(Key::ExampleOverviewRatingLabel),
                        max: "5",
                        value_label: context.t(Key::ExampleRatingValue),
                        item_label: context.t(Key::ExampleRatingItem),
                        hint: context.t(Key::ExampleRatingHint),
                    }
                }
            }
        ),
    ),
    (
        "otp-field",
        example!(
            r"OtpField { name, legend, box_label, length, hint, invalid_label }",
            |context| rsx! {
                OtpField {
                    name: "overview-otp",
                    legend: context.t(Key::ExampleOverviewOtpFieldLegend),
                    box_label: context.t(Key::ExampleOtpFieldBox),
                    length: "6",
                    hint: context.t(Key::ExampleOverviewOtpFieldHint),
                    invalid_label: context.t(Key::ExampleOtpFieldInvalid),
                }
            }
        ),
    ),
    (
        "tabs",
        example!(
            r"Tabs { name, label } + Tab { value, label, count } + TabPanel { value, … }",
            |context| rsx! {
                div { class: "nd-example-stack",
                    Tabs {
                        name: "overview-tabs",
                        label: context.t(Key::ExampleOverviewTabsLabel),
                        Tab {
                            name: "overview-tabs",
                            value: "items",
                            label: context.t(Key::ExampleTabsItems),
                            count: "3",
                            selected: true,
                        }
                        Tab {
                            name: "overview-tabs",
                            value: "delivery",
                            label: context.t(Key::ExampleTabsDelivery),
                        }
                        Tab {
                            name: "overview-tabs",
                            value: "payment",
                            label: context.t(Key::ExampleTabsPayment),
                        }
                    }
                    TabPanel { name: "overview-tabs", value: "items",
                        Paragraph { "{context.t(Key::ExampleTabsItemsBody)}" }
                    }
                    TabPanel { name: "overview-tabs", value: "delivery",
                        Paragraph { "{context.t(Key::ExampleTabsDeliveryBody)}" }
                    }
                    TabPanel { name: "overview-tabs", value: "payment",
                        Paragraph { "{context.t(Key::ExampleTabsPaymentBody)}" }
                    }
                }
            }
        ),
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
    example!(
        r#"Icon { icon: "check" }"#,
        Key::ExampleIconNoteHidden,
        |context| rsx! {
            span { class: "nd-example-row",
                Icon { icon: IconName::Check }
                Text { "{context.t(Key::ExampleIconSaved)}" }
            }
        }
    ),
    // THE ONE THAT NEEDS A NAME. Nothing beside it says what it means, so the
    // drawing becomes a `role="img"` carrying the caller's words.
    example!(
        r#"Icon { icon: "star", label: "…" }"#,
        Key::ExampleIconNoteNamed,
        |context| rsx! {
            // NO `size: Medium`. It is the default, so the example that
            // teaches the accessible name also carried a prop doing nothing.
            // The three sizes are the example below.
            Icon {
                icon: IconName::Star,
                label: context.t(Key::ExampleIconFavourite),
            }
        }
    ),
    example!(
        r"Icon { size: Small | Medium | Large }",
        Key::ExampleIconNoteSize,
        |_| rsx! {
            span { class: "nd-example-row",
                Icon { icon: IconName::Palette, size: IconSize::Small }
                Icon { icon: IconName::Palette, size: IconSize::Medium }
                Icon { icon: IconName::Palette, size: IconSize::Large }
            }
        }
    ),
    // The box is `1em`, which is the whole reason one class covers every
    // context: a heading and a caption size their own icon.
    example!(
        r"Icon { … } inside a Heading and inside a caption",
        Key::ExampleIconNoteInherit,
        |context| rsx! {
            div {
                Heading { level: HeadingLevel::_3, size: HeadingSize::H4,
                    Icon { icon: IconName::Info }
                    " {context.t(Key::ExampleIconHeading)}"
                }
                Text { size: TextSize::Small, tone: TextTone::Muted,
                    Icon { icon: IconName::Clock }
                    " {context.t(Key::ExampleIconCaption)}"
                }
            }
        }
    ),
];

// --- Rating -----------------------------------------------------------------

const RATING: &[Example] = &[
    // A READING, AND THE NUMBER IS WRITTEN OUT. The fifth star is filled three
    // tenths of the way, because rounding it would be a picture that disagrees
    // with the sentence beside it.
    example!(
        r#"Rating { name, label, readonly: true, value: "4.3", max: "5", value_label }"#,
        Key::ExampleRatingNoteReading,
        |context| rsx! {
            Rating {
                name: "rating-read",
                label: context.t(Key::ExampleRatingProduct),
                readonly: true,
                value: "4.3",
                max: "5",
                value_label: context.t(Key::ExampleRatingValue),
            }
        }
    ),
    // THE QUESTION. Real radios in a real fieldset, so it submits with no
    // script, and the number follows the reader's own choice.
    example!(
        r"Rating { item_label, value_label, hint, required: true, required_label }",
        Key::ExampleRatingNoteAsk,
        |context| rsx! {
            Rating {
                name: "rating-ask",
                label: context.t(Key::ExampleRatingDelivery),
                value_label: context.t(Key::ExampleRatingValue),
                item_label: context.t(Key::ExampleRatingItem),
                hint: context.t(Key::ExampleRatingHint),
                required: true,
                required_label: context.t(Key::ExampleRatingRequired),
            }
        }
    ),
    example!(
        r#"Rating { name, label, readonly: true, value: "4", tone: "warning", size: Small | Medium | Large, value_label }"#,
        Key::ExampleRatingNoteSize,
        |context| rsx! {
            div { class: "nd-example-stack",
                for (slug, size) in [
                    ("small", RatingSize::Small),
                    ("medium", RatingSize::Medium),
                    ("large", RatingSize::Large),
                ] {
                    Rating {
                        key: "{slug}",
                        name: "rating-size-{slug}",
                        label: context.t(Key::ExampleRatingProduct),
                        readonly: true,
                        value: "4",
                        tone: "warning",
                        size,
                        value_label: context.t(Key::ExampleRatingValue),
                    }
                }
            }
        }
    ),
    // A REFUSED ANSWER MARKS EVERY STAR, because the score is one value. And a
    // rating nobody may change any more takes no focus at all.
    example!(
        r"Rating { name, label, value_label, item_label, error }
Rating { name, label, value, value_label, item_label, hint, disabled: true }",
        Key::ExampleRatingNoteRefused,
        |context| rsx! {
            div { class: "nd-example-stack",
                Rating {
                    name: "rating-error",
                    label: context.t(Key::ExampleRatingDelivery),
                    value_label: context.t(Key::ExampleRatingValue),
                    item_label: context.t(Key::ExampleRatingItem),
                    error: context.t(Key::ExampleRatingError),
                }
                Rating {
                    name: "rating-off",
                    label: context.t(Key::ExampleRatingDelivery),
                    value: "2",
                    value_label: context.t(Key::ExampleRatingValue),
                    item_label: context.t(Key::ExampleRatingItem),
                    hint: context.t(Key::ExampleRatingClosed),
                    disabled: true,
                }
            }
        }
    ),
];

// --- OtpField ---------------------------------------------------------------

const OTP_FIELD: &[Example] = &[
    // SIX BOXES, ONE VALUE. Every box posts under the same name, so the code
    // reaches a server whether or not the script arrived.
    example!(
        r"OtpField { name, legend, box_label, hint, invalid_label, required, required_label }",
        Key::ExampleOtpFieldNoteCode,
        |context| rsx! {
            OtpField {
                name: "otp-code",
                legend: context.t(Key::ExampleOtpFieldLegend),
                box_label: context.t(Key::ExampleOtpFieldBox),
                hint: context.t(Key::ExampleOtpFieldHint),
                invalid_label: context.t(Key::ExampleOtpFieldInvalid),
                required: true,
                required_label: context.t(Key::ExampleOtpFieldRequired),
            }
        }
    ),
    example!(
        r#"OtpField { kind: Alphanumeric, length: "4", value: "AB12" }"#,
        Key::ExampleOtpFieldNoteLetters,
        |context| rsx! {
            OtpField {
                name: "otp-invite",
                legend: context.t(Key::ExampleOtpFieldLetters),
                box_label: context.t(Key::ExampleOtpFieldLettersBox),
                kind: OtpFieldKind::Alphanumeric,
                length: "4",
                value: "AB12",
            }
        }
    ),
    example!(
        r"OtpField { size: Small | Medium | Large }",
        Key::ExampleOtpFieldNoteSize,
        |context| rsx! {
            div { class: "nd-example-stack",
                for (slug, size) in [
                    ("small", OtpFieldSize::Small),
                    ("medium", OtpFieldSize::Medium),
                    ("large", OtpFieldSize::Large),
                ] {
                    OtpField {
                        key: "{slug}",
                        name: "otp-size-{slug}",
                        legend: context.t(Key::ExampleOtpFieldLegend),
                        box_label: context.t(Key::ExampleOtpFieldBox),
                        length: "4",
                        size,
                    }
                }
            }
        }
    ),
    example!(
        r"OtpField { name, legend, box_label, length, value, error }
OtpField { name, legend, box_label, length, autocomplete: false, disabled: true }",
        Key::ExampleOtpFieldNoteRefused,
        |context| rsx! {
            div { class: "nd-example-stack",
                OtpField {
                    name: "otp-error",
                    legend: context.t(Key::ExampleOtpFieldLegend),
                    box_label: context.t(Key::ExampleOtpFieldBox),
                    length: "4",
                    value: "4213",
                    error: context.t(Key::ExampleOtpFieldError),
                }
                OtpField {
                    name: "otp-off",
                    legend: context.t(Key::ExampleOtpFieldLegend),
                    box_label: context.t(Key::ExampleOtpFieldBox),
                    length: "4",
                    autocomplete: false,
                    disabled: true,
                }
            }
        }
    ),
];

/// One strip and its panels, built once so four examples differ by one prop.
///
/// A function rather than four copies: what each example is about is the prop
/// it passes, and a reader comparing them should not have to diff three tabs
/// and three panels to find it.
fn tab_set(context: Context, name: &'static str, strip: Element, panels: [Key; 3]) -> Element {
    let [items, delivery, payment] = panels;
    rsx! {
        div { class: "nd-example-stack",
            {strip}
            TabPanel { name, value: "items", Paragraph { "{context.t(items)}" } }
            TabPanel { name, value: "delivery", Paragraph { "{context.t(delivery)}" } }
            TabPanel { name, value: "payment", Paragraph { "{context.t(payment)}" } }
        }
    }
}

// --- Tabs -------------------------------------------------------------------

const TABS: &[Example] = &[
    // ONE STRIP, THREE PANELS. With no script every panel is on the page and
    // each tab is a link down to the one it names.
    example!(
        r"Tabs { name, label } / Tab { selected: true } / TabPanel { … }",
        Key::ExampleTabsNoteStrip,
        |context| tab_set(
            context,
            "tabs-plain",
            rsx! {
                Tabs { name: "tabs-plain", label: context.t(Key::ExampleTabsLabel),
                    Tab {
                        name: "tabs-plain",
                        value: "items",
                        label: context.t(Key::ExampleTabsItems),
                        selected: true,
                    }
                    Tab {
                        name: "tabs-plain",
                        value: "delivery",
                        label: context.t(Key::ExampleTabsDelivery),
                    }
                    Tab {
                        name: "tabs-plain",
                        value: "payment",
                        label: context.t(Key::ExampleTabsPayment),
                    }
                }
            },
            [
                Key::ExampleTabsItemsBody,
                Key::ExampleTabsDeliveryBody,
                Key::ExampleTabsPaymentBody,
            ],
        )
    ),
    // MANUAL ACTIVATION, which is the one place a tab differs from a radio:
    // the arrows move the focus and wait to be told.
    example!(
        r"Tabs { activation: Manual, appearance: Enclosed, labelledby }",
        Key::ExampleTabsNoteManual,
        |context| rsx! {
            div { class: "nd-example-stack",
                Label {
                    kind: LabelKind::Name,
                    text: context.t(Key::ExampleTabsLabel),
                    for_control: "tabs-manual",
                }
                {tab_set(
                    context,
                    "tabs-manual",
                    rsx! {
                        Tabs {
                            name: "tabs-manual",
                            labelledby: "tabs-manual-label",
                            activation: TabsActivation::Manual,
                            appearance: TabsAppearance::Enclosed,
                            Tab {
                                name: "tabs-manual",
                                value: "items",
                                label: context.t(Key::ExampleTabsItems),
                                selected: true,
                            }
                            Tab {
                                name: "tabs-manual",
                                value: "delivery",
                                label: context.t(Key::ExampleTabsDelivery),
                            }
                            Tab {
                                name: "tabs-manual",
                                value: "payment",
                                label: context.t(Key::ExampleTabsPayment),
                            }
                        }
                    },
                    [
                        Key::ExampleTabsItemsBody,
                        Key::ExampleTabsDeliveryBody,
                        Key::ExampleTabsPaymentBody,
                    ],
                )}
            }
        }
    ),
    example!(
        r"Tabs { orientation: Vertical, size: Small }",
        Key::ExampleTabsNoteVertical,
        |context| tab_set(
            context,
            "tabs-upright",
            rsx! {
                Tabs {
                    name: "tabs-upright",
                    label: context.t(Key::ExampleTabsLabel),
                    orientation: TabsOrientation::Vertical,
                    size: TabsSize::Small,
                    Tab {
                        name: "tabs-upright",
                        value: "items",
                        label: context.t(Key::ExampleTabsItems),
                        selected: true,
                    }
                    Tab {
                        name: "tabs-upright",
                        value: "delivery",
                        label: context.t(Key::ExampleTabsDelivery),
                    }
                    Tab {
                        name: "tabs-upright",
                        value: "payment",
                        label: context.t(Key::ExampleTabsPayment),
                    }
                }
            },
            [
                Key::ExampleTabsItemsBody,
                Key::ExampleTabsDeliveryBody,
                Key::ExampleTabsPaymentBody,
            ],
        )
    ),
    example!(
        r"Tabs { fill: Equal }",
        Key::ExampleTabsNoteFill,
        |context| tab_set(
            context,
            "tabs-equal",
            rsx! {
                Tabs {
                    name: "tabs-equal",
                    label: context.t(Key::ExampleTabsLabel),
                    fill: TabsFill::Equal,
                    Tab {
                        name: "tabs-equal",
                        value: "items",
                        label: context.t(Key::ExampleTabsItems),
                        selected: true,
                    }
                    Tab {
                        name: "tabs-equal",
                        value: "delivery",
                        label: context.t(Key::ExampleTabsDelivery),
                    }
                    Tab {
                        name: "tabs-equal",
                        value: "payment",
                        label: context.t(Key::ExampleTabsPayment),
                    }
                }
            },
            [
                Key::ExampleTabsItemsBody,
                Key::ExampleTabsDeliveryBody,
                Key::ExampleTabsPaymentBody,
            ],
        )
    ),
];

// --- Tab --------------------------------------------------------------------

const TAB: &[Example] = &[
    example!(
        r"Tab { name, value, label, selected: true }",
        Key::ExampleTabNoteLabel,
        |context| tab_set(
            context,
            "tab-plain",
            rsx! {
                Tabs { name: "tab-plain", label: context.t(Key::ExampleTabsLabel),
                    Tab {
                        name: "tab-plain",
                        value: "items",
                        label: context.t(Key::ExampleTabSettings),
                        selected: true,
                    }
                    Tab {
                        name: "tab-plain",
                        value: "delivery",
                        label: context.t(Key::ExampleTabPeople),
                    }
                    Tab {
                        name: "tab-plain",
                        value: "payment",
                        label: context.t(Key::ExampleTabBilling),
                    }
                }
            },
            [
                Key::ExampleTabSettingsBody,
                Key::ExampleTabPeopleBody,
                Key::ExampleTabBillingBody,
            ],
        )
    ),
    example!(
        r#"Tab { icon: "settings" }"#,
        Key::ExampleTabNoteIcon,
        |context| tab_set(
            context,
            "tab-icon",
            rsx! {
                Tabs { name: "tab-icon", label: context.t(Key::ExampleTabsLabel),
                    Tab {
                        name: "tab-icon",
                        value: "items",
                        label: context.t(Key::ExampleTabSettings),
                        icon: IconName::Settings,
                        selected: true,
                    }
                    Tab {
                        name: "tab-icon",
                        value: "delivery",
                        label: context.t(Key::ExampleTabPeople),
                        icon: IconName::User,
                    }
                    Tab {
                        name: "tab-icon",
                        value: "payment",
                        label: context.t(Key::ExampleTabBilling),
                        icon: IconName::Download,
                    }
                }
            },
            [
                Key::ExampleTabSettingsBody,
                Key::ExampleTabPeopleBody,
                Key::ExampleTabBillingBody,
            ],
        )
    ),
    example!(
        r#"Tab { count: "4" }"#,
        Key::ExampleTabNoteCount,
        |context| tab_set(
            context,
            "tab-count",
            rsx! {
                Tabs { name: "tab-count", label: context.t(Key::ExampleTabsLabel),
                    Tab {
                        name: "tab-count",
                        value: "items",
                        label: context.t(Key::ExampleTabSettings),
                        selected: true,
                    }
                    Tab {
                        name: "tab-count",
                        value: "delivery",
                        label: context.t(Key::ExampleTabPeople),
                        count: "4",
                    }
                    Tab {
                        name: "tab-count",
                        value: "payment",
                        label: context.t(Key::ExampleTabBilling),
                        count: "0",
                    }
                }
            },
            [
                Key::ExampleTabSettingsBody,
                Key::ExampleTabPeopleBody,
                Key::ExampleTabBillingBody,
            ],
        )
    ),
    example!(
        r"Tab { disabled: true }",
        Key::ExampleTabNoteDisabled,
        |context| tab_set(
            context,
            "tab-off",
            rsx! {
                Tabs { name: "tab-off", label: context.t(Key::ExampleTabsLabel),
                    Tab {
                        name: "tab-off",
                        value: "items",
                        label: context.t(Key::ExampleTabSettings),
                        selected: true,
                    }
                    Tab {
                        name: "tab-off",
                        value: "delivery",
                        label: context.t(Key::ExampleTabPeople),
                    }
                    Tab {
                        name: "tab-off",
                        value: "payment",
                        label: context.t(Key::ExampleTabBilling),
                        disabled: true,
                    }
                }
            },
            [
                Key::ExampleTabSettingsBody,
                Key::ExampleTabPeopleBody,
                Key::ExampleTabBillingBody,
            ],
        )
    ),
];

// --- TabPanel ---------------------------------------------------------------

const TAB_PANEL: &[Example] = &[
    example!(
        r"TabPanel { name, value, … } after the strip",
        Key::ExampleTabPanelNoteAfter,
        |context| rsx! {
            div { class: "nd-example-stack",
                Tabs { name: "panel-plain", label: context.t(Key::ExampleTabsLabel),
                    Tab {
                        name: "panel-plain",
                        value: "summary",
                        label: context.t(Key::ExampleTabPanelSummary),
                        selected: true,
                    }
                    Tab {
                        name: "panel-plain",
                        value: "detail",
                        label: context.t(Key::ExampleTabPanelDetail),
                    }
                }
                TabPanel { name: "panel-plain", value: "summary",
                    Paragraph { "{context.t(Key::ExampleTabPanelSummaryBody)}" }
                }
                TabPanel { name: "panel-plain", value: "detail",
                    Paragraph { "{context.t(Key::ExampleTabPanelDetailBody)}" }
                }
            }
        }
    ),
    example!(
        r"TabPanel { … } holding more than a sentence",
        Key::ExampleTabPanelNoteRich,
        |context| rsx! {
            div { class: "nd-example-stack",
                Tabs { name: "panel-rich", label: context.t(Key::ExampleTabsLabel),
                    Tab {
                        name: "panel-rich",
                        value: "summary",
                        label: context.t(Key::ExampleTabPanelSummary),
                        selected: true,
                    }
                    Tab {
                        name: "panel-rich",
                        value: "detail",
                        label: context.t(Key::ExampleTabPanelDetail),
                    }
                }
                TabPanel { name: "panel-rich", value: "summary",
                    List {
                        ListItem { "{context.t(Key::ExampleTabsItemsBody)}" }
                        ListItem { "{context.t(Key::ExampleTabsDeliveryBody)}" }
                        ListItem { "{context.t(Key::ExampleTabsPaymentBody)}" }
                    }
                }
                TabPanel { name: "panel-rich", value: "detail",
                    Paragraph { "{context.t(Key::ExampleTabPanelDetailBody)}" }
                }
            }
        }
    ),
    example!(
        // THE PANEL SHOWN IS THE ONE THE `value` NAMES, not the first one
        // written. This example used to be example 1 again with a different
        // `name`, under a caption claiming every panel is visible with no
        // script — which the rendered page, where the script runs, disproves
        // four inches below the sentence.
        r#"Tab { value: "delivery", selected: true } / TabPanel { name, value } three times"#,
        Key::ExampleTabPanelNoteChosen,
        |context| rsx! {
            div { class: "nd-example-stack",
                Tabs { name: "panel-chosen", label: context.t(Key::ExampleTabsLabel),
                    for (value, label, selected) in [
                        ("items", Key::ExampleTabsItems, false),
                        ("delivery", Key::ExampleTabsDelivery, true),
                        ("payment", Key::ExampleTabsPayment, false),
                    ] {
                        Tab { key: "{value}", name: "panel-chosen", value, label: context.t(label), selected }
                    }
                }
                for (value, body) in [
                    ("items", Key::ExampleTabsItemsBody),
                    ("delivery", Key::ExampleTabsDeliveryBody),
                    ("payment", Key::ExampleTabsPaymentBody),
                ] {
                    TabPanel { key: "{value}", name: "panel-chosen", value,
                        Paragraph { "{context.t(body)}" }
                    }
                }
            }
        }
    ),
    // A SET INSIDE A PANEL, which works because both ids come from a `name`
    // and this one differs from the set around it.
    example!(
        r"TabPanel { … Tabs { name: … } }",
        Key::ExampleTabPanelNoteNested,
        |context| rsx! {
            div { class: "nd-example-stack",
                Tabs { name: "panel-outer", label: context.t(Key::ExampleTabsLabel),
                    Tab {
                        name: "panel-outer",
                        value: "summary",
                        label: context.t(Key::ExampleTabPanelSummary),
                        selected: true,
                    }
                }
                TabPanel { name: "panel-outer", value: "summary",
                    Tabs { name: "panel-inner", label: context.t(Key::ExampleTabsLabel),
                        Tab {
                            name: "panel-inner",
                            value: "items",
                            label: context.t(Key::ExampleTabsItems),
                            selected: true,
                        }
                        Tab {
                            name: "panel-inner",
                            value: "delivery",
                            label: context.t(Key::ExampleTabsDelivery),
                        }
                    }
                    TabPanel { name: "panel-inner", value: "items",
                        Paragraph { "{context.t(Key::ExampleTabsItemsBody)}" }
                    }
                    TabPanel { name: "panel-inner", value: "delivery",
                        Paragraph { "{context.t(Key::ExampleTabsDeliveryBody)}" }
                    }
                }
            }
        }
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
        Key::ExampleDividerNoteNamed,
        |context| rsx! {
            div {
                Heading { level: HeadingLevel::_3, size: HeadingSize::H4,
                    "{context.t(Key::ExampleDividerAppearance)}"
                }
                Paragraph { "{context.t(Key::ExampleDividerAppearanceBody)}" }
                Divider { spacing: DividerSpacing::Loose }
                Heading { level: HeadingLevel::_3, size: HeadingSize::H4,
                    "{context.t(Key::ExampleDividerMotion)}"
                }
                Paragraph { "{context.t(Key::ExampleDividerMotionBody)}" }
            }
        }
    ),
    // A WORD ON THE RULE, which is a different element and a different
    // accessible name. `or` between two ways of doing the same thing.
    example!(
        r#"Divider { label: "or" }"#,
        Key::ExampleDividerNoteLabel,
        |context| rsx! {
            div { class: "nd-example-stack",
                Button {
                    variant: ButtonVariant::Solid,
                    tone: ButtonTone::Accent,
                    label: context.t(Key::ExampleDividerPassword),
                }
                Divider { label: context.t(Key::ExampleDividerOr) }
                Button {
                    variant: ButtonVariant::Outline,
                    tone: ButtonTone::Neutral,
                    label: context.t(Key::ExampleDividerLink),
                }
            }
        }
    ),
    // SIDEWAYS, between two groups sharing a line. Its height comes from the
    // row, so it is right whatever is standing beside it.
    example!(
        r"Divider { orientation: Vertical, weight: Strong }",
        Key::ExampleDividerNoteVertical,
        |context| rsx! {
            div { class: "nd-example-row",
                Button {
                    variant: ButtonVariant::Outline,
                    tone: ButtonTone::Neutral,
                    icon: IconName::Copy,
                    label: context.t(Key::ExampleCopy),
                }
                Divider {
                    orientation: DividerOrientation::Vertical,
                    weight: DividerWeight::Strong,
                }
                Button {
                    variant: ButtonVariant::Outline,
                    tone: ButtonTone::Neutral,
                    label: context.t(Key::ExampleButtonSave),
                }
                Button {
                    variant: ButtonVariant::Outline,
                    tone: ButtonTone::Neutral,
                    label: context.t(Key::ExampleButtonPublish),
                }
            }
        }
    ),
    // THE THREE THICKNESSES, ITERATED RATHER THAN LISTED, so this cannot fall
    // out of step with the enum the day a fourth border token arrives.
    example!(
        r"Divider { weight, spacing: Tight } for weight in DividerWeight::ALL",
        Key::ExampleDividerNoteWeight,
        |_| rsx! {
            div {
                for weight in DividerWeight::ALL {
                    Divider { weight, spacing: DividerSpacing::Tight }
                }
            }
        }
    ),
    // OUT OF THE TREE ALTOGETHER. The heading above it already draws the
    // boundary, so the rule repeats it for the eye and says nothing twice.
    example!(
        r"Divider { decorative: true, spacing: Tight }",
        Key::ExampleDividerNoteDecorative,
        |context| rsx! {
            div {
                Heading { level: HeadingLevel::_3, size: HeadingSize::H4,
                    "{context.t(Key::ExampleDividerMotion)}"
                }
                Divider { decorative: true, spacing: DividerSpacing::Tight }
                Paragraph { "{context.t(Key::ExampleDividerMotionBody)}" }
            }
        }
    ),
];

// --- Label ------------------------------------------------------------------

const LABEL: &[Example] = &[
    // A LABEL NAMING A CONTROL IT DOES NOT DRAW. The `<input>` here is a bare
    // one on purpose: `Field` and `Input` already compose this component, and
    // an example of them would be an example of them.
    example!(
        r#"Label { text, for_control: "…" }"#,
        Key::ExampleLabelNoteFor,
        |context| rsx! {
            div { class: "nd-example-stack",
                Label {
                    text: context.t(Key::ExampleInputEmailLabel),
                    for_control: "example-label-email",
                }
                input {
                    class: "nd-input-box",
                    id: "example-label-email",
                    r#type: "email",
                    name: "example-label-email",
                    autocomplete: "email",
                }
            }
        }
    ),
    // A GROUP'S NAME HAS TO BE A `<legend>`. Nothing else attaches a question
    // to every control under it, in every engine, with nothing to wire.
    example!(
        r"Label { kind: Legend, text, required, required_label }",
        Key::ExampleLabelNoteLegend,
        |context| rsx! {
            fieldset { class: "nd-field",
                Label {
                    kind: LabelKind::Legend,
                    text: context.t(Key::ExampleLabelNewsletter),
                    required: true,
                    required_label: context.t(Key::ExampleInputRequired),
                }
                Checkbox {
                    name: "example-label-send",
                    value: "offers",
                    label: context.t(Key::ExampleLabelOffers),
                }
                Checkbox {
                    name: "example-label-send",
                    value: "updates",
                    label: context.t(Key::ExampleLabelUpdates),
                }
            }
        }
    ),
    // AND A GROUP THAT IS NOT A `<fieldset>` IS NAMED BY REFERENCE. A
    // `role="radiogroup"` is not labelable, so a `for` aimed at one names
    // nothing whatsoever.
    example!(
        r#"Label { kind: Name, for_control: "…" } / Segmented { labelledby }"#,
        Key::ExampleLabelNoteName,
        |context| rsx! {
            div { class: "nd-example-stack",
                Label {
                    kind: LabelKind::Name,
                    text: context.t(Key::ExampleLabelDensity),
                    for_control: "example-label-density",
                }
                Segmented {
                    name: "example-label-density",
                    labelledby: "example-label-density-label",
                    for (value, label, selected) in [
                        ("compact", Key::ExampleLabelCompact, false),
                        ("cosy", Key::ExampleLabelCosy, true),
                        ("roomy", Key::ExampleLabelRoomy, false),
                    ] {
                        SegmentedItem { value, label: context.t(label), selected }
                    }
                }
            }
        }
    ),
    // ONE WORD OR THE OTHER, NEVER BOTH AND NEVER AN ASTERISK.
    example!(
        r"Label { required, required_label } / Label { optional_label }",
        Key::ExampleLabelNoteMark,
        |context| rsx! {
            div { class: "nd-example-stack",
                Label {
                    text: context.t(Key::ExampleInputEmailLabel),
                    for_control: "example-label-required",
                    required: true,
                    required_label: context.t(Key::ExampleInputRequired),
                }
                Label {
                    text: context.t(Key::ExampleInputPhoneLabel),
                    for_control: "example-label-optional",
                    optional_label: context.t(Key::ExampleLabelOptional),
                }
            }
        }
    ),
    // THE THREE SIZES, ITERATED, for the reason `Input` states: this cannot
    // fall out of step with the enum the day a fourth is added.
    example!(
        r"Label { size } for size in LabelSize::ALL",
        Key::ExampleLabelNoteSize,
        |context| rsx! {
            div { class: "nd-example-stack",
                for size in LabelSize::ALL {
                    Label {
                        size,
                        text: context.t(Key::ExampleInputNameLabel),
                        for_control: "example-label-{size.as_str()}",
                    }
                }
            }
        }
    ),
];

// --- Image ------------------------------------------------------------------

const IMAGE: &[Example] = &[
    // THE BOX BEFORE THE BYTES, which is the whole of this component. `width`
    // and `height` are the file's own numbers, handed to the browser before any
    // stylesheet has loaded.
    example!(
        r"Image { src, alt, ratio, width, height }",
        Key::ExampleImageNoteBox,
        |context| rsx! {
            div { class: "nd-example-picture",
                Image {
                    src: format!("{}assets/portraits/ada.svg", crate::site::BASE),
                    alt: context.t(Key::ExampleImageAlt),
                    ratio: ImageRatio::Landscape,
                    // NO `fit: Cover` AND NO `loading`. `cover` is the default
                    // and `loading` was named in the caption without ever
                    // being passed. Both are shown further down at a value
                    // that differs.
                    width: "480",
                    height: "360",
                }
            }
        }
    ),
    // A CAPTION MAKES IT A `<figure>`, which is what attaches the words to the
    // picture rather than leaving them under it.
    example!(
        r"Image { caption, ratio: Square }",
        Key::ExampleImageNoteFigure,
        |context| rsx! {
            div { class: "nd-example-picture",
                Image {
                    src: format!("{}assets/portraits/eve.svg", crate::site::BASE),
                    alt: context.t(Key::ExampleImageAlt),
                    caption: context.t(Key::ExampleImageCaption),
                    ratio: ImageRatio::Square,
                }
            }
        }
    ),
    // AN EMPTY `alt` IS AN ANSWER, and it is the right one here: the sentence
    // beside the picture already says who this is.
    example!(
        r#"Image { alt: "", rounded: true, ratio: Square, loading: Eager }"#,
        Key::ExampleImageNoteEmpty,
        |context| rsx! {
            span { class: "nd-example-byline",
                span { class: "nd-example-face",
                    Image {
                        src: format!("{}assets/portraits/cara.svg", crate::site::BASE),
                        alt: "",
                        rounded: true,
                        ratio: ImageRatio::Square,
                        loading: ImageLoading::Eager,
                    }
                }
                Text { "{context.t(Key::ExampleImageBeside)}" }
            }
        }
    ),
    // ROUNDED AND `contain`, for a drawing whose edges carry meaning. Cropping
    // a mark to fill a box is how a mark stops being the mark.
    example!(
        r"Image { rounded: true, fit: Contain, ratio: Wide }",
        Key::ExampleImageNoteRounded,
        |context| rsx! {
            div { class: "nd-example-picture",
                Image {
                    src: format!("{}favicon.svg", crate::site::BASE),
                    alt: context.t(Key::ExampleImageMarkAlt),
                    rounded: true,
                    fit: ImageFit::Contain,
                    ratio: ImageRatio::Wide,
                }
            }
        }
    ),
    // THE FOUR SHAPES, ITERATED, over one file. What changes is the box.
    example!(
        r"Image { ratio } for ratio in ImageRatio::ALL",
        Key::ExampleImageNoteRatio,
        |context| rsx! {
            div { class: "nd-example-row",
                for ratio in ImageRatio::ALL {
                    div { class: "nd-example-thumb",
                        Image {
                            src: format!("{}assets/portraits/bob.svg", crate::site::BASE),
                            alt: context.t(Key::ExampleImageAlt),
                            ratio,
                        }
                    }
                }
            }
        }
    ),
];

// --- MessageBar -------------------------------------------------------------

const MESSAGE_BAR: &[Example] = &[
    // THE DEFAULT, AND THE QUIET ONE. A status waits for a pause instead of
    // cutting a screen reader off, which is right about something that worked.
    example!(
        r#"MessageBar { tone: "success", icon: IconName::Check, title, … }"#,
        Key::ExampleMessageBarNoteStatus,
        |context| rsx! {
            MessageBar {
                tone: "success",
                icon: IconName::Check,
                title: context.t(Key::ExampleMessageBarSavedTitle),
                "{context.t(Key::ExampleMessageBarSaved)}"
            }
        }
    ),
    // THE RUDE ONE, OPTED INTO. `alert` cuts in, and `live` says how loudly,
    // which is the one case worth overruling the role for.
    example!(
        r#"MessageBar { alert: true, live: Assertive, tone: "danger", icon: IconName::Alert }"#,
        Key::ExampleMessageBarNoteAlert,
        |context| rsx! {
            MessageBar {
                tone: "danger",
                icon: IconName::Alert,
                alert: true,
                live: MessageBarLive::Assertive,
                title: context.t(Key::ExampleMessageBarRefusedTitle),
                "{context.t(Key::ExampleMessageBarRefused)}"
            }
        }
    ),
    // ONE A READER CAN TAKE AWAY. The control ships hidden and the script
    // reveals it, so a page with no script carries no button that closes
    // nothing.
    example!(
        r#"MessageBar { dismissible: true, dismiss_label, tone: "warning", icon: IconName::Warning }"#,
        Key::ExampleMessageBarNoteDismiss,
        |context| rsx! {
            MessageBar {
                tone: "warning",
                icon: IconName::Warning,
                dismissible: true,
                dismiss_label: context.t(Key::ExampleMessageBarDismiss),
                title: context.t(Key::ExampleMessageBarExpiringTitle),
                "{context.t(Key::ExampleMessageBarExpiring)}"
            }
        }
    ),
    // NO TONE AT ALL, which is the absence of the attribute rather than a name
    // called `default`. The words do the work on their own.
    example!(
        r"MessageBar { … }",
        Key::ExampleMessageBarNotePlain,
        |context| rsx! {
            MessageBar { icon: IconName::Info, "{context.t(Key::ExampleMessageBarPlain)}" }
        }
    ),
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
        Key::ExampleDialogNoteModal,
        |context| rsx! {
            Dialog {
                name: "dialog-remove",
                trigger_label: context.t(Key::ExampleDialogOpen),
                heading: context.t(Key::ExampleDialogHeading),
                description: context.t(Key::ExampleDialogDescription),
                close_label: context.t(Key::ExampleDialogClose),
                Paragraph { "{context.t(Key::ExampleDialogBody)}" }
                div { class: "nd-example-row",
                    Button {
                        variant: ButtonVariant::Outline,
                        tone: ButtonTone::Neutral,
                        label: context.t(Key::ExampleDialogCancel),
                    }
                    Button {
                        variant: ButtonVariant::Solid,
                        tone: ButtonTone::Danger,
                        label: context.t(Key::ExampleDialogConfirm),
                    }
                }
            }
        }
    ),
    // BESIDE THE PAGE RATHER THAN OVER IT. `show()` leaves the document
    // reachable, so the reader can keep reading the thing the notes are about.
    example!(
        r"Dialog { modal: false, size: DialogSize::Small, … }",
        Key::ExampleDialogNoteBeside,
        |context| rsx! {
            Dialog {
                name: "dialog-notes",
                modal: false,
                size: DialogSize::Small,
                trigger_label: context.t(Key::ExampleDialogNotesOpen),
                heading: context.t(Key::ExampleDialogNotesHeading),
                close_label: context.t(Key::ExampleDialogClose),
                Paragraph { "{context.t(Key::ExampleDialogNotesBody)}" }
            }
        }
    ),
    example!(
        r"Dialog { size: DialogSize::Large, dismissible: false, … }",
        Key::ExampleDialogNoteWide,
        |context| rsx! {
            Dialog {
                name: "dialog-terms",
                size: DialogSize::Large,
                dismissible: false,
                trigger_label: context.t(Key::ExampleDialogTermsOpen),
                heading: context.t(Key::ExampleDialogTermsHeading),
                Paragraph { "{context.t(Key::ExampleDialogTermsBody)}" }
            }
        }
    ),
    // NAMED BY `label`, because there is no title to name it from.
    example!(
        r#"Dialog { label: "…", … }"#,
        Key::ExampleDialogNotePlain,
        |context| rsx! {
            Dialog {
                name: "dialog-rename",
                label: context.t(Key::ExampleDialogRenameLabel),
                trigger_label: context.t(Key::ExampleDialogRenameOpen),
                close_label: context.t(Key::ExampleDialogClose),
                Paragraph { "{context.t(Key::ExampleDialogRenameBody)}" }
            }
        }
    ),
];

// --- Popover ----------------------------------------------------------------

const POPOVER: &[Example] = &[
    // CONTROLS IN IT, which is the difference between this and an `Info`.
    example!(
        r"Popover { name, trigger_label, heading, close_label, … }",
        Key::ExamplePopoverNoteControls,
        |context| rsx! {
            Popover {
                name: "popover-filter",
                trigger_label: context.t(Key::ExamplePopoverOpen),
                heading: context.t(Key::ExamplePopoverHeading),
                close_label: context.t(Key::ExamplePopoverClose),
                Checkbox {
                    name: "popover-filter-unread",
                    value: "unread",
                    label: context.t(Key::ExamplePopoverUnread),
                }
                Button {
                    variant: ButtonVariant::Solid,
                    tone: ButtonTone::Accent,
                    label: context.t(Key::ExamplePopoverApply),
                }
            }
        }
    ),
    example!(
        r"Popover { placement: PopoverPlacement::End, … }",
        Key::ExamplePopoverNoteEnd,
        |context| rsx! {
            div { class: "nd-example-end",
                Popover {
                    name: "popover-share",
                    placement: PopoverPlacement::End,
                    trigger_label: context.t(Key::ExamplePopoverShareOpen),
                    heading: context.t(Key::ExamplePopoverShareHeading),
                    close_label: context.t(Key::ExamplePopoverClose),
                    Paragraph { "{context.t(Key::ExamplePopoverShareBody)}" }
                    Button {
                        variant: ButtonVariant::Outline,
                        tone: ButtonTone::Neutral,
                        icon: IconName::Copy,
                        label: context.t(Key::ExamplePopoverCopy),
                    }
                }
            }
        }
    ),
    example!(
        r"Popover { dismissible: false, … }",
        Key::ExamplePopoverNoteFixed,
        |context| rsx! {
            Popover {
                name: "popover-help",
                dismissible: false,
                trigger_label: context.t(Key::ExamplePopoverHelpOpen),
                heading: context.t(Key::ExamplePopoverHelpHeading),
                Paragraph { "{context.t(Key::ExamplePopoverHelpBody)}" }
            }
        }
    ),
    example!(
        r#"Popover { label: "…", … }"#,
        Key::ExamplePopoverNotePlain,
        |context| rsx! {
            Popover {
                name: "popover-sort",
                label: context.t(Key::ExamplePopoverPlainLabel),
                trigger_label: context.t(Key::ExamplePopoverPlainOpen),
                close_label: context.t(Key::ExamplePopoverClose),
                Button {
                    variant: ButtonVariant::Transparent,
                    tone: ButtonTone::Neutral,
                    label: context.t(Key::ExamplePopoverNewest),
                }
                Button {
                    variant: ButtonVariant::Transparent,
                    tone: ButtonTone::Neutral,
                    label: context.t(Key::ExamplePopoverOldest),
                }
            }
        }
    ),
];

// --- Tooltip ----------------------------------------------------------------

const TOOLTIP: &[Example] = &[
    // THE HINT IS NEVER THE NAME. The button says `Copy` on it, and the hint
    // narrows what gets copied.
    example!(
        r#"Tooltip { name, text: "…", Button { … } }"#,
        Key::ExampleTooltipNoteHint,
        |context| rsx! {
            Tooltip {
                name: "tooltip-copy",
                text: context.t(Key::ExampleTooltipCopyHint),
                Button {
                    variant: ButtonVariant::Outline,
                    tone: ButtonTone::Neutral,
                    icon: IconName::Copy,
                    label: context.t(Key::ExampleTooltipCopy),
                }
            }
        }
    ),
    example!(
        r"Tooltip { delay: TooltipDelay::Long, … }",
        Key::ExampleTooltipNoteLong,
        |context| rsx! {
            Tooltip {
                name: "tooltip-export",
                delay: TooltipDelay::Long,
                text: context.t(Key::ExampleTooltipExportHint),
                Button {
                    variant: ButtonVariant::Outline,
                    tone: ButtonTone::Neutral,
                    icon: IconName::Download,
                    label: context.t(Key::ExampleTooltipExport),
                }
            }
        }
    ),
    example!(
        r"Tooltip { placement: TooltipPlacement::End, … }",
        Key::ExampleTooltipNoteEnd,
        |context| rsx! {
            div { class: "nd-example-end",
                Tooltip {
                    name: "tooltip-refresh",
                    placement: TooltipPlacement::End,
                    text: context.t(Key::ExampleTooltipRefreshHint),
                    Button {
                        variant: ButtonVariant::Outline,
                        tone: ButtonTone::Neutral,
                        icon: IconName::RotateCcw,
                        label: context.t(Key::ExampleTooltipRefresh),
                    }
                }
            }
        }
    ),
    // AN ICON-ONLY CONTROL, which is where a tooltip is most tempting and most
    // dangerous: `label` is still the name, and the hint only describes.
    example!(
        r"Tooltip { delay: TooltipDelay::None, … Button { hide_label: true } }",
        Key::ExampleTooltipNoteNone,
        |context| rsx! {
            Tooltip {
                name: "tooltip-lock",
                delay: TooltipDelay::None,
                text: context.t(Key::ExampleTooltipLockHint),
                Button {
                    variant: ButtonVariant::Outline,
                    tone: ButtonTone::Neutral,
                    icon: IconName::Eye,
                    hide_label: true,
                    label: context.t(Key::ExampleTooltipLock),
                }
            }
        }
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
        Key::ExampleToastNoteStays,
        |context| rsx! {
            div { class: "nd-example-toasts",
                Toast {
                    name: "toast-saved",
                    title: context.t(Key::ExampleToastSavedTitle),
                    dismiss_label: context.t(Key::ExampleToastDismiss),
                    "{context.t(Key::ExampleToastSaved)}"
                }
            }
        }
    ),
    example!(
        r#"Toast { tone: "success", duration: "6000", … }"#,
        Key::ExampleToastNoteTimer,
        |context| rsx! {
            div { class: "nd-example-toasts",
                Toast {
                    name: "toast-sent",
                    tone: "success",
                    duration: "6000",
                    title: context.t(Key::ExampleToastSentTitle),
                    dismiss_label: context.t(Key::ExampleToastDismiss),
                    "{context.t(Key::ExampleToastSent)}"
                }
            }
        }
    ),
    example!(
        r#"Toast { tone: "danger", live: Assertive, placement: TopEnd }"#,
        Key::ExampleToastNoteLoud,
        |context| rsx! {
            div { class: "nd-example-toasts",
                Toast {
                    name: "toast-failed",
                    tone: "danger",
                    live: ToastLive::Assertive,
                    placement: ToastPlacement::TopEnd,
                    title: context.t(Key::ExampleToastFailedTitle),
                    dismiss_label: context.t(Key::ExampleToastDismiss),
                    "{context.t(Key::ExampleToastFailed)}"
                }
            }
        }
    ),
    // A MESSAGE WITH SOMETHING TO DO IN IT, which `children`'s own contract
    // allows in as many words: "it may hold a `Link` to whatever a reader has
    // to do about it". Nothing demonstrated it, and two defects lived in that
    // gap — `toast.js` flattened the body to text on the one path that matters
    // and deleted the link, and its pause counter could never come back to
    // zero once a message held more than one focusable thing.
    example!(
        r#"Toast { tone: "danger", duration: "6000", … } — with a Link in it"#,
        Key::ExampleToastNoteAction,
        |context| rsx! {
            div { class: "nd-example-toasts",
                Toast {
                    name: "toast-review",
                    tone: "danger",
                    duration: "6000",
                    title: context.t(Key::ExampleToastReviewTitle),
                    dismiss_label: context.t(Key::ExampleToastDismiss),
                    "{context.t(Key::ExampleToastReview)} "
                    Link { href: "#nd-main", "{context.t(Key::ExampleToastReviewLink)}" }
                }
            }
        }
    ),
    example!(
        r"Toast { dismissible: false, placement: ToastPlacement::BottomStart }",
        Key::ExampleToastNoteFixed,
        |context| rsx! {
            div { class: "nd-example-toasts",
                Toast {
                    name: "toast-offline",
                    dismissible: false,
                    placement: ToastPlacement::BottomStart,
                    "{context.t(Key::ExampleToastOffline)}"
                }
            }
        }
    ),
];

// --- Abbr -------------------------------------------------------------------

const ABBR: &[Example] = &[
    example!(
        r#"Abbr { title: "Cascading Style Sheets", "CSS" }"#,
        |context| rsx! {
            Paragraph {
                "{context.t(Key::ExampleAbbrLead)} "
                Abbr { title: "Cascading Style Sheets", class: "nd-identifier", "CSS" }
                "."
            }
        }
    ),
    example!(
        r#"Abbr { title: "…", "WCAG" }"#,
        Key::ExampleAbbrTouch,
        |context| rsx! {
            Paragraph {
                "{context.t(Key::ExampleAbbrExpansion)} "
                Abbr { title: "Web Content Accessibility Guidelines", class: "nd-identifier", "WCAG" }
                "."
            }
        }
    ),
    example!(r"Heading { … Abbr { … } }", |context| rsx! {
        Heading { level: HeadingLevel::_3, size: HeadingSize::H4,
            Abbr { title: "Hypertext Markup Language", class: "nd-identifier", "HTML" }
            " {context.t(Key::ExampleAbbrHeading)}"
        }
    }),
    example!(r"Two in one sentence", |context| rsx! {
        Paragraph {
            Abbr { title: "Application Programming Interface", class: "nd-identifier", "API" }
            "{context.t(Key::ExampleAbbrPair)} "
            Abbr { title: "Accessible Rich Internet Applications", class: "nd-identifier", "ARIA" }
            "."
        }
    }),
];

// --- Blockquote -------------------------------------------------------------

const BLOCKQUOTE: &[Example] = &[
    example!(r"Blockquote { … }", |context| rsx! {
        Blockquote {
            Paragraph { "{context.t(Key::ExampleQuote)}" }
        }
    }),
    example!(
        r"Blockquote { tone: BlockquoteTone::Accent, … }",
        |context| rsx! {
            Blockquote { tone: BlockquoteTone::Accent,
                Paragraph { "{context.t(Key::ExampleQuote)}" }
            }
        }
    ),
    example!(r#"Blockquote { attribution: "…", … }"#, |context| rsx! {
        Blockquote { attribution: "Edsger W. Dijkstra",
            Paragraph { "{context.t(Key::ExampleQuote)}" }
        }
    }),
    example!(
        r#"Blockquote { cite: "…", attribution: "…", … }"#,
        Key::ExampleQuoteCite,
        |context| rsx! {
            Blockquote {
                tone: BlockquoteTone::Accent,
                cite: "https://example.invalid/on-simplicity",
                attribution: "Edsger W. Dijkstra",
                Paragraph { "{context.t(Key::ExampleQuote)}" }
            }
        }
    ),
    example!(r"Blockquote { … Link { … } }", |context| rsx! {
        Blockquote {
            Paragraph {
                "{context.t(Key::ExampleQuoteShort)} "
                Link { href: "#", "{context.t(Key::ExampleLinkText)}" }
            }
        }
    }),
    example!(
        r"Blockquote { glass: true, … }",
        Key::ExampleGlassBehind,
        |context| rsx! {
            Blockquote { glass: true,
                Paragraph { "{context.t(Key::ExampleQuote)}" }
            }
        }
    ),
];

// --- Button -----------------------------------------------------------------

const BUTTON: &[Example] = &[
    example!(r#"Button { label: "…" }"#, |context| rsx! {
        Button { label: context.t(Key::ExampleButtonSave) }
    }),
    example!(
        r"Button { variant: ButtonVariant::… }",
        Key::ExampleButtonVariants,
        |context| rsx! {
            div { class: "nd-example-row",
                for variant in ButtonVariant::ALL {
                    Button { variant, label: context.t(Key::ExampleButtonSave) }
                }
            }
        }
    ),
    example!(
        r"Button { tone: ButtonTone::… }",
        Key::ExampleButtonTones,
        |context| rsx! {
            div { class: "nd-example-row",
                for tone in ButtonTone::ALL {
                    Button { tone, label: context.t(Key::ExampleButtonPublish) }
                }
            }
        }
    ),
    example!(r"Button { size: ButtonSize::… }", |context| rsx! {
        div { class: "nd-example-row",
            for size in ButtonSize::ALL {
                Button { size, label: context.t(Key::ExampleButtonSave) }
            }
        }
    }),
    example!(
        r"Button { shape: ButtonShape::… }",
        Key::ExampleButtonShapes,
        |context| rsx! {
            div { class: "nd-example-row",
                for shape in ButtonShape::ALL {
                    Button { shape, label: context.t(Key::ExampleButtonSave) }
                }
            }
        }
    ),
    example!(r"Button { icon: IconName::Check, … }", |context| rsx! {
        div { class: "nd-example-row",
            Button { icon: IconName::Check, label: context.t(Key::ExampleButtonPublish) }
            Button {
                icon: IconName::Copy,
                variant: ButtonVariant::Outline,
                tone: ButtonTone::Neutral,
                label: context.t(Key::ExampleCopy),
            }
        }
    }),
    example!(
        // THE CAPTION NAMED AN ICON THE EXAMPLE DID NOT DRAW. It said
        // `ChevronDown` and rendered `ExternalLink`, which is a caption a reader
        // has no way to check and every reason to believe.
        r"Button { icon_end: IconName::ExternalLink | ::ChevronRight, … }",
        Key::ExampleButtonIconEnd,
        |context| rsx! {
            div { class: "nd-example-row",
                Button {
                    icon_end: IconName::ExternalLink,
                    variant: ButtonVariant::Outline,
                    tone: ButtonTone::Neutral,
                    label: context.t(Key::AppRepository),
                }
                // A second, because the prop is a CHOICE: a destination and a
                // direction are the two things a trailing icon is ever for, and
                // one button cannot show a choice.
                Button {
                    icon_end: IconName::ChevronRight,
                    variant: ButtonVariant::Subtle,
                    label: context.t(Key::ExampleButtonMore),
                }
            }
        }
    ),
    example!(
        r"Button { hide_label: true, icon: …, … }",
        Key::ExampleButtonIconOnly,
        |context| rsx! {
            div { class: "nd-example-row",
                for size in ButtonSize::ALL {
                    Button {
                        size,
                        hide_label: true,
                        icon: IconName::Settings,
                        variant: ButtonVariant::Transparent,
                        tone: ButtonTone::Neutral,
                        label: context.t(Key::SettingsLabel),
                    }
                }
            }
        }
    ),
    example!(
        r#"Button { detail: "…", justify: ButtonJustify::Start, … }"#,
        Key::ExampleButtonCompound,
        |context| rsx! {
            div { class: "nd-example-narrow",
                Button {
                    icon: IconName::Languages,
                    label: context.t(Key::ExampleButtonPublish),
                    detail: context.t(Key::ExampleButtonDetail),
                    justify: ButtonJustify::Start,
                    variant: ButtonVariant::Outline,
                    tone: ButtonTone::Neutral,
                    fill: true,
                }
            }
        }
    ),
    example!(
        r"Button { wrap: true, fill: true, … }",
        Key::ExampleButtonWrap,
        |context| rsx! {
            div { class: "nd-example-narrow",
                Button {
                    wrap: true,
                    fill: true,
                    label: context.t(Key::ExampleButtonLong),
                }
            }
        }
    ),
    example!(
        r#"Button { menu: true, expanded, controls: "…" }"#,
        Key::ExampleButtonMenu,
        |context| rsx! {
            div {
                // IT OPENS SOMETHING NOW. This shipped as a button carrying
                // `aria-haspopup="menu"` over a hidden paragraph — announcing a
                // menu to a screen reader and doing nothing on every press,
                // because the system had no menu for it to open. `Menu` is that
                // component, and `dist/menu.js` is what makes the press work.
                Button {
                    menu: true,
                    expanded: false,
                    controls: "example-button-actions",
                    variant: ButtonVariant::Outline,
                    tone: ButtonTone::Neutral,
                    label: context.t(Key::ExampleButtonMore),
                }
                Menu {
                    name: "example-button-actions",
                    label: context.t(Key::ExampleMenuLabel),
                    MenuItem { icon: "copy", label: context.t(Key::ExampleMenuDuplicate) }
                    MenuItem { label: context.t(Key::ExampleMenuRename) }
                    MenuItem { icon: "download", label: context.t(Key::ExampleMenuArchive) }
                }
            }
        }
    ),
    example!(
        r#"Button { href: "…", rel: "noreferrer", target: "_blank" }"#,
        Key::ExampleButtonLink,
        |context| rsx! {
            div { class: "nd-example-row",
                Button {
                    href: "https://github.com/noctua-world/noctua-design",
                    rel: "noreferrer",
                    target: "_blank",
                    icon_end: IconName::ExternalLink,
                    variant: ButtonVariant::Outline,
                    tone: ButtonTone::Neutral,
                    label: context.t(Key::AppRepository),
                }
                Button {
                    href: "#",
                    variant: ButtonVariant::Link,
                    label: context.t(Key::ExampleLinkText),
                }
            }
        }
    ),
    example!(
        r#"Button { pressed: true, icon_alt: …, label_alt: "…", action: "…" }"#,
        Key::ExampleButtonPressed,
        |context| rsx! {
            div { class: "nd-example-row",
                Button {
                    hide_label: true,
                    icon: IconName::Moon,
                    icon_alt: IconName::Sun,
                    label: context.t(Key::ThemeModeDark),
                    label_alt: context.t(Key::ThemeModeLight),
                    variant: ButtonVariant::Transparent,
                    tone: ButtonTone::Neutral,
                }
                Button {
                    pressed: true,
                    hide_label: true,
                    icon: IconName::Moon,
                    icon_alt: IconName::Sun,
                    // A TOGGLE IS THE BUTTON MOST LIKELY TO NEED FINDING. It
                    // has no form to post to and no address to go to, so the
                    // only thing that can act on a press is a script — and this
                    // is how the script finds it.
                    action: "example-mode",
                    label: context.t(Key::ThemeModeDark),
                    label_alt: context.t(Key::ThemeModeLight),
                    variant: ButtonVariant::Transparent,
                    tone: ButtonTone::Neutral,
                }
            }
        }
    ),
    example!(
        r"Button { disabled: true, … }",
        Key::ExampleButtonDisabled,
        |context| rsx! {
            div { class: "nd-example-row",
                Button { disabled: true, label: context.t(Key::ExampleButtonSave) }
                Button {
                    disabled: true,
                    variant: ButtonVariant::Outline,
                    tone: ButtonTone::Neutral,
                    label: context.t(Key::ExampleButtonCancel),
                }
            }
        }
    ),
    example!(
        r#"Button { submit: ButtonSubmit::Submit, name: "…", value: "…" }"#,
        Key::ExampleButtonSubmit,
        |context| rsx! {
            // A REAL FORM, BECAUSE `type` HAS NO APPEARANCE. Three buttons with
            // one label differing only in an attribute that does not paint is
            // three identical pictures — the prop looked like it did nothing.
            // Put them in a form with a field and the difference is the whole
            // point: `reset` empties it, `submit` sends it, `button` waits.
            form {
                class: "nd-example-stack",
                // Nothing leaves the page: the example is what the buttons do
                // to the field beside them, not where a real form would go.
                action: "#",
                Input {
                    name: "example-button-subject",
                    label: context.t(Key::ExampleButtonSubject),
                    value: context.t(Key::ExampleButtonPublish),
                }
                div { class: "nd-example-row",
                    for (submit, label, variant, tone) in [
                        (ButtonSubmit::Submit, Key::ExampleButtonSend, ButtonVariant::Solid, ButtonTone::Accent),
                        (ButtonSubmit::Button, Key::ExampleButtonSave, ButtonVariant::Outline, ButtonTone::Neutral),
                        (ButtonSubmit::Reset, Key::ExampleButtonClear, ButtonVariant::Subtle, ButtonTone::Danger),
                    ] {
                        Button {
                            key: "{submit.as_str()}",
                            submit,
                            // WHICH BUTTON WAS PRESSED, and the only way to ask
                            // it with no script: the browser posts the name and
                            // value of the one that was used and of no other.
                            name: "action",
                            value: submit.as_str(),
                            variant,
                            tone,
                            label: context.t(label),
                        }
                    }
                }
            }
        }
    ),
    example!(
        r"Button { variant: ButtonVariant::Nav, current: true, … }",
        Key::ExampleButtonNav,
        |context| rsx! {
            div { class: "nd-example-stack",
                Button {
                    variant: ButtonVariant::Nav,
                    href: "#",
                    current: true,
                    label: context.t(Key::ExampleButtonSave),
                }
                Button {
                    variant: ButtonVariant::Nav,
                    href: "#",
                    label: context.t(Key::ExampleButtonPublish),
                }
            }
        }
    ),
    example!(
        r"Button { glass: true, … }",
        Key::ExampleGlassBehind,
        |context| rsx! {
            div { class: "nd-example-row",
                for tone in ButtonTone::ALL {
                    Button { glass: true, tone, label: context.t(Key::ExampleButtonSave) }
                }
            }
        }
    ),
];

// --- ButtonGroup ------------------------------------------------------------

const BUTTON_GROUP: &[Example] = &[
    example!(
        r#"ButtonGroup { label: "…", Button { … } }"#,
        Key::ExampleGroupAttached,
        |context| rsx! {
            ButtonGroup { label: context.t(Key::ExampleGroupLabel),
                Button {
                    variant: ButtonVariant::Outline,
                    tone: ButtonTone::Neutral,
                    label: context.t(Key::ExampleGroupPrevious),
                }
                Button {
                    variant: ButtonVariant::Outline,
                    tone: ButtonTone::Neutral,
                    label: context.t(Key::ExampleGroupNext),
                }
            }
        }
    ),
    example!(r"ButtonGroup { attached: false, … }", |context| rsx! {
        ButtonGroup { label: context.t(Key::ExampleGroupLabel), attached: false,
            Button { label: context.t(Key::ExampleButtonSave) }
            Button {
                variant: ButtonVariant::Transparent,
                tone: ButtonTone::Neutral,
                label: context.t(Key::ExampleButtonCancel),
            }
        }
    }),
    example!(
        r"ButtonGroup { orientation: ButtonGroupOrientation::Vertical, … }",
        |context| rsx! {
            ButtonGroup {
                label: context.t(Key::ExampleGroupLabel),
                orientation: ButtonGroupOrientation::Vertical,
                for label in [Key::ExampleButtonSave, Key::ExampleButtonPublish, Key::ExampleButtonCancel] {
                    Button {
                        variant: ButtonVariant::Outline,
                        tone: ButtonTone::Neutral,
                        justify: ButtonJustify::Start,
                        label: context.t(label),
                    }
                }
            }
        }
    ),
    example!(
        r"ButtonGroup { fill: true, … }",
        Key::ExampleGroupFill,
        |context| rsx! {
            div { class: "nd-example-narrow",
                ButtonGroup { label: context.t(Key::ExampleGroupLabel), fill: true,
                    Button {
                        variant: ButtonVariant::Outline,
                        tone: ButtonTone::Neutral,
                        label: context.t(Key::ExampleGroupPrevious),
                    }
                    Button {
                        variant: ButtonVariant::Outline,
                        tone: ButtonTone::Neutral,
                        label: context.t(Key::ExampleGroupNext),
                    }
                }
            }
        }
    ),
    example!(
        r#"ButtonGroup { labelledby: "…", … } — a split button"#,
        Key::ExampleGroupSplit,
        |context| rsx! {
            div {
                Label {
                    kind: LabelKind::Name,
                    for_control: "example-split",
                    text: context.t(Key::ExampleGroupSplitLabel),
                }
                ButtonGroup { labelledby: "example-split-label",
                    Button { label: context.t(Key::ExampleButtonPublish) }
                    Button {
                        menu: true,
                        hide_label: true,
                        icon: IconName::ChevronDown,
                        controls: "example-split-menu",
                        label: context.t(Key::ExampleButtonMore),
                    }
                }
                // A REAL `Menu`, AND IT USED TO BE A HIDDEN `<div>`. The
                // second half of a split button announced `aria-haspopup` at a
                // paragraph, so `menu.js` skipped it and the control opened
                // nothing — the exact defect that page documents, demonstrated
                // on the page documenting it.
                Menu {
                    name: "example-split-menu",
                    label: context.t(Key::ExampleMenuLabel),
                    MenuItem { label: context.t(Key::ExampleMenuDuplicate) }
                    MenuItem { label: context.t(Key::ExampleMenuRename) }
                }
            }
        }
    ),
];

// --- Segmented and SegmentedItem ---------------------------------------------

const SEGMENTED: &[Example] = &[
    example!(
        r#"Segmented { name: "…", label: "…", SegmentedItem { … } }"#,
        Key::ExampleSegmentedTabStop,
        |context| rsx! {
            Segmented { name: "example-mode", label: context.t(Key::ThemeModeLabel),
                for (value, label, selected) in [
                    ("light", Key::ThemeModeLight, false),
                    ("dark", Key::ThemeModeDark, true),
                    ("system", Key::ThemeModeSystem, false),
                ] {
                    SegmentedItem { value, label: context.t(label), selected }
                }
            }
        }
    ),
    example!(
        r"Segmented { appearance: SegmentedAppearance::Spread, … }",
        Key::ExampleSegmentedSpread,
        |context| rsx! {
            div { class: "nd-example-stack",
                for appearance in SegmentedAppearance::ALL {
                    Segmented {
                        appearance,
                        name: "example-appearance-{appearance.as_str()}",
                        label: context.t(Key::ThemeGlassLabel),
                        for (value, label, selected) in [
                            ("off", Key::ThemeGlassOff, false),
                            ("subtle", Key::ThemeGlassSubtle, true),
                            ("full", Key::ThemeGlassFull, false),
                        ] {
                            SegmentedItem { value, label: context.t(label), selected }
                        }
                    }
                }
            }
        }
    ),
    example!(r"Segmented { size: SegmentedSize::… }", |context| rsx! {
        div { class: "nd-example-stack",
            for size in SegmentedSize::ALL {
                Segmented {
                    size,
                    name: "example-size-{size.as_str()}",
                    label: context.t(Key::ThemeFontLabel),
                    for (value, label, selected) in [
                        ("sans", Key::ThemeFontSans, true),
                        ("mono", Key::ThemeFontMono, false),
                    ] {
                        SegmentedItem { value, label: context.t(label), selected }
                    }
                }
            }
        }
    }),
    example!(
        r"Segmented { shape: SegmentedShape::… }",
        Key::ExampleSegmentedShapes,
        |context| rsx! {
            div { class: "nd-example-stack",
                for shape in SegmentedShape::ALL {
                    Segmented {
                        shape,
                        name: "example-shape-{shape.as_str()}",
                        label: context.t(Key::ThemeSaturationLabel),
                        for (value, label, selected) in [
                            ("balanced", Key::ThemeSaturationBalanced, true),
                            ("vivid", Key::ThemeSaturationVivid, false),
                        ] {
                            SegmentedItem { value, label: context.t(label), selected }
                        }
                    }
                }
            }
        }
    ),
    example!(
        r"Segmented { orientation: SegmentedOrientation::Vertical, … }",
        Key::ExampleSegmentedVertical,
        |context| rsx! {
            Segmented {
                name: "example-vertical",
                label: context.t(Key::ThemeRadiusLabel),
                orientation: SegmentedOrientation::Vertical,
                for (value, label, selected) in [
                    ("square", Key::ThemeRadiusSquare, true),
                    ("soft", Key::ThemeRadiusSoft, false),
                    ("round", Key::ThemeRadiusRound, false),
                    ("circular", Key::ThemeRadiusCircular, false),
                ] {
                    SegmentedItem { value, label: context.t(label), selected }
                }
            }
        }
    ),
    example!(
        r"Segmented { name, label, fill: SegmentedFill::Equal }
Segmented { name, label, fill: SegmentedFill::Stretch }",
        Key::ExampleSegmentedEqual,
        |context| rsx! {
            // BOTH VALUES, SIDE BY SIDE. The caption used to say `Equal` while
            // the body wrote `Stretch`, so the one value a reader came to see
            // was demonstrated nowhere on the site. `equal` gives every item
            // the width of the widest and `stretch` then takes the container,
            // which only shows when the two sit one above the other.
            div { class: "nd-example-narrow nd-example-stack",
                for (name, fill) in [
                    ("example-fill-equal", SegmentedFill::Equal),
                    ("example-fill-stretch", SegmentedFill::Stretch),
                ] {
                    Segmented {
                        key: "{name}",
                        name,
                        label: context.t(Key::SettingsPatternLabel),
                        fill,
                        for (value, label, selected) in [
                            ("none", Key::SettingsPatternNone, true),
                            ("dots", Key::SettingsPatternDots, false),
                            ("mesh", Key::SettingsPatternMesh, false),
                        ] {
                            SegmentedItem { value, label: context.t(label), selected }
                        }
                    }
                }
            }
        }
    ),
    example!(
        r#"Segmented { labelledby: "…", … }"#,
        Key::ExampleSegmentedLabelledby,
        |context| rsx! {
            div { class: "nd-example-stack",
                Label {
                    kind: LabelKind::Name,
                    for_control: "example-visible",
                    text: context.t(Key::ThemeSaturationLabel),
                }
                Segmented {
                    name: "example-labelledby",
                    labelledby: "example-visible-label",
                    for (value, label, selected) in [
                        ("balanced", Key::ThemeSaturationBalanced, false),
                        ("vivid", Key::ThemeSaturationVivid, true),
                        ("sober", Key::ThemeSaturationSober, false),
                    ] {
                        SegmentedItem { value, label: context.t(label), selected }
                    }
                }
            }
        }
    ),
    example!(
        r"Segmented { glass: true, … }",
        Key::ExampleGlassBehind,
        |context| rsx! {
            Segmented {
                name: "example-glass",
                glass: true,
                label: context.t(Key::ThemeSaturationLabel),
                SegmentedItem {
                    value: "balanced",
                    label: context.t(Key::ThemeSaturationBalanced),
                    selected: true,
                }
                SegmentedItem { value: "vivid", label: context.t(Key::ThemeSaturationVivid) }
            }
        }
    ),
    example!(
        r"Segmented { selection: SegmentedSelection::Quiet, … }",
        Key::ExampleSegmentedQuiet,
        |context| rsx! {
            Segmented {
                name: "example-quiet",
                selection: SegmentedSelection::Quiet,
                size: SegmentedSize::Small,
                label: context.t(Key::ExampleWidthLabel),
                for (value, label, detail, selected) in [
                    ("320", "320", "px", false),
                    ("sm", "30", "rem", true),
                    ("md", "48", "rem", false),
                ] {
                    SegmentedItem { value, class: "nd-identifier", label, detail, selected }
                }
            }
        }
    ),
];

const SEGMENTED_ITEM: &[Example] = &[
    example!(
        r#"SegmentedItem { value: "…", label: "…" }"#,
        |context| rsx! {
            // ONE ITEM STATES THE ANSWER. A radiogroup with nothing chosen is a
            // legal state and the group is still one tab stop, but the first
            // picture of this component should show what a radio group looks
            // like when it has been answered.
            Segmented { name: "example-item", label: context.t(Key::ThemeSaturationLabel),
                SegmentedItem {
                    value: "balanced",
                    label: context.t(Key::ThemeSaturationBalanced),
                    selected: true,
                }
                SegmentedItem { value: "vivid", label: context.t(Key::ThemeSaturationVivid) }
            }
        }
    ),
    example!(
        r"SegmentedItem { selected: true, … }",
        Key::ExampleItemSelected,
        |context| rsx! {
            Segmented { name: "example-selected", label: context.t(Key::ThemeSaturationLabel),
                SegmentedItem {
                    value: "vivid",
                    label: context.t(Key::ThemeSaturationVivid),
                    selected: true,
                }
                SegmentedItem { value: "sober", label: context.t(Key::ThemeSaturationSober) }
            }
        }
    ),
    example!(
        r"SegmentedItem { icon: …, hide_label: true, … }",
        Key::ExampleItemIcons,
        |context| rsx! {
            Segmented { name: "example-icons", label: context.t(Key::ThemeModeLabel),
                for (value, icon, label, selected) in [
                    ("light", IconName::Sun, Key::ThemeModeLight, false),
                    ("dark", IconName::Moon, Key::ThemeModeDark, false),
                    ("system", IconName::Monitor, Key::ThemeModeSystem, true),
                ] {
                    SegmentedItem { value, icon, hide_label: true, label: context.t(label), selected }
                }
            }
        }
    ),
    example!(
        r"SegmentedItem { disabled: true, … }",
        Key::ExampleItemDisabled,
        |context| rsx! {
            Segmented { name: "example-disabled", label: context.t(Key::ThemeFontLabel),
                SegmentedItem {
                    value: "sans",
                    label: context.t(Key::ThemeFontSans),
                    selected: true,
                }
                SegmentedItem {
                    value: "mono",
                    label: context.t(Key::ThemeFontMono),
                    disabled: true,
                }
            }
        }
    ),
    example!(
        r"SegmentedItem { Swatch { … } }",
        Key::ExampleItemSwatch,
        |context| rsx! {
            Segmented {
                name: "example-accent",
                label: context.t(Key::ThemeAccentLabel),
                appearance: SegmentedAppearance::Spread,
                for (value, label, selected) in [
                    ("rose-vivid", Key::ThemeAccentRose, false),
                    ("jade-balanced", Key::ThemeAccentJade, true),
                    ("indigo-sober", Key::ThemeAccentIndigo, false),
                ] {
                    SegmentedItem { value, label: context.t(label), selected,
                        Swatch { palette: value, size: SwatchSize::Small }
                    }
                }
            }
        }
    ),
    example!(
        r"SegmentedItem { glass: true, … }",
        Key::ExampleGlassBehind,
        |context| rsx! {
            Segmented { name: "example-item-glass", label: context.t(Key::ThemeSaturationLabel),
                SegmentedItem {
                    value: "balanced",
                    glass: true,
                    label: context.t(Key::ThemeSaturationBalanced),
                    selected: true,
                }
                SegmentedItem {
                    value: "vivid",
                    glass: true,
                    label: context.t(Key::ThemeSaturationVivid),
                }
            }
        }
    ),
    example!(
        r#"SegmentedItem { label: "320", detail: "px" }"#,
        Key::ExampleItemDetail,
        |context| rsx! {
            Segmented { name: "example-detail", label: context.t(Key::ExampleWidthLabel),
                for (value, label, detail, selected) in [
                    ("320", "320", "px", false),
                    ("sm", "30", "rem", true),
                    ("md", "48", "rem", false),
                ] {
                    // A length is an identifier, on both lines. The scanner
                    // that finds untranslated prose is right to ask, and
                    // marking it is how the width control answers.
                    SegmentedItem { value, class: "nd-identifier", label, detail, selected }
                }
            }
        }
    ),
];

// --- Switch -------------------------------------------------------------------

const SWITCH: &[Example] = &[
    example!(
        r#"Switch { name: "…", on: "on", off: "off", label: "…" }"#,
        Key::ExampleSwitchValues,
        |context| rsx! {
            Switch {
                name: "example-grain",
                on: "on",
                off: "off",
                label: context.t(Key::ExampleSwitchGrain),
            }
        }
    ),
    example!(
        r"Switch { checked: true, … }",
        Key::ExampleSwitchState,
        |context| rsx! {
            div { class: "nd-example-stack",
                Switch {
                    name: "example-pattern",
                    on: "on",
                    off: "off",
                    checked: true,
                    label: context.t(Key::ExampleSwitchPattern),
                }
                Switch {
                    name: "example-pattern-off",
                    on: "on",
                    off: "off",
                    label: context.t(Key::ExampleSwitchPattern),
                }
            }
        }
    ),
    example!(
        r#"Switch { describedby: "…", … }"#,
        Key::ExampleSwitchDescribed,
        |context| rsx! {
            div { class: "nd-example-stack",
                Switch {
                    name: "example-described",
                    on: "on",
                    off: "off",
                    checked: true,
                    describedby: "example-switch-note",
                    label: context.t(Key::SettingsGrainLabel),
                }
                p { class: "nd-note-line", id: "example-switch-note",
                    "{context.t(Key::SettingsGrainNote)}"
                }
            }
        }
    ),
    example!(r"Switch { size: SwitchSize::… }", |context| rsx! {
        div { class: "nd-example-stack",
            for size in SwitchSize::ALL {
                Switch {
                    size,
                    name: "example-size-{size.as_str()}",
                    on: "on",
                    off: "off",
                    checked: true,
                    label: context.t(Key::ExampleSwitchPattern),
                }
            }
        }
    }),
    example!(
        r"Switch { disabled: true, … }",
        Key::ExampleSwitchRefused,
        |context| rsx! {
            Switch {
                name: "example-refused",
                on: "on",
                off: "off",
                disabled: true,
                label: context.t(Key::SettingsGrainLabel),
            }
        }
    ),
    example!(
        r"Switch { glass: true, … }",
        Key::ExampleGlassBehind,
        |context| rsx! {
            Switch {
                name: "example-glass-switch",
                on: "on",
                off: "off",
                glass: true,
                checked: true,
                label: context.t(Key::ExampleSwitchGrain),
            }
        }
    ),
];

// --- Toolbar and ToolbarGroup ------------------------------------------------

/// The controls a toolbar example carries, so the four examples below differ
/// in the one prop each is about rather than in what is inside them.
macro_rules! toolbar_controls {
    ($context:expr) => {
        rsx! {
            Button {
                variant: ButtonVariant::Transparent,
                tone: ButtonTone::Neutral,
                size: ButtonSize::Small,
                hide_label: true,
                icon: IconName::Search,
                label: $context.t(Key::SearchLabel),
            }
            Button {
                variant: ButtonVariant::Transparent,
                tone: ButtonTone::Neutral,
                size: ButtonSize::Small,
                hide_label: true,
                icon: IconName::Copy,
                label: $context.t(Key::ExampleCopy),
            }
        }
    };
}

const TOOLBAR: &[Example] = &[
    example!(
        r#"Toolbar { label: "…", ToolbarGroup { … } }"#,
        Key::ExampleToolbarStops,
        |context| rsx! {
            Toolbar { label: context.t(Key::ExampleToolbarLabel),
                ToolbarGroup {
                    {toolbar_controls!(context)}
                }
                ToolbarGroup { align: ToolbarGroupAlign::End,
                    Segmented {
                        name: "example-toolbar-view",
                        size: SegmentedSize::Small,
                        label: context.t(Key::ExampleToolbarView),
                        for (value, label, selected) in [
                            ("light", Key::ThemeModeLight, true),
                            ("dark", Key::ThemeModeDark, false),
                        ] {
                            SegmentedItem { value, label: context.t(label), selected }
                        }
                    }
                    Button {
                        variant: ButtonVariant::Transparent,
                        tone: ButtonTone::Neutral,
                        size: ButtonSize::Small,
                        label: context.t(Key::ExampleReset),
                    }
                }
            }
        }
    ),
    example!(
        r"Toolbar { orientation: ToolbarOrientation::Vertical, … }",
        Key::ExampleToolbarVertical,
        |context| rsx! {
            Toolbar {
                label: context.t(Key::ExampleToolbarLabel),
                orientation: ToolbarOrientation::Vertical,
                ToolbarGroup {
                    {toolbar_controls!(context)}
                }
            }
        }
    ),
    example!(
        r"Toolbar { density: ToolbarDensity::Compact, wrap: false, … }",
        Key::ExampleToolbarDensity,
        |context| rsx! {
            div { class: "nd-example-stack",
                for density in ToolbarDensity::ALL {
                    Toolbar {
                        density,
                        wrap: false,
                        label: context.t(Key::ExampleToolbarLabel),
                        ToolbarGroup {
                            {toolbar_controls!(context)}
                        }
                        ToolbarGroup {
                            {toolbar_controls!(context)}
                        }
                    }
                }
            }
        }
    ),
    example!(r#"Toolbar { labelledby: "…", … }"#, |context| rsx! {
        div { class: "nd-example-stack",
            Label {
                kind: LabelKind::Name,
                for_control: "example-toolbar",
                text: context.t(Key::ExampleToolbarLabel),
            }
            Toolbar { labelledby: "example-toolbar-label",
                ToolbarGroup {
                    {toolbar_controls!(context)}
                }
            }
        }
    }),
    example!(
        r"Toolbar { glass: true, … }",
        Key::ExampleGlassBehind,
        |context| rsx! {
            Toolbar { glass: true, label: context.t(Key::ExampleToolbarLabel),
                ToolbarGroup {
                    {toolbar_controls!(context)}
                }
            }
        }
    ),
];

const TOOLBAR_GROUP: &[Example] = &[
    example!(
        r"ToolbarGroup { … }
ToolbarGroup { align: ToolbarGroupAlign::End, … }",
        Key::ExampleToolbarAlign,
        |context| rsx! {
            // NO `align: Start` ON THE FIRST GROUP. `start` is the default, so
            // writing it showed the prop doing nothing while the caption
            // claimed `End`. The value that does the work is on the group it
            // moves.
            Toolbar { label: context.t(Key::ExampleToolbarLabel),
                ToolbarGroup {
                    {toolbar_controls!(context)}
                }
                ToolbarGroup { align: ToolbarGroupAlign::End,
                    {toolbar_controls!(context)}
                }
            }
        }
    ),
    example!(r"ToolbarGroup { … }", |context| rsx! {
        Toolbar { label: context.t(Key::ExampleToolbarLabel),
            ToolbarGroup {
                {toolbar_controls!(context)}
            }
        }
    }),
    example!(r#"ToolbarGroup { label: "…", … }"#, |context| rsx! {
        Toolbar { label: context.t(Key::ExampleToolbarLabel),
            ToolbarGroup { label: context.t(Key::ExampleToolbarView),
                {toolbar_controls!(context)}
            }
        }
    }),
    // THREE GROUPS, AND ONE `align` PLACES TWO OF THEM. Example 1 is two
    // groups at the two ends and this used to be the identical arrangement
    // with different buttons in it. The claim worth its own example is the
    // rest of the sentence in the contract: `end` moves this group **and
    // everything after it**.
    example!(
        r"ToolbarGroup { … } × 3, with align: ToolbarGroupAlign::End on the second",
        Key::ExampleToolbarAlignRest,
        |context| rsx! {
            Toolbar { label: context.t(Key::ExampleToolbarLabel),
                ToolbarGroup {
                    Button {
                        variant: ButtonVariant::Outline,
                        tone: ButtonTone::Neutral,
                        size: ButtonSize::Small,
                        label: context.t(Key::ExampleGroupPrevious),
                    }
                }
                ToolbarGroup { align: ToolbarGroupAlign::End,
                    Button {
                        variant: ButtonVariant::Outline,
                        tone: ButtonTone::Neutral,
                        size: ButtonSize::Small,
                        label: context.t(Key::ExampleGroupNext),
                    }
                }
                ToolbarGroup {
                    {toolbar_controls!(context)}
                }
            }
        }
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
    example!(
        r#"Swatch { }  ·  Text { "…" }"#,
        Key::ExampleSwatchLive,
        |context| rsx! {
            span { class: "nd-example-byline",
                Swatch {}
                Text { "{context.t(Key::ExampleSwatchInForce)}" }
            }
        }
    ),
    example!(
        r#"Tag { swatch: "rose-vivid", label: "…" }"#,
        Key::ExampleSwatchNamed,
        |context| rsx! {
            div { class: "nd-example-row",
                // `Tag` draws a real `Swatch` before the text, which is the
                // shortest correct way to use one: the word is the name and
                // the chip is the colour of it.
                for (palette, label) in [
                    ("rose-vivid", Key::ThemeAccentRose),
                    ("jade-balanced", Key::ThemeAccentJade),
                    ("indigo-sober", Key::ThemeAccentIndigo),
                ] {
                    Tag {
                        key: "{palette}",
                        swatch: palette,
                        variant: TagVariant::Outline,
                        label: context.t(label),
                    }
                }
            }
        }
    ),
    example!(
        r#"Swatch { size: SwatchSize::…, palette: "amber-vivid" }  ·  Code { "…" }"#,
        |_| rsx! {
            div { class: "nd-example-row",
                for size in SwatchSize::ALL {
                    span { key: "{size.as_str()}", class: "nd-example-byline",
                        Swatch { size, palette: "amber-vivid" }
                        Code { class: "nd-identifier", "{size.as_str()}" }
                    }
                }
            }
        }
    ),
    example!(
        r#"Swatch { shape: SwatchShape::…, size: SwatchSize::Large, palette: "teal-balanced" }  ·  Code { "…" }"#,
        |_| rsx! {
            div { class: "nd-example-row",
                for shape in SwatchShape::ALL {
                    span { key: "{shape.as_str()}", class: "nd-example-byline",
                        Swatch { shape, size: SwatchSize::Large, palette: "teal-balanced" }
                        Code { class: "nd-identifier", "{shape.as_str()}" }
                    }
                }
            }
        }
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
        Key::ExampleDateInputBirthNote,
        |context| rsx! {
            // `novalidate`, so the sentence a reader hears is the
            // component's own rather than the browser bubble the no-script
            // path already has.
            form { class: "nd-example-form", method: "get", action: "#", novalidate: true,
                DateInput {
                    name: "birth",
                    legend: context.t(Key::ExampleDateInputBirthLegend),
                    hint: context.t(Key::ExampleDateInputHint),
                    day_label: context.t(Key::ExampleDateInputDay),
                    month_label: context.t(Key::ExampleDateInputMonth),
                    year_label: context.t(Key::ExampleDateInputYear),
                    bday: true,
                    required: true,
                    error_label: context.t(Key::ExampleDateInputErrorWord),
                    required_label: context.t(Key::ExampleDateInputRequired),
                    missing_label: context.t(Key::ExampleDateInputMissing),
                    number_label: context.t(Key::ExampleDateInputNumber),
                    invalid_label: context.t(Key::ExampleDateInputReal),
                    year_digits_label: context.t(Key::ExampleDateInputYearDigits),
                }
                // A form of three boxes and no submit control never submits
                // implicitly — the button is what makes Enter mean send.
                Button {
                    submit: ButtonSubmit::Submit,
                    label: context.t(Key::ExampleDateInputSend),
                }
            }
        }
    ),
    example!(
        r"DateInput { min, max, min_label, max_label, year_digits_label, calendar: true, suggest_label, previous_label, next_label, Time { … } }",
        Key::ExampleDateInputVisitNote,
        |context| rsx! {
            DateInput {
                name: "visit",
                legend: context.t(Key::ExampleDateInputVisitLegend),
                day_label: context.t(Key::ExampleDateInputDay),
                month_label: context.t(Key::ExampleDateInputMonth),
                year_label: context.t(Key::ExampleDateInputYear),
                min: "2026-01-01",
                max: "2027-12-31",
                error_label: context.t(Key::ExampleDateInputErrorWord),
                missing_label: context.t(Key::ExampleDateInputMissing),
                number_label: context.t(Key::ExampleDateInputNumber),
                invalid_label: context.t(Key::ExampleDateInputReal),
                year_digits_label: context.t(Key::ExampleDateInputYearDigits),
                min_label: context.t(Key::ExampleDateInputMin),
                max_label: context.t(Key::ExampleDateInputMax),
                calendar: true,
                suggest_label: context.t(Key::ExampleDateInputSuggest),
                previous_label: context.t(Key::ExampleDateInputPrevious),
                next_label: context.t(Key::ExampleDateInputNext),
                Time { datetime: "2026-12-25", "{context.t(Key::ExampleDateInputChristmas)}" }
                Time { datetime: "2027-01-01", "{context.t(Key::ExampleDateInputNewYear)}" }
            }
        }
    ),
    example!(
        r#"DateInput { value: "2026-08-07", order: Ymd, pad: true, expand: true }"#,
        Key::ExampleDateInputFilledNote,
        |context| rsx! {
            DateInput {
                name: "issued",
                legend: context.t(Key::ExampleDateInputIssuedLegend),
                day_label: context.t(Key::ExampleDateInputDay),
                month_label: context.t(Key::ExampleDateInputMonth),
                year_label: context.t(Key::ExampleDateInputYear),
                value: "2026-08-07",
                order: DateInputOrder::Ymd,
                pad: true,
                expand: true,
            }
        }
    ),
    example!(
        r"DateInput { size: Small }  ·  { size: Large, glass: true }  ·  { disabled: true }  ·  { error }",
        Key::ExampleDateInputFormsNote,
        |context| rsx! {
            DateInput {
                name: "compact",
                legend: context.t(Key::ExampleDateInputSmall),
                day_label: context.t(Key::ExampleDateInputDay),
                month_label: context.t(Key::ExampleDateInputMonth),
                year_label: context.t(Key::ExampleDateInputYear),
                size: DateInputSize::Small,
            }
            DateInput {
                name: "roomy",
                legend: context.t(Key::ExampleDateInputLarge),
                day_label: context.t(Key::ExampleDateInputDay),
                month_label: context.t(Key::ExampleDateInputMonth),
                year_label: context.t(Key::ExampleDateInputYear),
                size: DateInputSize::Large,
                glass: true,
            }
            DateInput {
                name: "closed",
                legend: context.t(Key::ExampleDateInputClosedLegend),
                day_label: context.t(Key::ExampleDateInputDay),
                month_label: context.t(Key::ExampleDateInputMonth),
                year_label: context.t(Key::ExampleDateInputYear),
                disabled: true,
            }
            DateInput {
                name: "booked",
                legend: context.t(Key::ExampleDateInputVisitLegend),
                day_label: context.t(Key::ExampleDateInputDay),
                month_label: context.t(Key::ExampleDateInputMonth),
                year_label: context.t(Key::ExampleDateInputYear),
                value: "2025-02-14",
                error: context.t(Key::ExampleDateInputErrorMessage),
                error_label: context.t(Key::ExampleDateInputErrorWord),
            }
        }
    ),
];

const CHECKBOX: &[Example] = &[
    example!(
        r#"Fieldset { … } / Checkbox { name: "apply", value: "study", label: "…" }"#,
        |context| rsx! {
            Fieldset {
                name: "apply",
                legend: context.t(Key::ExampleFormsApplyLegend),
                hint: context.t(Key::ExampleFormsApplyHint),
                Checkbox {
                    name: "apply",
                    value: "study",
                    label: context.t(Key::ExampleFormsApplyStudy),
                }
                Checkbox {
                    name: "apply",
                    value: "work",
                    label: context.t(Key::ExampleFormsApplyWork),
                    checked: true,
                }
                Checkbox {
                    name: "apply",
                    value: "care",
                    label: context.t(Key::ExampleFormsApplyCare),
                }
            }
        }
    ),
    // THE THREE SIZES AS REAL CONTROLS, not only as specimens in the variant
    // grid. The grid draws the box alone, which is what makes the prop legible
    // — but the claim the sizes exist for is that the *target* stays 44 pixels
    // while the box shrinks, and a drawn box with no input behind it cannot
    // show that. It is also what the browser suite measures.
    example!(
        r"Checkbox { size: CheckboxSize::Large | Medium | Small }",
        Key::ExampleFormsNoteSizes,
        |context| rsx! {
            // THE VALUE IS THE ANSWER, NEVER THE SIZE. Two boxes posting
            // `large` and `medium` beside a third posting `care` is a paste
            // slip, and it teaches that `value` has something to do with the
            // prop this example is about.
            Fieldset { name: "apply-sizes", legend: context.t(Key::ExampleFormsApplyLegend),
                Checkbox {
                    name: "apply-sizes",
                    value: "study",
                    label: context.t(Key::ExampleFormsApplyStudy),
                    size: CheckboxSize::Large,
                }
                Checkbox {
                    name: "apply-sizes",
                    value: "work",
                    label: context.t(Key::ExampleFormsApplyWork),
                    size: CheckboxSize::Medium,
                }
                Checkbox {
                    name: "apply-sizes",
                    value: "care",
                    label: context.t(Key::ExampleFormsApplyCare),
                    size: CheckboxSize::Small,
                }
            }
        }
    ),
    example!(
        r#"Checkbox { hint: "…", size: CheckboxSize::Small }"#,
        Key::ExampleFormsNoteHints,
        |context| rsx! {
            Fieldset {
                name: "apply-hints",
                legend: context.t(Key::ExampleFormsApplyLegend),
                Checkbox {
                    name: "apply-hints",
                    value: "study",
                    label: context.t(Key::ExampleFormsApplyStudy),
                    hint: context.t(Key::ExampleFormsApplyStudyHint),
                    size: CheckboxSize::Small,
                }
                Checkbox {
                    name: "apply-hints",
                    value: "work",
                    label: context.t(Key::ExampleFormsApplyWork),
                    hint: context.t(Key::ExampleFormsApplyWorkHint),
                    size: CheckboxSize::Small,
                }
            }
        }
    ),
    example!(
        r#"Checkbox { divider: "…", exclusive: true }"#,
        Key::ExampleFormsNoteExclusive,
        |context| rsx! {
            Fieldset {
                name: "apply-none",
                legend: context.t(Key::ExampleFormsApplyLegend),
                Checkbox {
                    name: "apply-none",
                    value: "study",
                    label: context.t(Key::ExampleFormsApplyStudy),
                }
                Checkbox {
                    name: "apply-none",
                    value: "work",
                    label: context.t(Key::ExampleFormsApplyWork),
                }
                Checkbox {
                    name: "apply-none",
                    value: "none",
                    label: context.t(Key::ExampleFormsApplyNone),
                    divider: context.t(Key::ExampleFormsOr),
                    exclusive: true,
                }
            }
        }
    ),
    example!(
        r"Checkbox { reveal: true, … }",
        Key::ExampleFormsNoteReveal,
        |context| rsx! {
            Fieldset {
                name: "apply-other",
                legend: context.t(Key::ExampleFormsApplyLegend),
                Checkbox {
                    name: "apply-other",
                    value: "care",
                    label: context.t(Key::ExampleFormsApplyCare),
                }
                Checkbox {
                    name: "apply-other",
                    value: "other",
                    label: context.t(Key::ExampleFormsApplyOther),
                    reveal: true,
                    Paragraph { size: ParagraphSize::Small,
                        "{context.t(Key::ExampleFormsApplyOtherDetail)}"
                    }
                }
            }
        }
    ),
    example!(
        r"Checkbox { checked: true, disabled: true, invalid: true, hide_label }",
        Key::ExampleFormsNoteTarget,
        |context| rsx! {
            Fieldset {
                name: "apply-states",
                legend: context.t(Key::ExampleFormsApplyLegend),
                Checkbox {
                    name: "apply-states",
                    value: "study",
                    label: context.t(Key::ExampleFormsApplyStudy),
                    checked: true,
                }
                Checkbox {
                    name: "apply-states",
                    value: "work",
                    label: context.t(Key::ExampleFormsApplyWork),
                    invalid: true,
                }
                Checkbox {
                    name: "apply-states",
                    value: "care",
                    label: context.t(Key::ExampleFormsApplyCare),
                    hint: context.t(Key::ExampleFormsContactPostHint),
                    disabled: true,
                }
            }
        }
    ),
];

const RADIO: &[Example] = &[
    example!(
        r#"Fieldset { … } / Radio { name: "contact", value: "email", label: "…", checked: true }"#,
        |context| rsx! {
            Fieldset {
                name: "contact",
                legend: context.t(Key::ExampleFormsContactLegend),
                hint: context.t(Key::ExampleFormsContactHint),
                Radio {
                    name: "contact",
                    value: "email",
                    label: context.t(Key::ExampleFormsContactEmail),
                    checked: true,
                }
                Radio {
                    name: "contact",
                    value: "phone",
                    label: context.t(Key::ExampleFormsContactPhone),
                }
                Radio {
                    name: "contact",
                    value: "text",
                    label: context.t(Key::ExampleFormsContactText),
                }
            }
        }
    ),
    // The three sizes as real controls. See `Checkbox` above for why the
    // variant grid is not enough on its own.
    example!(
        r"Radio { size: RadioSize::Large | Medium | Small }",
        Key::ExampleFormsNoteSizes,
        |context| rsx! {
            // THE VALUE IS THE ANSWER, NEVER THE SIZE. See `Checkbox` above.
            Fieldset { name: "contact-sizes", legend: context.t(Key::ExampleFormsContactLegend),
                Radio {
                    name: "contact-sizes",
                    value: "email",
                    label: context.t(Key::ExampleFormsContactEmail),
                    size: RadioSize::Large,
                }
                Radio {
                    name: "contact-sizes",
                    value: "phone",
                    label: context.t(Key::ExampleFormsContactPhone),
                    size: RadioSize::Medium,
                }
                Radio {
                    name: "contact-sizes",
                    value: "post",
                    label: context.t(Key::ExampleFormsContactPost),
                    size: RadioSize::Small,
                }
            }
        }
    ),
    example!(
        r#"Radio { hint: "…", size: RadioSize::Medium }"#,
        Key::ExampleFormsNoteHints,
        |context| rsx! {
            Fieldset {
                name: "contact-hints",
                legend: context.t(Key::ExampleFormsContactLegend),
                Radio {
                    name: "contact-hints",
                    value: "email",
                    label: context.t(Key::ExampleFormsContactEmail),
                    hint: context.t(Key::ExampleFormsContactEmailHint),
                    size: RadioSize::Medium,
                }
                Radio {
                    name: "contact-hints",
                    value: "phone",
                    label: context.t(Key::ExampleFormsContactPhone),
                    hint: context.t(Key::ExampleFormsContactPhoneHint),
                    size: RadioSize::Medium,
                }
            }
        }
    ),
    example!(
        r#"Radio { divider: "…", reveal: true, … }"#,
        Key::ExampleFormsNoteDivider,
        |context| rsx! {
            Fieldset {
                name: "contact-other",
                legend: context.t(Key::ExampleFormsContactLegend),
                Radio {
                    name: "contact-other",
                    value: "email",
                    label: context.t(Key::ExampleFormsContactEmail),
                }
                Radio {
                    name: "contact-other",
                    value: "other",
                    label: context.t(Key::ExampleFormsContactOther),
                    divider: context.t(Key::ExampleFormsOr),
                    reveal: true,
                    Checkbox {
                        name: "contact-copy",
                        value: "yes",
                        label: context.t(Key::ExampleFormsContactCopy),
                        size: CheckboxSize::Small,
                    }
                }
            }
        }
    ),
    example!(
        r"Radio { disabled: true, invalid: true, hide_label }",
        Key::ExampleFormsNoteTarget,
        |context| rsx! {
            Fieldset {
                name: "contact-states",
                legend: context.t(Key::ExampleFormsContactLegend),
                Radio {
                    name: "contact-states",
                    value: "email",
                    label: context.t(Key::ExampleFormsContactEmail),
                    invalid: true,
                }
                Radio {
                    name: "contact-states",
                    value: "post",
                    label: context.t(Key::ExampleFormsContactPost),
                    hint: context.t(Key::ExampleFormsContactPostHint),
                    disabled: true,
                }
            }
        }
    ),
];

const FIELDSET: &[Example] = &[
    example!(
        r#"Fieldset { name: "…", legend: "…", hint: "…" }"#,
        |context| rsx! {
            Fieldset {
                name: "notify",
                legend: context.t(Key::ExampleFormsContactLegend),
                hint: context.t(Key::ExampleFormsContactHint),
                Radio {
                    name: "notify",
                    value: "email",
                    label: context.t(Key::ExampleFormsContactEmail),
                }
                Radio {
                    name: "notify",
                    value: "text",
                    label: context.t(Key::ExampleFormsContactText),
                }
            }
        }
    ),
    example!(
        r#"Fieldset { error: "…", error_label: "…" }"#,
        Key::ExampleFormsNoteError,
        |context| rsx! {
            Fieldset {
                name: "terms",
                legend: context.t(Key::ExampleFormsTermsLegend),
                error: context.t(Key::ExampleFormsTermsError),
                error_label: context.t(Key::ExampleFormsErrorWord),
                Checkbox {
                    name: "terms",
                    value: "yes",
                    label: context.t(Key::ExampleFormsTermsAgree),
                    invalid: true,
                }
            }
        }
    ),
    example!(
        r"Fieldset { inline: true, … }",
        Key::ExampleFormsNoteInline,
        |context| rsx! {
            Fieldset {
                name: "notify-inline",
                legend: context.t(Key::ExampleFormsNotifyLegend),
                inline: true,
                Radio {
                    name: "notify-inline",
                    value: "yes",
                    label: context.t(Key::ExampleFormsYes),
                    size: RadioSize::Medium,
                }
                Radio {
                    name: "notify-inline",
                    value: "no",
                    label: context.t(Key::ExampleFormsNo),
                    size: RadioSize::Medium,
                }
            }
        }
    ),
    example!(
        r"Fieldset { heading: true, … }",
        Key::ExampleFormsNoteHeading,
        |context| rsx! {
            Fieldset {
                name: "notify-heading",
                legend: context.t(Key::ExampleFormsNotifyLegend),
                heading: true,
                Radio {
                    name: "notify-heading",
                    value: "yes",
                    label: context.t(Key::ExampleFormsYes),
                }
                Radio {
                    name: "notify-heading",
                    value: "no",
                    label: context.t(Key::ExampleFormsNo),
                }
            }
        }
    ),
    example!(
        r"Fieldset { disabled: true, … }",
        Key::ExampleFormsNoteDisabled,
        |context| rsx! {
            Fieldset {
                name: "closed",
                legend: context.t(Key::ExampleFormsClosedLegend),
                hint: context.t(Key::ExampleFormsClosedHint),
                disabled: true,
                Checkbox {
                    name: "closed",
                    value: "study",
                    label: context.t(Key::ExampleFormsApplyStudy),
                    checked: true,
                }
                Checkbox {
                    name: "closed",
                    value: "work",
                    label: context.t(Key::ExampleFormsApplyWork),
                }
            }
        }
    ),
];

// --- Info -------------------------------------------------------------------

const INFO: &[Example] = &[
    example!(
        r"Info { name, label } beside the word it explains",
        Key::ExampleInfoDeadlineHint,
        |context| rsx! {
            Text {
                "{context.t(Key::ExampleInfoDeadline)}"
                Info {
                    name: "deadline",
                    label: context.t(Key::ExampleInfoDeadlineLabel),
                    {context.t(Key::ExampleInfoDeadlineNote)}
                }
            }
        }
    ),
    example!(
        r"Info { heading }",
        Key::ExampleInfoUsageHint,
        |context| rsx! {
            Text {
                "{context.t(Key::ExampleInfoUsage)}"
                Info {
                    name: "usage",
                    label: context.t(Key::ExampleInfoUsageLabel),
                    heading: context.t(Key::ExampleInfoUsageHeading),
                    {context.t(Key::ExampleInfoUsageNote)}
                }
            }
        }
    ),
    example!(
        r"Info { close_label }",
        Key::ExampleInfoCloseHint,
        |context| rsx! {
            Text {
                "{context.t(Key::ExampleInfoDeadline)}"
                Info {
                    name: "closable",
                    label: context.t(Key::ExampleInfoDeadlineLabel),
                    close_label: context.t(Key::ExampleInfoClose),
                    {context.t(Key::ExampleInfoDeadlineNote)}
                }
            }
        }
    ),
    example!(
        r"Info { placement: InfoPlacement::End } at the edge of a table",
        |context| rsx! {
            div { class: "nd-example-row nd-example-row-end",
                Text {
                    "{context.t(Key::ExampleInfoColumn)}"
                    Info {
                        name: "balance",
                        label: context.t(Key::ExampleInfoColumnLabel),
                        placement: InfoPlacement::End,
                        close_label: context.t(Key::ExampleInfoClose),
                        {context.t(Key::ExampleInfoColumnNote)}
                    }
                }
            }
        }
    ),
];

// --- Meter and Progress -----------------------------------------------------

// REAL READINGS. A disk that is 82% full, a token quota past its threshold, a
// team with four seats left. A bar of invented numbers is a bar nobody can tell
// is wrong, and this component's whole claim is that the number is written out.

const METER: &[Example] = &[
    example!(
        r"Meter { name, label, value, max, unit, value_label }",
        Key::ExampleMeterStorageNote,
        |context| rsx! {
            Meter {
                name: "storage",
                label: context.t(Key::ExampleMeterStorage),
                value: "41.2",
                max: "50",
                unit: "GB",
                value_label: context.t(Key::ExampleMeterStorageValue),
            }
        }
    ),
    example!(
        r"Meter { name, label, value, max, low, high, optimum, value_label }",
        Key::ExampleMeterTokensNote,
        |context| rsx! {
            div { class: "nd-example-stack",
                // ONE READING PER BAND, which is the only way three thresholds
                // can be seen to do anything: under `low`, between the two, and
                // over `high`.
                //
                // AND ONE LABEL PER BAR. Three bars under one name is three
                // bars nobody can use, because a reader hearing the same
                // sentence three times has no way to tell them apart.
                for (name, value, label) in [
                    ("tokens-low", "48", Key::ExampleMeterTokensAtlas),
                    ("tokens-warn", "121", Key::ExampleMeterTokensBeacon),
                    ("tokens-over", "194", Key::ExampleMeterTokensCorvus),
                ] {
                    Meter {
                        key: "{name}",
                        name,
                        label: context.t(label),
                        value,
                        max: "200",
                        // Low is good on a quota, so `optimum` sits under
                        // `low` and the platform reads the range downward.
                        low: "60",
                        high: "160",
                        optimum: "0",
                        value_label: context.t(Key::ExampleMeterTokensValue),
                    }
                }
            }
        }
    ),
    example!(
        r"Meter { name, label, value, min, max, unit, size, value_label }",
        Key::ExampleMeterFreezerNote,
        |context| rsx! {
            // A SCALE THAT STARTS BELOW ZERO, which is the only thing `min`
            // is for. `min: "0"` is what absence already means, so a meter
            // declaring it shows the prop doing nothing.
            Meter {
                name: "freezer",
                label: context.t(Key::ExampleMeterFreezer),
                value: "-4",
                min: "-20",
                max: "10",
                unit: "°C",
                size: MeterSize::Small,
                value_label: context.t(Key::ExampleMeterFreezerValue),
            }
        }
    ),
    example!(
        r#"Meter { name, label, value, tone: "…", size: MeterSize::Large, value_label }"#,
        Key::ExampleMeterShareNote,
        |context| rsx! {
            div { class: "nd-example-stack",
                // THREE CATEGORIES, THREE NAMES, THREE TONES A READER CAN TELL
                // APART. Three bars with one label is three bars nobody can
                // use, and two blues is one colour.
                //
                // AND THREE TONES THAT NAME A CATEGORY. `info`, `success` and
                // `warning` are verdicts, so painting a share of a disk with
                // them says one slice is good and another is trouble. These
                // three name what the bytes ARE, which is the whole point of a
                // tone on a bar with no good end.
                for (name, value, tone, label) in [
                    ("share-cached", "0.42", "cached", Key::ExampleMeterShareCached),
                    ("share-archived", "0.35", "archived", Key::ExampleMeterShareArchived),
                    ("share-shared", "0.23", "shared", Key::ExampleMeterShareShared),
                ] {
                    Meter {
                        key: "{name}",
                        name,
                        label: context.t(label),
                        value,
                        tone,
                        size: MeterSize::Large,
                        value_label: context.t(Key::ExampleMeterShareValue),
                    }
                }
            }
        }
    ),
    example!(
        r"Meter { name, label, value, max, unit, hide_label: true, show_value: false, size, value_label }",
        Key::ExampleMeterHiddenNote,
        |context| rsx! {
            // A READING OF ITS OWN. Two bars showing the same number is two
            // pictures of one thing, and a reader comparing this page's
            // examples has no way to tell which one they are hearing.
            Meter {
                name: "bare",
                label: context.t(Key::ExampleMeterStorage),
                value: "8.6",
                max: "50",
                unit: "GB",
                hide_label: true,
                show_value: false,
                size: MeterSize::Small,
                value_label: context.t(Key::ExampleMeterStorageValue),
            }
        }
    ),
];

const PROGRESS: &[Example] = &[
    example!(
        r"Progress { name, label, value, max, value_label }",
        Key::ExampleProgressUploadNote,
        |context| rsx! {
            Progress {
                name: "upload",
                label: context.t(Key::ExampleProgressUpload),
                value: "7.4",
                max: "12",
                value_label: context.t(Key::ExampleProgressUploadValue),
            }
        }
    ),
    example!(
        r"Progress { busy_label } — no value at all",
        Key::ExampleProgressImportNote,
        |context| rsx! {
            Progress {
                name: "import",
                label: context.t(Key::ExampleProgressImport),
                busy_label: context.t(Key::ExampleProgressImportBusy),
            }
        }
    ),
    example!(
        r#"Progress { tone: "danger" }"#,
        Key::ExampleProgressFailedNote,
        |context| rsx! {
            Progress {
                name: "failed",
                label: context.t(Key::ExampleProgressFailed),
                value: "0.38",
                tone: "danger",
                value_label: context.t(Key::ExampleProgressFailedValue),
            }
        }
    ),
    example!(
        r"Progress { name, label, value, max, size: Large, value_label }
Progress { name, label, value, max, size: Small, hide_label: true, value_label }",
        Key::ExampleProgressStepsNote,
        |context| rsx! {
            div { class: "nd-example-stack",
                Progress {
                    name: "steps-large",
                    label: context.t(Key::ExampleProgressSteps),
                    value: "3200",
                    max: "4200",
                    size: ProgressSize::Large,
                    value_label: context.t(Key::ExampleProgressStepsValue),
                }
                Progress {
                    name: "steps-small",
                    label: context.t(Key::ExampleProgressSteps),
                    value: "3200",
                    max: "4200",
                    size: ProgressSize::Small,
                    hide_label: true,
                    value_label: context.t(Key::ExampleProgressStepsValue),
                }
            }
        }
    ),
];

// --- Pagination -------------------------------------------------------------

/// A `const` rather than a literal in the `rsx!`, and that is not style: the
/// macro reads a string literal as a FORMAT string, so `{page}` inside one is an
/// interpolation of a variable that does not exist. Same reason as
/// `BUTTON_SOURCE` in `pages.rs`.
const PAGE_HREF: &str = "#page-{page}";

const PAGINATION: &[Example] = &[
    example!(
        r"Pagination { name, label, page, pages, href, previous_label, next_label, page_label, current_label }",
        Key::ExamplePaginationNoteLinks,
        |context| rsx! {
            Pagination {
                name: "palettes",
                label: context.t(Key::ExamplePaginationLabel),
                // THIRTY-NINE PALETTES, TEN A PAGE, FOUR PAGES. The overview
                // and the last example on this page count the same set, so a
                // reader moving between them is reading one story.
                page: "1",
                pages: "4",
                href: PAGE_HREF,
                // BOTH WORDS ARE PASSED AND ONLY ONE CONTROL IS DRAWN, which
                // is the claim. Without `previous_label` the missing backward
                // link proves nothing, because a control with no name is not
                // rendered whatever page it sits on.
                previous_label: context.t(Key::ExamplePaginationPrevious),
                next_label: context.t(Key::ExamplePaginationNext),
                page_label: context.t(Key::ExamplePaginationPage),
                current_label: context.t(Key::ExamplePaginationCurrent),
            }
        }
    ),
    example!(
        r"Pagination { spread: PaginationSpread::Compact, previous_label, next_label }",
        Key::ExamplePaginationNoteWindow,
        |context| rsx! {
            Pagination {
                name: "palettes-compact",
                label: context.t(Key::ExamplePaginationLabel),
                page: "12",
                pages: "40",
                href: PAGE_HREF,
                spread: PaginationSpread::Compact,
                previous_label: context.t(Key::ExamplePaginationPrevious),
                next_label: context.t(Key::ExamplePaginationNext),
                page_label: context.t(Key::ExamplePaginationPage),
                current_label: context.t(Key::ExamplePaginationCurrent),
            }
        }
    ),
    example!(
        r"Pagination { spread: PaginationSpread::Wide, size: PaginationSize::Small }",
        |context| rsx! {
            Pagination {
                name: "palettes-wide",
                label: context.t(Key::ExamplePaginationLabel),
                page: "20",
                pages: "40",
                href: PAGE_HREF,
                spread: PaginationSpread::Wide,
                size: PaginationSize::Small,
                previous_label: context.t(Key::ExamplePaginationPrevious),
                next_label: context.t(Key::ExamplePaginationNext),
                page_label: context.t(Key::ExamplePaginationPage),
                current_label: context.t(Key::ExamplePaginationCurrent),
            }
        }
    ),
    example!(
        r"At the last page, with a summary",
        Key::ExamplePaginationNoteEnds,
        |context| rsx! {
            Pagination {
                name: "palettes-last",
                label: context.t(Key::ExamplePaginationLabel),
                // A CONTROL WHOSE NUMBERS DISAGREE WITH ITS OWN SENTENCE IS
                // ONE NOBODY TRUSTS. Page 9 of 9 under *showing 21 to 30 of
                // 39* is arithmetic a reader can do and the page cannot.
                page: "4",
                pages: "4",
                href: PAGE_HREF,
                size: PaginationSize::Large,
                previous_label: context.t(Key::ExamplePaginationPrevious),
                next_label: context.t(Key::ExamplePaginationNext),
                page_label: context.t(Key::ExamplePaginationPage),
                current_label: context.t(Key::ExamplePaginationCurrent),
                summary: context.t(Key::ExamplePaginationSummaryLast),
            }
        }
    ),
];

// --- Table, and its five parts ----------------------------------------------

// FOUR REAL DATASETS, AND NOT ONE TOKEN NAME.
//
// The version before this one showed the design system's own spacing scale in
// all thirty examples — the same caption thirty times, and three of the rows
// invented a `--noctua-space-5` that does not exist. A reader learning `Table`
// was reading a worse copy of `/spacing`, and could not tell which of the words
// on the screen were the table's props and which were the system's tokens.
//
// These are invoices, quotas, people and deployments. They are the four shapes
// a table is actually asked for, and each one earns the props it demonstrates:
// money wants alignment and totals, quotas want a bar in a cell, people want
// selection, and deployments want the long status palette and a fold.

/// One invoice, since a dozen rows differ only in five strings.
macro_rules! invoice {
    ($context:ident, $id:literal, $client:literal, $due:literal, $iso:literal, $status:expr, $tone:literal, $amount:literal, $sort:literal) => {
        rsx! {
            TableRow {
                TableHeaderCell {
                    scope: TableHeaderCellScope::Row,
                    column: "invoice",
                    class: "nd-identifier",
                    $id
                }
                // EVERY CELL CARRIES ITS COLUMN'S NAME. The stacked layout has
                // no heading row left, and a value with no label under it is a
                // run of data a reader has to guess the shape of.
                TableCell {
                    column: "client",
                    class: "nd-identifier",
                    label: $context.t(Key::ExampleTableColClient),
                    $client
                }
                TableCell {
                    column: "due",
                    class: "nd-identifier",
                    label: $context.t(Key::ExampleTableColDue),
                    sort_value: $iso,
                    $due
                }
                TableCell { column: "status", label: $context.t(Key::ExampleTableColStatus),
                    Tag { label: $context.t($status), tone: $tone, size: TagSize::Small }
                }
                TableCell {
                    column: "amount",
                    class: "nd-identifier",
                    label: $context.t(Key::ExampleTableColAmount),
                    align: TableCellAlign::Numeric,
                    sort_value: $sort,
                    $amount
                }
            }
        }
    };
}

/// The five columns the invoice tables share.
macro_rules! invoice_columns {
    () => {
        rsx! {
            TableColumn { name: "invoice", width: TableColumnWidth::Fit }
            TableColumn { name: "client" }
            TableColumn { name: "due", width: TableColumnWidth::Fit }
            TableColumn { name: "status", width: TableColumnWidth::Fit }
            TableColumn { name: "amount", width: TableColumnWidth::Fit }
        }
    };
}

macro_rules! invoice_headings {
    ($context:ident) => {
        rsx! {
            TableSection { kind: TableSectionKind::Head,
                TableRow {
                    TableHeaderCell { column: "invoice", {$context.t(Key::ExampleTableColInvoice)} }
                    TableHeaderCell { column: "client", {$context.t(Key::ExampleTableColClient)} }
                    TableHeaderCell { column: "due", {$context.t(Key::ExampleTableColDue)} }
                    TableHeaderCell { column: "status", {$context.t(Key::ExampleTableColStatus)} }
                    TableHeaderCell {
                        column: "amount",
                        align: TableHeaderCellAlign::Numeric,
                        {$context.t(Key::ExampleTableColAmount)}
                    }
                }
            }
        }
    };
}

/// One deployment, since a dozen rows differ only in five strings.
macro_rules! deploy {
    ($context:ident, $build:expr, $branch:literal, $where:expr, $took:literal, $result:expr, $tone:literal) => {
        rsx! {
            TableRow {
                TableHeaderCell {
                    scope: TableHeaderCellScope::Row,
                    column: "build",
                    class: "nd-identifier",
                    "#"
                    $build
                }
                // See `invoice!`: every cell carries its column's name, for the
                // stacked layout that has no heading row left.
                TableCell {
                    column: "branch",
                    class: "nd-identifier",
                    label: $context.t(Key::ExampleTableColBranch),
                    wrap: TableCellWrap::Nowrap,
                    $branch
                }
                TableCell { column: "where", label: $context.t(Key::ExampleTableColWhere),
                    {$context.t($where)}
                }
                TableCell {
                    column: "took",
                    class: "nd-identifier",
                    label: $context.t(Key::ExampleTableColTook),
                    align: TableCellAlign::Numeric,
                    $took
                }
                TableCell { column: "result", label: $context.t(Key::ExampleTableColResult),
                    Tag { label: $context.t($result), tone: $tone, size: TagSize::Small }
                }
            }
        }
    };
}

macro_rules! deploy_columns {
    () => {
        rsx! {
            // NO `filterable` HERE. It says which columns the search box
            // reads, and **the box is what `Table { search_label }` adds**.
            // None of the seven tables built from these columns passes one,
            // so the flag reached nothing at all on any of them.
            TableColumn { name: "build", width: TableColumnWidth::Fit }
            TableColumn { name: "branch" }
            TableColumn { name: "where", width: TableColumnWidth::Fit }
            TableColumn { name: "took", width: TableColumnWidth::Fit }
            TableColumn { name: "result", width: TableColumnWidth::Fit }
        }
    };
}

macro_rules! deploy_headings {
    ($context:ident) => {
        rsx! {
            TableSection { kind: TableSectionKind::Head,
                TableRow {
                    TableHeaderCell { column: "build", {$context.t(Key::ExampleTableColBuild)} }
                    TableHeaderCell { column: "branch", {$context.t(Key::ExampleTableColBranch)} }
                    TableHeaderCell { column: "where", {$context.t(Key::ExampleTableColWhere)} }
                    TableHeaderCell {
                        column: "took",
                        align: TableHeaderCellAlign::Numeric,
                        {$context.t(Key::ExampleTableColTook)}
                    }
                    TableHeaderCell { column: "result", {$context.t(Key::ExampleTableColResult)} }
                }
            }
        }
    };
}

const TABLE_COLUMN: &[Example] = &[
    example!(
        r"TableColumn { low, high, tone_below, tone_within, tone_above }",
        Key::ExampleTableNoteStatus,
        |context| rsx! {
            Table {
                name: "column-bands",
                caption: context.t(Key::ExampleTableUsage),
                TableColumn { name: "project" }
                TableColumn {
                    name: "storage",
                    width: TableColumnWidth::Fit,
                    sortable: true,
                    sort_as: TableColumnSortAs::Number,
                    // One reading per band, which is the only way three
                    // thresholds can be seen to do anything.
                    low: "20",
                    high: "45",
                    tone_below: "healthy",
                    tone_within: "caution",
                    tone_above: "critical",
                }
                TableSection { kind: TableSectionKind::Head,
                    TableRow {
                        TableHeaderCell { column: "project", {context.t(Key::ExampleTableColProject)} }
                        TableHeaderCell {
                            column: "storage",
                            align: TableHeaderCellAlign::Numeric,
                            {context.t(Key::ExampleTableColStorage)}
                        }
                    }
                }
                TableSection {
                    for (project, shown, sort) in [
                        ("atlas", "12,4 GB", "12.4"),
                        ("borealis", "33,1 GB", "33.1"),
                        ("cygnus", "48,6 GB", "48.6"),
                    ] {
                        TableRow { key: "{project}",
                            TableHeaderCell {
                                scope: TableHeaderCellScope::Row,
                                column: "project",
                                class: "nd-identifier",
                                "{project}"
                            }
                            TableCell {
                                column: "storage",
                                class: "nd-identifier",
                                align: TableCellAlign::Numeric,
                                sort_value: sort,
                                "{shown}"
                            }
                        }
                    }
                }
            }
        }
    ),
    example!(
        r"TableColumn { name, width: TableColumnWidth::Fit | ::Wide }",
        Key::ExampleTableNoteColumnWidth,
        |context| rsx! {
            Table {
                name: "column-width",
                caption: context.t(Key::ExampleTableDeploys),
                TableColumn { name: "build", width: TableColumnWidth::Fit }
                TableColumn { name: "branch", width: TableColumnWidth::Fit }
                TableColumn { name: "log" }
                TableSection { kind: TableSectionKind::Head,
                    TableRow {
                        TableHeaderCell { column: "build", {context.t(Key::ExampleTableColBuild)} }
                        TableHeaderCell { column: "branch", {context.t(Key::ExampleTableColBranch)} }
                        TableHeaderCell { column: "log", {context.t(Key::ExampleTableColResult)} }
                    }
                }
                TableSection {
                    for (build, branch) in [("4213", "main"), ("4211", "fix/anchor")] {
                        TableRow { key: "{build}",
                            TableHeaderCell {
                                scope: TableHeaderCellScope::Row,
                                column: "build",
                                class: "nd-identifier",
                                "#{build}"
                            }
                            TableCell { column: "branch", class: "nd-identifier", "{branch}" }
                            TableCell { column: "log", {context.t(Key::ExampleTableLog)} }
                        }
                    }
                }
            }
        }
    ),
    example!(
        r"TableColumn { sortable, sort_as: ::Number | ::Date }",
        Key::ExampleTableNoteColumnSort,
        |context| rsx! {
            Table {
                name: "column-sort",
                caption: context.t(Key::ExampleTableInvoices),
                sort_ascending_label: context.t(Key::ExampleTableSortAscending),
                sort_descending_label: context.t(Key::ExampleTableSortDescending),
                sort_cleared_label: context.t(Key::ExampleTableSortCleared),
                TableColumn { name: "invoice", width: TableColumnWidth::Fit, sortable: true }
                TableColumn { name: "client", sortable: true }
                TableColumn {
                    name: "due",
                    width: TableColumnWidth::Fit,
                    sortable: true,
                    sort_as: TableColumnSortAs::Date,
                }
                TableColumn { name: "status", width: TableColumnWidth::Fit }
                TableColumn {
                    name: "amount",
                    width: TableColumnWidth::Fit,
                    sortable: true,
                    sort_as: TableColumnSortAs::Number,
                }
                {invoice_headings!(context)}
                TableSection {
                    {invoice!(context, "INV-0045", "Cintra Studio", "24 Mar", "2026-03-24", Key::ExampleTableStatusPending, "pending", "R$ 680,00", "680")}
                    {invoice!(context, "INV-0043", "Aurora Ltd", "3 Mar", "2026-03-03", Key::ExampleTableStatusPaid, "paid", "R$ 4.900,00", "4900")}
                    {invoice!(context, "INV-0046", "Delgado SA", "29 Mar", "2026-03-29", Key::ExampleTableStatusRefunded, "refunded", "R$ 320,00", "320")}
                }
            }
        }
    ),
    example!(
        r"TableColumn { filterable, hideable, hidden }",
        Key::ExampleTableNoteColumnHide,
        |context| rsx! {
            Table {
                name: "column-hide",
                caption: context.t(Key::ExampleTableInvoices),
                search_label: context.t(Key::ExampleTableSearch),
                search_placeholder: context.t(Key::ExampleTableSearchPlaceholder),
                filtered_label: context.t(Key::ExampleTableFiltered),
                columns_label: context.t(Key::ExampleTableColumns),
                column_shown_label: context.t(Key::ExampleTableColumnShown),
                column_hidden_label: context.t(Key::ExampleTableColumnHidden),
                TableColumn { name: "invoice", width: TableColumnWidth::Fit, filterable: true }
                TableColumn { name: "client", filterable: true, hideable: true }
                TableColumn { name: "due", width: TableColumnWidth::Fit, hideable: true, hidden: true }
                TableColumn { name: "status", width: TableColumnWidth::Fit }
                TableColumn { name: "amount", width: TableColumnWidth::Fit, hideable: true }
                {invoice_headings!(context)}
                TableSection {
                    {invoice!(context, "INV-0043", "Aurora Ltd", "3 Mar", "2026-03-03", Key::ExampleTableStatusPaid, "paid", "R$ 4.900,00", "4900")}
                    {invoice!(context, "INV-0044", "Bracken & Co", "11 Mar", "2026-03-11", Key::ExampleTableStatusOverdue, "overdue", "R$ 1.240,00", "1240")}
                }
            }
        }
    ),
    example!(
        r"TableColumn { resizable, pinned } and the three status tones",
        Key::ExampleTableNoteDeploys,
        |context| rsx! {
            Table {
                name: "column-pin",
                caption: context.t(Key::ExampleTableDeploys),
                resize_label: context.t(Key::ExampleTableResize),
                resized_label: context.t(Key::ExampleTableResized),
                TableColumn { name: "build", width: TableColumnWidth::Fit, pinned: true }
                TableColumn { name: "branch", pinned: true, resizable: true }
                TableColumn { name: "where", width: TableColumnWidth::Fit }
                TableColumn { name: "took", width: TableColumnWidth::Fit }
                TableColumn { name: "result", width: TableColumnWidth::Fit }
                {deploy_headings!(context)}
                TableSection {
                    {deploy!(context, "4213", "main", Key::ExampleTableWhereProduction, "51s", Key::ExampleTableResultPassing, "passing")}
                    {deploy!(context, "4212", "release/2026-03", Key::ExampleTableWhereStaging, "48s", Key::ExampleTableResultRunning, "running")}
                    {deploy!(context, "4211", "fix/anchor", Key::ExampleTableWhereStaging, "12s", Key::ExampleTableResultFailed, "failed")}
                    {deploy!(context, "4209", "main", Key::ExampleTableWhereProduction, "0s", Key::ExampleTableResultQueued, "queued")}
                }
            }
        }
    ),
];

const TABLE_SECTION: &[Example] = &[
    example!(
        r"TableSection { kind: TableSectionKind::Head }",
        Key::ExampleTableNoteSectionHead,
        |context| rsx! {
            Table {
                name: "section-head",
                caption: context.t(Key::ExampleTableDeploys),
                {deploy_columns!()}
                {deploy_headings!(context)}
                TableSection {
                    {deploy!(context, "4213", "main", Key::ExampleTableWhereProduction, "51s", Key::ExampleTableResultPassing, "passing")}
                    {deploy!(context, "4211", "fix/anchor", Key::ExampleTableWhereStaging, "12s", Key::ExampleTableResultFailed, "failed")}
                }
            }
        }
    ),
    example!(
        r"TableSection { kind: TableSectionKind::Foot } with an aggregate",
        Key::ExampleTableNoteSectionFoot,
        |context| rsx! {
            Table {
                name: "section-foot",
                caption: context.t(Key::ExampleTableInvoices),
                {invoice_columns!()}
                {invoice_headings!(context)}
                TableSection {
                    {invoice!(context, "INV-0043", "Aurora Ltd", "3 Mar", "2026-03-03", Key::ExampleTableStatusPaid, "paid", "R$ 4.900,00", "4900")}
                    {invoice!(context, "INV-0044", "Bracken & Co", "11 Mar", "2026-03-11", Key::ExampleTableStatusOverdue, "overdue", "R$ 1.240,00", "1240")}
                }
                TableSection { kind: TableSectionKind::Foot,
                    TableRow { emphasis: TableRowEmphasis::Strong,
                        TableHeaderCell {
                            scope: TableHeaderCellScope::Row,
                            column: "invoice",
                            span: "4",
                            {context.t(Key::ExampleTableTotal)}
                        }
                        TableCell {
                            column: "amount",
                            align: TableCellAlign::Numeric,
                            aggregate: TableCellAggregate::Sum,
                            class: "nd-identifier",
                            "R$ 6.140,00"
                        }
                    }
                }
            }
        }
    ),
    example!(
        r"TableSection { name, label, collapsible } — several groups, each folding",
        Key::ExampleTableNoteGroups,
        |context| rsx! {
            Table {
                name: "section-groups",
                caption: context.t(Key::ExampleTableDeploys),
                density: TableDensity::Compact,
                {deploy_columns!()}
                {deploy_headings!(context)}
                TableSection {
                    name: "today",
                    label: context.t(Key::ExampleTableGroupToday),
                    collapsible: true,
                    {deploy!(context, "4213", "main", Key::ExampleTableWhereProduction, "51s", Key::ExampleTableResultPassing, "passing")}
                    {deploy!(context, "4212", "main", Key::ExampleTableWhereStaging, "48s", Key::ExampleTableResultRunning, "running")}
                    TableRow { emphasis: TableRowEmphasis::Subtle,
                        TableHeaderCell {
                            scope: TableHeaderCellScope::Row,
                            column: "build",
                            span: "3",
                            {context.t(Key::ExampleTableInThisGroup)}
                        }
                        TableCell {
                            column: "took",
                            align: TableCellAlign::Numeric,
                            aggregate: TableCellAggregate::Count,
                            aggregate_over: TableCellAggregateOver::Section,
                            class: "nd-identifier",
                            "2"
                        }
                        TableCell { column: "result" }
                    }
                }
                TableSection {
                    name: "yesterday",
                    label: context.t(Key::ExampleTableGroupYesterday),
                    collapsible: true,
                    {deploy!(context, "4211", "fix/anchor", Key::ExampleTableWhereStaging, "12s", Key::ExampleTableResultFailed, "failed")}
                    {deploy!(context, "4210", "main", Key::ExampleTableWhereProduction, "1m 04s", Key::ExampleTableResultCanceled, "canceled")}
                    {deploy!(context, "4209", "main", Key::ExampleTableWhereProduction, "58s", Key::ExampleTableResultPassing, "passing")}
                    TableRow { emphasis: TableRowEmphasis::Subtle,
                        TableHeaderCell {
                            scope: TableHeaderCellScope::Row,
                            column: "build",
                            span: "3",
                            {context.t(Key::ExampleTableInThisGroup)}
                        }
                        TableCell {
                            column: "took",
                            align: TableCellAlign::Numeric,
                            aggregate: TableCellAggregate::Count,
                            aggregate_over: TableCellAggregateOver::Section,
                            class: "nd-identifier",
                            "3"
                        }
                        TableCell { column: "result" }
                    }
                }
            }
        }
    ),
    example!(
        r"TableSection { children } — a table with no head at all",
        Key::ExampleTableNoteSectionNone,
        |context| rsx! {
            Table {
                name: "section-none",
                caption: context.t(Key::ExampleTableInvoices),
                width: TableWidth::Auto,
                TableColumn { name: "field", width: TableColumnWidth::Fit }
                TableColumn { name: "value" }
                TableSection {
                    for (field, value) in [
                        (Key::ExampleTableColInvoice, "INV-0044"),
                        (Key::ExampleTableColClient, "Bracken & Co"),
                        (Key::ExampleTableColDue, "11 Mar"),
                        (Key::ExampleTableColAmount, "R$ 1.240,00"),
                    ] {
                        TableRow { key: "{value}",
                            // NO `wrap`. `nowrap` is the default on a
                            // heading, so the value wrote what absence
                            // already writes.
                            TableHeaderCell {
                                scope: TableHeaderCellScope::Row,
                                column: "field",
                                {context.t(field)}
                            }
                            TableCell { column: "value", class: "nd-identifier", "{value}" }
                        }
                    }
                }
            }
        }
    ),
];

const TABLE: &[Example] = &[
    example!(
        r"Table { name, caption, note } with a Tag in a cell",
        Key::ExampleTableNoteInvoices,
        |context| rsx! {
            Table {
                name: "invoices",
                caption: context.t(Key::ExampleTableInvoices),
                note: context.t(Key::ExampleTableInvoicesNote),
                {invoice_columns!()}
                {invoice_headings!(context)}
                TableSection {
                    {invoice!(context, "INV-0043", "Aurora Ltd", "3 Mar", "2026-03-03", Key::ExampleTableStatusPaid, "paid", "R$ 4.900,00", "4900")}
                    {invoice!(context, "INV-0044", "Bracken & Co", "11 Mar", "2026-03-11", Key::ExampleTableStatusOverdue, "overdue", "R$ 1.240,00", "1240")}
                    {invoice!(context, "INV-0045", "Cintra Studio", "24 Mar", "2026-03-24", Key::ExampleTableStatusPending, "pending", "R$ 680,00", "680")}
                }
            }
        }
    ),
    example!(
        r#"TableColumn { low, tone_below, tone_within } and TableCell { tone: "…" }"#,
        Key::ExampleTableNoteStatus,
        |context| rsx! {
            Table {
                name: "balances",
                caption: context.t(Key::ExampleTableInvoices),
                TableColumn { name: "client" }
                TableColumn {
                    name: "balance",
                    width: TableColumnWidth::Fit,
                    sortable: true,
                    sort_as: TableColumnSortAs::Number,
                    // NO CONDITION IS WRITTEN ANYWHERE. The column declares
                    // where the boundary is and which tone each side wears.
                    low: "0",
                    tone_below: "overdue",
                    tone_within: "paid",
                }
                TableSection { kind: TableSectionKind::Head,
                    TableRow {
                        TableHeaderCell { column: "client", {context.t(Key::ExampleTableColClient)} }
                        TableHeaderCell {
                            column: "balance",
                            align: TableHeaderCellAlign::Numeric,
                            {context.t(Key::ExampleTableColBalance)}
                            Info {
                                name: "balance",
                                label: context.t(Key::ExampleTableBalanceInfo),
                                placement: InfoPlacement::End,
                                {context.t(Key::ExampleTableBalanceInfoNote)}
                            }
                        }
                    }
                }
                TableSection {
                    for (client, shown, sort) in [
                        ("Aurora Ltd", "R$ 4.900,00", "4900"),
                        ("Bracken & Co", "-R$ 240,00", "-240"),
                        ("Cintra Studio", "R$ 680,00", "680"),
                        ("Delgado SA", "-R$ 1.115,00", "-1115"),
                    ] {
                        TableRow { key: "{client}",
                            TableHeaderCell {
                                scope: TableHeaderCellScope::Row,
                                column: "client",
                                class: "nd-identifier",
                                "{client}"
                            }
                            TableCell {
                                column: "balance",
                                class: "nd-identifier",
                                align: TableCellAlign::Numeric,
                                sort_value: sort,
                                "{shown}"
                            }
                        }
                    }
                    // A CELL'S OWN TONE WINS. This one is negative and would
                    // have been painted `overdue` by the column; the server
                    // knows it has been written off, and says so.
                    TableRow {
                        TableHeaderCell {
                            scope: TableHeaderCellScope::Row,
                            column: "client",
                            class: "nd-identifier",
                            "Enseada Lda"
                        }
                        TableCell {
                            column: "balance",
                            class: "nd-identifier",
                            tone: "archived",
                            align: TableCellAlign::Numeric,
                            sort_value: "-90",
                            "-R$ 90,00"
                        }
                    }
                }
            }
        }
    ),
    example!(
        r"Table { name, caption, search_label, search_placeholder, filtered_label }
TableRow { emphasis } and TableCell { aggregate }: subtotal, tax, total",
        Key::ExampleTableNoteTotals,
        |context| rsx! {
            Table {
                name: "invoice-totals",
                caption: context.t(Key::ExampleTableInvoices),
                search_label: context.t(Key::ExampleTableSearch),
                search_placeholder: context.t(Key::ExampleTableSearchPlaceholder),
                filtered_label: context.t(Key::ExampleTableFiltered),
                TableColumn { name: "invoice", width: TableColumnWidth::Fit, filterable: true }
                TableColumn { name: "client", filterable: true }
                TableColumn { name: "due", width: TableColumnWidth::Fit }
                TableColumn { name: "status", width: TableColumnWidth::Fit, filterable: true }
                TableColumn { name: "amount", width: TableColumnWidth::Fit }
                {invoice_headings!(context)}
                TableSection {
                    {invoice!(context, "INV-0043", "Aurora Ltd", "3 Mar", "2026-03-03", Key::ExampleTableStatusPaid, "paid", "R$ 4.900,00", "4900")}
                    {invoice!(context, "INV-0044", "Bracken & Co", "11 Mar", "2026-03-11", Key::ExampleTableStatusOverdue, "overdue", "R$ 1.240,00", "1240")}
                    {invoice!(context, "INV-0045", "Cintra Studio", "24 Mar", "2026-03-24", Key::ExampleTableStatusPending, "pending", "R$ 680,00", "680")}
                    TableRow { empty: true,
                        TableCell { span: "5", {context.t(Key::ExampleTableEmpty)} }
                    }
                }
                TableSection { kind: TableSectionKind::Foot,
                    TableRow { emphasis: TableRowEmphasis::Subtle,
                        TableHeaderCell {
                            scope: TableHeaderCellScope::Row,
                            column: "invoice",
                            span: "4",
                            {context.t(Key::ExampleTableSubtotal)}
                        }
                        TableCell {
                            column: "amount",
                            align: TableCellAlign::Numeric,
                            aggregate: TableCellAggregate::Sum,
                            class: "nd-identifier",
                            "R$ 6.820,00"
                        }
                    }
                    TableRow { emphasis: TableRowEmphasis::Subtle,
                        TableHeaderCell {
                            scope: TableHeaderCellScope::Row,
                            column: "invoice",
                            span: "4",
                            {context.t(Key::ExampleTableTax)}
                        }
                        TableCell {
                            column: "amount",
                            class: "nd-identifier",
                            align: TableCellAlign::Numeric,
                            "R$ 1.568,60"
                        }
                    }
                    TableRow { emphasis: TableRowEmphasis::Strong,
                        TableHeaderCell {
                            scope: TableHeaderCellScope::Row,
                            column: "invoice",
                            span: "4",
                            {context.t(Key::ExampleTableTotal)}
                        }
                        TableCell {
                            column: "amount",
                            class: "nd-identifier",
                            align: TableCellAlign::Numeric,
                            "R$ 8.388,60"
                        }
                    }
                }
            }
        }
    ),
    example!(
        r"A Meter, a Progress and an Info, all inside cells",
        Key::ExampleTableNoteUsage,
        |context| rsx! {
            Table {
                name: "usage",
                caption: context.t(Key::ExampleTableUsage),
                density: TableDensity::Comfortable,
                TableColumn { name: "project" }
                TableColumn { name: "storage", width: TableColumnWidth::Medium }
                TableColumn { name: "tokens", width: TableColumnWidth::Medium }
                TableColumn { name: "import", width: TableColumnWidth::Medium }
                TableSection { kind: TableSectionKind::Head,
                    TableRow {
                        TableHeaderCell { column: "project", {context.t(Key::ExampleTableColProject)} }
                        TableHeaderCell { column: "storage", {context.t(Key::ExampleTableColStorage)} }
                        TableHeaderCell { column: "tokens",
                            {context.t(Key::ExampleTableColTokens)}
                            Info {
                                name: "tokens",
                                label: context.t(Key::ExampleTableTokensInfo),
                                {context.t(Key::ExampleTableTokensInfoNote)}
                            }
                        }
                        TableHeaderCell { column: "import", {context.t(Key::ExampleTableColImport)} }
                    }
                }
                TableSection {
                    TableRow {
                        TableHeaderCell { scope: TableHeaderCellScope::Row, column: "project", class: "nd-identifier", "atlas" }
                        TableCell { column: "storage",
                            Meter {
                                name: "atlas-storage",
                                label: context.t(Key::ExampleTableColStorage),
                                value: "41.2",
                                max: "50",
                                unit: "GB",
                                low: "10",
                                high: "40",
                                optimum: "0",
                                size: MeterSize::Small,
                                hide_label: true,
                                value_label: context.t(Key::ExampleTableStorageValue),
                            }
                        }
                        TableCell { column: "tokens",
                            Meter {
                                name: "atlas-tokens",
                                label: context.t(Key::ExampleTableColTokens),
                                value: "48",
                                max: "200",
                                low: "60",
                                high: "160",
                                optimum: "0",
                                size: MeterSize::Small,
                                hide_label: true,
                                value_label: context.t(Key::ExampleTableTokensValue),
                            }
                        }
                        TableCell { column: "import",
                            Progress {
                                name: "atlas-import",
                                label: context.t(Key::ExampleTableColImport),
                                value: "7",
                                max: "12",
                                size: ProgressSize::Small,
                                hide_label: true,
                                value_label: context.t(Key::ExampleTableImportValue),
                            }
                        }
                    }
                    TableRow {
                        TableHeaderCell { scope: TableHeaderCellScope::Row, column: "project", class: "nd-identifier", "borealis" }
                        TableCell { column: "storage",
                            Meter {
                                name: "borealis-storage",
                                label: context.t(Key::ExampleTableColStorage),
                                value: "48.6",
                                max: "50",
                                unit: "GB",
                                low: "10",
                                high: "40",
                                optimum: "0",
                                size: MeterSize::Small,
                                hide_label: true,
                                value_label: context.t(Key::ExampleTableStorageValue),
                            }
                        }
                        TableCell { column: "tokens",
                            Meter {
                                name: "borealis-tokens",
                                label: context.t(Key::ExampleTableColTokens),
                                value: "121",
                                max: "200",
                                low: "60",
                                high: "160",
                                optimum: "0",
                                size: MeterSize::Small,
                                hide_label: true,
                                value_label: context.t(Key::ExampleTableTokensValue),
                            }
                        }
                        TableCell { column: "import",
                            Progress {
                                name: "borealis-import",
                                label: context.t(Key::ExampleTableColImport),
                                size: ProgressSize::Small,
                                hide_label: true,
                                busy_label: context.t(Key::ExampleTableImportBusy),
                            }
                        }
                    }
                }
            }
        }
    ),
    example!(
        r"Table { search_label, search_placeholder, filtered_label, summary_label }",
        Key::ExampleTableNoteSearch,
        |context| rsx! {
            Table {
                name: "searchable",
                caption: context.t(Key::ExampleTableInvoices),
                search_label: context.t(Key::ExampleTableSearch),
                search_placeholder: context.t(Key::ExampleTableSearchPlaceholder),
                filtered_label: context.t(Key::ExampleTableFiltered),
                summary_label: context.t(Key::ExampleTableSummary),
                TableColumn { name: "invoice", width: TableColumnWidth::Fit, filterable: true }
                TableColumn { name: "client", filterable: true }
                TableColumn { name: "due", width: TableColumnWidth::Fit }
                TableColumn { name: "status", width: TableColumnWidth::Fit, filterable: true }
                TableColumn { name: "amount", width: TableColumnWidth::Fit }
                {invoice_headings!(context)}
                TableSection {
                    {invoice!(context, "INV-0043", "Aurora Ltd", "3 Mar", "2026-03-03", Key::ExampleTableStatusPaid, "paid", "R$ 4.900,00", "4900")}
                    {invoice!(context, "INV-0044", "Bracken & Co", "11 Mar", "2026-03-11", Key::ExampleTableStatusOverdue, "overdue", "R$ 1.240,00", "1240")}
                    {invoice!(context, "INV-0045", "Cintra Studio", "24 Mar", "2026-03-24", Key::ExampleTableStatusPending, "pending", "R$ 680,00", "680")}
                    {invoice!(context, "INV-0046", "Delgado SA", "29 Mar", "2026-03-29", Key::ExampleTableStatusRefunded, "refunded", "R$ 320,00", "320")}
                    TableRow { empty: true,
                        TableCell { span: "5", {context.t(Key::ExampleTableEmpty)} }
                    }
                }
            }
        }
    ),
    example!(
        r"Table { export, export_label, csv_label, json_label, print_label, exported_label, copy, copy_label, copied_label, selected_label }",
        Key::ExampleTableNoteExport,
        |context| rsx! {
            Table {
                name: "team",
                caption: context.t(Key::ExampleTableTeam),
                export: true,
                export_label: context.t(Key::ExampleTableExport),
                csv_label: context.t(Key::ExampleTableExportCsv),
                json_label: context.t(Key::ExampleTableExportJson),
                print_label: context.t(Key::ExampleTableExportPrint),
                exported_label: context.t(Key::ExampleTableExported),
                copy: true,
                copy_label: context.t(Key::ExampleTableExportCopy),
                copied_label: context.t(Key::ExampleTableCopied),
                selected_label: context.t(Key::ExampleTableChosen),
                TableColumn { name: "choose", width: TableColumnWidth::Fit }
                // No `filterable`: this table draws no search box either.
                TableColumn { name: "person" }
                TableColumn { name: "role", width: TableColumnWidth::Fit }
                TableColumn { name: "access", width: TableColumnWidth::Fit }
                TableSection { kind: TableSectionKind::Head,
                    TableRow {
                        select: TableRowSelect::All,
                        select_name: "person",
                        select_label: context.t(Key::ExampleTableChooseEveryone),
                        TableHeaderCell { column: "person", {context.t(Key::ExampleTableColPerson)} }
                        TableHeaderCell { column: "role", {context.t(Key::ExampleTableColRole)} }
                        TableHeaderCell { column: "access", {context.t(Key::ExampleTableColAccess)} }
                    }
                }
                TableSection {
                    for (id, person, role, tone, access, chosen, refused) in [
                        ("ada", "Ada Lovelace", Key::ExampleTableRoleOwner, "authorized", Key::ExampleTableAccessToday, true, false),
                        ("alan", "Alan Turing", Key::ExampleTableRoleAdmin, "admin", Key::ExampleTableAccessToday, false, false),
                        ("grace", "Grace Hopper", Key::ExampleTableRoleMember, "member", Key::ExampleTableAccessWeek, false, false),
                        ("katherine", "Katherine Johnson", Key::ExampleTableRoleGuest, "guest", Key::ExampleTableAccessNever, false, true),
                    ] {
                        TableRow {
                            key: "{id}",
                            name: id,
                            select: TableRowSelect::Many,
                            select_name: "person",
                            select_label: context.t(Key::ExampleTableChoosePerson),
                            selected: chosen,
                            disabled: refused,
                            TableHeaderCell {
                                scope: TableHeaderCellScope::Row,
                                column: "person",
                                class: "nd-identifier",
                                "{person}"
                            }
                            TableCell { column: "role",
                                Tag {
                                    label: context.t(role),
                                    tone,
                                    variant: TagVariant::Outline,
                                    size: TagSize::Small,
                                }
                            }
                            TableCell { column: "access", {context.t(access)} }
                        }
                    }
                }
            }
        }
    ),
    example!(
        r"Table { more_src, more_label, more_status_label, total } — load more",
        Key::ExampleTableNoteMore,
        |context| rsx! {
            Table {
                name: "deploys",
                caption: context.t(Key::ExampleTableDeploys),
                density: TableDensity::Compact,
                more_src: format!("{}assets/deploys-2.html", crate::site::BASE),
                more_label: context.t(Key::ExampleTableMore),
                more_status_label: context.t(Key::ExampleTableMoreStatus),
                total: "8",
                summary_label: context.t(Key::ExampleTableSummary),
                {deploy_columns!()}
                {deploy_headings!(context)}
                TableSection {
                    {deploy!(context, "4213", "main", Key::ExampleTableWhereProduction, "51s", Key::ExampleTableResultPassing, "passing")}
                    {deploy!(context, "4212", "main", Key::ExampleTableWhereStaging, "48s", Key::ExampleTableResultRunning, "running")}
                    {deploy!(context, "4211", "fix/anchor", Key::ExampleTableWhereStaging, "12s", Key::ExampleTableResultFailed, "failed")}
                    {deploy!(context, "4210", "main", Key::ExampleTableWhereProduction, "1m 04s", Key::ExampleTableResultDegraded, "degraded")}
                }
            }
        }
    ),
    example!(
        r"Table { density, rules, zebra, hover }",
        Key::ExampleTableNoteDensity,
        |context| rsx! {
            Table {
                name: "deploys-dense",
                caption: context.t(Key::ExampleTableDeploys),
                density: TableDensity::Compact,
                rules: TableRules::Grid,
                zebra: true,
                hover: true,
                {deploy_columns!()}
                {deploy_headings!(context)}
                TableSection {
                    {deploy!(context, "4213", "main", Key::ExampleTableWhereProduction, "51s", Key::ExampleTableResultPassing, "passing")}
                    {deploy!(context, "4212", "main", Key::ExampleTableWhereStaging, "48s", Key::ExampleTableResultRunning, "running")}
                    {deploy!(context, "4211", "fix/anchor", Key::ExampleTableWhereStaging, "12s", Key::ExampleTableResultFailed, "failed")}
                    {deploy!(context, "4210", "main", Key::ExampleTableWhereProduction, "1m 04s", Key::ExampleTableResultCanceled, "canceled")}
                }
            }
        }
    ),
    example!(
        r"Table { sticky_header: true, height: TableHeight::Short }",
        Key::ExampleTableNoteSticky,
        |context| rsx! {
            Table {
                name: "deploys-tall",
                caption: context.t(Key::ExampleTableDeploys),
                density: TableDensity::Compact,
                sticky_header: true,
                height: TableHeight::Short,
                {deploy_columns!()}
                {deploy_headings!(context)}
                TableSection {
                    for number in ["4213", "4212", "4211", "4210", "4209", "4208", "4207", "4206", "4205"] {
                        {deploy!(context, {number}, "main", Key::ExampleTableWhereProduction, "51s", Key::ExampleTableResultPassing, "passing")}
                    }
                }
            }
        }
    ),
    example!(
        r"Table { hide_caption: true, width: TableWidth::Auto, layout: TableLayout::Stack }",
        Key::ExampleTableNoteStack,
        |context| rsx! {
            Table {
                name: "invoices-stacked",
                caption: context.t(Key::ExampleTableInvoices),
                hide_caption: true,
                width: TableWidth::Auto,
                layout: TableLayout::Stack,
                {invoice_columns!()}
                {invoice_headings!(context)}
                TableSection {
                    TableRow {
                        TableHeaderCell {
                            scope: TableHeaderCellScope::Row,
                            column: "invoice",
                            class: "nd-identifier",
                            "INV-0044"
                        }
                        TableCell {
                            column: "client",
                            class: "nd-identifier",
                            label: context.t(Key::ExampleTableColClient),
                            "Bracken & Co"
                        }
                        TableCell {
                            column: "due",
                            class: "nd-identifier",
                            label: context.t(Key::ExampleTableColDue),
                            sort_value: "2026-03-11",
                            "11 Mar"
                        }
                        TableCell { column: "status", label: context.t(Key::ExampleTableColStatus),
                            Tag {
                                label: context.t(Key::ExampleTableStatusOverdue),
                                tone: "overdue",
                                size: TagSize::Small,
                            }
                        }
                        TableCell {
                            column: "amount",
                            class: "nd-identifier",
                            label: context.t(Key::ExampleTableColAmount),
                            align: TableCellAlign::Numeric,
                            "R$ 1.240,00"
                        }
                    }
                }
            }
        }
    ),
    example!(
        r"Table { resize_label, resized_label, columns_label, column_shown_label, column_hidden_label }",
        Key::ExampleTableNoteColumns,
        |context| rsx! {
            Table {
                name: "invoices-wide",
                caption: context.t(Key::ExampleTableInvoices),
                resize_label: context.t(Key::ExampleTableResize),
                resized_label: context.t(Key::ExampleTableResized),
                columns_label: context.t(Key::ExampleTableColumns),
                column_shown_label: context.t(Key::ExampleTableColumnShown),
                column_hidden_label: context.t(Key::ExampleTableColumnHidden),
                TableColumn { name: "invoice", width: TableColumnWidth::Fit, pinned: true }
                TableColumn { name: "client", pinned: true, resizable: true }
                TableColumn { name: "due", width: TableColumnWidth::Fit, hideable: true }
                TableColumn { name: "status", width: TableColumnWidth::Fit, hideable: true }
                TableColumn {
                    name: "amount",
                    width: TableColumnWidth::Fit,
                    hideable: true,
                    resizable: true,
                }
                {invoice_headings!(context)}
                TableSection {
                    {invoice!(context, "INV-0043", "Aurora Ltd", "3 Mar", "2026-03-03", Key::ExampleTableStatusPaid, "paid", "R$ 4.900,00", "4900")}
                    {invoice!(context, "INV-0044", "Bracken & Co", "11 Mar", "2026-03-11", Key::ExampleTableStatusOverdue, "overdue", "R$ 1.240,00", "1240")}
                }
            }
        }
    ),
    example!(
        r"Table { sort_ascending_label, sort_descending_label, sort_cleared_label }",
        Key::ExampleTableNoteSort,
        |context| rsx! {
            Table {
                name: "invoices-sorted",
                caption: context.t(Key::ExampleTableInvoices),
                sort_ascending_label: context.t(Key::ExampleTableSortAscending),
                sort_descending_label: context.t(Key::ExampleTableSortDescending),
                sort_cleared_label: context.t(Key::ExampleTableSortCleared),
                TableColumn { name: "invoice", width: TableColumnWidth::Fit, sortable: true }
                TableColumn { name: "client", sortable: true }
                TableColumn {
                    name: "due",
                    width: TableColumnWidth::Fit,
                    sortable: true,
                    sort_as: TableColumnSortAs::Date,
                }
                TableColumn { name: "status", width: TableColumnWidth::Fit }
                TableColumn {
                    name: "amount",
                    width: TableColumnWidth::Fit,
                    sortable: true,
                    sort_as: TableColumnSortAs::Number,
                }
                {invoice_headings!(context)}
                TableSection {
                    {invoice!(context, "INV-0045", "Cintra Studio", "24 Mar", "2026-03-24", Key::ExampleTableStatusPending, "pending", "R$ 680,00", "680")}
                    {invoice!(context, "INV-0043", "Aurora Ltd", "3 Mar", "2026-03-03", Key::ExampleTableStatusPaid, "paid", "R$ 4.900,00", "4900")}
                    {invoice!(context, "INV-0046", "Delgado SA", "29 Mar", "2026-03-29", Key::ExampleTableStatusRefunded, "refunded", "R$ 320,00", "320")}
                    {invoice!(context, "INV-0044", "Bracken & Co", "11 Mar", "2026-03-11", Key::ExampleTableStatusOverdue, "overdue", "R$ 1.240,00", "1240")}
                }
            }
        }
    ),
];

const TABLE_ROW: &[Example] = &[
    example!(
        r#"TableRow { tone: "…" } — a wash and a rule down the leading edge"#,
        Key::ExampleTableNoteRowTone,
        |context| rsx! {
            Table {
                name: "row-tone",
                caption: context.t(Key::ExampleTableDeploys),
                {deploy_columns!()}
                {deploy_headings!(context)}
                TableSection {
                    {deploy!(context, "4213", "main", Key::ExampleTableWhereProduction, "51s", Key::ExampleTableResultPassing, "passing")}
                    TableRow { tone: "failed",
                        TableHeaderCell {
                            scope: TableHeaderCellScope::Row,
                            column: "build",
                            class: "nd-identifier",
                            "#"
                            "4211"
                        }
                        TableCell { column: "branch", class: "nd-identifier", "fix/anchor" }
                        TableCell { column: "where", {context.t(Key::ExampleTableWhereStaging)} }
                        TableCell { column: "took", class: "nd-identifier", align: TableCellAlign::Numeric, "12s" }
                        TableCell { column: "result",
                            Tag {
                                label: context.t(Key::ExampleTableResultFailed),
                                tone: "failed",
                                size: TagSize::Small,
                            }
                        }
                    }
                    {deploy!(context, "4210", "main", Key::ExampleTableWhereProduction, "1m 04s", Key::ExampleTableResultPassing, "passing")}
                }
            }
        }
    ),
    example!(
        r"TableRow { name, expandable, expand_label } with a detail_for row",
        Key::ExampleTableNoteRowExpand,
        |context| rsx! {
            Table {
                name: "row-expand",
                caption: context.t(Key::ExampleTableDeploys),
                density: TableDensity::Compact,
                TableColumn { name: "open", width: TableColumnWidth::Fit }
                {deploy_columns!()}
                {deploy_headings!(context)}
                TableSection {
                    TableRow {
                        name: "b4213",
                        expandable: true,
                        expand_label: context.t(Key::ExampleTableLogOpen),
                        TableHeaderCell {
                            scope: TableHeaderCellScope::Row,
                            column: "build",
                            class: "nd-identifier",
                            "#"
                            "4213"
                        }
                        TableCell { column: "branch", class: "nd-identifier", "main" }
                        TableCell { column: "where", {context.t(Key::ExampleTableWhereProduction)} }
                        TableCell { column: "took", class: "nd-identifier", align: TableCellAlign::Numeric, "51s" }
                        TableCell { column: "result",
                            Tag {
                                label: context.t(Key::ExampleTableResultPassing),
                                tone: "passing",
                                size: TagSize::Small,
                            }
                        }
                    }
                    TableRow { detail_for: "b4213",
                        TableCell { span: "6", class: "nd-identifier", {context.t(Key::ExampleTableLog)} }
                    }
                    TableRow {
                        name: "b4211",
                        expandable: true,
                        expand_label: context.t(Key::ExampleTableLogOpen),
                        TableHeaderCell {
                            scope: TableHeaderCellScope::Row,
                            column: "build",
                            class: "nd-identifier",
                            "#"
                            "4211"
                        }
                        TableCell { column: "branch", class: "nd-identifier", "fix/anchor" }
                        TableCell { column: "where", {context.t(Key::ExampleTableWhereStaging)} }
                        TableCell { column: "took", class: "nd-identifier", align: TableCellAlign::Numeric, "12s" }
                        TableCell { column: "result",
                            Tag {
                                label: context.t(Key::ExampleTableResultFailed),
                                tone: "failed",
                                size: TagSize::Small,
                            }
                        }
                    }
                    TableRow { detail_for: "b4211",
                        TableCell { span: "6", class: "nd-identifier", {context.t(Key::ExampleTableLog)} }
                    }
                }
            }
        }
    ),
    example!(
        r"TableRow { select: TableRowSelect::One, select_name, select_label, selected, disabled }",
        Key::ExampleTableNoteRowSelect,
        |context| rsx! {
            Table {
                name: "row-one",
                caption: context.t(Key::ExampleTableTeam),
                selected_label: context.t(Key::ExampleTableChosen),
                TableColumn { name: "choose", width: TableColumnWidth::Fit }
                TableColumn { name: "person" }
                TableColumn { name: "role", width: TableColumnWidth::Fit }
                TableSection { kind: TableSectionKind::Head,
                    TableRow {
                        TableHeaderCell { column: "choose" }
                        TableHeaderCell { column: "person", {context.t(Key::ExampleTableColPerson)} }
                        TableHeaderCell { column: "role", {context.t(Key::ExampleTableColRole)} }
                    }
                }
                TableSection {
                    for (id, person, role, tone, refused) in [
                        ("ada", "Ada Lovelace", Key::ExampleTableRoleOwner, "authorized", false),
                        ("alan", "Alan Turing", Key::ExampleTableRoleAdmin, "admin", false),
                        ("katherine", "Katherine Johnson", Key::ExampleTableRoleGuest, "guest", true),
                    ] {
                        TableRow {
                            key: "{id}",
                            name: id,
                            select: TableRowSelect::One,
                            select_name: "owner",
                            select_label: context.t(Key::ExampleTableChoosePerson),
                            disabled: refused,
                            TableHeaderCell {
                                scope: TableHeaderCellScope::Row,
                                column: "person",
                                class: "nd-identifier",
                                "{person}"
                            }
                            TableCell { column: "role",
                                Tag {
                                    label: context.t(role),
                                    tone,
                                    variant: TagVariant::Outline,
                                    size: TagSize::Small,
                                }
                            }
                        }
                    }
                }
            }
        }
    ),
    example!(
        r"TableRow { emphasis: ::Subtle | ::Strong } and TableCell { aggregate }",
        Key::ExampleTableNoteRowEmphasis,
        |context| rsx! {
            // THE SAME TWO WEIGHTS AS THE INVOICE FOOT, WITH NO MONEY IN IT.
            // `emphasis` says how much weight a row carries and nothing about
            // what it means — these two close a group of builds and then the
            // fleet, which is a subtotal and a total in a ledger's words and
            // neither in this table's.
            Table {
                name: "row-emphasis",
                caption: context.t(Key::ExampleTableDeploys),
                TableColumn { name: "build" }
                TableColumn { name: "took", width: TableColumnWidth::Fit }
                TableSection { kind: TableSectionKind::Head,
                    TableRow {
                        TableHeaderCell { column: "build", {context.t(Key::ExampleTableColBuild)} }
                        TableHeaderCell {
                            column: "took",
                            align: TableHeaderCellAlign::Numeric,
                            {context.t(Key::ExampleTableColTook)}
                        }
                    }
                }
                for (name, label, builds, group) in [
                    ("production", Key::ExampleTableWhereProduction, [("4213", "51s"), ("4209", "48s")], "99s"),
                    ("staging", Key::ExampleTableWhereStaging, [("4212", "48s"), ("4211", "12s")], "60s"),
                ] {
                    TableSection { key: "{name}", name: "{name}",
                        for (build, took) in builds {
                            TableRow { key: "{build}",
                                TableHeaderCell {
                                    scope: TableHeaderCellScope::Row,
                                    column: "build",
                                    class: "nd-identifier",
                                    "#{build}"
                                }
                                TableCell {
                                    column: "took",
                                    class: "nd-identifier",
                                    align: TableCellAlign::Numeric,
                                    "{took}"
                                }
                            }
                        }
                        TableRow { emphasis: TableRowEmphasis::Subtle,
                            TableHeaderCell {
                                scope: TableHeaderCellScope::Row,
                                column: "build",
                                {context.t(label)}
                            }
                            TableCell {
                                column: "took",
                                class: "nd-identifier",
                                align: TableCellAlign::Numeric,
                                aggregate: TableCellAggregate::Sum,
                                aggregate_over: TableCellAggregateOver::Section,
                                "{group}"
                            }
                        }
                    }
                }
                TableSection { kind: TableSectionKind::Foot,
                    TableRow { emphasis: TableRowEmphasis::Strong,
                        TableHeaderCell {
                            scope: TableHeaderCellScope::Row,
                            column: "build",
                            {context.t(Key::ExampleTableAllEnvironments)}
                        }
                        TableCell {
                            column: "took",
                            class: "nd-identifier",
                            align: TableCellAlign::Numeric,
                            aggregate: TableCellAggregate::Sum,
                            "159s"
                        }
                    }
                }
            }
        }
    ),
    example!(
        r"Table { name, caption, search_label, search_placeholder, filtered_label }
TableRow { empty: true }: the caller's own sentence",
        Key::ExampleTableNoteEmpty,
        |context| rsx! {
            Table {
                name: "row-empty",
                caption: context.t(Key::ExampleTableInvoices),
                search_label: context.t(Key::ExampleTableSearch),
                search_placeholder: context.t(Key::ExampleTableSearchPlaceholder),
                filtered_label: context.t(Key::ExampleTableFiltered),
                TableColumn { name: "invoice", width: TableColumnWidth::Fit, filterable: true }
                TableColumn { name: "client", filterable: true }
                TableColumn { name: "due", width: TableColumnWidth::Fit }
                TableColumn { name: "status", width: TableColumnWidth::Fit, filterable: true }
                TableColumn { name: "amount", width: TableColumnWidth::Fit }
                {invoice_headings!(context)}
                TableSection {
                    {invoice!(context, "INV-0043", "Aurora Ltd", "3 Mar", "2026-03-03", Key::ExampleTableStatusPaid, "paid", "R$ 4.900,00", "4900")}
                    TableRow { empty: true,
                        TableCell { span: "5", {context.t(Key::ExampleTableEmpty)} }
                    }
                }
            }
        }
    ),
];

const TABLE_CELL: &[Example] = &[
    example!(
        r#"TableCell { align: ::Numeric, sort_value: "…" }"#,
        Key::ExampleTableNoteCellNumeric,
        |context| rsx! {
            Table {
                name: "cell-numeric",
                caption: context.t(Key::ExampleTableInvoices),
                sort_ascending_label: context.t(Key::ExampleTableSortAscending),
                sort_descending_label: context.t(Key::ExampleTableSortDescending),
                sort_cleared_label: context.t(Key::ExampleTableSortCleared),
                TableColumn { name: "client" }
                TableColumn {
                    name: "amount",
                    width: TableColumnWidth::Fit,
                    sortable: true,
                    sort_as: TableColumnSortAs::Number,
                }
                TableSection { kind: TableSectionKind::Head,
                    TableRow {
                        TableHeaderCell { column: "client", {context.t(Key::ExampleTableColClient)} }
                        TableHeaderCell {
                            column: "amount",
                            align: TableHeaderCellAlign::Numeric,
                            {context.t(Key::ExampleTableColAmount)}
                        }
                    }
                }
                TableSection {
                    for (client, shown, sort) in [
                        ("Aurora Ltd", "R$ 4.900,00", "4900"),
                        ("Bracken & Co", "R$ 1.240,00", "1240"),
                        ("Cintra Studio", "R$ 680,00", "680"),
                        ("Delgado SA", "R$ 12.310,00", "12310"),
                    ] {
                        TableRow { key: "{client}",
                            TableHeaderCell {
                                scope: TableHeaderCellScope::Row,
                                column: "client",
                                class: "nd-identifier",
                                "{client}"
                            }
                            TableCell {
                                column: "amount",
                                class: "nd-identifier",
                                align: TableCellAlign::Numeric,
                                sort_value: sort,
                                "{shown}"
                            }
                        }
                    }
                }
            }
        }
    ),
    example!(
        r"TableCell { wrap: ::Nowrap } beside TableCell { wrap: ::Truncate }",
        Key::ExampleTableNoteCellWrap,
        |context| rsx! {
            Table {
                name: "cell-wrap",
                caption: context.t(Key::ExampleTableDeploys),
                TableColumn { name: "branch", width: TableColumnWidth::Fit }
                TableColumn { name: "log", width: TableColumnWidth::Medium }
                TableSection { kind: TableSectionKind::Head,
                    TableRow {
                        TableHeaderCell { column: "branch", {context.t(Key::ExampleTableColBranch)} }
                        TableHeaderCell { column: "log", {context.t(Key::ExampleTableColResult)} }
                    }
                }
                TableSection {
                    TableRow {
                        TableCell {
                            column: "branch",
                            wrap: TableCellWrap::Nowrap,
                            class: "nd-identifier",
                            "release/2026-03-candidate"
                        }
                        TableCell {
                            column: "log",
                            wrap: TableCellWrap::Truncate,
                            {context.t(Key::ExampleTableLog)}
                        }
                    }
                }
            }
        }
    ),
    example!(
        r#"TableCell { span: "2", rows: "2", column: "…" }"#,
        Key::ExampleTableNoteCellSpan,
        |context| rsx! {
            Table {
                name: "cell-span",
                caption: context.t(Key::ExampleTableDeploys),
                rules: TableRules::Grid,
                TableColumn { name: "build", width: TableColumnWidth::Fit }
                TableColumn { name: "where", width: TableColumnWidth::Fit }
                TableColumn { name: "result" }
                TableSection { kind: TableSectionKind::Head,
                    TableRow {
                        TableHeaderCell { column: "build", {context.t(Key::ExampleTableColBuild)} }
                        TableHeaderCell { column: "where", {context.t(Key::ExampleTableColWhere)} }
                        TableHeaderCell { column: "result", {context.t(Key::ExampleTableColResult)} }
                    }
                }
                TableSection {
                    TableRow {
                        TableHeaderCell {
                            scope: TableHeaderCellScope::Row,
                            column: "build",
                            rows: "2",
                            class: "nd-identifier",
                            "#"
                            "4213"
                        }
                        TableCell { column: "where", {context.t(Key::ExampleTableWhereStaging)} }
                        TableCell { column: "result",
                            Tag {
                                label: context.t(Key::ExampleTableResultPassing),
                                tone: "passing",
                                size: TagSize::Small,
                            }
                        }
                    }
                    TableRow {
                        TableCell { span: "2", column: "where", {context.t(Key::ExampleTableLog)} }
                    }
                }
            }
        }
    ),
    example!(
        r#"TableCell { label, tone: "…" } — the stacked layout's own label"#,
        Key::ExampleTableNoteCellLabel,
        |context| rsx! {
            Table {
                name: "cell-label",
                caption: context.t(Key::ExampleTableTeam),
                layout: TableLayout::Stack,
                TableColumn { name: "person" }
                TableColumn { name: "role", width: TableColumnWidth::Fit }
                TableColumn { name: "access", width: TableColumnWidth::Fit }
                TableSection { kind: TableSectionKind::Head,
                    TableRow {
                        TableHeaderCell { column: "person", {context.t(Key::ExampleTableColPerson)} }
                        TableHeaderCell { column: "role", {context.t(Key::ExampleTableColRole)} }
                        TableHeaderCell { column: "access", {context.t(Key::ExampleTableColAccess)} }
                    }
                }
                TableSection {
                    TableRow {
                        TableHeaderCell {
                            scope: TableHeaderCellScope::Row,
                            column: "person",
                            class: "nd-identifier",
                            "Grace Hopper"
                        }
                        TableCell { column: "role", label: context.t(Key::ExampleTableColRole),
                            Tag {
                                label: context.t(Key::ExampleTableRoleMember),
                                tone: "member",
                                variant: TagVariant::Outline,
                                size: TagSize::Small,
                            }
                        }
                        TableCell {
                            column: "access",
                            label: context.t(Key::ExampleTableColAccess),
                            tone: "muted",
                            {context.t(Key::ExampleTableAccessWeek)}
                        }
                    }
                }
            }
        }
    ),
    example!(
        r"Table { name, caption, search_label, search_placeholder, filtered_label }
TableCell { aggregate, aggregate_over }: a total that recounts",
        Key::ExampleTableNoteCellAggregate,
        |context| rsx! {
            Table {
                name: "cell-total",
                caption: context.t(Key::ExampleTableInvoices),
                search_label: context.t(Key::ExampleTableSearch),
                search_placeholder: context.t(Key::ExampleTableSearchPlaceholder),
                filtered_label: context.t(Key::ExampleTableFiltered),
                TableColumn { name: "invoice", width: TableColumnWidth::Fit, filterable: true }
                TableColumn { name: "client", filterable: true }
                TableColumn { name: "due", width: TableColumnWidth::Fit }
                TableColumn { name: "status", width: TableColumnWidth::Fit, filterable: true }
                TableColumn { name: "amount", width: TableColumnWidth::Fit }
                {invoice_headings!(context)}
                TableSection {
                    {invoice!(context, "INV-0043", "Aurora Ltd", "3 Mar", "2026-03-03", Key::ExampleTableStatusPaid, "paid", "R$ 4.900,00", "4900")}
                    {invoice!(context, "INV-0044", "Bracken & Co", "11 Mar", "2026-03-11", Key::ExampleTableStatusOverdue, "overdue", "R$ 1.240,00", "1240")}
                    {invoice!(context, "INV-0045", "Cintra Studio", "24 Mar", "2026-03-24", Key::ExampleTableStatusPending, "pending", "R$ 680,00", "680")}
                    TableRow { empty: true,
                        TableCell { span: "5", {context.t(Key::ExampleTableEmpty)} }
                    }
                }
                TableSection { kind: TableSectionKind::Foot,
                    TableRow { emphasis: TableRowEmphasis::Strong,
                        TableHeaderCell {
                            scope: TableHeaderCellScope::Row,
                            column: "invoice",
                            span: "4",
                            {context.t(Key::ExampleTableTotal)}
                        }
                        TableCell {
                            column: "amount",
                            align: TableCellAlign::Numeric,
                            aggregate: TableCellAggregate::Sum,
                            class: "nd-identifier",
                            "R$ 6.820,00"
                        }
                    }
                }
            }
        }
    ),
];

const TABLE_HEADER_CELL: &[Example] = &[
    example!(
        r"TableHeaderCell { scope: ::Col | ::Row }",
        Key::ExampleTableNoteHeaderScope,
        |context| rsx! {
            Table {
                name: "header-scope",
                caption: context.t(Key::ExampleTableInvoices),
                {invoice_columns!()}
                {invoice_headings!(context)}
                TableSection {
                    {invoice!(context, "INV-0043", "Aurora Ltd", "3 Mar", "2026-03-03", Key::ExampleTableStatusPaid, "paid", "R$ 4.900,00", "4900")}
                    {invoice!(context, "INV-0046", "Delgado SA", "29 Mar", "2026-03-29", Key::ExampleTableStatusRefunded, "refunded", "R$ 320,00", "320")}
                }
            }
        }
    ),
    example!(
        r"TableHeaderCell { column, sort: ::Ascending } — the order a server applied",
        Key::ExampleTableNoteHeaderSort,
        |context| rsx! {
            Table {
                name: "header-sort",
                caption: context.t(Key::ExampleTableInvoices),
                sort_ascending_label: context.t(Key::ExampleTableSortAscending),
                sort_descending_label: context.t(Key::ExampleTableSortDescending),
                sort_cleared_label: context.t(Key::ExampleTableSortCleared),
                {invoice_columns!()}
                TableSection { kind: TableSectionKind::Head,
                    TableRow {
                        TableHeaderCell { column: "invoice", {context.t(Key::ExampleTableColInvoice)} }
                        TableHeaderCell { column: "client", {context.t(Key::ExampleTableColClient)} }
                        TableHeaderCell {
                            column: "due",
                            sort: TableHeaderCellSort::Ascending,
                            {context.t(Key::ExampleTableColDue)}
                        }
                        TableHeaderCell { column: "status", {context.t(Key::ExampleTableColStatus)} }
                        TableHeaderCell {
                            column: "amount",
                            align: TableHeaderCellAlign::Numeric,
                            {context.t(Key::ExampleTableColAmount)}
                        }
                    }
                }
                TableSection {
                    {invoice!(context, "INV-0043", "Aurora Ltd", "3 Mar", "2026-03-03", Key::ExampleTableStatusPaid, "paid", "R$ 4.900,00", "4900")}
                    {invoice!(context, "INV-0044", "Bracken & Co", "11 Mar", "2026-03-11", Key::ExampleTableStatusOverdue, "overdue", "R$ 1.240,00", "1240")}
                    {invoice!(context, "INV-0045", "Cintra Studio", "24 Mar", "2026-03-24", Key::ExampleTableStatusPending, "pending", "R$ 680,00", "680")}
                }
            }
        }
    ),
    example!(
        r#"TableHeaderCell { align, wrap, tone: "…" }"#,
        Key::ExampleTableNoteHeaderAlign,
        |context| rsx! {
            Table {
                name: "header-align",
                caption: context.t(Key::ExampleTableInvoices),
                rules: TableRules::Grid,
                TableColumn { name: "right", width: TableColumnWidth::Medium }
                TableColumn { name: "wrong", width: TableColumnWidth::Medium }
                TableSection { kind: TableSectionKind::Head,
                    TableRow {
                        // The same words over two columns of the same numbers,
                        // and only one of them lines up.
                        // `wrap`, WHICH IS NOT THE DEFAULT. A heading is
                        // `nowrap` unless it says otherwise, so `nowrap` here
                        // showed the prop doing nothing at all.
                        TableHeaderCell {
                            column: "right",
                            align: TableHeaderCellAlign::Numeric,
                            wrap: TableHeaderCellWrap::Wrap,
                            {context.t(Key::ExampleTableColAmount)}
                        }
                        TableHeaderCell {
                            column: "wrong",
                            tone: "caution",
                            {context.t(Key::ExampleTableColAmount)}
                        }
                    }
                }
                TableSection {
                    for amount in ["R$ 4.900,00", "R$ 1.240,00", "R$ 680,00"] {
                        TableRow { key: "{amount}",
                            TableCell { column: "right", class: "nd-identifier", align: TableCellAlign::Numeric, "{amount}" }
                            TableCell { column: "wrong", class: "nd-identifier", align: TableCellAlign::Numeric, "{amount}" }
                        }
                    }
                }
            }
        }
    ),
    example!(
        r#"TableHeaderCell { span: "2", rows: "2", scope: ::Colgroup }"#,
        Key::ExampleTableNoteHeaderGroup,
        |context| rsx! {
            // THE SPANNING HEADING HAS TO BE TRUE. An earlier draft put
            // *Result* over the environment and the duration, which are not
            // two halves of a result — a two-level heading whose top level does
            // not describe the columns under it teaches a reader to ignore the
            // top level. A build waits and then it runs; both are durations,
            // and *Duration* is what they are two of.
            Table {
                name: "header-group",
                caption: context.t(Key::ExampleTableDeploys),
                rules: TableRules::Grid,
                // Sized to its own content: five words spread across the full
                // width of the page put each value an inch from the heading
                // that names it, which is the distance a two-level head exists
                // to close.
                width: TableWidth::Auto,
                TableColumn { name: "build", width: TableColumnWidth::Fit }
                TableColumn { name: "wait", width: TableColumnWidth::Fit }
                TableColumn { name: "run", width: TableColumnWidth::Fit }
                TableSection { kind: TableSectionKind::Head,
                    TableRow {
                        TableHeaderCell {
                            rows: "2",
                            scope: TableHeaderCellScope::Rowgroup,
                            column: "build",
                            {context.t(Key::ExampleTableColBuild)}
                        }
                        TableHeaderCell {
                            span: "2",
                            scope: TableHeaderCellScope::Colgroup,
                            align: TableHeaderCellAlign::Center,
                            {context.t(Key::ExampleTableColTook)}
                        }
                    }
                    TableRow {
                        TableHeaderCell {
                            column: "wait",
                            align: TableHeaderCellAlign::Numeric,
                            {context.t(Key::ExampleTableColWait)}
                        }
                        TableHeaderCell {
                            column: "run",
                            align: TableHeaderCellAlign::Numeric,
                            {context.t(Key::ExampleTableColRun)}
                        }
                    }
                }
                TableSection {
                    for (build, wait, run) in [("4213", "6s", "51s"), ("4211", "2s", "12s")] {
                        TableRow { key: "{build}",
                            TableHeaderCell {
                                scope: TableHeaderCellScope::Row,
                                column: "build",
                                class: "nd-identifier",
                                "#{build}"
                            }
                            TableCell { column: "wait", class: "nd-identifier", align: TableCellAlign::Numeric, "{wait}" }
                            TableCell { column: "run", class: "nd-identifier", align: TableCellAlign::Numeric, "{run}" }
                        }
                    }
                }
            }
        }
    ),
];

// --- Menu and MenuItem ------------------------------------------------------

// A BUTTON AND A SURFACE THAT IS NOT INSIDE IT. Every example here renders the
// pair, because neither half is usable alone: a `Menu` with no button is a
// hidden div, and a button with no `Menu` is what this page existed to
// document before the component arrived — `aria-haspopup="menu"` over nothing.
macro_rules! menu_button {
    ($context:ident, $name:literal) => {
        rsx! {
            Button {
                menu: true,
                controls: $name,
                variant: ButtonVariant::Outline,
                tone: ButtonTone::Neutral,
                label: $context.t(Key::ExampleMenuOpen),
            }
        }
    };
}

const MENU: &[Example] = &[
    example!(
        r#"Menu { name, label } under a Button { menu: true, controls: "…" }"#,
        Key::ExampleMenuNoteBasic,
        |context| rsx! {
            div {
                {menu_button!(context, "example-menu-actions")}
                Menu {
                    name: "example-menu-actions",
                    label: context.t(Key::ExampleMenuLabel),
                    MenuItem { label: context.t(Key::ExampleMenuDuplicate) }
                    MenuItem { label: context.t(Key::ExampleMenuRename) }
                    MenuItem { label: context.t(Key::ExampleMenuArchive) }
                }
            }
        }
    ),
    example!(
        r"Menu { placement: MenuPlacement::End }",
        Key::ExampleMenuNotePlacement,
        |context| rsx! {
            div { class: "nd-example-end",
                {menu_button!(context, "example-menu-end")}
                Menu {
                    name: "example-menu-end",
                    label: context.t(Key::ExampleMenuLabel),
                    placement: MenuPlacement::End,
                    MenuItem { label: context.t(Key::ExampleMenuDuplicate) }
                    MenuItem { label: context.t(Key::ExampleMenuRename) }
                }
            }
        }
    ),
    example!(
        // A MENU LONG ENOUGH FOR THE WALK TO BE WORTH ANYTHING. This example
        // used to be example 1 with a fourth item, under a caption promising
        // *every key a menu owes* and markup showing none of them. Thirteen
        // items with thirteen different first letters is where Home, End and
        // the letter jump stop being claims and start being visible.
        r"Menu { name, label } with thirteen MenuItem { label }",
        Key::ExampleMenuNoteKeys,
        |context| rsx! {
            div {
                Button {
                    menu: true,
                    controls: "example-menu-keys",
                    variant: ButtonVariant::Outline,
                    tone: ButtonTone::Neutral,
                    label: context.t(Key::ExampleMenuAccentOpen),
                }
                Menu {
                    name: "example-menu-keys",
                    label: context.t(Key::ExampleMenuAccentLabel),
                    for accent in [
                        Key::ThemeAccentOchre,
                        Key::ThemeAccentAmber,
                        Key::ThemeAccentLime,
                        Key::ThemeAccentJade,
                        Key::ThemeAccentTeal,
                        Key::ThemeAccentAzure,
                        Key::ThemeAccentBlue,
                        Key::ThemeAccentIndigo,
                        Key::ThemeAccentViolet,
                        Key::ThemeAccentMagenta,
                        Key::ThemeAccentRose,
                        Key::ThemeAccentClay,
                        Key::ThemeAccentUmber,
                    ] {
                        MenuItem { key: "{accent.id()}", label: context.t(accent) }
                    }
                }
            }
        }
    ),
    example!(
        r"Menu, twice on one page",
        Key::ExampleMenuNoteOne,
        |context| rsx! {
            div { class: "nd-example-row",
                {menu_button!(context, "example-menu-first")}
                Menu {
                    name: "example-menu-first",
                    label: context.t(Key::ExampleMenuLabel),
                    MenuItem { label: context.t(Key::ExampleMenuDuplicate) }
                    MenuItem { label: context.t(Key::ExampleMenuRename) }
                }
                {menu_button!(context, "example-menu-second")}
                Menu {
                    name: "example-menu-second",
                    label: context.t(Key::ExampleMenuLabel),
                    MenuItem { label: context.t(Key::ExampleMenuArchive) }
                    MenuItem { label: context.t(Key::ExampleMenuDelete) }
                }
            }
        }
    ),
];

const MENU_ITEM: &[Example] = &[
    example!(
        r#"MenuItem { icon, label, action: "…", value: "…" }"#,
        Key::ExampleMenuNoteIcon,
        |context| rsx! {
            div {
                {menu_button!(context, "example-item-icon")}
                Menu {
                    name: "example-item-icon",
                    label: context.t(Key::ExampleMenuLabel),
                    MenuItem {
                        icon: "copy",
                        action: "duplicate",
                        value: "invoice-43",
                        label: context.t(Key::ExampleMenuDuplicate),
                    }
                    MenuItem {
                        icon: "download",
                        action: "archive",
                        value: "invoice-43",
                        label: context.t(Key::ExampleMenuArchive),
                    }
                }
            }
        }
    ),
    example!(
        r#"MenuItem { tone: "danger", detail: "…", action: "…" }"#,
        Key::ExampleMenuNoteItem,
        |context| rsx! {
            div {
                {menu_button!(context, "example-item-tone")}
                Menu {
                    name: "example-item-tone",
                    label: context.t(Key::ExampleMenuLabel),
                    MenuItem { label: context.t(Key::ExampleMenuDuplicate) }
                    MenuItem {
                        icon: "close",
                        tone: "danger",
                        // The hook the caller's own script binds to. This menu
                        // closes itself and returns the focus; what happens
                        // next is nobody else's business.
                        action: "example-menu-delete",
                        label: context.t(Key::ExampleMenuDelete),
                        detail: context.t(Key::ExampleMenuDeleteDetail),
                    }
                }
            }
        }
    ),
    example!(
        r"MenuItem { disabled: true }",
        Key::ExampleMenuNoteRefused,
        |context| rsx! {
            div {
                {menu_button!(context, "example-item-refused")}
                Menu {
                    name: "example-item-refused",
                    label: context.t(Key::ExampleMenuLabel),
                    MenuItem { label: context.t(Key::ExampleMenuDuplicate) }
                    MenuItem { disabled: true, label: context.t(Key::ExampleMenuLocked) }
                }
            }
        }
    ),
    example!(
        r#"MenuItem { href: "…", rel, target }"#,
        Key::ExampleMenuNoteLinks,
        |context| rsx! {
            div {
                {menu_button!(context, "example-item-links")}
                Menu {
                    name: "example-item-links",
                    label: context.t(Key::ExampleMenuLabel),
                    MenuItem { href: "#", label: context.t(Key::ExampleMenuDocs) }
                    MenuItem {
                        href: "https://github.com/noctua-world/noctua-design",
                        target: "_blank",
                        icon: "external-link",
                        label: context.t(Key::ExampleMenuRepo),
                    }
                }
            }
        }
    ),
];

// --- Select, SelectGroup and SelectOption -----------------------------------

// THE OPTIONS ARE THIS SYSTEM'S OWN PALETTES.
//
// Thirty-nine of them, which is the only list on this site long enough to need
// a select at all — and the one place a chip beside an option is the answer
// rather than a decoration. The names are identifiers and carry
// `nd-identifier`, so the pseudo-locale test reads them as the palette names
// they are; everything around them is a sentence and goes through the
// catalogue like anything else.

/// One palette option, since a dozen of them differ only in three strings.
///
/// The value IS the palette, so the chip beside the label takes its colour from
/// the palette it names and nothing here says a colour.
macro_rules! palette {
    ($value:literal, $label:literal) => {
        rsx! {
            SelectOption {
                class: "nd-identifier",
                value: $value,
                label: $label,
                swatch: $value,
            }
        }
    };
    ($value:literal, $label:literal, $detail:expr) => {
        rsx! {
            SelectOption {
                class: "nd-identifier",
                value: $value,
                label: $label,
                swatch: $value,
                detail: $detail,
            }
        }
    };
}

const SELECT: &[Example] = &[
    example!(
        r#"Select { name: "palette", label: "…", hint: "…", placeholder: "…" }"#,
        Key::ExampleSelectNoteNative,
        |context| rsx! {
            Select {
                name: "palette",
                label: context.t(Key::ExampleSelectPaletteLabel),
                hint: context.t(Key::ExampleSelectPaletteHint),
                placeholder: context.t(Key::ExampleSelectPalettePlaceholder),
                SelectGroup { label: context.t(Key::ExampleSelectWarm),
                    { palette!("ochre-balanced", "Ochre") }
                    { palette!("amber-vivid", "Amber") }
                    { palette!("clay-sober", "Clay") }
                }
                SelectGroup { label: context.t(Key::ExampleSelectCool),
                    { palette!("teal-balanced", "Teal") }
                    { palette!("azure-vivid", "Azure") }
                    { palette!("indigo-sober", "Indigo") }
                }
            }
        }
    ),
    example!(
        r"Select { searchable: true, search_label, empty_label, clearable: true, clear_label, size: SelectSize::Large }",
        Key::ExampleSelectNoteSearch,
        |context| rsx! {
            Select {
                name: "palette-search",
                label: context.t(Key::ExampleSelectPaletteLabel),
                placeholder: context.t(Key::ExampleSelectPalettePlaceholder),
                size: SelectSize::Large,
                searchable: true,
                clearable: true,
                search_label: context.t(Key::ExampleSelectSearchLabel),
                empty_label: context.t(Key::ExampleSelectEmptyLabel),
                clear_label: context.t(Key::ExampleSelectClearLabel),
                { palette!("ochre-balanced", "Ochre") }
                { palette!("amber-vivid", "Amber") }
                { palette!("clay-sober", "Clay") }
                { palette!("teal-balanced", "Teal") }
                { palette!("azure-vivid", "Azure") }
                { palette!("indigo-sober", "Indigo") }
            }
        }
    ),
    example!(
        r"Select { multiple: true, required: true, selected_label }",
        Key::ExampleSelectNoteMultiple,
        |context| rsx! {
            Select {
                name: "palette-compare",
                label: context.t(Key::ExampleSelectCompareLabel),
                hint: context.t(Key::ExampleSelectCompareHint),
                placeholder: context.t(Key::ExampleSelectPalettePlaceholder),
                multiple: true,
                required: true,
                // THE WORD IN FRONT OF A SENTENCE NOBODY HAS WRITTEN YET. This
                // field has no `error`: the sentence comes from the browser
                // when a reader submits without answering, and `Field` draws
                // this in front of whatever lands in the slot.
                error_label: context.t(Key::ExampleFormsErrorWord),
                searchable: true,
                search_label: context.t(Key::ExampleSelectSearchLabel),
                empty_label: context.t(Key::ExampleSelectEmptyLabel),
                selected_label: context.t(Key::ExampleSelectSelectedLabel),
                { palette!("ochre-balanced", "Ochre", context.t(Key::ExampleSelectDetailBalanced)) }
                { palette!("amber-vivid", "Amber", context.t(Key::ExampleSelectDetailVivid)) }
                { palette!("clay-sober", "Clay", context.t(Key::ExampleSelectDetailSober)) }
                { palette!("teal-balanced", "Teal", context.t(Key::ExampleSelectDetailBalanced)) }
            }
        }
    ),
    example!(
        r"Select { creatable: true, create_label }",
        Key::ExampleSelectNoteCreate,
        |context| rsx! {
            Select {
                name: "palette-new",
                label: context.t(Key::ExampleSelectPaletteLabel),
                placeholder: context.t(Key::ExampleSelectPalettePlaceholder),
                searchable: true,
                creatable: true,
                search_label: context.t(Key::ExampleSelectSearchLabel),
                empty_label: context.t(Key::ExampleSelectEmptyLabel),
                create_label: context.t(Key::ExampleSelectCreateLabel),
                { palette!("ochre-balanced", "Ochre") }
                { palette!("teal-balanced", "Teal") }
            }
        }
    ),
    example!(
        r"Select { required: true, … } inside a <form>",
        Key::ExampleSelectNoteForm,
        // THE ONLY EXAMPLE ON THIS SITE THAT IS INSIDE A `<form>`, and its
        // absence is what hid two defects for a whole stage: a `required`
        // control that is hidden cannot be reported, so the submit failed in
        // silence, and a reset repainted nothing and left every created option
        // behind. Both are only reachable through a form, and there was none.
        //
        // Press it. The answer arrives in the address bar, which is the claim
        // this component makes about itself made rather than described.
        |context| rsx! {
            form { class: "nd-example-form", method: "get",
                Select {
                    name: "palette-form",
                    label: context.t(Key::ExampleSelectPaletteLabel),
                    placeholder: context.t(Key::ExampleSelectPalettePlaceholder),
                    required: true,
                    searchable: true,
                    creatable: true,
                    search_label: context.t(Key::ExampleSelectSearchLabel),
                    empty_label: context.t(Key::ExampleSelectEmptyLabel),
                    create_label: context.t(Key::ExampleSelectCreateLabel),
                    { palette!("ochre-balanced", "Ochre") }
                    { palette!("teal-balanced", "Teal") }
                    { palette!("indigo-sober", "Indigo") }
                }
                div { class: "nd-example-row",
                    Button {
                        submit: ButtonSubmit::Submit,
                        label: context.t(Key::ExampleSelectSend),
                    }
                    Button {
                        submit: ButtonSubmit::Reset,
                        variant: ButtonVariant::Outline,
                        tone: ButtonTone::Neutral,
                        label: context.t(Key::ExampleSelectReset),
                    }
                }
            }
        }
    ),
    example!(
        r#"Select { error: "…", error_label: "…" }"#,
        Key::ExampleSelectNoteError,
        |context| rsx! {
            Select {
                name: "palette-error",
                label: context.t(Key::ExampleSelectPaletteLabel),
                placeholder: context.t(Key::ExampleSelectPalettePlaceholder),
                error: context.t(Key::ExampleSelectError),
                error_label: context.t(Key::ExampleFormsErrorWord),
                { palette!("ochre-balanced", "Ochre") }
                { palette!("teal-balanced", "Teal") }
            }
        }
    ),
    example!(
        r"Select { loading: true, loading_label, size: SelectSize::Small }",
        Key::ExampleSelectNoteLoading,
        |context| rsx! {
            Select {
                name: "palette-loading",
                label: context.t(Key::ExampleSelectPaletteLabel),
                hint: context.t(Key::ExampleSelectLoadingHint),
                placeholder: context.t(Key::ExampleSelectPalettePlaceholder),
                size: SelectSize::Small,
                loading: true,
                loading_label: context.t(Key::ExampleSelectLoadingLabel),
            }
        }
    ),
    // NOT ALSO DISABLED, which the loading example used to be.
    //
    // A disabled control cannot be opened, so the one sentence the loading
    // state exists to say was in the page and unreachable — a demonstration of
    // a feature that demonstrated nothing. `disabled` gets an example of its
    // own instead, where it is the whole point.
    example!(
        r"Select { disabled: true }",
        Key::ExampleSelectNoteDisabled,
        |context| rsx! {
            Select {
                name: "palette-off",
                label: context.t(Key::ExampleSelectPaletteLabel),
                placeholder: context.t(Key::ExampleSelectPalettePlaceholder),
                disabled: true,
                { palette!("ochre-balanced", "Ochre") }
                { palette!("teal-balanced", "Teal") }
            }
        }
    ),
];

const SELECT_GROUP: &[Example] = &[
    example!(
        r#"SelectGroup { label: "…" }"#,
        Key::ExampleSelectNoteGroup,
        |context| rsx! {
            Select {
                name: "group-one",
                label: context.t(Key::ExampleSelectPaletteLabel),
                placeholder: context.t(Key::ExampleSelectPalettePlaceholder),
                SelectGroup { label: context.t(Key::ExampleSelectWarm),
                    { palette!("ochre-balanced", "Ochre") }
                    { palette!("amber-vivid", "Amber") }
                }
            }
        }
    ),
    example!(r"Two runs, one list", |context| rsx! {
        Select {
            name: "group-two",
            label: context.t(Key::ExampleSelectPaletteLabel),
            placeholder: context.t(Key::ExampleSelectPalettePlaceholder),
            SelectGroup { label: context.t(Key::ExampleSelectWarm),
                { palette!("ochre-balanced", "Ochre") }
                { palette!("clay-sober", "Clay") }
            }
            SelectGroup { label: context.t(Key::ExampleSelectCool),
                { palette!("teal-balanced", "Teal") }
                { palette!("indigo-sober", "Indigo") }
            }
        }
    }),
    example!(
        r"SelectGroup { disabled: true, … }",
        Key::ExampleSelectNoteDisabledGroup,
        |context| rsx! {
            Select {
                name: "group-refused",
                label: context.t(Key::ExampleSelectPaletteLabel),
                placeholder: context.t(Key::ExampleSelectPalettePlaceholder),
                SelectGroup { label: context.t(Key::ExampleSelectWarm),
                    { palette!("ochre-balanced", "Ochre") }
                }
                SelectGroup { label: context.t(Key::ExampleSelectCool), disabled: true,
                    { palette!("teal-balanced", "Teal") }
                    { palette!("azure-vivid", "Azure") }
                }
            }
        }
    ),
    example!(r"A run beside ungrouped options", |context| rsx! {
        Select {
            name: "group-mixed",
            label: context.t(Key::ExampleSelectPaletteLabel),
            placeholder: context.t(Key::ExampleSelectPalettePlaceholder),
            { palette!("ochre-balanced", "Ochre") }
            SelectGroup { label: context.t(Key::ExampleSelectCool),
                { palette!("teal-balanced", "Teal") }
                { palette!("azure-vivid", "Azure") }
            }
        }
    }),
];

const SELECT_OPTION: &[Example] = &[
    example!(
        r#"SelectOption { value: "…", label: "…" }"#,
        Key::ExampleSelectNotePlain,
        |context| rsx! {
            Select {
                name: "option-plain",
                label: context.t(Key::ExampleSelectPaletteLabel),
                placeholder: context.t(Key::ExampleSelectPalettePlaceholder),
                SelectOption { class: "nd-identifier", value: "ochre-balanced", label: "Ochre" }
                SelectOption { class: "nd-identifier", value: "teal-balanced", label: "Teal" }
            }
        }
    ),
    example!(
        r#"SelectOption { detail: "…", swatch: "…" }"#,
        Key::ExampleSelectNoteRich,
        |context| rsx! {
            Select {
                name: "option-rich",
                label: context.t(Key::ExampleSelectPaletteLabel),
                placeholder: context.t(Key::ExampleSelectPalettePlaceholder),
                searchable: true,
                search_label: context.t(Key::ExampleSelectSearchLabel),
                empty_label: context.t(Key::ExampleSelectEmptyLabel),
                { palette!("amber-vivid", "Amber", context.t(Key::ExampleSelectDetailVivid)) }
                { palette!("ochre-balanced", "Ochre", context.t(Key::ExampleSelectDetailBalanced)) }
                { palette!("clay-sober", "Clay", context.t(Key::ExampleSelectDetailSober)) }
            }
        }
    ),
    example!(
        r"SelectOption { icon: IconName::Sun, … }",
        Key::ExampleSelectNoteIcon,
        |context| rsx! {
            Select {
                name: "option-icon",
                label: context.t(Key::ExampleSelectModeLabel),
                SelectOption {
                    value: "light",
                    label: context.t(Key::ExampleSelectModeLight),
                    icon: IconName::Sun,
                }
                SelectOption {
                    value: "dark",
                    label: context.t(Key::ExampleSelectModeDark),
                    icon: IconName::Moon,
                }
                SelectOption {
                    value: "system",
                    label: context.t(Key::ExampleSelectModeSystem),
                    icon: IconName::Monitor,
                }
            }
        }
    ),
    example!(
        r"SelectOption { selected: true, disabled: true }",
        Key::ExampleSelectNoteSelected,
        |context| rsx! {
            Select {
                name: "option-states",
                label: context.t(Key::ExampleSelectModeLabel),
                SelectOption {
                    value: "light",
                    label: context.t(Key::ExampleSelectModeLight),
                    icon: IconName::Sun,
                    selected: true,
                }
                SelectOption {
                    value: "dark",
                    label: context.t(Key::ExampleSelectModeDark),
                    icon: IconName::Moon,
                    disabled: true,
                }
            }
        }
    ),
];

// --- Code -----------------------------------------------------

const CODE: &[Example] = &[
    example!(r#"Code { "…" }"#, |context| rsx! {
        Paragraph {
            "{context.t(Key::ExampleCodeLead)} "
            Code { class: "nd-identifier", "--noctua-space-gap" }
            "."
        }
    }),
    example!(r"Code { tone: CodeTone::Subtle, … }", |context| rsx! {
        Paragraph {
            "{context.t(Key::ExampleCodeLead)} "
            Code { class: "nd-identifier", tone: CodeTone::Subtle, "--noctua-space-gap" }
            "."
        }
    }),
    example!(r"Heading { … Code { … } }", |context| rsx! {
        Heading { level: HeadingLevel::_3, size: HeadingSize::H4,
            Code { class: "nd-identifier", "ThemeScope" }
            " {context.t(Key::ExampleCodeInHeading)}"
        }
    }),
    example!(
        r"A long name, in a narrow column",
        Key::ExampleCodeLong,
        |_| rsx! {
            Prose { measure: ProseMeasure::Default,
                Paragraph {
                    Code { class: "nd-identifier", "--nc-accent-bg-element-hover" }
                }
            }
        }
    ),
];

// --- CodeBlock --------------------------------------------------------------

const CODE_BLOCK: &[Example] = &[
    example!(r#"CodeBlock { code: "…", language: "rust" }"#, |_| rsx! {
        CodeBlock { code: SAMPLE_RUST, language: "rust" }
    }),
    example!(
        r#"CodeBlock { language: "…" }, thirty-four of them"#,
        Key::ExampleCodeBlockLanguages,
        |_| rsx! {
            div { class: "nd-example-stack",
                CodeBlock { code: SAMPLE_TS, language: "typescript" }
                CodeBlock { code: SAMPLE_PYTHON, language: "python" }
                CodeBlock { code: SAMPLE_SHELL, language: "sh" }
            }
        }
    ),
    example!(
        r#"CodeBlock { code: "…", language: "rust", theme: CodeBlockTheme::… }"#,
        Key::ExampleCodeBlockThemes,
        |_| rsx! {
            // NO `filename` HERE, AND THAT IS THE REPAIR. These four carried
            // `spectrum`, `duotone`, `muted` and `mono`, which are theme names
            // and not files. `filename` is also the accessible name of the
            // scrollable region, so a wrong one is a wrong name announced.
            // With it absent the name falls back on the language, which is
            // what the contract promises.
            div { class: "nd-example-stack",
                // The four that override, and not `auto`, which is the
                // default and would show the prop doing nothing.
                for theme in [
                    CodeBlockTheme::Spectrum,
                    CodeBlockTheme::Duotone,
                    CodeBlockTheme::Muted,
                    CodeBlockTheme::Mono,
                ] {
                    CodeBlock {
                        key: "{theme.as_str()}",
                        code: SAMPLE_RUST,
                        language: "rust",
                        theme,
                    }
                }
            }
        }
    ),
    example!(
        r#"CodeBlock { numbered: true, start_line: "…", mark_lines: "…", focus: true }"#,
        Key::ExampleCodeBlockLines,
        |_| rsx! {
            CodeBlock {
                code: SAMPLE_RUST,
                language: "rust",
                numbered: true,
                start_line: "48",
                mark_lines: "50-51",
                focus: true,
            }
        }
    ),
    example!(
        r#"CodeBlock { language: "diff", … }"#,
        Key::ExampleCodeBlockDiff,
        |_| rsx! {
            CodeBlock { code: SAMPLE_DIFF, language: "diff", filename: "spec/tokens.toml" }
        }
    ),
    example!(
        r#"CodeBlock { filename: "…", copy: "…", copied: "…", wrap_label: "…", caption: "…" }"#,
        Key::ExampleCodeBlockBar,
        |context| rsx! {
            CodeBlock {
                code: SAMPLE_LONG,
                language: "rust",
                filename: "scope.rs",
                copy: context.t(Key::ExampleCopy),
                copied: context.t(Key::ExampleCopied),
                wrap_label: context.t(Key::ExampleCodeBlockWrapLabel),
                caption: context.t(Key::ExampleCodeBlockCaption),
            }
        }
    ),
    example!(
        r"CodeBlock { wrap: true, … }",
        Key::ExampleCodeBlockWrap,
        |_| rsx! {
            CodeBlock { code: SAMPLE_LONG, language: "rust", wrap: true }
        }
    ),
    example!(
        r#"CodeBlock { max_lines: "…", expand_label: "…", collapse_label: "…" }"#,
        Key::ExampleCodeBlockCollapse,
        |context| rsx! {
            CodeBlock {
                code: SAMPLE_TALL,
                language: "rust",
                numbered: true,
                max_lines: "6",
                expand_label: context.t(Key::ExampleCodeBlockExpand),
                collapse_label: context.t(Key::ExampleCodeBlockCollapseLabel),
            }
        }
    ),
    example!(
        r"A language nothing recognises",
        Key::ExampleCodeBlockUnknown,
        |_| rsx! {
            CodeBlock { code: SAMPLE_RUST, language: "brainfuck" }
        }
    ),
    example!(
        r"CodeBlock { glass: true, … }",
        Key::ExampleGlassBehind,
        |_| rsx! {
            CodeBlock { code: SAMPLE_RUST, language: "rust", glass: true }
        }
    ),
    example!(r"Language::ALL", Key::ExampleCodeBlockEvery, |_| rsx! {
        // READ FROM THE GENERATED SET, never listed here. A second copy of
        // `spec/grammars/` is a second copy that stops agreeing, and this
        // is the page a reader comes to to find out what they may pass.
        ul { class: "nd-language-index",
            for language in Language::ALL {
                li {
                    Code { class: "nd-identifier", "{language.as_str()}" }
                    " "
                    // `nd-identifier`, because a language's own name is a
                    // proper noun rather than a sentence: "Rust" is "Rust" in
                    // every locale, and the translation scan skips a subtree
                    // that says so — the same call the token names on the
                    // spacing page already make.
                    Text {
                        class: "nd-identifier",
                        size: TextSize::Caption,
                        tone: TextTone::Muted,
                        "{language.display()}"
                    }
                }
            }
        }
    }),
];

/// Real code in five languages, because a sample nobody would write teaches
/// nothing — and because a grammar's traps only show on code that has them.
const SAMPLE_RUST: &str = r"/// Every attribute a scope stamps, or none.
pub fn scope_attributes(request: &ScopeRequest) -> Option<Attributes> {
    let resolved = resolve(request)?;
    if resolved == inherited {
        return None; // a no-op scope is 1,767 declarations for nothing
    }
    Some(Attributes { palette: resolved.palette, mode: resolved.mode })
}";

const SAMPLE_TS: &str = r"export interface Token {
  readonly role: Role;
  readonly text: string;
}

export function total(tokens: Token[]): number {
  return tokens.reduce((sum, token) => sum + token.text.length, 0);
}";

const SAMPLE_PYTHON: &str = r#"class Palette:
    """A palette, and the mode it was solved in."""

    MAX_STEPS = 12

    def __init__(self, name: str, mode: str = "light") -> None:
        self.name = name
        self.mode = mode

    def step(self, n):
        if n < 1 or n > self.MAX_STEPS:
            raise ValueError(f"{n} is outside 1..{self.MAX_STEPS}")
        return self._steps[n - 1]"#;

const SAMPLE_SHELL: &str = r#"#!/usr/bin/env bash
set -euo pipefail

for palette in "$PALETTES"/*.css; do
  name="$(basename "$palette" .css)"
  echo "checking ${name}" >&2
  grep -c -- '--nc-' "$palette" || exit 1
done"#;

const SAMPLE_DIFF: &str = r#"--- a/spec/tokens.toml
+++ b/spec/tokens.toml
@@ -12,7 +12,7 @@
 [space]
 base = "0.25rem"
-steps = [0, 1, 2, 3, 4, 6, 8]
+steps = [0, 1, 2, 3, 4, 6, 8, 12, 16]
 
 [space.named]"#;

/// One line, longer than any column this site has.
const SAMPLE_LONG: &str =
    "let attributes = scope_attributes(request, inherited).expect(\"a change worth stamping\");";

/// Long enough that the collapse has something to collapse.
const SAMPLE_TALL: &str = r"pub enum Role {
    Plain,
    Operator,
    Keyword,
    Punctuation,
    Comment,
    String,
    Escape,
    Number,
    Constant,
    Type,
    Tag,
    Function,
    Property,
    Attribute,
    Inserted,
    Deleted,
    Invalid,
}";

// --- Del and Ins ------------------------------------------------------------

const DEL: &[Example] = &[
    example!(r#"Del { "…" }"#, |context| rsx! {
        Paragraph {
            Del { "{context.t(Key::ExampleEditOld)}" }
            " "
            "{context.t(Key::ExampleEditNew)}"
        }
    }),
    example!(
        r#"Del { label: "removed", … }"#,
        Key::ExampleEditLabel,
        |context| rsx! {
            Paragraph {
                Del { label: context.t(Key::ExampleEditRemoved), "{context.t(Key::ExampleEditOld)}" }
            }
        }
    ),
    example!(
        r#"Del { datetime: "2026-07-29", cite: "…", … }"#,
        |context| rsx! {
            Paragraph {
                Del {
                    datetime: "2026-07-29",
                    cite: "https://example.invalid/change-1",
                    label: context.t(Key::ExampleEditRemoved),
                    "{context.t(Key::ExampleEditOld)}"
                }
            }
        }
    ),
    example!(r"Del beside Ins", Key::ExampleEditPair, |context| rsx! {
        Paragraph {
            Del { label: context.t(Key::ExampleEditRemoved), "{context.t(Key::ExampleEditOld)}" }
            " "
            Ins { label: context.t(Key::ExampleEditAdded), "{context.t(Key::ExampleEditNew)}" }
        }
    }),
];

const INS: &[Example] = &[
    example!(r#"Ins { "…" }"#, |context| rsx! {
        Paragraph {
            Ins { "{context.t(Key::ExampleEditNew)}" }
        }
    }),
    example!(
        r#"Ins { label: "added", … }"#,
        Key::ExampleEditLabel,
        |context| rsx! {
            Paragraph {
                Ins { label: context.t(Key::ExampleEditAdded), "{context.t(Key::ExampleEditNew)}" }
            }
        }
    ),
    example!(
        r#"Ins { datetime: "2026-07-29", cite: "…", … }"#,
        |context| rsx! {
            Paragraph {
                Ins {
                    datetime: "2026-07-29",
                    cite: "https://example.invalid/change-2",
                    label: context.t(Key::ExampleEditAdded),
                    "{context.t(Key::ExampleEditNew)}"
                }
            }
        }
    ),
    // NOT `Del` BESIDE `Ins` AGAIN. That example is on `Del`'s own table with
    // the same note key, and one question asked twice on one page is one of
    // the two examples doing nothing. This one is `Ins` around a whole item.
    example!(
        r"List { ListItem { Ins { label, … } } }",
        Key::ExampleEditBlock,
        |context| rsx! {
            List { kind: ListKind::Unordered, spacing: ListSpacing::Tight,
                ListItem { "{context.t(Key::ExampleListOne)}" }
                ListItem {
                    Ins {
                        label: context.t(Key::ExampleEditAdded),
                        "{context.t(Key::ExampleListTwo)}"
                    }
                }
            }
        }
    ),
];

// --- Disclosure -------------------------------------------------------------

const DISCLOSURE: &[Example] = &[
    example!(r#"Disclosure { summary: "…", … }"#, |context| rsx! {
        Disclosure { summary: context.t(Key::ExampleDisclosureSummary),
            Paragraph { "{context.t(Key::ExampleDisclosureBody)}" }
        }
    }),
    example!(
        r"Disclosure { open: true, … }",
        Key::ExampleDisclosureOpen,
        |context| rsx! {
            Disclosure { summary: context.t(Key::ExampleDisclosureSummary), open: true,
                Paragraph { "{context.t(Key::ExampleDisclosureBody)}" }
            }
        }
    ),
    example!(r"Disclosure { marker: false, … }", |context| rsx! {
        Disclosure { summary: context.t(Key::ExampleDisclosureSummary), marker: false,
            Paragraph { "{context.t(Key::ExampleDisclosureBody)}" }
        }
    }),
    example!(
        r#"Disclosure { icon: "settings", … }"#,
        Key::ExampleDisclosureIcon,
        |context| rsx! {
            Disclosure {
                summary: context.t(Key::SettingsLabel),
                icon: IconName::Settings,
                Paragraph { "{context.t(Key::ExampleDisclosureBody)}" }
            }
        }
    ),
    example!(
        r#"Disclosure { name: "…", … }"#,
        Key::ExampleDisclosureName,
        |context| rsx! {
            Disclosure { summary: context.t(Key::ExampleDisclosureSummary), name: "example",
                List { kind: ListKind::Unordered, spacing: ListSpacing::Tight,
                    ListItem { "{context.t(Key::ExampleListOne)}" }
                    ListItem { "{context.t(Key::ExampleListTwo)}" }
                }
            }
        }
    ),
    example!(
        r"Disclosure { heading: DisclosureHeading::H3, … }",
        Key::ExampleDisclosureHeading,
        |context| rsx! {
            Disclosure {
                summary: context.t(Key::ExampleDisclosureSection),
                heading: DisclosureHeading::H3,
                Paragraph { "{context.t(Key::ExampleDisclosureBody)}" }
            }
        }
    ),
    example!(
        r"Disclosure { glass: true, … }",
        Key::ExampleGlassBehind,
        |context| rsx! {
            Disclosure {
                summary: context.t(Key::ExampleDisclosureSummary),
                glass: true,
                open: true,
                Paragraph { "{context.t(Key::ExampleDisclosureBody)}" }
            }
        }
    ),
];

// --- Em and Strong ----------------------------------------------------------

const EM: &[Example] = &[
    example!(r#"Em { "…" }"#, |context| rsx! {
        Paragraph {
            "{context.t(Key::ExampleEmLead)} "
            Em { "{context.t(Key::ExampleEmWord)}" }
            "."
        }
    }),
    example!(
        r"Strong { Em { … } }",
        Key::ExampleEmNested,
        |context| rsx! {
            Paragraph {
                Strong {
                    "{context.t(Key::ExampleStrongLead)} "
                    Em { "{context.t(Key::ExampleEmWord)}" }
                }
            }
        }
    ),
    example!(r"Em inside a heading", |context| rsx! {
        Heading { level: HeadingLevel::_3, size: HeadingSize::H4,
            "{context.t(Key::ExampleEmLead)} "
            Em { "{context.t(Key::ExampleEmWord)}" }
        }
    }),
    example!(r"Em beside Strong", Key::ExampleEmVersus, |context| rsx! {
        Paragraph {
            Em { "{context.t(Key::ExampleEmWord)}" }
            " · "
            Strong { "{context.t(Key::ExampleStrongWord)}" }
        }
    }),
];

const STRONG: &[Example] = &[
    example!(r#"Strong { "…" }"#, |context| rsx! {
        Paragraph {
            "{context.t(Key::ExampleStrongLead)} "
            Strong { "{context.t(Key::ExampleStrongWord)}" }
            "."
        }
    }),
    // NOT `Strong { Em { … } }` AGAIN. `Em`'s own table asks that question,
    // under this very note key. The question `Strong` alone can answer is what
    // a second `Strong` inside the first one means.
    example!(
        r"Strong { … Strong { … } }",
        Key::ExampleStrongDegrees,
        |context| rsx! {
            Paragraph {
                Strong {
                    "{context.t(Key::ExampleStrongLead)} "
                    Strong { "{context.t(Key::ExampleStrongWord)}" }
                }
            }
        }
    ),
    example!(r"Strong inside a list", |context| rsx! {
        List { kind: ListKind::Unordered, spacing: ListSpacing::Tight,
            ListItem {
                Strong { "{context.t(Key::ExampleStrongWord)}" }
                " — {context.t(Key::ExampleListOne)}"
            }
            ListItem { "{context.t(Key::ExampleListTwo)}" }
        }
    }),
    // `Em` beside `Strong` is on `Em`'s table already. What belongs here is
    // the case the element exists for: the part of a sentence that matters
    // most, which is usually the word a warning opens on.
    example!(
        r"Strong opening a warning",
        Key::ExampleStrongWarning,
        |context| rsx! {
            Paragraph {
                Strong { "{context.t(Key::ExampleStrongWarningWord)}" }
                " {context.t(Key::ExampleStrongWarningBody)}"
            }
        }
    ),
];

// --- Heading ----------------------------------------------------------------

const HEADING: &[Example] = &[
    example!(
        r"An outline: levels 2, 3 and 4",
        Key::ExampleHeadingOutline,
        |context| rsx! {
            Heading { level: HeadingLevel::_2, "{context.t(Key::ExampleHeadingSection)}" }
            Heading { level: HeadingLevel::_3, size: HeadingSize::H3,
                "{context.t(Key::ExampleHeadingSub)}"
            }
            Heading { level: HeadingLevel::_4, size: HeadingSize::H4,
                "{context.t(Key::ExampleHeadingDetail)}"
            }
        }
    ),
    example!(
        r"Heading { level: _3, size: HeadingSize::Display, … }",
        Key::ExampleHeadingIndependent,
        |context| rsx! {
            Heading { level: HeadingLevel::_3, size: HeadingSize::Display,
                "{context.t(Key::ExampleHeadingSection)}"
            }
        }
    ),
    example!(r"Heading { tone: HeadingTone::Muted, … }", |context| rsx! {
        Heading { level: HeadingLevel::_3, size: HeadingSize::H3, tone: HeadingTone::Muted,
            "{context.t(Key::ExampleHeadingSub)}"
        }
    }),
    example!(
        r"Heading { tone: HeadingTone::Accent, … }",
        |context| rsx! {
            Heading { level: HeadingLevel::_3, size: HeadingSize::H3, tone: HeadingTone::Accent,
                "{context.t(Key::ExampleHeadingSub)}"
            }
        }
    ),
    example!(
        r"Heading { balance: true, … }",
        Key::ExampleHeadingBalance,
        |context| rsx! {
            Prose { measure: ProseMeasure::Default,
                Heading { level: HeadingLevel::_3, size: HeadingSize::H2, balance: true,
                    "{context.t(Key::ExampleHeadingLong)}"
                }
            }
        }
    ),
];

// --- Kbd --------------------------------------------------------------------

const KBD: &[Example] = &[
    example!(r#"Kbd { "Esc" }"#, |_| rsx! {
        Kbd { class: "nd-identifier", "Esc" }
    }),
    example!(r"A chord", Key::ExampleKbdChord, |_| rsx! {
        span {
            Kbd { class: "nd-identifier", "Ctrl" }
            " + "
            Kbd { class: "nd-identifier", "K" }
        }
    }),
    example!(r"Kbd { size: KbdSize::Body, … }", |_| rsx! {
        Kbd { class: "nd-identifier", size: KbdSize::Body, "Enter" }
    }),
    example!(r"In a sentence", |context| rsx! {
        Paragraph {
            "{context.t(Key::ExampleKbdLead)} "
            Kbd { class: "nd-identifier", "Esc" }
            "."
        }
    }),
];

// --- Link -------------------------------------------------------------------

const LINK: &[Example] = &[
    example!(r#"Link { href: "…", "…" }"#, |context| rsx! {
        Paragraph {
            Link { href: "#", "{context.t(Key::ExampleLinkText)}" }
        }
    }),
    example!(
        r#"Link { external: true, rel: "noreferrer", … }"#,
        Key::ExampleLinkExternal,
        |context| rsx! {
            Paragraph {
                Link {
                    href: "https://noctua-world.github.io/noctua-colors/",
                    external: true,
                    rel: "noreferrer",
                    "{context.t(Key::ColorsSystemLink)}"
                }
            }
        }
    ),
    example!(
        r"Link { tone: LinkTone::Inherit, … }",
        Key::ExampleLinkInherit,
        |context| rsx! {
            Paragraph { tone: ParagraphTone::Muted,
                "{context.t(Key::ExampleLinkLead)} "
                Link { href: "#", tone: LinkTone::Inherit, "{context.t(Key::ExampleLinkText)}" }
                "."
            }
        }
    ),
    example!(r"In a sentence", |context| rsx! {
        Paragraph {
            "{context.t(Key::ExampleLinkLead)} "
            Link { href: "#", "{context.t(Key::ExampleLinkText)}" }
            "."
        }
    }),
];

// --- List and ListItem ------------------------------------------------------

const LIST: &[Example] = &[
    // NO `kind` IN THE CALL AND NONE IN THE CAPTION. `unordered` is the
    // default, so the caption named a prop the body never passed. The two
    // examples below pass the two values that change the element.
    example!(r"List { ListItem { … } × 3 }", |context| rsx! {
        List {
            ListItem { "{context.t(Key::ExampleListOne)}" }
            ListItem { "{context.t(Key::ExampleListTwo)}" }
            ListItem { "{context.t(Key::ExampleListThree)}" }
        }
    }),
    example!(
        r"List { kind: ListKind::Ordered, … }",
        Key::ExampleListOrdered,
        |context| rsx! {
            List { kind: ListKind::Ordered,
                ListItem { "{context.t(Key::ExampleListOne)}" }
                ListItem { "{context.t(Key::ExampleListTwo)}" }
                ListItem { "{context.t(Key::ExampleListThree)}" }
            }
        }
    ),
    example!(
        r"List { kind: ListKind::Description, … }",
        Key::ExampleListDescription,
        |context| rsx! {
            List { kind: ListKind::Description,
                ListItem { term: context.t(Key::ThemeAccentLabel), "{context.t(Key::ExampleTermAccent)}" }
                ListItem { term: context.t(Key::ThemeGlassLabel), "{context.t(Key::ExampleTermGlass)}" }
            }
        }
    ),
    example!(
        r"List { marker: ListMarker::None, … }",
        Key::ExampleListMarker,
        |context| rsx! {
            List { marker: ListMarker::None,
                ListItem { "{context.t(Key::ExampleListOne)}" }
                ListItem { "{context.t(Key::ExampleListTwo)}" }
            }
        }
    ),
    example!(r"List { spacing: ListSpacing::Loose, … }", |context| rsx! {
        List { spacing: ListSpacing::Loose,
            ListItem { "{context.t(Key::ExampleListOne)}" }
            ListItem { "{context.t(Key::ExampleListTwo)}" }
        }
    }),
    example!(
        r"A list inside a list",
        Key::ExampleListNested,
        |context| rsx! {
            List {
                ListItem {
                    "{context.t(Key::ExampleListOne)}"
                    List { spacing: ListSpacing::Tight,
                        ListItem { "{context.t(Key::ExampleListTwo)}" }
                        ListItem { "{context.t(Key::ExampleListThree)}" }
                    }
                }
            }
        }
    ),
];

const LIST_ITEM: &[Example] = &[
    example!(r#"ListItem { "…" }"#, |context| rsx! {
        List { spacing: ListSpacing::Tight,
            ListItem { "{context.t(Key::ExampleListOne)}" }
        }
    }),
    example!(
        r#"ListItem { term: "…", "…" }"#,
        Key::ExampleItemTerm,
        |context| rsx! {
            List { kind: ListKind::Description,
                ListItem { term: context.t(Key::ThemeGlassLabel), "{context.t(Key::ExampleTermGlass)}" }
            }
        }
    ),
    example!(r"An item holding a whole paragraph", |context| rsx! {
        List { spacing: ListSpacing::Loose,
            ListItem {
                Paragraph { "{context.t(Key::ExampleListLong)}" }
            }
        }
    }),
    example!(r"An item holding a link", |context| rsx! {
        List { spacing: ListSpacing::Tight,
            ListItem {
                Link { href: "#", "{context.t(Key::ExampleLinkText)}" }
            }
        }
    }),
];

// --- Mark -------------------------------------------------------------------

const MARK: &[Example] = &[
    example!(r#"Mark { "…" }"#, |context| rsx! {
        Paragraph {
            "{context.t(Key::ExampleMarkLead)} "
            Mark { "{context.t(Key::ExampleMarkWord)}" }
            "."
        }
    }),
    example!(
        r"Two hits in one line",
        Key::ExampleMarkSearch,
        |context| rsx! {
            Paragraph {
                Mark { "{context.t(Key::ExampleMarkWord)}" }
                " — {context.t(Key::ExampleMarkLead)} "
                Mark { "{context.t(Key::ExampleMarkWord)}" }
            }
        }
    ),
    example!(r"Mark inside a heading", |context| rsx! {
        Heading { level: HeadingLevel::_3, size: HeadingSize::H4,
            Mark { "{context.t(Key::ExampleMarkWord)}" }
        }
    }),
    example!(r"Mark inside a list", |context| rsx! {
        List { spacing: ListSpacing::Tight,
            ListItem {
                Mark { "{context.t(Key::ExampleMarkWord)}" }
                " — {context.t(Key::ExampleListOne)}"
            }
        }
    }),
];

// --- Paragraph and Prose ----------------------------------------------------

const PARAGRAPH: &[Example] = &[
    example!(r"Paragraph { … }", |context| rsx! {
        Paragraph { "{context.t(Key::ExampleListLong)}" }
    }),
    example!(
        r"Paragraph { size: ParagraphSize::Lead, … }",
        Key::ExampleParagraphLead,
        |context| rsx! {
            Paragraph { size: ParagraphSize::Lead, "{context.t(Key::ExampleListLong)}" }
        }
    ),
    example!(
        r"Paragraph { size: ParagraphSize::Small, … }",
        |context| rsx! {
            Paragraph { size: ParagraphSize::Small, "{context.t(Key::ExampleListLong)}" }
        }
    ),
    example!(
        r"Paragraph { tone: ParagraphTone::Muted, … }",
        |context| rsx! {
            Paragraph { tone: ParagraphTone::Muted, "{context.t(Key::ExampleListLong)}" }
        }
    ),
    example!(r"Two, one after the other", |context| rsx! {
        div {
            Paragraph { "{context.t(Key::ExampleListLong)}" }
            Paragraph { tone: ParagraphTone::Muted, "{context.t(Key::ExampleListLong)}" }
        }
    }),
];

const PROSE: &[Example] = &[
    example!(r"Prose { … }", Key::ExampleProseMeasure, |context| rsx! {
        Prose {
            Heading { level: HeadingLevel::_3, size: HeadingSize::H3,
                "{context.t(Key::ExampleHeadingSection)}"
            }
            Paragraph { "{context.t(Key::ExampleListLong)}" }
            Paragraph { "{context.t(Key::ExampleListLong)}" }
        }
    }),
    example!(
        r"Prose { measure: ProseMeasure::Lead, … }",
        |context| rsx! {
            Prose { measure: ProseMeasure::Lead,
                Paragraph { size: ParagraphSize::Lead, "{context.t(Key::ExampleListLong)}" }
            }
        }
    ),
    example!(
        r"Prose { measure: ProseMeasure::None, … }",
        Key::ExampleProseNone,
        |context| rsx! {
            Prose { measure: ProseMeasure::None,
                Paragraph { "{context.t(Key::ExampleListLong)}" }
            }
        }
    ),
    example!(r"A whole reading column", |context| rsx! {
        Prose {
            Heading { level: HeadingLevel::_3, size: HeadingSize::H3,
                "{context.t(Key::ExampleHeadingSection)}"
            }
            Paragraph { "{context.t(Key::ExampleListLong)}" }
            List { spacing: ListSpacing::Tight,
                ListItem { "{context.t(Key::ExampleListOne)}" }
                ListItem { "{context.t(Key::ExampleListTwo)}" }
            }
            Blockquote {
                Paragraph { "{context.t(Key::ExampleQuoteShort)}" }
            }
        }
    }),
];

// --- Search, SearchGroup and SearchResult -----------------------------------

// EVERY EXAMPLE HERE SEARCHES THIS SITE, and that is not decoration.
//
// A search demonstrated over three invented rows shows the box and hides the
// component: ranking needs enough candidates to rank, a highlight needs a name
// long enough to match partway into, and the `src` form needs an index that is
// really at a URL. So the children examples carry real pages of this site, and
// the palette example fetches the same file the sidebar's trigger does.

/// The results the children examples show, since they differ only in the props
/// on the box above them.
///
/// Real links to real pages: this is the half of the component that works with
/// the script blocked, and an example that faked it would be an example of
/// nothing. The page rows carry the page's own translated title and summary;
/// the component rows carry a name, which is the same word in every language
/// and says so with `nd-identifier`.
///
/// WHICH GROUP A ROW GOES IN FOLLOWS THAT, and `Search` moved because of it.
/// It sat among the pages with a translated label, back when `/search/` was
/// *Busca* in Portuguese — and `site::group_label` never agreed, because the
/// real index has always grouped it by `Kind::Component`. Its title is a name
/// now, so the example says what the index says.
macro_rules! pages {
    ($context:expr) => {
        rsx! {
            SearchGroup { label: $context.t(Key::SearchGroupPages),
                SearchResult {
                    href: "../spacing/",
                    label: crate::site::SPACING.title($context.locale),
                    detail: $context.t(Key::PageSpacingSummary),
                    // Letters a reader might type that the name does not
                    // contain. `gap` reaches this row and nothing else.
                    keywords: "gap inset step scale rhythm",
                }
                SearchResult {
                    href: "../typography/",
                    label: crate::site::TYPOGRAPHY.title($context.locale),
                    detail: $context.t(Key::PageTypographySummary),
                }
                SearchResult {
                    href: "../intro/",
                    label: crate::site::INTRO.title($context.locale),
                    detail: $context.t(Key::PageIntroSummary),
                }
            }
            SearchGroup { label: $context.t(Key::SearchGroupComponents),
                SearchResult { class: "nd-identifier", href: "../button/", label: "Button" }
                SearchResult { class: "nd-identifier", href: "../button-group/", label: "ButtonGroup" }
                SearchResult { class: "nd-identifier", href: "../checkbox/", label: "Checkbox" }
                // The page this example is on, so `current` is a claim the
                // document can back rather than a decoration.
                SearchResult {
                    class: "nd-identifier",
                    href: "../search/",
                    label: crate::site::SEARCH.title($context.locale),
                    current: true,
                }
                SearchResult { class: "nd-identifier", href: "../segmented/", label: "Segmented" }
                SearchResult { class: "nd-identifier", href: "../select/", label: "Select" }
                SearchResult { class: "nd-identifier", href: "../switch/", label: "Switch" }
                SearchResult { class: "nd-identifier", href: "../tables/", label: "Table" }
            }
        }
    };
}

const SEARCH: &[Example] = &[
    example!(
        r#"Search { name: "…", label: "…", placeholder: "…", status: "one:… |other:…" }"#,
        Key::ExampleSearchChildren,
        |context| rsx! {
            Search {
                name: "children",
                label: context.t(Key::SearchLabel),
                placeholder: context.t(Key::SearchPlaceholder),
                status: context.t(Key::SearchResults),
                empty_label: context.t(Key::SearchEmpty),
                clear_label: context.t(Key::SearchClear),
                {pages!(context)}
            }
        }
    ),
    example!(
        r#"Search { presentation: SearchPresentation::Popover, value: "but", cap: "3", cap_label: "…" }"#,
        Key::ExampleSearchPopover,
        |context| rsx! {
            Search {
                name: "popover",
                presentation: SearchPresentation::Popover,
                label: context.t(Key::SearchLabel),
                placeholder: context.t(Key::SearchPlaceholder),
                value: "but",
                cap: "3",
                status: context.t(Key::SearchResults),
                cap_label: context.t(Key::SearchShowing),
                empty_label: context.t(Key::SearchEmpty),
                clear_label: context.t(Key::SearchClear),
                {pages!(context)}
            }
        }
    ),
    example!(
        r#"Search { presentation: SearchPresentation::Dialog, src: "…", hotkey: true, recent: "…", status, empty_label, loading_label, error_label, cap_label, clear_label, recent_label, forget_label, close_label }"#,
        Key::ExampleSearchDialog,
        |context| rsx! {
            Search {
                name: "palette",
                presentation: SearchPresentation::Dialog,
                label: context.t(Key::SearchLabel),
                placeholder: context.t(Key::SearchPlaceholder),
                // The site's own index, fetched once and shared with the
                // sidebar's trigger: one file, whichever box asks for it.
                src: format!("{}assets/search-{}.json", crate::site::BASE, context.locale.tag()),
                hotkey: true,
                cap: "10",
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
    ),
    example!(
        r#"Search { name, action: "…", query: "term", label, placeholder, clear_label }"#,
        Key::ExampleSearchForm,
        |context| rsx! {
            Search {
                name: "form",
                action: "../search/",
                // NOT `q`, WHICH IS THE DEFAULT. A field posting the name it
                // would have posted anyway is an example of nothing, and this
                // is the one prop on the page a server has to agree with.
                query: "term",
                label: context.t(Key::SearchLabel),
                placeholder: context.t(Key::SearchPlaceholder),
                clear_label: context.t(Key::SearchClear),
            }
        }
    ),
    example!(
        r"Search { filter: true }",
        Key::ExampleSearchFilter,
        // A `<section>` rather than a `<div>`, and that is what bounds the
        // filtering: the script hides the `data-nd-index` elements inside the
        // nearest landmark, so without one this box would reach every other
        // example on the page.
        |context| rsx! {
            section { class: "nd-example-filter",
                Search {
                    name: "filter",
                    filter: true,
                    label: context.t(Key::SearchLabel),
                    placeholder: context.t(Key::SearchPlaceholder),
                    clear_label: context.t(Key::SearchClear),
                }
                ul { class: "nd-example-filtered",
                    for name in ["Button", "ButtonGroup", "Checkbox", "Select", "Switch", "Table"] {
                        li { key: "{name}", "data-nd-index": "{name}",
                            Code { class: "nd-identifier", "{name}" }
                        }
                    }
                }
            }
        }
    ),
    example!(
        r"Search { size: SearchSize::Small } / Medium / Large",
        Key::ExampleSearchSizes,
        |context| rsx! {
            div { class: "nd-example-stack",
                Search {
                    name: "small",
                    size: SearchSize::Small,
                    label: context.t(Key::SearchLabel),
                    placeholder: context.t(Key::SearchPlaceholder),
                }
                Search {
                    name: "medium",
                    label: context.t(Key::SearchLabel),
                    placeholder: context.t(Key::SearchPlaceholder),
                }
                Search {
                    name: "large",
                    size: SearchSize::Large,
                    label: context.t(Key::SearchLabel),
                    placeholder: context.t(Key::SearchPlaceholder),
                }
            }
        }
    ),
    example!(
        r"Search { loading: true } / Search { disabled: true } / Search { clearable: false }",
        Key::ExampleSearchStates,
        |context| rsx! {
            div { class: "nd-example-stack",
                Search {
                    name: "loading",
                    loading: true,
                    label: context.t(Key::SearchLabel),
                    placeholder: context.t(Key::SearchPlaceholder),
                    loading_label: context.t(Key::SearchLoading),
                    status: context.t(Key::SearchResults),
                    {pages!(context)}
                }
                Search {
                    name: "refused",
                    disabled: true,
                    value: "azure",
                    label: context.t(Key::SearchLabel),
                    placeholder: context.t(Key::SearchPlaceholder),
                }
                Search {
                    name: "bare",
                    clearable: false,
                    label: context.t(Key::SearchLabel),
                    placeholder: context.t(Key::SearchPlaceholder),
                }
            }
        }
    ),
    example!(
        r"Search { glass: true }",
        Key::ExampleGlassBehind,
        |context| rsx! {
            Search {
                name: "frosted",
                glass: true,
                label: context.t(Key::SearchLabel),
                placeholder: context.t(Key::SearchPlaceholder),
                status: context.t(Key::SearchResults),
                empty_label: context.t(Key::SearchEmpty),
                clear_label: context.t(Key::SearchClear),
                {pages!(context)}
            }
        }
    ),
];

const SEARCH_GROUP: &[Example] = &[
    example!(
        r#"SearchGroup { label: "…" }"#,
        Key::ExampleSearchGroup,
        |context| rsx! {
            Search {
                name: "group-one",
                label: context.t(Key::SearchLabel),
                placeholder: context.t(Key::SearchPlaceholder),
                status: context.t(Key::SearchResults),
                empty_label: context.t(Key::SearchEmpty),
                SearchGroup { label: context.t(Key::SearchGroupComponents),
                    SearchResult { class: "nd-identifier", href: "../button/", label: "Button" }
                    SearchResult { class: "nd-identifier", href: "../select/", label: "Select" }
                }
            }
        }
    ),
    example!(r"Two runs, one list", |context| rsx! {
        Search {
            name: "group-two",
            label: context.t(Key::SearchLabel),
            placeholder: context.t(Key::SearchPlaceholder),
            status: context.t(Key::SearchResults),
            empty_label: context.t(Key::SearchEmpty),
            {pages!(context)}
        }
    }),
    example!(
        r"A run beside ungrouped results",
        Key::ExampleSearchGroupMixed,
        |context| rsx! {
            Search {
                name: "group-mixed",
                label: context.t(Key::SearchLabel),
                placeholder: context.t(Key::SearchPlaceholder),
                status: context.t(Key::SearchResults),
                empty_label: context.t(Key::SearchEmpty),
                SearchResult {
                    href: "../intro/",
                    label: crate::site::INTRO.title(context.locale),
                    detail: context.t(Key::PageIntroSummary),
                }
                SearchGroup { label: context.t(Key::SearchGroupComponents),
                    SearchResult { class: "nd-identifier", href: "../button/", label: "Button" }
                    SearchResult { class: "nd-identifier", href: "../select/", label: "Select" }
                }
            }
        }
    ),
    example!(
        r"An emptied group leaves nothing behind",
        Key::ExampleSearchGroupEmpty,
        |context| rsx! {
            Search {
                name: "group-empty",
                value: "sel",
                label: context.t(Key::SearchLabel),
                placeholder: context.t(Key::SearchPlaceholder),
                status: context.t(Key::SearchResults),
                empty_label: context.t(Key::SearchEmpty),
                clear_label: context.t(Key::SearchClear),
                {pages!(context)}
            }
        }
    ),
];

const SEARCH_RESULT: &[Example] = &[
    example!(
        r#"SearchResult { href: "…", label: "…" }"#,
        Key::ExampleSearchResult,
        |context| rsx! {
            Search {
                name: "result-plain",
                label: context.t(Key::SearchLabel),
                placeholder: context.t(Key::SearchPlaceholder),
                status: context.t(Key::SearchResults),
                empty_label: context.t(Key::SearchEmpty),
                SearchResult { class: "nd-identifier", href: "../button/", label: "Button" }
                SearchResult { class: "nd-identifier", href: "../button-group/", label: "ButtonGroup" }
                SearchResult { class: "nd-identifier", href: "../select/", label: "Select" }
            }
        }
    ),
    example!(
        r#"SearchResult { detail: "…" }"#,
        Key::ExampleSearchResultDetail,
        |context| rsx! {
            Search {
                name: "result-detail",
                label: context.t(Key::SearchLabel),
                placeholder: context.t(Key::SearchPlaceholder),
                status: context.t(Key::SearchResults),
                empty_label: context.t(Key::SearchEmpty),
                SearchResult {
                    href: "../spacing/",
                    label: crate::site::SPACING.title(context.locale),
                    detail: context.t(Key::PageSpacingSummary),
                }
                SearchResult {
                    href: "../icons/",
                    label: crate::site::ICONS.title(context.locale),
                    detail: context.t(Key::PageIconsSummary),
                }
            }
        }
    ),
    example!(
        r#"SearchResult { keywords: "gap inset step scale rhythm" }"#,
        Key::ExampleSearchResultKeywords,
        |context| rsx! {
            Search {
                name: "result-keywords",
                value: "gap",
                label: context.t(Key::SearchLabel),
                placeholder: context.t(Key::SearchPlaceholder),
                status: context.t(Key::SearchResults),
                empty_label: context.t(Key::SearchEmpty),
                clear_label: context.t(Key::SearchClear),
                SearchResult {
                    href: "../spacing/",
                    label: crate::site::SPACING.title(context.locale),
                    detail: context.t(Key::PageSpacingSummary),
                    keywords: "gap inset step scale rhythm",
                }
                SearchResult {
                    href: "../typography/",
                    label: crate::site::TYPOGRAPHY.title(context.locale),
                    detail: context.t(Key::PageTypographySummary),
                }
            }
        }
    ),
    example!(
        r#"SearchResult { badge: "…" } / SearchResult { current: true }"#,
        Key::ExampleSearchCurrent,
        |context| rsx! {
            Search {
                name: "result-marked",
                label: context.t(Key::SearchLabel),
                placeholder: context.t(Key::SearchPlaceholder),
                status: context.t(Key::SearchResults),
                empty_label: context.t(Key::SearchEmpty),
                SearchResult {
                    href: "../icons/",
                    label: crate::site::ICONS.title(context.locale),
                    badge: context.t(Key::ExampleSearchBadge),
                }
                SearchResult {
                    class: "nd-identifier",
                    href: "../search/",
                    label: crate::site::SEARCH.title(context.locale),
                    current: true,
                }
            }
        }
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
        Key::ExampleColorPickerPlainNote,
        |context| rsx! {
            ColorPicker {
                name: "label-colour",
                label: context.t(Key::ExampleColorPickerLabel),
                value: "#7c3aed",
                // NO `size`, NO `shape`, NO `formats`. All three were set to
                // their own declared default here, so the plainest example on
                // the page carried three props a reader could delete without
                // changing a pixel. Each is shown at a value that differs
                // further down.
                trigger_label: context.t(Key::ExampleColorPickerTrigger),
                area_label: context.t(Key::ExampleColorPickerArea),
                hue_label: context.t(Key::ExampleColorPickerHue),
                entry_label: context.t(Key::ExampleColorPickerEntry),
                entry_invalid_label: context.t(Key::ExampleColorPickerEntryInvalid),
                codes_label: context.t(Key::ExampleColorPickerCodes),
                copy_label: context.t(Key::ExampleColorPickerCopy),
                copied_label: context.t(Key::ExampleColorPickerCopied),
                close_label: context.t(Key::ExampleColorPickerClose),
            }
        }
    ),
    example!(
        r"ColorPicker { size: Large, shape: Pill, alpha: true, alpha_label, eyedropper: true, eyedropper_label, recent, recent_label, forget_label }",
        Key::ExampleColorPickerExtrasNote,
        |context| rsx! {
            ColorPicker {
                name: "brand-colour",
                label: context.t(Key::ExampleColorPickerBrandLabel),
                value: "#0ea5e9",
                size: ColorPickerSize::Large,
                shape: ColorPickerShape::Pill,
                alpha: true,
                alpha_label: context.t(Key::ExampleColorPickerAlpha),
                eyedropper: true,
                eyedropper_label: context.t(Key::ExampleColorPickerEyedropper),
                // A key the CALLER names. Nothing is remembered without one,
                // which is what keeps the decision to write to a reader's
                // browser out of this component's hands.
                recent: "noctua.docs.colours",
                recent_label: context.t(Key::ExampleColorPickerRecent),
                forget_label: context.t(Key::ExampleColorPickerForget),
                trigger_label: context.t(Key::ExampleColorPickerTrigger),
                area_label: context.t(Key::ExampleColorPickerArea),
                hue_label: context.t(Key::ExampleColorPickerHue),
                entry_label: context.t(Key::ExampleColorPickerEntry),
                entry_invalid_label: context.t(Key::ExampleColorPickerEntryInvalid),
                codes_label: context.t(Key::ExampleColorPickerCodes),
                copy_label: context.t(Key::ExampleColorPickerCopy),
                copied_label: context.t(Key::ExampleColorPickerCopied),
                close_label: context.t(Key::ExampleColorPickerClose),
            }
        }
    ),
    example!(
        r"ColorPicker { multiple: true, max, max_label, add_label, remove_label, added_label, removed_label, status }",
        Key::ExampleColorPickerMultipleNote,
        |context| rsx! {
            ColorPicker {
                name: "tag-colours",
                label: context.t(Key::ExampleColorPickerTagsLabel),
                value: "#7c3aed #0ea5e9 #22c55e",
                multiple: true,
                max: "4",
                max_label: context.t(Key::ExampleColorPickerMax),
                add_label: context.t(Key::ExampleColorPickerAdd),
                remove_label: context.t(Key::ExampleColorPickerRemove),
                added_label: context.t(Key::ExampleColorPickerAdded),
                removed_label: context.t(Key::ExampleColorPickerRemoved),
                status: context.t(Key::ExampleColorPickerStatus),
                trigger_label: context.t(Key::ExampleColorPickerTrigger),
                area_label: context.t(Key::ExampleColorPickerArea),
                hue_label: context.t(Key::ExampleColorPickerHue),
                entry_label: context.t(Key::ExampleColorPickerEntry),
                entry_invalid_label: context.t(Key::ExampleColorPickerEntryInvalid),
                codes_label: context.t(Key::ExampleColorPickerCodes),
                copy_label: context.t(Key::ExampleColorPickerCopy),
                copied_label: context.t(Key::ExampleColorPickerCopied),
                close_label: context.t(Key::ExampleColorPickerClose),
            }
        }
    ),
    example!(
        r#"ColorPicker { palette: true, palette_label, spectrum_label, formats: "hex oklch token" }"#,
        Key::ExampleColorPickerPaletteNote,
        |context| rsx! {
            ColorPicker {
                name: "noctua-colour",
                label: context.t(Key::ExampleColorPickerLabel),
                value: "#55a45b",
                palette: true,
                palette_label: context.t(Key::ExampleColorPickerPalette),
                spectrum_label: context.t(Key::ExampleColorPickerSpectrum),
                // `token` says nothing about a colour the palette does not
                // contain, so it appears on the Noctua tab and nowhere else
                // however it is listed.
                formats: "hex oklch token",
                trigger_label: context.t(Key::ExampleColorPickerTrigger),
                area_label: context.t(Key::ExampleColorPickerArea),
                hue_label: context.t(Key::ExampleColorPickerHue),
                entry_label: context.t(Key::ExampleColorPickerEntry),
                entry_invalid_label: context.t(Key::ExampleColorPickerEntryInvalid),
                codes_label: context.t(Key::ExampleColorPickerCodes),
                copy_label: context.t(Key::ExampleColorPickerCopy),
                copied_label: context.t(Key::ExampleColorPickerCopied),
                close_label: context.t(Key::ExampleColorPickerClose),
            }
        }
    ),
    example!(
        r"ColorPicker { hint, error, required: true, glass: true, entry: false, size: Small, shape: Square } / ColorPicker { disabled: true }",
        Key::ExampleColorPickerFormNote,
        |context| rsx! {
            form { class: "nd-example-form", method: "get", action: "#", novalidate: true,
                ColorPicker {
                    name: "page-colour",
                    label: context.t(Key::ExampleColorPickerLabel),
                    value: "#f59e0b",
                    hint: context.t(Key::ExampleColorPickerHint),
                    error: context.t(Key::ExampleColorPickerError),
                    required: true,
                    glass: true,
                    // Off, so the panel is the square and the tracks and
                    // nothing else — the shape a product wants when the
                    // colours are a reader's own rather than a brand's.
                    entry: false,
                    size: ColorPickerSize::Small,
                    shape: ColorPickerShape::Square,
                    trigger_label: context.t(Key::ExampleColorPickerTrigger),
                    area_label: context.t(Key::ExampleColorPickerArea),
                    hue_label: context.t(Key::ExampleColorPickerHue),
                    codes_label: context.t(Key::ExampleColorPickerCodes),
                    copy_label: context.t(Key::ExampleColorPickerCopy),
                    copied_label: context.t(Key::ExampleColorPickerCopied),
                    close_label: context.t(Key::ExampleColorPickerClose),
                }
                ColorPicker {
                    name: "refused-colour",
                    label: context.t(Key::ExampleColorPickerBrandLabel),
                    value: "#928f8e",
                    disabled: true,
                    trigger_label: context.t(Key::ExampleColorPickerTrigger),
                    area_label: context.t(Key::ExampleColorPickerArea),
                    hue_label: context.t(Key::ExampleColorPickerHue),
                    entry_label: context.t(Key::ExampleColorPickerEntry),
                    entry_invalid_label: context.t(Key::ExampleColorPickerEntryInvalid),
                    codes_label: context.t(Key::ExampleColorPickerCodes),
                    copy_label: context.t(Key::ExampleColorPickerCopy),
                    copied_label: context.t(Key::ExampleColorPickerCopied),
                    close_label: context.t(Key::ExampleColorPickerClose),
                }
                Button {
                    submit: ButtonSubmit::Submit,
                    label: context.t(Key::ExampleColorPickerSend),
                }
            }
        }
    ),
];

// --- The input controls -----------------------------------------------------

// ONE FORM PER EXAMPLE, AND A REAL ONE. `novalidate` so the sentence a reader
// hears is the component's own rather than the browser bubble the no-script
// path already gives them — the same rig `DateInput`'s examples use, for the
// reason recorded there.

const FIELD: &[Example] = &[
    example!(
        r"Field { control, label, hint }",
        Key::ExampleFieldNote,
        |context| rsx! {
            Input {
                name: "field-plain",
                label: context.t(Key::ExampleInputNameLabel),
                hint: context.t(Key::ExampleInputHint),
                autocomplete: "name",
            }
        }
    ),
    example!(r"Field { required, required_label }", |context| rsx! {
        Input {
            name: "field-required",
            label: context.t(Key::ExampleInputEmailLabel),
            kind: InputKind::Email,
            required: true,
            required_label: context.t(Key::ExampleInputRequired),
            autocomplete: "email",
        }
    }),
    example!(
        r"Field { error, error_label } / Field { error_slot } / Field { disabled }",
        |context| rsx! {
            div { class: "nd-example-stack",
                Input {
                    name: "field-wrong",
                    label: context.t(Key::ExampleInputEmailLabel),
                    kind: InputKind::Email,
                    error: context.t(Key::ExampleInputError),
                }
                // The slot, rendered empty. `Select` and `DateInput` each arrived
                // at needing one, which is why `Field` has it: a control whose
                // real element the script hides cannot be reported by the browser,
                // so this is where its own complaint goes.
                // `error_label` IS THE WORD ANNOUNCED BEFORE AN ERROR, so it
                // is *Error:* and never *required*. Setting it to the word
                // that marks a compulsory question makes every complaint on
                // the field announce as the field being obligatory.
                Select {
                    name: "field-slot",
                    label: context.t(Key::ExampleInputNameLabel),
                    required: true,
                    error_label: context.t(Key::ExampleFormsErrorWord),
                    SelectOption { value: "one", label: "1" }
                }
                Input {
                    name: "field-off",
                    label: context.t(Key::ExampleInputNameLabel),
                    disabled: true,
                }
            }
        }
    ),
    example!(r"Field { size }", |context| rsx! {
        div { class: "nd-example-stack",
            Input {
                name: "field-small",
                label: context.t(Key::ExampleInputNameLabel),
                size: InputSize::Small,
            }
            Input {
                name: "field-large",
                label: context.t(Key::ExampleInputNameLabel),
                size: InputSize::Large,
                hint: context.t(Key::ExampleInputHint),
            }
        }
    }),
    example!(
        r"Field { kind: Group } — a `<legend>`, because a `<label>` names one element",
        |context| rsx! {
            TimeInput {
                name: "field-group",
                legend: context.t(Key::ExampleInputTimeLegend),
                hour_label: context.t(Key::ExampleInputHour),
                minute_label: context.t(Key::ExampleInputMinute),
                hint: context.t(Key::ExampleInputTimeHint),
            }
        }
    ),
];

const INPUT: &[Example] = &[
    // ONE FIELD, IN A FORM THAT SUBMITS. Press it and the answer arrives in the
    // address bar, which is the only way to show that the box is a real
    // `<input>` with a real `name` rather than a picture of one.
    example!(
        r"Input { name, label, value, hint, required, required_label, autocomplete }",
        Key::ExampleInputNoteAutocomplete,
        |context| rsx! {
            form { class: "nd-example-form", method: "get", action: "#", novalidate: true,
                Input {
                    name: "who",
                    label: context.t(Key::ExampleInputNameLabel),
                    value: context.t(Key::ExampleInputNameValue),
                    hint: context.t(Key::ExampleInputHint),
                    required: true,
                    required_label: context.t(Key::ExampleInputRequired),
                    autocomplete: "name",
                }
                Button { submit: ButtonSubmit::Submit, label: context.t(Key::ExampleInputSend) }
            }
        }
    ),
    // THE KEYBOARD A PHONE OPENS. Three kinds, three different questions, and
    // three different labels — a row of boxes all called *Website* is the
    // defect this example replaced.
    example!(
        r"Input { kind, placeholder, pattern, spellcheck, prefix }",
        Key::ExampleInputNoteKind,
        |context| rsx! {
            div { class: "nd-example-stack",
                Input {
                    name: "where",
                    label: context.t(Key::ExampleInputEmailLabel),
                    kind: InputKind::Email,
                    placeholder: "ada@example.org",
                    autocomplete: "email",
                    pattern: ".+@.+\\..+",
                }
                // THE ONE BOX HERE THAT HOLDS WORDS, and the only one where
                // `spellcheck` has anything to say. It is **off** by default
                // on an `Input`, which is the opposite of the platform, so
                // `false` on the address above showed the prop doing nothing.
                Input {
                    name: "title",
                    label: context.t(Key::ExampleInputTitleLabel),
                    spellcheck: true,
                }
                Input {
                    name: "phone",
                    label: context.t(Key::ExampleInputPhoneLabel),
                    hint: context.t(Key::ExampleInputPhoneHint),
                    kind: InputKind::Tel,
                    prefix: "+55",
                    autocomplete: "tel",
                }
                Input {
                    name: "site",
                    label: context.t(Key::ExampleInputSiteLabel),
                    kind: InputKind::Url,
                    prefix: "https://",
                    autocomplete: "url",
                }
            }
        }
    ),
    // A PASSWORD A READER CAN CHECK BEFORE SENDING IT. Both words ship, because
    // the button says two different things depending on which state it is in.
    example!(
        r"Input { kind: Password, reveal_label, conceal_label }",
        Key::ExampleInputNotePassword,
        |context| rsx! {
            Input {
                name: "secret",
                label: context.t(Key::ExampleInputPasswordLabel),
                kind: InputKind::Password,
                hint: context.t(Key::ExampleInputPasswordHint),
                autocomplete: "new-password",
                reveal_label: context.t(Key::ExampleInputReveal),
                conceal_label: context.t(Key::ExampleInputConceal),
            }
        }
    ),
    // WHAT THE BOX CARRIES BESIDE THE TEXT. A handle, which is what an icon, a
    // prefix, a suffix and a character count are all actually for.
    example!(
        r"Input { icon, prefix, suffix, clearable, clear_label, maxlength, counter, counter_label }",
        Key::ExampleInputNoteExtras,
        |context| rsx! {
            Input {
                name: "handle",
                label: context.t(Key::ExampleInputHandleLabel),
                hint: context.t(Key::ExampleInputHandleHint),
                icon: IconName::User,
                prefix: "@",
                suffix: ".noctua",
                clearable: true,
                clear_label: context.t(Key::ExampleInputClear),
                maxlength: "24",
                counter: true,
                counter_label: context.t(Key::ExampleInputCounter),
            }
        }
    ),
    // THE THREE SIZES, ITERATED RATHER THAN LISTED, so this cannot fall out of
    // step with the enum the day a fourth is added.
    example!(
        r"Input { size } for size in InputSize::ALL",
        Key::ExampleInputNoteSize,
        |context| rsx! {
            div { class: "nd-example-stack",
                for size in InputSize::ALL {
                    Input {
                        name: "size-{size.as_str()}",
                        label: context.t(Key::ExampleInputNameLabel),
                        size,
                        autocomplete: "name",
                    }
                }
            }
        }
    ),
    // REFUSED, LOCKED AND UNAVAILABLE — three different states, each on a box
    // the state makes sense on. The error sits on the address that was refused,
    // not on a reference number nobody can edit.
    example!(
        r"Input { name, label, kind: Email, value, error, autocomplete }
Input { name, label, value, readonly }
Input { name, label, kind: Password, reveal_label, conceal_label, disabled, glass }",
        Key::ExampleInputNoteRefused,
        |context| rsx! {
            div { class: "nd-example-stack",
                Input {
                    name: "refused",
                    label: context.t(Key::ExampleInputEmailLabel),
                    kind: InputKind::Email,
                    value: "ada@examp",
                    error: context.t(Key::ExampleInputError),
                    autocomplete: "email",
                }
                Input {
                    name: "reference",
                    label: context.t(Key::ExampleInputReferenceLabel),
                    value: context.t(Key::ExampleInputReference),
                    readonly: true,
                }
                Input {
                    name: "locked",
                    label: context.t(Key::ExampleInputPasswordLabel),
                    kind: InputKind::Password,
                    reveal_label: context.t(Key::ExampleInputReveal),
                    conceal_label: context.t(Key::ExampleInputConceal),
                    disabled: true,
                    glass: true,
                }
            }
        }
    ),
];

const TEXTAREA: &[Example] = &[
    example!(
        r"Textarea { name, label, rows, autosize, max_rows, placeholder, hint, required, required_label }",
        Key::ExampleTextareaNote,
        |context| rsx! {
            form { class: "nd-example-form", method: "get", action: "#", novalidate: true,
                Textarea {
                    name: "note",
                    label: context.t(Key::ExampleInputNoteLabel),
                    hint: context.t(Key::ExampleInputNoteHint),
                    rows: "3",
                    autosize: true,
                    max_rows: "8",
                    required: true,
                    required_label: context.t(Key::ExampleInputRequired),
                    placeholder: "…",
                }
                Button { submit: ButtonSubmit::Submit, label: context.t(Key::ExampleInputSend) }
            }
        }
    ),
    example!(
        r"Textarea { name, label, rows, maxlength, counter, counter_label, spellcheck: false }",
        |context| rsx! {
            // A LIST OF ADDRESSES, NOT PROSE, which is the one case where
            // turning the underlining off is right. `spellcheck` is **on** by
            // default on a `Textarea`, so `true` here showed nothing.
            Textarea {
                name: "recipients",
                label: context.t(Key::ExampleTextareaRecipients),
                rows: "3",
                maxlength: "400",
                counter: true,
                counter_label: context.t(Key::ExampleInputCounter),
                spellcheck: false,
            }
        }
    ),
    example!(
        r"Textarea { name, label, value, rows, resize: None, autosize, size, glass, error }",
        |context| rsx! {
            // `resize: None` WANTS `autosize` ON, and the contract says so in
            // writing: a box a reader can neither drag nor have grown for them
            // is a box whose text runs out of sight.
            Textarea {
                name: "fixed",
                label: context.t(Key::ExampleInputNoteLabel),
                value: context.t(Key::ExampleInputFixedNote),
                rows: "2",
                resize: TextareaResize::None,
                autosize: true,
                size: TextareaSize::Small,
                glass: true,
                error: context.t(Key::ExampleInputError),
            }
        }
    ),
    example!(
        r"Textarea { readonly } / Textarea { disabled }",
        |context| rsx! {
            div { class: "nd-example-stack",
                Textarea {
                    name: "locked",
                    label: context.t(Key::ExampleInputNoteLabel),
                    value: context.t(Key::ExampleInputLockedNote),
                    readonly: true,
                    rows: "2",
                    size: TextareaSize::Large,
                }
                Textarea {
                    name: "note-off",
                    label: context.t(Key::ExampleInputNoteLabel),
                    rows: "2",
                    disabled: true,
                }
            }
        }
    ),
];

const NUMBER_INPUT: &[Example] = &[
    // ONE QUESTION, IN A FORM THAT SUBMITS. Press Save and the quantity
    // arrives in the address bar, which is the only way to show that this is a
    // real `<input>` with a real `name` rather than a picture of one.
    example!(
        r"NumberInput { name, label, value, min, max, step, big_step, increment_label, decrement_label, hint, required, required_label }",
        Key::ExampleNumberInputNoteForm,
        |context| rsx! {
            form { class: "nd-example-form", method: "get", action: "#", novalidate: true,
                NumberInput {
                    name: "tickets",
                    label: context.t(Key::ExampleNumberInputTicketsLabel),
                    value: "2",
                    min: "1",
                    max: "20",
                    step: "1",
                    big_step: "5",
                    increment_label: context.t(Key::ExampleInputIncrement),
                    decrement_label: context.t(Key::ExampleInputDecrement),
                    hint: context.t(Key::ExampleNumberInputTicketsHint),
                    required: true,
                    required_label: context.t(Key::ExampleInputRequired),
                }
                Button { submit: ButtonSubmit::Submit, label: context.t(Key::ExampleInputSend) }
            }
        }
    ),
    // REFUSED AND EXPLAINED, NEVER REWRITTEN. Type 40, leave the box, and the
    // sentence under the question is the caller's own. The number stays as it
    // was typed, so a reader can see what was wrong with it.
    example!(
        r"NumberInput { name, label, value, min, max, min_label, max_label, invalid_label, increment_label, decrement_label }",
        Key::ExampleNumberInputNoteBounds,
        |context| rsx! {
            NumberInput {
                name: "boxes",
                label: context.t(Key::ExampleNumberInputBoxesLabel),
                value: "5",
                min: "1",
                max: "10",
                min_label: context.t(Key::ExampleInputTooSmall),
                max_label: context.t(Key::ExampleInputTooBig),
                invalid_label: context.t(Key::ExampleInputNotANumber),
                increment_label: context.t(Key::ExampleInputIncrement),
                decrement_label: context.t(Key::ExampleInputDecrement),
            }
        }
    ),
    // MONEY, AND WHAT `group` PROMISES. The separators are the page's own and
    // they are drawn while nobody is typing. This one submits as well, because
    // "never submitted" is a claim only a form can settle.
    example!(
        r"NumberInput { name, label, value, min, max, step, prefix, group, placeholder, size, increment_label, decrement_label }",
        Key::ExampleNumberInputNoteMoney,
        |context| rsx! {
            form { class: "nd-example-form", method: "get", action: "#", novalidate: true,
                NumberInput {
                    name: "price",
                    label: context.t(Key::ExampleNumberInputPriceLabel),
                    value: "1499.9",
                    min: "0",
                    max: "9999.99",
                    step: "0.01",
                    prefix: "R$",
                    group: true,
                    placeholder: "0,00",
                    size: NumberInputSize::Large,
                    increment_label: context.t(Key::ExampleNumberInputMore),
                    decrement_label: context.t(Key::ExampleNumberInputLess),
                }
                Button { submit: ButtonSubmit::Submit, label: context.t(Key::ExampleInputSend) }
            }
        }
    ),
    // A MEASUREMENT, WITH ITS UNIT INSIDE THE BOX. `step` carries the decimal
    // here: it sets what one press moves, how many decimals are drawn, and
    // which keyboard a phone opens.
    example!(
        r"NumberInput { name, label, value, min, max, step, suffix, increment_label, decrement_label }",
        Key::ExampleNumberInputNoteUnit,
        |context| rsx! {
            NumberInput {
                name: "weight",
                label: context.t(Key::ExampleNumberInputWeightLabel),
                value: "4.5",
                min: "0",
                max: "30",
                step: "0.1",
                suffix: "kg",
                increment_label: context.t(Key::ExampleNumberInputMore),
                decrement_label: context.t(Key::ExampleNumberInputLess),
            }
        }
    ),
    // TOO LARGE TO STEP TO, which is the one reason to turn the two controls
    // off. The arrow keys and Page Up still move it, and `big_step` is what
    // makes crossing this range from the keyboard possible at all.
    example!(
        r"NumberInput { name, label, value, step, big_step, steppers, group }",
        Key::ExampleNumberInputNoteLarge,
        |context| rsx! {
            NumberInput {
                name: "words",
                label: context.t(Key::ExampleNumberInputWordsLabel),
                value: "50000",
                step: "500",
                big_step: "5000",
                steppers: false,
                group: true,
            }
        }
    ),
    // REFUSED, LOCKED AND UNAVAILABLE — three states, each on a question the
    // state makes sense on. The error sits on the number that was refused, and
    // the locked value carries none, because there is nothing to correct.
    example!(
        r"NumberInput { name, label, value, max, error, increment_label, decrement_label }
NumberInput { name, label, value, readonly, size, increment_label, decrement_label }
NumberInput { name, label, disabled, glass, increment_label, decrement_label }",
        Key::ExampleNumberInputNoteStates,
        |context| rsx! {
            div { class: "nd-example-stack",
                NumberInput {
                    name: "seats",
                    label: context.t(Key::ExampleNumberInputSeatsLabel),
                    value: "6",
                    max: "4",
                    error: context.t(Key::ExampleNumberInputError),
                    increment_label: context.t(Key::ExampleInputIncrement),
                    decrement_label: context.t(Key::ExampleInputDecrement),
                }
                NumberInput {
                    name: "packed",
                    label: context.t(Key::ExampleNumberInputPackedLabel),
                    value: "12",
                    readonly: true,
                    size: NumberInputSize::Small,
                    increment_label: context.t(Key::ExampleInputIncrement),
                    decrement_label: context.t(Key::ExampleInputDecrement),
                }
                NumberInput {
                    name: "bags",
                    label: context.t(Key::ExampleNumberInputBagsLabel),
                    disabled: true,
                    glass: true,
                    increment_label: context.t(Key::ExampleInputIncrement),
                    decrement_label: context.t(Key::ExampleInputDecrement),
                }
            }
        }
    ),
];

const SLIDER: &[Example] = &[
    example!(
        r"Slider { name, label, value, min, max, step, output_label, hint, required, required_label }",
        Key::ExampleSliderNote,
        |context| rsx! {
            form { class: "nd-example-form", method: "get", action: "#", novalidate: true,
                Slider {
                    name: "volume",
                    label: context.t(Key::ExampleInputVolumeLabel),
                    value: "35",
                    min: "0",
                    max: "100",
                    step: "5",
                    output_label: context.t(Key::ExampleInputPercentValue),
                    hint: context.t(Key::ExampleInputHint),
                    required: true,
                    required_label: context.t(Key::ExampleInputRequired),
                }
                Button { submit: ButtonSubmit::Submit, label: context.t(Key::ExampleInputSend) }
            }
        }
    ),
    example!(r"Slider { marks, mark_labels }", |context| rsx! {
        Slider {
            name: "marked",
            label: context.t(Key::ExampleInputVolumeLabel),
            value: "50",
            marks: "0 50 100",
            mark_labels: context.t(Key::ExampleInputVolumeMarks),
            output_label: context.t(Key::ExampleInputPercentValue),
        }
    }),
    example!(
        r"Slider { name, label, range, value, end, min, max, step, output_label, tone, size }",
        |context| rsx! {
            Slider {
                name: "budget",
                label: context.t(Key::ExampleInputBudgetLabel),
                range: true,
                value: "200",
                end: "800",
                min: "0",
                max: "1000",
                step: "50",
                output_label: context.t(Key::ExampleInputCurrencyValue),
                tone: SliderTone::Success,
                size: SliderSize::Large,
            }
        }
    ),
    example!(
        r"Slider { name, label, value, output: false, tone, size, glass, error }
Slider { name, label, value, tone, disabled }",
        |context| rsx! {
            div { class: "nd-example-stack",
                Slider {
                    name: "quiet",
                    label: context.t(Key::ExampleInputVolumeLabel),
                    value: "60",
                    output: false,
                    tone: SliderTone::Neutral,
                    size: SliderSize::Small,
                    glass: true,
                    error: context.t(Key::ExampleInputError),
                }
                Slider {
                    name: "refused-slider",
                    label: context.t(Key::ExampleInputVolumeLabel),
                    value: "20",
                    tone: SliderTone::Danger,
                    disabled: true,
                }
            }
        }
    ),
];

const FILE_INPUT: &[Example] = &[
    example!(
        r"FileInput { name, label, browse_label, drop_label, empty_label, status, hint, required, required_label }",
        Key::ExampleFileInputNote,
        |context| rsx! {
            // NO `drop: true` HERE. It is the default, so it showed nothing.
            // The example that turns it off is the last one on this page.
            form { class: "nd-example-form", method: "get", action: "#", novalidate: true,
                FileInput {
                    name: "one-attachment",
                    label: context.t(Key::ExampleInputFilesLabel),
                    browse_label: context.t(Key::ExampleInputBrowse),
                    drop_label: context.t(Key::ExampleInputDrop),
                    empty_label: context.t(Key::ExampleInputFileEmpty),
                    status: context.t(Key::ExampleInputFileStatus),
                    hint: context.t(Key::ExampleInputFilesHint),
                    required: true,
                    required_label: context.t(Key::ExampleInputRequired),
                }
                Button { submit: ButtonSubmit::Submit, label: context.t(Key::ExampleInputSend) }
            }
        }
    ),
    example!(
        r"FileInput { multiple, max, max_label, remove_label, added_label, removed_label }",
        |context| rsx! {
            FileInput {
                name: "attachments",
                label: context.t(Key::ExampleInputFilesLabel),
                browse_label: context.t(Key::ExampleInputBrowse),
                drop_label: context.t(Key::ExampleInputDrop),
                multiple: true,
                max: "3",
                max_label: context.t(Key::ExampleInputFileTooMany),
                remove_label: context.t(Key::ExampleInputFileRemove),
                added_label: context.t(Key::ExampleInputFileAdded),
                removed_label: context.t(Key::ExampleInputFileRemoved),
                empty_label: context.t(Key::ExampleInputFileEmpty),
                status: context.t(Key::ExampleInputFileStatus),
            }
        }
    ),
    example!(
        r"FileInput { accept, accept_label, max_size, max_size_label } — help, never security",
        |context| rsx! {
            FileInput {
                name: "pictures",
                label: context.t(Key::ExampleInputFilesLabel),
                browse_label: context.t(Key::ExampleInputBrowse),
                accept: ".pdf,image/*",
                accept_label: context.t(Key::ExampleInputFileType),
                max_size: "2097152",
                max_size_label: context.t(Key::ExampleInputFileTooBig),
                multiple: true,
                remove_label: context.t(Key::ExampleInputFileRemove),
                empty_label: context.t(Key::ExampleInputFileEmpty),
                hint: context.t(Key::ExampleInputFilesHint),
            }
        }
    ),
    example!(
        r"FileInput { name, label, browse_label, drop: false, size, glass, error }
FileInput { name, label, browse_label, disabled, size }",
        |context| rsx! {
            div { class: "nd-example-stack",
                FileInput {
                    name: "one-file",
                    label: context.t(Key::ExampleInputFilesLabel),
                    browse_label: context.t(Key::ExampleInputBrowse),
                    drop: false,
                    size: FileInputSize::Small,
                    glass: true,
                    error: context.t(Key::ExampleInputError),
                }
                FileInput {
                    name: "refused-files",
                    label: context.t(Key::ExampleInputFilesLabel),
                    browse_label: context.t(Key::ExampleInputBrowse),
                    disabled: true,
                    size: FileInputSize::Large,
                }
            }
        }
    ),
];

const TIME_INPUT: &[Example] = &[
    example!(
        r"TimeInput { name, legend, hour_label, minute_label, value, hint, size }",
        Key::ExampleTimeInputNote,
        |context| rsx! {
            form { class: "nd-example-form", method: "get", action: "#", novalidate: true,
                TimeInput {
                    name: "starts",
                    legend: context.t(Key::ExampleInputTimeLegend),
                    hour_label: context.t(Key::ExampleInputHour),
                    minute_label: context.t(Key::ExampleInputMinute),
                    value: "09:30",
                    hint: context.t(Key::ExampleInputTimeHint),
                    size: TimeInputSize::Medium,
                }
                Button { submit: ButtonSubmit::Submit, label: context.t(Key::ExampleInputSend) }
            }
        }
    ),
    example!(
        r"TimeInput { name, legend, hour_label, minute_label, clock: _12, meridiem_label, am_label, pm_label, now_label, value }",
        |context| rsx! {
            TimeInput {
                name: "meeting",
                legend: context.t(Key::ExampleInputTimeLegend),
                hour_label: context.t(Key::ExampleInputHour),
                minute_label: context.t(Key::ExampleInputMinute),
                // `_12`, NOT `Auto`. `auto` is the default, so the example
                // asked for what it would have got anyway, and the four
                // meridiem strings under it had no clock to appear on.
                clock: TimeInputClock::_12,
                meridiem_label: context.t(Key::ExampleInputMeridiem),
                am_label: context.t(Key::ExampleInputAm),
                pm_label: context.t(Key::ExampleInputPm),
                now_label: context.t(Key::ExampleInputNow),
                value: "14:05",
            }
        }
    ),
    example!(
        r"TimeInput { min, max, step, pad: false, required, required_label, missing_label, invalid_label, min_label, max_label }",
        |context| rsx! {
            TimeInput {
                name: "booking",
                legend: context.t(Key::ExampleInputTimeLegend),
                hour_label: context.t(Key::ExampleInputHour),
                minute_label: context.t(Key::ExampleInputMinute),
                min: "08:00",
                max: "18:00",
                step: "15",
                // `pad` is ON by default, so `false` is the only value that
                // shows it doing anything: leave a box holding `9` and it
                // stays `9` rather than becoming `09`.
                pad: false,
                required: true,
                required_label: context.t(Key::ExampleInputTimeNeeded),
                missing_label: context.t(Key::ExampleInputTimeMissing),
                invalid_label: context.t(Key::ExampleInputTimeInvalid),
                min_label: context.t(Key::ExampleInputTimeEarly),
                max_label: context.t(Key::ExampleInputTimeLate),
            }
        }
    ),
    example!(
        r"TimeInput { seconds, second_label, glass, error } / TimeInput { disabled }",
        |context| rsx! {
            div { class: "nd-example-stack",
                TimeInput {
                    name: "logged",
                    legend: context.t(Key::ExampleInputTimeLegend),
                    hour_label: context.t(Key::ExampleInputHour),
                    minute_label: context.t(Key::ExampleInputMinute),
                    second_label: context.t(Key::ExampleInputSecond),
                    seconds: true,
                    clock: TimeInputClock::_24,
                    value: "23:59:30",
                    size: TimeInputSize::Small,
                    glass: true,
                    error: context.t(Key::ExampleInputError),
                }
                TimeInput {
                    name: "refused-time",
                    legend: context.t(Key::ExampleInputTimeLegend),
                    hour_label: context.t(Key::ExampleInputHour),
                    minute_label: context.t(Key::ExampleInputMinute),
                    clock: TimeInputClock::_12,
                    am_label: context.t(Key::ExampleInputAm),
                    pm_label: context.t(Key::ExampleInputPm),
                    disabled: true,
                    size: TimeInputSize::Large,
                }
            }
        }
    ),
];

// --- Carousel ---------------------------------------------------------------

// FIVE SLIDES OF WORDS RATHER THAN PICTURES, on purpose: a carousel of
// photographs demonstrates the photographs, and what these pages are about is
// the control. The one that rotates is the only thing on this site that moves
// on its own, and its own contract is what stops it.

macro_rules! slides {
    ($context:expr) => {
        rsx! {
            CarouselSlide { label: $context.t(Key::ExampleCarouselSlideOne),
                Paragraph { {$context.t(Key::ExampleCarouselSlideOne)} }
            }
            CarouselSlide { label: $context.t(Key::ExampleCarouselSlideTwo),
                Paragraph { {$context.t(Key::ExampleCarouselSlideTwo)} }
            }
            CarouselSlide { label: $context.t(Key::ExampleCarouselSlideThree),
                Paragraph { {$context.t(Key::ExampleCarouselSlideThree)} }
            }
            CarouselSlide { label: $context.t(Key::ExampleCarouselSlideFour),
                Paragraph { {$context.t(Key::ExampleCarouselSlideFour)} }
            }
            CarouselSlide { label: $context.t(Key::ExampleCarouselSlideFive),
                Paragraph { {$context.t(Key::ExampleCarouselSlideFive)} }
            }
        }
    };
}

const CAROUSEL: &[Example] = &[
    example!(
        r"Carousel { name, label, snap: Center, previous_label, next_label, announce_label }",
        Key::ExampleCarouselGalleryNote,
        |context| rsx! {
            Carousel {
                name: "gallery",
                label: context.t(Key::ExampleCarouselLabel),
                // NO `per_view: "1"` AND NO `controls: true`. Both are the
                // declared default, so the first example on the page opened
                // on two props a reader could delete without changing
                // anything. Each is shown at a value that differs below.
                snap: CarouselSnap::Center,
                previous_label: context.t(Key::ExampleCarouselPrevious),
                next_label: context.t(Key::ExampleCarouselNext),
                announce_label: context.t(Key::ExampleCarouselAnnounce),
                {slides!(context)}
            }
        }
    ),
    example!(
        r#"Carousel { per_view: "auto" } — the shelf, which is the shape that survives a telephone"#,
        |context| rsx! {
            Carousel {
                name: "shelf",
                label: context.t(Key::ExampleCarouselShelfLabel),
                per_view: "auto",
                previous_label: context.t(Key::ExampleCarouselPrevious),
                next_label: context.t(Key::ExampleCarouselNext),
                announce_label: context.t(Key::ExampleCarouselAnnounce),
                {slides!(context)}
            }
        }
    ),
    example!(
        r"Carousel { dots, dot_label, counter, counter_label, glass }",
        Key::ExampleCarouselDotsNote,
        |context| rsx! {
            Carousel {
                name: "counted",
                label: context.t(Key::ExampleCarouselLabel),
                per_view: "2",
                dots: true,
                dot_label: context.t(Key::ExampleCarouselDot),
                counter: true,
                counter_label: context.t(Key::ExampleCarouselCounter),
                announce_label: context.t(Key::ExampleCarouselAnnounce),
                previous_label: context.t(Key::ExampleCarouselPrevious),
                next_label: context.t(Key::ExampleCarouselNext),
                glass: true,
                {slides!(context)}
            }
        }
    ),
    example!(
        r"Carousel { autoplay, pause_label, play_label } — and the four rules it carries",
        Key::ExampleCarouselRotatingNote,
        |context| rsx! {
            Carousel {
                name: "rotating",
                label: context.t(Key::ExampleCarouselLabel),
                autoplay: "4000",
                pause_label: context.t(Key::ExampleCarouselPause),
                play_label: context.t(Key::ExampleCarouselPlay),
                dots: true,
                dot_label: context.t(Key::ExampleCarouselDot),
                announce_label: context.t(Key::ExampleCarouselAnnounce),
                previous_label: context.t(Key::ExampleCarouselPrevious),
                next_label: context.t(Key::ExampleCarouselNext),
                {slides!(context)}
            }
        }
    ),
    example!(
        r"Carousel { per_view, wrap, snap: None, counter, counter_label }
Carousel { orientation: Vertical, controls: false }",
        Key::ExampleCarouselWrapNote,
        |context| rsx! {
            div { class: "nd-example-stack",
                Carousel {
                    name: "looping",
                    label: context.t(Key::ExampleCarouselShelfLabel),
                    per_view: "3",
                    wrap: true,
                    snap: CarouselSnap::None,
                    counter: true,
                    counter_label: context.t(Key::ExampleCarouselCounter),
                    previous_label: context.t(Key::ExampleCarouselPrevious),
                    next_label: context.t(Key::ExampleCarouselNext),
                    announce_label: context.t(Key::ExampleCarouselAnnounce),
                    {slides!(context)}
                }
                // THE ONE CAROUSEL THAT CAN DO WITHOUT THE ARROWS. The
                // contract draws them because a mouse with no horizontal
                // wheel cannot move a sideways track. This track runs
                // downwards, where every wheel already works.
                Carousel {
                    name: "downwards",
                    label: context.t(Key::ExampleCarouselLabel),
                    orientation: CarouselOrientation::Vertical,
                    controls: false,
                    announce_label: context.t(Key::ExampleCarouselAnnounce),
                    {slides!(context)}
                }
            }
        }
    ),
];

const CAROUSEL_SLIDE: &[Example] = &[
    example!(
        r"CarouselSlide { label } — added to the position, never instead of it",
        |context| rsx! {
            Carousel {
                name: "named-slides",
                label: context.t(Key::ExampleCarouselLabel),
                per_view: "2",
                announce_label: context.t(Key::ExampleCarouselAnnounce),
                previous_label: context.t(Key::ExampleCarouselPrevious),
                next_label: context.t(Key::ExampleCarouselNext),
                {slides!(context)}
            }
        }
    ),
    example!(
        r"CarouselSlide { children } — anything focusable in one stays in the tab order",
        |context| rsx! {
            Carousel {
                name: "with-links",
                label: context.t(Key::ExampleCarouselShelfLabel),
                per_view: "auto",
                announce_label: context.t(Key::ExampleCarouselAnnounce),
                previous_label: context.t(Key::ExampleCarouselPrevious),
                next_label: context.t(Key::ExampleCarouselNext),
                CarouselSlide {
                    Link { href: "../button/", {context.t(Key::ExampleCarouselSlideOne)} }
                }
                CarouselSlide {
                    Link { href: "../link/", {context.t(Key::ExampleCarouselSlideTwo)} }
                }
                CarouselSlide {
                    Link { href: "../list/", {context.t(Key::ExampleCarouselSlideThree)} }
                }
            }
        }
    ),
    example!(
        r"CarouselSlide — with no label, the position alone is the name",
        |context| rsx! {
            Carousel {
                name: "unnamed-slides",
                label: context.t(Key::ExampleCarouselLabel),
                per_view: "3",
                counter: true,
                counter_label: context.t(Key::ExampleCarouselCounter),
                announce_label: context.t(Key::ExampleCarouselAnnounce),
                previous_label: context.t(Key::ExampleCarouselPrevious),
                next_label: context.t(Key::ExampleCarouselNext),
                CarouselSlide { Paragraph { {context.t(Key::ExampleCarouselSlideOne)} } }
                CarouselSlide { Paragraph { {context.t(Key::ExampleCarouselSlideTwo)} } }
                CarouselSlide { Paragraph { {context.t(Key::ExampleCarouselSlideThree)} } }
                CarouselSlide { Paragraph { {context.t(Key::ExampleCarouselSlideFour)} } }
            }
        }
    ),
    example!(
        r"CarouselSlide — in a vertical carousel, the same markup",
        |context| rsx! {
            Carousel {
                name: "slides-down",
                label: context.t(Key::ExampleCarouselLabel),
                orientation: CarouselOrientation::Vertical,
                announce_label: context.t(Key::ExampleCarouselAnnounce),
                previous_label: context.t(Key::ExampleCarouselPrevious),
                next_label: context.t(Key::ExampleCarouselNext),
                {slides!(context)}
            }
        }
    ),
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
        Key::ExampleSummaryRenderedNote,
        |context| rsx! {
            form { class: "nd-example-form", method: "get", action: "#", novalidate: true,
                ErrorSummary {
                    name: "rendered",
                    title: context.t(Key::ExampleSummaryTitle),
                    description: context.t(Key::ExampleSummaryDescription),
                    // Off here and nowhere else: four summaries on one page
                    // all reaching for the focus is a page a reader cannot
                    // read. The next example is the one that takes it.
                    focus: false,
                    ErrorSummaryItem {
                        control: "nd-input-summary-name",
                        {context.t(Key::ExampleSummaryNameError)}
                    }
                    ErrorSummaryItem {
                        control: "nd-input-summary-email",
                        {context.t(Key::ExampleSummaryEmailError)}
                    }
                }
                Input {
                    name: "summary-name",
                    label: context.t(Key::ExampleSummaryNameLabel),
                    error: context.t(Key::ExampleSummaryNameError),
                    autocomplete: "name",
                }
                Input {
                    name: "summary-email",
                    label: context.t(Key::ExampleSummaryEmailLabel),
                    kind: InputKind::Email,
                    error: context.t(Key::ExampleSummaryEmailError),
                    autocomplete: "email",
                }
                Button { submit: ButtonSubmit::Submit, label: context.t(Key::ExampleSummarySend) }
            }
        }
    ),
    example!(
        r"ErrorSummary { collect: true, form, title_prefix } — the browser's own refusal",
        Key::ExampleSummaryCollectNote,
        |context| rsx! {
            form { id: "nd-example-collect", class: "nd-example-form", method: "get", action: "#",
                ErrorSummary {
                    name: "collected",
                    title: context.t(Key::ExampleSummaryTitle),
                    collect: true,
                    form: "nd-example-collect",
                    title_prefix: context.t(Key::ExampleSummaryPrefix),
                }
                Input {
                    name: "collect-name",
                    label: context.t(Key::ExampleSummaryNameLabel),
                    required: true,
                    autocomplete: "name",
                }
                Input {
                    name: "collect-email",
                    label: context.t(Key::ExampleSummaryEmailLabel),
                    kind: InputKind::Email,
                    required: true,
                    autocomplete: "email",
                }
                Button { submit: ButtonSubmit::Submit, label: context.t(Key::ExampleSummarySend) }
            }
        }
    ),
    example!(
        r"An entry reaching a control the script has HIDDEN — focus lands on the visible one",
        Key::ExampleSummaryLinkNote,
        |context| rsx! {
            form { class: "nd-example-form", method: "get", action: "#", novalidate: true,
                ErrorSummary {
                    name: "grouped",
                    title: context.t(Key::ExampleSummaryTitle),
                    focus: false,
                    ErrorSummaryItem {
                        control: "nd-select-summary-colour",
                        {context.t(Key::ExampleSummaryChoiceError)}
                    }
                    ErrorSummaryItem {
                        control: "nd-time-input-summary-time-hour",
                        {context.t(Key::ExampleSummaryDateError)}
                    }
                }
                Select {
                    name: "summary-colour",
                    label: context.t(Key::ExampleSummaryChoiceLabel),
                    placeholder: context.t(Key::ExampleSelectPalettePlaceholder),
                    error: context.t(Key::ExampleSummaryChoiceError),
                    { palette!("ochre-balanced", "Ochre") }
                    { palette!("teal-balanced", "Teal") }
                }
                TimeInput {
                    name: "summary-time",
                    legend: context.t(Key::ExampleInputTimeLegend),
                    hour_label: context.t(Key::ExampleInputHour),
                    minute_label: context.t(Key::ExampleInputMinute),
                    error: context.t(Key::ExampleSummaryDateError),
                }
                Button { submit: ButtonSubmit::Submit, label: context.t(Key::ExampleSummarySend) }
            }
        }
    ),
    example!(
        r"ErrorSummary { focus: false } and no description",
        Key::ExampleSummaryPlainNote,
        |context| rsx! {
            ErrorSummary {
                name: "plain",
                title: context.t(Key::ExampleSummaryTitle),
                focus: false,
                ErrorSummaryItem {
                    control: "nd-input-summary-name",
                    {context.t(Key::ExampleSummaryNameError)}
                }
            }
        }
    ),
];

const ERROR_SUMMARY_ITEM: &[Example] = &[
    example!(
        r"ErrorSummaryItem { control } — the id of the control, and it must exist",
        |context| rsx! {
            ErrorSummary {
                name: "item-one",
                title: context.t(Key::ExampleSummaryTitle),
                focus: false,
                ErrorSummaryItem {
                    control: "nd-input-summary-name",
                    {context.t(Key::ExampleSummaryNameError)}
                }
            }
        }
    ),
    example!(
        r"Several, in the order the form asks the questions",
        |context| rsx! {
            ErrorSummary {
                name: "item-order",
                title: context.t(Key::ExampleSummaryTitle),
                focus: false,
                ErrorSummaryItem {
                    control: "nd-input-summary-name",
                    {context.t(Key::ExampleSummaryNameError)}
                }
                ErrorSummaryItem {
                    control: "nd-input-summary-email",
                    {context.t(Key::ExampleSummaryEmailError)}
                }
                ErrorSummaryItem {
                    control: "nd-time-input-summary-time-hour",
                    {context.t(Key::ExampleSummaryDateError)}
                }
            }
        }
    ),
    example!(
        r"ErrorSummaryItem { control } for a question asked as several boxes",
        Key::ExampleSummaryItemGroupNote,
        |context| rsx! {
            ErrorSummary {
                name: "item-group",
                title: context.t(Key::ExampleSummaryTitle),
                focus: false,
                ErrorSummaryItem {
                    control: "nd-time-input-summary-time-hour",
                    {context.t(Key::ExampleSummaryDateError)}
                }
            }
        }
    ),
    example!(
        r"ErrorSummaryItem { children } — the sentence, never the name of the field",
        Key::ExampleSummaryItemWordsNote,
        |context| rsx! {
            ErrorSummary {
                name: "item-words",
                title: context.t(Key::ExampleSummaryTitle),
                focus: false,
                ErrorSummaryItem {
                    control: "nd-input-summary-email",
                    {context.t(Key::ExampleSummaryEmailError)}
                }
            }
        }
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
        Key::ExampleTagTones,
        |context| rsx! {
            div { class: "nd-example-row",
                // ONE OF THESE IS NOT ONE OF `Button`'S FOUR. `shipped` is a
                // family the colour system has always declared and no component
                // could name until `tone` stopped being a closed list.
                for (tone, label) in [
                    ("neutral", Key::ExampleTagDraft),
                    ("accent", Key::ExampleTagReview),
                    ("shipped", Key::ExampleTagShipped),
                    ("danger", Key::ExampleTagBlocked),
                ] {
                    Tag { key: "{tone}", tone, label: context.t(label) }
                }
            }
            div { class: "nd-example-row",
                for (variant, label) in [
                    (TagVariant::Soft, Key::ExampleTagDraft),
                    (TagVariant::Solid, Key::ExampleTagReview),
                    (TagVariant::Outline, Key::ExampleTagShipped),
                ] {
                    Tag {
                        key: "{variant.as_str()}",
                        variant,
                        tone: "accent",
                        label: context.t(label),
                    }
                }
            }
        }
    ),
    example!(
        r#"Tag { icon: IconName::Check, swatch: "…", image: "…" }"#,
        Key::ExampleTagBeside,
        |context| rsx! {
            div { class: "nd-example-row",
                Tag { icon: IconName::Check, tone: "success", label: context.t(Key::ExampleTagShipped) }
                Tag { class: "nd-identifier", swatch: "teal-balanced", label: "teal-balanced" }
                Tag { class: "nd-identifier", swatch: "clay-sober", label: "clay-sober" }
                // The site's own mark, which is a file the build already
                // writes — an example that pointed at a URL nothing serves
                // would be an example of the fallback.
                Tag {
                    image: format!("{}favicon.svg", crate::site::BASE),
                    label: context.t(Key::AppName),
                }
            }
        }
    ),
    example!(
        r#"Tag { href: "…" } / Tag { pressed: TagPressed::On } / Tag { disabled: true }"#,
        Key::ExampleTagKinds,
        |context| rsx! {
            div { class: "nd-example-row",
                Tag { class: "nd-identifier", href: "../button/", label: "Button" }
                Tag { class: "nd-identifier", href: "../select/", label: "Select" }
            }
            div { class: "nd-example-row",
                Tag {
                    pressed: TagPressed::On,
                    label: context.t(Key::ExampleTagShipped),
                }
                Tag { pressed: TagPressed::Off, label: context.t(Key::ExampleTagReview) }
                Tag { pressed: TagPressed::Off, disabled: true, label: context.t(Key::ExampleTagBlocked) }
            }
        }
    ),
    example!(
        r#"Tag { remove_label: "…", remove_name: "…", value: "…" } inside a <form>"#,
        Key::ExampleTagRemovable,
        // A REAL FORM, so the claim is made rather than described: taking a tag
        // off posts which one, and the server renders the list again. Press one
        // with the script blocked and it still works.
        |context| rsx! {
            form { class: "nd-example-form", method: "get",
                div { class: "nd-example-row",
                    for (value, label) in [
                        ("draft", Key::ExampleTagDraft),
                        ("review", Key::ExampleTagReview),
                        ("shipped", Key::ExampleTagShipped),
                    ] {
                        Tag {
                            key: "{value}",
                            value,
                            label: context.t(label),
                            remove_label: context.t(Key::ExampleTagRemove),
                            remove_name: "remove",
                        }
                    }
                }
            }
        }
    ),
    example!(
        r"Tag { size: TagSize::… } / Tag { shape: TagShape::… }",
        Key::ExampleTagSizes,
        |context| rsx! {
            div { class: "nd-example-row",
                for size in TagSize::ALL {
                    Tag {
                        key: "{size.as_str()}",
                        size,
                        label: context.t(Key::ExampleTagReview),
                        remove_label: context.t(Key::ExampleTagRemove),
                    }
                }
            }
            div { class: "nd-example-row",
                for shape in TagShape::ALL {
                    Tag {
                        key: "{shape.as_str()}",
                        shape,
                        variant: TagVariant::Outline,
                        label: context.t(Key::ExampleTagDraft),
                    }
                }
            }
        }
    ),
    example!(
        r"Tag { glass: true }",
        Key::ExampleGlassBehind,
        |context| rsx! {
            div { class: "nd-example-row",
                // THE WORD AND THE COLOUR SAY THE SAME THING. Four pills
                // reading *In review* with one painted `danger` and one
                // `shipped` is the page teaching the opposite of the rule two
                // examples above it.
                for (tone, label) in [
                    ("accent", Key::ExampleTagReview),
                    ("neutral", Key::ExampleTagDraft),
                    ("danger", Key::ExampleTagBlocked),
                    ("shipped", Key::ExampleTagShipped),
                ] {
                    Tag {
                        key: "{tone}",
                        tone,
                        glass: true,
                        variant: TagVariant::Outline,
                        label: context.t(label),
                    }
                }
            }
        }
    ),
];

// --- Badge ------------------------------------------------------------------

const BADGE: &[Example] = &[
    example!(
        r#"Badge { count: "…", max: "99", label: "…" }"#,
        Key::ExampleBadgeCount,
        |context| rsx! {
            div { class: "nd-example-row",
                Badge { count: "3", label: context.t(Key::ExampleBadgeUnread),
                    Button {
                        variant: ButtonVariant::Outline,
                        tone: ButtonTone::Neutral,
                        label: context.t(Key::ExampleBadgeInbox),
                    }
                }
                // THE LABEL IS THE COUNT THE BADGE STANDS FOR, NOT THE ONE IT
                // DRAWS. `max` caps the digits at `99+`; it does not cap what
                // happened, and a reader who cannot see the corner is owed the
                // real number.
                Badge { count: "128", max: "99", label: context.t(Key::ExampleBadgeUnread128),
                    Button {
                        variant: ButtonVariant::Outline,
                        tone: ButtonTone::Neutral,
                        label: context.t(Key::ExampleBadgeInbox),
                    }
                }
            }
        }
    ),
    example!(
        r#"Badge { dot: true } / Badge { count: "0" } / Badge { count: "0", show_zero: true }"#,
        Key::ExampleBadgeDot,
        |context| rsx! {
            div { class: "nd-example-row",
                Badge { dot: true, label: context.t(Key::ExampleBadgeChanged),
                    Button {
                        variant: ButtonVariant::Outline,
                        tone: ButtonTone::Neutral,
                        hide_label: true,
                        icon: IconName::Settings,
                        label: context.t(Key::SettingsLabel),
                    }
                }
                Badge { count: "0", label: context.t(Key::ExampleBadgeUnread0),
                    Button {
                        variant: ButtonVariant::Outline,
                        tone: ButtonTone::Neutral,
                        hide_label: true,
                        icon: IconName::Settings,
                        label: context.t(Key::SettingsLabel),
                    }
                }
                Badge { count: "0", show_zero: true, label: context.t(Key::ExampleBadgeUnread0),
                    Button {
                        variant: ButtonVariant::Outline,
                        tone: ButtonTone::Neutral,
                        hide_label: true,
                        icon: IconName::Settings,
                        label: context.t(Key::SettingsLabel),
                    }
                }
            }
        }
    ),
    example!(
        r#"Badge { tone: "…" }"#,
        Key::ExampleBadgeTones,
        |context| rsx! {
            div { class: "nd-example-row",
                // The last is a family from the colour system's long tail — the
                // whole point of a tone that is a name rather than a list.
                for tone in ["danger", "accent", "neutral", "overdue"] {
                    Badge {
                        key: "{tone}",
                        tone,
                        count: "9",
                        label: context.t(Key::ExampleBadgeUnread9),
                        Button {
                            variant: ButtonVariant::Outline,
                            tone: ButtonTone::Neutral,
                            label: context.t(Key::ExampleBadgeInbox),
                        }
                    }
                }
            }
        }
    ),
    example!(
        r"Badge { placement: BadgePlacement::… } / Badge { overlap: BadgeOverlap::… }",
        Key::ExampleBadgePlacement,
        |context| rsx! {
            // A ROOMIER ROW, and the reason is the component: a badge draws
            // OUTSIDE the box of what it marks, so four of them at four corners
            // in an ordinary row collide with their neighbours. Nothing can
            // reserve that space — reserving it is what a badge is not.
            div { class: "nd-example-row nd-example-row-airy",
                for placement in BadgePlacement::ALL {
                    Badge {
                        key: "{placement.as_str()}",
                        placement,
                        count: "2",
                        tone: "accent",
                        label: context.t(Key::ExampleBadgeUnread2),
                        Button {
                            variant: ButtonVariant::Outline,
                            tone: ButtonTone::Neutral,
                            label: context.t(Key::ExampleBadgeInbox),
                        }
                    }
                }
            }
            div { class: "nd-example-row",
                for overlap in BadgeOverlap::ALL {
                    Badge {
                        key: "{overlap.as_str()}",
                        overlap,
                        count: "2",
                        label: context.t(Key::ExampleBadgeUnread2),
                        Tag {
                            shape: if overlap == BadgeOverlap::Circle { TagShape::Pill } else { TagShape::Square },
                            variant: TagVariant::Outline,
                            label: context.t(Key::ExampleBadgeInbox),
                        }
                    }
                }
            }
        }
    ),
];

// --- Avatar and AvatarGroup -------------------------------------------------

/// The people the avatar examples are about.
///
/// Real names with real initials, and one of each shape the fallback takes: a
/// picture that loads, letters, and nothing at all. The pictures are the site's
/// own mark, because it is a file the build already writes — an example
/// pointing at a URL nothing answers would be a permanent 404 on a published
/// page, and `theme.spec.js` refuses those in writing.
macro_rules! people {
    ($context:expr) => {
        rsx! {
            li {
                Avatar {
                    label: $context.t(Key::ExampleAvatarAna),
                    initials: "AL",
                    src: format!("{}favicon.svg", crate::site::BASE),
                }
            }
            li { Avatar { label: $context.t(Key::ExampleAvatarBruno), initials: "BS" } }
            li { Avatar { label: $context.t(Key::ExampleAvatarCarla), initials: "CM" } }
            li { Avatar { label: $context.t(Key::ExampleAvatarPending) } }
        }
    };
}

const AVATAR: &[Example] = &[
    example!(
        r#"Avatar { label: "…", src: "…" } / { initials: "…" } / { icon: IconName::Monitor }"#,
        Key::ExampleAvatarLayers,
        |context| rsx! {
            div { class: "nd-example-row",
                Avatar {
                    label: context.t(Key::ExampleAvatarAna),
                    initials: "AL",
                    src: format!("{}favicon.svg", crate::site::BASE),
                }
                Avatar { label: context.t(Key::ExampleAvatarBruno), initials: "BS" }
                // NOT `user`, WHICH IS WHAT ABSENCE ALREADY DRAWS. An example
                // setting a prop to its own default shows the prop doing
                // nothing. This account is a machine, so it gets a machine.
                Avatar { label: context.t(Key::ExampleAvatarDeployBot), icon: IconName::Monitor }
            }
        }
    ),
    example!(
        r"Avatar { size: AvatarSize::… } / Avatar { shape: AvatarShape::… }",
        Key::ExampleAvatarBox,
        |context| rsx! {
            div { class: "nd-example-row",
                for size in AvatarSize::ALL {
                    Avatar {
                        key: "{size.as_str()}",
                        size,
                        initials: "CM",
                        label: context.t(Key::ExampleAvatarCarla),
                    }
                }
            }
            div { class: "nd-example-row",
                for shape in AvatarShape::ALL {
                    Avatar {
                        key: "{shape.as_str()}",
                        shape,
                        initials: "CM",
                        label: context.t(Key::ExampleAvatarCarla),
                    }
                }
            }
        }
    ),
    example!(
        r"Avatar { tone: AvatarTone::… }",
        Key::ExampleAvatarTone,
        |context| rsx! {
            div { class: "nd-example-row",
                // FOUR SETS OF INITIALS ARE FOUR PEOPLE, so each carries its
                // own name. One name over four faces makes three of them
                // announce somebody who is not there.
                for (tone, initials, name) in [
                    (AvatarTone::Neutral, "AL", Key::ExampleAvatarAna),
                    (AvatarTone::Accent, "BS", Key::ExampleAvatarBruno),
                    (AvatarTone::Success, "CM", Key::ExampleAvatarCarla),
                    (AvatarTone::Danger, "DR", Key::ExampleAvatarDiego),
                ] {
                    Avatar {
                        key: "{tone.as_str()}",
                        tone,
                        initials,
                        label: context.t(name),
                    }
                }
            }
        }
    ),
    example!(
        r#"Avatar { href: "…" } / Avatar { decorative: true }"#,
        Key::ExampleAvatarKinds,
        |context| rsx! {
            div { class: "nd-example-row",
                Avatar {
                    href: "../button/",
                    initials: "AL",
                    label: context.t(Key::ExampleAvatarAna),
                }
                // Beside a name that is already on the screen, so the picture
                // is a picture of a word the reader has just heard.
                span { class: "nd-example-byline",
                    Avatar { decorative: true, initials: "BS", label: context.t(Key::ExampleAvatarBruno) }
                    Text { "{context.t(Key::ExampleAvatarBruno)}" }
                }
            }
        }
    ),
];

const AVATAR_GROUP: &[Example] = &[
    example!(
        r#"AvatarGroup { label: "…", more: "+5", more_label: "…", more_href: "…" }"#,
        Key::ExampleAvatarGroupCounter,
        |context| rsx! {
            AvatarGroup {
                label: context.t(Key::ExampleAvatarReviewers),
                more: "+5",
                more_label: context.t(Key::ExampleAvatarMore),
                more_href: "../avatars/",
                {people!(context)}
            }
        }
    ),
    example!(
        r"AvatarGroup { overlap: AvatarGroupOverlap::… }",
        Key::ExampleAvatarGroupOverlap,
        |context| rsx! {
            div { class: "nd-example-stack nd-example-stack-wide",
                for overlap in AvatarGroupOverlap::ALL {
                    AvatarGroup {
                        key: "{overlap.as_str()}",
                        overlap,
                        label: context.t(Key::ExampleAvatarReviewers),
                        {people!(context)}
                    }
                }
            }
        }
    ),
    example!(
        r"AvatarGroup { size: AvatarGroupSize::…, shape: AvatarGroupShape::… }",
        Key::ExampleAvatarGroupBox,
        |context| rsx! {
            div { class: "nd-example-stack nd-example-stack-wide",
                for size in AvatarGroupSize::ALL {
                    AvatarGroup {
                        key: "{size.as_str()}",
                        size,
                        label: context.t(Key::ExampleAvatarReviewers),
                        more: "+2",
                        more_label: context.t(Key::ExampleAvatarMore),
                        {people!(context)}
                    }
                }
                for shape in AvatarGroupShape::ALL {
                    AvatarGroup {
                        key: "{shape.as_str()}",
                        shape,
                        size: AvatarGroupSize::Small,
                        label: context.t(Key::ExampleAvatarReviewers),
                        {people!(context)}
                    }
                }
            }
        }
    ),
    example!(
        r"AvatarGroup { children }",
        Key::ExampleAvatarGroupPlain,
        |context| rsx! {
            AvatarGroup { label: context.t(Key::ExampleAvatarReviewers),
                li { Avatar { label: context.t(Key::ExampleAvatarAna), initials: "AL" } }
                li { Avatar { label: context.t(Key::ExampleAvatarBruno), initials: "BS" } }
            }
        }
    ),
];

// --- TagPicker --------------------------------------------------------------

/// The suggestions every picker example offers.
///
/// `SelectOption` and `SelectGroup`, which is the point: they emit real
/// `<option>` and `<optgroup>` elements, so with no script the reader picks
/// from a native listbox. Their value equals their label here and must — a
/// reader can produce the label by typing it, and two ways to make one tag
/// must not submit two different strings.
macro_rules! topics {
    ($context:expr) => {
        rsx! {
            SelectGroup { label: $context.t(Key::ExamplePickerWriting),
                SelectOption { class: "nd-identifier", value: "docs", label: "docs" }
                SelectOption { class: "nd-identifier", value: "spec", label: "spec" }
            }
            SelectGroup { label: $context.t(Key::ExamplePickerBuilding),
                SelectOption { class: "nd-identifier", value: "rust", label: "rust" }
                SelectOption { class: "nd-identifier", value: "react", label: "react" }
                SelectOption { class: "nd-identifier", value: "css", label: "css" }
            }
        }
    };
}

const TAG_PICKER: &[Example] = &[
    example!(
        r#"TagPicker { name: "…", label: "…", hint: "…", placeholder: "…", status, create_label, empty_label, remove_label, added_label, removed_label }"#,
        Key::ExamplePickerFree,
        |context| rsx! {
            TagPicker {
                name: "topics",
                label: context.t(Key::ExamplePickerLabel),
                placeholder: context.t(Key::ExamplePickerPlaceholder),
                hint: context.t(Key::ExamplePickerHint),
                status: context.t(Key::ExamplePickerStatus),
                create_label: context.t(Key::ExamplePickerCreate),
                empty_label: context.t(Key::ExamplePickerEmpty),
                remove_label: context.t(Key::ExamplePickerRemove),
                added_label: context.t(Key::ExamplePickerAdded),
                removed_label: context.t(Key::ExamplePickerRemoved),
                {topics!(context)}
            }
        }
    ),
    example!(
        r#"TagPicker { creatable: false, separators: ",;", max: "3", max_label: "…" }"#,
        Key::ExamplePickerBounded,
        |context| rsx! {
            div { class: "nd-example-stack nd-example-stack-wide",
                TagPicker {
                    name: "from-the-list",
                    creatable: false,
                    label: context.t(Key::ExamplePickerLabel),
                    placeholder: context.t(Key::ExamplePickerPlaceholder),
                    empty_label: context.t(Key::ExamplePickerEmpty),
                    remove_label: context.t(Key::ExamplePickerRemove),
                    status: context.t(Key::ExamplePickerStatus),
                    {topics!(context)}
                }
                TagPicker {
                    name: "at-most-three",
                    separators: ",;",
                    max: "3",
                    max_label: context.t(Key::ExamplePickerMax),
                    label: context.t(Key::ExamplePickerLabel),
                    placeholder: context.t(Key::ExamplePickerPlaceholder),
                    create_label: context.t(Key::ExamplePickerCreate),
                    remove_label: context.t(Key::ExamplePickerRemove),
                    status: context.t(Key::ExamplePickerStatus),
                    {topics!(context)}
                }
            }
        }
    ),
    example!(
        r#"TagPicker { pattern: "…", invalid_label: "…", clearable: true, clear_label: "…" }
TagPicker { paste: false, duplicates: true, … }"#,
        Key::ExamplePickerRefusing,
        |context| rsx! {
            div { class: "nd-example-stack nd-example-stack-wide",
                // NO `paste: true` AND NO `duplicates: false` HERE. Those are
                // the defaults, so both showed the prop doing nothing, on the
                // one page a reader comes to to find out what they change.
                TagPicker {
                    name: "addresses",
                    // The platform's own `pattern`, not an invention here.
                    pattern: r"[^@\s]+@[^@\s]+\.[^@\s]+",
                    invalid_label: context.t(Key::ExamplePickerInvalid),
                    clearable: true,
                    clear_label: context.t(Key::ExamplePickerClear),
                    label: context.t(Key::ExamplePickerPeopleLabel),
                    placeholder: context.t(Key::ExamplePickerPeoplePlaceholder),
                    hint: context.t(Key::ExamplePickerPeopleHint),
                    create_label: context.t(Key::ExamplePickerCreate),
                    remove_label: context.t(Key::ExamplePickerRemove),
                    status: context.t(Key::ExamplePickerStatus),
                }
                // THE TWO VALUES THAT ARE NOT THE DEFAULT, on a picker where
                // both are right: a written name may contain the separator,
                // and two guests may share a name.
                TagPicker {
                    name: "guests",
                    paste: false,
                    duplicates: true,
                    label: context.t(Key::ExamplePickerNamesLabel),
                    placeholder: context.t(Key::ExamplePickerNamesPlaceholder),
                    hint: context.t(Key::ExamplePickerNamesHint),
                    create_label: context.t(Key::ExamplePickerCreate),
                    remove_label: context.t(Key::ExamplePickerRemove),
                    status: context.t(Key::ExamplePickerStatus),
                }
            }
        }
    ),
    example!(
        r#"TagPicker { required: true, error: "…" } inside a <form>"#,
        Key::ExamplePickerForm,
        // A REAL FORM AGAIN, because everything this component claims is a
        // claim about what a server receives — and because `required` on the
        // composer rather than on the hidden `<select>` is only checkable here.
        |context| rsx! {
            form { class: "nd-example-form", method: "get",
                TagPicker {
                    name: "labels",
                    required: true,
                    error: context.t(Key::ExamplePickerError),
                    label: context.t(Key::ExamplePickerLabel),
                    placeholder: context.t(Key::ExamplePickerPlaceholder),
                    create_label: context.t(Key::ExamplePickerCreate),
                    remove_label: context.t(Key::ExamplePickerRemove),
                    status: context.t(Key::ExamplePickerStatus),
                    {topics!(context)}
                }
                Button {
                    submit: ButtonSubmit::Submit,
                    label: context.t(Key::ExampleSelectSend),
                }
            }
        }
    ),
    example!(
        r"TagPicker { size: TagPickerSize::… } / { loading: true, loading_label } / { disabled: true } / { glass: true }",
        Key::ExamplePickerStates,
        |context| rsx! {
            div { class: "nd-example-stack nd-example-stack-wide",
                for size in TagPickerSize::ALL {
                    TagPicker {
                        key: "{size.as_str()}",
                        size,
                        name: "size-{size.as_str()}",
                        label: context.t(Key::ExamplePickerLabel),
                        placeholder: context.t(Key::ExamplePickerPlaceholder),
                        remove_label: context.t(Key::ExamplePickerRemove),
                        status: context.t(Key::ExamplePickerStatus),
                        {topics!(context)}
                    }
                }
                TagPicker {
                    name: "still-coming",
                    loading: true,
                    loading_label: context.t(Key::SearchLoading),
                    label: context.t(Key::ExamplePickerLabel),
                    placeholder: context.t(Key::ExamplePickerPlaceholder),
                    remove_label: context.t(Key::ExamplePickerRemove),
                }
                TagPicker {
                    name: "refused",
                    disabled: true,
                    label: context.t(Key::ExamplePickerLabel),
                    placeholder: context.t(Key::ExamplePickerPlaceholder),
                    remove_label: context.t(Key::ExamplePickerRemove),
                    {topics!(context)}
                }
                TagPicker {
                    name: "frosted",
                    glass: true,
                    label: context.t(Key::ExamplePickerLabel),
                    placeholder: context.t(Key::ExamplePickerPlaceholder),
                    remove_label: context.t(Key::ExamplePickerRemove),
                    {topics!(context)}
                }
            }
        }
    ),
];

// --- Text -------------------------------------------------------------------

const TEXT: &[Example] = &[
    example!(r"Every size", |context| rsx! {
        div { class: "nd-example-stack",
            for size in [TextSize::Caption, TextSize::Small, TextSize::Body, TextSize::Lead] {
                Text { size, "{context.t(Key::TypeSpecimenLine)}" }
            }
        }
    }),
    example!(r"Every weight", |context| rsx! {
        div { class: "nd-example-stack",
            for weight in [
                TextWeight::Regular,
                TextWeight::Medium,
                TextWeight::Semibold,
                TextWeight::Bold,
            ] {
                Text { weight, "{context.t(Key::TypeSpecimenLine)}" }
            }
        }
    }),
    example!(r"Every tone", |context| rsx! {
        div { class: "nd-example-stack",
            for tone in [TextTone::Default, TextTone::Muted, TextTone::Accent] {
                Text { tone, "{context.t(Key::TypeSpecimenLine)}" }
            }
        }
    }),
    example!(
        r"Text { numeric: true, … }",
        Key::ExampleTextNumeric,
        |_| rsx! {
            div { class: "nd-example-stack",
                Text { class: "nd-identifier", numeric: true, "1 111 111" }
                Text { class: "nd-identifier", numeric: true, "8 888 888" }
            }
        }
    ),
    example!(
        r"Text { size: Caption, weight: Semibold, tone: Muted }",
        Key::ExampleTextCombined,
        |context| rsx! {
            Text {
                size: TextSize::Caption,
                weight: TextWeight::Semibold,
                tone: TextTone::Muted,
                "{context.t(Key::TypeSpecimenLine)}"
            }
        }
    ),
];

// --- Time -------------------------------------------------------------------

const TIME: &[Example] = &[
    example!(
        r#"Time { datetime: "2026-07-29", "…" }"#,
        Key::ExampleTimeMachine,
        |context| rsx! {
            Paragraph {
                Time { datetime: "2026-07-29", "{context.t(Key::ExampleTimeDate)}" }
            }
        }
    ),
    example!(
        r#"Time { datetime: "2026-07-29T14:30", … }"#,
        |context| rsx! {
            Paragraph {
                Time { datetime: "2026-07-29T14:30", "{context.t(Key::ExampleTimeDateTime)}" }
            }
        }
    ),
    example!(r"In a sentence", |context| rsx! {
        Paragraph {
            "{context.t(Key::ExampleTimeLead)} "
            Time { datetime: "2026-07-29", "{context.t(Key::ExampleTimeDate)}" }
            "."
        }
    }),
    example!(
        r"In a list of dates",
        Key::ExampleTimeAligned,
        |context| rsx! {
            List { marker: ListMarker::None, spacing: ListSpacing::Tight,
                ListItem {
                    Time { datetime: "2026-07-01", "{context.t(Key::ExampleTimeDate)}" }
                }
                ListItem {
                    Time { datetime: "2026-07-29", "{context.t(Key::ExampleTimeDateTime)}" }
                }
            }
        }
    ),
];

// --- Tree and TreeItem ------------------------------------------------------

// THE TREE IS THIS SITE'S OWN PAGES, which is the only hierarchy on hand that
// is real — a tree of invented folders shows the indent and hides everything
// else. Every leaf is a link that goes somewhere, so the claim that the row is
// a link and the `<li>` is the node is checkable by following one.

/// The nodes a SELECTABLE tree shows: names and values, and no links.
///
/// Choosing and going somewhere are two answers to one click, and a row that
/// does both does both — the browser follows the link and the node is chosen on
/// a page the reader has just left. The contract says so; this is what saying
/// so looks like at a call site.
macro_rules! choices {
    ($context:expr) => {
        rsx! {
            TreeItem {
                label: $context.t(Key::ExampleTreeFoundations),
                icon: IconName::Palette,
                expanded: true,
                TreeItem { value: "typography", label: crate::site::TYPOGRAPHY.title($context.locale) }
                TreeItem { value: "spacing", label: crate::site::SPACING.title($context.locale) }
                TreeItem { value: "icons", label: crate::site::ICONS.title($context.locale) }
            }
            TreeItem {
                label: $context.t(Key::ExampleTreeControls),
                icon: IconName::Settings,
                expanded: true,
                TreeItem { class: "nd-identifier", value: "button", label: "Button" }
                TreeItem { class: "nd-identifier", value: "select", label: "Select" }
            }
        }
    };
}

/// The nodes every tree example shows.
macro_rules! branches {
    ($context:expr, $open:expr) => {
        rsx! {
            TreeItem {
                label: $context.t(Key::ExampleTreeFoundations),
                icon: IconName::Palette,
                expanded: $open,
                badge: "3",
                TreeItem { href: "../typography/", label: crate::site::TYPOGRAPHY.title($context.locale) }
                TreeItem { href: "../spacing/", label: crate::site::SPACING.title($context.locale) }
                TreeItem { href: "../icons/", label: crate::site::ICONS.title($context.locale) }
            }
            TreeItem {
                label: $context.t(Key::ExampleTreeControls),
                icon: IconName::Settings,
                expanded: $open,
                TreeItem { class: "nd-identifier", href: "../button/", label: "Button" }
                TreeItem { class: "nd-identifier", href: "../select/", label: "Select" }
                TreeItem {
                    class: "nd-identifier",
                    label: "Tag",
                    TreeItem { class: "nd-identifier", href: "../tag/", label: "Tag", current: true }
                    TreeItem { class: "nd-identifier", href: "../tag-picker/", label: "TagPicker" }
                }
            }
            // Children that were not passed: the lazily-loaded shape. Nothing
            // is hidden, because nothing is there — the link is where the rest
            // of it lives.
            TreeItem {
                label: $context.t(Key::ExampleTreeEverything),
                icon: IconName::Search,
                branch: true,
                href: "../search/",
            }
        }
    };
}

const TREE: &[Example] = &[
    example!(
        r#"Tree { name: "…", label: "…", expand_label: "…", collapse_label: "…", empty_label: "…" }"#,
        Key::ExampleTreeWalk,
        |context| rsx! {
            Tree {
                name: "pages",
                label: context.t(Key::ExampleTreeLabel),
                expand_label: context.t(Key::ExampleTreeExpand),
                collapse_label: context.t(Key::ExampleTreeCollapse),
                empty_label: context.t(Key::ExampleTreeEmpty),
                {branches!(context, true)}
            }
        }
    ),
    example!(
        r"Tree { guides: true, size: TreeSize::… }",
        Key::ExampleTreeGuides,
        |context| rsx! {
            div { class: "nd-example-stack nd-example-stack-wide",
                for size in TreeSize::ALL {
                    Tree {
                        key: "{size.as_str()}",
                        size,
                        guides: true,
                        name: "guides-{size.as_str()}",
                        label: context.t(Key::ExampleTreeLabel),
                        expand_label: context.t(Key::ExampleTreeExpand),
                        collapse_label: context.t(Key::ExampleTreeCollapse),
                        {branches!(context, true)}
                    }
                }
            }
        }
    ),
    example!(
        r#"Tree { selection: TreeSelection::Multiple, form: "…" } inside a <form>"#,
        Key::ExampleTreeSelection,
        |context| rsx! {
            form { class: "nd-example-form", method: "get",
                Tree {
                    name: "chosen",
                    selection: TreeSelection::Multiple,
                    form: "page",
                    label: context.t(Key::ExampleTreeLabel),
                    expand_label: context.t(Key::ExampleTreeExpand),
                    collapse_label: context.t(Key::ExampleTreeCollapse),
                    {choices!(context)}
                }
                Button {
                    submit: ButtonSubmit::Submit,
                    label: context.t(Key::ExampleSelectSend),
                }
            }
        }
    ),
    example!(
        r"Tree { selection: TreeSelection::Single }",
        Key::ExampleTreeSingle,
        |context| rsx! {
            Tree {
                name: "one",
                selection: TreeSelection::Single,
                label: context.t(Key::ExampleTreeLabel),
                expand_label: context.t(Key::ExampleTreeExpand),
                collapse_label: context.t(Key::ExampleTreeCollapse),
                {choices!(context)}
            }
        }
    ),
];

const TREE_ITEM: &[Example] = &[
    example!(
        r#"TreeItem { label: "…", href: "…", icon, icon_expanded, badge: "…" }"#,
        Key::ExampleTreeItemRow,
        |context| rsx! {
            Tree {
                name: "rows",
                label: context.t(Key::ExampleTreeLabel),
                expand_label: context.t(Key::ExampleTreeExpand),
                collapse_label: context.t(Key::ExampleTreeCollapse),
                TreeItem {
                    label: context.t(Key::ExampleTreeFoundations),
                    icon: IconName::Palette,
                    icon_expanded: IconName::Check,
                    badge: "3",
                    expanded: true,
                    TreeItem { href: "../typography/", label: crate::site::TYPOGRAPHY.title(context.locale) }
                    TreeItem { href: "../spacing/", label: crate::site::SPACING.title(context.locale) }
                }
            }
        }
    ),
    example!(
        r"TreeItem { expanded: true } / TreeItem { branch: true }",
        Key::ExampleTreeItemBranch,
        |context| rsx! {
            Tree {
                name: "branches",
                label: context.t(Key::ExampleTreeLabel),
                expand_label: context.t(Key::ExampleTreeExpand),
                collapse_label: context.t(Key::ExampleTreeCollapse),
                TreeItem {
                    label: context.t(Key::ExampleTreeControls),
                    expanded: true,
                    TreeItem { class: "nd-identifier", href: "../button/", label: "Button" }
                }
                TreeItem {
                    label: context.t(Key::ExampleTreeFoundations),
                    expanded: false,
                    TreeItem { href: "../spacing/", label: crate::site::SPACING.title(context.locale) }
                }
                TreeItem {
                    label: context.t(Key::ExampleTreeEverything),
                    branch: true,
                    href: "../search/",
                }
            }
        }
    ),
    example!(
        r"TreeItem { current: true } / TreeItem { disabled: true } / TreeItem { selected: true }",
        Key::ExampleTreeItemStates,
        |context| rsx! {
            Tree {
                name: "states",
                selection: TreeSelection::Multiple,
                label: context.t(Key::ExampleTreeLabel),
                expand_label: context.t(Key::ExampleTreeExpand),
                collapse_label: context.t(Key::ExampleTreeCollapse),
                TreeItem { class: "nd-identifier", value: "tag", label: "Tag", current: true }
                TreeItem { class: "nd-identifier", value: "button", label: "Button", selected: true }
                TreeItem { class: "nd-identifier", value: "select", label: "Select", disabled: true }
            }
        }
    ),
    example!(
        r#"TreeItem { value: "…" }"#,
        Key::ExampleTreeItemValue,
        |context| rsx! {
            form { class: "nd-example-form", method: "get",
                Tree {
                    name: "values",
                    selection: TreeSelection::Multiple,
                    form: "slug",
                    label: context.t(Key::ExampleTreeLabel),
                    expand_label: context.t(Key::ExampleTreeExpand),
                    collapse_label: context.t(Key::ExampleTreeCollapse),
                    TreeItem { value: "tag", label: crate::site::TYPOGRAPHY.title(context.locale) }
                    TreeItem { value: "spacing", label: crate::site::SPACING.title(context.locale) }
                }
                Button {
                    submit: ButtonSubmit::Submit,
                    label: context.t(Key::ExampleSelectSend),
                }
            }
        }
    ),
];

// --- Truncate ---------------------------------------------------------------

const TRUNCATE: &[Example] = &[
    example!(
        r#"Truncate { title: "…", "…" }"#,
        Key::ExampleTruncateWhole,
        |context| rsx! {
            div { class: "nd-example-narrow",
                Truncate {
                    title: context.t(Key::ExampleListLong),
                    "{context.t(Key::ExampleListLong)}"
                }
            }
        }
    ),
    example!(
        r"Truncate { lines: TruncateLines::_2, … }",
        |context| rsx! {
            div { class: "nd-example-narrow",
                Truncate {
                    lines: TruncateLines::_2,
                    title: context.t(Key::ExampleListLong),
                    "{context.t(Key::ExampleListLong)}"
                }
            }
        }
    ),
    example!(
        r"Truncate { lines: TruncateLines::_4, … }",
        |context| rsx! {
            div { class: "nd-example-narrow",
                Truncate {
                    lines: TruncateLines::_4,
                    title: context.t(Key::ExampleListLong),
                    "{context.t(Key::ExampleListLong)}"
                }
            }
        }
    ),
    example!(
        r"Short enough not to clip",
        Key::ExampleTruncateShort,
        |context| rsx! {
            div { class: "nd-example-narrow",
                Truncate {
                    title: context.t(Key::ExampleListOne),
                    "{context.t(Key::ExampleListOne)}"
                }
            }
        }
    ),
];

// --- VisuallyHidden ---------------------------------------------------------

const VISUALLY_HIDDEN: &[Example] = &[
    example!(
        r#"VisuallyHidden { "…" }"#,
        Key::ExampleHiddenNothing,
        |context| rsx! {
            Paragraph {
                VisuallyHidden { "{context.t(Key::ExampleHiddenText)}" }
                "{context.t(Key::ExampleHiddenVisible)}"
            }
        }
    ),
    // Hand-written markup, deliberately, in the one example that is ABOUT
    // hand-written markup: this is the technique, and `Button { hide_label }`
    // is the technique already done for you. The note says so.
    example!(
        r"Naming an icon-only control",
        Key::ExampleHiddenNaming,
        |context| rsx! {
            button {
                class: "nd-button",
                r#type: "button",
                "data-nd-variant": "outline",
                "data-nd-tone": "neutral",
                "data-nd-size": "medium",
                "data-nd-shape": "rounded",
                "data-nd-hide-label": "true",
                VisuallyHidden { "{context.t(Key::SearchLabel)}" }
                noctua_design::Icon { icon: IconName::Search }
            }
        }
    ),
    example!(
        r"VisuallyHidden { focusable: true, … }",
        Key::ExampleHiddenFocusable,
        |context| rsx! {
            VisuallyHidden { focusable: true,
                a { class: "nd-a", href: "#nd-main", "{context.t(Key::AppSkip)}" }
            }
        }
    ),
    example!(
        r"Extending a link's name",
        Key::ExampleHiddenContext,
        |context| rsx! {
            Paragraph {
                Link { href: "#",
                    "{context.t(Key::ExampleLinkText)}"
                    VisuallyHidden { " — {context.t(Key::ExampleHiddenContextWord)}" }
                }
            }
        }
    ),
];

#[cfg(test)]
mod tests {
    use super::*;
    use crate::generated::catalogue::COMPONENTS;

    /// EVERY component, and this is what makes that true.
    ///
    /// Thirteen of the twenty-five had no enum prop, so the generated variant
    /// grid rendered nothing at all for them and their page carried a heading
    /// saying "Every variant" over empty space. A component added to
    /// `spec/components/` now fails the build until somebody shows how it is
    /// used, which is the only version of this rule that survives.
    /// A tone that names no family is an attribute that paints nothing.
    ///
    /// `tone` is text rather than an enum, because the vocabulary belongs to
    /// the colour system's 351 names and not to any one component — which
    /// means a typo is no longer a compile error. It is this instead: the
    /// generated `tones::ALL` is the list, and a name outside it fails here
    /// rather than resolving to nothing on the screen, which is the quietest
    /// way a status disappears.
    #[test]
    fn every_tone_this_file_writes_is_one_the_palette_declares() {
        let source = include_str!("examples.rs");
        let mut wrong = Vec::new();
        for (index, line) in source.lines().enumerate() {
            // An example's `source` caption is prose about the prop rather than
            // a use of it, and it is where the ellipsis lives.
            if line.trim_start().starts_with("r\"") || line.trim_start().starts_with("r#\"") {
                continue;
            }
            let Some(at) = line.find("tone: \"") else {
                continue;
            };
            let rest = &line[at + 7..];
            let Some(end) = rest.find('"') else { continue };
            let name = &rest[..end];
            if !noctua_design::tones::known(name) {
                wrong.push(format!("examples.rs:{}: `{name}`", index + 1));
            }
        }
        assert!(
            wrong.is_empty(),
            "these tones name no family in the colour system:\n{}",
            wrong.join("\n")
        );
    }

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

    /// The examples must reach every prop the component declares.
    ///
    /// Not every combination — that is a product, not a page — but every prop
    /// named at least once across the set, so a prop cannot be added to the
    /// contract and left undemonstrated. `children` is exempt: every example
    /// Every icon an example names is an icon that exists.
    ///
    /// A component whose `icon` prop is typed cannot get this wrong — `Button`
    /// takes an `IconName`. The ones that take TEXT can: the body ends in
    /// `IconName::parse`, which answers `None` for a name nothing declares, and
    /// `None` draws nothing at all. `MenuItem { icon: "x" }` shipped that way
    /// and the glyph was simply absent, with no error anywhere — the file is
    /// `x.svg` and the icon's own name is `close`.
    #[test]
    fn every_icon_an_example_names_exists() {
        let source = include_str!("examples.rs");
        let mut missing = Vec::new();
        for line in source.lines() {
            // Its own doc comment names the icon that started this, so a
            // scanner reading its own file has to skip the prose.
            let trimmed = line.trim_start();
            if trimmed.starts_with("//") {
                continue;
            }
            let Some(at) = line.find("icon: \"") else {
                continue;
            };
            let rest = &line[at + 7..];
            let Some(end) = rest.find('"') else { continue };
            let name = &rest[..end];
            if IconName::parse(name).is_none() {
                missing.push(name.to_owned());
            }
        }
        assert!(missing.is_empty(), "no such icon: {missing:?}");
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
