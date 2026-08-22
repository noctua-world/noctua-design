//! `Checkbox`, as trees.
//!
//! One entry per example in `examples::CHECKBOX`, in the same order. The
//! caption is repeated here so the pairing is pinned to something a
//! reader can check: `every_example_has_a_tree` reads it against
//! the table's own, and a tree that slid a row apart fails there rather
//! than drawing the wrong picture.

use noctua_design_i18n::Key;

use crate::tree::{Node, Value};
use crate::trees::Tree;

/// Every `Checkbox` example. 6 of them.
pub(crate) const EXAMPLES: &[Tree] = &[
    (
        r#"Fieldset { … } / Checkbox { name: "apply", value: "study", label: "…" }"#,
        &[Node::Component {
            name: "Fieldset",
            props: &[
                ("name", Value::Text("apply")),
                ("legend", Value::Message(Key::ExampleFormsApplyLegend)),
                ("hint", Value::Message(Key::ExampleFormsApplyHint)),
            ],
            children: &[
                Node::Component {
                    name: "Checkbox",
                    props: &[
                        ("name", Value::Text("apply")),
                        ("value", Value::Text("study")),
                        ("label", Value::Message(Key::ExampleFormsApplyStudy)),
                    ],
                    children: &[],
                },
                Node::Component {
                    name: "Checkbox",
                    props: &[
                        ("name", Value::Text("apply")),
                        ("value", Value::Text("work")),
                        ("label", Value::Message(Key::ExampleFormsApplyWork)),
                        ("checked", Value::Bool(true)),
                    ],
                    children: &[],
                },
                Node::Component {
                    name: "Checkbox",
                    props: &[
                        ("name", Value::Text("apply")),
                        ("value", Value::Text("care")),
                        ("label", Value::Message(Key::ExampleFormsApplyCare)),
                    ],
                    children: &[],
                },
            ],
        }],
    ),
    // THE THREE SIZES AS REAL CONTROLS, not only as specimens in the variant
    // grid. The grid draws the box alone, which is what makes the prop legible
    // — but the claim the sizes exist for is that the *target* stays 44 pixels
    // while the box shrinks, and a drawn box with no input behind it cannot
    // show that. It is also what the browser suite measures.
    (
        r"Checkbox { size: CheckboxSize::Large | Medium | Small }",
        // THE VALUE IS THE ANSWER, NEVER THE SIZE. Two boxes posting
        // `large` and `medium` beside a third posting `care` is a paste
        // slip, and it teaches that `value` has something to do with the
        // prop this example is about.
        &[Node::Component {
            name: "Fieldset",
            props: &[
                ("name", Value::Text("apply-sizes")),
                ("legend", Value::Message(Key::ExampleFormsApplyLegend)),
            ],
            children: &[
                Node::Component {
                    name: "Checkbox",
                    props: &[
                        ("name", Value::Text("apply-sizes")),
                        ("value", Value::Text("study")),
                        ("label", Value::Message(Key::ExampleFormsApplyStudy)),
                        ("size", Value::Text("large")),
                    ],
                    children: &[],
                },
                Node::Component {
                    name: "Checkbox",
                    props: &[
                        ("name", Value::Text("apply-sizes")),
                        ("value", Value::Text("work")),
                        ("label", Value::Message(Key::ExampleFormsApplyWork)),
                        ("size", Value::Text("medium")),
                    ],
                    children: &[],
                },
                Node::Component {
                    name: "Checkbox",
                    props: &[
                        ("name", Value::Text("apply-sizes")),
                        ("value", Value::Text("care")),
                        ("label", Value::Message(Key::ExampleFormsApplyCare)),
                        ("size", Value::Text("small")),
                    ],
                    children: &[],
                },
            ],
        }],
    ),
    (
        r#"Checkbox { hint: "…", size: CheckboxSize::Small }"#,
        &[Node::Component {
            name: "Fieldset",
            props: &[
                ("name", Value::Text("apply-hints")),
                ("legend", Value::Message(Key::ExampleFormsApplyLegend)),
            ],
            children: &[
                Node::Component {
                    name: "Checkbox",
                    props: &[
                        ("name", Value::Text("apply-hints")),
                        ("value", Value::Text("study")),
                        ("label", Value::Message(Key::ExampleFormsApplyStudy)),
                        ("hint", Value::Message(Key::ExampleFormsApplyStudyHint)),
                        ("size", Value::Text("small")),
                    ],
                    children: &[],
                },
                Node::Component {
                    name: "Checkbox",
                    props: &[
                        ("name", Value::Text("apply-hints")),
                        ("value", Value::Text("work")),
                        ("label", Value::Message(Key::ExampleFormsApplyWork)),
                        ("hint", Value::Message(Key::ExampleFormsApplyWorkHint)),
                        ("size", Value::Text("small")),
                    ],
                    children: &[],
                },
            ],
        }],
    ),
    (
        r#"Checkbox { divider: "…", exclusive: true }"#,
        &[Node::Component {
            name: "Fieldset",
            props: &[
                ("name", Value::Text("apply-none")),
                ("legend", Value::Message(Key::ExampleFormsApplyLegend)),
            ],
            children: &[
                Node::Component {
                    name: "Checkbox",
                    props: &[
                        ("name", Value::Text("apply-none")),
                        ("value", Value::Text("study")),
                        ("label", Value::Message(Key::ExampleFormsApplyStudy)),
                    ],
                    children: &[],
                },
                Node::Component {
                    name: "Checkbox",
                    props: &[
                        ("name", Value::Text("apply-none")),
                        ("value", Value::Text("work")),
                        ("label", Value::Message(Key::ExampleFormsApplyWork)),
                    ],
                    children: &[],
                },
                Node::Component {
                    name: "Checkbox",
                    props: &[
                        ("name", Value::Text("apply-none")),
                        ("value", Value::Text("none")),
                        ("label", Value::Message(Key::ExampleFormsApplyNone)),
                        ("divider", Value::Message(Key::ExampleFormsOr)),
                        ("exclusive", Value::Bool(true)),
                    ],
                    children: &[],
                },
            ],
        }],
    ),
    (
        r"Checkbox { reveal: true, … }",
        &[Node::Component {
            name: "Fieldset",
            props: &[
                ("name", Value::Text("apply-other")),
                ("legend", Value::Message(Key::ExampleFormsApplyLegend)),
            ],
            children: &[
                Node::Component {
                    name: "Checkbox",
                    props: &[
                        ("name", Value::Text("apply-other")),
                        ("value", Value::Text("care")),
                        ("label", Value::Message(Key::ExampleFormsApplyCare)),
                    ],
                    children: &[],
                },
                Node::Component {
                    name: "Checkbox",
                    props: &[
                        ("name", Value::Text("apply-other")),
                        ("value", Value::Text("other")),
                        ("label", Value::Message(Key::ExampleFormsApplyOther)),
                        ("reveal", Value::Bool(true)),
                    ],
                    children: &[Node::Component {
                        name: "Paragraph",
                        props: &[("size", Value::Text("small"))],
                        children: &[Node::Text(Value::Message(
                            Key::ExampleFormsApplyOtherDetail,
                        ))],
                    }],
                },
            ],
        }],
    ),
    (
        r"Checkbox { checked: true, disabled: true, invalid: true, hide_label }",
        &[Node::Component {
            name: "Fieldset",
            props: &[
                ("name", Value::Text("apply-states")),
                ("legend", Value::Message(Key::ExampleFormsApplyLegend)),
            ],
            children: &[
                Node::Component {
                    name: "Checkbox",
                    props: &[
                        ("name", Value::Text("apply-states")),
                        ("value", Value::Text("study")),
                        ("label", Value::Message(Key::ExampleFormsApplyStudy)),
                        ("checked", Value::Bool(true)),
                    ],
                    children: &[],
                },
                Node::Component {
                    name: "Checkbox",
                    props: &[
                        ("name", Value::Text("apply-states")),
                        ("value", Value::Text("work")),
                        ("label", Value::Message(Key::ExampleFormsApplyWork)),
                        ("invalid", Value::Bool(true)),
                    ],
                    children: &[],
                },
                Node::Component {
                    name: "Checkbox",
                    props: &[
                        ("name", Value::Text("apply-states")),
                        ("value", Value::Text("care")),
                        ("label", Value::Message(Key::ExampleFormsApplyCare)),
                        ("hint", Value::Message(Key::ExampleFormsContactPostHint)),
                        ("disabled", Value::Bool(true)),
                    ],
                    children: &[],
                },
            ],
        }],
    ),
];
