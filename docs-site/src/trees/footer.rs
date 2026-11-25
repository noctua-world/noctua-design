//! `Footer`, as trees.
//!
//! One entry per example in `examples::FOOTER`, in the same order. The
//! caption is repeated here so the pairing is pinned to something a
//! reader can check: `every_example_has_a_tree` reads it against
//! the table's own, and a tree that slid a row apart fails there rather
//! than drawing the wrong picture.

use noctua_design_i18n::Key;

use crate::tree::{Node, Value};
use crate::trees::Tree;

/// Every `Footer` example. 4 of them.
pub(crate) const EXAMPLES: &[Tree] = &[
    (
        r#"Footer { label: "…", FooterLine { … } FooterLine { … } FooterLine { … } }"#,
        &[Node::Component {
            name: "Footer",
            props: &[("label", Value::Message(Key::ExampleFooterLabel))],
            children: &[
                Node::Component {
                    name: "FooterLine",
                    props: &[("kind", Value::Text("modeline"))],
                    children: &[
                        Node::Component {
                            name: "Button",
                            props: &[
                                ("hide_label", Value::Bool(true)),
                                ("icon", Value::Text("panel-left")),
                                ("variant", Value::Text("transparent")),
                                ("tone", Value::Text("neutral")),
                                ("size", Value::Text("small")),
                                ("label", Value::Message(Key::ExampleFooterSidebar)),
                            ],
                            children: &[],
                        },
                        Node::Component {
                            name: "Icon",
                            props: &[
                                ("icon", Value::Text("file-text")),
                                ("size", Value::Text("small")),
                            ],
                            children: &[],
                        },
                        Node::Element {
                            tag: "span",
                            attributes: &[],
                            children: &[Node::Text(Value::Message(Key::ExampleFooterPage))],
                        },
                        Node::Element {
                            tag: "span",
                            attributes: &[("class", Value::Text("nd-example-end"))],
                            children: &[
                                Node::Element {
                                    tag: "span",
                                    attributes: &[("class", Value::Text("nd-identifier"))],
                                    children: &[Node::Text(Value::Data("home / ana"))],
                                },
                                Node::Component {
                                    name: "Icon",
                                    props: &[
                                        ("icon", Value::Text("wifi-off")),
                                        ("size", Value::Text("small")),
                                    ],
                                    children: &[],
                                },
                                Node::Element {
                                    tag: "span",
                                    attributes: &[],
                                    children: &[Node::Text(Value::Message(
                                        Key::ExampleFooterOffline,
                                    ))],
                                },
                            ],
                        },
                    ],
                },
                Node::Component {
                    name: "FooterLine",
                    props: &[("kind", Value::Text("info"))],
                    children: &[
                        Node::Component {
                            name: "Icon",
                            props: &[
                                ("icon", Value::Text("info")),
                                ("size", Value::Text("small")),
                            ],
                            children: &[],
                        },
                        Node::Component {
                            name: "Icon",
                            props: &[
                                ("icon", Value::Text("mouse-pointer")),
                                ("size", Value::Text("small")),
                            ],
                            children: &[],
                        },
                        Node::Element {
                            tag: "span",
                            attributes: &[],
                            children: &[Node::Text(Value::Message(Key::ExampleFooterHover))],
                        },
                    ],
                },
                Node::Component {
                    name: "FooterLine",
                    props: &[("kind", Value::Text("prompt"))],
                    children: &[
                        Node::Component {
                            name: "Icon",
                            props: &[
                                ("icon", Value::Text("chevron-right")),
                                ("size", Value::Text("small")),
                            ],
                            children: &[],
                        },
                        Node::Element {
                            tag: "span",
                            attributes: &[],
                            children: &[Node::Text(Value::Message(Key::ExampleFooterTyped))],
                        },
                    ],
                },
            ],
        }],
    ),
    (
        r"Footer { visibility: FooterVisibility::Auto, … }",
        &[Node::Component {
            name: "Footer",
            props: &[
                ("label", Value::Message(Key::ExampleFooterLabel)),
                ("visibility", Value::Text("auto")),
            ],
            children: &[
                Node::Component {
                    name: "FooterLine",
                    props: &[("kind", Value::Text("modeline"))],
                    children: &[
                        Node::Component {
                            name: "Button",
                            props: &[
                                ("hide_label", Value::Bool(true)),
                                ("icon", Value::Text("panel-left")),
                                ("variant", Value::Text("transparent")),
                                ("tone", Value::Text("neutral")),
                                ("size", Value::Text("small")),
                                ("label", Value::Message(Key::ExampleFooterSidebar)),
                            ],
                            children: &[],
                        },
                        Node::Component {
                            name: "Icon",
                            props: &[
                                ("icon", Value::Text("file-text")),
                                ("size", Value::Text("small")),
                            ],
                            children: &[],
                        },
                        Node::Element {
                            tag: "span",
                            attributes: &[],
                            children: &[Node::Text(Value::Message(Key::ExampleFooterPage))],
                        },
                        Node::Element {
                            tag: "span",
                            attributes: &[("class", Value::Text("nd-example-end"))],
                            children: &[
                                Node::Element {
                                    tag: "span",
                                    attributes: &[("class", Value::Text("nd-identifier"))],
                                    children: &[Node::Text(Value::Data("home / ana"))],
                                },
                                Node::Component {
                                    name: "Icon",
                                    props: &[
                                        ("icon", Value::Text("wifi-off")),
                                        ("size", Value::Text("small")),
                                    ],
                                    children: &[],
                                },
                                Node::Element {
                                    tag: "span",
                                    attributes: &[],
                                    children: &[Node::Text(Value::Message(
                                        Key::ExampleFooterOffline,
                                    ))],
                                },
                            ],
                        },
                    ],
                },
                Node::Component {
                    name: "FooterLine",
                    props: &[("kind", Value::Text("info"))],
                    children: &[
                        Node::Component {
                            name: "Icon",
                            props: &[
                                ("icon", Value::Text("info")),
                                ("size", Value::Text("small")),
                            ],
                            children: &[],
                        },
                        Node::Component {
                            name: "Icon",
                            props: &[
                                ("icon", Value::Text("mouse-pointer")),
                                ("size", Value::Text("small")),
                            ],
                            children: &[],
                        },
                        Node::Element {
                            tag: "span",
                            attributes: &[],
                            children: &[Node::Text(Value::Message(Key::ExampleFooterHover))],
                        },
                    ],
                },
                Node::Component {
                    name: "FooterLine",
                    props: &[("kind", Value::Text("prompt"))],
                    children: &[
                        Node::Component {
                            name: "Icon",
                            props: &[
                                ("icon", Value::Text("chevron-right")),
                                ("size", Value::Text("small")),
                            ],
                            children: &[],
                        },
                        Node::Element {
                            tag: "span",
                            attributes: &[],
                            children: &[Node::Text(Value::Message(Key::ExampleFooterTyped))],
                        },
                    ],
                },
            ],
        }],
    ),
    (
        r"Footer { visibility: FooterVisibility::Auto, shown: true, … }",
        &[Node::Component {
            name: "Footer",
            props: &[
                ("label", Value::Message(Key::ExampleFooterLabel)),
                ("visibility", Value::Text("auto")),
                ("shown", Value::Bool(true)),
            ],
            children: &[
                Node::Component {
                    name: "FooterLine",
                    props: &[("kind", Value::Text("modeline"))],
                    children: &[
                        Node::Component {
                            name: "Button",
                            props: &[
                                ("hide_label", Value::Bool(true)),
                                ("icon", Value::Text("panel-left")),
                                ("variant", Value::Text("transparent")),
                                ("tone", Value::Text("neutral")),
                                ("size", Value::Text("small")),
                                ("label", Value::Message(Key::ExampleFooterSidebar)),
                            ],
                            children: &[],
                        },
                        Node::Component {
                            name: "Icon",
                            props: &[
                                ("icon", Value::Text("file-text")),
                                ("size", Value::Text("small")),
                            ],
                            children: &[],
                        },
                        Node::Element {
                            tag: "span",
                            attributes: &[],
                            children: &[Node::Text(Value::Message(Key::ExampleFooterPage))],
                        },
                        Node::Element {
                            tag: "span",
                            attributes: &[("class", Value::Text("nd-example-end"))],
                            children: &[
                                Node::Element {
                                    tag: "span",
                                    attributes: &[("class", Value::Text("nd-identifier"))],
                                    children: &[Node::Text(Value::Data("home / ana"))],
                                },
                                Node::Component {
                                    name: "Icon",
                                    props: &[
                                        ("icon", Value::Text("wifi-off")),
                                        ("size", Value::Text("small")),
                                    ],
                                    children: &[],
                                },
                                Node::Element {
                                    tag: "span",
                                    attributes: &[],
                                    children: &[Node::Text(Value::Message(
                                        Key::ExampleFooterOffline,
                                    ))],
                                },
                            ],
                        },
                    ],
                },
                Node::Component {
                    name: "FooterLine",
                    props: &[("kind", Value::Text("info"))],
                    children: &[
                        Node::Component {
                            name: "Icon",
                            props: &[
                                ("icon", Value::Text("info")),
                                ("size", Value::Text("small")),
                            ],
                            children: &[],
                        },
                        Node::Component {
                            name: "Icon",
                            props: &[
                                ("icon", Value::Text("mouse-pointer")),
                                ("size", Value::Text("small")),
                            ],
                            children: &[],
                        },
                        Node::Element {
                            tag: "span",
                            attributes: &[],
                            children: &[Node::Text(Value::Message(Key::ExampleFooterHover))],
                        },
                    ],
                },
                Node::Component {
                    name: "FooterLine",
                    props: &[("kind", Value::Text("prompt"))],
                    children: &[
                        Node::Component {
                            name: "Icon",
                            props: &[
                                ("icon", Value::Text("chevron-right")),
                                ("size", Value::Text("small")),
                            ],
                            children: &[],
                        },
                        Node::Element {
                            tag: "span",
                            attributes: &[],
                            children: &[Node::Text(Value::Message(Key::ExampleFooterTyped))],
                        },
                    ],
                },
            ],
        }],
    ),
    (
        r#"Footer { label: "…", … } in a narrow window"#,
        &[Node::Element {
            tag: "div",
            attributes: &[("class", Value::Text("nd-example-narrow"))],
            children: &[Node::Component {
                name: "Footer",
                props: &[("label", Value::Message(Key::ExampleFooterLabel))],
                children: &[
                    Node::Component {
                        name: "FooterLine",
                        props: &[("kind", Value::Text("modeline"))],
                        children: &[
                            Node::Component {
                                name: "Button",
                                props: &[
                                    ("hide_label", Value::Bool(true)),
                                    ("icon", Value::Text("panel-left")),
                                    ("variant", Value::Text("transparent")),
                                    ("tone", Value::Text("neutral")),
                                    ("size", Value::Text("small")),
                                    ("label", Value::Message(Key::ExampleFooterSidebar)),
                                ],
                                children: &[],
                            },
                            Node::Component {
                                name: "Icon",
                                props: &[
                                    ("icon", Value::Text("file-text")),
                                    ("size", Value::Text("small")),
                                ],
                                children: &[],
                            },
                            Node::Element {
                                tag: "span",
                                attributes: &[],
                                children: &[Node::Text(Value::Message(Key::ExampleFooterPage))],
                            },
                            Node::Element {
                                tag: "span",
                                attributes: &[("class", Value::Text("nd-example-end"))],
                                children: &[
                                    Node::Element {
                                        tag: "span",
                                        attributes: &[("class", Value::Text("nd-identifier"))],
                                        children: &[Node::Text(Value::Data("home / ana"))],
                                    },
                                    Node::Component {
                                        name: "Icon",
                                        props: &[
                                            ("icon", Value::Text("wifi-off")),
                                            ("size", Value::Text("small")),
                                        ],
                                        children: &[],
                                    },
                                    Node::Element {
                                        tag: "span",
                                        attributes: &[],
                                        children: &[Node::Text(Value::Message(
                                            Key::ExampleFooterOffline,
                                        ))],
                                    },
                                ],
                            },
                        ],
                    },
                    Node::Component {
                        name: "FooterLine",
                        props: &[("kind", Value::Text("info"))],
                        children: &[
                            Node::Component {
                                name: "Icon",
                                props: &[
                                    ("icon", Value::Text("info")),
                                    ("size", Value::Text("small")),
                                ],
                                children: &[],
                            },
                            Node::Component {
                                name: "Icon",
                                props: &[
                                    ("icon", Value::Text("mouse-pointer")),
                                    ("size", Value::Text("small")),
                                ],
                                children: &[],
                            },
                            Node::Element {
                                tag: "span",
                                attributes: &[],
                                children: &[Node::Text(Value::Message(Key::ExampleFooterHover))],
                            },
                        ],
                    },
                    Node::Component {
                        name: "FooterLine",
                        props: &[("kind", Value::Text("prompt"))],
                        children: &[
                            Node::Component {
                                name: "Icon",
                                props: &[
                                    ("icon", Value::Text("chevron-right")),
                                    ("size", Value::Text("small")),
                                ],
                                children: &[],
                            },
                            Node::Element {
                                tag: "span",
                                attributes: &[],
                                children: &[Node::Text(Value::Message(Key::ExampleFooterTyped))],
                            },
                        ],
                    },
                ],
            }],
        }],
    ),
];
