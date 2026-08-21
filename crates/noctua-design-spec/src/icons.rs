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
//! licences, and the generated output is one **constant per icon**. A name that
//! does not exist is a compile error rather than an empty box, and
//! `NOCTUA.md §9.5` asks for exactly that.
//!
//! # A source may be swept
//!
//! The whole Lucide set is 1,776 drawings, and 1,776 hand-written tables would
//! be a list nobody reads and nobody keeps. A `[source.*]` with `sweep = true`
//! turns every drawing in its directory into an icon named after its file. What
//! stays a table is what a filename cannot say: a deliberate rename, a
//! hand-written `doc`, or `paint = "fill"`.
//!
//! **A table claims its file.** `[icon.close]` with `file = "x.svg"` produces
//! `close` and no `x`, so a rename cannot leave two names for one drawing.
//!
//! # What normalisation means here
//!
//! An inlined SVG shares a document with everything else on the page. So the
//! wrapper is discarded and rebuilt by the component, and the drawing itself
//! may carry only geometry: no `id` (which would collide the second time the
//! same icon appears), no `class`, no `style`, and no `fill`/`stroke` naming a
//! color of its own — which would refuse `currentColor` and paint the icon a
//! fixed color inside a themed subtree, the frozen-color trap in a different
//! costume. `currentColor` and `none` are the two values that are not that, and
//! they are the two this accepts: ten Lucide drawings fill a half-unit dot with
//! `currentColor`, which follows the theme rather than escaping it.

use std::collections::{BTreeMap, BTreeSet};
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

    /// Every icon a table declares, keyed by the name it will carry.
    ///
    /// Not every icon: a swept source contributes the rest.
    #[serde(default)]
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
    /// Its directory under `assets/icons/`.
    pub directory: String,
    /// Whether every drawing in that directory is an icon.
    ///
    /// `false` keeps the original rule in both directions: a drawing no table
    /// declares fails the build.
    #[serde(default)]
    pub sweep: bool,
    /// The commands that produced the directory, for the attribution.
    ///
    /// Vendoring is a person running these, not a build step — the build reads
    /// what is on disk and needs no network. Recording them here is what makes
    /// the result reproducible by somebody who was not there.
    pub fetch: String,
}

/// One icon a table declares.
#[derive(Debug, Clone, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Icon {
    /// Which `[source.*]` it came from.
    pub source: String,
    /// Its file in that source's directory. Defaults to `<name>.svg`.
    #[serde(default)]
    pub file: Option<String>,
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
    /// The name the generated constant carries, in `kebab-case`.
    pub name: String,
    /// Its `SCREAMING_SNAKE_CASE` spelling, for the Rust constant.
    pub constant: String,
    /// Its `camelCase` spelling, for the JavaScript export.
    pub export: String,
    /// The upstream project's name, for the attribution.
    pub project: String,
    /// The `[source.*]` key it came from.
    pub source: String,
    /// Its file in that source's directory.
    pub file: String,
    /// Its `viewBox`, taken from the drawing rather than assumed.
    pub view_box: String,
    /// How it is painted.
    pub paint: Paint,
    /// The drawing: the children of `<svg>`, whitespace collapsed.
    pub body: String,
    /// What it is for.
    pub doc: String,
    /// Whether a table declared it, rather than a sweep finding it.
    pub curated: bool,
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

/// The two paint attributes, and the only two values either may take.
///
/// `currentColor` is not a color the drawing chose; it is the drawing refusing
/// to choose, which is the whole property this rule protects. `none` is the
/// same refusal for the other half. Anything else — a hex, an `oklch()`, a
/// `var()`, a keyword — is a color frozen into a drawing that will then ignore
/// the theme it is rendered inside.
const PAINT: [&str; 2] = ["fill", "stroke"];
/// What a paint attribute may say.
const PAINTABLE: [&str; 2] = ["currentColor", "none"];

/// Attributes that must not survive into an inlined drawing, and why.
const FORBIDDEN: [(&str, &str); 6] = [
    (
        "id",
        "an id collides the second time the same icon is on the page",
    ),
    (
        "class",
        "a class in a drawing is styling that escapes the component",
    ),
    ("style", "the same, and it wins over the stylesheet"),
    ("mask", "an inlined mask needs an id, and ids collide"),
    ("clip-path", "the same"),
    (
        "filter",
        "a filter on an ancestor stops `backdrop-filter` sampling",
    ),
];

/// The name a file may have: what becomes an identifier in two languages.
fn is_vendorable(file: &str) -> bool {
    let Some(stem) = file.strip_suffix(".svg") else {
        return false;
    };
    !stem.is_empty()
        && stem.starts_with(|c: char| c.is_ascii_lowercase() || c.is_ascii_digit())
        && stem
            .chars()
            .all(|c| c.is_ascii_lowercase() || c.is_ascii_digit() || c == '-')
}

/// Reads one source's directory and verifies it against its `SHA256SUMS`.
///
/// Returns the drawings it holds, sorted, and everything wrong with them.
fn vendored(directory: &Path, key: &str) -> (Vec<String>, Vec<Problem>) {
    let mut drawings: Vec<String> = Vec::new();
    let mut problems = Vec::new();
    let mut sums: BTreeMap<String, String> = BTreeMap::new();

    let Ok(entries) = std::fs::read_dir(directory) else {
        problems.push(
            Problem::whole_file(format!(
                "`{key}` names {}, which is not a directory",
                directory.display()
            ))
            .fix("Create it and vendor the drawings there, beside their licence."),
        );
        return (drawings, problems);
    };

    for entry in entries.flatten() {
        let file = entry.file_name().to_string_lossy().into_owned();
        if !file.to_ascii_lowercase().ends_with(".svg") {
            continue;
        }
        if is_vendorable(&file) {
            drawings.push(file);
        } else {
            problems.push(
                Problem::whole_file(format!("`{key}` holds {file}, which cannot be a name")).fix(
                    "A drawing's file name becomes an identifier in Rust and in \
                     JavaScript: lowercase letters, digits and hyphens, starting with a \
                     letter or a digit.",
                ),
            );
        }
    }
    drawings.sort();

    // THE CHECKSUMS ARE VERIFIED, NEVER REGENERATED HERE. A build that rewrote
    // `SHA256SUMS` from what it found would verify nothing at all: the file is
    // recorded once, by the person who vendored the drawings, and every build
    // afterwards asks whether the bytes on disk are still those.
    let list = directory.join("SHA256SUMS");
    match std::fs::read_to_string(&list) {
        Ok(text) => {
            for line in text.lines().filter(|line| !line.trim().is_empty()) {
                if let Some((digest, file)) = line.split_once("  ") {
                    sums.insert(file.trim().to_owned(), digest.trim().to_owned());
                }
            }
        }
        Err(error) => problems.push(
            Problem::whole_file(format!("could not read {}: {error}", list.display())).fix(
                "Record it where the drawings were vendored: \
                 `( cd <directory> && sha256sum *.svg > SHA256SUMS )`.",
            ),
        ),
    }

    for file in &drawings {
        let Some(expected) = sums.get(file) else {
            problems.push(
                Problem::whole_file(format!(
                    "{file} is vendored but SHA256SUMS does not list it"
                ))
                .fix("Re-record the checksums, deliberately, after re-vendoring."),
            );
            continue;
        };
        let Ok(bytes) = std::fs::read(directory.join(file)) else {
            continue;
        };
        let actual = crate::sha256::hex(&bytes);
        if &actual != expected {
            problems.push(
                Problem::whole_file(format!(
                    "{file} does not match its recorded checksum ({expected} != {actual})"
                ))
                .fix(
                    "A vendored drawing is upstream's bytes. Restore it from the pinned \
                     tag, or re-vendor and re-record the whole directory.",
                ),
            );
        }
    }
    for file in sums.keys() {
        if !drawings.contains(file) {
            problems.push(
                Problem::whole_file(format!("SHA256SUMS lists {file}, which is not there"))
                    .fix("Restore the drawing from the pinned tag, or re-record the checksums."),
            );
        }
    }

    (drawings, problems)
}

/// Parses `spec/icons.toml` and reads every drawing it points at.
///
/// `directory` is where the vendored source directories live.
///
/// # Errors
///
/// Every problem at once: a bad table, a missing drawing, an undeclared
/// drawing, a checksum that does not match, a drawing that uses something an
/// inlined icon may not.
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
                "Every source is a `[source.<name>]` table with a `directory`, a tag, a \
                 licence and the commands that vendored it. A `[icon.<name>]` table is \
                 only needed where a filename cannot say it: a rename, a written `doc`, \
                 or `paint`.",
            )],
        )
    })?;

    let mut problems = Vec::new();
    let mut resolved: Vec<Resolved> = Vec::new();

    // Read every source directory once, verifying its checksums as it goes.
    let mut listings: IndexMap<&str, Vec<String>> = IndexMap::new();
    for (key, source) in &icons.source {
        let (drawings, mut found) = vendored(&directory.join(&source.directory), key);
        problems.append(&mut found);
        listings.insert(key.as_str(), drawings);
    }

    // A table claims its file, so the sweep cannot produce a second name for
    // the same drawing. Both halves add to the same two sets, which is why they
    // are threaded through rather than owned by either.
    let mut claimed: BTreeSet<(String, String)> = BTreeSet::new();
    let mut named: BTreeSet<String> = BTreeSet::new();

    declared(
        &icons,
        directory,
        &mut claimed,
        &mut named,
        &mut resolved,
        &mut problems,
    );
    swept(
        &icons,
        directory,
        &listings,
        &claimed,
        &mut named,
        &mut resolved,
        &mut problems,
    );

    // ONE ORDER, AND IT IS THE READER'S. The set was in the order the spec
    // declared it, which was readable at forty and is meaningless at a
    // thousand: the gallery, the manifest, the attribution and `ALL` are all
    // alphabetical now, so a person looking for one icon looks in one place.
    resolved.sort_by(|a, b| a.name.cmp(&b.name));

    // TWO NAMES, TWO IDENTIFIERS, AND A THIRD WAY TO COLLIDE. Names are
    // distinct by construction — a file name is a path and a table key is a TOML
    // key — but the spellings derived from them are not: `columns-2` and
    // `columns_2` would be one Rust constant, and a reserved word taking an
    // `Icon` suffix could land on a drawing genuinely called that. Either would
    // shadow the other in a generated file that still compiles.
    collisions(&resolved, &mut problems);

    if problems.is_empty() {
        Ok((icons, resolved))
    } else {
        Err(SpecError::new(path, text.to_owned(), problems))
    }
}

/// Two icons that would generate one identifier.
///
/// A generated constant or export written twice is not a compile error in
/// either language: Rust reports a duplicate definition, but JavaScript takes
/// the second module quietly, and the two spellings are derived rather than
/// declared — so the collision is between things nobody wrote down.
fn collisions(resolved: &[Resolved], problems: &mut Vec<Problem>) {
    let mut constants: BTreeMap<&str, &str> = BTreeMap::new();
    let mut exports: BTreeMap<&str, &str> = BTreeMap::new();
    for icon in resolved {
        for (spelling, taken, language) in [
            (icon.constant.as_str(), &mut constants, "Rust constant"),
            (icon.export.as_str(), &mut exports, "JavaScript export"),
        ] {
            if let Some(other) = taken.insert(spelling, icon.name.as_str()) {
                problems.push(
                    Problem::whole_file(format!(
                        "`{}` and `{other}` are both the {language} `{spelling}`",
                        icon.name
                    ))
                    .fix("Rename one of them with an `[icon.<name>]` table."),
                );
            }
        }
    }
}

/// The icons a `[icon.<name>]` table declares: a rename, a written `doc`, or a
/// `paint` — everything a filename cannot say on its own.
fn declared(
    icons: &Icons,
    directory: &Path,
    claimed: &mut BTreeSet<(String, String)>,
    named: &mut BTreeSet<String>,
    resolved: &mut Vec<Resolved>,
    problems: &mut Vec<Problem>,
) {
    for (name, icon) in &icons.icon {
        if !is_vendorable(&format!("{name}.svg")) {
            problems.push(
                Problem::whole_file(format!("`{name}` is not kebab-case")).fix(
                    "An icon name becomes a constant in Rust and an export in \
                     JavaScript. Lowercase, digits and hyphens.",
                ),
            );
            continue;
        }
        if !named.insert(name.clone()) {
            problems.push(
                Problem::whole_file(format!("`{name}` is declared twice")).fix(
                    "One table per icon. Two tables of one name is a drawing chosen at random.",
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
        let file = icon.file.clone().unwrap_or_else(|| format!("{name}.svg"));
        let full = directory.join(&source.directory).join(&file);

        if !claimed.insert((icon.source.clone(), file.clone())) {
            problems.push(
                Problem::whole_file(format!("`{name}` claims {file}, which another icon claims"))
                    .fix(
                        "One drawing, one name. Two names for one drawing is two things to \
                         keep in step for a difference nobody can see.",
                    ),
            );
            continue;
        }

        let Ok(svg) = std::fs::read_to_string(&full) else {
            problems.push(
                Problem::whole_file(format!(
                    "`{name}` names {}, which is not there",
                    full.display()
                ))
                .fix(format!(
                    "Fetch it from {} at {} and vendor it there, byte for byte.",
                    source.repository, source.tag
                )),
            );
            continue;
        };

        match normalise(&svg) {
            Ok((view_box, body)) => resolved.push(Resolved {
                name: name.clone(),
                constant: screaming(name),
                export: camel(name),
                project: source.project.clone(),
                source: icon.source.clone(),
                file,
                view_box,
                paint: icon.paint,
                body,
                doc: icon.doc.trim().to_owned(),
                curated: true,
            }),
            Err(reason) => problems.push(
                Problem::whole_file(format!("`{name}` cannot be inlined: {reason}")).fix(
                    "Vendor the upstream drawing unmodified; if upstream itself carries \
                     the offending attribute, the icon is not one this system can inline.",
                ),
            ),
        }
    }
}

/// Everything else in a swept source: one icon per drawing, named after its
/// file. A source that is not swept keeps the original rule instead.
fn swept(
    icons: &Icons,
    directory: &Path,
    listings: &IndexMap<&str, Vec<String>>,
    claimed: &BTreeSet<(String, String)>,
    named: &mut BTreeSet<String>,
    resolved: &mut Vec<Resolved>,
    problems: &mut Vec<Problem>,
) {
    for (key, source) in &icons.source {
        let Some(drawings) = listings.get(key.as_str()) else {
            continue;
        };
        for file in drawings {
            if claimed.contains(&(key.clone(), file.clone())) {
                continue;
            }
            // A source that is not swept keeps the original rule: a drawing
            // nothing declares is payload, and `NOCTUA.md §9.5` says so.
            if !source.sweep {
                problems.push(
                    Problem::whole_file(format!(
                        "{}/{file} is vendored but no icon declares it",
                        source.directory
                    ))
                    .fix(
                        "Declare it in spec/icons.toml, delete it, or mark its source \
                         `sweep = true`.",
                    ),
                );
                continue;
            }

            let name = file.trim_end_matches(".svg").to_owned();
            if !named.insert(name.clone()) {
                problems.push(
                    Problem::whole_file(format!(
                        "{}/{file} would be `{name}`, which a table already declares",
                        source.directory
                    ))
                    .fix(
                        "Rename the table's icon, or point it at this file so it claims \
                         it.",
                    ),
                );
                continue;
            }

            let Ok(svg) = std::fs::read_to_string(directory.join(&source.directory).join(file))
            else {
                continue;
            };
            match normalise(&svg) {
                Ok((view_box, body)) => resolved.push(Resolved {
                    constant: screaming(&name),
                    export: camel(&name),
                    project: source.project.clone(),
                    source: key.clone(),
                    file: file.clone(),
                    view_box,
                    paint: Paint::Stroke,
                    body,
                    doc: format!("`{name}`, from {}.", source.project),
                    curated: false,
                    name,
                }),
                Err(reason) => problems.push(
                    Problem::whole_file(format!(
                        "{}/{file} cannot be inlined: {reason}",
                        source.directory
                    ))
                    .fix(
                        "Vendor the upstream drawing unmodified; if upstream itself \
                         carries the offending attribute, the icon is not one this system \
                         can inline.",
                    ),
                ),
            }
        }
    }
}

/// Strips the wrapper and checks what is left.
///
/// Returns the `viewBox` and the drawing, whitespace collapsed.
fn normalise(svg: &str) -> Result<(String, String), String> {
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
        if body.contains(&format!("{name}=\"")) {
            return Err(format!("it carries `{name}`, and {why}"));
        }
    }
    for (attribute, value) in attributes(&body) {
        if PAINT.contains(&attribute.as_str()) {
            if !PAINTABLE.contains(&value.as_str()) {
                return Err(format!(
                    "it paints with `{attribute}=\"{value}\"`, and a color frozen into a \
                     drawing ignores the theme it is rendered inside — only {} may appear",
                    PAINTABLE.join(" or ")
                ));
            }
        } else if !GEOMETRY.contains(&attribute.as_str()) {
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

/// Every attribute in a drawing, with its value.
///
/// Quote-aware, because a `d` is full of spaces: splitting a tag on whitespace
/// turns `d="M4 6h16"` into two "attributes", one of which is `6h16"/`.
fn attributes(body: &str) -> Vec<(String, String)> {
    let mut found = Vec::new();
    for tag in body.split('<').skip(1) {
        let Some(end) = tag.find('>') else { continue };
        let mut rest = tag[..end].trim_start();
        // Past the element name.
        rest = rest.split_once(char::is_whitespace).map_or("", |(_, r)| r);
        while let Some(equals) = rest.find('=') {
            let name = rest[..equals].trim();
            let after = &rest[equals + 1..];
            let Some(open) = after.find('"') else { break };
            let Some(close) = after[open + 1..].find('"') else {
                break;
            };
            if !name.is_empty() {
                found.push((
                    name.to_owned(),
                    after[open + 1..open + 1 + close].to_owned(),
                ));
            }
            rest = &after[open + close + 2..];
        }
    }
    found
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

/// `chevron-down` → `CHEVRON_DOWN`, the Rust constant.
#[must_use]
pub fn screaming(name: &str) -> String {
    name.to_ascii_uppercase().replace('-', "_")
}

/// Words JavaScript will not let a `const` be called.
///
/// Three drawings land on one — `delete`, `import` and `package` — and
/// `export const delete = …` is a syntax error, not a lint. The whole list
/// rather than those three: the next tag Lucide publishes may add a `class` or
/// a `default`, and a check that knows only about today's collisions finds out
/// about tomorrow's from a bundler.
const RESERVED: [&str; 39] = [
    "await",
    "break",
    "case",
    "catch",
    "class",
    "const",
    "continue",
    "debugger",
    "default",
    "delete",
    "do",
    "else",
    "enum",
    "export",
    "extends",
    "false",
    "finally",
    "for",
    "function",
    "if",
    "implements",
    "import",
    "in",
    "instanceof",
    "interface",
    "let",
    "new",
    "null",
    "package",
    "private",
    "protected",
    "public",
    "return",
    "static",
    "super",
    "switch",
    "this",
    "throw",
    "true",
];

/// `chevron-down` → `chevronDown`, the JavaScript export.
///
/// A name that is a reserved word takes an `Icon` suffix — `delete` becomes
/// `deleteIcon` — because the alternative is a module that does not parse. The
/// icon's own name is untouched: the module is still `icons/delete.js` and
/// `data-nd-icon` still says `delete`, so only the binding moves.
#[must_use]
pub fn camel(name: &str) -> String {
    let mut parts = name.split('-').filter(|part| !part.is_empty());
    let first = parts.next().unwrap_or_default().to_owned();
    let export = first + &parts.map(pascal).collect::<String>();
    if RESERVED.contains(&export.as_str()) {
        export + "Icon"
    } else {
        export
    }
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
        let (view_box, body) = normalise(LUCIDE).expect("valid");
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
        let error = normalise(svg).unwrap_err();
        assert!(error.contains("id"), "{error}");
    }

    /// The ten Lucide drawings that fill a half-unit dot. `currentColor` is the
    /// drawing declining to choose a color, which is the property the rule
    /// exists to protect rather than something it should refuse.
    #[test]
    fn current_color_is_allowed_on_a_drawing() {
        let svg = r#"<svg viewBox="0 0 24 24"><circle cx="13.5" cy="6.5" r=".5" fill="currentColor"/></svg>"#;
        let (_, body) = normalise(svg).expect("currentColor follows the theme");
        assert!(body.contains("fill=\"currentColor\""));
    }

    #[test]
    fn a_fixed_color_is_refused() {
        for value in ["#f00", "red", "oklch(50% 0 0)", "var(--nc-color-accent)"] {
            let svg =
                format!(r#"<svg viewBox="0 0 24 24"><path fill="{value}" d="M0 0h1"/></svg>"#);
            let error = normalise(&svg).unwrap_err();
            assert!(error.contains("frozen into a drawing"), "{value}: {error}");
        }
    }

    #[test]
    fn a_foreign_element_is_refused() {
        let svg = r##"<svg viewBox="0 0 24 24"><use href="#x"/></svg>"##;
        let error = normalise(svg).unwrap_err();
        assert!(error.contains("use"), "{error}");
    }

    /// The defect that shipped: a request for an icon that did not exist fell
    /// through to `_ => ""` and rendered a 44×44 clickable void.
    #[test]
    fn an_empty_path_is_refused() {
        let svg = r#"<svg viewBox="0 0 24 24"><path d=""/></svg>"#;
        let error = normalise(svg).unwrap_err();
        assert!(error.contains("empty path"), "{error}");
    }

    #[test]
    fn a_drawing_with_no_view_box_is_refused() {
        let svg = r#"<svg width="24"><path d="M0 0h1"/></svg>"#;
        assert!(normalise(svg).is_err());
    }

    /// A file name is an identifier in two languages before it is anything else.
    #[test]
    fn a_file_name_that_cannot_be_an_identifier_is_refused() {
        assert!(is_vendorable("chevron-down.svg"));
        assert!(is_vendorable("columns-2.svg"));
        assert!(!is_vendorable("Chevron_Down.svg"));
        assert!(!is_vendorable("-leading.svg"));
        assert!(!is_vendorable("../escape.svg"));
        assert!(!is_vendorable(".svg"));
    }

    /// The set as it ships must validate, and its failure must be readable.
    #[test]
    fn the_shipped_set_validates() {
        let root = concat!(env!("CARGO_MANIFEST_DIR"), "/../..");
        match load(
            format!("{root}/spec/icons.toml"),
            format!("{root}/assets/icons"),
        ) {
            Ok((_, icons)) => assert!(icons.len() > 1_000, "the sweep found {}", icons.len()),
            Err(error) => panic!("{error:?}"),
        }
    }

    /// A rename claims its file, so the upstream name is not a second icon.
    #[test]
    fn a_claimed_drawing_is_not_swept_again() {
        let root = concat!(env!("CARGO_MANIFEST_DIR"), "/../..");
        let (_, icons) = load(
            format!("{root}/spec/icons.toml"),
            format!("{root}/assets/icons"),
        )
        .expect("the shipped set");
        assert!(icons.iter().any(|icon| icon.name == "close"));
        assert!(
            !icons.iter().any(|icon| icon.name == "x"),
            "`x.svg` is claimed by `close` and must not also be `x`"
        );
    }

    #[test]
    fn names_become_identifiers() {
        assert_eq!(pascal("chevron-down"), "ChevronDown");
        assert_eq!(screaming("chevron-down"), "CHEVRON_DOWN");
        assert_eq!(screaming("columns-2"), "COLUMNS_2");
        assert_eq!(camel("chevron-down"), "chevronDown");
        assert_eq!(camel("menu"), "menu");
        assert_eq!(camel("a-arrow-down"), "aArrowDown");
        // `export const delete = …` is a syntax error, not a lint.
        assert_eq!(camel("delete"), "deleteIcon");
        assert_eq!(camel("import"), "importIcon");
        assert_eq!(camel("package"), "packageIcon");
    }
}
