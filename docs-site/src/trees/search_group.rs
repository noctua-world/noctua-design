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
use crate::trees::search::PAGES;

/// Every `SearchGroup` example. 4 of them.
pub(crate) const EXAMPLES: &[Tree] = &[
    (
        r#"SearchGroup { label: "…" }"#,
        &[Node::Component {
            name: "Search",
            props: &[
                ("name", Value::Text("group-one")),
                ("label", Value::Message(Key::SearchLabel)),
                ("placeholder", Value::Message(Key::SearchPlaceholder)),
                ("status", Value::Message(Key::SearchResults)),
                ("empty_label", Value::Message(Key::SearchEmpty)),
            ],
            children: &[Node::Component {
                name: "SearchGroup",
                props: &[("label", Value::Message(Key::SearchGroupComponents))],
                children: &[
                    Node::Component {
                        name: "SearchResult",
                        props: &[
                            ("class", Value::Text("nd-identifier")),
                            ("href", Value::Text("../button/")),
                            ("label", Value::Data("Button")),
                        ],
                        children: &[],
                    },
                    Node::Component {
                        name: "SearchResult",
                        props: &[
                            ("class", Value::Text("nd-identifier")),
                            ("href", Value::Text("../select/")),
                            ("label", Value::Data("Select")),
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
                ("label", Value::Message(Key::SearchLabel)),
                ("placeholder", Value::Message(Key::SearchPlaceholder)),
                ("status", Value::Message(Key::SearchResults)),
                ("empty_label", Value::Message(Key::SearchEmpty)),
            ],
            children: &[Node::Fragment(PAGES)],
        }],
    ),
    (
        r"A run beside ungrouped results",
        &[Node::Component {
            name: "Search",
            props: &[
                ("name", Value::Text("group-mixed")),
                ("label", Value::Message(Key::SearchLabel)),
                ("placeholder", Value::Message(Key::SearchPlaceholder)),
                ("status", Value::Message(Key::SearchResults)),
                ("empty_label", Value::Message(Key::SearchEmpty)),
            ],
            children: &[
                Node::Component {
                    name: "SearchResult",
                    props: &[
                        ("href", Value::Text("../intro/")),
                        ("label", Value::PageTitle("intro")),
                        ("detail", Value::Message(Key::PageIntroSummary)),
                    ],
                    children: &[],
                },
                Node::Component {
                    name: "SearchGroup",
                    props: &[("label", Value::Message(Key::SearchGroupComponents))],
                    children: &[
                        Node::Component {
                            name: "SearchResult",
                            props: &[
                                ("class", Value::Text("nd-identifier")),
                                ("href", Value::Text("../button/")),
                                ("label", Value::Data("Button")),
                            ],
                            children: &[],
                        },
                        Node::Component {
                            name: "SearchResult",
                            props: &[
                                ("class", Value::Text("nd-identifier")),
                                ("href", Value::Text("../select/")),
                                ("label", Value::Data("Select")),
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
                ("value", Value::Data("sel")),
                ("label", Value::Message(Key::SearchLabel)),
                ("placeholder", Value::Message(Key::SearchPlaceholder)),
                ("status", Value::Message(Key::SearchResults)),
                ("empty_label", Value::Message(Key::SearchEmpty)),
                ("clear_label", Value::Message(Key::SearchClear)),
            ],
            children: &[Node::Fragment(PAGES)],
        }],
    ),
];
