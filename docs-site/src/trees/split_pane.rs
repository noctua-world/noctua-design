//! `SplitPane`, as trees.
//!
//! One entry per example in `examples::SPLIT_PANE`, in the same order. The
//! caption is repeated here so the pairing is pinned to something a
//! reader can check: `every_example_has_a_tree` reads it against
//! the table's own, and a tree that slid a row apart fails there rather
//! than drawing the wrong picture.

use noctua_design_i18n::Key;

use crate::tree::{Node, Value};
use crate::trees::Tree;

/// Every `SplitPane` example. 4 of them.
pub(crate) const EXAMPLES: &[Tree] = &[
    (
        r#"SplitPane { name: "…", … } / SplitPane { name: "…", side: SplitPaneSide::End, … }"#,
        &[Node::Component {
            name: "Split",
            props: &[
                ("name", Value::Text("pane-sides")),
                ("value", Value::Text("30")),
            ],
            children: &[
                Node::Component {
                    name: "SplitPane",
                    props: &[("name", Value::Text("pane-sides"))],
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
                    name: "SplitPane",
                    props: &[
                        ("name", Value::Text("pane-sides")),
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
        r"SplitPane { … } with more in it than the split is tall",
        &[Node::Element {
            tag: "div",
            attributes: &[("class", Value::Text("nd-example-window"))],
            children: &[Node::Component {
                name: "Split",
                props: &[
                    ("name", Value::Text("pane-scroll")),
                    ("value", Value::Text("35")),
                ],
                children: &[
                    Node::Component {
                        name: "SplitPane",
                        props: &[("name", Value::Text("pane-scroll"))],
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
                            ("name", Value::Text("pane-scroll")),
                            ("label", Value::Message(Key::ExampleSplitHandle)),
                        ],
                        children: &[],
                    },
                    Node::Component {
                        name: "SplitPane",
                        props: &[
                            ("name", Value::Text("pane-scroll")),
                            ("side", Value::Text("end")),
                        ],
                        children: &[Node::Element {
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
                        }],
                    },
                ],
            }],
        }],
    ),
    (
        r"SplitPane { Heading { … } Paragraph { … } }",
        &[Node::Component {
            name: "Split",
            props: &[
                ("name", Value::Text("pane-content")),
                ("value", Value::Text("40")),
            ],
            children: &[
                Node::Component {
                    name: "SplitPane",
                    props: &[("name", Value::Text("pane-content"))],
                    children: &[Node::Element {
                        tag: "div",
                        attributes: &[("class", Value::Text("nd-example-pane"))],
                        children: &[
                            Node::Component {
                                name: "Heading",
                                props: &[("level", Value::Text("3")), ("size", Value::Text("h4"))],
                                children: &[Node::Text(Value::Message(Key::ExampleRailCommands))],
                            },
                            Node::Component {
                                name: "Paragraph",
                                props: &[],
                                children: &[Node::Text(Value::Message(
                                    Key::ExampleRailCommandsBody,
                                ))],
                            },
                        ],
                    }],
                },
                Node::Component {
                    name: "SplitHandle",
                    props: &[
                        ("name", Value::Text("pane-content")),
                        ("label", Value::Message(Key::ExampleSplitHandle)),
                    ],
                    children: &[],
                },
                Node::Component {
                    name: "SplitPane",
                    props: &[
                        ("name", Value::Text("pane-content")),
                        ("side", Value::Text("end")),
                    ],
                    children: &[Node::Element {
                        tag: "div",
                        attributes: &[("class", Value::Text("nd-example-pane"))],
                        children: &[
                            Node::Component {
                                name: "Heading",
                                props: &[("level", Value::Text("3")), ("size", Value::Text("h4"))],
                                children: &[Node::Text(Value::Message(Key::ExampleRailSettings))],
                            },
                            Node::Component {
                                name: "Paragraph",
                                props: &[],
                                children: &[Node::Text(Value::Message(
                                    Key::ExampleRailSettingsBody,
                                ))],
                            },
                        ],
                    }],
                },
            ],
        }],
    ),
    (
        r"SplitPane { side: SplitPaneSide::End, Split { … } }",
        &[Node::Component {
            name: "Split",
            props: &[
                ("name", Value::Text("pane-outer")),
                ("value", Value::Text("30")),
            ],
            children: &[
                Node::Component {
                    name: "SplitPane",
                    props: &[("name", Value::Text("pane-outer"))],
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
                        ("name", Value::Text("pane-outer")),
                        ("label", Value::Message(Key::ExampleSplitHandle)),
                    ],
                    children: &[],
                },
                Node::Component {
                    name: "SplitPane",
                    props: &[
                        ("name", Value::Text("pane-outer")),
                        ("side", Value::Text("end")),
                    ],
                    children: &[Node::Component {
                        name: "Split",
                        props: &[
                            ("name", Value::Text("pane-inner")),
                            ("value", Value::Text("40")),
                        ],
                        children: &[
                            Node::Component {
                                name: "SplitPane",
                                props: &[("name", Value::Text("pane-inner"))],
                                children: &[Node::Element {
                                    tag: "div",
                                    attributes: &[("class", Value::Text("nd-example-pane"))],
                                    children: &[Node::Component {
                                        name: "Paragraph",
                                        props: &[],
                                        children: &[Node::Text(Value::Message(
                                            Key::ExampleRailSettings,
                                        ))],
                                    }],
                                }],
                            },
                            Node::Component {
                                name: "SplitHandle",
                                props: &[
                                    ("name", Value::Text("pane-inner")),
                                    ("label", Value::Message(Key::ExampleSplitHandle)),
                                ],
                                children: &[],
                            },
                            Node::Component {
                                name: "SplitPane",
                                props: &[
                                    ("name", Value::Text("pane-inner")),
                                    ("side", Value::Text("end")),
                                ],
                                children: &[Node::Element {
                                    tag: "div",
                                    attributes: &[("class", Value::Text("nd-example-pane"))],
                                    children: &[Node::Component {
                                        name: "Paragraph",
                                        props: &[],
                                        children: &[Node::Text(Value::Message(
                                            Key::ExampleRailSettingsBody,
                                        ))],
                                    }],
                                }],
                            },
                        ],
                    }],
                },
            ],
        }],
    ),
];
