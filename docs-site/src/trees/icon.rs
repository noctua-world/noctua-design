//! `Icon`, as trees.
//!
//! One entry per example in `examples::ICON`, in the same order. The
//! caption is repeated here so the pairing is pinned to something a
//! reader can check: `every_example_has_a_tree` reads it against
//! the table's own, and a tree that slid a row apart fails there rather
//! than drawing the wrong picture.

use noctua_design_i18n::Key;

use crate::tree::{Node, Value};
use crate::trees::Tree;

/// Every `Icon` example. 4 of them.
pub(crate) const EXAMPLES: &[Tree] = &[
    // THE COMMON CASE, AND THE DEFAULT. A drawing beside a word says what the
    // word says, so it is out of the accessibility tree.
    (
        r#"Icon { icon: "check" }"#,
        &[Node::Element {
            tag: "span",
            attributes: &[("class", Value::Text("nd-example-row"))],
            children: &[
                Node::Component {
                    name: "Icon",
                    props: &[("icon", Value::Text("check"))],
                    children: &[],
                },
                Node::Component {
                    name: "Text",
                    props: &[],
                    children: &[Node::Text(Value::Message(Key::ExampleIconSaved))],
                },
            ],
        }],
    ),
    // THE ONE THAT NEEDS A NAME. Nothing beside it says what it means, so the
    // drawing becomes a `role="img"` carrying the caller's words.
    (
        r#"Icon { icon: "star", label: "…" }"#,
        // NO `size: Medium`. It is the default, so the example that
        // teaches the accessible name also carried a prop doing nothing.
        // The three sizes are the example below.
        &[Node::Component {
            name: "Icon",
            props: &[
                ("icon", Value::Text("star")),
                ("label", Value::Message(Key::ExampleIconFavourite)),
            ],
            children: &[],
        }],
    ),
    (
        r"Icon { size: Small | Medium | Large }",
        &[Node::Element {
            tag: "span",
            attributes: &[("class", Value::Text("nd-example-row"))],
            children: &[
                Node::Component {
                    name: "Icon",
                    props: &[
                        ("icon", Value::Text("palette")),
                        ("size", Value::Text("small")),
                    ],
                    children: &[],
                },
                Node::Component {
                    name: "Icon",
                    props: &[
                        ("icon", Value::Text("palette")),
                        ("size", Value::Text("medium")),
                    ],
                    children: &[],
                },
                Node::Component {
                    name: "Icon",
                    props: &[
                        ("icon", Value::Text("palette")),
                        ("size", Value::Text("large")),
                    ],
                    children: &[],
                },
            ],
        }],
    ),
    // The box is `1em`, which is the whole reason one class covers every
    // context: a heading and a caption size their own icon.
    (
        r"Icon { … } inside a Heading and inside a caption",
        &[Node::Element {
            tag: "div",
            attributes: &[],
            children: &[
                Node::Component {
                    name: "Heading",
                    props: &[("level", Value::Text("3")), ("size", Value::Text("h4"))],
                    children: &[
                        Node::Component {
                            name: "Icon",
                            props: &[("icon", Value::Text("info"))],
                            children: &[],
                        },
                        Node::Text(Value::Join(&[
                            Value::Text(" "),
                            Value::Message(Key::ExampleIconHeading),
                        ])),
                    ],
                },
                Node::Component {
                    name: "Text",
                    props: &[
                        ("size", Value::Text("small")),
                        ("tone", Value::Text("muted")),
                    ],
                    children: &[
                        Node::Component {
                            name: "Icon",
                            props: &[("icon", Value::Text("clock"))],
                            children: &[],
                        },
                        Node::Text(Value::Join(&[
                            Value::Text(" "),
                            Value::Message(Key::ExampleIconCaption),
                        ])),
                    ],
                },
            ],
        }],
    ),
];
