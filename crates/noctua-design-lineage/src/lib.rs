//! A relationship graph, read from text and placed in three dimensions.
//!
//! # Why the graph is text and not markup
//!
//! `Lineage` lays out the whole structure at once — ranks, ordering, routing —
//! so it has to *see* the whole structure. A Dioxus component cannot look
//! inside its own children (`AvatarGroup` says so in its contract, and
//! `NOCTUA.md §9.3` forbids one implementation doing what the other cannot), so
//! a graph handed over as child components could never be laid out. This is the
//! same wall `CodeBlock` hit when it could not number lines it could not read,
//! and it has the same answer: **the structure is a text prop, parsed here.**
//!
//! # Its twin is `layout.js`
//!
//! The layout runs where the component renders: in Rust during pre-rendering,
//! in JavaScript for the React package. So the algorithm exists twice, in one
//! directory, and **neither half is checked against the other** — both are
//! checked against `spec/lineage/cases.toml`. Two halves that agree with each
//! other and disagree with the spec is the failure that actually happens.
//!
//! # Three invariants, borrowed from `scan.rs` because they are the same three
//!
//! 1. **Totality.** Every line either parses or is counted in
//!    [`Scene::skipped`]. No input is rejected, nothing is silently dropped
//!    without being counted, and no unparsable byte can reach a style
//!    attribute.
//! 2. **Termination.** Every loop is bounded by a constant or by the node
//!    count. A cycle an author wrote by accident — X descends from Y descends
//!    from X — is broken deterministically and counted in [`Scene::broken`],
//!    never followed.
//! 3. **Purity.** The output is a function of `(graph, options)` and nothing
//!    else. No clock, no locale, no environment, no randomness — the last one
//!    is not a preference: the React and Dioxus halves must agree to the last
//!    digit, and a randomised sweep would make that impossible to test.
//!
//! # Everything is integer arithmetic
//!
//! Ranks, orders and coordinates are `i32` **half-cells** throughout, converted
//! to `f64` only when they are handed out. Halving an integer is exact in
//! binary, so the two implementations cannot drift apart by an ulp anywhere in
//! the layout. The one exception is [`Segment::basis`], which needs `sqrt` —
//! exactly rounded in both languages — and nothing else transcendental. That is
//! why a taut connection is drawn from a rotation matrix rather than from the
//! `atan2` a reader would expect: `atan2` is *not* required to agree between a
//! Rust target's libm and a JavaScript engine, and a disagreement there would
//! be a sub-pixel difference nobody could reproduce.
//!
//! # And it never sees a length
//!
//! Coordinates are in cells. A cell becomes a length in the stylesheet, from
//! `--noctua-scene-cell`, which is what lets the whole scene rescale at a
//! breakpoint and is also why `gates::token_literal` has nothing to complain
//! about here.

mod draw;
mod layout;
mod parse;

pub use draw::{
    Drawing, DrawnBond, DrawnPiece, Tie, axis_name, draw, nameable, set_parts, shown_name, some,
};
pub use layout::place;
pub use parse::parse;

/// What a piece of the graph is.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Piece {
    /// A figure: a person, an animal, a role, a company — anything with a card.
    #[default]
    Figure,
    /// A mark: a small point *on* the structure rather than a member of it. A
    /// union between partners, a milestone, a note, the switch that swaps one
    /// branch for another. Every hyperedge in this model is a mark: "these two
    /// had these three children" is two bonds in and three bonds out.
    Mark,
}

/// What a bond does to the ranks of the two things it joins.
///
/// The author's own word for the bond is kept in [`Bond::kind`] and reaches the
/// DOM, so a consumer may invent `apprenticeship` and style it. Only these
/// three shapes change where anything is placed.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Shape {
    /// `to` sits one rank below `from`. Descent, adoption, fostering, a
    /// reporting line — the default, and the only shape that ranks anything.
    #[default]
    Descent,
    /// The two sit on the same rank where that is possible. A partnership.
    Union,
    /// Neither is above the other, and the connection is drawn as a straight
    /// run between them. An annotation across the structure.
    Lateral,
}

impl Shape {
    /// The shape a bond's `kind` implies.
    ///
    /// Unknown kinds are descent, because that is the shape of almost every
    /// relationship anybody draws, and because a kind nobody recognised must
    /// still place its endpoints somewhere sensible.
    #[must_use]
    pub fn of(kind: &str) -> Self {
        match kind {
            // A DONOR JOINS A UNION THE WAY A PARENT DOES, which is what the
            // shape says and all it says: it is the same line into the same
            // point, drawn in its own tone and named by its own word.
            "union" | "partner" | "marriage" | "donor" => Self::Union,
            "link" | "lateral" | "note" => Self::Lateral,
            _ => Self::Descent,
        }
    }
}

/// One declared piece, before anything has been placed.
#[derive(Debug, Clone)]
pub struct Node {
    /// The identifier bonds refer to. `[A-Za-z0-9_-]+`, and nothing else ever
    /// reaches this field — see [`parse`].
    pub id: String,
    /// Figure or mark.
    pub piece: Piece,
    /// The name shown on the card. A name, not a sentence.
    pub label: String,
    /// How many figures this node stands in for, when it is a cut marker the
    /// pruning pass synthesised. Zero for everything an author wrote.
    pub elided: usize,
    /// A second line: dates, a role, an identifier.
    pub meta: String,
    /// What a filter matches, beyond the label. Space-separated words.
    pub terms: String,
    /// A picture, as a URL the consumer owns. Empty means none.
    pub image: String,
    /// The author's word for what this is. Reaches the DOM as an attribute.
    pub kind: String,
    /// The author's word for its colour family.
    pub tone: String,
    /// The author's word for its outline.
    pub shape: String,
    /// The swappable set this belongs to, as `group:name`. Empty means it is
    /// always shown.
    pub set: String,
    /// A link, when the piece is one. Empty means it is not.
    pub href: String,
    /// Which tree this belongs to, when the author says rather than lets the
    /// connections decide.
    pub cluster: String,
    /// An explicit generation, for the cases inference cannot win.
    pub rank: Option<i32>,
    /// How many cells wide, doubled — so a half-cell is expressible.
    pub span: i32,
    /// How far connections stop short of this piece, in TENTHS of a cell;
    /// `-1` means the scene's own `reach` decides. Tenths rather than the
    /// half-cells everything structural uses, because this is a distance a
    /// reader tunes against a name's descenders — `reach=0.8` rounded to a
    /// whole cell once, and the line stood a wilderness away. A piece-level
    /// reach works in a `touch` scene too.
    pub reach: i32,
    /// Whether the content comes from a `LineageFigure` child rather than from
    /// `label` and `meta`.
    pub slot: bool,
    /// Whether this is the top of the tree: everything otherwise parentless in
    /// its cluster hangs from it, however much is added later.
    pub origin: bool,
}

impl Default for Node {
    /// Written out rather than derived, for one field: a piece is **two**
    /// half-cells wide, not zero. A derived `Default` would place every figure
    /// on top of the next one, and it would do it silently.
    fn default() -> Self {
        Self {
            id: String::new(),
            piece: Piece::Figure,
            label: String::new(),
            elided: 0,
            meta: String::new(),
            terms: String::new(),
            image: String::new(),
            kind: String::new(),
            tone: String::new(),
            shape: String::new(),
            set: String::new(),
            href: String::new(),
            cluster: String::new(),
            rank: None,
            span: 2,
            reach: -1,
            slot: false,
            origin: false,
        }
    }
}

/// One declared connection.
#[derive(Debug, Clone, Default)]
pub struct Bond {
    /// The id it leaves.
    pub from: String,
    /// The id it reaches.
    pub to: String,
    /// The author's word, kept verbatim for the DOM.
    pub kind: String,
    /// What that word does to the ranks.
    pub shape: Shape,
    /// A word or two drawn on the connection.
    pub label: String,
    /// A sentence shown when the connection is focused or hovered.
    pub note: String,
    /// The author's word for its colour family.
    pub tone: String,
    /// The author's word for its stroke: `solid`, `dashed`, `dotted`.
    pub dash: String,
    /// `elbow` or `taut`. Empty takes the field's setting.
    pub route: String,
    /// The swappable set this belongs to, as `group:name`.
    pub set: String,
}

/// Everything a graph declared, before anything is placed.
#[derive(Debug, Clone, Default)]
pub struct Graph {
    /// Figures and marks, in declaration order. Declaration order is the tie
    /// break everywhere below, so a reader can predict the drawing from the
    /// text.
    pub nodes: Vec<Node>,
    /// Connections, in declaration order.
    pub bonds: Vec<Bond>,
    /// Lines that said nothing this parser understands.
    pub skipped: usize,
}

/// How the scene is arranged.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum View {
    /// Everything on one plane, as every family tree has always been drawn.
    /// There is still perspective and still depth — a figure is a slab, not a
    /// sticker — but nothing is placed behind anything else.
    #[default]
    Flat,
    /// Branches fan into depth. Each family lane takes its own plane, so a
    /// generation forty figures wide becomes four rows of ten at four depths
    /// instead of a line that has to be scrolled.
    Deep,
}

/// How a connection is drawn when the bond itself does not say.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Route {
    /// Axis-aligned runs, sharing one bracket between siblings. The drawing
    /// every family tree and every organisation chart already uses, and the
    /// only one that needs no rotation at all.
    #[default]
    Elbow,
    /// One straight run from end to end, rotated into place.
    Taut,
}

/// What was asked of [`place`].
#[derive(Debug, Clone, Copy, Default)]
pub struct Options {
    /// Flat or deep.
    pub view: View,
    /// The default route.
    ///
    /// THERE IS NO `spacing`. It was three gaps — tight, comfortable, roomy —
    /// and two of the three drew figures touching or overlapping, because a
    /// figure was as wide as its cell and no gap could help that. The width is
    /// fixed where it was wrong (`[scene] figure`), and one gap is left: the
    /// one that reads.
    pub route: Route,
    /// Whether the filter removes rather than dims.
    ///
    /// `isolate` re-lays the scene on the subgraph the filter keeps, because a
    /// drawing that preserves the holes of the people it removed is worse than
    /// one that moves: four removed figures left four figures' empty space and
    /// a cut marker floating at the far end of a page-wide line. `highlight`
    /// never moves anything, which is the promise this flag deliberately does
    /// not extend.
    pub isolate: bool,
    /// Whether connections stop short of the figures they join.
    ///
    /// `reach: shy` — for scenes whose figures are not boxes (`shape=bare`),
    /// where a line driven to the middle of a floating name reads as a
    /// strike-through. The trim happens after routing, at every run end that
    /// lands on a figure's own point; marks keep their touch, because a union
    /// line stopping short of its own dot reads as a break, not a manner.
    pub shy: bool,
}

/// Which way a run of connection travels.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Axis {
    /// Across.
    X,
    /// Down.
    Y,
    /// Into the scene.
    Z,
    /// None of the three: the run carries a [`Segment::basis`] instead.
    Free,
}

/// One straight run of one connection.
///
/// The origin is the end the run starts from and `len` is how far it goes,
/// both in cells. A stylesheet turns a cell into a length; this crate never
/// sees one.
#[derive(Debug, Clone, Copy)]
pub struct Segment {
    /// Which bond drew it, as an index into [`Scene::bonds`].
    pub bond: usize,
    /// Across, in cells.
    pub x: f64,
    /// Down, in cells.
    pub y: f64,
    /// Into the scene, in cells.
    pub z: f64,
    /// How long, in cells.
    pub len: f64,
    /// Which way.
    pub axis: Axis,
    /// The rotation that takes the run's own `+x` onto its direction, as a
    /// column-major 3×3, when [`Segment::axis`] is [`Axis::Free`]. Pure
    /// rotation: no scale and no translation, so it stays correct however the
    /// stylesheet scales a cell.
    pub basis: [f64; 9],
}

/// One piece, placed.
#[derive(Debug, Clone)]
pub struct Place {
    /// The declaration it came from, as an index into [`Graph::nodes`].
    pub node: usize,
    /// Across, in cells — the centre of the piece.
    pub x: f64,
    /// Down, in cells.
    pub y: f64,
    /// Into the scene, in cells.
    pub z: f64,
    /// Which generation, counted from the shallowest in the whole scene.
    pub rank: i32,
    /// Which tree, counted in the order the trees were first mentioned.
    pub cluster: usize,
    /// Which depth lane, within its rank.
    pub lane: i32,
}

/// A whole graph, placed.
#[derive(Debug, Clone, Default)]
pub struct Scene {
    /// What was declared. Every [`Place`] and every [`Segment`] indexes into
    /// it, so the renderer reads one structure rather than two.
    pub graph: Graph,
    /// One per node, in the same order.
    pub places: Vec<Place>,
    /// The runs to draw, grouped by bond but flattened.
    pub segments: Vec<Segment>,
    /// How wide the scene is, in cells.
    pub width: f64,
    /// How tall, in cells.
    pub height: f64,
    /// How deep, in cells.
    pub depth: f64,
    /// How many pairs of connections cross. Reported rather than hidden,
    /// because "the ordering pass reduces crossings" is a claim, and a claim
    /// nothing measures is a claim that stops being true.
    pub crossings: usize,
    /// Lines the parser did not understand.
    pub skipped: usize,
    /// Bonds dropped to break a cycle. An author who says X descends from Y
    /// descends from X has written something impossible; the drawing keeps
    /// every figure and drops the connection that closed the loop.
    pub broken: usize,
}

/// Whether a piece answers a filter.
///
/// Matches the label, the metadata and the declared terms, case-folded and
/// diacritic-folded per code point, as a substring. Per code point because the
/// fold has to be length-preserving in both languages: `toLowerCase()` is not,
/// and this is shared by two implementations that must agree.
///
/// An empty filter matches everything, which is what makes "no filter" and "a
/// filter that happens to be empty" the same thing rather than two.
///
/// **A filter of nothing but spaces is an empty filter**, and it took the
/// JavaScript half to prove it: the Rust test that claimed so passed for a
/// reason that had nothing to do with the rule — a node with no metadata and no
/// terms produces a haystack with two spaces in it, and `"  "` was matching
/// *that*. The trim is ASCII-only in both languages, because `str::trim` and
/// `String.prototype.trim` do not agree about every code point.
#[must_use]
pub fn matches(node: &Node, filter: &str) -> bool {
    let filter = trim(&fold(filter));
    if filter.is_empty() {
        return true;
    }
    let hay = fold(&format!("{} {} {}", node.label, node.meta, node.terms));
    hay.contains(&filter)
}

/// Drops the ASCII whitespace either side. See [`matches`] for why not `trim`.
fn trim(text: &str) -> String {
    text.trim_matches(|ch: char| ch.is_ascii_whitespace())
        .to_owned()
}

/// Lower-cases and strips the Latin-1 diacritics, one code point in, one out.
///
/// The table is the one `site.rs` and `search-match.js` already agree on. It is
/// deliberately not `to_lowercase()`: that is locale-independent in Rust but
/// length-changing (`İ` becomes two code points), and the JavaScript twin's
/// `toLowerCase()` is length-changing too, in different places.
fn fold(text: &str) -> String {
    let mut out = String::with_capacity(text.len());
    for ch in text.chars() {
        out.push(match ch {
            'À'..='Å' | 'à'..='å' => 'a',
            'Ç' | 'ç' => 'c',
            'È'..='Ë' | 'è'..='ë' => 'e',
            'Ì'..='Ï' | 'ì'..='ï' => 'i',
            'Ñ' | 'ñ' => 'n',
            'Ò'..='Ö' | 'ò'..='ö' | 'Ø' | 'ø' => 'o',
            'Ù'..='Ü' | 'ù'..='ü' => 'u',
            'Ý' | 'ý' | 'ÿ' => 'y',
            other => other.to_ascii_lowercase(),
        });
    }
    out
}

/// Parses a graph and places it, which is the whole crate in one call.
#[must_use]
pub fn scene(graph: &str, options: Options) -> Scene {
    place(parse(graph), options)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn an_unknown_kind_still_places_its_endpoints() {
        // A consumer inventing `apprenticeship` gets a descent, because that is
        // the shape of almost every relationship anybody draws — and because a
        // bond nobody recognised must still put its ends somewhere.
        assert_eq!(Shape::of("apprenticeship"), Shape::Descent);
        assert_eq!(Shape::of("union"), Shape::Union);
        assert_eq!(Shape::of("link"), Shape::Lateral);
    }

    #[test]
    fn the_fold_is_one_code_point_in_one_out() {
        // The length-preserving property, asserted rather than assumed: the
        // JavaScript twin folds per code point for the same reason, and a fold
        // that changed a length would make the two halves disagree about where
        // a match starts.
        for text in ["Ada", "ÀÉÎÕÜ", "coração", "ÿ", "Doe-Smith"] {
            assert_eq!(
                fold(text).chars().count(),
                text.chars().count(),
                "{text} changed length"
            );
        }
        assert_eq!(fold("Coração"), "coracao");
    }

    #[test]
    fn an_empty_filter_matches_everything() {
        let node = Node {
            label: "Ada Doe".to_owned(),
            ..Node::default()
        };
        assert!(matches(&node, ""));
        assert!(matches(&node, "  "));
        assert!(matches(&node, "doe"));
        assert!(!matches(&node, "smith"));
    }

    #[test]
    fn a_filter_reaches_the_metadata_and_the_terms() {
        let node = Node {
            label: "Ada".to_owned(),
            meta: "1815–1852".to_owned(),
            terms: "doe founder".to_owned(),
            ..Node::default()
        };
        assert!(matches(&node, "1815"));
        assert!(matches(&node, "FOUNDER"));
    }
}
