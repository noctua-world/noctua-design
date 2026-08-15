//! `Search`, as trees.
//!
//! One entry per example in `examples::SEARCH`, in the same order. The
//! caption is repeated here so the pairing is pinned to something a
//! reader can check: `every_tree_belongs_to_its_example` reads it against
//! the table's own, and a tree that slid a row apart fails there rather
//! than drawing the wrong picture.

use noctua_design_i18n::Key;

use crate::tree::{Each, Node, Value};
use crate::trees::Tree;

/// The results the children examples show, since they differ only in the props
/// on the box above them.
///
/// Real links to real pages: this is the half of the component that works with
/// the script blocked, and an example that faked it would be an example of
/// nothing. The page rows carry the page's own translated title and summary;
/// the component rows carry a name, which is the same word in every language
/// and says so with `nd-identifier`.
///
/// WHICH GROUP A ROW GOES IN FOLLOWS THAT, and `Search` moved because of it.
/// It sat among the pages with a translated label, back when `/search/` was
/// *Busca* in Portuguese — and `site::group_label` never agreed, because the
/// real index has always grouped it by `Kind::Component`. Its title is a name
/// now, so the example says what the index says.
///
/// One fragment rather than six copies, which is what the closures had as a
/// macro: six copies of a list are six chances for one of them to drift.
pub(super) const PAGES: &[Node] = &[
    Node::Component {
        name: "SearchGroup",
        props: &[("label", Value::Message(Key::SearchGroupPages))],
        children: &[
            Node::Component {
                name: "SearchResult",
                props: &[
                    ("href", Value::Text("../spacing/")),
                    ("label", Value::PageTitle("spacing")),
                    ("detail", Value::Message(Key::PageSpacingSummary)),
                    // Letters a reader might type that the name does not
                    // contain. `gap` reaches this row and nothing else.
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
            Node::Component {
                name: "SearchResult",
                props: &[
                    ("href", Value::Text("../intro/")),
                    ("label", Value::PageTitle("intro")),
                    ("detail", Value::Message(Key::PageIntroSummary)),
                ],
                children: &[],
            },
        ],
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
                    ("href", Value::Text("../button-group/")),
                    ("label", Value::Data("ButtonGroup")),
                ],
                children: &[],
            },
            Node::Component {
                name: "SearchResult",
                props: &[
                    ("class", Value::Text("nd-identifier")),
                    ("href", Value::Text("../checkbox/")),
                    ("label", Value::Data("Checkbox")),
                ],
                children: &[],
            },
            // The page this example is on, so `current` is a claim the
            // document can back rather than a decoration.
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
            Node::Component {
                name: "SearchResult",
                props: &[
                    ("class", Value::Text("nd-identifier")),
                    ("href", Value::Text("../segmented/")),
                    ("label", Value::Data("Segmented")),
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
            Node::Component {
                name: "SearchResult",
                props: &[
                    ("class", Value::Text("nd-identifier")),
                    ("href", Value::Text("../switch/")),
                    ("label", Value::Data("Switch")),
                ],
                children: &[],
            },
            Node::Component {
                name: "SearchResult",
                props: &[
                    ("class", Value::Text("nd-identifier")),
                    ("href", Value::Text("../tables/")),
                    ("label", Value::Data("Table")),
                ],
                children: &[],
            },
        ],
    },
];

/// Every `Search` example. 8 of them.
pub(crate) const EXAMPLES: &[Tree] = &[
    (
        r#"Search { name: "…", label: "…", placeholder: "…", status: "one:… |other:…" }"#,
        &[Node::Component {
            name: "Search",
            props: &[
                ("name", Value::Text("children")),
                ("label", Value::Message(Key::SearchLabel)),
                ("placeholder", Value::Message(Key::SearchPlaceholder)),
                ("status", Value::Message(Key::SearchResults)),
                ("empty_label", Value::Message(Key::SearchEmpty)),
                ("clear_label", Value::Message(Key::SearchClear)),
            ],
            children: &[Node::Fragment(PAGES)],
        }],
    ),
    (
        r#"Search { presentation: SearchPresentation::Popover, value: "but", cap: "3", cap_label: "…" }"#,
        &[Node::Component {
            name: "Search",
            props: &[
                ("name", Value::Text("popover")),
                ("presentation", Value::Text("popover")),
                ("label", Value::Message(Key::SearchLabel)),
                ("placeholder", Value::Message(Key::SearchPlaceholder)),
                ("value", Value::Data("but")),
                ("cap", Value::Text("3")),
                ("status", Value::Message(Key::SearchResults)),
                ("cap_label", Value::Message(Key::SearchShowing)),
                ("empty_label", Value::Message(Key::SearchEmpty)),
                ("clear_label", Value::Message(Key::SearchClear)),
            ],
            children: &[Node::Fragment(PAGES)],
        }],
    ),
    (
        r#"Search { presentation: SearchPresentation::Dialog, src: "…", hotkey: true, recent: "…", status, empty_label, loading_label, error_label, cap_label, clear_label, recent_label, forget_label, close_label }"#,
        &[Node::Component {
            name: "Search",
            props: &[
                ("name", Value::Text("palette")),
                ("presentation", Value::Text("dialog")),
                ("label", Value::Message(Key::SearchLabel)),
                ("placeholder", Value::Message(Key::SearchPlaceholder)),
                // The site's own index, fetched once and shared with the
                // sidebar's trigger: one file, whichever box asks for it.
                ("src", Value::Asset("assets/search-{locale}.json")),
                ("hotkey", Value::Bool(true)),
                ("cap", Value::Text("10")),
                ("recent", Value::Text("noctua.docs.recent")),
                ("status", Value::Message(Key::SearchResults)),
                ("empty_label", Value::Message(Key::SearchEmpty)),
                ("loading_label", Value::Message(Key::SearchLoading)),
                ("error_label", Value::Message(Key::SearchError)),
                ("cap_label", Value::Message(Key::SearchShowing)),
                ("clear_label", Value::Message(Key::SearchClear)),
                ("recent_label", Value::Message(Key::SearchRecent)),
                ("forget_label", Value::Message(Key::SearchForget)),
                ("close_label", Value::Message(Key::NavDismiss)),
            ],
            children: &[],
        }],
    ),
    (
        r#"Search { name, action: "…", query: "term", label, placeholder, clear_label }"#,
        &[Node::Component {
            name: "Search",
            props: &[
                ("name", Value::Text("form")),
                ("action", Value::Text("../search/")),
                // NOT `q`, WHICH IS THE DEFAULT. A field posting the name it
                // would have posted anyway is an example of nothing, and this
                // is the one prop on the page a server has to agree with.
                ("query", Value::Text("term")),
                ("label", Value::Message(Key::SearchLabel)),
                ("placeholder", Value::Message(Key::SearchPlaceholder)),
                ("clear_label", Value::Message(Key::SearchClear)),
            ],
            children: &[],
        }],
    ),
    (
        r"Search { filter: true }",
        // A `<section>` rather than a `<div>`, and that is what bounds the
        // filtering: the script hides the `data-nd-index` elements inside the
        // nearest landmark, so without one this box would reach every other
        // example on the page.
        &[Node::Element {
            tag: "section",
            attributes: &[("class", Value::Text("nd-example-filter"))],
            children: &[
                Node::Component {
                    name: "Search",
                    props: &[
                        ("name", Value::Text("filter")),
                        ("filter", Value::Bool(true)),
                        ("label", Value::Message(Key::SearchLabel)),
                        ("placeholder", Value::Message(Key::SearchPlaceholder)),
                        ("clear_label", Value::Message(Key::SearchClear)),
                    ],
                    children: &[],
                },
                Node::Element {
                    tag: "ul",
                    attributes: &[("class", Value::Text("nd-example-filtered"))],
                    children: &[Node::Each {
                        over: Each::Rows(&[
                            &[Value::Data("Button")],
                            &[Value::Data("ButtonGroup")],
                            &[Value::Data("Checkbox")],
                            &[Value::Data("Select")],
                            &[Value::Data("Switch")],
                            &[Value::Data("Table")],
                        ]),
                        body: &[Node::Element {
                            tag: "li",
                            attributes: &[("data-nd-index", Value::Bound(0))],
                            children: &[Node::Component {
                                name: "Code",
                                props: &[("class", Value::Text("nd-identifier"))],
                                children: &[Node::Text(Value::Bound(0))],
                            }],
                        }],
                    }],
                },
            ],
        }],
    ),
    (
        r"Search { size: SearchSize::Small } / Medium / Large",
        &[Node::Element {
            tag: "div",
            attributes: &[("class", Value::Text("nd-example-stack"))],
            children: &[
                Node::Component {
                    name: "Search",
                    props: &[
                        ("name", Value::Text("small")),
                        ("size", Value::Text("small")),
                        ("label", Value::Message(Key::SearchLabel)),
                        ("placeholder", Value::Message(Key::SearchPlaceholder)),
                    ],
                    children: &[],
                },
                Node::Component {
                    name: "Search",
                    props: &[
                        ("name", Value::Text("medium")),
                        ("label", Value::Message(Key::SearchLabel)),
                        ("placeholder", Value::Message(Key::SearchPlaceholder)),
                    ],
                    children: &[],
                },
                Node::Component {
                    name: "Search",
                    props: &[
                        ("name", Value::Text("large")),
                        ("size", Value::Text("large")),
                        ("label", Value::Message(Key::SearchLabel)),
                        ("placeholder", Value::Message(Key::SearchPlaceholder)),
                    ],
                    children: &[],
                },
            ],
        }],
    ),
    (
        r"Search { loading: true } / Search { disabled: true } / Search { clearable: false }",
        &[Node::Element {
            tag: "div",
            attributes: &[("class", Value::Text("nd-example-stack"))],
            children: &[
                Node::Component {
                    name: "Search",
                    props: &[
                        ("name", Value::Text("loading")),
                        ("loading", Value::Bool(true)),
                        ("label", Value::Message(Key::SearchLabel)),
                        ("placeholder", Value::Message(Key::SearchPlaceholder)),
                        ("loading_label", Value::Message(Key::SearchLoading)),
                        ("status", Value::Message(Key::SearchResults)),
                    ],
                    children: &[Node::Fragment(PAGES)],
                },
                Node::Component {
                    name: "Search",
                    props: &[
                        ("name", Value::Text("refused")),
                        ("disabled", Value::Bool(true)),
                        ("value", Value::Data("azure")),
                        ("label", Value::Message(Key::SearchLabel)),
                        ("placeholder", Value::Message(Key::SearchPlaceholder)),
                    ],
                    children: &[],
                },
                Node::Component {
                    name: "Search",
                    props: &[
                        ("name", Value::Text("bare")),
                        ("clearable", Value::Bool(false)),
                        ("label", Value::Message(Key::SearchLabel)),
                        ("placeholder", Value::Message(Key::SearchPlaceholder)),
                    ],
                    children: &[],
                },
            ],
        }],
    ),
    (
        r"Search { glass: true }",
        &[Node::Component {
            name: "Search",
            props: &[
                ("name", Value::Text("frosted")),
                ("glass", Value::Bool(true)),
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
