//! `Info`, as trees.
//!
//! One entry per example in `examples::INFO`, in the same order. The
//! caption is repeated here so the pairing is pinned to something a
//! reader can check: `every_tree_belongs_to_its_example` reads it against
//! the table's own, and a tree that slid a row apart fails there rather
//! than drawing the wrong picture.

use noctua_design_i18n::Key;

use crate::tree::{Node, Value};
use crate::trees::Tree;

/// Every `Info` example. 4 of them.
pub(crate) const EXAMPLES: &[Tree] = &[
    (
        r"Info { name, label } beside the word it explains",
        &[Node::Component {
            name: "Text",
            props: &[],
            children: &[
                Node::Text(Value::Message(Key::ExampleInfoDeadline)),
                Node::Component {
                    name: "Info",
                    props: &[
                        ("name", Value::Text("deadline")),
                        ("label", Value::Message(Key::ExampleInfoDeadlineLabel)),
                    ],
                    children: &[Node::Text(Value::Message(Key::ExampleInfoDeadlineNote))],
                },
            ],
        }],
    ),
    (
        r"Info { heading }",
        &[Node::Component {
            name: "Text",
            props: &[],
            children: &[
                Node::Text(Value::Message(Key::ExampleInfoUsage)),
                Node::Component {
                    name: "Info",
                    props: &[
                        ("name", Value::Text("usage")),
                        ("label", Value::Message(Key::ExampleInfoUsageLabel)),
                        ("heading", Value::Message(Key::ExampleInfoUsageHeading)),
                    ],
                    children: &[Node::Text(Value::Message(Key::ExampleInfoUsageNote))],
                },
            ],
        }],
    ),
    (
        r"Info { close_label }",
        &[Node::Component {
            name: "Text",
            props: &[],
            children: &[
                Node::Text(Value::Message(Key::ExampleInfoDeadline)),
                Node::Component {
                    name: "Info",
                    props: &[
                        ("name", Value::Text("closable")),
                        ("label", Value::Message(Key::ExampleInfoDeadlineLabel)),
                        ("close_label", Value::Message(Key::ExampleInfoClose)),
                    ],
                    children: &[Node::Text(Value::Message(Key::ExampleInfoDeadlineNote))],
                },
            ],
        }],
    ),
    (
        r"Info { placement: InfoPlacement::End } at the edge of a table",
        &[Node::Element {
            tag: "div",
            attributes: &[("class", Value::Text("nd-example-row nd-example-row-end"))],
            children: &[Node::Component {
                name: "Text",
                props: &[],
                children: &[
                    Node::Text(Value::Message(Key::ExampleInfoColumn)),
                    Node::Component {
                        name: "Info",
                        props: &[
                            ("name", Value::Text("balance")),
                            ("label", Value::Message(Key::ExampleInfoColumnLabel)),
                            ("placement", Value::Text("end")),
                            ("close_label", Value::Message(Key::ExampleInfoClose)),
                        ],
                        children: &[Node::Text(Value::Message(Key::ExampleInfoColumnNote))],
                    },
                ],
            }],
        }],
    ),
];
