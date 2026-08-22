//! `ErrorSummaryItem`, as trees.
//!
//! One entry per example in `examples::ERROR_SUMMARY_ITEM`, in the same order. The
//! caption is repeated here so the pairing is pinned to something a
//! reader can check: `every_example_has_a_tree` reads it against
//! the table's own, and a tree that slid a row apart fails there rather
//! than drawing the wrong picture.

use noctua_design_i18n::Key;

use crate::tree::{Node, Value};
use crate::trees::Tree;

/// The entry pointing at the name box.
const NAME: &[Node] = &[Node::Component {
    name: "ErrorSummaryItem",
    props: &[("control", Value::Text("nd-input-summary-name"))],
    children: &[Node::Text(Value::Message(Key::ExampleSummaryNameError))],
}];

/// The entry pointing at the email box.
const EMAIL: &[Node] = &[Node::Component {
    name: "ErrorSummaryItem",
    props: &[("control", Value::Text("nd-input-summary-email"))],
    children: &[Node::Text(Value::Message(Key::ExampleSummaryEmailError))],
}];

/// The entry pointing at the first box of the time question.
const HOUR: &[Node] = &[Node::Component {
    name: "ErrorSummaryItem",
    props: &[("control", Value::Text("nd-time-input-summary-time-hour"))],
    children: &[Node::Text(Value::Message(Key::ExampleSummaryDateError))],
}];

/// Every `ErrorSummaryItem` example. 4 of them.
pub(crate) const EXAMPLES: &[Tree] = &[
    (
        r"ErrorSummaryItem { control } — the id of the control, and it must exist",
        &[Node::Component {
            name: "ErrorSummary",
            props: &[
                ("name", Value::Text("item-one")),
                ("title", Value::Message(Key::ExampleSummaryTitle)),
                ("focus", Value::Bool(false)),
            ],
            children: &[Node::Fragment(NAME)],
        }],
    ),
    (
        r"Several, in the order the form asks the questions",
        &[Node::Component {
            name: "ErrorSummary",
            props: &[
                ("name", Value::Text("item-order")),
                ("title", Value::Message(Key::ExampleSummaryTitle)),
                ("focus", Value::Bool(false)),
            ],
            children: &[
                Node::Fragment(NAME),
                Node::Fragment(EMAIL),
                Node::Fragment(HOUR),
            ],
        }],
    ),
    (
        r"ErrorSummaryItem { control } for a question asked as several boxes",
        &[Node::Component {
            name: "ErrorSummary",
            props: &[
                ("name", Value::Text("item-group")),
                ("title", Value::Message(Key::ExampleSummaryTitle)),
                ("focus", Value::Bool(false)),
            ],
            children: &[Node::Fragment(HOUR)],
        }],
    ),
    (
        r"ErrorSummaryItem { children } — the sentence, never the name of the field",
        &[Node::Component {
            name: "ErrorSummary",
            props: &[
                ("name", Value::Text("item-words")),
                ("title", Value::Message(Key::ExampleSummaryTitle)),
                ("focus", Value::Bool(false)),
            ],
            children: &[Node::Fragment(EMAIL)],
        }],
    ),
];
