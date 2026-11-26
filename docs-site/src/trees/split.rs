//! `Split`, as trees.
//!
//! One entry per example in `examples::SPLIT`, in the same order. The
//! caption is repeated here so the pairing is pinned to something a
//! reader can check: `every_example_has_a_tree` reads it against
//! the table's own, and a tree that slid a row apart fails there rather
//! than drawing the wrong picture.

use noctua_design_i18n::Key;

use crate::tree::{Node, Value};
use crate::trees::Tree;

/// Every `Split` example. 5 of them.
pub(crate) const EXAMPLES: &[Tree] = &[
    (
        r#"Split { name: "…", SplitPane { … } SplitHandle { label: "…" } SplitPane { side: SplitPaneSide::End, … } }"#,
        &[Node::Component {
            name: "Split",
            props: &[("name", Value::Text("split-basic"))],
            children: &[
                Node::Component {
                    name: "SplitPane",
                    props: &[("name", Value::Text("split-basic"))],
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
                        ("name", Value::Text("split-basic")),
                        ("label", Value::Message(Key::ExampleSplitHandle)),
                    ],
                    children: &[],
                },
                Node::Component {
                    name: "SplitPane",
                    props: &[
                        ("name", Value::Text("split-basic")),
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
        r#"Split { value: "30", min: "20", max: "60", … }"#,
        &[Node::Component {
            name: "Split",
            props: &[
                ("name", Value::Text("split-limits")),
                ("value", Value::Text("30")),
                ("min", Value::Text("20")),
                ("max", Value::Text("60")),
            ],
            children: &[
                Node::Component {
                    name: "SplitPane",
                    props: &[("name", Value::Text("split-limits"))],
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
                        ("name", Value::Text("split-limits")),
                        ("label", Value::Message(Key::ExampleSplitHandle)),
                    ],
                    children: &[],
                },
                Node::Component {
                    name: "SplitPane",
                    props: &[
                        ("name", Value::Text("split-limits")),
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
        r"Split { collapse: SplitCollapse::Stack, … } in a narrow column",
        &[Node::Element {
            tag: "div",
            attributes: &[("class", Value::Text("nd-example-narrow"))],
            children: &[Node::Component {
                name: "Split",
                props: &[
                    ("name", Value::Text("split-stack")),
                    ("collapse", Value::Text("stack")),
                ],
                children: &[
                    Node::Component {
                        name: "SplitPane",
                        props: &[("name", Value::Text("split-stack"))],
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
                            ("name", Value::Text("split-stack")),
                            ("label", Value::Message(Key::ExampleSplitHandle)),
                        ],
                        children: &[],
                    },
                    Node::Component {
                        name: "SplitPane",
                        props: &[
                            ("name", Value::Text("split-stack")),
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
    // TWO COLUMNS, EACH A CHILD OF THE STAGE. A split is a query container and
    // has no width of its content's, so a row that sized itself from two of
    // them would be as wide as its own gap.
    (
        r"Split { collapse: SplitCollapse::HideStart, … } / Split { collapse: SplitCollapse::HideEnd, … } in a narrow column",
        &[
            Node::Element {
                tag: "div",
                attributes: &[("class", Value::Text("nd-example-narrow"))],
                children: &[Node::Component {
                    name: "Split",
                    props: &[
                        ("name", Value::Text("split-hide-start")),
                        ("collapse", Value::Text("hide-start")),
                    ],
                    children: &[
                        Node::Component {
                            name: "SplitPane",
                            props: &[("name", Value::Text("split-hide-start"))],
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
                                ("name", Value::Text("split-hide-start")),
                                ("label", Value::Message(Key::ExampleSplitHandle)),
                            ],
                            children: &[],
                        },
                        Node::Component {
                            name: "SplitPane",
                            props: &[
                                ("name", Value::Text("split-hide-start")),
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
            },
            Node::Element {
                tag: "div",
                attributes: &[("class", Value::Text("nd-example-narrow"))],
                children: &[Node::Component {
                    name: "Split",
                    props: &[
                        ("name", Value::Text("split-hide-end")),
                        ("collapse", Value::Text("hide-end")),
                    ],
                    children: &[
                        Node::Component {
                            name: "SplitPane",
                            props: &[("name", Value::Text("split-hide-end"))],
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
                                ("name", Value::Text("split-hide-end")),
                                ("label", Value::Message(Key::ExampleSplitHandle)),
                            ],
                            children: &[],
                        },
                        Node::Component {
                            name: "SplitPane",
                            props: &[
                                ("name", Value::Text("split-hide-end")),
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
            },
        ],
    ),
    (
        r"Split { collapse: SplitCollapse::None, … } in a narrow column",
        &[Node::Element {
            tag: "div",
            attributes: &[("class", Value::Text("nd-example-narrow"))],
            children: &[Node::Component {
                name: "Split",
                props: &[
                    ("name", Value::Text("split-none")),
                    ("collapse", Value::Text("none")),
                ],
                children: &[
                    Node::Component {
                        name: "SplitPane",
                        props: &[("name", Value::Text("split-none"))],
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
                            ("name", Value::Text("split-none")),
                            ("label", Value::Message(Key::ExampleSplitHandle)),
                        ],
                        children: &[],
                    },
                    Node::Component {
                        name: "SplitPane",
                        props: &[
                            ("name", Value::Text("split-none")),
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
