//! `AILabel`, as trees.
//!
//! One entry per example in `examples::AI_LABEL`, in the same order. The
//! caption is repeated here so the pairing is pinned to something a
//! reader can check: `every_example_has_a_tree` reads it against
//! the table's own, and a tree that slid a row apart fails there rather
//! than drawing the wrong picture.
//!
//! EVERY `mark` HERE IS A MESSAGE. The two letters are a translation and not a
//! name, so `Value::Data` would put an unbracketed run on the page and
//! `.nd-identifier` would tell the pseudo-locale scanner to skip the one string
//! it most needs to watch. `the_letters_of_every_mark_are_a_message` in
//! `trees::structure` is what holds every tree to it.

use noctua_design_i18n::Key;

use crate::tree::{Each, Node, Value};
use crate::trees::Tree;

/// Every `AILabel` example. 5 of them.
pub(crate) const EXAMPLES: &[Tree] = &[
    (
        r"AILabel { mark, label } beside the value it marks",
        &[Node::Component {
            name: "Paragraph",
            props: &[],
            children: &[
                Node::Text(Value::Join(&[
                    Value::Message(Key::ExampleAiLabelSummary),
                    Value::Text(" "),
                ])),
                Node::Component {
                    name: "AILabel",
                    props: &[
                        ("mark", Value::Message(Key::ExampleAiMark)),
                        ("label", Value::Message(Key::ExampleAiLabelNameSummary)),
                    ],
                    children: &[],
                },
            ],
        }],
    ),
    (
        r"AILabel { note, close_label, placement }",
        &[Node::Element {
            tag: "div",
            attributes: &[("class", Value::Text("nd-example-row nd-example-row-airy"))],
            children: &[
                Node::Component {
                    name: "AILabel",
                    props: &[
                        ("mark", Value::Message(Key::ExampleAiMark)),
                        ("label", Value::Message(Key::ExampleAiLabelNameTotal)),
                        ("note", Value::Message(Key::ExampleAiLabelNoteTotal)),
                        ("close_label", Value::Message(Key::ExampleAiClose)),
                    ],
                    children: &[],
                },
                Node::Component {
                    name: "AILabel",
                    props: &[
                        ("mark", Value::Message(Key::ExampleAiMark)),
                        ("label", Value::Message(Key::ExampleAiLabelNameFigure)),
                        ("note", Value::Message(Key::ExampleAiLabelNoteFigure)),
                        ("placement", Value::Text("start")),
                    ],
                    children: &[],
                },
                Node::Component {
                    name: "AILabel",
                    props: &[
                        ("mark", Value::Message(Key::ExampleAiMark)),
                        ("label", Value::Message(Key::ExampleAiLabelNameDraft)),
                        ("note", Value::Message(Key::ExampleAiLabelNoteDraft)),
                        ("placement", Value::Text("end")),
                    ],
                    children: &[],
                },
            ],
        }],
    ),
    (
        r"AILabel { variant } for variant in AILabelVariant::ALL",
        &[Node::Element {
            tag: "div",
            attributes: &[("class", Value::Text("nd-example-row nd-example-row-airy"))],
            children: &[Node::Each {
                over: Each::Values {
                    component: "AILabel",
                    prop: "variant",
                },
                body: &[Node::Component {
                    name: "AILabel",
                    props: &[
                        ("mark", Value::Message(Key::ExampleAiMark)),
                        ("label", Value::Message(Key::ExampleAiLabelNameSummary)),
                        ("variant", Value::Bound(0)),
                    ],
                    children: &[],
                }],
            }],
        }],
    ),
    (
        r"AILabel { size } for size in AILabelSize::ALL",
        &[Node::Element {
            tag: "div",
            attributes: &[("class", Value::Text("nd-example-row nd-example-row-airy"))],
            children: &[Node::Each {
                over: Each::Values {
                    component: "AILabel",
                    prop: "size",
                },
                body: &[Node::Component {
                    name: "AILabel",
                    props: &[
                        ("mark", Value::Message(Key::ExampleAiMark)),
                        ("label", Value::Message(Key::ExampleAiLabelNameSummary)),
                        ("size", Value::Bound(0)),
                    ],
                    children: &[],
                }],
            }],
        }],
    ),
    (
        r#"AILabel { tone: "pending" }  ·  AILabel { tone: "approved" }"#,
        &[Node::Element {
            tag: "div",
            attributes: &[("class", Value::Text("nd-example-row nd-example-row-airy"))],
            children: &[
                Node::Component {
                    name: "AILabel",
                    props: &[
                        ("mark", Value::Message(Key::ExampleAiMark)),
                        ("label", Value::Message(Key::ExampleAiLabelNameFigure)),
                        ("tone", Value::Text("pending")),
                    ],
                    children: &[],
                },
                Node::Component {
                    name: "AILabel",
                    props: &[
                        ("mark", Value::Message(Key::ExampleAiMark)),
                        ("label", Value::Message(Key::ExampleAiLabelNameTotal)),
                        ("tone", Value::Text("approved")),
                        ("variant", Value::Text("solid")),
                    ],
                    children: &[],
                },
            ],
        }],
    ),
];
