//! What pages exist, what they are called, and what order they go in.

use noctua_design_i18n::{Key, Locale};

use crate::generated::catalogue::{COMPONENTS, DocComponent};

/// What a page is.
///
/// The sidebar shows foundations as a short block at the top and then every
/// component, alphabetically and with no categories among them — which is what
/// `NOCTUA.md §9.10` closes on. A foundation is not a component, so it is not
/// in that list; it is above it.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum Kind {
    /// Colours, typography, spacing, icons, effects, scoped themes.
    Foundation,
    /// One component of the design system.
    Component,
}

/// A page's title.
///
/// A foundation's title is a message. A component's title is its **name** — an
/// identifier, the same in every locale, and the thing a developer types. §1
/// keeps identifiers in English; translating `Heading` to `Título` would name
/// something that does not exist.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum Title {
    /// Translated.
    Message(Key),
    /// An identifier, verbatim.
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
    /// A directory with an `index.html` inside, so the URL is `/en/colours/`
    /// and no reader ever sees `.html` — least of all `index.html`, which is
    /// the file name for "this page has no name of its own".
    #[must_use]
    pub(crate) fn file(self) -> String {
        format!("{}/index.html", self.slug)
    }

    /// The URL directory, relative to the locale root: `colours/`.
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
}

/// Every page, in **declaration** order — which is not the order they are shown
/// in. See [`sorted`].
///
/// There is no landing page. An "overview" existed and was deleted: a
/// documentation site's front door is its first real page, and a page whose
/// content is a description of the other pages is a table of contents the
/// sidebar already is. `<locale>/index.html` is a redirect document, not a
/// page — see `main::redirect_to`.
pub(crate) const COLOURS: Page = Page {
    slug: "colours",
    title: Title::Message(Key::PageColoursTitle),
    summary: Key::PageColoursSummary,
    kind: Kind::Foundation,
    component: None,
};
pub(crate) const TYPOGRAPHY: Page = Page {
    slug: "typography",
    title: Title::Message(Key::PageTypographyTitle),
    summary: Key::PageTypographySummary,
    kind: Kind::Foundation,
    component: None,
};
pub(crate) const SPACING: Page = Page {
    slug: "spacing",
    title: Title::Message(Key::PageSpacingTitle),
    summary: Key::PageSpacingSummary,
    kind: Kind::Foundation,
    component: None,
};
pub(crate) const ICONS: Page = Page {
    slug: "icons",
    title: Title::Message(Key::PageIconsTitle),
    summary: Key::PageIconsSummary,
    kind: Kind::Foundation,
    component: None,
};
pub(crate) const EFFECTS: Page = Page {
    slug: "effects",
    title: Title::Message(Key::PageEffectsTitle),
    summary: Key::PageEffectsSummary,
    kind: Kind::Foundation,
    component: None,
};
pub(crate) const SCOPED_THEMES: Page = Page {
    slug: "scoped-themes",
    title: Title::Message(Key::PageScopedThemesTitle),
    summary: Key::PageScopedThemesSummary,
    kind: Kind::Foundation,
    component: None,
};

/// The pages that are not components.
pub(crate) const FOUNDATIONS: [Page; 6] =
    [COLOURS, TYPOGRAPHY, SPACING, ICONS, EFFECTS, SCOPED_THEMES];

/// Every page: the foundations, then one per component.
///
/// Built rather than listed. The old `const [Page; 7]` had a single page called
/// "Components" on it, which is what a hand-maintained list of pages turns into
/// — and adding a component to `spec/components/` documented nothing at all.
#[must_use]
pub(crate) fn pages() -> Vec<Page> {
    let mut pages = FOUNDATIONS.to_vec();
    pages.extend(COMPONENTS.iter().map(|component| Page {
        slug: component.slug,
        title: Title::Name(component.name),
        summary: component.summary,
        kind: Kind::Component,
        component: Some(*component),
    }));
    pages
}

/// The pages in the order this locale reads them.
///
/// # Sorted by the translated title, never by the slug
///
/// Sorting by `slug` would produce one fixed order for every language, which is
/// alphabetical in none of them but English. `colours` sorts before `effects`;
/// *Cores* sorts before *Efeitos* too, but *Espaçamento* and *Efeitos* swap
/// places against `spacing` and `effects`, and in the pseudo-locale every title
/// gains a leading bracket. The reader's alphabet is the one that matters.
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
    // Foundations first, as a block; then every component, alphabetically and
    // with nothing between them. `Kind` sorts before the title, so the two
    // groups never interleave however a locale spells them.
    pages.sort_by_key(|page| {
        let title = page.title(locale);
        (matches!(page.kind, Kind::Component), collate(title), title)
    });
    pages
}

/// Where a reader who asked for nothing in particular lands.
///
/// The first entry in this locale's sidebar, so the front door and the top of
/// the navigation are the same place by construction rather than by a constant
/// somebody has to remember to update.
/// The first component in this locale's list — where the rule above the
/// component block is drawn.
#[must_use]
pub(crate) fn first_component(locale: Locale) -> Page {
    sorted(locale)
        .into_iter()
        .find(|page| page.kind == Kind::Component)
        .unwrap_or_else(|| first(locale))
}

#[must_use]
pub(crate) fn first(locale: Locale) -> Page {
    *sorted(locale).first().expect("the site has pages")
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

    /// The point of sorting by title: two locales legitimately disagree about
    /// the order, and a single fixed order would be wrong in at least one.
    #[test]
    fn the_order_follows_the_locale() {
        let english = sorted(Locale::En);
        let portuguese = sorted(Locale::PtBr);
        assert_ne!(
            english.iter().map(|p| p.slug).collect::<Vec<_>>(),
            portuguese.iter().map(|p| p.slug).collect::<Vec<_>>(),
            "both locales produced the same order, so the sort is not using the title"
        );
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
