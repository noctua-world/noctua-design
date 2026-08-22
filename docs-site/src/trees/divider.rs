//! `Divider`, as trees.
//!
//! One entry per example in `examples::DIVIDER`, in the same order. The
//! caption is repeated here so the pairing is pinned to something a
//! reader can check: `every_example_has_a_tree` reads it against
//! the table's own, and a tree that slid a row apart fails there rather
//! than drawing the wrong picture.

use noctua_design_i18n::Key;

use crate::tree::{Each, Node, Value};
use crate::trees::Tree;

/// Every `Divider` example. 5 of them.
pub(crate) const EXAMPLES: &[Tree] = &[
    (
        r"Divider { spacing: DividerSpacing::Loose }",
        &[Node::Element {
            tag: "div",
            attributes: &[],
            children: &[
                Node::Component {
                    name: "Heading",
                    props: &[("level", Value::Text("3")), ("size", Value::Text("h4"))],
                    children: &[Node::Text(Value::Message(Key::ExampleDividerAppearance))],
                },
                Node::Component {
                    name: "Paragraph",
                    props: &[],
                    children: &[Node::Text(Value::Message(
                        Key::ExampleDividerAppearanceBody,
                    ))],
                },
                Node::Component {
                    name: "Divider",
                    props: &[("spacing", Value::Text("loose"))],
                    children: &[],
                },
                Node::Component {
                    name: "Heading",
                    props: &[("level", Value::Text("3")), ("size", Value::Text("h4"))],
                    children: &[Node::Text(Value::Message(Key::ExampleDividerMotion))],
                },
                Node::Component {
                    name: "Paragraph",
                    props: &[],
                    children: &[Node::Text(Value::Message(Key::ExampleDividerMotionBody))],
                },
            ],
        }],
    ),
    (
        r#"Divider { label: "or" }"#,
        &[Node::Element {
            tag: "div",
            attributes: &[("class", Value::Text("nd-example-stack"))],
            children: &[
                Node::Component {
                    name: "Button",
                    props: &[
                        ("variant", Value::Text("solid")),
                        ("tone", Value::Text("accent")),
                        ("label", Value::Message(Key::ExampleDividerPassword)),
                    ],
                    children: &[],
                },
                Node::Component {
                    name: "Divider",
                    props: &[("label", Value::Message(Key::ExampleDividerOr))],
                    children: &[],
                },
                Node::Component {
                    name: "Button",
                    props: &[
                        ("variant", Value::Text("outline")),
                        ("tone", Value::Text("neutral")),
                        ("label", Value::Message(Key::ExampleDividerLink)),
                    ],
                    children: &[],
                },
            ],
        }],
    ),
    (
        r"Divider { orientation: Vertical, weight: Strong }",
        &[Node::Element {
            tag: "div",
            attributes: &[("class", Value::Text("nd-example-row"))],
            children: &[
                Node::Component {
                    name: "Button",
                    props: &[
                        ("variant", Value::Text("outline")),
                        ("tone", Value::Text("neutral")),
                        ("icon", Value::Text("copy")),
                        ("label", Value::Message(Key::ExampleCopy)),
                    ],
                    children: &[],
                },
                Node::Component {
                    name: "Divider",
                    props: &[
                        ("orientation", Value::Text("vertical")),
                        ("weight", Value::Text("strong")),
                    ],
                    children: &[],
                },
                Node::Component {
                    name: "Button",
                    props: &[
                        ("variant", Value::Text("outline")),
                        ("tone", Value::Text("neutral")),
                        ("label", Value::Message(Key::ExampleButtonSave)),
                    ],
                    children: &[],
                },
                Node::Component {
                    name: "Button",
                    props: &[
                        ("variant", Value::Text("outline")),
                        ("tone", Value::Text("neutral")),
                        ("label", Value::Message(Key::ExampleButtonPublish)),
                    ],
                    children: &[],
                },
            ],
        }],
    ),
    (
        r"Divider { weight, spacing: Tight } for weight in DividerWeight::ALL",
        &[Node::Element {
            tag: "div",
            attributes: &[],
            // Read from the contract, not listed: a fourth border token added to
            // `spec/components/divider.toml` draws itself here.
            children: &[Node::Each {
                over: Each::Values {
                    component: "Divider",
                    prop: "weight",
                },
                body: &[Node::Component {
                    name: "Divider",
                    props: &[
                        ("weight", Value::Bound(0)),
                        ("spacing", Value::Text("tight")),
                    ],
                    children: &[],
                }],
            }],
        }],
    ),
    (
        r"Divider { decorative: true, spacing: Tight }",
        &[Node::Element {
            tag: "div",
            attributes: &[],
            children: &[
                Node::Component {
                    name: "Heading",
                    props: &[("level", Value::Text("3")), ("size", Value::Text("h4"))],
                    children: &[Node::Text(Value::Message(Key::ExampleDividerMotion))],
                },
                Node::Component {
                    name: "Divider",
                    props: &[
                        ("decorative", Value::Bool(true)),
                        ("spacing", Value::Text("tight")),
                    ],
                    children: &[],
                },
                Node::Component {
                    name: "Paragraph",
                    props: &[],
                    children: &[Node::Text(Value::Message(Key::ExampleDividerMotionBody))],
                },
            ],
        }],
    ),
];
