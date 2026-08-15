//! `Progress`, as trees.
//!
//! One entry per example in `examples::PROGRESS`, in the same order. The
//! caption is repeated here so the pairing is pinned to something a
//! reader can check: `every_tree_belongs_to_its_example` reads it against
//! the table's own, and a tree that slid a row apart fails there rather
//! than drawing the wrong picture.

use noctua_design_i18n::Key;

use crate::tree::{Node, Value};
use crate::trees::Tree;

/// Every `Progress` example. 4 of them.
pub(crate) const EXAMPLES: &[Tree] = &[
    (
        r"Progress { name, label, value, max, value_label }",
        &[Node::Component {
            name: "Progress",
            props: &[
                ("name", Value::Text("upload")),
                ("label", Value::Message(Key::ExampleProgressUpload)),
                ("value", Value::Text("7.4")),
                ("max", Value::Text("12")),
                (
                    "value_label",
                    Value::Message(Key::ExampleProgressUploadValue),
                ),
            ],
            children: &[],
        }],
    ),
    (
        r"Progress { busy_label } — no value at all",
        &[Node::Component {
            name: "Progress",
            props: &[
                ("name", Value::Text("import")),
                ("label", Value::Message(Key::ExampleProgressImport)),
                ("busy_label", Value::Message(Key::ExampleProgressImportBusy)),
            ],
            children: &[],
        }],
    ),
    (
        r#"Progress { tone: "danger" }"#,
        &[Node::Component {
            name: "Progress",
            props: &[
                ("name", Value::Text("failed")),
                ("label", Value::Message(Key::ExampleProgressFailed)),
                ("value", Value::Text("0.38")),
                ("tone", Value::Text("danger")),
                (
                    "value_label",
                    Value::Message(Key::ExampleProgressFailedValue),
                ),
            ],
            children: &[],
        }],
    ),
    (
        r"Progress { name, label, value, max, size: Large, value_label }
Progress { name, label, value, max, size: Small, hide_label: true, value_label }",
        &[Node::Element {
            tag: "div",
            attributes: &[("class", Value::Text("nd-example-stack"))],
            children: &[
                Node::Component {
                    name: "Progress",
                    props: &[
                        ("name", Value::Text("steps-large")),
                        ("label", Value::Message(Key::ExampleProgressSteps)),
                        ("value", Value::Text("3200")),
                        ("max", Value::Text("4200")),
                        ("size", Value::Text("large")),
                        (
                            "value_label",
                            Value::Message(Key::ExampleProgressStepsValue),
                        ),
                    ],
                    children: &[],
                },
                Node::Component {
                    name: "Progress",
                    props: &[
                        ("name", Value::Text("steps-small")),
                        ("label", Value::Message(Key::ExampleProgressSteps)),
                        ("value", Value::Text("3200")),
                        ("max", Value::Text("4200")),
                        ("size", Value::Text("small")),
                        ("hide_label", Value::Bool(true)),
                        (
                            "value_label",
                            Value::Message(Key::ExampleProgressStepsValue),
                        ),
                    ],
                    children: &[],
                },
            ],
        }],
    ),
];
