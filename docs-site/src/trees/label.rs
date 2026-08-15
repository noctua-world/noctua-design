//! `Label`, as trees.
//!
//! One entry per example in `examples::LABEL`, in the same order. The
//! caption is repeated here so the pairing is pinned to something a
//! reader can check: `every_tree_belongs_to_its_example` reads it against
//! the table's own, and a tree that slid a row apart fails there rather
//! than drawing the wrong picture.

use noctua_design_i18n::Key;

use crate::tree::{Each, Node, Value};
use crate::trees::Tree;

/// Every `Label` example. 5 of them.
pub(crate) const EXAMPLES: &[Tree] = &[
    // A LABEL NAMING A CONTROL IT DOES NOT DRAW. The `<input>` here is a bare
    // one on purpose: `Field` and `Input` already compose this component, and
    // an example of them would be an example of them.
    (
        r#"Label { text, for_control: "…" }"#,
        &[Node::Element {
            tag: "div",
            attributes: &[("class", Value::Text("nd-example-stack"))],
            children: &[
                Node::Component {
                    name: "Label",
                    props: &[
                        ("text", Value::Message(Key::ExampleInputEmailLabel)),
                        ("for_control", Value::Text("example-label-email")),
                    ],
                    children: &[],
                },
                Node::Element {
                    tag: "input",
                    attributes: &[
                        ("class", Value::Text("nd-input-box")),
                        ("id", Value::Text("example-label-email")),
                        ("type", Value::Text("email")),
                        ("name", Value::Text("example-label-email")),
                        ("autocomplete", Value::Text("email")),
                    ],
                    children: &[],
                },
            ],
        }],
    ),
    // A GROUP'S NAME HAS TO BE A `<legend>`. Nothing else attaches a question
    // to every control under it, in every engine, with nothing to wire.
    (
        r"Label { kind: Legend, text, required, required_label }",
        &[Node::Element {
            tag: "fieldset",
            attributes: &[("class", Value::Text("nd-field"))],
            children: &[
                Node::Component {
                    name: "Label",
                    props: &[
                        ("kind", Value::Text("legend")),
                        ("text", Value::Message(Key::ExampleLabelNewsletter)),
                        ("required", Value::Bool(true)),
                        ("required_label", Value::Message(Key::ExampleInputRequired)),
                    ],
                    children: &[],
                },
                Node::Component {
                    name: "Checkbox",
                    props: &[
                        ("name", Value::Text("example-label-send")),
                        ("value", Value::Text("offers")),
                        ("label", Value::Message(Key::ExampleLabelOffers)),
                    ],
                    children: &[],
                },
                Node::Component {
                    name: "Checkbox",
                    props: &[
                        ("name", Value::Text("example-label-send")),
                        ("value", Value::Text("updates")),
                        ("label", Value::Message(Key::ExampleLabelUpdates)),
                    ],
                    children: &[],
                },
            ],
        }],
    ),
    // AND A GROUP THAT IS NOT A `<fieldset>` IS NAMED BY REFERENCE. A
    // `role="radiogroup"` is not labelable, so a `for` aimed at one names
    // nothing whatsoever.
    (
        r#"Label { kind: Name, for_control: "…" } / Segmented { labelledby }"#,
        &[Node::Element {
            tag: "div",
            attributes: &[("class", Value::Text("nd-example-stack"))],
            children: &[
                Node::Component {
                    name: "Label",
                    props: &[
                        ("kind", Value::Text("name")),
                        ("text", Value::Message(Key::ExampleLabelDensity)),
                        ("for_control", Value::Text("example-label-density")),
                    ],
                    children: &[],
                },
                Node::Component {
                    name: "Segmented",
                    props: &[
                        ("name", Value::Text("example-label-density")),
                        ("labelledby", Value::Text("example-label-density-label")),
                    ],
                    children: &[Node::Each {
                        over: Each::Rows(&[
                            &[
                                Value::Text("compact"),
                                Value::Message(Key::ExampleLabelCompact),
                                Value::Bool(false),
                            ],
                            &[
                                Value::Text("cosy"),
                                Value::Message(Key::ExampleLabelCosy),
                                Value::Bool(true),
                            ],
                            &[
                                Value::Text("roomy"),
                                Value::Message(Key::ExampleLabelRoomy),
                                Value::Bool(false),
                            ],
                        ]),
                        body: &[Node::Component {
                            name: "SegmentedItem",
                            props: &[
                                ("value", Value::Bound(0)),
                                ("label", Value::Bound(1)),
                                ("selected", Value::Bound(2)),
                            ],
                            children: &[],
                        }],
                    }],
                },
            ],
        }],
    ),
    // ONE WORD OR THE OTHER, NEVER BOTH AND NEVER AN ASTERISK.
    (
        r"Label { required, required_label } / Label { optional_label }",
        &[Node::Element {
            tag: "div",
            attributes: &[("class", Value::Text("nd-example-stack"))],
            children: &[
                Node::Component {
                    name: "Label",
                    props: &[
                        ("text", Value::Message(Key::ExampleInputEmailLabel)),
                        ("for_control", Value::Text("example-label-required")),
                        ("required", Value::Bool(true)),
                        ("required_label", Value::Message(Key::ExampleInputRequired)),
                    ],
                    children: &[],
                },
                Node::Component {
                    name: "Label",
                    props: &[
                        ("text", Value::Message(Key::ExampleInputPhoneLabel)),
                        ("for_control", Value::Text("example-label-optional")),
                        ("optional_label", Value::Message(Key::ExampleLabelOptional)),
                    ],
                    children: &[],
                },
            ],
        }],
    ),
    // THE THREE SIZES, ITERATED, for the reason `Input` states: this cannot
    // fall out of step with the enum the day a fourth is added.
    (
        r"Label { size } for size in LabelSize::ALL",
        &[Node::Element {
            tag: "div",
            attributes: &[("class", Value::Text("nd-example-stack"))],
            children: &[Node::Each {
                over: Each::Values {
                    component: "Label",
                    prop: "size",
                },
                body: &[Node::Component {
                    name: "Label",
                    props: &[
                        ("size", Value::Bound(0)),
                        ("text", Value::Message(Key::ExampleInputNameLabel)),
                        (
                            "for_control",
                            Value::Join(&[Value::Text("example-label-"), Value::Bound(0)]),
                        ),
                    ],
                    children: &[],
                }],
            }],
        }],
    ),
];
