//! `Ins`, as trees.
//!
//! One entry per example in `examples::INS`, in the same order. The
//! caption is repeated here so the pairing is pinned to something a
//! reader can check: `every_tree_belongs_to_its_example` reads it against
//! the table's own, and a tree that slid a row apart fails there rather
//! than drawing the wrong picture.

use noctua_design_i18n::Key;

use crate::tree::{Node, Value};
use crate::trees::Tree;

/// Every `Ins` example. 4 of them.
pub(crate) const EXAMPLES: &[Tree] = &[
    (
        r#"Ins { "…" }"#,
        &[Node::Component {
            name: "Paragraph",
            props: &[],
            children: &[Node::Component {
                name: "Ins",
                props: &[],
                children: &[Node::Text(Value::Message(Key::ExampleEditNew))],
            }],
        }],
    ),
    (
        r#"Ins { label: "added", … }"#,
        &[Node::Component {
            name: "Paragraph",
            props: &[],
            children: &[Node::Component {
                name: "Ins",
                props: &[("label", Value::Message(Key::ExampleEditAdded))],
                children: &[Node::Text(Value::Message(Key::ExampleEditNew))],
            }],
        }],
    ),
    (
        r#"Ins { datetime: "2026-07-29", cite: "…", … }"#,
        &[Node::Component {
            name: "Paragraph",
            props: &[],
            children: &[Node::Component {
                name: "Ins",
                props: &[
                    ("datetime", Value::Text("2026-07-29")),
                    ("cite", Value::Text("https://example.invalid/change-2")),
                    ("label", Value::Message(Key::ExampleEditAdded)),
                ],
                children: &[Node::Text(Value::Message(Key::ExampleEditNew))],
            }],
        }],
    ),
    // NOT `Del` BESIDE `Ins` AGAIN. That example is on `Del`'s own table with
    // the same note key, and one question asked twice on one page is one of
    // the two examples doing nothing. This one is `Ins` around a whole item.
    (
        r"List { ListItem { Ins { label, … } } }",
        &[Node::Component {
            name: "List",
            props: &[
                ("kind", Value::Text("unordered")),
                ("spacing", Value::Text("tight")),
            ],
            children: &[
                Node::Component {
                    name: "ListItem",
                    props: &[],
                    children: &[Node::Text(Value::Message(Key::ExampleListOne))],
                },
                Node::Component {
                    name: "ListItem",
                    props: &[],
                    children: &[Node::Component {
                        name: "Ins",
                        props: &[("label", Value::Message(Key::ExampleEditAdded))],
                        children: &[Node::Text(Value::Message(Key::ExampleListTwo))],
                    }],
                },
            ],
        }],
    ),
];
