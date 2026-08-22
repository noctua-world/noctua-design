//! `TreeItem`, as trees.
//!
//! One entry per example in `examples::TREE_ITEM`, in the same order. The
//! caption is repeated here so the pairing is pinned to something a
//! reader can check: `every_example_has_a_tree` reads it against
//! the table's own, and a tree that slid a row apart fails there rather
//! than drawing the wrong picture.

use noctua_design_i18n::Key;

use crate::tree::{Node, Value};
use crate::trees::Tree;

/// Every `TreeItem` example. 5 of them.
pub(crate) const EXAMPLES: &[Tree] = &[
    (
        r#"TreeItem { label: "…", href: "…", icon, icon_expanded, badge: "…" }"#,
        &[Node::Component {
            name: "Tree",
            props: &[
                ("name", Value::Text("rows")),
                ("label", Value::Message(Key::ExampleTreeLabel)),
                ("expand_label", Value::Message(Key::ExampleTreeExpand)),
                ("collapse_label", Value::Message(Key::ExampleTreeCollapse)),
            ],
            children: &[Node::Component {
                name: "TreeItem",
                props: &[
                    ("label", Value::Message(Key::ExampleTreeFoundations)),
                    ("icon", Value::Text("palette")),
                    ("icon_expanded", Value::Text("check")),
                    ("badge", Value::Data("3")),
                    ("expanded", Value::Bool(true)),
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
                ],
            }],
        }],
    ),
    (
        r"TreeItem { expanded: true } / TreeItem { branch: true }",
        &[Node::Component {
            name: "Tree",
            props: &[
                ("name", Value::Text("branches")),
                ("label", Value::Message(Key::ExampleTreeLabel)),
                ("expand_label", Value::Message(Key::ExampleTreeExpand)),
                ("collapse_label", Value::Message(Key::ExampleTreeCollapse)),
            ],
            children: &[
                Node::Component {
                    name: "TreeItem",
                    props: &[
                        ("label", Value::Message(Key::ExampleTreeControls)),
                        ("expanded", Value::Bool(true)),
                    ],
                    children: &[Node::Component {
                        name: "TreeItem",
                        props: &[
                            ("class", Value::Text("nd-identifier")),
                            ("href", Value::Text("../button/")),
                            ("label", Value::Data("Button")),
                        ],
                        children: &[],
                    }],
                },
                Node::Component {
                    name: "TreeItem",
                    props: &[
                        ("label", Value::Message(Key::ExampleTreeFoundations)),
                        ("expanded", Value::Bool(false)),
                    ],
                    children: &[Node::Component {
                        name: "TreeItem",
                        props: &[
                            ("href", Value::Text("../spacing/")),
                            ("label", Value::PageTitle("spacing")),
                        ],
                        children: &[],
                    }],
                },
                Node::Component {
                    name: "TreeItem",
                    props: &[
                        ("label", Value::Message(Key::ExampleTreeEverything)),
                        ("branch", Value::Bool(true)),
                        ("href", Value::Text("../search/")),
                    ],
                    children: &[],
                },
            ],
        }],
    ),
    (
        r"TreeItem { current: true } / TreeItem { disabled: true } / TreeItem { selected: true }",
        &[Node::Component {
            name: "Tree",
            props: &[
                ("name", Value::Text("states")),
                ("selection", Value::Text("multiple")),
                ("label", Value::Message(Key::ExampleTreeLabel)),
                ("expand_label", Value::Message(Key::ExampleTreeExpand)),
                ("collapse_label", Value::Message(Key::ExampleTreeCollapse)),
            ],
            children: &[
                Node::Component {
                    name: "TreeItem",
                    props: &[
                        ("class", Value::Text("nd-identifier")),
                        ("value", Value::Text("tag")),
                        ("label", Value::Data("Tag")),
                        ("current", Value::Bool(true)),
                    ],
                    children: &[],
                },
                Node::Component {
                    name: "TreeItem",
                    props: &[
                        ("class", Value::Text("nd-identifier")),
                        ("value", Value::Text("button")),
                        ("label", Value::Data("Button")),
                        ("selected", Value::Bool(true)),
                    ],
                    children: &[],
                },
                Node::Component {
                    name: "TreeItem",
                    props: &[
                        ("class", Value::Text("nd-identifier")),
                        ("value", Value::Text("select")),
                        ("label", Value::Data("Select")),
                        ("disabled", Value::Bool(true)),
                    ],
                    children: &[],
                },
            ],
        }],
    ),
    (
        r#"TreeItem { value: "…" }"#,
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
                        ("name", Value::Text("values")),
                        ("selection", Value::Text("multiple")),
                        ("form", Value::Text("slug")),
                        ("label", Value::Message(Key::ExampleTreeLabel)),
                        ("expand_label", Value::Message(Key::ExampleTreeExpand)),
                        ("collapse_label", Value::Message(Key::ExampleTreeCollapse)),
                    ],
                    children: &[
                        Node::Component {
                            name: "TreeItem",
                            props: &[
                                ("value", Value::Text("tag")),
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
                    ],
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
        r"TreeItem { ai_mark, ai_label }",
        &[Node::Component {
            name: "Tree",
            props: &[
                ("name", Value::Text("rows-ai")),
                ("label", Value::Message(Key::ExampleTreeLabel)),
                ("expand_label", Value::Message(Key::ExampleTreeExpand)),
                ("collapse_label", Value::Message(Key::ExampleTreeCollapse)),
            ],
            children: &[Node::Component {
                name: "TreeItem",
                props: &[
                    ("ai_mark", Value::Message(Key::ExampleAiMark)),
                    ("ai_label", Value::Message(Key::ExampleAiName)),
                    ("label", Value::Message(Key::ExampleTreeFoundations)),
                    ("icon", Value::Text("palette")),
                    ("icon_expanded", Value::Text("check")),
                    ("badge", Value::Data("3")),
                    ("expanded", Value::Bool(true)),
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
                ],
            }],
        }],
    ),
];
