//! `FooterCorner`, as trees.
//!
//! One entry per example in `examples::FOOTER_CORNER`, in the same order. The
//! caption is repeated here so the pairing is pinned to something a
//! reader can check: `every_example_has_a_tree` reads it against
//! the table's own, and a tree that slid a row apart fails there rather
//! than drawing the wrong picture.

use noctua_design_i18n::Key;

use crate::tree::{Node, Value};
use crate::trees::Tree;

/// Every `FooterCorner` example. 4 of them.
pub(crate) const EXAMPLES: &[Tree] = &[
    (
        r"FooterCorner { side: FooterCornerSide::Start } / FooterCorner { side: FooterCornerSide::End }",
        &[Node::Component {
            name: "Footer",
            props: &[("label", Value::Message(Key::ExampleFooterLabel))],
            children: &[
                Node::Component {
                    name: "FooterCorner",
                    props: &[("side", Value::Text("start"))],
                    children: &[],
                },
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
                Node::Component {
                    name: "FooterCorner",
                    props: &[("side", Value::Text("end"))],
                    children: &[],
                },
            ],
        }],
    ),
    (
        r#"FooterCorner { Button { hide_label: true, icon: …, label: "…" } }"#,
        &[Node::Component {
            name: "Footer",
            props: &[("label", Value::Message(Key::ExampleFooterLabel))],
            children: &[
                Node::Component {
                    name: "FooterCorner",
                    props: &[("side", Value::Text("start"))],
                    children: &[Node::Component {
                        name: "Button",
                        props: &[
                            ("hide_label", Value::Bool(true)),
                            ("icon", Value::Text("settings")),
                            ("variant", Value::Text("transparent")),
                            ("tone", Value::Text("neutral")),
                            ("label", Value::Message(Key::SettingsLabel)),
                        ],
                        children: &[],
                    }],
                },
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
        r"FooterCorner { side: FooterCornerSide::End } alone",
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
                Node::Component {
                    name: "FooterCorner",
                    props: &[("side", Value::Text("end"))],
                    children: &[],
                },
            ],
        }],
    ),
    (
        r"FooterCorner { Button { … } Button { … } }",
        &[Node::Component {
            name: "Footer",
            props: &[("label", Value::Message(Key::ExampleFooterLabel))],
            children: &[
                Node::Component {
                    name: "FooterCorner",
                    props: &[("side", Value::Text("start"))],
                    children: &[
                        Node::Component {
                            name: "Button",
                            props: &[
                                ("hide_label", Value::Bool(true)),
                                ("icon", Value::Text("settings")),
                                ("variant", Value::Text("transparent")),
                                ("tone", Value::Text("neutral")),
                                ("label", Value::Message(Key::SettingsLabel)),
                            ],
                            children: &[],
                        },
                        Node::Component {
                            name: "Button",
                            props: &[
                                ("hide_label", Value::Bool(true)),
                                ("icon", Value::Text("circle-question-mark")),
                                ("variant", Value::Text("transparent")),
                                ("tone", Value::Text("neutral")),
                                ("label", Value::Message(Key::ExampleOptionListHelp)),
                            ],
                            children: &[],
                        },
                    ],
                },
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
];
