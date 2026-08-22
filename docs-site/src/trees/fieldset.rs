//! `Fieldset`, as trees.
//!
//! One entry per example in `examples::FIELDSET`, in the same order. The
//! caption is repeated here so the pairing is pinned to something a
//! reader can check: `every_example_has_a_tree` reads it against
//! the table's own, and a tree that slid a row apart fails there rather
//! than drawing the wrong picture.

use noctua_design_i18n::Key;

use crate::tree::{Node, Value};
use crate::trees::Tree;

/// Every `Fieldset` example. 6 of them.
pub(crate) const EXAMPLES: &[Tree] = &[
    (
        r#"Fieldset { name: "…", legend: "…", hint: "…" }"#,
        &[Node::Component {
            name: "Fieldset",
            props: &[
                ("name", Value::Text("notify")),
                ("legend", Value::Message(Key::ExampleFormsContactLegend)),
                ("hint", Value::Message(Key::ExampleFormsContactHint)),
            ],
            children: &[
                Node::Component {
                    name: "Radio",
                    props: &[
                        ("name", Value::Text("notify")),
                        ("value", Value::Text("email")),
                        ("label", Value::Message(Key::ExampleFormsContactEmail)),
                    ],
                    children: &[],
                },
                Node::Component {
                    name: "Radio",
                    props: &[
                        ("name", Value::Text("notify")),
                        ("value", Value::Text("text")),
                        ("label", Value::Message(Key::ExampleFormsContactText)),
                    ],
                    children: &[],
                },
            ],
        }],
    ),
    (
        r#"Fieldset { error: "…", error_label: "…" }"#,
        &[Node::Component {
            name: "Fieldset",
            props: &[
                ("name", Value::Text("terms")),
                ("legend", Value::Message(Key::ExampleFormsTermsLegend)),
                ("error", Value::Message(Key::ExampleFormsTermsError)),
                ("error_label", Value::Message(Key::ExampleFormsErrorWord)),
            ],
            children: &[Node::Component {
                name: "Checkbox",
                props: &[
                    ("name", Value::Text("terms")),
                    ("value", Value::Text("yes")),
                    ("label", Value::Message(Key::ExampleFormsTermsAgree)),
                    ("invalid", Value::Bool(true)),
                ],
                children: &[],
            }],
        }],
    ),
    (
        r"Fieldset { inline: true, … }",
        &[Node::Component {
            name: "Fieldset",
            props: &[
                ("name", Value::Text("notify-inline")),
                ("legend", Value::Message(Key::ExampleFormsNotifyLegend)),
                ("inline", Value::Bool(true)),
            ],
            children: &[
                Node::Component {
                    name: "Radio",
                    props: &[
                        ("name", Value::Text("notify-inline")),
                        ("value", Value::Text("yes")),
                        ("label", Value::Message(Key::ExampleFormsYes)),
                        ("size", Value::Text("medium")),
                    ],
                    children: &[],
                },
                Node::Component {
                    name: "Radio",
                    props: &[
                        ("name", Value::Text("notify-inline")),
                        ("value", Value::Text("no")),
                        ("label", Value::Message(Key::ExampleFormsNo)),
                        ("size", Value::Text("medium")),
                    ],
                    children: &[],
                },
            ],
        }],
    ),
    (
        r"Fieldset { heading: true, … }",
        &[Node::Component {
            name: "Fieldset",
            props: &[
                ("name", Value::Text("notify-heading")),
                ("legend", Value::Message(Key::ExampleFormsNotifyLegend)),
                ("heading", Value::Bool(true)),
            ],
            children: &[
                Node::Component {
                    name: "Radio",
                    props: &[
                        ("name", Value::Text("notify-heading")),
                        ("value", Value::Text("yes")),
                        ("label", Value::Message(Key::ExampleFormsYes)),
                    ],
                    children: &[],
                },
                Node::Component {
                    name: "Radio",
                    props: &[
                        ("name", Value::Text("notify-heading")),
                        ("value", Value::Text("no")),
                        ("label", Value::Message(Key::ExampleFormsNo)),
                    ],
                    children: &[],
                },
            ],
        }],
    ),
    (
        r"Fieldset { disabled: true, … }",
        &[Node::Component {
            name: "Fieldset",
            props: &[
                ("name", Value::Text("closed")),
                ("legend", Value::Message(Key::ExampleFormsClosedLegend)),
                ("hint", Value::Message(Key::ExampleFormsClosedHint)),
                ("disabled", Value::Bool(true)),
            ],
            children: &[
                Node::Component {
                    name: "Checkbox",
                    props: &[
                        ("name", Value::Text("closed")),
                        ("value", Value::Text("study")),
                        ("label", Value::Message(Key::ExampleFormsApplyStudy)),
                        ("checked", Value::Bool(true)),
                    ],
                    children: &[],
                },
                Node::Component {
                    name: "Checkbox",
                    props: &[
                        ("name", Value::Text("closed")),
                        ("value", Value::Text("work")),
                        ("label", Value::Message(Key::ExampleFormsApplyWork)),
                    ],
                    children: &[],
                },
            ],
        }],
    ),
    (
        r"Fieldset { ai_mark, ai_label, ai_note }",
        &[Node::Component {
            name: "Fieldset",
            props: &[
                ("ai_mark", Value::Message(Key::ExampleAiMark)),
                ("ai_label", Value::Message(Key::ExampleAiName)),
                ("ai_note", Value::Message(Key::ExampleAiNote)),
                ("name", Value::Text("notify-ai")),
                ("legend", Value::Message(Key::ExampleFormsContactLegend)),
                ("hint", Value::Message(Key::ExampleFormsContactHint)),
            ],
            children: &[
                Node::Component {
                    name: "Radio",
                    props: &[
                        ("name", Value::Text("notify-ai")),
                        ("value", Value::Text("email")),
                        ("label", Value::Message(Key::ExampleFormsContactEmail)),
                    ],
                    children: &[],
                },
                Node::Component {
                    name: "Radio",
                    props: &[
                        ("name", Value::Text("notify-ai")),
                        ("value", Value::Text("text")),
                        ("label", Value::Message(Key::ExampleFormsContactText)),
                    ],
                    children: &[],
                },
            ],
        }],
    ),
];
