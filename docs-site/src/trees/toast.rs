//! `Toast`, as trees.
//!
//! One entry per example in `examples::TOAST`, in the same order. The
//! caption is repeated here so the pairing is pinned to something a
//! reader can check: `every_tree_belongs_to_its_example` reads it against
//! the table's own, and a tree that slid a row apart fails there rather
//! than drawing the wrong picture.

use noctua_design_i18n::Key;

use crate::tree::{Node, Value};
use crate::trees::Tree;

/// Every `Toast` example. 5 of them.
pub(crate) const EXAMPLES: &[Tree] = &[
    // NO TIMER, which is the default: the message stays until it is dismissed.
    (
        r"Toast { name, title, dismiss_label, … }",
        &[Node::Element {
            tag: "div",
            attributes: &[("class", Value::Text("nd-example-toasts"))],
            children: &[Node::Component {
                name: "Toast",
                props: &[
                    ("name", Value::Text("toast-saved")),
                    ("title", Value::Message(Key::ExampleToastSavedTitle)),
                    ("dismiss_label", Value::Message(Key::ExampleToastDismiss)),
                ],
                children: &[Node::Text(Value::Message(Key::ExampleToastSaved))],
            }],
        }],
    ),
    // THE ONE THAT GOES AWAY, AND THE TWO THINGS THAT MAKE IT READABLE.
    //
    // `countdown_label` is the COMPONENT's: a message with a deadline owes its
    // reader the deadline, and `toast.toml` argues it beside the three
    // guarantees the timer already made. The control beside it is the
    // DOCUMENTATION's: raising a message a second time is the page's job, so
    // it is a `Button` with an `action` that `shell.js` answers, and it ships
    // nowhere. Without it this example is an empty box for every reader who
    // arrives more than six seconds late.
    (
        r#"Toast { tone: "success", duration: "6000", countdown_label, … } — with a control that runs it again"#,
        &[Node::Element {
            tag: "div",
            attributes: &[("class", Value::Text("nd-example-toasts"))],
            children: &[
                Node::Component {
                    name: "Button",
                    props: &[
                        ("label", Value::Message(Key::ExampleToastReplay)),
                        ("action", Value::Text("toast-replay")),
                        ("variant", Value::Text("outline")),
                        ("tone", Value::Text("neutral")),
                        ("size", Value::Text("small")),
                        ("icon", Value::Text("rotate-ccw")),
                    ],
                    children: &[],
                },
                Node::Component {
                    name: "Toast",
                    props: &[
                        ("name", Value::Text("toast-sent")),
                        ("tone", Value::Text("success")),
                        ("duration", Value::Text("6000")),
                        (
                            "countdown_label",
                            Value::Message(Key::ExampleToastCountdown),
                        ),
                        ("title", Value::Message(Key::ExampleToastSentTitle)),
                        ("dismiss_label", Value::Message(Key::ExampleToastDismiss)),
                    ],
                    children: &[Node::Text(Value::Message(Key::ExampleToastSent))],
                },
            ],
        }],
    ),
    (
        r#"Toast { tone: "danger", live: Assertive, placement: TopEnd }"#,
        &[Node::Element {
            tag: "div",
            attributes: &[("class", Value::Text("nd-example-toasts"))],
            children: &[Node::Component {
                name: "Toast",
                props: &[
                    ("name", Value::Text("toast-failed")),
                    ("tone", Value::Text("danger")),
                    ("live", Value::Text("assertive")),
                    ("placement", Value::Text("top-end")),
                    ("title", Value::Message(Key::ExampleToastFailedTitle)),
                    ("dismiss_label", Value::Message(Key::ExampleToastDismiss)),
                ],
                children: &[Node::Text(Value::Message(Key::ExampleToastFailed))],
            }],
        }],
    ),
    // A MESSAGE WITH SOMETHING TO DO IN IT, which `children`'s own contract
    // allows in as many words: "it may hold a `Link` to whatever a reader has
    // to do about it". Nothing demonstrated it, and two defects lived in that
    // gap — `toast.js` flattened the body to text on the one path that matters
    // and deleted the link, and its pause counter could never come back to
    // zero once a message held more than one focusable thing.
    (
        r#"Toast { tone: "danger", duration: "6000", … } — with a Link in it"#,
        &[Node::Element {
            tag: "div",
            attributes: &[("class", Value::Text("nd-example-toasts"))],
            children: &[Node::Component {
                name: "Toast",
                props: &[
                    ("name", Value::Text("toast-review")),
                    ("tone", Value::Text("danger")),
                    ("duration", Value::Text("6000")),
                    ("title", Value::Message(Key::ExampleToastReviewTitle)),
                    ("dismiss_label", Value::Message(Key::ExampleToastDismiss)),
                ],
                children: &[
                    Node::Text(Value::Join(&[
                        Value::Message(Key::ExampleToastReview),
                        Value::Text(" "),
                    ])),
                    Node::Component {
                        name: "Link",
                        props: &[("href", Value::Text("#nd-main"))],
                        children: &[Node::Text(Value::Message(Key::ExampleToastReviewLink))],
                    },
                ],
            }],
        }],
    ),
    (
        r"Toast { dismissible: false, placement: ToastPlacement::BottomStart }",
        &[Node::Element {
            tag: "div",
            attributes: &[("class", Value::Text("nd-example-toasts"))],
            children: &[Node::Component {
                name: "Toast",
                props: &[
                    ("name", Value::Text("toast-offline")),
                    ("dismissible", Value::Bool(false)),
                    ("placement", Value::Text("bottom-start")),
                ],
                children: &[Node::Text(Value::Message(Key::ExampleToastOffline))],
            }],
        }],
    ),
];
