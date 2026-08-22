//! `ErrorSummary`, as trees.
//!
//! One entry per example in `examples::ERROR_SUMMARY`, in the same order. The
//! caption is repeated here so the pairing is pinned to something a
//! reader can check: `every_example_has_a_tree` reads it against
//! the table's own, and a tree that slid a row apart fails there rather
//! than drawing the wrong picture.

use noctua_design_i18n::Key;

use crate::tree::{Node, Value};
use crate::trees::Tree;

/// The submit button three of the four forms end on.
const SEND: &[Node] = &[Node::Component {
    name: "Button",
    props: &[
        ("submit", Value::Text("submit")),
        ("label", Value::Message(Key::ExampleSummarySend)),
    ],
    children: &[],
}];

/// Every `ErrorSummary` example. 4 of them.
pub(crate) const EXAMPLES: &[Tree] = &[
    (
        r"ErrorSummary { name, title, description, focus } with ErrorSummaryItem children",
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
                    name: "ErrorSummary",
                    props: &[
                        ("name", Value::Text("rendered")),
                        ("title", Value::Message(Key::ExampleSummaryTitle)),
                        (
                            "description",
                            Value::Message(Key::ExampleSummaryDescription),
                        ),
                        // Off here and nowhere else: four summaries on one page
                        // all reaching for the focus is a page a reader cannot
                        // read. The next example is the one that takes it.
                        ("focus", Value::Bool(false)),
                    ],
                    children: &[
                        Node::Component {
                            name: "ErrorSummaryItem",
                            props: &[("control", Value::Text("nd-input-summary-name"))],
                            children: &[Node::Text(Value::Message(Key::ExampleSummaryNameError))],
                        },
                        Node::Component {
                            name: "ErrorSummaryItem",
                            props: &[("control", Value::Text("nd-input-summary-email"))],
                            children: &[Node::Text(Value::Message(Key::ExampleSummaryEmailError))],
                        },
                    ],
                },
                Node::Component {
                    name: "Input",
                    props: &[
                        ("name", Value::Text("summary-name")),
                        ("label", Value::Message(Key::ExampleSummaryNameLabel)),
                        ("error", Value::Message(Key::ExampleSummaryNameError)),
                        ("autocomplete", Value::Text("name")),
                    ],
                    children: &[],
                },
                Node::Component {
                    name: "Input",
                    props: &[
                        ("name", Value::Text("summary-email")),
                        ("label", Value::Message(Key::ExampleSummaryEmailLabel)),
                        ("kind", Value::Text("email")),
                        ("error", Value::Message(Key::ExampleSummaryEmailError)),
                        ("autocomplete", Value::Text("email")),
                    ],
                    children: &[],
                },
                Node::Fragment(SEND),
            ],
        }],
    ),
    (
        r"ErrorSummary { collect: true, form, title_prefix } — the browser's own refusal",
        &[Node::Element {
            tag: "form",
            attributes: &[
                ("id", Value::Text("nd-example-collect")),
                ("class", Value::Text("nd-example-form")),
                ("method", Value::Text("get")),
                ("action", Value::Text("#")),
            ],
            children: &[
                Node::Component {
                    name: "ErrorSummary",
                    props: &[
                        ("name", Value::Text("collected")),
                        ("title", Value::Message(Key::ExampleSummaryTitle)),
                        ("collect", Value::Bool(true)),
                        ("form", Value::Text("nd-example-collect")),
                        ("title_prefix", Value::Message(Key::ExampleSummaryPrefix)),
                    ],
                    children: &[],
                },
                Node::Component {
                    name: "Input",
                    props: &[
                        ("name", Value::Text("collect-name")),
                        ("label", Value::Message(Key::ExampleSummaryNameLabel)),
                        ("required", Value::Bool(true)),
                        ("autocomplete", Value::Text("name")),
                    ],
                    children: &[],
                },
                Node::Component {
                    name: "Input",
                    props: &[
                        ("name", Value::Text("collect-email")),
                        ("label", Value::Message(Key::ExampleSummaryEmailLabel)),
                        ("kind", Value::Text("email")),
                        ("required", Value::Bool(true)),
                        ("autocomplete", Value::Text("email")),
                    ],
                    children: &[],
                },
                Node::Fragment(SEND),
            ],
        }],
    ),
    (
        r"An entry reaching a control the script has HIDDEN — focus lands on the visible one",
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
                    name: "ErrorSummary",
                    props: &[
                        ("name", Value::Text("grouped")),
                        ("title", Value::Message(Key::ExampleSummaryTitle)),
                        ("focus", Value::Bool(false)),
                    ],
                    children: &[
                        Node::Component {
                            name: "ErrorSummaryItem",
                            props: &[("control", Value::Text("nd-select-summary-colour"))],
                            children: &[Node::Text(Value::Message(Key::ExampleSummaryChoiceError))],
                        },
                        Node::Component {
                            name: "ErrorSummaryItem",
                            props: &[("control", Value::Text("nd-time-input-summary-time-hour"))],
                            children: &[Node::Text(Value::Message(Key::ExampleSummaryDateError))],
                        },
                    ],
                },
                Node::Component {
                    name: "Select",
                    props: &[
                        ("name", Value::Text("summary-colour")),
                        ("label", Value::Message(Key::ExampleSummaryChoiceLabel)),
                        (
                            "placeholder",
                            Value::Message(Key::ExampleSelectPalettePlaceholder),
                        ),
                        ("error", Value::Message(Key::ExampleSummaryChoiceError)),
                    ],
                    // The options the `Select` page writes, spliced rather than
                    // copied: `examples::palette!` exists so the two cannot
                    // drift, and a fragment is what keeps that here.
                    children: &[
                        Node::Fragment(crate::trees::select::OCHRE),
                        Node::Fragment(crate::trees::select::TEAL),
                    ],
                },
                Node::Component {
                    name: "TimeInput",
                    props: &[
                        ("name", Value::Text("summary-time")),
                        ("legend", Value::Message(Key::ExampleInputTimeLegend)),
                        ("hour_label", Value::Message(Key::ExampleInputHour)),
                        ("minute_label", Value::Message(Key::ExampleInputMinute)),
                        ("error", Value::Message(Key::ExampleSummaryDateError)),
                    ],
                    children: &[],
                },
                Node::Fragment(SEND),
            ],
        }],
    ),
    (
        r"ErrorSummary { focus: false } and no description",
        &[Node::Component {
            name: "ErrorSummary",
            props: &[
                ("name", Value::Text("plain")),
                ("title", Value::Message(Key::ExampleSummaryTitle)),
                ("focus", Value::Bool(false)),
            ],
            children: &[Node::Component {
                name: "ErrorSummaryItem",
                props: &[("control", Value::Text("nd-input-summary-name"))],
                children: &[Node::Text(Value::Message(Key::ExampleSummaryNameError))],
            }],
        }],
    ),
];
