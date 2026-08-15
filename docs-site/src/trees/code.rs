//! `Code`, as trees.
//!
//! One entry per example in `examples::CODE`, in the same order. The
//! caption is repeated here so the pairing is pinned to something a
//! reader can check: `every_tree_belongs_to_its_example` reads it against
//! the table's own, and a tree that slid a row apart fails there rather
//! than drawing the wrong picture.

use noctua_design_i18n::Key;

use crate::tree::{Node, Value};
use crate::trees::Tree;

/// Every `Code` example. 4 of them.
pub(crate) const EXAMPLES: &[Tree] = &[
    (
        r#"Code { "…" }"#,
        &[Node::Component {
            name: "Paragraph",
            props: &[],
            children: &[
                Node::Text(Value::Join(&[
                    Value::Message(Key::ExampleCodeLead),
                    Value::Text(" "),
                ])),
                Node::Component {
                    name: "Code",
                    props: &[("class", Value::Text("nd-identifier"))],
                    children: &[Node::Text(Value::Data("--noctua-space-gap"))],
                },
                Node::Text(Value::Data(".")),
            ],
        }],
    ),
    (
        r"Code { tone: CodeTone::Subtle, … }",
        &[Node::Component {
            name: "Paragraph",
            props: &[],
            children: &[
                Node::Text(Value::Join(&[
                    Value::Message(Key::ExampleCodeLead),
                    Value::Text(" "),
                ])),
                Node::Component {
                    name: "Code",
                    props: &[
                        ("class", Value::Text("nd-identifier")),
                        ("tone", Value::Text("subtle")),
                    ],
                    children: &[Node::Text(Value::Data("--noctua-space-gap"))],
                },
                Node::Text(Value::Data(".")),
            ],
        }],
    ),
    (
        r"Heading { … Code { … } }",
        &[Node::Component {
            name: "Heading",
            props: &[("level", Value::Text("3")), ("size", Value::Text("h4"))],
            children: &[
                Node::Component {
                    name: "Code",
                    props: &[("class", Value::Text("nd-identifier"))],
                    children: &[Node::Text(Value::Data("ThemeScope"))],
                },
                Node::Text(Value::Join(&[
                    Value::Text(" "),
                    Value::Message(Key::ExampleCodeInHeading),
                ])),
            ],
        }],
    ),
    (
        r"A long name, in a narrow column",
        &[Node::Component {
            name: "Prose",
            props: &[("measure", Value::Text("default"))],
            children: &[Node::Component {
                name: "Paragraph",
                props: &[],
                children: &[Node::Component {
                    name: "Code",
                    props: &[("class", Value::Text("nd-identifier"))],
                    children: &[Node::Text(Value::Data("--nc-accent-bg-element-hover"))],
                }],
            }],
        }],
    ),
];
