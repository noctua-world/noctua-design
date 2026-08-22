//! `Del`, as trees.
//!
//! One entry per example in `examples::DEL`, in the same order. The
//! caption is repeated here so the pairing is pinned to something a
//! reader can check: `every_example_has_a_tree` reads it against
//! the table's own, and a tree that slid a row apart fails there rather
//! than drawing the wrong picture.

use noctua_design_i18n::Key;

use crate::tree::{Node, Value};
use crate::trees::Tree;

/// Every `Del` example. 4 of them.
pub(crate) const EXAMPLES: &[Tree] = &[
    (
        r#"Del { "…" }"#,
        &[Node::Component {
            name: "Paragraph",
            props: &[],
            children: &[
                Node::Component {
                    name: "Del",
                    props: &[],
                    children: &[Node::Text(Value::Message(Key::ExampleEditOld))],
                },
                Node::Text(Value::Text(" ")),
                Node::Text(Value::Message(Key::ExampleEditNew)),
            ],
        }],
    ),
    (
        r#"Del { label: "removed", … }"#,
        &[Node::Component {
            name: "Paragraph",
            props: &[],
            children: &[Node::Component {
                name: "Del",
                props: &[("label", Value::Message(Key::ExampleEditRemoved))],
                children: &[Node::Text(Value::Message(Key::ExampleEditOld))],
            }],
        }],
    ),
    (
        r#"Del { datetime: "2026-07-29", cite: "…", … }"#,
        &[Node::Component {
            name: "Paragraph",
            props: &[],
            children: &[Node::Component {
                name: "Del",
                props: &[
                    ("datetime", Value::Text("2026-07-29")),
                    ("cite", Value::Text("https://example.invalid/change-1")),
                    ("label", Value::Message(Key::ExampleEditRemoved)),
                ],
                children: &[Node::Text(Value::Message(Key::ExampleEditOld))],
            }],
        }],
    ),
    (
        r"Del beside Ins",
        &[Node::Component {
            name: "Paragraph",
            props: &[],
            children: &[
                Node::Component {
                    name: "Del",
                    props: &[("label", Value::Message(Key::ExampleEditRemoved))],
                    children: &[Node::Text(Value::Message(Key::ExampleEditOld))],
                },
                Node::Text(Value::Text(" ")),
                Node::Component {
                    name: "Ins",
                    props: &[("label", Value::Message(Key::ExampleEditAdded))],
                    children: &[Node::Text(Value::Message(Key::ExampleEditNew))],
                },
            ],
        }],
    ),
];
