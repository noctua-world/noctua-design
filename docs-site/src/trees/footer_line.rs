//! `FooterLine`, as trees.
//!
//! One entry per example in `examples::FOOTER_LINE`, in the same order. The
//! caption is repeated here so the pairing is pinned to something a
//! reader can check: `every_example_has_a_tree` reads it against
//! the table's own, and a tree that slid a row apart fails there rather
//! than drawing the wrong picture.

use noctua_design_i18n::Key;

use crate::tree::{Node, Value};
use crate::trees::Tree;

/// What the page turns an expandable line with: the stage it is in, and the
/// control's two names. See `enhanceLineExpanders` in
/// `docs-site/assets/shell.js`, which plays the caller's part.
pub(super) const EXPAND: &[(&str, Value)] = &[
    ("class", Value::Text("nd-example-expand")),
    (
        "data-nd-expand-label",
        Value::Message(Key::ExampleFooterExpand),
    ),
    (
        "data-nd-collapse-label",
        Value::Message(Key::ExampleFooterCollapse),
    ),
];

/// Every `FooterLine` example. 4 of them.
pub(crate) const EXAMPLES: &[Tree] = &[
    (
        r"FooterLine { kind: FooterLineKind::Modeline } / FooterLine { kind: FooterLineKind::Info } / FooterLine { kind: FooterLineKind::Prompt }",
        &[Node::Component {
            name: "Footer",
            props: &[("label", Value::Message(Key::ExampleFooterLabel))],
            children: &[
                Node::Component {
                    name: "FooterLine",
                    props: &[("kind", Value::Text("modeline"))],
                    children: &[Node::Element {
                        tag: "span",
                        attributes: &[],
                        children: &[Node::Text(Value::Message(Key::ExampleFooterModeline))],
                    }],
                },
                Node::Component {
                    name: "FooterLine",
                    props: &[("kind", Value::Text("info"))],
                    children: &[Node::Element {
                        tag: "span",
                        attributes: &[],
                        children: &[Node::Text(Value::Message(Key::ExampleFooterInfo))],
                    }],
                },
                Node::Component {
                    name: "FooterLine",
                    props: &[("kind", Value::Text("prompt"))],
                    children: &[Node::Element {
                        tag: "span",
                        attributes: &[],
                        children: &[Node::Text(Value::Message(Key::ExampleFooterPrompt))],
                    }],
                },
            ],
        }],
    ),
    (
        r#"FooterLine { kind: FooterLineKind::Info, expandable: true, expand_label: "…", collapse_label: "…", action: "…" }"#,
        &[Node::Element {
            tag: "div",
            attributes: EXPAND,
            children: &[Node::Component {
                name: "Footer",
                props: &[("label", Value::Message(Key::ExampleFooterLabel))],
                children: &[Node::Component {
                    name: "FooterLine",
                    props: &[
                        ("kind", Value::Text("info")),
                        ("expandable", Value::Bool(true)),
                        ("expand_label", Value::Message(Key::ExampleFooterExpand)),
                        ("collapse_label", Value::Message(Key::ExampleFooterCollapse)),
                        ("action", Value::Text("example-footer-expand")),
                    ],
                    children: &[
                        Node::Component {
                            name: "Icon",
                            props: &[
                                ("icon", Value::Text("info")),
                                ("size", Value::Text("small")),
                            ],
                            children: &[],
                        },
                        Node::Element {
                            tag: "span",
                            attributes: &[("class", Value::Text("nd-example-run"))],
                            children: &[
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
                                    children: &[Node::Text(Value::Message(
                                        Key::ExampleFooterHover,
                                    ))],
                                },
                            ],
                        },
                        Node::Element {
                            tag: "span",
                            attributes: &[("class", Value::Text("nd-example-run"))],
                            children: &[
                                Node::Component {
                                    name: "Icon",
                                    props: &[
                                        ("icon", Value::Text("keyboard")),
                                        ("size", Value::Text("small")),
                                    ],
                                    children: &[],
                                },
                                Node::Element {
                                    tag: "span",
                                    attributes: &[],
                                    children: &[Node::Text(Value::Message(
                                        Key::ExampleFooterSelection,
                                    ))],
                                },
                            ],
                        },
                    ],
                }],
            }],
        }],
    ),
    (
        r"FooterLine { kind: FooterLineKind::Info, expandable: true, expanded: true, … }",
        &[Node::Element {
            tag: "div",
            attributes: EXPAND,
            children: &[Node::Component {
                name: "Footer",
                props: &[("label", Value::Message(Key::ExampleFooterLabel))],
                children: &[Node::Component {
                    name: "FooterLine",
                    props: &[
                        ("kind", Value::Text("info")),
                        ("expandable", Value::Bool(true)),
                        ("expand_label", Value::Message(Key::ExampleFooterExpand)),
                        ("collapse_label", Value::Message(Key::ExampleFooterCollapse)),
                        ("expanded", Value::Bool(true)),
                        ("action", Value::Text("example-footer-collapse")),
                    ],
                    children: &[
                        Node::Component {
                            name: "Icon",
                            props: &[
                                ("icon", Value::Text("info")),
                                ("size", Value::Text("small")),
                            ],
                            children: &[],
                        },
                        Node::Element {
                            tag: "span",
                            attributes: &[("class", Value::Text("nd-example-run"))],
                            children: &[
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
                                    children: &[Node::Text(Value::Message(
                                        Key::ExampleFooterHover,
                                    ))],
                                },
                            ],
                        },
                        Node::Element {
                            tag: "span",
                            attributes: &[("class", Value::Text("nd-example-run"))],
                            children: &[
                                Node::Component {
                                    name: "Icon",
                                    props: &[
                                        ("icon", Value::Text("keyboard")),
                                        ("size", Value::Text("small")),
                                    ],
                                    children: &[],
                                },
                                Node::Element {
                                    tag: "span",
                                    attributes: &[],
                                    children: &[Node::Text(Value::Message(
                                        Key::ExampleFooterSelection,
                                    ))],
                                },
                            ],
                        },
                    ],
                }],
            }],
        }],
    ),
    (
        r"FooterLine { kind: FooterLineKind::Info, live: true }",
        &[Node::Component {
            name: "Footer",
            props: &[("label", Value::Message(Key::ExampleFooterLabel))],
            children: &[Node::Component {
                name: "FooterLine",
                props: &[("kind", Value::Text("info")), ("live", Value::Bool(true))],
                children: &[
                    Node::Component {
                        name: "Icon",
                        props: &[
                            ("icon", Value::Text("info")),
                            ("size", Value::Text("small")),
                        ],
                        children: &[],
                    },
                    Node::Element {
                        tag: "span",
                        attributes: &[],
                        children: &[Node::Text(Value::Message(Key::ExampleFooterNotice))],
                    },
                ],
            }],
        }],
    ),
];
