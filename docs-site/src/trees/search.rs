//! `Search`, as trees.
//!
//! One entry per example in `examples::SEARCH`, in the same order. The
//! caption is repeated here so the pairing is pinned to something a
//! reader can check: `every_example_has_a_tree` reads it against
//! the table's own, and a tree that slid a row apart fails there rather
//! than drawing the wrong picture.

use noctua_design_i18n::Key;

use crate::tree::{Each, Node, Value};
use crate::trees::Tree;

/// The results the children examples show, since they differ only in the props
/// on the box above them.
///
/// A SEED CATALOGUE, INVENTED, AND NOT THIS SITE. These rows used to be the
/// real pages and components of this documentation site, which made every
/// example on the page a picture of the page it was on. A reader could not tell
/// the demonstration from the application, and the one thing an example has to
/// do is be distinguishable from the thing around it.
///
/// WHAT IS TRANSLATED AND WHAT IS NOT. Every sentence here is a message in both
/// locales: the two group names and the line under each variety. The varieties
/// themselves are proper names, so they cross unchanged and say so with
/// `nd-identifier` — the same division the rest of this file already made, and
/// what lets a pre-filled query like `san` match in every language.
///
/// One fragment rather than six copies, which is what the closures had as a
/// macro: six copies of a list are six chances for one of them to drift.
pub(super) const CATALOGUE: &[Node] = &[
    Node::Component {
        name: "SearchGroup",
        props: &[("label", Value::Message(Key::ExampleSearchGroupTomatoes))],
        children: &[
            Node::Component {
                name: "SearchResult",
                props: &[
                    ("class", Value::Text("nd-identifier")),
                    ("href", Value::Text("#cherokee-purple")),
                    ("label", Value::Data("Cherokee Purple")),
                    ("detail", Value::Message(Key::ExampleSearchCherokee)),
                    // Letters a reader might type that the name does not
                    // contain. `heir` reaches this row and nothing else.
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
            Node::Component {
                name: "SearchResult",
                props: &[
                    ("class", Value::Text("nd-identifier")),
                    ("href", Value::Text("#san-marzano")),
                    ("label", Value::Data("San Marzano")),
                    ("detail", Value::Message(Key::ExampleSearchMarzano)),
                ],
                children: &[],
            },
        ],
    },
    Node::Component {
        name: "SearchGroup",
        props: &[("label", Value::Message(Key::ExampleSearchGroupHerbs))],
        children: &[
            // The variety whose own page this catalogue is standing on, so
            // `current` is a claim the example can back rather than a
            // decoration.
            Node::Component {
                name: "SearchResult",
                props: &[
                    ("class", Value::Text("nd-identifier")),
                    ("href", Value::Text("#genovese")),
                    ("label", Value::Data("Genovese")),
                    ("detail", Value::Message(Key::ExampleSearchGenovese)),
                    ("current", Value::Bool(true)),
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
        ],
    },
];

/// Every `Search` example. 9 of them.
pub(crate) const EXAMPLES: &[Tree] = &[
    (
        r#"Search { name: "…", label: "…", placeholder: "…", status: "one:… |other:…" }"#,
        &[Node::Component {
            name: "Search",
            props: &[
                ("name", Value::Text("children")),
                ("label", Value::Message(Key::ExampleSearchLabel)),
                ("placeholder", Value::Message(Key::ExampleSearchPlaceholder)),
                ("status", Value::Message(Key::SearchResults)),
                ("empty_label", Value::Message(Key::SearchEmpty)),
                ("clear_label", Value::Message(Key::SearchClear)),
            ],
            children: &[Node::Fragment(CATALOGUE)],
        }],
    ),
    (
        r#"Search { presentation: SearchPresentation::Popover, value: "san", cap: "3", cap_label: "…" }"#,
        &[Node::Component {
            name: "Search",
            props: &[
                ("name", Value::Text("popover")),
                ("presentation", Value::Text("popover")),
                ("label", Value::Message(Key::ExampleSearchLabel)),
                ("placeholder", Value::Message(Key::ExampleSearchPlaceholder)),
                ("value", Value::Data("san")),
                ("cap", Value::Text("3")),
                ("status", Value::Message(Key::SearchResults)),
                ("cap_label", Value::Message(Key::SearchShowing)),
                ("empty_label", Value::Message(Key::SearchEmpty)),
                ("clear_label", Value::Message(Key::SearchClear)),
            ],
            children: &[Node::Fragment(CATALOGUE)],
        }],
    ),
    (
        r#"Search { presentation: SearchPresentation::Dialog, src: "…", hotkey: true, recent: "…", status, empty_label, loading_label, error_label, cap_label, clear_label, recent_label, forget_label, close_label }"#,
        &[Node::Component {
            name: "Search",
            props: &[
                ("name", Value::Text("palette")),
                ("presentation", Value::Text("dialog")),
                ("label", Value::Message(Key::ExampleSearchLabel)),
                ("placeholder", Value::Message(Key::ExampleSearchPlaceholder)),
                // THE ONE EXAMPLE THAT IS STILL THIS SITE, AND `src` IS WHY.
                // What this prop does is FETCH an index over the network, and
                // the only index this build has to fetch is its own — a
                // fictional one would be a static file with no locale behind
                // it, which is the untranslated content the catalogue exists
                // to prevent. Nothing of it is rendered until a reader opens
                // the palette and types.
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
                ("label", Value::Message(Key::ExampleSearchLabel)),
                ("placeholder", Value::Message(Key::ExampleSearchPlaceholder)),
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
                        ("label", Value::Message(Key::ExampleSearchLabel)),
                        ("placeholder", Value::Message(Key::ExampleSearchPlaceholder)),
                        ("clear_label", Value::Message(Key::SearchClear)),
                    ],
                    children: &[],
                },
                Node::Element {
                    tag: "ul",
                    attributes: &[("class", Value::Text("nd-example-filtered"))],
                    children: &[Node::Each {
                        // The same catalogue as a plain list, because `filter`
                        // hides rows a caller wrote rather than rows the
                        // component drew. A variety is a proper name, so it is
                        // the same string a reader types in every locale.
                        over: Each::Rows(&[
                            &[Value::Data("Cherokee Purple")],
                            &[Value::Data("Sungold")],
                            &[Value::Data("San Marzano")],
                            &[Value::Data("Genovese")],
                            &[Value::Data("Dark Opal")],
                            &[Value::Data("Lemon Gem")],
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
                        ("label", Value::Message(Key::ExampleSearchLabel)),
                        ("placeholder", Value::Message(Key::ExampleSearchPlaceholder)),
                    ],
                    children: &[],
                },
                Node::Component {
                    name: "Search",
                    props: &[
                        ("name", Value::Text("medium")),
                        ("label", Value::Message(Key::ExampleSearchLabel)),
                        ("placeholder", Value::Message(Key::ExampleSearchPlaceholder)),
                    ],
                    children: &[],
                },
                Node::Component {
                    name: "Search",
                    props: &[
                        ("name", Value::Text("large")),
                        ("size", Value::Text("large")),
                        ("label", Value::Message(Key::ExampleSearchLabel)),
                        ("placeholder", Value::Message(Key::ExampleSearchPlaceholder)),
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
                        ("label", Value::Message(Key::ExampleSearchLabel)),
                        ("placeholder", Value::Message(Key::ExampleSearchPlaceholder)),
                        ("loading_label", Value::Message(Key::SearchLoading)),
                        ("status", Value::Message(Key::SearchResults)),
                    ],
                    children: &[Node::Fragment(CATALOGUE)],
                },
                Node::Component {
                    name: "Search",
                    props: &[
                        ("name", Value::Text("refused")),
                        ("disabled", Value::Bool(true)),
                        ("value", Value::Data("sungold")),
                        ("label", Value::Message(Key::ExampleSearchLabel)),
                        ("placeholder", Value::Message(Key::ExampleSearchPlaceholder)),
                    ],
                    children: &[],
                },
                Node::Component {
                    name: "Search",
                    props: &[
                        ("name", Value::Text("bare")),
                        ("clearable", Value::Bool(false)),
                        ("label", Value::Message(Key::ExampleSearchLabel)),
                        ("placeholder", Value::Message(Key::ExampleSearchPlaceholder)),
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
                ("label", Value::Message(Key::ExampleSearchLabel)),
                ("placeholder", Value::Message(Key::ExampleSearchPlaceholder)),
                ("status", Value::Message(Key::SearchResults)),
                ("empty_label", Value::Message(Key::SearchEmpty)),
                ("clear_label", Value::Message(Key::SearchClear)),
            ],
            children: &[Node::Fragment(CATALOGUE)],
        }],
    ),
    (
        r"Search { ai_mark, ai_label, ai_note }",
        &[Node::Component {
            name: "Search",
            props: &[
                ("ai_mark", Value::Message(Key::ExampleAiMark)),
                ("ai_label", Value::Message(Key::ExampleAiName)),
                ("ai_note", Value::Message(Key::ExampleAiNote)),
                ("name", Value::Text("children-ai")),
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
