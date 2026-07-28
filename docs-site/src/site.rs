//! What pages exist, what they are called, and what order they go in.

use noctua_design_i18n::{Key, Locale};

/// One documentation page.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct Page {
    /// The URL segment.
    pub slug: &'static str,
    /// Its title in the sidebar and the `<title>`.
    pub title: Key,
    /// The sentence under the title.
    pub summary: Key,
}

impl Page {
    /// The file name this page is written to, within its locale directory.
    #[must_use]
    pub(crate) fn file(self) -> String {
        format!("{}.html", self.slug)
    }

    /// The href another page in the same locale uses to reach this one.
    #[must_use]
    pub(crate) fn href(self) -> String {
        self.file()
    }
}

/// Every page, in **declaration** order — which is not the order they are shown
/// in. See [`sorted`].
///
/// There is no landing page. An "overview" existed and was deleted: a
/// documentation site's front door is its first real page, and a page whose
/// content is a description of the other pages is a table of contents the
/// sidebar already is. `<locale>/index.html` is a redirect document, not a
/// page — see `main::locale_redirect`.
pub(crate) const PAGES: [Page; 6] = [
    Page {
        slug: "colours",
        title: Key::PageColoursTitle,
        summary: Key::PageColoursSummary,
    },
    Page {
        slug: "typography",
        title: Key::PageTypographyTitle,
        summary: Key::PageTypographySummary,
    },
    Page {
        slug: "spacing",
        title: Key::PageSpacingTitle,
        summary: Key::PageSpacingSummary,
    },
    Page {
        slug: "effects",
        title: Key::PageEffectsTitle,
        summary: Key::PageEffectsSummary,
    },
    Page {
        slug: "scoped-themes",
        title: Key::PageScopedThemesTitle,
        summary: Key::PageScopedThemesSummary,
    },
    Page {
        slug: "components",
        title: Key::PageComponentsTitle,
        summary: Key::PageComponentsSummary,
    },
];

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
    let mut pages = PAGES.to_vec();
    pages.sort_by_key(|page| {
        let title = locale.message(page.title);
        (collate(title), title)
    });
    pages
}

/// Where a reader who asked for nothing in particular lands.
///
/// The first entry in this locale's sidebar, so the front door and the top of
/// the navigation are the same place by construction rather than by a constant
/// somebody has to remember to update.
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
        let mut slugs: Vec<_> = PAGES.iter().map(|page| page.slug).collect();
        slugs.sort_unstable();
        let count = slugs.len();
        slugs.dedup();
        assert_eq!(slugs.len(), count);
    }

    /// `index.html` is a redirect document, never a page. A page that claimed
    /// the name would be a landing page by the back door.
    #[test]
    fn no_page_claims_the_index() {
        for page in PAGES {
            assert_ne!(page.file(), "index.html", "{}", page.slug);
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
            assert_eq!(sorted(locale).len(), PAGES.len(), "{}", locale.tag());
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
