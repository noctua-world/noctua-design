//! What pages exist, what they are called, and what order they go in.

use noctua_design_i18n::{Key, Locale};

use crate::generated::catalogue::{COMPONENTS, DocComponent};

/// What a page is.
///
/// The sidebar shows the introduction alone at the top, then the foundations as
/// a short block, then every component alphabetically and with no categories
/// among them — which is what `NOCTUA.md §9.10` closes on. A foundation is not a
/// component, so it is not in that list; it is above it.
///
/// The variants are **ordered**, and that ordering is the sidebar's: `Kind`
/// sorts before the title in [`sorted`], so the three groups can never
/// interleave however a locale spells their titles.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub(crate) enum Kind {
    /// The front door. Exactly one page, and it is not one of a list.
    Intro,
    /// Typography and icons.
    Foundation,
    /// One component of the design system.
    Component,
}

/// What a page is called as a run of search results.
///
/// The palette groups its results, and the group is what the reader is looking
/// at rather than what the page is: *Pages* for a place they can go, and the
/// component's own name for a prop that belongs to it. Read from `Kind` so a
/// page added to the site lands in a run without anybody choosing one.
#[must_use]
pub(crate) fn group_label(page: Page, locale: Locale) -> &'static str {
    match page.kind {
        Kind::Intro | Kind::Foundation => locale.message(Key::SearchGroupPages),
        Kind::Component => locale.message(Key::SearchGroupComponents),
    }
}

/// A page's title.
///
/// **The foundation block is translated; the component block is not.** A
/// foundation's title is a sentence about the system — *Typography*, *Icons* —
/// and a reader reads it. A component's title is its **name**: an identifier,
/// the same in every locale, and the thing a developer types. §1 keeps
/// identifiers in English; translating `Heading` to `Título` would name
/// something that does not exist.
///
/// **A page that groups several components is titled by the same rule**, and
/// this is the part that was wrong: `/tree/` was *Árvore* in Portuguese, and
/// `Árvore` names nothing a reader can type, import or search the site for any
/// more than `Título` does. A collection is titled with the name at its head —
/// `Tree`, `Search`, `Avatar`, `Table`, `Lineage` — or, where the components on
/// it share no name, with an English label (`Inline text`). Either way the
/// component block reads in one language, which is the language its contents
/// are written in.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum Title {
    /// Translated. Foundations only.
    Message(Key),
    /// Verbatim, in every locale: a component's name, or the English label of a
    /// page that groups several.
    Name(&'static str),
}

/// One documentation page.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct Page {
    /// The URL segment.
    pub slug: &'static str,
    /// Its title in the sidebar and the `<title>`.
    pub title: Title,
    /// The sentence under the title.
    pub summary: Key,
    /// Foundation or component.
    pub kind: Kind,
    /// The contract, for a component page.
    pub component: Option<DocComponent>,
}

/// How far a page is from the site root: `<locale>/<slug>/index.html`.
///
/// Uniform on purpose. Every document sits at exactly this depth — foundation
/// pages, component pages, all of them — so the base, the asset links and the
/// locale switch are one constant rather than a computation that has to agree
/// with the directory layout in four places.
pub(crate) const BASE: &str = "../../";

impl Page {
    /// The path this page is written to, within its locale directory.
    ///
    /// A directory with an `index.html` inside, so the URL is `/en/heading/`
    /// and no reader ever sees `.html` — least of all `index.html`, which is
    /// the file name for "this page has no name of its own".
    #[must_use]
    pub(crate) fn file(self) -> String {
        format!("{}/index.html", self.slug)
    }

    /// The URL directory, relative to the locale root: `heading/`.
    #[must_use]
    pub(crate) fn dir(self) -> String {
        format!("{}/", self.slug)
    }

    /// The href another page in the same locale uses to reach this one.
    ///
    /// Up out of this page's own directory, then down into the other's — which
    /// is what a sibling directory costs, and the reason the depth is uniform.
    #[must_use]
    pub(crate) fn href(self) -> String {
        format!("../{}/", self.slug)
    }

    /// The href this page uses to reach its translation.
    #[must_use]
    pub(crate) fn locale_href(self, other: Locale) -> String {
        format!("{BASE}{}/{}", other.tag(), self.dir())
    }

    /// Its title, in this locale.
    #[must_use]
    pub(crate) fn title(self, locale: Locale) -> &'static str {
        match self.title {
            Title::Message(key) => locale.message(key),
            Title::Name(name) => name,
        }
    }

    /// Whether the title is a name rather than a sentence.
    ///
    /// A component's title is `Heading`, in every locale, because that is what
    /// a developer types. The sidebar says so with `.nd-identifier`, which is
    /// what tells `every_sentence_on_every_page_is_translated` that an
    /// unbracketed word there is deliberate rather than a missed translation.
    #[must_use]
    pub(crate) const fn title_is_a_name(self) -> bool {
        matches!(self.title, Title::Name(_))
    }
}

// The pages, in DECLARATION order — which is not the order they are shown in.
// See `sorted`.
//
// `<locale>/index.html` is a redirect document rather than one of these: a
// door, not a page. See `main::redirect_to`.

/// The front door, and the only page of its kind.
///
/// It sorts first in every locale by construction — `Kind::Intro` is the lowest
/// variant and `Kind` is the first term of the sort key — which is also what
/// makes it the home page, since [`first`] is the top of the sidebar and both
/// the wordmark and the root redirect read it.
pub(crate) const INTRO: Page = Page {
    slug: "intro",
    title: Title::Message(Key::PageIntroTitle),
    summary: Key::PageIntroSummary,
    kind: Kind::Intro,
    component: None,
};
pub(crate) const TYPOGRAPHY: Page = Page {
    slug: "typography",
    title: Title::Message(Key::PageTypographyTitle),
    summary: Key::PageTypographySummary,
    kind: Kind::Foundation,
    component: None,
};
/// The one scale everything on this site is spaced from.
///
/// It had a page, and the page was deleted at the developer's request — which
/// is why `index::ENTRIES` says in so many words that the spacing tokens are
/// not searchable and that "adding them back means writing the page first".
/// This is that page, written again on purpose rather than by accident, and the
/// tokens go back into the index with it.
pub(crate) const SPACING: Page = Page {
    slug: "spacing",
    title: Title::Message(Key::PageSpacingTitle),
    summary: Key::PageSpacingSummary,
    kind: Kind::Foundation,
    component: None,
};
/// Every drawing the platform ships, and how to add one.
///
/// **Named *Icons list*, because *Icons* was the component wearing a
/// foundation's clothes.** `Icon` is a component and has its own row three
/// lines further down the sidebar, so two adjacent entries differed by a single
/// letter and neither said which was the gallery. This one is the gallery: what
/// each drawing means, and what `spec/icons.toml` has to say for a new one to
/// exist. Its title is a sentence a reader reads, so it is translated, and the
/// slug stays `icons` for the reason `/avatars/` keeps its own.
pub(crate) const ICONS: Page = Page {
    slug: "icons",
    title: Title::Message(Key::PageIconsTitle),
    summary: Key::PageIconsSummary,
    kind: Kind::Foundation,
    component: None,
};

/// The one component with no `spec/components/*.toml`.
///
/// Its props **are** `spec/theme.toml`'s scopable settings, and a second file
/// listing them could only disagree with the first — so its contract is
/// generated from the theme model and its page is written rather than derived.
/// It is still a component, and it is in the component block of the sidebar
/// with the rest of them.
pub(crate) const THEME_SCOPE: Page = Page {
    slug: "theme-scope",
    title: Title::Name("ThemeScope"),
    summary: Key::ComponentThemeScopeSummary,
    kind: Kind::Component,
    component: None,
};

/// The site's own apparatus, documented like everything else on it.
///
/// `ExampleFrame` is not in `spec/components/` and must not be: everything it
/// owns is documentation machinery — lazily importing a React bundle to prove
/// two packages agree, a viewport rig, an appearance toggle scoped to itself —
/// and none of that is something a design system ships to its consumers
/// (`NOCTUA.md §9.10`).
///
/// It still earns a page, for the same reason `ThemeScope` does: it is the
/// abstraction every example on this site sits in, and a reader who wants to
/// know what the controls above an example do has nowhere else to look. It
/// sits in the component block because that is where a reader looks for a
/// thing with props.
pub(crate) const EXAMPLE_FRAME: Page = Page {
    slug: "example-frame",
    title: Title::Name("ExampleFrame"),
    summary: Key::ComponentExampleFrameSummary,
    kind: Kind::Component,
    component: None,
};

/// Ten marks that go inside a sentence, on one page.
///
/// `Em` was a page. `Strong` was a page. A reader asking which mark a keystroke
/// takes opened three of them, and each held one paragraph, one frame and a
/// two-row table. They are one reference now.
///
/// WHICH components are on it is not written here — each of their contracts
/// says `page = "inline-text"`, and `pages` groups them by it. A list here
/// would be a second copy of that fact, and the two would stop agreeing the
/// first time somebody added an eleventh mark.
///
/// **The one collection with no name at its head** — there is no `InlineText` —
/// so its title is an English label rather than an identifier. It is still not
/// translated: it sits in the component block, and that block reads in one
/// language.
pub(crate) const INLINE_TEXT: Page = Page {
    slug: "inline-text",
    title: Title::Name("Inline text"),
    summary: Key::PageInlineTextSummary,
    kind: Kind::Component,
    component: None,
};

/// A search box, the run of results it groups, and one result.
///
/// A collection by the rule below: these three are only ever used together, and
/// a `SearchResult` on a page of its own would be a link with nothing to be a
/// result of.
pub(crate) const SEARCH: Page = Page {
    slug: "search",
    title: Title::Name("Search"),
    summary: Key::PageSearchSummary,
    kind: Kind::Component,
    component: None,
};

/// A person, and several of them.
///
/// A collection by the rule below: an `AvatarGroup` with no `Avatar`s in it is
/// an empty list, and the two are never reached for separately.
///
/// Titled `Avatar`, not *Avatars*: a plural is not an identifier either, and
/// *Avatares* was what a translator did with the one that was there. The slug
/// stays `avatars` — a URL a reader may have kept is not worth a rename, and
/// nothing derives the title from it.
pub(crate) const AVATARS: Page = Page {
    slug: "avatars",
    title: Title::Name("Avatar"),
    summary: Key::PageAvatarsSummary,
    kind: Kind::Component,
    component: None,
};

/// A scene of figures, and anything at all standing in one.
///
/// A collection because the two are only ever used together: a scene with
/// nothing in it is a picture of a graph, and a figure outside one has no place
/// to stand.
pub(crate) const LINEAGE: Page = Page {
    slug: "lineage",
    title: Title::Name("Lineage"),
    summary: Key::PageLineageSummary,
    kind: Kind::Component,
    component: None,
};

/// A row a reader walks sideways, and one thing standing in it.
///
/// A collection by the rule below: a `CarouselSlide` outside a carousel is a
/// list item claiming to be a slide of something that does not exist.
pub(crate) const CAROUSEL: Page = Page {
    slug: "carousel",
    title: Title::Name("Carousel"),
    summary: Key::PageCarouselSummary,
    kind: Kind::Component,
    component: None,
};

/// Everything wrong with a form, and one thing wrong in it.
///
/// A collection by the rule below: an `ErrorSummaryItem` outside a summary is a
/// list item pointing at a field for no stated reason.
pub(crate) const ERROR_SUMMARY: Page = Page {
    slug: "error-summary",
    title: Title::Name("ErrorSummary"),
    summary: Key::PageErrorSummarySummary,
    kind: Kind::Component,
    component: None,
};

/// A hierarchy, and one node of it.
///
/// The menu and its items.
///
/// A collection by the rule below, and its slug is the component's own — the
/// same collision `tree` and `search` have, for the same reason: there is one
/// page at `/menu/` and it is the one the component points at.
pub(crate) const MENU: Page = Page {
    slug: "menu",
    title: Title::Name("Menu"),
    summary: Key::PageMenuSummary,
    kind: Kind::Component,
    component: None,
};

/// A collection by the rule below, and its slug is the component's own — the
/// same collision `search` has, and allowed for the same reason: there is one
/// page at `/tree/` and it is the one the component points at.
pub(crate) const TREE: Page = Page {
    slug: "tree",
    title: Title::Name("Tree"),
    summary: Key::PageTreeSummary,
    kind: Kind::Component,
    component: None,
};

/// The strip, one tab and one panel, which are only ever used together.
///
/// A collection for the reason `TREE` is one: `Tab` and `TabPanel` are parts
/// rather than alternatives, and three sidebar rows for one idea is three
/// places a reader has to look for it. Titled `Tabs`, which is the name a
/// developer types.
pub(crate) const TABS: Page = Page {
    slug: "tabs",
    title: Title::Name("Tabs"),
    summary: Key::PageTabsSummary,
    kind: Kind::Component,
    component: None,
};

/// A list, and one item of it.
///
/// A collection by the rule below: a `ListItem` outside a list is an `<li>`
/// with nothing to be an item of, and the two are never reached for
/// separately. Its slug is the component's own, which is the collision
/// `/search/`, `/menu/` and `/tree/` already have and is allowed for the same
/// reason: there is one page at `/list/` and it is the one `List` points at.
pub(crate) const LIST: Page = Page {
    slug: "list",
    title: Title::Name("List"),
    summary: Key::PageListSummary,
    kind: Kind::Component,
    component: None,
};

/// The row of options, and one of the values it chooses between.
///
/// A collection by the rule below, and the contract says so in as many words:
/// a `SegmentedItem` outside a `Segmented` is a `role="radio"` with no group,
/// which announces a control belonging to a set that does not exist.
pub(crate) const SEGMENTED: Page = Page {
    slug: "segmented",
    title: Title::Name("Segmented"),
    summary: Key::PageSegmentedSummary,
    kind: Kind::Component,
    component: None,
};

/// The field, a named run of options, and one option.
///
/// A collection by the rule below: an `<optgroup>` outside a `<select>` is not
/// a control at all, and an `<option>` on its own is a line of text. All three
/// are written in one call and read in one place.
pub(crate) const SELECT: Page = Page {
    slug: "select",
    title: Title::Name("Select"),
    summary: Key::PageSelectSummary,
    kind: Kind::Component,
    component: None,
};

/// The six components that make a table, and nothing else.
///
/// `Pagination` was on it once and has a page of its own now. Paging is a
/// question about a dataset rather than about the table showing it, and a list
/// of search results wants exactly the same control, so it was never a part of
/// this family. The `parts` list sixty lines below is the authority, and this
/// sentence used to disagree with it.
///
/// Titled `Table`, for the reason `AVATARS` is titled `Avatar`. Six parts
/// sharing one page does not make *Tables* a name. It makes the page's title
/// the name of the component a reader came for.
pub(crate) const TABLES: Page = Page {
    slug: "tables",
    title: Title::Name("Table"),
    summary: Key::PageTablesSummary,
    kind: Kind::Component,
    component: None,
};

/// One page several components are documented on, and the two sentences that
/// are its own rather than any component's.
///
/// The lead lives HERE rather than in a `match` beside the page body, because a
/// second list of which pages are collections is a second copy of this fact and
/// the two would stop agreeing the first time one was added.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct Collection {
    /// The page itself: slug, title, summary, sidebar position.
    pub page: Page,
    /// The paragraph under the summary, saying what these have in common.
    pub body: Key,
    /// The components on it, in READING order.
    ///
    /// Without this a collection page renders its components in the order
    /// `spec/components/*.toml` sorts by filename, which is a fact about the
    /// alphabet rather than about the subject. `/tables/` opened with the whole
    /// of `Pagination` — four examples and two reference tables — before the
    /// first table, because *p* precedes *t*; and inside the family it put
    /// `TableCell` before `TableColumn` and cells before the sections that
    /// contain them.
    ///
    /// Named rather than sorted, and `pages` panics on a component this list
    /// does not mention, so a new part has to be placed deliberately instead of
    /// landing wherever its filename falls.
    pub parts: &'static [&'static str],
}

/// Every collection page: one that several components are documented on.
///
/// A component's `page` must name one of these. `pages` panics otherwise, which
/// is a build failure rather than a component whose documentation quietly
/// disappeared.
///
/// **There were three.** `/forms` held `Checkbox`, `Radio`, `Fieldset`,
/// `Select`, `SelectGroup` and `SelectOption`, on the argument that a reader
/// choosing between them is comparing them. Removed at the developer's
/// request, and the argument was weaker than it looked: the two collections
/// that survive hold parts that are only ever used TOGETHER — ten marks that
/// go inside one sentence, and the six elements of one table. A checkbox and a
/// select are alternatives, not parts, and a page of alternatives is six
/// component pages a reader has to scroll past to reach the one they came for.
///
/// **`Select`, `SelectGroup` and `SelectOption` share a page again, and that
/// is not `/forms` coming back.** What `/forms` got wrong was putting
/// alternatives together: a reader who wanted a checkbox scrolled past a
/// select to reach it. These three are one control written in one call, and
/// neither of the other two means anything outside it — the same test `Table`
/// and its parts pass. `List` with `ListItem` and `Segmented` with
/// `SegmentedItem` are that test again, and each replaces a sidebar row for a
/// component nobody looks up on its own.
///
/// **`Pagination` used to be on `/tables/` and is not any more.** Its own
/// contract already argued the point — *"a component of its own rather than a
/// prop on `Table` because paging is a question about the DATASET, and a list
/// of search results needs exactly the same control"* — and the page order made
/// the cost visible: a reader who clicked *Table* in the sidebar got the whole
/// of `Pagination` first, because `pagination.toml` sorts before `table.toml`.
/// It also gained something by leaving, and that half is fixed rather than
/// still true: `index.rs` used to index a page only where
/// `page.component.is_some()`, so nobody could search for `spread` or
/// `current_label`. It reads `documented_on` now, so a component on a shared
/// page is as findable as one with a page of its own.
pub(crate) const COLLECTIONS: [Collection; 13] = [
    Collection {
        page: LINEAGE,
        parts: &["Lineage", "LineageFigure"],
        body: Key::PageLineageBody,
    },
    Collection {
        page: ERROR_SUMMARY,
        parts: &["ErrorSummary", "ErrorSummaryItem"],
        body: Key::PageErrorSummaryBody,
    },
    Collection {
        page: CAROUSEL,
        parts: &["Carousel", "CarouselSlide"],
        body: Key::PageCarouselBody,
    },
    Collection {
        page: TREE,
        parts: &["Tree", "TreeItem"],
        body: Key::PageTreeBody,
    },
    Collection {
        page: MENU,
        parts: &["Menu", "MenuItem"],
        body: Key::PageMenuBody,
    },
    Collection {
        // The strip first, then one tab, then one panel: the order a caller
        // writes them in, which is not the order their filenames sort in.
        page: TABS,
        parts: &["Tabs", "Tab", "TabPanel"],
        body: Key::PageTabsBody,
    },
    Collection {
        page: AVATARS,
        parts: &["Avatar", "AvatarGroup"],
        body: Key::PageAvatarsBody,
    },
    Collection {
        page: INLINE_TEXT,
        // The one collection whose members are alternatives rather than parts,
        // so the order is what a reader would look for: the run of text first,
        // then emphasis, then the marks that annotate a document, then the two
        // that are machine-readable.
        parts: &[
            "Text", "Strong", "Em", "Mark", "Code", "Kbd", "Abbr", "Time", "Ins", "Del",
        ],
        body: Key::PageInlineTextBody,
    },
    Collection {
        page: SEARCH,
        parts: &["Search", "SearchGroup", "SearchResult"],
        body: Key::PageSearchBody,
    },
    Collection {
        // The list first, then the item, which is the order a caller writes
        // them in and the order the alphabet happens to agree with.
        page: LIST,
        parts: &["List", "ListItem"],
        body: Key::PageListBody,
    },
    Collection {
        page: SEGMENTED,
        parts: &["Segmented", "SegmentedItem"],
        body: Key::PageSegmentedBody,
    },
    Collection {
        // The field, then the run of options, then one option: outside in,
        // which is the order they nest and not the order they sort in.
        page: SELECT,
        parts: &["Select", "SelectGroup", "SelectOption"],
        body: Key::PageSelectBody,
    },
    Collection {
        page: TABLES,
        parts: &[
            "Table",
            "TableColumn",
            "TableSection",
            "TableRow",
            "TableHeaderCell",
            "TableCell",
        ],
        body: Key::PageTablesBody,
    },
];

/// The collection a page is, if it is one.
///
/// `None` for every other page, which is what makes the body's dispatch a
/// question about the page rather than a second list of slugs.
#[must_use]
pub(crate) fn collection(page: Page) -> Option<Collection> {
    COLLECTIONS
        .iter()
        .find(|collection| collection.page.slug == page.slug)
        .copied()
}

/// The pages that are not components.
///
/// Three were deleted, and each for its own reason. `/effects` documented four
/// settings a reader reaches from the appearance panel in one click, and was
/// the only page whose whole subject was decoration — which `NOCTUA.md §9.7`
/// says the system has none of. `/colors-and-themes` and `/spacing` were
/// removed at the developer's request; what survived of the first is the
/// `ThemeScope` page above, because deleting the page would otherwise have
/// deleted the only rendered scope on the site and with it the browser
/// coverage of the invariant the whole theme layer rests on.
///
/// **`/spacing` is back**, also at the developer's request, and rebuilt rather
/// than restored: the page that was deleted carried its own copy of the seven
/// named gaps and the twelve steps, and this one reads both from the generated
/// scale. `index::ENTRIES` had already written down what would have to happen
/// first — "adding them back means writing the page first" — so the twenty-one
/// spacing names are searchable again with it.
pub(crate) const FOUNDATIONS: [Page; 4] = [INTRO, TYPOGRAPHY, ICONS, SPACING];

/// Every page: the foundations, then one per component.
///
/// Built rather than listed. The old `const [Page; 7]` had a single page called
/// "Components" on it, which is what a hand-maintained list of pages turns into
/// — and adding a component to `spec/components/` documented nothing at all.
#[must_use]
pub(crate) fn pages() -> Vec<Page> {
    let mut pages = FOUNDATIONS.to_vec();
    pages.push(THEME_SCOPE);
    pages.push(EXAMPLE_FRAME);
    pages.extend(COLLECTIONS.iter().map(|collection| collection.page));
    pages.extend(
        COMPONENTS
            .iter()
            // A component documented on a collection page has no page of its
            // own — and a component naming a collection nobody declared has no
            // page at all, which is why that is a panic and not a `filter`.
            .filter(|component| match component.page {
                None => true,
                Some(page) => {
                    assert!(
                        COLLECTIONS.iter().any(|known| known.page.slug == page),
                        "{} is documented on `{page}`, which is not a page",
                        component.name
                    );
                    false
                }
            })
            .map(|component| Page {
                slug: component.slug,
                title: Title::Name(component.name),
                summary: component.summary,
                kind: Kind::Component,
                component: Some(*component),
            }),
    );
    pages
}

/// The page with this slug, if the site has one.
///
/// An example that links to another page names it by slug and shows its title
/// in the reader's language, which is a lookup the site did not have: the
/// closures reached the `const`s by their Rust names, and a tree carries a
/// string. Built from [`pages`] rather than from a second list, so a page this
/// cannot find is a page that does not exist.
#[must_use]
pub(crate) fn by_slug(slug: &str) -> Option<Page> {
    pages().into_iter().find(|page| page.slug == slug)
}

/// Every component documented on one collection page, in READING order.
///
/// The order is the collection's own `parts`, not the catalogue's — see
/// `Collection::parts` for what the catalogue's order is a fact about.
#[must_use]
pub(crate) fn documented_on(page: Page) -> Vec<DocComponent> {
    let order = COLLECTIONS
        .iter()
        .find(|collection| collection.page.slug == page.slug)
        .map(|collection| collection.parts);
    let mut wanted: Vec<DocComponent> = COMPONENTS
        .iter()
        .filter(|component| component.page == Some(page.slug))
        .copied()
        .collect();
    if let Some(order) = order {
        wanted.sort_by_key(|component| {
            order
                .iter()
                .position(|name| *name == component.name)
                .unwrap_or(usize::MAX)
        });
    }
    wanted
}

/// The pages in the order this locale reads them.
///
/// # Sorted by the translated title, never by the slug
///
/// Sorting by `slug` would produce one fixed order for every language, which is
/// alphabetical in none of them but English. `icons` sorts before `typography`;
/// in Portuguese *Tipografia* comes before *Ícones*, which is the other order —
/// and in the pseudo-locale every title gains a leading bracket. The reader's
/// alphabet is the one that matters.
///
/// # Diacritics fold rather than sort after `z`
///
/// Rust's `str` ordering is by scalar value, so `Â` (U+00C2) sorts after every
/// unaccented letter — putting *Âmbar* at the end of a list where a Portuguese
/// reader expects it second. Folding the common Latin-1 diacritics to their base
/// letter for the sort key fixes that, with the raw title as the tie-break so
/// the order stays total and the build stays deterministic.
#[must_use]
pub(crate) fn sorted(locale: Locale) -> Vec<Page> {
    let mut pages = pages();
    // The introduction, then the foundations as a block, then every component
    // alphabetically with nothing between them. `Kind` sorts before the title,
    // so the three groups never interleave however a locale spells them.
    pages.sort_by_key(|page| {
        let title = page.title(locale);
        (page.kind, collate(title), title)
    });
    pages
}

/// Where a reader who asked for nothing in particular lands.
///
/// The first entry in this locale's sidebar, so the front door and the top of
/// the navigation are the same place by construction rather than by a constant
/// somebody has to remember to update. [`INTRO`] is that page today, and it is
/// that page because it sorts there — not because anything names it.
#[must_use]
pub(crate) fn first(locale: Locale) -> Page {
    *sorted(locale).first().expect("the site has pages")
}

/// Whether a rule is drawn above this row of the sidebar.
///
/// True wherever the kind changes, so the introduction is separated from the
/// foundations and the foundations from the components. It is not a category
/// heading: a heading among the components is exactly what `NOCTUA.md §9.10`
/// forbids, and a rule says *these are a different sort of thing* without
/// naming a category nobody asked for.
#[must_use]
pub(crate) fn starts_a_group(pages: &[Page], index: usize) -> bool {
    index > 0 && pages[index - 1].kind != pages[index].kind
}

/// A sort key that ignores case and diacritics.
#[must_use]
pub(crate) fn collate(text: &str) -> String {
    text.chars()
        .flat_map(|ch| fold(ch).to_lowercase())
        .filter(|ch| ch.is_alphanumeric() || *ch == ' ')
        .collect()
}

/// The base letter behind an accented one.
///
/// Deliberately a table rather than Unicode normalization: this repository takes
/// no dependency it does not need (`NOCTUA.md §2.3`), the alphabet in question
/// is Portuguese and English, and a table a reader can check by eye beats a
/// crate they have to trust.
fn fold(ch: char) -> char {
    match ch {
        'á' | 'à' | 'â' | 'ã' | 'ä' | 'Á' | 'À' | 'Â' | 'Ã' | 'Ä' => 'a',
        'é' | 'è' | 'ê' | 'ë' | 'É' | 'È' | 'Ê' | 'Ë' => 'e',
        'í' | 'ì' | 'î' | 'ï' | 'Í' | 'Ì' | 'Î' | 'Ï' => 'i',
        'ó' | 'ò' | 'ô' | 'õ' | 'ö' | 'Ó' | 'Ò' | 'Ô' | 'Õ' | 'Ö' => 'o',
        'ú' | 'ù' | 'û' | 'ü' | 'Ú' | 'Ù' | 'Û' | 'Ü' => 'u',
        'ç' | 'Ç' => 'c',
        'ñ' | 'Ñ' => 'n',
        other => other,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn every_page_has_a_distinct_slug() {
        let mut slugs: Vec<_> = pages().iter().map(|page| page.slug).collect();
        slugs.sort_unstable();
        let count = slugs.len();
        slugs.dedup();
        assert_eq!(slugs.len(), count);
    }

    /// `<locale>/index.html` is a redirect document, never a page. A page that
    /// claimed the name would be a landing page by the back door.
    #[test]
    fn no_page_claims_the_locale_index() {
        for page in pages() {
            assert_ne!(page.file(), "index.html", "{}", page.slug);
        }
    }

    /// No URL a reader can see ends in `.html`, and none of them says `index`.
    #[test]
    fn no_href_shows_a_file_name() {
        for page in pages() {
            for href in [page.href(), page.dir(), page.locale_href(Locale::En)] {
                assert!(!href.contains(".html"), "{href}");
                assert!(
                    href.ends_with('/'),
                    "a directory URL needs its slash: {href}"
                );
            }
        }
    }

    /// Every document is at the same depth, which is what makes [`BASE`] a
    /// constant instead of a calculation.
    #[test]
    fn every_page_is_two_levels_below_the_root() {
        for page in pages() {
            assert_eq!(page.file().matches('/').count(), 1, "{}", page.slug);
            assert_eq!(BASE, "../../");
        }
    }

    /// The front door is the top of the sidebar, in every locale.
    #[test]
    fn the_first_page_is_the_first_in_the_sidebar() {
        for locale in Locale::ALL {
            assert_eq!(first(locale), sorted(locale)[0], "{}", locale.tag());
        }
    }

    /// The sidebar is alphabetical **in the reader's language**, inside each
    /// group.
    ///
    /// This used to compare the English order against the Portuguese one and
    /// assert they differed — which tested the property only for as long as
    /// some page happened to sort differently in the two. Deleting two
    /// foundations left `Icons` and `Typography`, which sort the same way in
    /// both, and the test failed while the sort was perfectly correct. It was
    /// asserting a *consequence* that the page set happened to produce.
    ///
    /// The property itself is this: within a group, the collated titles never
    /// go backwards, in every locale. Sorting by `slug` would break it in
    /// Portuguese — *Tipografia* before *Ícones* — and in the pseudo-locale,
    /// where every title gains a leading bracket.
    #[test]
    fn each_group_is_alphabetical_in_the_readers_language() {
        for locale in Locale::ALL {
            let mut group = None;
            let mut previous = String::new();
            for page in sorted(locale) {
                let title = page.title(locale);
                if group != Some(page.kind) {
                    group = Some(page.kind);
                    previous = String::new();
                }
                assert!(
                    collate(title) >= previous,
                    "{} lists {title:?} after {previous:?}",
                    locale.tag()
                );
                previous = collate(title);
            }
        }
    }

    /// And the order really is read from the title rather than the slug.
    ///
    /// `theme-scope` is the proof: its title is `ThemeScope`, so it sorts
    /// between `Text` and `Time` — where a slug sort would put it after
    /// `Text` and before `Time` only by accident, and after `Strong` if the
    /// slug were spelled `themescope`.
    /// A collection's `parts` names every component on it, and nothing else.
    ///
    /// Without this the sort silently drops an unnamed component to the end and
    /// the page reads in an order nobody chose — which is the failure the field
    /// exists to replace, moved rather than fixed.
    #[test]
    fn every_collection_names_every_part_it_holds() {
        for collection in COLLECTIONS {
            let held: Vec<&str> = COMPONENTS
                .iter()
                .filter(|component| component.page == Some(collection.page.slug))
                .map(|component| component.name)
                .collect();
            for name in &held {
                assert!(
                    collection.parts.contains(name),
                    "`{name}` is documented on `{}` and its `parts` does not name it",
                    collection.page.slug
                );
            }
            for name in collection.parts {
                assert!(
                    held.contains(name),
                    "`{}`'s `parts` names `{name}`, which is documented somewhere else",
                    collection.page.slug
                );
            }
        }
    }

    #[test]
    fn a_collection_holds_the_components_that_name_it_and_they_have_no_page() {
        // The two halves of one fact, and the failure mode if they part is a
        // component whose documentation is nowhere: on no page of its own
        // because it named a collection, and on no collection because the name
        // was wrong. `pages` panics on the second; this is the first.
        for collection in COLLECTIONS {
            assert!(
                !documented_on(collection.page).is_empty(),
                "{} is a page nothing is documented on",
                collection.page.slug
            );
            assert_eq!(
                self::collection(collection.page),
                Some(collection),
                "{} is not reachable by its own page",
                collection.page.slug
            );
        }
        // A COMPONENT THAT NAMES A COLLECTION MUST NOT ALSO PRODUCE A PAGE —
        // and a page that happens to share its slug is only allowed when it is
        // that very collection.
        //
        // This was `!slugs.contains(&component.slug)`, which read the presence
        // of the slug as proof of a second page and was right until a
        // collection was named after one of the components on it. `Table` /
        // `tables` never collided; `Search` / `search` does, and there is no
        // second page — there is one page, and it is the one the component
        // pointed at. Comparing the slugs it landed on says exactly that, and
        // still fails on a component that names the wrong collection.
        let pages = pages();
        for component in COMPONENTS {
            if let Some(page) = component.page {
                if let Some(found) = pages.iter().find(|it| it.slug == component.slug) {
                    assert_eq!(
                        found.slug, page,
                        "{} is documented on `{page}` and still has a page of its own",
                        component.name
                    );
                }
            }
        }
    }

    #[test]
    fn a_page_sorts_by_its_title_not_its_slug() {
        let order: Vec<&str> = sorted(Locale::En)
            .iter()
            .map(|page| page.title(Locale::En))
            .collect();
        let at = |name: &str| {
            order
                .iter()
                .position(|title| *title == name)
                .unwrap_or_else(|| panic!("{name} is not in the sidebar: {order:?}"))
        };
        // `Swatch` sorts before `Switch` and both before `ThemeScope`, which
        // is title order and not slug order — `theme-scope` would sit between
        // `swatch` and `switch` if the slug were the key. It used to be `Text`
        // and `Time` that said this; they are on the inline-text page now, and
        // a test that names a page is a test that has to move when it does.
        assert!(at("Swatch") < at("Switch"));
        assert!(at("Switch") < at("ThemeScope"));
        assert!(at("ThemeScope") < at("Toolbar"));
    }

    /// THE COMPONENT BLOCK READS THE SAME IN EVERY LANGUAGE.
    ///
    /// `/tree/` was *Árvore* in Portuguese, `/search/` was *Busca* and
    /// `/avatars/` was *Avatares* — three sidebar rows naming components that do
    /// not exist under those names in any file, any import or any search. It
    /// shipped because a translated title is precisely what `Title::Message` is
    /// for, so nothing could tell a foundation's title from a component's.
    ///
    /// The two are different types now, and this is the assertion that says
    /// which block each belongs to. Structural on purpose: comparing the
    /// English spelling against the Portuguese one would pass the day somebody
    /// translated a title to the same word, and the comment above
    /// `each_group_is_alphabetical_in_the_readers_language` records what
    /// asserting a *consequence* costs.
    #[test]
    fn the_component_block_is_never_translated_and_the_foundations_always_are() {
        for page in pages() {
            match page.kind {
                Kind::Component => {
                    assert!(
                        page.title_is_a_name(),
                        "{} is in the component block with a translated title",
                        page.slug
                    );
                    for locale in Locale::ALL {
                        assert_eq!(
                            page.title(locale),
                            page.title(Locale::En),
                            "{} is spelled differently in {}",
                            page.slug,
                            locale.tag()
                        );
                    }
                }
                Kind::Intro | Kind::Foundation => assert!(
                    !page.title_is_a_name(),
                    "{} is a sentence a reader reads, so it is a message",
                    page.slug
                ),
            }
        }
    }

    #[test]
    fn every_locale_orders_every_page() {
        for locale in Locale::ALL {
            assert_eq!(sorted(locale).len(), pages().len(), "{}", locale.tag());
        }
    }

    /// `Â` sorts after `z` by scalar value, which would put *Âmbar* last in a
    /// list where a Portuguese reader expects it second.
    #[test]
    fn diacritics_fold_to_their_base_letter() {
        assert_eq!(collate("Âmbar"), "ambar");
        assert_eq!(collate("Espaçamento"), "espacamento");
        assert_eq!(collate("Visão geral"), "visao geral");
        assert!(collate("Âmbar") < collate("Azul"));
        assert!("Âmbar" > "Azul", "the raw comparison is the wrong one");
    }

    #[test]
    fn the_sort_is_total_and_deterministic() {
        for locale in Locale::ALL {
            assert_eq!(sorted(locale), sorted(locale));
        }
    }
}
