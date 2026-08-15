//! The shape of `spec/icons.toml`, and the normalisation of what it points at.
//!
//! # Why an icon is a spec entry and not a file somebody pastes
//!
//! The documentation site shipped a header link whose icon was `<path d="">` —
//! a forty-four pixel clickable void. It compiled because the icon was
//! requested by *name*, as a `String`, against a `match` with a `_ => ""` arm.
//! Nothing anywhere could tell the difference between "this icon" and "no such
//! icon".
//!
//! So the set is declared here, the drawings are vendored beside their
//! licences, and the generated output is an **enum**. `Icon { icon: "repository" }`
//! is now a type error rather than an empty box, and `NOCTUA.md §9.5` asks for
//! exactly that: "`Icon` takes a name from a generated enum/union, so a typo is
//! a compile error."
//!
//! # What normalisation means here
//!
//! An inlined SVG shares a document with everything else on the page. So the
//! wrapper is discarded and rebuilt by the component, and the drawing itself
//! may carry only geometry: no `id` (which would collide the second time the
//! same icon appears), no `class`, no `style`, and no `fill`/`stroke` of its
//! own (which would refuse `currentColor` and paint the icon a fixed color
//! inside a themed subtree — the frozen-color trap in a different costume).

use std::collections::BTreeSet;
use std::path::Path;

use indexmap::IndexMap;
use serde::Deserialize;

use crate::error::{Problem, SpecError};

/// The whole icon set.
#[derive(Debug, Clone, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Icons {
    /// Sizes and stroke weight.
    #[serde(default)]
    pub meta: Meta,

    /// Where the drawings come from, keyed by the name an icon cites.
    pub source: IndexMap<String, Source>,

    /// Every icon, keyed by the name the generated enum will carry.
    pub icon: IndexMap<String, Icon>,
}

/// What every icon shares, whoever drew it.
///
/// # There were three size fields here and they decided nothing
///
/// `size_small`, `size_medium` and `size_large` were written into the generated
/// `IconSize`'s doc comments and read by nothing else. What actually sizes an
/// icon is `.nd-icon-glyph`'s `font-size`, from the type scale, and the two had
/// already drifted: the spec claimed `1.5rem` for the medium box where the
/// stylesheet gives it the body step. A declaration that documents a value
/// nothing uses is worse than a missing one, so the sizes are stated where they
/// are true — `spec/components/icon.toml`'s `size` doc — and this holds only
/// what the drawings genuinely share.
#[derive(Debug, Clone, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Meta {
    /// The stroke weight stroked icons are drawn at.
    pub stroke_width: String,
}

impl Default for Meta {
    fn default() -> Self {
        Self {
            stroke_width: "1.75".to_owned(),
        }
    }
}

/// One upstream project, pinned.
#[derive(Debug, Clone, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Source {
    /// Its name, as its authors write it.
    pub project: String,
    /// Where it lives.
    pub repository: String,
    /// The tag the vendored files were taken at.
    pub tag: String,
    /// The licence, by its short identifier.
    pub licence: String,
    /// The licence text, vendored beside the drawings.
    pub licence_file: String,
}

/// One icon.
#[derive(Debug, Clone, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Icon {
    /// Which `[source.*]` it came from.
    pub source: String,
    /// Its file under `assets/icons/`, named as upstream names it.
    pub file: String,
    /// How it is painted.
    #[serde(default)]
    pub paint: Paint,
    /// What it is for, in English — a developer-facing note about an API.
    pub doc: String,
}

/// How an icon is painted.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Paint {
    /// Outlined, `fill: none` and `stroke: currentColor`. Every Lucide icon.
    #[default]
    Stroke,
    /// Solid, `fill: currentColor` and no stroke. Brand marks.
    Fill,
}

/// An icon with its drawing read, normalised and checked.
#[derive(Debug, Clone)]
pub struct Resolved {
    /// The name the generated enum carries, in `kebab-case`.
    pub name: String,
    /// Its `PascalCase` spelling, for the Rust variant.
    pub variant: String,
    /// The upstream project's name, for the attribution.
    pub project: String,
    /// Its `viewBox`, taken from the drawing rather than assumed.
    pub view_box: String,
    /// How it is painted.
    pub paint: Paint,
    /// The drawing: the children of `<svg>`, whitespace collapsed.
    pub body: String,
    /// What it is for.
    pub doc: String,
}

/// The elements an inlined drawing may use.
const ALLOWED: [&str; 7] = [
    "path", "circle", "rect", "line", "polyline", "polygon", "ellipse",
];

/// The attributes those elements may carry: geometry, and nothing else.
///
/// `width` and `height` are here because a `<rect>` is drawn with them. On the
/// `<svg>` wrapper they would be a fixed size overriding the component's, but
/// the wrapper is stripped before this runs.
const GEOMETRY: [&str; 16] = [
    "d",
    "cx",
    "cy",
    "r",
    "rx",
    "ry",
    "x",
    "y",
    "x1",
    "y1",
    "x2",
    "y2",
    "width",
    "height",
    "points",
    "transform",
];

/// Attributes that must not survive into an inlined drawing, and why.
const FORBIDDEN: [(&str, &str); 8] = [
    (
        "id",
        "an id collides the second time the same icon is on the page",
    ),
    (
        "class",
        "a class in a drawing is styling that escapes the component",
    ),
    ("style", "the same, and it wins over the stylesheet"),
    (
        "fill",
        "a fixed fill refuses `currentColor` and ignores the theme",
    ),
    ("stroke", "the same, for the outline"),
    ("mask", "an inlined mask needs an id, and ids collide"),
    ("clip-path", "the same"),
    (
        "filter",
        "a filter on an ancestor stops `backdrop-filter` sampling",
    ),
];

/// Parses `spec/icons.toml` and reads every drawing it points at.
///
/// `directory` is where the vendored SVGs live.
///
/// # Errors
///
/// Every problem at once: a bad table, a missing drawing, an unused drawing, a
/// drawing that uses something an inlined icon may not.
pub fn parse(
    path: &str,
    text: &str,
    directory: &Path,
) -> Result<(Icons, Vec<Resolved>), SpecError> {
    let icons: Icons = toml::from_str(text).map_err(|error| {
        SpecError::new(
            path,
            text.to_owned(),
            vec![Problem::whole_file(error.message().to_owned()).fix(
                "Every icon is `[icon.<name>]` with `source`, `file` and `doc`, and every \
                 source it cites is a `[source.<name>]` table.",
            )],
        )
    })?;

    let mut problems = Vec::new();
    let mut resolved = Vec::new();
    let mut used: BTreeSet<String> = BTreeSet::new();

    for (name, icon) in &icons.icon {
        if !name
            .chars()
            .all(|c| c.is_ascii_lowercase() || c.is_ascii_digit() || c == '-')
        {
            problems.push(
                Problem::whole_file(format!("`{name}` is not kebab-case")).fix(
                    "An icon name becomes an enum variant in Rust and a union member in \
                     TypeScript. Lowercase, digits and hyphens.",
                ),
            );
            continue;
        }

        let Some(source) = icons.source.get(&icon.source) else {
            problems.push(
                Problem::whole_file(format!(
                    "`{name}` cites the source `{}`, which is not declared",
                    icon.source
                ))
                .fix("Add a `[source.…]` table for it, with its tag and its licence."),
            );
            continue;
        };

        let file = directory.join(&icon.file);
        let Ok(svg) = std::fs::read_to_string(&file) else {
            problems.push(
                Problem::whole_file(format!(
                    "`{name}` names {}, which is not there",
                    file.display()
                ))
                .fix(format!(
                    "Fetch it from {} at {} and vendor it there, byte for byte.",
                    source.repository, source.tag
                )),
            );
            continue;
        };
        used.insert(icon.file.clone());

        match normalise(&svg, icon.paint) {
            Ok((view_box, body)) => resolved.push(Resolved {
                name: name.clone(),
                variant: pascal(name),
                project: source.project.clone(),
                view_box,
                paint: icon.paint,
                body,
                doc: icon.doc.clone(),
            }),
            Err(reason) => problems.push(
                Problem::whole_file(format!("`{name}` cannot be inlined: {reason}")).fix(
                    "Vendor the upstream drawing unmodified; if upstream itself carries \
                     the offending attribute, the icon is not one this system can inline.",
                ),
            ),
        }
    }

    // A drawing nothing declares is payload, and `NOCTUA.md §9.5` says so.
    if let Ok(entries) = std::fs::read_dir(directory) {
        for entry in entries.flatten() {
            let is_svg = entry
                .path()
                .extension()
                .is_some_and(|e| e.eq_ignore_ascii_case("svg"));
            let file = entry.file_name().to_string_lossy().into_owned();
            if is_svg && !used.contains(&file) {
                problems.push(
                    Problem::whole_file(format!("{file} is vendored but no icon declares it")).fix(
                        "Declare it in spec/icons.toml, or delete it. An unused icon is \
                         payload and review surface.",
                    ),
                );
            }
        }
    }

    if problems.is_empty() {
        Ok((icons, resolved))
    } else {
        Err(SpecError::new(path, text.to_owned(), problems))
    }
}

/// Strips the wrapper and checks what is left.
///
/// Returns the `viewBox` and the drawing, whitespace collapsed.
fn normalise(svg: &str, paint: Paint) -> Result<(String, String), String> {
    let open_end = svg.find('>').ok_or("there is no `<svg>` element")?;
    let open = &svg[..=open_end];
    if !open.starts_with("<svg") {
        return Err("the file does not begin with `<svg`".to_owned());
    }
    let view_box = attribute(open, "viewBox").ok_or("the `<svg>` declares no viewBox")?;

    let close = svg.rfind("</svg>").ok_or("the `<svg>` is not closed")?;
    let inner = &svg[open_end + 1..close];

    let mut body = String::with_capacity(inner.len());
    let mut in_tag = false;
    let mut last_space = false;
    for ch in inner.chars() {
        match ch {
            '<' => {
                in_tag = true;
                body.push(ch);
                last_space = false;
            }
            '>' => {
                in_tag = false;
                body.push(ch);
                last_space = false;
            }
            c if c.is_whitespace() => {
                // Collapsed everywhere, and dropped entirely between elements:
                // whitespace inside a `d` is meaningful, whitespace around a
                // `<path>` is indentation.
                if in_tag && !last_space {
                    body.push(' ');
                    last_space = true;
                }
            }
            c => {
                body.push(c);
                last_space = false;
            }
        }
    }
    let body = body.replace(" />", "/>").trim().to_owned();

    for element in elements(&body) {
        if !ALLOWED.contains(&element.as_str()) {
            return Err(format!(
                "it uses `<{element}>`, which is not one of {}",
                ALLOWED.join(", ")
            ));
        }
    }
    for (name, why) in FORBIDDEN {
        // A filled brand mark's `fill` lives on the wrapper, which is gone; a
        // `fill` on the drawing itself is still refused for both kinds.
        if body.contains(&format!("{name}=\"")) {
            return Err(format!("it carries `{name}`, and {why}"));
        }
    }
    for attribute in attributes(&body) {
        if !GEOMETRY.contains(&attribute.as_str()) {
            return Err(format!(
                "it carries `{attribute}`, which is not geometry — only {} may survive",
                GEOMETRY.join(", ")
            ));
        }
    }
    if body.is_empty() {
        return Err("it draws nothing".to_owned());
    }
    // The failure that shipped, asserted here so it cannot reach a page.
    if body.contains("d=\"\"") {
        return Err("it contains an empty path".to_owned());
    }
    let _ = paint;

    Ok((view_box, body))
}

/// The value of an attribute on an opening tag.
fn attribute(tag: &str, name: &str) -> Option<String> {
    let at = tag.find(&format!("{name}=\""))? + name.len() + 2;
    Some(tag[at..].split('"').next()?.to_owned())
}

/// Every element name in a drawing.
fn elements(body: &str) -> Vec<String> {
    body.split('<')
        .skip(1)
        .filter(|part| !part.starts_with('/'))
        .filter_map(|part| part.split([' ', '>', '/']).next())
        .map(str::to_owned)
        .filter(|name| !name.is_empty())
        .collect()
}

/// Every attribute name in a drawing.
///
/// Quote-aware, because a `d` is full of spaces: splitting a tag on whitespace
/// turns `d="M4 6h16"` into two "attributes", one of which is `6h16"/`.
fn attributes(body: &str) -> Vec<String> {
    let mut names = Vec::new();
    for tag in body.split('<').skip(1) {
        let Some(end) = tag.find('>') else { continue };
        let mut rest = tag[..end].trim_start();
        // Past the element name.
        rest = rest.split_once(char::is_whitespace).map_or("", |(_, r)| r);
        while let Some(equals) = rest.find('=') {
            let name = rest[..equals].trim();
            if !name.is_empty() {
                names.push(name.to_owned());
            }
            let after = &rest[equals + 1..];
            let Some(open) = after.find('"') else { break };
            let Some(close) = after[open + 1..].find('"') else {
                break;
            };
            rest = &after[open + close + 2..];
        }
    }
    names
}

/// `chevron-down` → `ChevronDown`.
#[must_use]
pub fn pascal(name: &str) -> String {
    name.split('-')
        .filter(|part| !part.is_empty())
        .map(|part| {
            let mut chars = part.chars();
            chars.next().map_or_else(String::new, |first| {
                first.to_uppercase().collect::<String>() + chars.as_str()
            })
        })
        .collect()
}

/// Loads `spec/icons.toml` and the drawings beside it.
///
/// # Errors
///
/// A missing file, or anything [`parse`] refuses.
pub fn load(
    path: impl AsRef<Path>,
    directory: impl AsRef<Path>,
) -> Result<(Icons, Vec<Resolved>), SpecError> {
    let path = path.as_ref();
    let text = std::fs::read_to_string(path).map_err(|error| {
        SpecError::new(
            &path.display().to_string(),
            String::new(),
            vec![
                Problem::whole_file(format!("could not read {}: {error}", path.display()))
                    .fix("Create spec/icons.toml. See NOCTUA.md §9.5."),
            ],
        )
    })?;
    parse(&path.display().to_string(), &text, directory.as_ref())
}

#[cfg(test)]
mod tests {
    use super::*;

    const LUCIDE: &str = r#"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24"
        viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
        <line x1="4" x2="20" y1="12" y2="12" />
        <path d="M4 6h16" />
      </svg>"#;

    #[test]
    fn the_wrapper_is_stripped_and_the_drawing_survives() {
        let (view_box, body) = normalise(LUCIDE, Paint::Stroke).expect("valid");
        assert_eq!(view_box, "0 0 24 24");
        assert_eq!(
            body,
            r#"<line x1="4" x2="20" y1="12" y2="12"/><path d="M4 6h16"/>"#
        );
        // The wrapper's own fill and stroke are gone with it: the component
        // writes those, so an icon inside a themed scope paints the scope's
        // color rather than a color baked in upstream.
        assert!(!body.contains("fill="));
        assert!(!body.contains("stroke="));
    }

    #[test]
    fn an_id_is_refused() {
        let svg = r#"<svg viewBox="0 0 24 24"><path id="a" d="M0 0h1"/></svg>"#;
        let error = normalise(svg, Paint::Stroke).unwrap_err();
        assert!(error.contains("id"), "{error}");
    }

    #[test]
    fn a_fixed_color_is_refused() {
        let svg = r##"<svg viewBox="0 0 24 24"><path fill="#f00" d="M0 0h1"/></svg>"##;
        assert!(normalise(svg, Paint::Fill).is_err());
    }

    #[test]
    fn a_foreign_element_is_refused() {
        let svg = r##"<svg viewBox="0 0 24 24"><use href="#x"/></svg>"##;
        let error = normalise(svg, Paint::Stroke).unwrap_err();
        assert!(error.contains("use"), "{error}");
    }

    /// The defect that shipped: a request for an icon that did not exist fell
    /// through to `_ => ""` and rendered a 44×44 clickable void.
    #[test]
    fn an_empty_path_is_refused() {
        let svg = r#"<svg viewBox="0 0 24 24"><path d=""/></svg>"#;
        let error = normalise(svg, Paint::Stroke).unwrap_err();
        assert!(error.contains("empty path"), "{error}");
    }

    #[test]
    fn a_drawing_with_no_view_box_is_refused() {
        let svg = r#"<svg width="24"><path d="M0 0h1"/></svg>"#;
        assert!(normalise(svg, Paint::Stroke).is_err());
    }

    /// The set as it ships must validate, and its failure must be readable.
    #[test]
    fn the_shipped_set_validates() {
        let root = concat!(env!("CARGO_MANIFEST_DIR"), "/../..");
        match load(
            format!("{root}/spec/icons.toml"),
            format!("{root}/assets/icons"),
        ) {
            Ok((_, icons)) => assert!(!icons.is_empty(), "the set is empty"),
            Err(error) => panic!("{error:?}"),
        }
    }

    #[test]
    fn names_become_variants() {
        assert_eq!(pascal("chevron-down"), "ChevronDown");
        assert_eq!(pascal("menu"), "Menu");
        assert_eq!(pascal("external-link"), "ExternalLink");
    }
}
