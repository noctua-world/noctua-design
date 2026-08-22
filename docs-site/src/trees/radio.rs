//! `Radio`, as trees.
//!
//! One entry per example in `examples::RADIO`, in the same order. The
//! caption is repeated here so the pairing is pinned to something a
//! reader can check: `every_example_has_a_tree` reads it against
//! the table's own, and a tree that slid a row apart fails there rather
//! than drawing the wrong picture.

use noctua_design_i18n::Key;

use crate::tree::{Node, Value};
use crate::trees::Tree;

/// Every `Radio` example. 6 of them.
pub(crate) const EXAMPLES: &[Tree] = &[
    (
        r#"Fieldset { … } / Radio { name: "contact", value: "email", label: "…", checked: true }"#,
        &[Node::Component {
            name: "Fieldset",
            props: &[
                ("name", Value::Text("contact")),
                ("legend", Value::Message(Key::ExampleFormsContactLegend)),
                ("hint", Value::Message(Key::ExampleFormsContactHint)),
            ],
            children: &[
                Node::Component {
                    name: "Radio",
                    props: &[
                        ("name", Value::Text("contact")),
                        ("value", Value::Text("email")),
                        ("label", Value::Message(Key::ExampleFormsContactEmail)),
                        ("checked", Value::Bool(true)),
                    ],
                    children: &[],
                },
                Node::Component {
                    name: "Radio",
                    props: &[
                        ("name", Value::Text("contact")),
                        ("value", Value::Text("phone")),
                        ("label", Value::Message(Key::ExampleFormsContactPhone)),
                    ],
                    children: &[],
                },
                Node::Component {
                    name: "Radio",
                    props: &[
                        ("name", Value::Text("contact")),
                        ("value", Value::Text("text")),
                        ("label", Value::Message(Key::ExampleFormsContactText)),
                    ],
                    children: &[],
                },
            ],
        }],
    ),
    (
        r"Radio { size: RadioSize::Large | Medium | Small }",
        // THE VALUE IS THE ANSWER, NEVER THE SIZE. See `Checkbox` above.
        &[Node::Component {
            name: "Fieldset",
            props: &[
                ("name", Value::Text("contact-sizes")),
                ("legend", Value::Message(Key::ExampleFormsContactLegend)),
            ],
            children: &[
                Node::Component {
                    name: "Radio",
                    props: &[
                        ("name", Value::Text("contact-sizes")),
                        ("value", Value::Text("email")),
                        ("label", Value::Message(Key::ExampleFormsContactEmail)),
                        ("size", Value::Text("large")),
                    ],
                    children: &[],
                },
                Node::Component {
                    name: "Radio",
                    props: &[
                        ("name", Value::Text("contact-sizes")),
                        ("value", Value::Text("phone")),
                        ("label", Value::Message(Key::ExampleFormsContactPhone)),
                        ("size", Value::Text("medium")),
                    ],
                    children: &[],
                },
                Node::Component {
                    name: "Radio",
                    props: &[
                        ("name", Value::Text("contact-sizes")),
                        ("value", Value::Text("post")),
                        ("label", Value::Message(Key::ExampleFormsContactPost)),
                        ("size", Value::Text("small")),
                    ],
                    children: &[],
                },
            ],
        }],
    ),
    (
        r#"Radio { hint: "…", size: RadioSize::Medium }"#,
        &[Node::Component {
            name: "Fieldset",
            props: &[
                ("name", Value::Text("contact-hints")),
                ("legend", Value::Message(Key::ExampleFormsContactLegend)),
            ],
            children: &[
                Node::Component {
                    name: "Radio",
                    props: &[
                        ("name", Value::Text("contact-hints")),
                        ("value", Value::Text("email")),
                        ("label", Value::Message(Key::ExampleFormsContactEmail)),
                        ("hint", Value::Message(Key::ExampleFormsContactEmailHint)),
                        ("size", Value::Text("medium")),
                    ],
                    children: &[],
                },
                Node::Component {
                    name: "Radio",
                    props: &[
                        ("name", Value::Text("contact-hints")),
                        ("value", Value::Text("phone")),
                        ("label", Value::Message(Key::ExampleFormsContactPhone)),
                        ("hint", Value::Message(Key::ExampleFormsContactPhoneHint)),
                        ("size", Value::Text("medium")),
                    ],
                    children: &[],
                },
            ],
        }],
    ),
    (
        r#"Radio { divider: "…", reveal: true, … }"#,
        &[Node::Component {
            name: "Fieldset",
            props: &[
                ("name", Value::Text("contact-other")),
                ("legend", Value::Message(Key::ExampleFormsContactLegend)),
            ],
            children: &[
                Node::Component {
                    name: "Radio",
                    props: &[
                        ("name", Value::Text("contact-other")),
                        ("value", Value::Text("email")),
                        ("label", Value::Message(Key::ExampleFormsContactEmail)),
                    ],
                    children: &[],
                },
                Node::Component {
                    name: "Radio",
                    props: &[
                        ("name", Value::Text("contact-other")),
                        ("value", Value::Text("other")),
                        ("label", Value::Message(Key::ExampleFormsContactOther)),
                        ("divider", Value::Message(Key::ExampleFormsOr)),
                        ("reveal", Value::Bool(true)),
                    ],
                    children: &[Node::Component {
                        name: "Checkbox",
                        props: &[
                            ("name", Value::Text("contact-copy")),
                            ("value", Value::Text("yes")),
                            ("label", Value::Message(Key::ExampleFormsContactCopy)),
                            ("size", Value::Text("small")),
                        ],
                        children: &[],
                    }],
                },
            ],
        }],
    ),
    (
        r"Radio { disabled: true, invalid: true, hide_label }",
        &[Node::Component {
            name: "Fieldset",
            props: &[
                ("name", Value::Text("contact-states")),
                ("legend", Value::Message(Key::ExampleFormsContactLegend)),
            ],
            children: &[
                Node::Component {
                    name: "Radio",
                    props: &[
                        ("name", Value::Text("contact-states")),
                        ("value", Value::Text("email")),
                        ("label", Value::Message(Key::ExampleFormsContactEmail)),
                        ("invalid", Value::Bool(true)),
                    ],
                    children: &[],
                },
                Node::Component {
                    name: "Radio",
                    props: &[
                        ("name", Value::Text("contact-states")),
                        ("value", Value::Text("post")),
                        ("label", Value::Message(Key::ExampleFormsContactPost)),
                        ("hint", Value::Message(Key::ExampleFormsContactPostHint)),
                        ("disabled", Value::Bool(true)),
                    ],
                    children: &[],
                },
            ],
        }],
    ),
    (
        r"Radio { ai_mark, ai_label, ai_note }",
        &[Node::Component {
            name: "Fieldset",
            props: &[
                ("name", Value::Text("contact-ai")),
                ("legend", Value::Message(Key::ExampleFormsContactLegend)),
                ("hint", Value::Message(Key::ExampleFormsContactHint)),
            ],
            children: &[
                Node::Component {
                    name: "Radio",
                    props: &[
                        ("ai_mark", Value::Message(Key::ExampleAiMark)),
                        ("ai_label", Value::Message(Key::ExampleAiName)),
                        ("ai_note", Value::Message(Key::ExampleAiNote)),
                        ("name", Value::Text("contact-ai")),
                        ("value", Value::Text("email")),
                        ("label", Value::Message(Key::ExampleFormsContactEmail)),
                        ("checked", Value::Bool(true)),
                    ],
                    children: &[],
                },
                Node::Component {
                    name: "Radio",
                    props: &[
                        ("name", Value::Text("contact-ai")),
                        ("value", Value::Text("phone")),
                        ("label", Value::Message(Key::ExampleFormsContactPhone)),
                    ],
                    children: &[],
                },
                Node::Component {
                    name: "Radio",
                    props: &[
                        ("name", Value::Text("contact-ai")),
                        ("value", Value::Text("text")),
                        ("label", Value::Message(Key::ExampleFormsContactText)),
                    ],
                    children: &[],
                },
            ],
        }],
    ),
];
