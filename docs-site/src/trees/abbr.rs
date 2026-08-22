//! `Abbr`, as trees.
//!
//! One entry per example in `examples::ABBR`, in the same order. The
//! caption is repeated here so the pairing is pinned to something a
//! reader can check: `every_example_has_a_tree` reads it against
//! the table's own, and a tree that slid a row apart fails there rather
//! than drawing the wrong picture.

use noctua_design_i18n::Key;

use crate::tree::{Node, Value};
use crate::trees::Tree;

/// Every `Abbr` example. 4 of them.
pub(crate) const EXAMPLES: &[Tree] = &[
    (
        r#"Abbr { title: "Cascading Style Sheets", "CSS" }"#,
        &[Node::Component {
            name: "Paragraph",
            props: &[],
            children: &[
                Node::Text(Value::Join(&[
                    Value::Message(Key::ExampleAbbrLead),
                    Value::Text(" "),
                ])),
                Node::Component {
                    name: "Abbr",
                    props: &[
                        ("title", Value::Data("Cascading Style Sheets")),
                        ("class", Value::Text("nd-identifier")),
                    ],
                    children: &[Node::Text(Value::Data("CSS"))],
                },
                Node::Text(Value::Data(".")),
            ],
        }],
    ),
    (
        r#"Abbr { title: "…", "WCAG" }"#,
        &[Node::Component {
            name: "Paragraph",
            props: &[],
            children: &[
                Node::Text(Value::Join(&[
                    Value::Message(Key::ExampleAbbrExpansion),
                    Value::Text(" "),
                ])),
                Node::Component {
                    name: "Abbr",
                    props: &[
                        ("title", Value::Data("Web Content Accessibility Guidelines")),
                        ("class", Value::Text("nd-identifier")),
                    ],
                    children: &[Node::Text(Value::Data("WCAG"))],
                },
                Node::Text(Value::Data(".")),
            ],
        }],
    ),
    (
        r"Heading { … Abbr { … } }",
        &[Node::Component {
            name: "Heading",
            props: &[("level", Value::Text("3")), ("size", Value::Text("h4"))],
            children: &[
                Node::Component {
                    name: "Abbr",
                    props: &[
                        ("title", Value::Data("Hypertext Markup Language")),
                        ("class", Value::Text("nd-identifier")),
                    ],
                    children: &[Node::Text(Value::Data("HTML"))],
                },
                Node::Text(Value::Join(&[
                    Value::Text(" "),
                    Value::Message(Key::ExampleAbbrHeading),
                ])),
            ],
        }],
    ),
    (
        r"Two in one sentence",
        &[Node::Component {
            name: "Paragraph",
            props: &[],
            children: &[
                Node::Component {
                    name: "Abbr",
                    props: &[
                        ("title", Value::Data("Application Programming Interface")),
                        ("class", Value::Text("nd-identifier")),
                    ],
                    children: &[Node::Text(Value::Data("API"))],
                },
                Node::Text(Value::Join(&[
                    Value::Message(Key::ExampleAbbrPair),
                    Value::Text(" "),
                ])),
                Node::Component {
                    name: "Abbr",
                    props: &[
                        (
                            "title",
                            Value::Data("Accessible Rich Internet Applications"),
                        ),
                        ("class", Value::Text("nd-identifier")),
                    ],
                    children: &[Node::Text(Value::Data("ARIA"))],
                },
                Node::Text(Value::Data(".")),
            ],
        }],
    ),
];
