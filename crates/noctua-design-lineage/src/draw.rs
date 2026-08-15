//! Everything a renderer needs, worked out once.
//!
//! # Why this is here and not in the two component bodies
//!
//! `Lineage` is written twice — once in `rsx!` and once in `createElement` —
//! and every line of logic in those bodies is a line that can drift. So the
//! bodies are dumb: they walk this and stamp attributes. Which of a swappable
//! set is showing, whether a figure answers the filter, which figures a figure
//! is joined to and where a connection's label sits are all decided here, in
//! the one place that has a twin held to a blessed file.
//!
//! # The ties are links, and that is an accessibility decision
//!
//! Each piece carries the pieces it is joined to. A renderer turns them into
//! visually hidden links named by their target, because the alternative —
//! *child of Ada and Bob, partner of Eve* — is a sentence assembled from
//! fragments, which cannot be translated (`NOCTUA.md §9.9` rule 1) and which no
//! screen reader can navigate. A link named by its target says the same thing
//! in every language and can be followed.
//!
//! # A group with nothing chosen shows what was declared first
//!
//! Otherwise declaring a swappable set would make a branch invisible until
//! somebody switched it on, and the common case — *this is the local data, that
//! is the same branch from the family's own server* — would need configuration
//! to draw at all.

use crate::{Axis, Bond, Graph, Node, Options, Piece, Place, Scene, Segment, Shape, matches};

/// One figure a renderer is joined to, and how.
#[derive(Debug, Clone)]
pub struct Tie {
    /// The id at the other end.
    pub id: String,
    /// That piece's own name, which is what names the link.
    pub label: String,
    /// The author's word for the connection.
    pub kind: String,
    /// The connection's own label, when it has one.
    pub note: String,
    /// Whether the other end is above this one.
    pub upward: bool,
}

/// One piece, ready to draw.
///
/// Four independent booleans, and clippy is right that that is usually a shape
/// worth questioning — but these are four different questions about one piece
/// (*is it a mark, does it wait for a slot, does it answer the filter, is its
/// set the one showing*) and any enum grouping them would have to enumerate
/// their sixteen combinations.
#[allow(clippy::struct_excessive_bools)]
#[derive(Debug, Clone)]
pub struct DrawnPiece {
    /// Its id, which is already known to be a word.
    pub id: String,
    /// A mark rather than a figure: half the size, and a point on the structure
    /// rather than a member of it.
    pub mark: bool,
    /// Its content comes from a `LineageFigure` rather than from the graph.
    pub slot: bool,
    /// The name on the card.
    pub label: String,
    /// The second line.
    pub meta: String,
    /// A picture, already checked for a scheme that could run.
    pub image: String,
    /// A link, likewise.
    pub href: String,
    /// The author's words, which reach the DOM as attributes.
    pub kind: String,
    /// Likewise.
    pub tone: String,
    /// Likewise.
    pub shape: String,
    /// Likewise, as `group:name`.
    pub set: String,
    /// Across, in cells.
    pub x: f64,
    /// Down, in cells.
    pub y: f64,
    /// Into the scene, in cells.
    pub z: f64,
    /// How wide, in cells.
    pub span: f64,
    /// How far connections stop short of this piece, in cells; negative means
    /// the scene's own `reach` decided.
    pub reach: f64,
    /// Whether it answers the filter. Always true when there is no filter.
    pub matched: bool,
    /// Whether its set is the one showing.
    pub shown: bool,
    /// What it is joined to.
    pub ties: Vec<Tie>,
    /// How many figures this piece stands in for, when it is a cut marker.
    /// Zero for everything else, which is how a renderer tells them apart.
    pub elided: usize,
}

/// The subgraph `isolate` draws: what answers, plus one marker per cut.
///
/// The removed figures form connected groups. Each group with at least one
/// person in it becomes a single marker node — a ring carrying the count —
/// and every bond that crossed the cut is re-aimed at the marker, keeping its
/// kind, so the marker stands exactly where the graph put the people it
/// replaces: beside a union if they married in, under a bracket if they
/// descended. Duplicate re-aimed bonds collapse to one.
fn prune(graph: &Graph, filter: &str) -> Graph {
    let kept: Vec<bool> = graph
        .nodes
        .iter()
        .map(|node| answers(graph, node, filter))
        .collect();
    let at = |id: &str| graph.nodes.iter().position(|node| node.id == id);

    let mut seen = vec![false; graph.nodes.len()];
    let mut markers: Vec<Node> = Vec::new();
    let mut stand_in: Vec<Option<String>> = vec![None; graph.nodes.len()];
    for index in 0..graph.nodes.len() {
        if kept[index] || seen[index] {
            continue;
        }
        let mut component = vec![index];
        seen[index] = true;
        let mut walk = 0;
        while walk < component.len() {
            let here = &graph.nodes[component[walk]].id;
            walk += 1;
            for bond in &graph.bonds {
                let far = if &bond.from == here {
                    &bond.to
                } else if &bond.to == here {
                    &bond.from
                } else {
                    continue;
                };
                if let Some(other) = at(far) {
                    if !kept[other] && !seen[other] {
                        seen[other] = true;
                        component.push(other);
                    }
                }
            }
        }
        let count = component
            .iter()
            .filter(|&&member| graph.nodes[member].piece == Piece::Figure)
            .count();
        if count == 0 {
            continue;
        }
        let marker = format!("{}-elided", graph.nodes[component[0]].id);
        markers.push(Node {
            id: marker.clone(),
            kind: "elided".to_owned(),
            elided: count,
            ..Node::default()
        });
        let cluster = graph.nodes[component[0]].cluster.clone();
        markers.last_mut().expect("just pushed").cluster = cluster;
        for &member in &component {
            stand_in[member] = Some(marker.clone());
        }
    }

    let mut nodes: Vec<Node> = graph
        .nodes
        .iter()
        .zip(&kept)
        .filter(|(_, keep)| **keep)
        .map(|(node, _)| node.clone())
        .collect();
    nodes.extend(markers);

    let mut bonds: Vec<Bond> = Vec::new();
    for bond in &graph.bonds {
        let map_end = |id: &str| -> Option<String> {
            match at(id) {
                Some(index) if kept[index] => Some(id.to_owned()),
                Some(index) => stand_in[index].clone(),
                None => Some(id.to_owned()),
            }
        };
        let (Some(from), Some(to)) = (map_end(&bond.from), map_end(&bond.to)) else {
            continue;
        };
        if from == to {
            continue;
        }
        // A re-aimed bond that another bond already re-aimed says nothing new.
        if bonds
            .iter()
            .any(|prior| prior.from == from && prior.to == to && prior.kind == bond.kind)
        {
            continue;
        }
        let mut kept_bond = bond.clone();
        kept_bond.from = from;
        kept_bond.to = to;
        bonds.push(kept_bond);
    }

    Graph {
        nodes,
        bonds,
        skipped: graph.skipped,
    }
}

/// The filter, folded and trimmed the way [`matches`] folds and trims it.
fn trimmed(filter: &str) -> String {
    filter
        .trim_matches(|ch: char| ch.is_ascii_whitespace())
        .to_owned()
}

/// One connection, ready to draw.
#[derive(Debug, Clone)]
pub struct DrawnBond {
    /// Its position in the graph, which is what a run refers back to.
    pub index: usize,
    /// The author's word for what it is.
    pub kind: String,
    /// Likewise.
    pub tone: String,
    /// Likewise.
    pub dash: String,
    /// Likewise, as `group:name`.
    pub set: String,
    /// A word or two drawn on it.
    pub label: String,
    /// A sentence shown when it is focused or hovered.
    pub note: String,
    /// Where it leaves.
    pub from: String,
    /// Where it arrives.
    pub to: String,
    /// Whether both ends answer the filter.
    pub matched: bool,
    /// Whether its set is the one showing.
    pub shown: bool,
    /// Where its label and its hit area sit, in cells.
    pub tag: [f64; 3],
    /// Whether the label goes just BELOW its line instead of just above it.
    ///
    /// A label lives above its line. On a row-line between two cards that
    /// stand close — a partner with no union mark, a donor beside a wide
    /// unknown card — "above the line" is inside somebody's card, and the
    /// empty strip is the channel underneath. The engine decides, because the
    /// engine knows the widths; a stylesheet cannot.
    pub tag_below: bool,
    /// The runs that draw it.
    pub runs: Vec<Segment>,
}

/// A whole scene, ready to draw.
#[derive(Debug, Clone, Default)]
pub struct Drawing {
    /// Every piece, in declaration order.
    pub pieces: Vec<DrawnPiece>,
    /// Every connection, in declaration order.
    pub bonds: Vec<DrawnBond>,
    /// How wide the scene is, in cells.
    pub width: f64,
    /// How tall.
    pub height: f64,
    /// How deep.
    pub depth: f64,
    /// How many pairs of connections cross.
    pub crossings: usize,
    /// How many lines or attributes the parser did not understand.
    pub skipped: usize,
    /// How many bonds were dropped to break a cycle.
    pub broken: usize,
    /// How many figures answer the filter. Equal to the number of pieces when
    /// there is no filter, which is what makes "no filter" and "everything
    /// matches" the same state rather than two.
    pub matched: usize,
}

/// Which way a run travels, as the attribute a stylesheet keys on.
///
/// Here rather than in either renderer, so the two cannot spell an axis
/// differently — and `Debug` is not it: a derived name is a Rust detail that
/// would silently become part of a selector.
#[must_use]
pub fn axis_name(axis: Axis) -> &'static str {
    match axis {
        Axis::X => "x",
        Axis::Y => "y",
        Axis::Z => "z",
        Axis::Free => "free",
    }
}

/// An attribute value, or nothing at all when the author said nothing.
///
/// Both renderers stamp `data-nd-tone` only where a tone was declared, and this
/// is what keeps them agreeing about that. An attribute present with an empty
/// value is an attribute a selector matches — `[data-nd-tone]` would be every
/// piece in the scene — which is a defect that looks like a styling mistake.
#[must_use]
pub fn some(value: &str) -> Option<String> {
    (!value.is_empty()).then(|| value.to_owned())
}

/// Whether a word may be interpolated into a custom property name.
///
/// **This is a security check, not a tidiness one.** An occasion and a set both
/// become part of a property name in a `style` attribute, and both arrive from
/// a prop rather than from the graph — so neither has been through the parser's
/// identifier rule. A value containing `;` or `)` would end the declaration and
/// begin another one. `NOCTUA.md §2.4`: fail closed, so anything else is
/// dropped rather than escaped.
#[must_use]
pub fn nameable(value: &str) -> bool {
    !value.is_empty()
        && value.len() <= 64
        && value
            .chars()
            .all(|ch| ch.is_ascii_alphanumeric() || ch == '_' || ch == '-')
}

/// A `group:name` set, as the two words that make a property name.
#[must_use]
pub fn set_parts(set: &str) -> Option<(String, String)> {
    let (group, name) = set.split_once(':')?;
    (nameable(group) && nameable(name)).then(|| (group.to_owned(), name.to_owned()))
}

/// Reads a graph, places it, and works out everything a renderer needs.
#[must_use]
pub fn draw(source: &str, options: Options, filter: &str, sets: &str) -> Drawing {
    // `isolate` REMOVES, SO THE LAYOUT MUST NOT REMEMBER. The scene is laid
    // out on the subgraph the filter keeps, plus one cut marker per removed
    // group — because a drawing that preserves the holes of the removed kept
    // four figures' empty space and a marker floating at the far end of a
    // page-wide line. `highlight` still lays out the whole graph and dims,
    // which is the mode for readers who must not lose their place.
    let parsed = crate::parse(source);
    let parsed = if options.isolate && !trimmed(filter).is_empty() {
        prune(&parsed, filter)
    } else {
        parsed
    };
    let laid = crate::place(parsed, options);
    let active = chosen(&laid.graph, sets);

    let pieces: Vec<DrawnPiece> = laid
        .places
        .iter()
        .map(|place| piece(&laid, place, filter, &active))
        .collect();
    let matched = pieces
        .iter()
        .filter(|piece| piece.matched && !piece.mark && piece.elided == 0)
        .count();
    let bonds = laid
        .graph
        .bonds
        .iter()
        .enumerate()
        .map(|(index, bond)| line(&laid, index, bond, &pieces, &active))
        .collect();

    Drawing {
        pieces,
        bonds,
        width: laid.width,
        height: laid.height,
        depth: laid.depth,
        crossings: laid.crossings,
        skipped: laid.skipped,
        broken: laid.broken,
        matched,
    }
}

/// Which set of each group is showing.
///
/// What the caller asked for, and for any group they did not mention, whichever
/// set that group declared first.
fn chosen(graph: &Graph, sets: &str) -> Vec<(String, String)> {
    let mut active: Vec<(String, String)> = Vec::new();
    for entry in sets.split_whitespace() {
        if let Some((group, name)) = set_parts(entry)
            && !active.iter().any(|(other, _)| other == &group)
        {
            active.push((group, name));
        }
    }
    let declared = graph
        .nodes
        .iter()
        .map(|node| &node.set)
        .chain(graph.bonds.iter().map(|bond| &bond.set));
    for set in declared {
        if let Some((group, name)) = set_parts(set)
            && !active.iter().any(|(other, _)| other == &group)
        {
            active.push((group, name));
        }
    }
    active
}

/// Whether a thing carrying this set is the one showing.
fn showing(set: &str, active: &[(String, String)]) -> bool {
    match set_parts(set) {
        None => true,
        Some((group, name)) => active
            .iter()
            .any(|(other, chosen)| other == &group && chosen == &name),
    }
}

/// One piece, with its ties.
fn piece(laid: &Scene, place: &Place, filter: &str, active: &[(String, String)]) -> DrawnPiece {
    let node = &laid.graph.nodes[place.node];
    DrawnPiece {
        id: node.id.clone(),
        mark: node.piece == Piece::Mark,
        slot: node.slot,
        // THE NAME AS IT IS DRAWN, decided here so the two renderers cannot
        // disagree about it. A figure with no label shows its id, because a
        // card that is an empty rectangle is a card nobody can point at; a mark
        // with no label shows nothing, because a mark is a dot on a line and
        // `u0` written beside it is noise.
        label: if node.piece == Piece::Mark {
            node.label.clone()
        } else {
            shown_name(node)
        },
        meta: node.meta.clone(),
        image: node.image.clone(),
        href: node.href.clone(),
        kind: node.kind.clone(),
        tone: node.tone.clone(),
        shape: node.shape.clone(),
        set: node.set.clone(),
        x: place.x,
        y: place.y,
        z: place.z,
        span: f64::from(node.span) / 2.0,
        reach: f64::from(node.reach) / 10.0,
        matched: node.elided > 0 || answers(&laid.graph, node, filter),
        shown: showing(&node.set, active),
        ties: ties(laid, &node.id),
        elided: node.elided,
    }
}

/// Whether a piece answers the filter.
///
/// # A union point answers when the people it joins do
///
/// A mark with no name of its own is not something a reader can search for —
/// it is the join between the things they can. Asked whether it matches
/// "Doe" it said no, every bond through it was a bond with an unmatched end,
/// and a scene filtered to one family drew that family as a set of cards with
/// nothing between them. Every connection in a genealogy passes through a
/// union, so this was every connection.
///
/// So an unlabelled mark answers when either end does — the same walk-through
/// [`ties`] already makes, for the same reason. A mark WITH a label is a
/// milestone, a note, a family's server: a thing in its own right, and it
/// answers for itself.
///
/// One hop, not a search: a mark joined to a mark is not a shape this engine
/// produces, and a filter that walked the whole graph would light up a family
/// because somebody at the other end of it shares a surname.
fn answers(graph: &Graph, node: &Node, filter: &str) -> bool {
    if node.piece != Piece::Mark || !node.label.is_empty() {
        return matches(node, filter);
    }
    graph.bonds.iter().any(|bond| {
        let far = if bond.from == node.id {
            &bond.to
        } else if bond.to == node.id {
            &bond.from
        } else {
            return false;
        };
        graph
            .nodes
            .iter()
            .any(|other| &other.id == far && matches(other, filter))
    })
}

/// Everything one piece is joined to, in declaration order.
///
/// # It sees through an unlabelled mark, and it has to
///
/// A union is a mark, so Ada's only *direct* connection is to `u0` — and a
/// visually hidden link announcing "u0" is a link that says nothing, which is
/// exactly what the pseudo-locale check caught. So a mark with no name of its
/// own is walked through: Ada's ties are her partner and her children, which is
/// what a reader would have said they were.
///
/// A mark **with** a label is not walked through, because then it is a thing in
/// its own right — a milestone, a note, the family's server — and a link to it
/// is a link somebody meant.
///
/// # Direction is a rank comparison, not a bond direction
///
/// A bond points from a parent to a union and from that union to a child, so
/// following the arrows says nothing about who is above whom once a mark is in
/// the way. The ranks already know.
fn ties(laid: &Scene, id: &str) -> Vec<Tie> {
    let rank_of = |other: &str| -> Option<i32> {
        laid.places
            .iter()
            .find(|place| laid.graph.nodes[place.node].id == other)
            .map(|place| place.rank)
    };
    let node_of = |other: &str| laid.graph.nodes.iter().find(|node| node.id == other);
    let mine = rank_of(id).unwrap_or(0);
    let other_end = |bond: &Bond, from: &str| -> Option<String> {
        if bond.from == from {
            Some(bond.to.clone())
        } else if bond.to == from {
            Some(bond.from.clone())
        } else {
            None
        }
    };

    let mut out: Vec<Tie> = Vec::new();
    for bond in &laid.graph.bonds {
        let Some(next) = other_end(bond, id) else {
            continue;
        };
        let through = node_of(&next)
            .is_some_and(|node| node.piece == Piece::Mark && node.label.is_empty())
            .then(|| {
                laid.graph
                    .bonds
                    .iter()
                    .filter_map(|onward| {
                        let far = other_end(onward, &next)?;
                        // WHICH OF THE TWO BONDS DESCRIBES THE PAIR. Cara
                        // reaches Ada through the union: the bond she arrived
                        // on says *adopted* and the one going on says
                        // *union*, and it is the first that describes them.
                        // Ada reaches Cara the other way round, so the rule is
                        // "whichever is not the union" rather than "the first".
                        let plain = bond.shape == Shape::Union && onward.shape != Shape::Union;
                        let (kind, note) = if plain {
                            (onward.kind.clone(), onward.label.clone())
                        } else {
                            (bond.kind.clone(), bond.label.clone())
                        };
                        let note = if note.is_empty() && plain {
                            bond.label.clone()
                        } else if note.is_empty() {
                            onward.label.clone()
                        } else {
                            note
                        };
                        (far != id).then_some((far, kind, note))
                    })
                    .collect::<Vec<_>>()
            });
        let reached =
            through.unwrap_or_else(|| vec![(next, bond.kind.clone(), bond.label.clone())]);
        for (far, kind, note) in reached {
            let Some(node) = node_of(&far) else { continue };
            if out.iter().any(|tie| tie.id == far) {
                continue;
            }
            out.push(Tie {
                id: far.clone(),
                label: shown_name(node),
                kind,
                note,
                upward: rank_of(&far).is_some_and(|rank| rank < mine),
            });
        }
    }
    out
}

/// One connection, with the runs that draw it.
fn line(
    laid: &Scene,
    index: usize,
    bond: &Bond,
    pieces: &[DrawnPiece],
    active: &[(String, String)],
) -> DrawnBond {
    let at = |id: &str| pieces.iter().find(|piece| piece.id == id);
    let from = at(&bond.from);
    let to = at(&bond.to);
    let ends = |pick: fn(&DrawnPiece) -> f64| {
        let a = from.map_or(0.0, pick);
        let b = to.map_or(0.0, pick);
        // allow-literal: `f64::midpoint` has no JavaScript spelling, and
        // `layout.js` is a transcription a reader checks line against line.
        #[allow(clippy::manual_midpoint)]
        {
            (a + b) / 2.0
        }
    };
    DrawnBond {
        index,
        kind: bond.kind.clone(),
        tone: bond.tone.clone(),
        dash: bond.dash.clone(),
        set: bond.set.clone(),
        label: bond.label.clone(),
        note: bond.note.clone(),
        from: bond.from.clone(),
        to: bond.to.clone(),
        matched: from.is_none_or(|piece| piece.matched) && to.is_none_or(|piece| piece.matched),
        tag_below: match (from, to) {
            // allow-literal: `f64::midpoint` has no JavaScript spelling, and
            // `layout.js` is a transcription a reader checks line against line.
            #[allow(clippy::manual_midpoint)]
            (Some(a), Some(b)) => {
                (a.y - b.y).abs() < f64::EPSILON
                    && ((a.x - b.x).abs() - (a.span + b.span) / 2.0) < 1.0
            }
            _ => false,
        },
        // A CONNECTION WITH A MARKER AT ONE END STILL REACHES SOMETHING.
        //
        // `isolate` hides everything that does not answer, connections
        // included, so the line to a figure it removed went with it — and a
        // marker standing where that figure was had nothing joining it to the
        // family. This says the far end is a marker, and the stylesheet draws
        // the line for that reason alone.
        shown: showing(&bond.set, active)
            && from.is_none_or(|piece| piece.shown)
            && to.is_none_or(|piece| piece.shown),
        // THE LABEL SITS IN THE CHANNEL, NOT BETWEEN THE TWO ENDS.
        //
        // The midpoint of a parent and a child is halfway down the gap AND
        // halfway across it — which for a bracket that reaches sideways is
        // directly over whoever is standing between them. It was reported as a
        // label written over a person's face. A bracket has a horizontal run
        // lying in the air between two rows, and the middle of that run is a
        // place where nothing else ever is; a connection with no horizontal run
        // is a straight one, and the midpoint is right for it.
        tag: match (from, to) {
            // A row line: the label centres on the FREE stretch between the
            // two card edges, which is the only part of the line a reader can
            // see — between the centres it lands inside the wider card.
            (Some(a), Some(b)) if (a.y - b.y).abs() < f64::EPSILON => {
                let (left, right) = if a.x <= b.x { (a, b) } else { (b, a) };
                [
                    ((left.x + left.span / 2.0) + (right.x - right.span / 2.0)) / 2.0,
                    a.y,
                    a.z,
                ]
            }
            _ => laid
                .segments
                .iter()
                .find(|run| run.bond == index && run.axis == Axis::X)
                .map_or_else(
                    || {
                        [
                            ends(|piece| piece.x),
                            ends(|piece| piece.y),
                            ends(|piece| piece.z),
                        ]
                    },
                    |run| [run.x + run.len / 2.0, run.y, run.z],
                ),
        },
        runs: laid
            .segments
            .iter()
            .filter(|run| run.bond == index)
            .copied()
            .collect(),
    }
}

/// The name a piece is drawn with: its label, or its id when it has none.
///
/// Shared so that the two renderers cannot disagree about what an unlabelled
/// figure is called — and it is the id rather than nothing, because a card
/// showing an empty rectangle is a card nobody can point at.
#[must_use]
pub fn shown_name(node: &Node) -> String {
    if node.label.is_empty() {
        node.id.clone()
    } else {
        node.label.clone()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const FAMILY: &str = "figure ada label=\"Ada Doe\" terms=\"doe\"\n\
                          figure bob label=\"Bob Smith\"\n\
                          mark u0 kind=union\n\
                          bond ada u0 kind=union\n\
                          bond bob u0 kind=union\n\
                          figure cara label=\"Cara Doe\"\n\
                          bond u0 cara kind=descent label=\"adopted\"\n";

    #[test]
    fn a_tie_sees_through_a_union_and_names_a_person() {
        // THE DEFECT THIS NAMES. Ada's only direct connection is to the union
        // mark, so her ties used to announce "u0" — a visually hidden link that
        // says nothing, which is what the pseudo-locale check caught on the
        // rendered page. A mark with no name of its own is walked through.
        let drawing = draw(FAMILY, Options::default(), "", "");
        let ada = &drawing.pieces[0];
        let named: Vec<&str> = ada.ties.iter().map(|tie| tie.label.as_str()).collect();
        assert_eq!(named, ["Bob Smith", "Cara Doe"]);
        assert!(!ada.ties.iter().any(|tie| tie.id == "u0"), "{named:?}");

        // And the direction is a rank comparison: a bond points from a parent
        // to a union and from that union to a child, so following the arrows
        // says nothing about who is above whom.
        let cara = drawing
            .pieces
            .iter()
            .find(|piece| piece.id == "cara")
            .expect("cara");
        assert!(cara.ties.iter().all(|tie| tie.upward), "{:?}", cara.ties);
        assert!(ada.ties.iter().all(|tie| !tie.upward), "{:?}", ada.ties);
        assert_eq!(
            cara.ties
                .iter()
                .find(|tie| tie.id == "ada")
                .map(|tie| tie.note.as_str()),
            Some("adopted"),
            "the connection's own word survives the walk"
        );
    }

    #[test]
    fn a_filter_marks_rather_than_moves() {
        let with = draw(FAMILY, Options::default(), "doe", "");
        let without = draw(FAMILY, Options::default(), "", "");
        assert_eq!(with.matched, 2, "Ada Doe and Cara Doe");
        assert_eq!(without.matched, 3, "no filter is everything");
        // THE CLAIM THE MODE RESTS ON: filtering changes what is marked and
        // nothing about where anything is. A scene that rearranged itself as a
        // reader typed would have to be re-read after every letter.
        for (a, b) in with.pieces.iter().zip(&without.pieces) {
            assert!((a.x - b.x).abs() < f64::EPSILON, "{} moved", a.id);
            assert!((a.y - b.y).abs() < f64::EPSILON, "{} moved", a.id);
        }
    }

    #[test]
    fn a_group_with_nothing_chosen_shows_what_was_declared_first() {
        let graph = "figure a set=src:local\nfigure b set=src:remote\nfigure c\n";
        let drawing = draw(graph, Options::default(), "", "");
        assert!(drawing.pieces[0].shown, "the first declared set");
        assert!(!drawing.pieces[1].shown);
        assert!(drawing.pieces[2].shown, "no set is always shown");

        let switched = draw(graph, Options::default(), "", "src:remote");
        assert!(!switched.pieces[0].shown);
        assert!(switched.pieces[1].shown);
    }

    #[test]
    fn nothing_but_a_word_can_reach_a_property_name() {
        // The injection this exists for: an occasion and a set are props, so
        // neither has been through the parser's identifier rule.
        assert!(nameable("christmas"));
        assert!(nameable("dia-das-maes"));
        assert!(!nameable("a);color:red;--x:("));
        assert!(!nameable(""));
        assert!(!nameable("a b"));
        assert_eq!(
            set_parts("src:remote"),
            Some(("src".to_owned(), "remote".to_owned()))
        );
        assert_eq!(set_parts("src"), None);
        assert_eq!(set_parts("s);x:(:remote"), None);
    }

    #[test]
    fn every_run_belongs_to_the_bond_that_drew_it() {
        let drawing = draw(FAMILY, Options::default(), "", "");
        for (index, bond) in drawing.bonds.iter().enumerate() {
            assert!(!bond.runs.is_empty(), "bond {index} draws nothing");
            for run in &bond.runs {
                assert_eq!(run.bond, index);
            }
        }
    }
}
