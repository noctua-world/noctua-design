//! `Field`, as trees.
//!
//! One entry per example in `examples::FIELD`, in the same order. The
//! caption is repeated here so the pairing is pinned to something a
//! reader can check: `every_tree_belongs_to_its_example` reads it against
//! the table's own, and a tree that slid a row apart fails there rather
//! than drawing the wrong picture.

use noctua_design_i18n::Key;

use crate::tree::{Node, Value};
use crate::trees::Tree;

/// Every `Field` example. 5 of them.
pub(crate) const EXAMPLES: &[Tree] = &[
    (
        r"Field { control, label, hint }",
        &[Node::Component {
            name: "Input",
            props: &[
                ("name", Value::Text("field-plain")),
                ("label", Value::Message(Key::ExampleInputNameLabel)),
                ("hint", Value::Message(Key::ExampleInputHint)),
                ("autocomplete", Value::Text("name")),
            ],
            children: &[],
        }],
    ),
    (
        r"Field { required, required_label }",
        &[Node::Component {
            name: "Input",
            props: &[
                ("name", Value::Text("field-required")),
                ("label", Value::Message(Key::ExampleInputEmailLabel)),
                ("kind", Value::Text("email")),
                ("required", Value::Bool(true)),
                ("required_label", Value::Message(Key::ExampleInputRequired)),
                ("autocomplete", Value::Text("email")),
            ],
            children: &[],
        }],
    ),
    (
        r"Field { error, error_label } / Field { error_slot } / Field { disabled }",
        &[Node::Element {
            tag: "div",
            attributes: &[("class", Value::Text("nd-example-stack"))],
            children: &[
                Node::Component {
                    name: "Input",
                    props: &[
                        ("name", Value::Text("field-wrong")),
                        ("label", Value::Message(Key::ExampleInputEmailLabel)),
                        ("kind", Value::Text("email")),
                        ("error", Value::Message(Key::ExampleInputError)),
                    ],
                    children: &[],
                },
                // The slot, rendered empty. `Select` and `DateInput` each arrived
                // at needing one, which is why `Field` has it: a control whose
                // real element the script hides cannot be reported by the browser,
                // so this is where its own complaint goes.
                // `error_label` IS THE WORD ANNOUNCED BEFORE AN ERROR, so it
                // is *Error:* and never *required*. Setting it to the word
                // that marks a compulsory question makes every complaint on
                // the field announce as the field being obligatory.
                Node::Component {
                    name: "Select",
                    props: &[
                        ("name", Value::Text("field-slot")),
                        ("label", Value::Message(Key::ExampleInputNameLabel)),
                        ("required", Value::Bool(true)),
                        ("error_label", Value::Message(Key::ExampleFormsErrorWord)),
                    ],
                    children: &[Node::Component {
                        name: "SelectOption",
                        props: &[("value", Value::Text("one")), ("label", Value::Data("1"))],
                        children: &[],
                    }],
                },
                Node::Component {
                    name: "Input",
                    props: &[
                        ("name", Value::Text("field-off")),
                        ("label", Value::Message(Key::ExampleInputNameLabel)),
                        ("disabled", Value::Bool(true)),
                    ],
                    children: &[],
                },
            ],
        }],
    ),
    (
        r"Field { size }",
        &[Node::Element {
            tag: "div",
            attributes: &[("class", Value::Text("nd-example-stack"))],
            children: &[
                Node::Component {
                    name: "Input",
                    props: &[
                        ("name", Value::Text("field-small")),
                        ("label", Value::Message(Key::ExampleInputNameLabel)),
                        ("size", Value::Text("small")),
                    ],
                    children: &[],
                },
                Node::Component {
                    name: "Input",
                    props: &[
                        ("name", Value::Text("field-large")),
                        ("label", Value::Message(Key::ExampleInputNameLabel)),
                        ("size", Value::Text("large")),
                        ("hint", Value::Message(Key::ExampleInputHint)),
                    ],
                    children: &[],
                },
            ],
        }],
    ),
    (
        r"Field { kind: Group } — a `<legend>`, because a `<label>` names one element",
        &[Node::Component {
            name: "TimeInput",
            props: &[
                ("name", Value::Text("field-group")),
                ("legend", Value::Message(Key::ExampleInputTimeLegend)),
                ("hour_label", Value::Message(Key::ExampleInputHour)),
                ("minute_label", Value::Message(Key::ExampleInputMinute)),
                ("hint", Value::Message(Key::ExampleInputTimeHint)),
            ],
            children: &[],
        }],
    ),
];
