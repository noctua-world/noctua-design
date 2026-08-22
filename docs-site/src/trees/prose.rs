//! `Prose`, as trees.
//!
//! One entry per example in `examples::PROSE`, in the same order. The
//! caption is repeated here so the pairing is pinned to something a
//! reader can check: `every_example_has_a_tree` reads it against
//! the table's own, and a tree that slid a row apart fails there rather
//! than drawing the wrong picture.

use noctua_design_i18n::Key;

use crate::tree::{Node, Value};
use crate::trees::Tree;

/// Every `Prose` example. 4 of them.
pub(crate) const EXAMPLES: &[Tree] = &[
    (
        r"Prose { … }",
        &[Node::Component {
            name: "Prose",
            props: &[],
            children: &[
                Node::Component {
                    name: "Heading",
                    props: &[("level", Value::Text("3")), ("size", Value::Text("h3"))],
                    children: &[Node::Text(Value::Message(Key::ExampleHeadingSection))],
                },
                Node::Component {
                    name: "Paragraph",
                    props: &[],
                    children: &[Node::Text(Value::Message(Key::ExampleListLong))],
                },
                Node::Component {
                    name: "Paragraph",
                    props: &[],
                    children: &[Node::Text(Value::Message(Key::ExampleListLong))],
                },
            ],
        }],
    ),
    (
        r"Prose { measure: ProseMeasure::Lead, … }",
        &[Node::Component {
            name: "Prose",
            props: &[("measure", Value::Text("lead"))],
            children: &[Node::Component {
                name: "Paragraph",
                props: &[("size", Value::Text("lead"))],
                children: &[Node::Text(Value::Message(Key::ExampleListLong))],
            }],
        }],
    ),
    (
        r"Prose { measure: ProseMeasure::None, … }",
        &[Node::Component {
            name: "Prose",
            props: &[("measure", Value::Text("none"))],
            children: &[Node::Component {
                name: "Paragraph",
                props: &[],
                children: &[Node::Text(Value::Message(Key::ExampleListLong))],
            }],
        }],
    ),
    (
        r"A whole reading column",
        &[Node::Component {
            name: "Prose",
            props: &[],
            children: &[
                Node::Component {
                    name: "Heading",
                    props: &[("level", Value::Text("3")), ("size", Value::Text("h3"))],
                    children: &[Node::Text(Value::Message(Key::ExampleHeadingSection))],
                },
                Node::Component {
                    name: "Paragraph",
                    props: &[],
                    children: &[Node::Text(Value::Message(Key::ExampleListLong))],
                },
                Node::Component {
                    name: "List",
                    props: &[("spacing", Value::Text("tight"))],
                    children: &[
                        Node::Component {
                            name: "ListItem",
                            props: &[],
                            children: &[Node::Text(Value::Message(Key::ExampleListOne))],
                        },
                        Node::Component {
                            name: "ListItem",
                            props: &[],
                            children: &[Node::Text(Value::Message(Key::ExampleListTwo))],
                        },
                    ],
                },
                Node::Component {
                    name: "Blockquote",
                    props: &[],
                    children: &[Node::Component {
                        name: "Paragraph",
                        props: &[],
                        children: &[Node::Text(Value::Message(Key::ExampleQuoteShort))],
                    }],
                },
            ],
        }],
    ),
];
