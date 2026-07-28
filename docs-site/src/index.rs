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

/// One searchable name and where it is documented.
#[derive(Debug, Clone, Copy)]
pub(crate) struct Entry {
    /// The name as a developer would type it.
    pub name: &'static str,
    /// The page that explains it, relative to the locale directory.
    pub page: &'static str,
}

/// Every name worth finding.
///
/// Deliberately the *names*, not their values: a search result that shows
/// `0.25rem` invites copying the number instead of the token, which is the one
/// habit the source gates exist to break.
pub(crate) const ENTRIES: &[Entry] = &[
    Entry {
        name: "--noctua-space-base",
        page: "spacing.html",
    },
    Entry {
        name: "--noctua-space-tight",
        page: "spacing.html",
    },
    Entry {
        name: "--noctua-space-inset",
        page: "spacing.html",
    },
    Entry {
        name: "--noctua-space-gap",
        page: "spacing.html",
    },
    Entry {
        name: "--noctua-space-paragraph",
        page: "spacing.html",
    },
    Entry {
        name: "--noctua-space-heading",
        page: "spacing.html",
    },
    Entry {
        name: "--noctua-space-section",
        page: "spacing.html",
    },
    Entry {
        name: "--noctua-space-region",
        page: "spacing.html",
    },
    Entry {
        name: "--noctua-measure",
        page: "typography.html",
    },
    Entry {
        name: "--noctua-measure-lead",
        page: "typography.html",
    },
    Entry {
        name: "--noctua-size-body",
        page: "typography.html",
    },
    Entry {
        name: "--noctua-size-h1",
        page: "typography.html",
    },
    Entry {
        name: "--noctua-size-display",
        page: "typography.html",
    },
    Entry {
        name: "--noctua-leading-body",
        page: "typography.html",
    },
    Entry {
        name: "--noctua-weight-semibold",
        page: "typography.html",
    },
    Entry {
        name: "--noctua-radius-control",
        page: "components.html",
    },
    Entry {
        name: "--noctua-radius-surface",
        page: "components.html",
    },
    Entry {
        name: "--noctua-glass-blur",
        page: "effects.html",
    },
    Entry {
        name: "--noctua-glass-opacity",
        page: "effects.html",
    },
    Entry {
        name: "--noctua-pattern-ink",
        page: "effects.html",
    },
    Entry {
        name: "--noctua-glow-ink",
        page: "effects.html",
    },
    Entry {
        name: "--nc-color-surface",
        page: "colours.html",
    },
    Entry {
        name: "--nc-color-fg",
        page: "colours.html",
    },
    Entry {
        name: "--nc-color-border",
        page: "colours.html",
    },
    Entry {
        name: "--nc-color-accent",
        page: "colours.html",
    },
    Entry {
        name: "--nc-color-on-accent",
        page: "colours.html",
    },
    Entry {
        name: "ThemeScope",
        page: "components.html",
    },
    Entry {
        name: "useTheme",
        page: "components.html",
    },
    Entry {
        name: "data-noctua-scope",
        page: "scoped-themes.html",
    },
    Entry {
        name: "data-palette",
        page: "scoped-themes.html",
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
            page: "components.html",
        });
    }
    for palette in theme::Palette::ALL {
        entries.push(Entry {
            name: palette.as_str(),
            page: "colours.html",
        });
    }
    entries
}

#[cfg(test)]
mod tests {
    use super::*;

    /// A search result that leads nowhere is worse than no result.
    #[test]
    fn every_entry_points_at_a_real_page() {
        let pages: Vec<String> = crate::site::PAGES.iter().map(|page| page.file()).collect();
        for entry in all() {
            assert!(
                pages.iter().any(|page| page == entry.page),
                "{} points at {}, which is not a page",
                entry.name,
                entry.page
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
