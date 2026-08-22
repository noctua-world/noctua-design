//! `Toolbar`, as trees.
//!
//! One entry per example in `examples::TOOLBAR`, in the same order. The
//! caption is repeated here so the pairing is pinned to something a
//! reader can check: `every_example_has_a_tree` reads it against
//! the table's own, and a tree that slid a row apart fails there rather
//! than drawing the wrong picture.

use noctua_design_i18n::Key;

use crate::tree::{Each, Node, Value};
use crate::trees::Tree;

/// The pair of buttons every toolbar example fills a group with.
///
/// Nine groups across this file and `toolbar_group.rs` draw it, and
/// `examples.rs` drew them from one macro so the copies could not drift. A
/// fragment keeps exactly that.
pub(crate) const CONTROLS: &[Node] = &[
    Node::Component {
        name: "Button",
        props: &[
            ("variant", Value::Text("transparent")),
            ("tone", Value::Text("neutral")),
            ("size", Value::Text("small")),
            ("hide_label", Value::Bool(true)),
            ("icon", Value::Text("search")),
            ("label", Value::Message(Key::SearchLabel)),
        ],
        children: &[],
    },
    Node::Component {
        name: "Button",
        props: &[
            ("variant", Value::Text("transparent")),
            ("tone", Value::Text("neutral")),
            ("size", Value::Text("small")),
            ("hide_label", Value::Bool(true)),
            ("icon", Value::Text("copy")),
            ("label", Value::Message(Key::ExampleCopy)),
        ],
        children: &[],
    },
];

/// Every `Toolbar` example. 5 of them.
pub(crate) const EXAMPLES: &[Tree] = &[
    (
        r#"Toolbar { label: "…", ToolbarGroup { … } }"#,
        &[Node::Component {
            name: "Toolbar",
            props: &[("label", Value::Message(Key::ExampleToolbarLabel))],
            children: &[
                Node::Component {
                    name: "ToolbarGroup",
                    props: &[],
                    children: &[Node::Fragment(CONTROLS)],
                },
                Node::Component {
                    name: "ToolbarGroup",
                    props: &[("align", Value::Text("end"))],
                    children: &[
                        Node::Component {
                            name: "Segmented",
                            props: &[
                                ("name", Value::Text("example-toolbar-view")),
                                ("size", Value::Text("small")),
                                ("label", Value::Message(Key::ExampleToolbarView)),
                            ],
                            children: &[Node::Each {
                                over: Each::Rows(&[
                                    &[
                                        Value::Text("light"),
                                        Value::Message(Key::ThemeModeLight),
                                        Value::Bool(true),
                                    ],
                                    &[
                                        Value::Text("dark"),
                                        Value::Message(Key::ThemeModeDark),
                                        Value::Bool(false),
                                    ],
                                ]),
                                body: &[Node::Component {
                                    name: "SegmentedItem",
                                    props: &[
                                        ("value", Value::Bound(0)),
                                        ("label", Value::Bound(1)),
                                        ("selected", Value::Bound(2)),
                                    ],
                                    children: &[],
                                }],
                            }],
                        },
                        Node::Component {
                            name: "Button",
                            props: &[
                                ("variant", Value::Text("transparent")),
                                ("tone", Value::Text("neutral")),
                                ("size", Value::Text("small")),
                                ("label", Value::Message(Key::ExampleReset)),
                            ],
                            children: &[],
                        },
                    ],
                },
            ],
        }],
    ),
    (
        r"Toolbar { orientation: ToolbarOrientation::Vertical, … }",
        &[Node::Component {
            name: "Toolbar",
            props: &[
                ("label", Value::Message(Key::ExampleToolbarLabel)),
                ("orientation", Value::Text("vertical")),
            ],
            children: &[Node::Component {
                name: "ToolbarGroup",
                props: &[],
                children: &[Node::Fragment(CONTROLS)],
            }],
        }],
    ),
    (
        r"Toolbar { density: ToolbarDensity::Compact, wrap: false, … }",
        &[Node::Element {
            tag: "div",
            attributes: &[("class", Value::Text("nd-example-stack"))],
            children: &[Node::Each {
                over: Each::Values {
                    component: "Toolbar",
                    prop: "density",
                },
                body: &[Node::Component {
                    name: "Toolbar",
                    props: &[
                        ("density", Value::Bound(0)),
                        ("wrap", Value::Bool(false)),
                        ("label", Value::Message(Key::ExampleToolbarLabel)),
                    ],
                    children: &[
                        Node::Component {
                            name: "ToolbarGroup",
                            props: &[],
                            children: &[Node::Fragment(CONTROLS)],
                        },
                        Node::Component {
                            name: "ToolbarGroup",
                            props: &[],
                            children: &[Node::Fragment(CONTROLS)],
                        },
                    ],
                }],
            }],
        }],
    ),
    (
        r#"Toolbar { labelledby: "…", … }"#,
        &[Node::Element {
            tag: "div",
            attributes: &[("class", Value::Text("nd-example-stack"))],
            children: &[
                Node::Component {
                    name: "Label",
                    props: &[
                        ("kind", Value::Text("name")),
                        ("for_control", Value::Text("example-toolbar")),
                        ("text", Value::Message(Key::ExampleToolbarLabel)),
                    ],
                    children: &[],
                },
                Node::Component {
                    name: "Toolbar",
                    props: &[("labelledby", Value::Text("example-toolbar-label"))],
                    children: &[Node::Component {
                        name: "ToolbarGroup",
                        props: &[],
                        children: &[Node::Fragment(CONTROLS)],
                    }],
                },
            ],
        }],
    ),
    (
        r"Toolbar { glass: true, … }",
        &[Node::Component {
            name: "Toolbar",
            props: &[
                ("glass", Value::Bool(true)),
                ("label", Value::Message(Key::ExampleToolbarLabel)),
            ],
            children: &[Node::Component {
                name: "ToolbarGroup",
                props: &[],
                children: &[Node::Fragment(CONTROLS)],
            }],
        }],
    ),
];
