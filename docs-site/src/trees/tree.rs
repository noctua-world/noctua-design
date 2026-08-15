//! `Tree`, as trees.
//!
//! One entry per example in `examples::TREE`, in the same order. The
//! caption is repeated here so the pairing is pinned to something a
//! reader can check: `every_tree_belongs_to_its_example` reads it against
//! the table's own, and a tree that slid a row apart fails there rather
//! than drawing the wrong picture.

use noctua_design_i18n::Key;

use crate::tree::{Each, Node, Value};
use crate::trees::Tree;

// THE TREE IS THIS SITE'S OWN PAGES, which is the only hierarchy on hand that
// is real — a tree of invented folders shows the indent and hides everything
// else. Every leaf is a link that goes somewhere, so the claim that the row is
// a link and the `<li>` is the node is checkable by following one.

/// The nodes every tree example shows.
///
/// One fragment rather than two copies, for the reason the macro it replaces
/// existed: two examples show the same branches, and two copies stop agreeing.
const BRANCHES: &[Node] = &[
    Node::Component {
        name: "TreeItem",
        props: &[
            ("label", Value::Message(Key::ExampleTreeFoundations)),
            ("icon", Value::Text("palette")),
            ("expanded", Value::Bool(true)),
            ("badge", Value::Data("3")),
        ],
        children: &[
            Node::Component {
                name: "TreeItem",
                props: &[
                    ("href", Value::Text("../typography/")),
                    ("label", Value::PageTitle("typography")),
                ],
                children: &[],
            },
            Node::Component {
                name: "TreeItem",
                props: &[
                    ("href", Value::Text("../spacing/")),
                    ("label", Value::PageTitle("spacing")),
                ],
                children: &[],
            },
            Node::Component {
                name: "TreeItem",
                props: &[
                    ("href", Value::Text("../icons/")),
                    ("label", Value::PageTitle("icons")),
                ],
                children: &[],
            },
        ],
    },
    Node::Component {
        name: "TreeItem",
        props: &[
            ("label", Value::Message(Key::ExampleTreeControls)),
            ("icon", Value::Text("settings")),
            ("expanded", Value::Bool(true)),
        ],
        children: &[
            Node::Component {
                name: "TreeItem",
                props: &[
                    ("class", Value::Text("nd-identifier")),
                    ("href", Value::Text("../button/")),
                    ("label", Value::Data("Button")),
                ],
                children: &[],
            },
            Node::Component {
                name: "TreeItem",
                props: &[
                    ("class", Value::Text("nd-identifier")),
                    ("href", Value::Text("../select/")),
                    ("label", Value::Data("Select")),
                ],
                children: &[],
            },
            Node::Component {
                name: "TreeItem",
                props: &[
                    ("class", Value::Text("nd-identifier")),
                    ("label", Value::Data("Tag")),
                ],
                children: &[
                    Node::Component {
                        name: "TreeItem",
                        props: &[
                            ("class", Value::Text("nd-identifier")),
                            ("href", Value::Text("../tag/")),
                            ("label", Value::Data("Tag")),
                            ("current", Value::Bool(true)),
                        ],
                        children: &[],
                    },
                    Node::Component {
                        name: "TreeItem",
                        props: &[
                            ("class", Value::Text("nd-identifier")),
                            ("href", Value::Text("../tag-picker/")),
                            ("label", Value::Data("TagPicker")),
                        ],
                        children: &[],
                    },
                ],
            },
        ],
    },
    // Children that were not passed: the lazily-loaded shape. Nothing
    // is hidden, because nothing is there — the link is where the rest
    // of it lives.
    Node::Component {
        name: "TreeItem",
        props: &[
            ("label", Value::Message(Key::ExampleTreeEverything)),
            ("icon", Value::Text("search")),
            ("branch", Value::Bool(true)),
            ("href", Value::Text("../search/")),
        ],
        children: &[],
    },
];

/// The nodes a SELECTABLE tree shows: names and values, and no links.
///
/// Choosing and going somewhere are two answers to one click, and a row that
/// does both does both — the browser follows the link and the node is chosen on
/// a page the reader has just left. The contract says so; this is what saying
/// so looks like at a call site.
const CHOICES: &[Node] = &[
    Node::Component {
        name: "TreeItem",
        props: &[
            ("label", Value::Message(Key::ExampleTreeFoundations)),
            ("icon", Value::Text("palette")),
            ("expanded", Value::Bool(true)),
        ],
        children: &[
            Node::Component {
                name: "TreeItem",
                props: &[
                    ("value", Value::Text("typography")),
                    ("label", Value::PageTitle("typography")),
                ],
                children: &[],
            },
            Node::Component {
                name: "TreeItem",
                props: &[
                    ("value", Value::Text("spacing")),
                    ("label", Value::PageTitle("spacing")),
                ],
                children: &[],
            },
            Node::Component {
                name: "TreeItem",
                props: &[
                    ("value", Value::Text("icons")),
                    ("label", Value::PageTitle("icons")),
                ],
                children: &[],
            },
        ],
    },
    Node::Component {
        name: "TreeItem",
        props: &[
            ("label", Value::Message(Key::ExampleTreeControls)),
            ("icon", Value::Text("settings")),
            ("expanded", Value::Bool(true)),
        ],
        children: &[
            Node::Component {
                name: "TreeItem",
                props: &[
                    ("class", Value::Text("nd-identifier")),
                    ("value", Value::Text("button")),
                    ("label", Value::Data("Button")),
                ],
                children: &[],
            },
            Node::Component {
                name: "TreeItem",
                props: &[
                    ("class", Value::Text("nd-identifier")),
                    ("value", Value::Text("select")),
                    ("label", Value::Data("Select")),
                ],
                children: &[],
            },
        ],
    },
];

/// Every `Tree` example. 4 of them.
pub(crate) const EXAMPLES: &[Tree] = &[
    (
        r#"Tree { name: "…", label: "…", expand_label: "…", collapse_label: "…", empty_label: "…" }"#,
        &[Node::Component {
            name: "Tree",
            props: &[
                ("name", Value::Text("pages")),
                ("label", Value::Message(Key::ExampleTreeLabel)),
                ("expand_label", Value::Message(Key::ExampleTreeExpand)),
                ("collapse_label", Value::Message(Key::ExampleTreeCollapse)),
                ("empty_label", Value::Message(Key::ExampleTreeEmpty)),
            ],
            children: &[Node::Fragment(BRANCHES)],
        }],
    ),
    (
        r"Tree { guides: true, size: TreeSize::… }",
        &[Node::Element {
            tag: "div",
            attributes: &[(
                "class",
                Value::Text("nd-example-stack nd-example-stack-wide"),
            )],
            children: &[Node::Each {
                over: Each::Values {
                    component: "Tree",
                    prop: "size",
                },
                body: &[Node::Component {
                    name: "Tree",
                    props: &[
                        ("size", Value::Bound(0)),
                        ("guides", Value::Bool(true)),
                        (
                            "name",
                            Value::Join(&[Value::Text("guides-"), Value::Bound(0)]),
                        ),
                        ("label", Value::Message(Key::ExampleTreeLabel)),
                        ("expand_label", Value::Message(Key::ExampleTreeExpand)),
                        ("collapse_label", Value::Message(Key::ExampleTreeCollapse)),
                    ],
                    children: &[Node::Fragment(BRANCHES)],
                }],
            }],
        }],
    ),
    (
        r#"Tree { selection: TreeSelection::Multiple, form: "…" } inside a <form>"#,
        &[Node::Element {
            tag: "form",
            attributes: &[
                ("class", Value::Text("nd-example-form")),
                ("method", Value::Text("get")),
            ],
            children: &[
                Node::Component {
                    name: "Tree",
                    props: &[
                        ("name", Value::Text("chosen")),
                        ("selection", Value::Text("multiple")),
                        ("form", Value::Text("page")),
                        ("label", Value::Message(Key::ExampleTreeLabel)),
                        ("expand_label", Value::Message(Key::ExampleTreeExpand)),
                        ("collapse_label", Value::Message(Key::ExampleTreeCollapse)),
                    ],
                    children: &[Node::Fragment(CHOICES)],
                },
                Node::Component {
                    name: "Button",
                    props: &[
                        ("submit", Value::Text("submit")),
                        ("label", Value::Message(Key::ExampleSelectSend)),
                    ],
                    children: &[],
                },
            ],
        }],
    ),
    (
        r"Tree { selection: TreeSelection::Single }",
        &[Node::Component {
            name: "Tree",
            props: &[
                ("name", Value::Text("one")),
                ("selection", Value::Text("single")),
                ("label", Value::Message(Key::ExampleTreeLabel)),
                ("expand_label", Value::Message(Key::ExampleTreeExpand)),
                ("collapse_label", Value::Message(Key::ExampleTreeCollapse)),
            ],
            children: &[Node::Fragment(CHOICES)],
        }],
    ),
];
