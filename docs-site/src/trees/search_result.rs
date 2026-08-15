//! `SearchResult`, as trees.
//!
//! One entry per example in `examples::SEARCH_RESULT`, in the same order. The
//! caption is repeated here so the pairing is pinned to something a
//! reader can check: `every_tree_belongs_to_its_example` reads it against
//! the table's own, and a tree that slid a row apart fails there rather
//! than drawing the wrong picture.

use noctua_design_i18n::Key;

use crate::tree::{Node, Value};
use crate::trees::Tree;

/// Every `SearchResult` example. 4 of them.
pub(crate) const EXAMPLES: &[Tree] = &[
    (
        r#"SearchResult { href: "…", label: "…" }"#,
        &[Node::Component {
            name: "Search",
            props: &[
                ("name", Value::Text("result-plain")),
                ("label", Value::Message(Key::SearchLabel)),
                ("placeholder", Value::Message(Key::SearchPlaceholder)),
                ("status", Value::Message(Key::SearchResults)),
                ("empty_label", Value::Message(Key::SearchEmpty)),
            ],
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
                        ("href", Value::Text("../button-group/")),
                        ("label", Value::Data("ButtonGroup")),
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
    ),
    (
        r#"SearchResult { detail: "…" }"#,
        &[Node::Component {
            name: "Search",
            props: &[
                ("name", Value::Text("result-detail")),
                ("label", Value::Message(Key::SearchLabel)),
                ("placeholder", Value::Message(Key::SearchPlaceholder)),
                ("status", Value::Message(Key::SearchResults)),
                ("empty_label", Value::Message(Key::SearchEmpty)),
            ],
            children: &[
                Node::Component {
                    name: "SearchResult",
                    props: &[
                        ("href", Value::Text("../spacing/")),
                        ("label", Value::PageTitle("spacing")),
                        ("detail", Value::Message(Key::PageSpacingSummary)),
                    ],
                    children: &[],
                },
                Node::Component {
                    name: "SearchResult",
                    props: &[
                        ("href", Value::Text("../icons/")),
                        ("label", Value::PageTitle("icons")),
                        ("detail", Value::Message(Key::PageIconsSummary)),
                    ],
                    children: &[],
                },
            ],
        }],
    ),
    (
        r#"SearchResult { keywords: "gap inset step scale rhythm" }"#,
        &[Node::Component {
            name: "Search",
            props: &[
                ("name", Value::Text("result-keywords")),
                ("value", Value::Data("gap")),
                ("label", Value::Message(Key::SearchLabel)),
                ("placeholder", Value::Message(Key::SearchPlaceholder)),
                ("status", Value::Message(Key::SearchResults)),
                ("empty_label", Value::Message(Key::SearchEmpty)),
                ("clear_label", Value::Message(Key::SearchClear)),
            ],
            children: &[
                Node::Component {
                    name: "SearchResult",
                    props: &[
                        ("href", Value::Text("../spacing/")),
                        ("label", Value::PageTitle("spacing")),
                        ("detail", Value::Message(Key::PageSpacingSummary)),
                        ("keywords", Value::Text("gap inset step scale rhythm")),
                    ],
                    children: &[],
                },
                Node::Component {
                    name: "SearchResult",
                    props: &[
                        ("href", Value::Text("../typography/")),
                        ("label", Value::PageTitle("typography")),
                        ("detail", Value::Message(Key::PageTypographySummary)),
                    ],
                    children: &[],
                },
            ],
        }],
    ),
    (
        r#"SearchResult { badge: "…" } / SearchResult { current: true }"#,
        &[Node::Component {
            name: "Search",
            props: &[
                ("name", Value::Text("result-marked")),
                ("label", Value::Message(Key::SearchLabel)),
                ("placeholder", Value::Message(Key::SearchPlaceholder)),
                ("status", Value::Message(Key::SearchResults)),
                ("empty_label", Value::Message(Key::SearchEmpty)),
            ],
            children: &[
                Node::Component {
                    name: "SearchResult",
                    props: &[
                        ("href", Value::Text("../icons/")),
                        ("label", Value::PageTitle("icons")),
                        ("badge", Value::Message(Key::ExampleSearchBadge)),
                    ],
                    children: &[],
                },
                Node::Component {
                    name: "SearchResult",
                    props: &[
                        ("class", Value::Text("nd-identifier")),
                        ("href", Value::Text("../search/")),
                        ("label", Value::PageTitle("search")),
                        ("current", Value::Bool(true)),
                    ],
                    children: &[],
                },
            ],
        }],
    ),
];
