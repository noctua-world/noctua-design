//! `Mark`, as trees.
//!
//! One entry per example in `examples::MARK`, in the same order. The
//! caption is repeated here so the pairing is pinned to something a
//! reader can check: `every_tree_belongs_to_its_example` reads it against
//! the table's own, and a tree that slid a row apart fails there rather
//! than drawing the wrong picture.

use noctua_design_i18n::Key;

use crate::tree::{Node, Value};
use crate::trees::Tree;

/// Every `Mark` example. 4 of them.
pub(crate) const EXAMPLES: &[Tree] = &[
    (
        r#"Mark { "…" }"#,
        &[Node::Component {
            name: "Paragraph",
            props: &[],
            children: &[
                Node::Text(Value::Join(&[
                    Value::Message(Key::ExampleMarkLead),
                    Value::Text(" "),
                ])),
                Node::Component {
                    name: "Mark",
                    props: &[],
                    children: &[Node::Text(Value::Message(Key::ExampleMarkWord))],
                },
                Node::Text(Value::Text(".")),
            ],
        }],
    ),
    (
        r"Two hits in one line",
        &[Node::Component {
            name: "Paragraph",
            props: &[],
            children: &[
                Node::Component {
                    name: "Mark",
                    props: &[],
                    children: &[Node::Text(Value::Message(Key::ExampleMarkWord))],
                },
                Node::Text(Value::Join(&[
                    Value::Text(" — "),
                    Value::Message(Key::ExampleMarkLead),
                    Value::Text(" "),
                ])),
                Node::Component {
                    name: "Mark",
                    props: &[],
                    children: &[Node::Text(Value::Message(Key::ExampleMarkWord))],
                },
            ],
        }],
    ),
    (
        r"Mark inside a heading",
        &[Node::Component {
            name: "Heading",
            props: &[("level", Value::Text("3")), ("size", Value::Text("h4"))],
            children: &[Node::Component {
                name: "Mark",
                props: &[],
                children: &[Node::Text(Value::Message(Key::ExampleMarkWord))],
            }],
        }],
    ),
    (
        r"Mark inside a list",
        &[Node::Component {
            name: "List",
            props: &[("spacing", Value::Text("tight"))],
            children: &[Node::Component {
                name: "ListItem",
                props: &[],
                children: &[
                    Node::Component {
                        name: "Mark",
                        props: &[],
                        children: &[Node::Text(Value::Message(Key::ExampleMarkWord))],
                    },
                    Node::Text(Value::Join(&[
                        Value::Text(" — "),
                        Value::Message(Key::ExampleListOne),
                    ])),
                ],
            }],
        }],
    ),
];
