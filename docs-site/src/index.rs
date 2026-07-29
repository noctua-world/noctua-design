//! What search can find beyond the page titles.
//!
//! A documentation search that only indexes page titles answers "where is the
//! colour page". It does not answer "what was that spacing token called", which
//! is the question a developer with a half-remembered name actually has — and
//! the one they will otherwise answer by grepping the source.
//!
//! Every entry here is read from the generated crates, so a token added to
//! `spec/tokens.toml` or a palette added upstream in `noctua-colors` becomes
//! searchable on the next build with no edit to this file.

use noctua_design_theme as theme;

use crate::site::{self, Page};

/// One searchable name and where it is documented.
#[derive(Debug, Clone, Copy)]
pub(crate) struct Entry {
    /// The name as a developer would type it.
    pub name: &'static str,
    /// The page that explains it.
    ///
    /// A `Page`, not a path. A search result that leads to a 404 used to be a
    /// typo away, guarded by a test that compared strings; now it does not
    /// compile.
    pub page: Page,
}

/// Every name worth finding.
///
/// Deliberately the *names*, not their values: a search result that shows
/// `0.25rem` invites copying the number instead of the token, which is the one
/// habit the source gates exist to break.
pub(crate) const ENTRIES: &[Entry] = &[
    Entry {
        name: "--noctua-space-base",
        page: site::SPACING,
    },
    Entry {
        name: "--noctua-space-tight",
        page: site::SPACING,
    },
    Entry {
        name: "--noctua-space-inset",
        page: site::SPACING,
    },
    Entry {
        name: "--noctua-space-gap",
        page: site::SPACING,
    },
    Entry {
        name: "--noctua-space-paragraph",
        page: site::SPACING,
    },
    Entry {
        name: "--noctua-space-heading",
        page: site::SPACING,
    },
    Entry {
        name: "--noctua-space-section",
        page: site::SPACING,
    },
    Entry {
        name: "--noctua-space-region",
        page: site::SPACING,
    },
    Entry {
        name: "--noctua-measure",
        page: site::TYPOGRAPHY,
    },
    Entry {
        name: "--noctua-measure-lead",
        page: site::TYPOGRAPHY,
    },
    Entry {
        name: "--noctua-size-body",
        page: site::TYPOGRAPHY,
    },
    Entry {
        name: "--noctua-size-h1",
        page: site::TYPOGRAPHY,
    },
    Entry {
        name: "--noctua-size-display",
        page: site::TYPOGRAPHY,
    },
    Entry {
        name: "--noctua-leading-body",
        page: site::TYPOGRAPHY,
    },
    Entry {
        name: "--noctua-weight-semibold",
        page: site::TYPOGRAPHY,
    },
    Entry {
        name: "--noctua-radius-control",
        page: site::COMPONENTS,
    },
    Entry {
        name: "--noctua-radius-surface",
        page: site::COMPONENTS,
    },
    Entry {
        name: "--noctua-glass-blur",
        page: site::EFFECTS,
    },
    Entry {
        name: "--noctua-glass-opacity",
        page: site::EFFECTS,
    },
    Entry {
        name: "--noctua-pattern-ink",
        page: site::EFFECTS,
    },
    Entry {
        name: "--noctua-glow-ink",
        page: site::EFFECTS,
    },
    Entry {
        name: "--nc-color-surface",
        page: site::COLOURS,
    },
    Entry {
        name: "--nc-color-fg",
        page: site::COLOURS,
    },
    Entry {
        name: "--nc-color-border",
        page: site::COLOURS,
    },
    Entry {
        name: "--nc-color-accent",
        page: site::COLOURS,
    },
    Entry {
        name: "--nc-color-on-accent",
        page: site::COLOURS,
    },
    Entry {
        name: "ThemeScope",
        page: site::COMPONENTS,
    },
    Entry {
        name: "useTheme",
        page: site::COMPONENTS,
    },
    Entry {
        name: "data-noctua-scope",
        page: site::SCOPED_THEMES,
    },
    Entry {
        name: "data-palette",
        page: site::SCOPED_THEMES,
    },
];

/// Every prop and every palette, appended to [`ENTRIES`] at build time.
///
/// Separate from the constant because the palette list has thirty-nine members
/// that come from the vendored axes rather than from anything written here.
#[must_use]
pub(crate) fn all() -> Vec<Entry> {
    let mut entries = ENTRIES.to_vec();
    for prop in crate::props::SCOPABLE {
        entries.push(Entry {
            name: prop.name,
            page: site::COMPONENTS,
        });
    }
    for palette in theme::Palette::ALL {
        entries.push(Entry {
            name: palette.as_str(),
            page: site::COLOURS,
        });
    }
    entries
}

#[cfg(test)]
mod tests {
    use super::*;

    /// A search result that leads nowhere is worse than no result.
    ///
    /// Now a type invariant rather than a comparison — kept as a tripwire in
    /// case `Entry.page` ever loosens back into a string.
    #[test]
    fn every_entry_points_at_a_real_page() {
        for entry in all() {
            assert!(
                site::PAGES.contains(&entry.page),
                "{} points at {}, which is not a page",
                entry.name,
                entry.page.slug
            );
        }
    }

    /// Every palette is findable, including any added upstream.
    #[test]
    fn every_palette_is_searchable() {
        let entries = all();
        for palette in theme::Palette::ALL {
            assert!(
                entries.iter().any(|entry| entry.name == palette.as_str()),
                "{} is not searchable",
                palette.as_str()
            );
        }
    }

    #[test]
    fn no_entry_carries_a_value() {
        for entry in all() {
            assert!(
                !entry.name.contains("rem") && !entry.name.contains("px"),
                "{} looks like a value; index names, not numbers",
                entry.name
            );
        }
    }
}
