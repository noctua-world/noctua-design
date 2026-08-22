//! `SearchResult`, as trees.
//!
//! One entry per example in `examples::SEARCH_RESULT`, in the same order. The
//! caption is repeated here so the pairing is pinned to something a
//! reader can check: `every_example_has_a_tree` reads it against
//! the table's own, and a tree that slid a row apart fails there rather
//! than drawing the wrong picture.

use noctua_design_i18n::Key;

use crate::tree::{Node, Value};
use crate::trees::Tree;

/// Every `SearchResult` example. 5 of them.
pub(crate) const EXAMPLES: &[Tree] = &[
    (
        r#"SearchResult { href: "…", label: "…" }"#,
        &[Node::Component {
            name: "Search",
            props: &[
                ("name", Value::Text("result-plain")),
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
                        ("href", Value::Text("#cherokee-purple")),
                        ("label", Value::Data("Cherokee Purple")),
                    ],
                    children: &[],
                },
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
    ),
    (
        r#"SearchResult { detail: "…" }"#,
        &[Node::Component {
            name: "Search",
            props: &[
                ("name", Value::Text("result-detail")),
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
                        ("href", Value::Text("#genovese")),
                        ("label", Value::Data("Genovese")),
                        ("detail", Value::Message(Key::ExampleSearchGenovese)),
                    ],
                    children: &[],
                },
                Node::Component {
                    name: "SearchResult",
                    props: &[
                        ("class", Value::Text("nd-identifier")),
                        ("href", Value::Text("#dark-opal")),
                        ("label", Value::Data("Dark Opal")),
                        ("detail", Value::Message(Key::ExampleSearchOpal)),
                    ],
                    children: &[],
                },
            ],
        }],
    ),
    (
        r#"SearchResult { keywords: "heirloom slicing beefsteak" }"#,
        &[Node::Component {
            name: "Search",
            props: &[
                ("name", Value::Text("result-keywords")),
                ("value", Value::Data("heir")),
                ("label", Value::Message(Key::ExampleSearchLabel)),
                ("placeholder", Value::Message(Key::ExampleSearchPlaceholder)),
                ("status", Value::Message(Key::SearchResults)),
                ("empty_label", Value::Message(Key::SearchEmpty)),
                ("clear_label", Value::Message(Key::SearchClear)),
            ],
            children: &[
                Node::Component {
                    name: "SearchResult",
                    props: &[
                        ("class", Value::Text("nd-identifier")),
                        ("href", Value::Text("#cherokee-purple")),
                        ("label", Value::Data("Cherokee Purple")),
                        ("detail", Value::Message(Key::ExampleSearchCherokee)),
                        ("keywords", Value::Text("heirloom slicing beefsteak")),
                    ],
                    children: &[],
                },
                Node::Component {
                    name: "SearchResult",
                    props: &[
                        ("class", Value::Text("nd-identifier")),
                        ("href", Value::Text("#sungold")),
                        ("label", Value::Data("Sungold")),
                        ("detail", Value::Message(Key::ExampleSearchSungold)),
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
                        ("badge", Value::Message(Key::ExampleSearchBadge)),
                    ],
                    children: &[],
                },
                Node::Component {
                    name: "SearchResult",
                    props: &[
                        ("class", Value::Text("nd-identifier")),
                        ("href", Value::Text("#genovese")),
                        ("label", Value::Data("Genovese")),
                        ("current", Value::Bool(true)),
                    ],
                    children: &[],
                },
            ],
        }],
    ),
    (
        r"SearchResult { ai_mark, ai_label }",
        &[Node::Component {
            name: "Search",
            props: &[
                ("name", Value::Text("result-plain-ai")),
                ("label", Value::Message(Key::ExampleSearchLabel)),
                ("placeholder", Value::Message(Key::ExampleSearchPlaceholder)),
                ("status", Value::Message(Key::SearchResults)),
                ("empty_label", Value::Message(Key::SearchEmpty)),
            ],
            children: &[
                Node::Component {
                    name: "SearchResult",
                    props: &[
                        ("ai_mark", Value::Message(Key::ExampleAiMark)),
                        ("ai_label", Value::Message(Key::ExampleAiName)),
                        ("class", Value::Text("nd-identifier")),
                        ("href", Value::Text("#cherokee-purple")),
                        ("label", Value::Data("Cherokee Purple")),
                    ],
                    children: &[],
                },
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
    ),
];
