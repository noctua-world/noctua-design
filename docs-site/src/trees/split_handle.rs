//! `SplitHandle`, as trees.
//!
//! One entry per example in `examples::SPLIT_HANDLE`, in the same order. The
//! caption is repeated here so the pairing is pinned to something a
//! reader can check: `every_example_has_a_tree` reads it against
//! the table's own, and a tree that slid a row apart fails there rather
//! than drawing the wrong picture.

use noctua_design_i18n::Key;

use crate::tree::{Node, Value};
use crate::trees::Tree;

/// Every `SplitHandle` example. 4 of them.
pub(crate) const EXAMPLES: &[Tree] = &[
    (
        r#"SplitHandle { name: "…", label: "…" }"#,
        &[Node::Component {
            name: "Split",
            props: &[("name", Value::Text("handle-basic"))],
            children: &[
                Node::Component {
                    name: "SplitPane",
                    props: &[("name", Value::Text("handle-basic"))],
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
                Node::Component {
                    name: "SplitHandle",
                    props: &[
                        ("name", Value::Text("handle-basic")),
                        ("label", Value::Message(Key::ExampleSplitHandle)),
                    ],
                    children: &[],
                },
                Node::Component {
                    name: "SplitPane",
                    props: &[
                        ("name", Value::Text("handle-basic")),
                        ("side", Value::Text("end")),
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
        r#"SplitHandle { … } in a Split { value: "20", min: "20", … }"#,
        &[Node::Component {
            name: "Split",
            props: &[
                ("name", Value::Text("handle-least")),
                ("value", Value::Text("20")),
                ("min", Value::Text("20")),
            ],
            children: &[
                Node::Component {
                    name: "SplitPane",
                    props: &[("name", Value::Text("handle-least"))],
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
                Node::Component {
                    name: "SplitHandle",
                    props: &[
                        ("name", Value::Text("handle-least")),
                        ("label", Value::Message(Key::ExampleSplitHandle)),
                    ],
                    children: &[],
                },
                Node::Component {
                    name: "SplitPane",
                    props: &[
                        ("name", Value::Text("handle-least")),
                        ("side", Value::Text("end")),
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
        r#"SplitHandle { … } in a Split { min: "0", max: "100", … }"#,
        &[Node::Component {
            name: "Split",
            props: &[
                ("name", Value::Text("handle-away")),
                ("min", Value::Text("0")),
                ("max", Value::Text("100")),
            ],
            children: &[
                Node::Component {
                    name: "SplitPane",
                    props: &[("name", Value::Text("handle-away"))],
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
                Node::Component {
                    name: "SplitHandle",
                    props: &[
                        ("name", Value::Text("handle-away")),
                        ("label", Value::Message(Key::ExampleSplitHandle)),
                    ],
                    children: &[],
                },
                Node::Component {
                    name: "SplitPane",
                    props: &[
                        ("name", Value::Text("handle-away")),
                        ("side", Value::Text("end")),
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
        r"SplitHandle { … } in a split read from the right",
        &[Node::Element {
            tag: "div",
            attributes: &[
                (
                    "class",
                    Value::Text("nd-example-stack nd-example-stack-wide"),
                ),
                ("dir", Value::Text("rtl")),
            ],
            children: &[Node::Component {
                name: "Split",
                props: &[
                    ("name", Value::Text("handle-mirrored")),
                    ("value", Value::Text("30")),
                ],
                children: &[
                    Node::Component {
                        name: "SplitPane",
                        props: &[("name", Value::Text("handle-mirrored"))],
                        children: &[Node::Element {
                            tag: "div",
                            attributes: &[("class", Value::Text("nd-example-pane"))],
                            children: &[
                                Node::Component {
                                    name: "Paragraph",
                                    props: &[],
                                    children: &[Node::Text(Value::Message(
                                        Key::ExampleRailCommands,
                                    ))],
                                },
                                Node::Component {
                                    name: "Paragraph",
                                    props: &[],
                                    children: &[Node::Text(Value::Message(
                                        Key::ExampleRailSettings,
                                    ))],
                                },
                                Node::Component {
                                    name: "Paragraph",
                                    props: &[],
                                    children: &[Node::Text(Value::Message(
                                        Key::ExampleRailRegistries,
                                    ))],
                                },
                            ],
                        }],
                    },
                    Node::Component {
                        name: "SplitHandle",
                        props: &[
                            ("name", Value::Text("handle-mirrored")),
                            ("label", Value::Message(Key::ExampleSplitHandle)),
                        ],
                        children: &[],
                    },
                    Node::Component {
                        name: "SplitPane",
                        props: &[
                            ("name", Value::Text("handle-mirrored")),
                            ("side", Value::Text("end")),
                        ],
                        children: &[Node::Element {
                            tag: "div",
                            attributes: &[("class", Value::Text("nd-example-pane"))],
                            children: &[Node::Component {
                                name: "Paragraph",
                                props: &[],
                                children: &[Node::Text(Value::Message(
                                    Key::ExampleRailCommandsBody,
                                ))],
                            }],
                        }],
                    },
                ],
            }],
        }],
    ),
];
