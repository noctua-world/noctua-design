//! What search can find beyond the page titles.
//!
//! A documentation search that only indexes page titles answers "where is the
//! color page". It does not answer "what was that spacing token called", which
//! is the question a developer with a half-remembered name actually has — and
//! the one they will otherwise answer by grepping the source.
//!
//! Every entry here is read from the generated crates, so a token added to
//! `spec/tokens.toml` or a palette added upstream in `noctua-colors` becomes
//! searchable on the next build with no edit to this file.

use std::fmt::Write as _;

use noctua_design_i18n::Locale;
use noctua_design_theme as theme;
use noctua_design_tokens as tokens;

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
///
/// **A token with no page is not listed.** A search hit that lands somewhere
/// with no answer on it costs a reader more than no hit at all, so adding a
/// name here means writing the page first. The colour contexts still have no
/// page and are still absent; the spacing scale has one again, and its twenty-one
/// names come back in [`all`] — read from the generated scale rather than typed
/// out, so a step added to `spec/tokens.toml` is searchable with no edit here.
pub(crate) const ENTRIES: &[Entry] = &[
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
        page: site::THEME_SCOPE,
    },
    Entry {
        name: "--noctua-radius-surface",
        page: site::THEME_SCOPE,
    },
    Entry {
        name: "--noctua-glass-blur",
        page: site::THEME_SCOPE,
    },
    Entry {
        name: "--noctua-glass-opacity",
        page: site::THEME_SCOPE,
    },
    Entry {
        name: "--noctua-pattern-ink",
        page: site::THEME_SCOPE,
    },
    Entry {
        name: "ThemeScope",
        page: site::THEME_SCOPE,
    },
    Entry {
        name: "useTheme",
        page: site::THEME_SCOPE,
    },
    Entry {
        name: "data-noctua-scope",
        page: site::THEME_SCOPE,
    },
    Entry {
        name: "data-palette",
        page: site::THEME_SCOPE,
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
            page: site::THEME_SCOPE,
        });
    }
    // The frame's own props, for the page that documents it. Both of the
    // written pages contribute their prop names here, so "mountable" leads
    // somewhere for the same reason "glass" does.
    for prop in crate::props::FRAME {
        entries.push(Entry {
            name: prop.name,
            page: site::EXAMPLE_FRAME,
        });
    }
    // Every component, and every prop it has. A developer with a half-remembered
    // name — `truncate`, `lines`, `ListMarker` — lands on the page that explains
    // it. Generated, so a component added to the spec is searchable with no edit
    // here; the thirty hand-written entries below are the token layer, which has
    // no catalogue of its own.
    for page in site::pages() {
        let Some(component) = page.component else {
            continue;
        };
        entries.push(Entry {
            name: component.name,
            page,
        });
        for prop in component.props {
            entries.push(Entry {
                name: prop.name,
                page,
            });
        }
    }
    // Every name the spacing scale emits — the base, thirteen steps and
    // seven gaps — pointed at the page that explains them. Generated, so this
    // is the list and not a copy of it.
    for name in tokens::space::NAMES {
        entries.push(Entry {
            name,
            page: site::SPACING,
        });
    }
    for palette in theme::Palette::ALL {
        entries.push(Entry {
            name: palette.as_str(),
            page: site::THEME_SCOPE,
        });
    }
    entries
}

/// The index as the search actually reads it: one JSON document per locale.
///
/// # Why this is a file and not markup
///
/// It was markup. Every one of these entries shipped as a hidden `<a>` inside
/// the sidebar of **every page of every locale** — thirty-five kilobytes per
/// document, two hundred and sixty gzipped kilobytes across the site, re-sent
/// on every page load, for a list most readers never open. As a file it is
/// fetched once, on the first search, and cached for the rest of the visit.
///
/// That is also the honest half of `Search`'s contract: an index at a URL is
/// the form that **stops working with the script blocked**, and it says so.
/// The other form — children — is real links that always work, and the site
/// does not use it because thirty-four pages of results do not fit in a
/// two-hundred-and-forty-pixel sidebar.
///
/// # The pages are in it now
///
/// `all()` answers *what was that token called*. On its own it does not answer
/// *where is the typography page*, which the sidebar's own filtering used to —
/// and that filtering is gone. So the pages join the index, with the title the
/// reader sees in their own locale, and the palette answers both questions.
#[must_use]
pub(crate) fn json(locale: Locale) -> String {
    let mut out = String::from("{\n  \"entries\": [\n");
    let mut first = true;
    let mut row = |label: &str, href: &str, group: &str, out: &mut String| {
        if !first {
            out.push_str(",\n");
        }
        first = false;
        let _ = write!(
            out,
            "    {{\"label\": \"{}\", \"href\": \"{}\", \"group\": \"{}\"}}",
            escape(label),
            escape(href),
            escape(group)
        );
    };

    // The pages first, so a tie between a page and a prop of the same name goes
    // to the page. `sort` is stable and the matcher never compares strings, so
    // the order this is written in IS the tie-break.
    for page in site::sorted(locale) {
        row(
            page.title(locale),
            &page.href(),
            site::group_label(page, locale),
            &mut out,
        );
    }
    for entry in all() {
        // A COMPONENT IS ALREADY A PAGE. `all()` pushes every component's own
        // name pointing at its own page, and the loop above pushed every page
        // — so without this every component is in the palette twice, once as
        // *List / Components* and once as *List / List*.
        if entry.name == entry.page.title(locale) {
            continue;
        }
        row(
            entry.name,
            &entry.page.href(),
            entry.page.title(locale),
            &mut out,
        );
    }
    out.push_str("\n  ]\n}\n");
    out
}

/// A JSON string's body. Every name here is an identifier or a translated
/// title, so this is a guard rather than a workload.
fn escape(text: &str) -> String {
    let mut out = String::with_capacity(text.len());
    for c in text.chars() {
        match c {
            '"' => out.push_str("\\\""),
            '\\' => out.push_str("\\\\"),
            '\n' => out.push_str("\\n"),
            other if (other as u32) < 0x20 => {
                let _ = write!(out, "\\u{:04x}", other as u32);
            }
            other => out.push(other),
        }
    }
    out
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
                site::pages().contains(&entry.page),
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

    /// A JSON index that does not parse is a search box that silently finds
    /// nothing — no error in the console, no failure in the build, and a
    /// reader who concludes the documentation has no page about it.
    #[test]
    fn every_locale_emits_an_index_that_parses() {
        for locale in [Locale::En, Locale::PtBr, Locale::Qps] {
            let text = json(locale);
            assert!(text.starts_with("{\n  \"entries\""), "{text:.80}");
            // A hand-rolled parser is not wanted here; what is worth asserting
            // is the shape the reader of this file depends on.
            let rows = text.matches("{\"label\"").count();
            assert!(rows > 300, "{} has only {rows} entries", locale.tag());
            assert_eq!(
                text.matches("},\n").count() + 1,
                rows,
                "{}'s rows are not comma-separated exactly once each",
                locale.tag()
            );
            assert!(
                !text.contains(",\n\n  ]"),
                "a trailing comma in {}",
                locale.tag()
            );
        }
    }

    /// Every page a reader can open is findable by the name they see on it.
    #[test]
    fn every_page_is_in_the_index() {
        let text = json(Locale::En);
        for page in site::pages() {
            let title = page.title(Locale::En);
            assert!(
                text.contains(&format!("\"label\": \"{title}\"")),
                "{title} is not in the index"
            );
        }
    }

    /// A NUMBER FOLLOWED BY A UNIT, NOT THE LETTERS ANYWHERE IN THE NAME.
    ///
    /// This was `contains("rem")`, which is what a value looks like and also
    /// what `remove_label`, `remove_name` and `remember` look like — the first
    /// prop in the repository to contain those three letters failed a test
    /// about the spacing scale. What the test means is that the index lists
    /// names and not measurements, so it asks for a measurement: digits, then
    /// a unit, then the end of a word.
    #[test]
    fn no_entry_carries_a_value() {
        for entry in all() {
            let measurement = entry
                .name
                .split(|c: char| !c.is_ascii_alphanumeric())
                .any(|word| {
                    let unit = word.trim_end_matches(|c: char| c.is_ascii_alphabetic());
                    let tail = &word[unit.len()..];
                    !unit.is_empty()
                        && unit.chars().all(|c| c.is_ascii_digit())
                        && matches!(tail, "rem" | "px" | "em" | "ms")
                });
            assert!(
                !measurement,
                "{} looks like a value; index names, not numbers",
                entry.name
            );
        }
    }
}
