//! `SearchGroup`, as trees.
//!
//! One entry per example in `examples::SEARCH_GROUP`, in the same order. The
//! caption is repeated here so the pairing is pinned to something a
//! reader can check: `every_tree_belongs_to_its_example` reads it against
//! the table's own, and a tree that slid a row apart fails there rather
//! than drawing the wrong picture.

use noctua_design_i18n::Key;

use crate::tree::{Node, Value};
use crate::trees::Tree;
use crate::trees::search::CATALOGUE;

/// Every `SearchGroup` example. 4 of them.
pub(crate) const EXAMPLES: &[Tree] = &[
    (
        r#"SearchGroup { label: "…" }"#,
        &[Node::Component {
            name: "Search",
            props: &[
                ("name", Value::Text("group-one")),
                ("label", Value::Message(Key::ExampleSearchLabel)),
                ("placeholder", Value::Message(Key::ExampleSearchPlaceholder)),
                ("status", Value::Message(Key::SearchResults)),
                ("empty_label", Value::Message(Key::SearchEmpty)),
            ],
            children: &[Node::Component {
                name: "SearchGroup",
                props: &[("label", Value::Message(Key::ExampleSearchGroupTomatoes))],
                children: &[
                    Node::Component {
                        name: "SearchResult",
                        props: &[
                            ("class", Value::Text("nd-identifier")),
                            ("href", Value::Text("#sungold")),
                            ("label", Value::Data("Sungold")),
                        ],
                        children: &[],
                    },
                    Node::Component {
                        name: "SearchResult",
                        props: &[
                            ("class", Value::Text("nd-identifier")),
                            ("href", Value::Text("#san-marzano")),
                            ("label", Value::Data("San Marzano")),
                        ],
                        children: &[],
                    },
                ],
            }],
        }],
    ),
    (
        r"Two runs, one list",
        &[Node::Component {
            name: "Search",
            props: &[
                ("name", Value::Text("group-two")),
                ("label", Value::Message(Key::ExampleSearchLabel)),
                ("placeholder", Value::Message(Key::ExampleSearchPlaceholder)),
                ("status", Value::Message(Key::SearchResults)),
                ("empty_label", Value::Message(Key::SearchEmpty)),
            ],
            children: &[Node::Fragment(CATALOGUE)],
        }],
    ),
    (
        r"A run beside ungrouped results",
        &[Node::Component {
            name: "Search",
            props: &[
                ("name", Value::Text("group-mixed")),
                ("label", Value::Message(Key::ExampleSearchLabel)),
                ("placeholder", Value::Message(Key::ExampleSearchPlaceholder)),
                ("status", Value::Message(Key::SearchResults)),
                ("empty_label", Value::Message(Key::SearchEmpty)),
            ],
            children: &[
                Node::Component {
                    name: "SearchResult",
                    props: &[
                        ("class", Value::Text("nd-identifier")),
                        ("href", Value::Text("#lemon-gem")),
                        ("label", Value::Data("Lemon Gem")),
                        ("detail", Value::Message(Key::ExampleSearchGem)),
                    ],
                    children: &[],
                },
                Node::Component {
                    name: "SearchGroup",
                    props: &[("label", Value::Message(Key::ExampleSearchGroupTomatoes))],
                    children: &[
                        Node::Component {
                            name: "SearchResult",
                            props: &[
                                ("class", Value::Text("nd-identifier")),
                                ("href", Value::Text("#sungold")),
                                ("label", Value::Data("Sungold")),
                            ],
                            children: &[],
                        },
                        Node::Component {
                            name: "SearchResult",
                            props: &[
                                ("class", Value::Text("nd-identifier")),
                                ("href", Value::Text("#san-marzano")),
                                ("label", Value::Data("San Marzano")),
                            ],
                            children: &[],
                        },
                    ],
                },
            ],
        }],
    ),
    (
        r"An emptied group leaves nothing behind",
        &[Node::Component {
            name: "Search",
            props: &[
                ("name", Value::Text("group-empty")),
                ("value", Value::Data("okra")),
                ("label", Value::Message(Key::ExampleSearchLabel)),
                ("placeholder", Value::Message(Key::ExampleSearchPlaceholder)),
                ("status", Value::Message(Key::SearchResults)),
                ("empty_label", Value::Message(Key::SearchEmpty)),
                ("clear_label", Value::Message(Key::SearchClear)),
            ],
            children: &[Node::Fragment(CATALOGUE)],
        }],
    ),
];
