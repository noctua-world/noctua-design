//! `Time`, as trees.
//!
//! One entry per example in `examples::TIME`, in the same order. The
//! caption is repeated here so the pairing is pinned to something a
//! reader can check: `every_example_has_a_tree` reads it against
//! the table's own, and a tree that slid a row apart fails there rather
//! than drawing the wrong picture.

use noctua_design_i18n::Key;

use crate::tree::{Node, Value};
use crate::trees::Tree;

/// Every `Time` example. 4 of them.
pub(crate) const EXAMPLES: &[Tree] = &[
    (
        r#"Time { datetime: "2026-07-29", "…" }"#,
        &[Node::Component {
            name: "Paragraph",
            props: &[],
            children: &[Node::Component {
                name: "Time",
                props: &[("datetime", Value::Text("2026-07-29"))],
                children: &[Node::Text(Value::Message(Key::ExampleTimeDate))],
            }],
        }],
    ),
    (
        r#"Time { datetime: "2026-07-29T14:30", … }"#,
        &[Node::Component {
            name: "Paragraph",
            props: &[],
            children: &[Node::Component {
                name: "Time",
                props: &[("datetime", Value::Text("2026-07-29T14:30"))],
                children: &[Node::Text(Value::Message(Key::ExampleTimeDateTime))],
            }],
        }],
    ),
    (
        r"In a sentence",
        &[Node::Component {
            name: "Paragraph",
            props: &[],
            children: &[
                Node::Text(Value::Join(&[
                    Value::Message(Key::ExampleTimeLead),
                    Value::Text(" "),
                ])),
                Node::Component {
                    name: "Time",
                    props: &[("datetime", Value::Text("2026-07-29"))],
                    children: &[Node::Text(Value::Message(Key::ExampleTimeDate))],
                },
                Node::Text(Value::Data(".")),
            ],
        }],
    ),
    (
        r"In a list of dates",
        &[Node::Component {
            name: "List",
            props: &[
                ("marker", Value::Text("none")),
                ("spacing", Value::Text("tight")),
            ],
            children: &[
                Node::Component {
                    name: "ListItem",
                    props: &[],
                    children: &[Node::Component {
                        name: "Time",
                        props: &[("datetime", Value::Text("2026-07-01"))],
                        children: &[Node::Text(Value::Message(Key::ExampleTimeDate))],
                    }],
                },
                Node::Component {
                    name: "ListItem",
                    props: &[],
                    children: &[Node::Component {
                        name: "Time",
                        props: &[("datetime", Value::Text("2026-07-29"))],
                        children: &[Node::Text(Value::Message(Key::ExampleTimeDateTime))],
                    }],
                },
            ],
        }],
    ),
];
