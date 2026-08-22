//! `Em`, as trees.
//!
//! One entry per example in `examples::EM`, in the same order. The
//! caption is repeated here so the pairing is pinned to something a
//! reader can check: `every_example_has_a_tree` reads it against
//! the table's own, and a tree that slid a row apart fails there rather
//! than drawing the wrong picture.

use noctua_design_i18n::Key;

use crate::tree::{Node, Value};
use crate::trees::Tree;

/// Every `Em` example. 4 of them.
pub(crate) const EXAMPLES: &[Tree] = &[
    (
        r#"Em { "…" }"#,
        &[Node::Component {
            name: "Paragraph",
            props: &[],
            children: &[
                Node::Text(Value::Join(&[
                    Value::Message(Key::ExampleEmLead),
                    Value::Text(" "),
                ])),
                Node::Component {
                    name: "Em",
                    props: &[],
                    children: &[Node::Text(Value::Message(Key::ExampleEmWord))],
                },
                Node::Text(Value::Data(".")),
            ],
        }],
    ),
    (
        r"Strong { Em { … } }",
        &[Node::Component {
            name: "Paragraph",
            props: &[],
            children: &[Node::Component {
                name: "Strong",
                props: &[],
                children: &[
                    Node::Text(Value::Join(&[
                        Value::Message(Key::ExampleStrongLead),
                        Value::Text(" "),
                    ])),
                    Node::Component {
                        name: "Em",
                        props: &[],
                        children: &[Node::Text(Value::Message(Key::ExampleEmWord))],
                    },
                ],
            }],
        }],
    ),
    (
        r"Em inside a heading",
        &[Node::Component {
            name: "Heading",
            props: &[("level", Value::Text("3")), ("size", Value::Text("h4"))],
            children: &[
                Node::Text(Value::Join(&[
                    Value::Message(Key::ExampleEmLead),
                    Value::Text(" "),
                ])),
                Node::Component {
                    name: "Em",
                    props: &[],
                    children: &[Node::Text(Value::Message(Key::ExampleEmWord))],
                },
            ],
        }],
    ),
    (
        r"Em beside Strong",
        &[Node::Component {
            name: "Paragraph",
            props: &[],
            children: &[
                Node::Component {
                    name: "Em",
                    props: &[],
                    children: &[Node::Text(Value::Message(Key::ExampleEmWord))],
                },
                Node::Text(Value::Data(" · ")),
                Node::Component {
                    name: "Strong",
                    props: &[],
                    children: &[Node::Text(Value::Message(Key::ExampleStrongWord))],
                },
            ],
        }],
    ),
];
