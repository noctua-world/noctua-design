//! `Strong`, as trees.
//!
//! One entry per example in `examples::STRONG`, in the same order. The
//! caption is repeated here so the pairing is pinned to something a
//! reader can check: `every_tree_belongs_to_its_example` reads it against
//! the table's own, and a tree that slid a row apart fails there rather
//! than drawing the wrong picture.

use noctua_design_i18n::Key;

use crate::tree::{Node, Value};
use crate::trees::Tree;

/// Every `Strong` example. 4 of them.
pub(crate) const EXAMPLES: &[Tree] = &[
    (
        r#"Strong { "…" }"#,
        &[Node::Component {
            name: "Paragraph",
            props: &[],
            children: &[
                Node::Text(Value::Join(&[
                    Value::Message(Key::ExampleStrongLead),
                    Value::Text(" "),
                ])),
                Node::Component {
                    name: "Strong",
                    props: &[],
                    children: &[Node::Text(Value::Message(Key::ExampleStrongWord))],
                },
                Node::Text(Value::Data(".")),
            ],
        }],
    ),
    // NOT `Strong { Em { … } }` AGAIN. `Em`'s own table asks that question,
    // under this very note key. The question `Strong` alone can answer is what
    // a second `Strong` inside the first one means.
    (
        r"Strong { … Strong { … } }",
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
                        name: "Strong",
                        props: &[],
                        children: &[Node::Text(Value::Message(Key::ExampleStrongWord))],
                    },
                ],
            }],
        }],
    ),
    (
        r"Strong inside a list",
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
                    children: &[
                        Node::Component {
                            name: "Strong",
                            props: &[],
                            children: &[Node::Text(Value::Message(Key::ExampleStrongWord))],
                        },
                        Node::Text(Value::Join(&[
                            Value::Text(" — "),
                            Value::Message(Key::ExampleListOne),
                        ])),
                    ],
                },
                Node::Component {
                    name: "ListItem",
                    props: &[],
                    children: &[Node::Text(Value::Message(Key::ExampleListTwo))],
                },
            ],
        }],
    ),
    // `Em` beside `Strong` is on `Em`'s table already. What belongs here is
    // the case the element exists for: the part of a sentence that matters
    // most, which is usually the word a warning opens on.
    (
        r"Strong opening a warning",
        &[Node::Component {
            name: "Paragraph",
            props: &[],
            children: &[
                Node::Component {
                    name: "Strong",
                    props: &[],
                    children: &[Node::Text(Value::Message(Key::ExampleStrongWarningWord))],
                },
                Node::Text(Value::Join(&[
                    Value::Text(" "),
                    Value::Message(Key::ExampleStrongWarningBody),
                ])),
            ],
        }],
    ),
];
