//! `Heading`, as trees.
//!
//! One entry per example in `examples::HEADING`, in the same order. The
//! caption is repeated here so the pairing is pinned to something a
//! reader can check: `every_tree_belongs_to_its_example` reads it against
//! the table's own, and a tree that slid a row apart fails there rather
//! than drawing the wrong picture.

use noctua_design_i18n::Key;

use crate::tree::{Node, Value};
use crate::trees::Tree;

/// Every `Heading` example. 5 of them.
pub(crate) const EXAMPLES: &[Tree] = &[
    (
        r"An outline: levels 2, 3 and 4",
        &[
            Node::Component {
                name: "Heading",
                props: &[("level", Value::Text("2"))],
                children: &[Node::Text(Value::Message(Key::ExampleHeadingSection))],
            },
            Node::Component {
                name: "Heading",
                props: &[("level", Value::Text("3")), ("size", Value::Text("h3"))],
                children: &[Node::Text(Value::Message(Key::ExampleHeadingSub))],
            },
            Node::Component {
                name: "Heading",
                props: &[("level", Value::Text("4")), ("size", Value::Text("h4"))],
                children: &[Node::Text(Value::Message(Key::ExampleHeadingDetail))],
            },
        ],
    ),
    (
        r"Heading { level: _3, size: HeadingSize::Display, … }",
        &[Node::Component {
            name: "Heading",
            props: &[
                ("level", Value::Text("3")),
                ("size", Value::Text("display")),
            ],
            children: &[Node::Text(Value::Message(Key::ExampleHeadingSection))],
        }],
    ),
    (
        r"Heading { tone: HeadingTone::Muted, … }",
        &[Node::Component {
            name: "Heading",
            props: &[
                ("level", Value::Text("3")),
                ("size", Value::Text("h3")),
                ("tone", Value::Text("muted")),
            ],
            children: &[Node::Text(Value::Message(Key::ExampleHeadingSub))],
        }],
    ),
    (
        r"Heading { tone: HeadingTone::Accent, … }",
        &[Node::Component {
            name: "Heading",
            props: &[
                ("level", Value::Text("3")),
                ("size", Value::Text("h3")),
                ("tone", Value::Text("accent")),
            ],
            children: &[Node::Text(Value::Message(Key::ExampleHeadingSub))],
        }],
    ),
    (
        r"Heading { balance: true, … }",
        &[Node::Component {
            name: "Prose",
            props: &[("measure", Value::Text("default"))],
            children: &[Node::Component {
                name: "Heading",
                props: &[
                    ("level", Value::Text("3")),
                    ("size", Value::Text("h2")),
                    ("balance", Value::Bool(true)),
                ],
                children: &[Node::Text(Value::Message(Key::ExampleHeadingLong))],
            }],
        }],
    ),
];
