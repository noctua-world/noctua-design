//! `Drawer`, as trees.
//!
//! One entry per example in `examples::DRAWER`, in the same order. The
//! caption is repeated here so the pairing is pinned to something a
//! reader can check: `every_example_has_a_tree` reads it against
//! the table's own, and a tree that slid a row apart fails there rather
//! than drawing the wrong picture.

use noctua_design_i18n::Key;

use crate::tree::{Node, Value};
use crate::trees::Tree;

/// Every `Drawer` example. 4 of them.
pub(crate) const EXAMPLES: &[Tree] = &[
    (
        r#"Drawer { name: "…", label: "…", … }"#,
        &[Node::Element {
            tag: "div",
            attributes: &[("class", Value::Text("nd-example-window"))],
            children: &[
                Node::Element {
                    tag: "div",
                    attributes: &[("class", Value::Text("nd-example-work"))],
                    children: &[Node::Component {
                        name: "Paragraph",
                        props: &[],
                        children: &[Node::Text(Value::Message(Key::ExampleDrawerWork))],
                    }],
                },
                Node::Component {
                    name: "Drawer",
                    props: &[
                        ("name", Value::Text("drawer-basic")),
                        ("label", Value::Message(Key::ExampleDrawerLabel)),
                    ],
                    children: &[Node::Element {
                        tag: "div",
                        attributes: &[("class", Value::Text("nd-example-pane"))],
                        children: &[Node::Component {
                            name: "Paragraph",
                            props: &[],
                            children: &[Node::Text(Value::Message(Key::ExampleRailCommandsBody))],
                        }],
                    }],
                },
            ],
        }],
    ),
    (
        r#"Drawer { id: "…", collapsed: true, … } beside Button { controls: "…" }"#,
        &[Node::Element {
            tag: "div",
            attributes: &[("class", Value::Text("nd-example-window"))],
            children: &[
                Node::Element {
                    tag: "div",
                    attributes: &[("class", Value::Text("nd-example-work"))],
                    children: &[
                        Node::Component {
                            name: "Paragraph",
                            props: &[],
                            children: &[Node::Text(Value::Message(Key::ExampleDrawerWorkShut))],
                        },
                        Node::Component {
                            name: "Button",
                            props: &[
                                ("variant", Value::Text("outline")),
                                ("tone", Value::Text("neutral")),
                                ("size", Value::Text("small")),
                                ("icon", Value::Text("panel-bottom")),
                                ("action", Value::Text("drawer-toggle")),
                                ("controls", Value::Text("drawer-shut")),
                                ("label", Value::Message(Key::ExampleDrawerToggle)),
                            ],
                            children: &[],
                        },
                    ],
                },
                Node::Component {
                    name: "Drawer",
                    props: &[
                        ("name", Value::Text("drawer-shut")),
                        ("id", Value::Text("drawer-shut")),
                        ("label", Value::Message(Key::ExampleDrawerLabel)),
                        ("collapsed", Value::Bool(true)),
                    ],
                    children: &[
                        Node::Component {
                            name: "DrawerRail",
                            props: &[("side", Value::Text("start"))],
                            children: &[Node::Component {
                                name: "Toolbar",
                                props: &[
                                    ("density", Value::Text("rail")),
                                    ("label", Value::Message(Key::ExampleRailControls)),
                                ],
                                children: &[Node::Component {
                                    name: "ToolbarGroup",
                                    props: &[],
                                    children: &[
                                        Node::Component {
                                            name: "Tooltip",
                                            props: &[
                                                ("name", Value::Text("drawer-shut-collapse")),
                                                ("text", Value::Message(Key::ExampleRailCollapse)),
                                            ],
                                            children: &[Node::Component {
                                                name: "Button",
                                                props: &[
                                                    ("variant", Value::Text("transparent")),
                                                    ("tone", Value::Text("neutral")),
                                                    ("size", Value::Text("small")),
                                                    ("hide_label", Value::Bool(true)),
                                                    ("icon", Value::Text("chevrons-down")),
                                                    (
                                                        "label",
                                                        Value::Message(Key::ExampleRailCollapse),
                                                    ),
                                                    ("action", Value::Text("drawer-toggle")),
                                                ],
                                                children: &[],
                                            }],
                                        },
                                        Node::Component {
                                            name: "Tooltip",
                                            props: &[
                                                ("name", Value::Text("drawer-shut-close")),
                                                ("text", Value::Message(Key::ExampleRailClose)),
                                            ],
                                            children: &[Node::Component {
                                                name: "Button",
                                                props: &[
                                                    ("variant", Value::Text("transparent")),
                                                    ("tone", Value::Text("neutral")),
                                                    ("size", Value::Text("small")),
                                                    ("hide_label", Value::Bool(true)),
                                                    ("icon", Value::Text("close")),
                                                    (
                                                        "label",
                                                        Value::Message(Key::ExampleRailClose),
                                                    ),
                                                ],
                                                children: &[],
                                            }],
                                        },
                                    ],
                                }],
                            }],
                        },
                        Node::Element {
                            tag: "div",
                            attributes: &[("class", Value::Text("nd-example-pane"))],
                            children: &[
                                Node::Component {
                                    name: "Paragraph",
                                    props: &[],
                                    children: &[Node::Text(Value::Message(
                                        Key::ExampleRailCommandsBody,
                                    ))],
                                },
                                Node::Component {
                                    name: "Paragraph",
                                    props: &[],
                                    children: &[Node::Text(Value::Message(
                                        Key::ExampleRailSettingsBody,
                                    ))],
                                },
                            ],
                        },
                    ],
                },
            ],
        }],
    ),
    (
        r"Drawer { side: DrawerSide::Start, … } / Drawer { side: DrawerSide::End, … }",
        &[Node::Element {
            tag: "div",
            attributes: &[(
                "class",
                Value::Text("nd-example-window nd-example-window-row"),
            )],
            children: &[
                Node::Component {
                    name: "Drawer",
                    props: &[
                        ("name", Value::Text("drawer-start")),
                        ("label", Value::Message(Key::ExampleDrawerSidebar)),
                        ("side", Value::Text("start")),
                    ],
                    children: &[Node::Element {
                        tag: "div",
                        attributes: &[("class", Value::Text("nd-example-pane"))],
                        children: &[
                            Node::Component {
                                name: "Paragraph",
                                props: &[],
                                children: &[Node::Text(Value::Message(Key::ExampleRailCommands))],
                            },
                            Node::Component {
                                name: "Paragraph",
                                props: &[],
                                children: &[Node::Text(Value::Message(Key::ExampleRailSettings))],
                            },
                            Node::Component {
                                name: "Paragraph",
                                props: &[],
                                children: &[Node::Text(Value::Message(Key::ExampleRailRegistries))],
                            },
                        ],
                    }],
                },
                Node::Element {
                    tag: "div",
                    attributes: &[("class", Value::Text("nd-example-work"))],
                    children: &[Node::Component {
                        name: "Paragraph",
                        props: &[],
                        children: &[Node::Text(Value::Message(Key::ExampleDrawerWorkSide))],
                    }],
                },
                Node::Component {
                    name: "Drawer",
                    props: &[
                        ("name", Value::Text("drawer-end")),
                        ("label", Value::Message(Key::ExampleDrawerDetails)),
                        ("side", Value::Text("end")),
                    ],
                    children: &[Node::Element {
                        tag: "div",
                        attributes: &[("class", Value::Text("nd-example-pane"))],
                        children: &[Node::Component {
                            name: "Paragraph",
                            props: &[],
                            children: &[Node::Text(Value::Message(Key::ExampleRailRegistriesBody))],
                        }],
                    }],
                },
            ],
        }],
    ),
    (
        r"Drawer { … } with more in it than its share of the window",
        &[Node::Element {
            tag: "div",
            attributes: &[("class", Value::Text("nd-example-window"))],
            children: &[
                Node::Element {
                    tag: "div",
                    attributes: &[("class", Value::Text("nd-example-work"))],
                    children: &[Node::Component {
                        name: "Paragraph",
                        props: &[],
                        children: &[Node::Text(Value::Message(Key::ExampleDrawerWork))],
                    }],
                },
                Node::Component {
                    name: "Drawer",
                    props: &[
                        ("name", Value::Text("drawer-long")),
                        ("label", Value::Message(Key::ExampleDrawerLabel)),
                    ],
                    children: &[
                        Node::Component {
                            name: "DrawerRail",
                            props: &[("side", Value::Text("start"))],
                            children: &[Node::Component {
                                name: "Toolbar",
                                props: &[
                                    ("density", Value::Text("rail")),
                                    ("label", Value::Message(Key::ExampleRailControls)),
                                ],
                                children: &[Node::Component {
                                    name: "ToolbarGroup",
                                    props: &[],
                                    children: &[
                                        Node::Component {
                                            name: "Tooltip",
                                            props: &[
                                                ("name", Value::Text("drawer-long-collapse")),
                                                ("text", Value::Message(Key::ExampleRailCollapse)),
                                            ],
                                            children: &[Node::Component {
                                                name: "Button",
                                                props: &[
                                                    ("variant", Value::Text("transparent")),
                                                    ("tone", Value::Text("neutral")),
                                                    ("size", Value::Text("small")),
                                                    ("hide_label", Value::Bool(true)),
                                                    ("icon", Value::Text("chevrons-down")),
                                                    (
                                                        "label",
                                                        Value::Message(Key::ExampleRailCollapse),
                                                    ),
                                                ],
                                                children: &[],
                                            }],
                                        },
                                        Node::Component {
                                            name: "Tooltip",
                                            props: &[
                                                ("name", Value::Text("drawer-long-close")),
                                                ("text", Value::Message(Key::ExampleRailClose)),
                                            ],
                                            children: &[Node::Component {
                                                name: "Button",
                                                props: &[
                                                    ("variant", Value::Text("transparent")),
                                                    ("tone", Value::Text("neutral")),
                                                    ("size", Value::Text("small")),
                                                    ("hide_label", Value::Bool(true)),
                                                    ("icon", Value::Text("close")),
                                                    (
                                                        "label",
                                                        Value::Message(Key::ExampleRailClose),
                                                    ),
                                                ],
                                                children: &[],
                                            }],
                                        },
                                    ],
                                }],
                            }],
                        },
                        Node::Element {
                            tag: "div",
                            attributes: &[("class", Value::Text("nd-example-pane"))],
                            children: &[
                                Node::Component {
                                    name: "Paragraph",
                                    props: &[],
                                    children: &[Node::Text(Value::Message(
                                        Key::ExampleRailCommandsBody,
                                    ))],
                                },
                                Node::Component {
                                    name: "Paragraph",
                                    props: &[],
                                    children: &[Node::Text(Value::Message(
                                        Key::ExampleRailSettingsBody,
                                    ))],
                                },
                                Node::Component {
                                    name: "Paragraph",
                                    props: &[],
                                    children: &[Node::Text(Value::Message(
                                        Key::ExampleRailRegistriesBody,
                                    ))],
                                },
                                Node::Component {
                                    name: "Paragraph",
                                    props: &[],
                                    children: &[Node::Text(Value::Message(
                                        Key::ExampleDrawerLongOne,
                                    ))],
                                },
                                Node::Component {
                                    name: "Paragraph",
                                    props: &[],
                                    children: &[Node::Text(Value::Message(
                                        Key::ExampleDrawerLongTwo,
                                    ))],
                                },
                                Node::Component {
                                    name: "Paragraph",
                                    props: &[],
                                    children: &[Node::Text(Value::Message(
                                        Key::ExampleDrawerLongThree,
                                    ))],
                                },
                            ],
                        },
                    ],
                },
            ],
        }],
    ),
];
