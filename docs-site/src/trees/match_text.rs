//! `MatchText`, as trees.
//!
//! One entry per example in `examples::MATCH_TEXT`, in the same order. The
//! caption is repeated here so the pairing is pinned to something a
//! reader can check: `every_example_has_a_tree` reads it against
//! the table's own, and a tree that slid a row apart fails there rather
//! than drawing the wrong picture.
//!
//! THE NAMES ARE DATA, NOT MESSAGES. A position is an index into the text, so
//! a name that changed with the locale would be marked on other letters in
//! each one. A component's name is the same in every language.

use crate::tree::{Node, Value};
use crate::trees::Tree;

/// Every `MatchText` example. 5 of them.
pub(crate) const EXAMPLES: &[Tree] = &[
    (
        r#"MatchText { text: "ButtonGroup", hits: "0-5" }"#,
        &[Node::Component {
            name: "MatchText",
            props: &[
                ("class", Value::Text("nd-identifier")),
                ("text", Value::Data("ButtonGroup")),
                ("hits", Value::Text("0-5")),
            ],
            children: &[],
        }],
    ),
    (
        r#"MatchText { text: "ButtonGroup", hits: "0-2|6-8|9-10" }"#,
        &[Node::Component {
            name: "MatchText",
            props: &[
                ("class", Value::Text("nd-identifier")),
                ("text", Value::Data("ButtonGroup")),
                ("hits", Value::Text("0-2|6-8|9-10")),
            ],
            children: &[],
        }],
    ),
    (
        r#"MatchText { text: "ButtonGroup", hits: "0-5|3-8" }"#,
        &[Node::Component {
            name: "MatchText",
            props: &[
                ("class", Value::Text("nd-identifier")),
                ("text", Value::Data("ButtonGroup")),
                ("hits", Value::Text("0-5|3-8")),
            ],
            children: &[],
        }],
    ),
    (
        r#"MatchText { text: "ButtonGroup", hits: "|6-10" }"#,
        &[Node::Component {
            name: "MatchText",
            props: &[
                ("class", Value::Text("nd-identifier")),
                ("text", Value::Data("ButtonGroup")),
                ("hits", Value::Text("|6-10")),
            ],
            children: &[],
        }],
    ),
    (
        r#"MatchText { text: "ButtonGroup" }"#,
        &[Node::Component {
            name: "MatchText",
            props: &[
                ("class", Value::Text("nd-identifier")),
                ("text", Value::Data("ButtonGroup")),
            ],
            children: &[],
        }],
    ),
];
