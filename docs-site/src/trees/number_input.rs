//! `NumberInput`, as trees.
//!
//! One entry per example in `examples::NUMBER_INPUT`, in the same order. The
//! caption is repeated here so the pairing is pinned to something a
//! reader can check: `every_example_has_a_tree` reads it against
//! the table's own, and a tree that slid a row apart fails there rather
//! than drawing the wrong picture.

use noctua_design_i18n::Key;

use crate::tree::{Node, Value};
use crate::trees::Tree;

/// Every `NumberInput` example. 8 of them.
pub(crate) const EXAMPLES: &[Tree] = &[
    // ONE QUESTION, IN A FORM THAT SUBMITS. Press Save and the quantity
    // arrives in the address bar, which is the only way to show that this is a
    // real `<input>` with a real `name` rather than a picture of one.
    (
        r"NumberInput { name, label, value, min, max, step, big_step, increment_label, decrement_label, hint, required, required_label }",
        &[Node::Element {
            tag: "form",
            attributes: &[
                ("class", Value::Text("nd-example-form")),
                ("method", Value::Text("get")),
                ("action", Value::Text("#")),
                ("novalidate", Value::Bool(true)),
            ],
            children: &[
                Node::Component {
                    name: "NumberInput",
                    props: &[
                        ("name", Value::Text("tickets")),
                        ("label", Value::Message(Key::ExampleNumberInputTicketsLabel)),
                        ("value", Value::Text("2")),
                        ("min", Value::Text("1")),
                        ("max", Value::Text("20")),
                        ("step", Value::Text("1")),
                        ("big_step", Value::Text("5")),
                        (
                            "increment_label",
                            Value::Message(Key::ExampleInputIncrement),
                        ),
                        (
                            "decrement_label",
                            Value::Message(Key::ExampleInputDecrement),
                        ),
                        ("hint", Value::Message(Key::ExampleNumberInputTicketsHint)),
                        ("required", Value::Bool(true)),
                        ("required_label", Value::Message(Key::ExampleInputRequired)),
                    ],
                    children: &[],
                },
                Node::Component {
                    name: "Button",
                    props: &[
                        ("submit", Value::Text("submit")),
                        ("label", Value::Message(Key::ExampleInputSend)),
                    ],
                    children: &[],
                },
            ],
        }],
    ),
    // STEPPING IS THE ANSWER RATHER THAN A CORRECTION. `primary` is the shape
    // a quantity takes on a product page: one full control either side of the
    // value, each big enough to hit with a thumb, and the digits between them.
    // The markup is the same as the default's, so the arrow keys, the bounds
    // and the sentences are all still this control's.
    (
        r#"NumberInput { stepping: "primary", label, value, min, max, increment_label, decrement_label }"#,
        &[Node::Component {
            name: "NumberInput",
            props: &[
                ("name", Value::Text("cart")),
                ("label", Value::Message(Key::ExampleNumberInputCartLabel)),
                ("stepping", Value::Text("primary")),
                ("value", Value::Text("1")),
                ("min", Value::Text("1")),
                ("max", Value::Text("12")),
                (
                    "increment_label",
                    Value::Message(Key::ExampleNumberInputMore),
                ),
                (
                    "decrement_label",
                    Value::Message(Key::ExampleNumberInputLess),
                ),
            ],
            children: &[],
        }],
    ),
    // REFUSED AND EXPLAINED, NEVER REWRITTEN. Type 40, leave the box, and the
    // sentence under the question is the caller's own. The number stays as it
    // was typed, so a reader can see what was wrong with it.
    (
        r"NumberInput { name, label, value, min, max, min_label, max_label, invalid_label, increment_label, decrement_label }",
        &[Node::Component {
            name: "NumberInput",
            props: &[
                ("name", Value::Text("boxes")),
                ("label", Value::Message(Key::ExampleNumberInputBoxesLabel)),
                ("value", Value::Text("5")),
                ("min", Value::Text("1")),
                ("max", Value::Text("10")),
                ("min_label", Value::Message(Key::ExampleInputTooSmall)),
                ("max_label", Value::Message(Key::ExampleInputTooBig)),
                ("invalid_label", Value::Message(Key::ExampleInputNotANumber)),
                (
                    "increment_label",
                    Value::Message(Key::ExampleInputIncrement),
                ),
                (
                    "decrement_label",
                    Value::Message(Key::ExampleInputDecrement),
                ),
            ],
            children: &[],
        }],
    ),
    // MONEY, AND WHAT `group` PROMISES. The separators are the page's own and
    // they are drawn while nobody is typing. This one submits as well, because
    // "never submitted" is a claim only a form can settle.
    (
        r"NumberInput { name, label, value, min, max, step, prefix, group, placeholder, size, increment_label, decrement_label }",
        &[Node::Element {
            tag: "form",
            attributes: &[
                ("class", Value::Text("nd-example-form")),
                ("method", Value::Text("get")),
                ("action", Value::Text("#")),
                ("novalidate", Value::Bool(true)),
            ],
            children: &[
                Node::Component {
                    name: "NumberInput",
                    props: &[
                        ("name", Value::Text("price")),
                        ("label", Value::Message(Key::ExampleNumberInputPriceLabel)),
                        ("value", Value::Text("1499.9")),
                        ("min", Value::Text("0")),
                        ("max", Value::Text("9999.99")),
                        ("step", Value::Text("0.01")),
                        ("prefix", Value::Data("R$")),
                        ("group", Value::Bool(true)),
                        ("placeholder", Value::Data("0,00")),
                        ("size", Value::Text("large")),
                        (
                            "increment_label",
                            Value::Message(Key::ExampleNumberInputMore),
                        ),
                        (
                            "decrement_label",
                            Value::Message(Key::ExampleNumberInputLess),
                        ),
                    ],
                    children: &[],
                },
                Node::Component {
                    name: "Button",
                    props: &[
                        ("submit", Value::Text("submit")),
                        ("label", Value::Message(Key::ExampleInputSend)),
                    ],
                    children: &[],
                },
            ],
        }],
    ),
    // A MEASUREMENT, WITH ITS UNIT INSIDE THE BOX. `step` carries the decimal
    // here: it sets what one press moves, how many decimals are drawn, and
    // which keyboard a phone opens.
    (
        r"NumberInput { name, label, value, min, max, step, suffix, increment_label, decrement_label }",
        &[Node::Component {
            name: "NumberInput",
            props: &[
                ("name", Value::Text("weight")),
                ("label", Value::Message(Key::ExampleNumberInputWeightLabel)),
                ("value", Value::Text("4.5")),
                ("min", Value::Text("0")),
                ("max", Value::Text("30")),
                ("step", Value::Text("0.1")),
                ("suffix", Value::Data("kg")),
                (
                    "increment_label",
                    Value::Message(Key::ExampleNumberInputMore),
                ),
                (
                    "decrement_label",
                    Value::Message(Key::ExampleNumberInputLess),
                ),
            ],
            children: &[],
        }],
    ),
    // TOO LARGE TO STEP TO, which is the one reason to turn the two controls
    // off. The arrow keys and Page Up still move it, and `big_step` is what
    // makes crossing this range from the keyboard possible at all.
    (
        r"NumberInput { name, label, value, step, big_step, steppers, group }",
        &[Node::Component {
            name: "NumberInput",
            props: &[
                ("name", Value::Text("words")),
                ("label", Value::Message(Key::ExampleNumberInputWordsLabel)),
                ("value", Value::Text("50000")),
                ("step", Value::Text("500")),
                ("big_step", Value::Text("5000")),
                ("steppers", Value::Bool(false)),
                ("group", Value::Bool(true)),
            ],
            children: &[],
        }],
    ),
    // REFUSED, LOCKED AND UNAVAILABLE — three states, each on a question the
    // state makes sense on. The error sits on the number that was refused, and
    // the locked value carries none, because there is nothing to correct.
    (
        r"NumberInput { name, label, value, max, error, increment_label, decrement_label }
NumberInput { name, label, value, readonly, size, increment_label, decrement_label }
NumberInput { name, label, disabled, glass, increment_label, decrement_label }",
        &[Node::Element {
            tag: "div",
            attributes: &[("class", Value::Text("nd-example-stack"))],
            children: &[
                Node::Component {
                    name: "NumberInput",
                    props: &[
                        ("name", Value::Text("seats")),
                        ("label", Value::Message(Key::ExampleNumberInputSeatsLabel)),
                        ("value", Value::Text("6")),
                        ("max", Value::Text("4")),
                        ("error", Value::Message(Key::ExampleNumberInputError)),
                        (
                            "increment_label",
                            Value::Message(Key::ExampleInputIncrement),
                        ),
                        (
                            "decrement_label",
                            Value::Message(Key::ExampleInputDecrement),
                        ),
                    ],
                    children: &[],
                },
                Node::Component {
                    name: "NumberInput",
                    props: &[
                        ("name", Value::Text("packed")),
                        ("label", Value::Message(Key::ExampleNumberInputPackedLabel)),
                        ("value", Value::Text("12")),
                        ("readonly", Value::Bool(true)),
                        ("size", Value::Text("small")),
                        (
                            "increment_label",
                            Value::Message(Key::ExampleInputIncrement),
                        ),
                        (
                            "decrement_label",
                            Value::Message(Key::ExampleInputDecrement),
                        ),
                    ],
                    children: &[],
                },
                Node::Component {
                    name: "NumberInput",
                    props: &[
                        ("name", Value::Text("bags")),
                        ("label", Value::Message(Key::ExampleNumberInputBagsLabel)),
                        ("disabled", Value::Bool(true)),
                        ("glass", Value::Bool(true)),
                        (
                            "increment_label",
                            Value::Message(Key::ExampleInputIncrement),
                        ),
                        (
                            "decrement_label",
                            Value::Message(Key::ExampleInputDecrement),
                        ),
                    ],
                    children: &[],
                },
            ],
        }],
    ),
    (
        r"NumberInput { ai_mark, ai_label, ai_note }",
        &[Node::Element {
            tag: "form",
            attributes: &[
                ("class", Value::Text("nd-example-form")),
                ("method", Value::Text("get")),
                ("action", Value::Text("#")),
                ("novalidate", Value::Bool(true)),
            ],
            children: &[
                Node::Component {
                    name: "NumberInput",
                    props: &[
                        ("ai_mark", Value::Message(Key::ExampleAiMark)),
                        ("ai_label", Value::Message(Key::ExampleAiName)),
                        ("ai_note", Value::Message(Key::ExampleAiNote)),
                        ("name", Value::Text("tickets-ai")),
                        ("label", Value::Message(Key::ExampleAiSubject)),
                        ("value", Value::Text("2")),
                        ("min", Value::Text("1")),
                        ("max", Value::Text("20")),
                        ("step", Value::Text("1")),
                        ("big_step", Value::Text("5")),
                        (
                            "increment_label",
                            Value::Message(Key::ExampleInputIncrement),
                        ),
                        (
                            "decrement_label",
                            Value::Message(Key::ExampleInputDecrement),
                        ),
                        ("hint", Value::Message(Key::ExampleNumberInputTicketsHint)),
                        ("required", Value::Bool(true)),
                        ("required_label", Value::Message(Key::ExampleInputRequired)),
                    ],
                    children: &[],
                },
                Node::Component {
                    name: "Button",
                    props: &[
                        ("submit", Value::Text("submit")),
                        ("label", Value::Message(Key::ExampleInputSend)),
                    ],
                    children: &[],
                },
            ],
        }],
    ),
];
