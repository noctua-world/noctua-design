//! `Popover`, as trees.
//!
//! One entry per example in `examples::POPOVER`, in the same order. The
//! caption is repeated here so the pairing is pinned to something a
//! reader can check: `every_tree_belongs_to_its_example` reads it against
//! the table's own, and a tree that slid a row apart fails there rather
//! than drawing the wrong picture.

use noctua_design_i18n::Key;

use crate::tree::{Node, Value};
use crate::trees::Tree;

/// Every `Popover` example. 4 of them.
pub(crate) const EXAMPLES: &[Tree] = &[
    // CONTROLS IN IT, which is the difference between this and an `Info`.
    (
        r"Popover { name, trigger_label, heading, close_label, … }",
        &[Node::Component {
            name: "Popover",
            props: &[
                ("name", Value::Text("popover-filter")),
                ("trigger_label", Value::Message(Key::ExamplePopoverOpen)),
                ("heading", Value::Message(Key::ExamplePopoverHeading)),
                ("close_label", Value::Message(Key::ExamplePopoverClose)),
            ],
            children: &[
                Node::Component {
                    name: "Checkbox",
                    props: &[
                        ("name", Value::Text("popover-filter-unread")),
                        ("value", Value::Text("unread")),
                        ("label", Value::Message(Key::ExamplePopoverUnread)),
                    ],
                    children: &[],
                },
                Node::Component {
                    name: "Button",
                    props: &[
                        ("variant", Value::Text("solid")),
                        ("tone", Value::Text("accent")),
                        ("label", Value::Message(Key::ExamplePopoverApply)),
                    ],
                    children: &[],
                },
            ],
        }],
    ),
    (
        r"Popover { placement: PopoverPlacement::End, … }",
        &[Node::Element {
            tag: "div",
            attributes: &[("class", Value::Text("nd-example-end"))],
            children: &[Node::Component {
                name: "Popover",
                props: &[
                    ("name", Value::Text("popover-share")),
                    ("placement", Value::Text("end")),
                    (
                        "trigger_label",
                        Value::Message(Key::ExamplePopoverShareOpen),
                    ),
                    ("heading", Value::Message(Key::ExamplePopoverShareHeading)),
                    ("close_label", Value::Message(Key::ExamplePopoverClose)),
                ],
                children: &[
                    Node::Component {
                        name: "Paragraph",
                        props: &[],
                        children: &[Node::Text(Value::Message(Key::ExamplePopoverShareBody))],
                    },
                    Node::Component {
                        name: "Button",
                        props: &[
                            ("variant", Value::Text("outline")),
                            ("tone", Value::Text("neutral")),
                            ("icon", Value::Text("copy")),
                            ("label", Value::Message(Key::ExamplePopoverCopy)),
                        ],
                        children: &[],
                    },
                ],
            }],
        }],
    ),
    (
        r"Popover { dismissible: false, … }",
        &[Node::Component {
            name: "Popover",
            props: &[
                ("name", Value::Text("popover-help")),
                ("dismissible", Value::Bool(false)),
                ("trigger_label", Value::Message(Key::ExamplePopoverHelpOpen)),
                ("heading", Value::Message(Key::ExamplePopoverHelpHeading)),
            ],
            children: &[Node::Component {
                name: "Paragraph",
                props: &[],
                children: &[Node::Text(Value::Message(Key::ExamplePopoverHelpBody))],
            }],
        }],
    ),
    (
        r#"Popover { label: "…", … }"#,
        &[Node::Component {
            name: "Popover",
            props: &[
                ("name", Value::Text("popover-sort")),
                ("label", Value::Message(Key::ExamplePopoverPlainLabel)),
                (
                    "trigger_label",
                    Value::Message(Key::ExamplePopoverPlainOpen),
                ),
                ("close_label", Value::Message(Key::ExamplePopoverClose)),
            ],
            children: &[
                Node::Component {
                    name: "Button",
                    props: &[
                        ("variant", Value::Text("transparent")),
                        ("tone", Value::Text("neutral")),
                        ("label", Value::Message(Key::ExamplePopoverNewest)),
                    ],
                    children: &[],
                },
                Node::Component {
                    name: "Button",
                    props: &[
                        ("variant", Value::Text("transparent")),
                        ("tone", Value::Text("neutral")),
                        ("label", Value::Message(Key::ExamplePopoverOldest)),
                    ],
                    children: &[],
                },
            ],
        }],
    ),
];
