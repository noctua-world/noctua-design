//! The graph language, and the reasons it is this small.
//!
//! ```text
//! # A comment. Blank lines are ignored.
//! figure ada  label="Ada Doe" meta="1815–1852" terms="doe" origin
//! mark   u1   kind=union label="1832"
//! bond   ada u1  kind=union
//! bond   u1 cara kind=descent
//! ```
//!
//! Three statements — `figure`, `mark`, `bond` — and everything else is an
//! attribute of one of them. A hyperedge (*these two had these three
//! children*) is a mark with bonds in and bonds out, which is also exactly what
//! the user asked for when they asked for an element placed at a point on a
//! connection: the union point and the "Doe Family Server" are one mechanism,
//! not two.
//!
//! # Totality
//!
//! Nothing here can fail. A line either produces a statement or is counted in
//! [`Graph::skipped`], and the same is true of an attribute nobody recognises —
//! so a typo is *visible* (the count reaches the DOM) rather than silent, and a
//! malformed graph still draws every part of itself that made sense.
//!
//! # Why identifiers are restricted, and it is not tidiness
//!
//! An id is interpolated into a **custom property name** and into a `style`
//! attribute, which is how a child positions itself at a place its parent
//! computed. An id containing `;` or `)` would end the declaration and start
//! another — a CSS injection, in a component whose graph may well come from a
//! consumer's database. So an id is `[A-Za-z0-9_-]{1,64}` and anything else
//! skips the line: default deny, `NOCTUA.md §2.4`.
//!
//! The same reasoning bounds URLs: a `javascript:` href in author data is the
//! oldest cross-site scripting there is.

use crate::{Bond, Graph, Node, Piece, Shape};

/// The longest an identifier may be.
///
/// Not a taste: every id becomes part of a custom property name, and a scene
/// publishes three per slotted piece. A bound here is a bound on the style
/// attribute a page carries.
const ID_LIMIT: usize = 64;

/// Reads a graph. Never fails; see the module documentation.
#[must_use]
pub fn parse(source: &str) -> Graph {
    let mut graph = Graph::default();
    for line in source.lines() {
        let tokens = tokenize(line);
        if tokens.is_empty() {
            continue;
        }
        match tokens[0].as_str() {
            "figure" => statement(&mut graph, &tokens, Piece::Figure),
            "mark" => statement(&mut graph, &tokens, Piece::Mark),
            "bond" => bond(&mut graph, &tokens),
            _ => graph.skipped += 1,
        }
    }
    graph
}

/// Splits a line into tokens, honouring double quotes.
///
/// A `#` begins a comment only where a token would begin, so a label may
/// contain one. An unterminated quote takes the rest of the line rather than
/// discarding it: the parser is total, and a missing closing quote is a typo,
/// not an attack.
fn tokenize(line: &str) -> Vec<String> {
    let mut tokens = Vec::new();
    let mut token = String::new();
    let mut quoted = false;
    for ch in line.chars() {
        if quoted {
            if ch == '"' {
                quoted = false;
            } else {
                token.push(ch);
            }
        } else if ch == '"' {
            quoted = true;
        } else if ch.is_whitespace() {
            if !token.is_empty() {
                tokens.push(std::mem::take(&mut token));
            }
        } else if ch == '#' && token.is_empty() {
            break;
        } else {
            token.push(ch);
        }
    }
    if !token.is_empty() {
        tokens.push(token);
    }
    tokens
}

/// `figure <id> …` and `mark <id> …`.
fn statement(graph: &mut Graph, tokens: &[String], piece: Piece) {
    let Some(id) = tokens.get(1).and_then(|id| identifier(id)) else {
        graph.skipped += 1;
        return;
    };
    let mut node = Node {
        id,
        piece,
        span: if piece == Piece::Mark { 1 } else { 2 },
        ..Node::default()
    };
    for token in &tokens[2..] {
        if !attribute(&mut node, token) {
            graph.skipped += 1;
        }
    }
    graph.nodes.push(node);
}

/// `bond <from> <to> …`.
fn bond(graph: &mut Graph, tokens: &[String]) {
    let from = tokens.get(1).and_then(|id| identifier(id));
    let to = tokens.get(2).and_then(|id| identifier(id));
    let (Some(from), Some(to)) = (from, to) else {
        graph.skipped += 1;
        return;
    };
    if from == to {
        // A bond from a thing to itself has no drawing and no rank, and every
        // pass below would have to carry a special case for it forever.
        graph.skipped += 1;
        return;
    }
    let mut bond = Bond {
        from,
        to,
        ..Bond::default()
    };
    for token in &tokens[3..] {
        if !bond_attribute(&mut bond, token) {
            graph.skipped += 1;
        }
    }
    bond.shape = Shape::of(&bond.kind);
    graph.bonds.push(bond);
}

/// One `key=value` or one flag on a piece. `false` means nobody recognised it.
fn attribute(node: &mut Node, token: &str) -> bool {
    match token {
        "origin" => {
            node.origin = true;
            return true;
        }
        "slot" => {
            node.slot = true;
            return true;
        }
        _ => {}
    }
    let Some((key, value)) = token.split_once('=') else {
        return false;
    };
    match key {
        "label" => node.label = text(value),
        "meta" => node.meta = text(value),
        "terms" => node.terms = text(value),
        "kind" => node.kind = word(value),
        "tone" => node.tone = word(value),
        "shape" => node.shape = word(value),
        "set" => node.set = word(value),
        "cluster" => node.cluster = word(value),
        "image" => node.image = url(value),
        "href" => node.href = url(value),
        "rank" => node.rank = value.parse::<i32>().ok().map(|rank| rank.clamp(-999, 999)),
        "reach" => {
            // The same spellings `span` accepts; `reach=0` is a legal way to
            // say "touch me even in a shy scene".
            let plain = value.chars().all(|c| c.is_ascii_digit() || c == '.')
                && value.matches('.').count() <= 1
                && !value.is_empty();
            if plain {
                let cells = value.parse::<f64>().unwrap_or(0.0).clamp(0.0, 6.0);
                // Tenths, not half-cells — see `Node::reach`.
                #[allow(clippy::cast_possible_truncation)] // bounded by the clamp
                {
                    node.reach = (cells * 10.0).round() as i32;
                }
            }
        }
        "span" => {
            // Doubled, because the layout counts half-cells: a mark is half a
            // cell wide and a figure is one, and integers all the way through
            // are what keep the two implementations bit-identical. Halves are
            // legal — `span=1.5` is three half-cells — because the half-cell
            // IS the model's unit, and refusing it here is what once made a
            // magic kind word resize geometry instead.
            let plain = value.chars().all(|c| c.is_ascii_digit() || c == '.')
                && value.matches('.').count() <= 1
                && !value.is_empty();
            let cells = if plain {
                value.parse::<f64>().unwrap_or(1.0).clamp(0.5, 12.0)
            } else {
                1.0
            };
            #[allow(clippy::cast_possible_truncation)] // bounded by the clamp
            {
                node.span = (cells * 2.0).round() as i32;
            }
        }
        _ => return false,
    }
    true
}

/// One `key=value` on a bond. `false` means nobody recognised it.
fn bond_attribute(bond: &mut Bond, token: &str) -> bool {
    let Some((key, value)) = token.split_once('=') else {
        return false;
    };
    match key {
        "kind" => bond.kind = word(value),
        "label" => bond.label = text(value),
        "note" => bond.note = text(value),
        "tone" => bond.tone = word(value),
        "dash" => bond.dash = word(value),
        "route" => bond.route = word(value),
        "set" => bond.set = word(value),
        _ => return false,
    }
    true
}

/// An identifier, or nothing.
fn identifier(value: &str) -> Option<String> {
    let ok = !value.is_empty()
        && value.len() <= ID_LIMIT
        && value
            .chars()
            .all(|ch| ch.is_ascii_alphanumeric() || ch == '_' || ch == '-');
    ok.then(|| value.to_owned())
}

/// A word that reaches the DOM as an attribute value and a CSS selector.
///
/// `:` is allowed because a set is `group:name`; nothing else is, for the
/// reason the module documentation gives. An unacceptable word becomes the
/// empty string rather than a rejected line: the piece is still worth drawing.
fn word(value: &str) -> String {
    let ok = !value.is_empty()
        && value.len() <= ID_LIMIT
        && value
            .chars()
            .all(|ch| ch.is_ascii_alphanumeric() || ch == '_' || ch == '-' || ch == ':');
    if ok { value.to_owned() } else { String::new() }
}

/// Reader-facing text. Escaped by both frameworks on the way out, so the only
/// limit here is length: a label nobody can read is a label that broke a
/// layout.
fn text(value: &str) -> String {
    value.chars().take(160).collect()
}

/// A URL, or nothing.
///
/// Relative, or `http`/`https` absolute. Everything else — `javascript:`,
/// `data:`, `vbscript:`, a scheme nobody has heard of — is dropped, because
/// this string may have come from a consumer's database and lands in an `href`
/// or a `src`. Fail closed.
fn url(value: &str) -> String {
    let lower = value.to_ascii_lowercase();
    let absolute = lower.starts_with("https://") || lower.starts_with("http://");
    let scheme_ish = value
        .split_once(':')
        .is_some_and(|(head, _)| !head.contains('/'));
    if value.is_empty() || (scheme_ish && !absolute) {
        String::new()
    } else {
        value.to_owned()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_quoted_label_keeps_its_spaces_and_its_hash() {
        let graph = parse(r#"figure ada label="Ada # Doe" meta="1815–1852""#);
        assert_eq!(graph.nodes.len(), 1);
        assert_eq!(graph.nodes[0].label, "Ada # Doe");
        assert_eq!(graph.nodes[0].meta, "1815–1852");
        assert_eq!(graph.skipped, 0);
    }

    #[test]
    fn a_comment_and_a_blank_line_are_not_skipped_lines() {
        // `skipped` is what the DOM reports to an author, so counting the
        // comments would cry wolf on every well-written graph.
        let graph = parse("# a comment\n\n   \nfigure ada\n");
        assert_eq!(graph.nodes.len(), 1);
        assert_eq!(graph.skipped, 0);
    }

    #[test]
    fn no_identifier_can_carry_anything_but_a_word() {
        // THE INJECTION THIS EXISTS FOR. Every id is interpolated into a custom
        // property NAME and a `style` attribute, so a `;` or a `)` in one would
        // end the declaration and begin another. The claim is not that these
        // lines are refused — a quote is a delimiter, so `a"b` becomes the
        // perfectly safe `ab` — it is that **nothing but a word ever reaches an
        // id**, whichever way the line was malformed.
        for hostile in [
            "figure a);color:red;--x:(",
            "figure a\"b",
            "figure a b",
            "figure ",
            "figure url(evil)",
            "bond a);x:( b",
            "bond a b);x:(",
        ] {
            let graph = parse(hostile);
            for id in graph
                .nodes
                .iter()
                .map(|node| &node.id)
                .chain(graph.bonds.iter().flat_map(|bond| [&bond.from, &bond.to]))
            {
                assert!(
                    !id.is_empty()
                        && id
                            .chars()
                            .all(|ch| ch.is_ascii_alphanumeric() || ch == '_' || ch == '-'),
                    "`{hostile}` produced the id `{id}`"
                );
            }
        }
    }

    #[test]
    fn a_line_that_says_nothing_is_counted() {
        for hostile in ["figure a);color:red;--x:(", "figure ", "bond a);x:( b"] {
            let graph = parse(hostile);
            assert_eq!(graph.skipped, 1, "`{hostile}` was not counted");
        }
    }

    #[test]
    fn a_javascript_url_is_dropped_and_a_relative_one_is_kept() {
        let graph = parse(
            "figure a href=javascript:alert(1)\n\
             figure b href=https://example.test/x image=../p.avif\n\
             figure c image=data:image/svg+xml;base64,AAAA",
        );
        assert_eq!(graph.nodes[0].href, "");
        assert_eq!(graph.nodes[1].href, "https://example.test/x");
        assert_eq!(graph.nodes[1].image, "../p.avif");
        assert_eq!(graph.nodes[2].image, "");
    }

    #[test]
    fn an_unknown_attribute_is_counted_rather_than_swallowed() {
        // A prop that does nothing is a defect; so is an attribute that does
        // nothing. The count is stamped on the field, so a typo is visible.
        let graph = parse("figure ada labl=\"Ada\"");
        assert_eq!(graph.nodes.len(), 1);
        assert_eq!(graph.nodes[0].label, "");
        assert_eq!(graph.skipped, 1);
    }

    #[test]
    fn a_bond_to_itself_is_not_a_bond() {
        let graph = parse("bond ada ada");
        assert!(graph.bonds.is_empty(), "{:?}", graph.bonds);
        assert_eq!(graph.skipped, 1);
    }

    #[test]
    fn an_unterminated_quote_keeps_the_rest_of_the_line() {
        let graph = parse("figure ada label=\"Ada Doe");
        assert_eq!(graph.nodes[0].label, "Ada Doe");
    }

    #[test]
    fn a_mark_is_half_a_cell_and_a_figure_is_one() {
        let graph = parse("figure ada\nmark u1\nfigure wide span=3");
        assert_eq!(graph.nodes[0].span, 2);
        assert_eq!(graph.nodes[1].span, 1);
        assert_eq!(graph.nodes[2].span, 6);
    }
}
