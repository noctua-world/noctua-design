//! `Rating`, as trees.
//!
//! One entry per example in `examples::RATING`, in the same order. The
//! caption is repeated here so the pairing is pinned to something a
//! reader can check: `every_example_has_a_tree` reads it against
//! the table's own, and a tree that slid a row apart fails there rather
//! than drawing the wrong picture.

use noctua_design_i18n::Key;

use crate::tree::{Each, Node, Value};
use crate::trees::Tree;

/// Every `Rating` example. 5 of them.
pub(crate) const EXAMPLES: &[Tree] = &[
    // A READING, AND THE NUMBER IS WRITTEN OUT. The fifth star is filled three
    // tenths of the way, because rounding it would be a picture that disagrees
    // with the sentence beside it.
    (
        r#"Rating { name, label, readonly: true, value: "4.3", max: "5", value_label }"#,
        &[Node::Component {
            name: "Rating",
            props: &[
                ("name", Value::Text("rating-read")),
                ("label", Value::Message(Key::ExampleRatingProduct)),
                ("readonly", Value::Bool(true)),
                ("value", Value::Text("4.3")),
                ("max", Value::Text("5")),
                ("value_label", Value::Message(Key::ExampleRatingValue)),
            ],
            children: &[],
        }],
    ),
    // THE QUESTION. Real radios in a real fieldset, so it submits with no
    // script, and the number follows the reader's own choice.
    (
        r"Rating { item_label, value_label, hint, required: true, required_label }",
        &[Node::Component {
            name: "Rating",
            props: &[
                ("name", Value::Text("rating-ask")),
                ("label", Value::Message(Key::ExampleRatingDelivery)),
                ("value_label", Value::Message(Key::ExampleRatingValue)),
                ("item_label", Value::Message(Key::ExampleRatingItem)),
                ("hint", Value::Message(Key::ExampleRatingHint)),
                ("required", Value::Bool(true)),
                ("required_label", Value::Message(Key::ExampleRatingRequired)),
            ],
            children: &[],
        }],
    ),
    (
        r#"Rating { name, label, readonly: true, value: "4", tone: "warning", size: Small | Medium | Large, value_label }"#,
        &[Node::Element {
            tag: "div",
            attributes: &[("class", Value::Text("nd-example-stack"))],
            children: &[Node::Each {
                over: Each::Rows(&[
                    &[Value::Text("small"), Value::Text("small")],
                    &[Value::Text("medium"), Value::Text("medium")],
                    &[Value::Text("large"), Value::Text("large")],
                ]),
                body: &[Node::Component {
                    name: "Rating",
                    props: &[
                        (
                            "name",
                            Value::Join(&[Value::Text("rating-size-"), Value::Bound(0)]),
                        ),
                        ("label", Value::Message(Key::ExampleRatingProduct)),
                        ("readonly", Value::Bool(true)),
                        ("value", Value::Text("4")),
                        ("tone", Value::Text("warning")),
                        ("size", Value::Bound(1)),
                        ("value_label", Value::Message(Key::ExampleRatingValue)),
                    ],
                    children: &[],
                }],
            }],
        }],
    ),
    // A REFUSED ANSWER MARKS EVERY STAR, because the score is one value. And a
    // rating nobody may change any more takes no focus at all.
    (
        r"Rating { name, label, value_label, item_label, error }
Rating { name, label, value, value_label, item_label, hint, disabled: true }",
        &[Node::Element {
            tag: "div",
            attributes: &[("class", Value::Text("nd-example-stack"))],
            children: &[
                Node::Component {
                    name: "Rating",
                    props: &[
                        ("name", Value::Text("rating-error")),
                        ("label", Value::Message(Key::ExampleRatingDelivery)),
                        ("value_label", Value::Message(Key::ExampleRatingValue)),
                        ("item_label", Value::Message(Key::ExampleRatingItem)),
                        ("error", Value::Message(Key::ExampleRatingError)),
                    ],
                    children: &[],
                },
                Node::Component {
                    name: "Rating",
                    props: &[
                        ("name", Value::Text("rating-off")),
                        ("label", Value::Message(Key::ExampleRatingDelivery)),
                        ("value", Value::Text("2")),
                        ("value_label", Value::Message(Key::ExampleRatingValue)),
                        ("item_label", Value::Message(Key::ExampleRatingItem)),
                        ("hint", Value::Message(Key::ExampleRatingClosed)),
                        ("disabled", Value::Bool(true)),
                    ],
                    children: &[],
                },
            ],
        }],
    ),
    (
        r"Rating { ai_mark, ai_label, ai_note }",
        &[Node::Component {
            name: "Rating",
            props: &[
                ("ai_mark", Value::Message(Key::ExampleAiMark)),
                ("ai_label", Value::Message(Key::ExampleAiName)),
                ("ai_note", Value::Message(Key::ExampleAiNote)),
                ("name", Value::Text("rating-read-ai")),
                ("label", Value::Message(Key::ExampleRatingProduct)),
                ("readonly", Value::Bool(true)),
                ("value", Value::Text("4.3")),
                ("max", Value::Text("5")),
                ("value_label", Value::Message(Key::ExampleRatingValue)),
            ],
            children: &[],
        }],
    ),
];
