//! `Input`, as trees.
//!
//! One entry per example in `examples::INPUT`, in the same order. The
//! caption is repeated here so the pairing is pinned to something a
//! reader can check: `every_tree_belongs_to_its_example` reads it against
//! the table's own, and a tree that slid a row apart fails there rather
//! than drawing the wrong picture.

use noctua_design_i18n::Key;

use crate::tree::{Each, Node, Value};
use crate::trees::Tree;

/// Every `Input` example. 6 of them.
pub(crate) const EXAMPLES: &[Tree] = &[
    // ONE FIELD, IN A FORM THAT SUBMITS. Press it and the answer arrives in the
    // address bar, which is the only way to show that the box is a real
    // `<input>` with a real `name` rather than a picture of one.
    (
        r"Input { name, label, value, hint, required, required_label, autocomplete }",
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
                    name: "Input",
                    props: &[
                        ("name", Value::Text("who")),
                        ("label", Value::Message(Key::ExampleInputNameLabel)),
                        ("value", Value::Message(Key::ExampleInputNameValue)),
                        ("hint", Value::Message(Key::ExampleInputHint)),
                        ("required", Value::Bool(true)),
                        ("required_label", Value::Message(Key::ExampleInputRequired)),
                        ("autocomplete", Value::Text("name")),
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
    // THE KEYBOARD A PHONE OPENS. Three kinds, three different questions, and
    // three different labels — a row of boxes all called *Website* is the
    // defect this example replaced.
    (
        r"Input { kind, placeholder, pattern, spellcheck, prefix }",
        &[Node::Element {
            tag: "div",
            attributes: &[("class", Value::Text("nd-example-stack"))],
            children: &[
                Node::Component {
                    name: "Input",
                    props: &[
                        ("name", Value::Text("where")),
                        ("label", Value::Message(Key::ExampleInputEmailLabel)),
                        ("kind", Value::Text("email")),
                        ("placeholder", Value::Data("ada@example.org")),
                        ("autocomplete", Value::Text("email")),
                        ("pattern", Value::Text(r".+@.+\..+")),
                    ],
                    children: &[],
                },
                // THE ONE BOX HERE THAT HOLDS WORDS, and the only one where
                // `spellcheck` has anything to say. It is **off** by default
                // on an `Input`, which is the opposite of the platform, so
                // `false` on the address above showed the prop doing nothing.
                Node::Component {
                    name: "Input",
                    props: &[
                        ("name", Value::Text("title")),
                        ("label", Value::Message(Key::ExampleInputTitleLabel)),
                        ("spellcheck", Value::Bool(true)),
                    ],
                    children: &[],
                },
                Node::Component {
                    name: "Input",
                    props: &[
                        ("name", Value::Text("phone")),
                        ("label", Value::Message(Key::ExampleInputPhoneLabel)),
                        ("hint", Value::Message(Key::ExampleInputPhoneHint)),
                        ("kind", Value::Text("tel")),
                        ("prefix", Value::Data("+55")),
                        ("autocomplete", Value::Text("tel")),
                    ],
                    children: &[],
                },
                Node::Component {
                    name: "Input",
                    props: &[
                        ("name", Value::Text("site")),
                        ("label", Value::Message(Key::ExampleInputSiteLabel)),
                        ("kind", Value::Text("url")),
                        ("prefix", Value::Data("https://")),
                        ("autocomplete", Value::Text("url")),
                    ],
                    children: &[],
                },
            ],
        }],
    ),
    // A PASSWORD A READER CAN CHECK BEFORE SENDING IT. Both words ship, because
    // the button says two different things depending on which state it is in.
    (
        r"Input { kind: Password, reveal_label, conceal_label }",
        &[Node::Component {
            name: "Input",
            props: &[
                ("name", Value::Text("secret")),
                ("label", Value::Message(Key::ExampleInputPasswordLabel)),
                ("kind", Value::Text("password")),
                ("hint", Value::Message(Key::ExampleInputPasswordHint)),
                ("autocomplete", Value::Text("new-password")),
                ("reveal_label", Value::Message(Key::ExampleInputReveal)),
                ("conceal_label", Value::Message(Key::ExampleInputConceal)),
            ],
            children: &[],
        }],
    ),
    // WHAT THE BOX CARRIES BESIDE THE TEXT: ONE ORNAMENT AT EACH END, AND A
    // COUNT OF WHAT IS BETWEEN THEM.
    //
    // It carried an `icon` AND a `prefix`, both at the leading end, so the box
    // opened with a person and an `@` doing one job twice. `prefix` is already
    // shown twice in the example above, on `+55` and on `https://`, and
    // nowhere else does an icon say what kind of answer a box wants, so the
    // `@` is the one that goes.
    //
    // AND IT SHIPS A VALUE, because the counter counts the value. Empty, it
    // read *0 of 24 characters* under a box with `.noctua` visibly in it,
    // which is the counter appearing to disagree with the box. Filled, the
    // same sentence says the affix is drawn and never counted.
    (
        r"Input { icon, suffix, value, clearable, clear_label, maxlength, counter, counter_label }",
        &[Node::Component {
            name: "Input",
            props: &[
                ("name", Value::Text("handle")),
                ("label", Value::Message(Key::ExampleInputHandleLabel)),
                ("hint", Value::Message(Key::ExampleInputHandleHint)),
                ("icon", Value::Text("user")),
                ("value", Value::Data("ada.lovelace")),
                ("suffix", Value::Data(".noctua")),
                ("clearable", Value::Bool(true)),
                ("clear_label", Value::Message(Key::ExampleInputClear)),
                ("maxlength", Value::Text("24")),
                ("counter", Value::Bool(true)),
                ("counter_label", Value::Message(Key::ExampleInputCounter)),
            ],
            children: &[],
        }],
    ),
    // THE THREE SIZES, ITERATED RATHER THAN LISTED, so this cannot fall out of
    // step with the enum the day a fourth is added.
    (
        r"Input { size } for size in InputSize::ALL",
        &[Node::Element {
            tag: "div",
            attributes: &[("class", Value::Text("nd-example-stack"))],
            children: &[Node::Each {
                over: Each::Values {
                    component: "Input",
                    prop: "size",
                },
                body: &[Node::Component {
                    name: "Input",
                    props: &[
                        (
                            "name",
                            Value::Join(&[Value::Text("size-"), Value::Bound(0)]),
                        ),
                        ("label", Value::Message(Key::ExampleInputNameLabel)),
                        ("size", Value::Bound(0)),
                        ("autocomplete", Value::Text("name")),
                    ],
                    children: &[],
                }],
            }],
        }],
    ),
    // REFUSED, LOCKED AND UNAVAILABLE — three different states, each on a box
    // the state makes sense on. The error sits on the address that was refused,
    // not on a reference number nobody can edit.
    (
        r"Input { name, label, kind: Email, value, error, autocomplete }
Input { name, label, value, readonly }
Input { name, label, kind: Password, reveal_label, conceal_label, disabled, glass }",
        &[Node::Element {
            tag: "div",
            attributes: &[("class", Value::Text("nd-example-stack"))],
            children: &[
                Node::Component {
                    name: "Input",
                    props: &[
                        ("name", Value::Text("refused")),
                        ("label", Value::Message(Key::ExampleInputEmailLabel)),
                        ("kind", Value::Text("email")),
                        ("value", Value::Data("ada@examp")),
                        ("error", Value::Message(Key::ExampleInputError)),
                        ("autocomplete", Value::Text("email")),
                    ],
                    children: &[],
                },
                Node::Component {
                    name: "Input",
                    props: &[
                        ("name", Value::Text("reference")),
                        ("label", Value::Message(Key::ExampleInputReferenceLabel)),
                        ("value", Value::Message(Key::ExampleInputReference)),
                        ("readonly", Value::Bool(true)),
                    ],
                    children: &[],
                },
                Node::Component {
                    name: "Input",
                    props: &[
                        ("name", Value::Text("locked")),
                        ("label", Value::Message(Key::ExampleInputPasswordLabel)),
                        ("kind", Value::Text("password")),
                        ("reveal_label", Value::Message(Key::ExampleInputReveal)),
                        ("conceal_label", Value::Message(Key::ExampleInputConceal)),
                        ("disabled", Value::Bool(true)),
                        ("glass", Value::Bool(true)),
                    ],
                    children: &[],
                },
            ],
        }],
    ),
];
