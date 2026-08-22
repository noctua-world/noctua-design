//! `MenuItem`, as trees.
//!
//! One entry per example in `examples::MENU_ITEM`, in the same order. The
//! caption is repeated here so the pairing is pinned to something a
//! reader can check: `every_example_has_a_tree` reads it against
//! the table's own, and a tree that slid a row apart fails there rather
//! than drawing the wrong picture.

use noctua_design_i18n::Key;

use crate::tree::{Node, Value};
use crate::trees::Tree;
use crate::trees::menu::menu_button;

/// Every `MenuItem` example. 4 of them.
pub(crate) const EXAMPLES: &[Tree] = &[
    (
        r#"MenuItem { icon, label, action: "…", value: "…" }"#,
        &[Node::Element {
            tag: "div",
            attributes: &[],
            children: &[
                menu_button!("example-item-icon"),
                Node::Component {
                    name: "Menu",
                    props: &[
                        ("name", Value::Text("example-item-icon")),
                        ("label", Value::Message(Key::ExampleMenuLabel)),
                    ],
                    children: &[
                        Node::Component {
                            name: "MenuItem",
                            props: &[
                                ("icon", Value::Text("copy")),
                                ("action", Value::Text("duplicate")),
                                ("value", Value::Text("invoice-43")),
                                ("label", Value::Message(Key::ExampleMenuDuplicate)),
                            ],
                            children: &[],
                        },
                        Node::Component {
                            name: "MenuItem",
                            props: &[
                                ("icon", Value::Text("download")),
                                ("action", Value::Text("archive")),
                                ("value", Value::Text("invoice-43")),
                                ("label", Value::Message(Key::ExampleMenuArchive)),
                            ],
                            children: &[],
                        },
                    ],
                },
            ],
        }],
    ),
    (
        r#"MenuItem { tone: "danger", detail: "…", action: "…" }"#,
        &[Node::Element {
            tag: "div",
            attributes: &[],
            children: &[
                menu_button!("example-item-tone"),
                Node::Component {
                    name: "Menu",
                    props: &[
                        ("name", Value::Text("example-item-tone")),
                        ("label", Value::Message(Key::ExampleMenuLabel)),
                    ],
                    children: &[
                        Node::Component {
                            name: "MenuItem",
                            props: &[("label", Value::Message(Key::ExampleMenuDuplicate))],
                            children: &[],
                        },
                        Node::Component {
                            name: "MenuItem",
                            props: &[
                                ("icon", Value::Text("close")),
                                ("tone", Value::Text("danger")),
                                // The hook the caller's own script binds to. This menu
                                // closes itself and returns the focus; what happens
                                // next is nobody else's business.
                                ("action", Value::Text("example-menu-delete")),
                                ("label", Value::Message(Key::ExampleMenuDelete)),
                                ("detail", Value::Message(Key::ExampleMenuDeleteDetail)),
                            ],
                            children: &[],
                        },
                    ],
                },
            ],
        }],
    ),
    (
        r"MenuItem { disabled: true }",
        &[Node::Element {
            tag: "div",
            attributes: &[],
            children: &[
                menu_button!("example-item-refused"),
                Node::Component {
                    name: "Menu",
                    props: &[
                        ("name", Value::Text("example-item-refused")),
                        ("label", Value::Message(Key::ExampleMenuLabel)),
                    ],
                    children: &[
                        Node::Component {
                            name: "MenuItem",
                            props: &[("label", Value::Message(Key::ExampleMenuDuplicate))],
                            children: &[],
                        },
                        Node::Component {
                            name: "MenuItem",
                            props: &[
                                ("disabled", Value::Bool(true)),
                                ("label", Value::Message(Key::ExampleMenuLocked)),
                            ],
                            children: &[],
                        },
                    ],
                },
            ],
        }],
    ),
    (
        r#"MenuItem { href: "…", rel, target }"#,
        &[Node::Element {
            tag: "div",
            attributes: &[],
            children: &[
                menu_button!("example-item-links"),
                Node::Component {
                    name: "Menu",
                    props: &[
                        ("name", Value::Text("example-item-links")),
                        ("label", Value::Message(Key::ExampleMenuLabel)),
                    ],
                    children: &[
                        Node::Component {
                            name: "MenuItem",
                            props: &[
                                ("href", Value::Text("#")),
                                ("label", Value::Message(Key::ExampleMenuDocs)),
                            ],
                            children: &[],
                        },
                        Node::Component {
                            name: "MenuItem",
                            props: &[
                                (
                                    "href",
                                    Value::Text("https://github.com/noctua-world/noctua-design"),
                                ),
                                ("target", Value::Text("_blank")),
                                ("icon", Value::Text("external-link")),
                                ("label", Value::Message(Key::ExampleMenuRepo)),
                            ],
                            children: &[],
                        },
                    ],
                },
            ],
        }],
    ),
];
