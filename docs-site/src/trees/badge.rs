//! `Badge`, as trees.
//!
//! One entry per example in `examples::BADGE`, in the same order. The
//! caption is repeated here so the pairing is pinned to something a
//! reader can check: `every_tree_belongs_to_its_example` reads it against
//! the table's own, and a tree that slid a row apart fails there rather
//! than drawing the wrong picture.

use noctua_design_i18n::Key;

use crate::tree::{Each, Node, Value};
use crate::trees::Tree;

/// The control a badge marks, in the three examples that mark a button.
const INBOX: &[Node] = &[Node::Component {
    name: "Button",
    props: &[
        ("variant", Value::Text("outline")),
        ("tone", Value::Text("neutral")),
        ("label", Value::Message(Key::ExampleBadgeInbox)),
    ],
    children: &[],
}];

/// The same, with the label hidden behind an icon.
const SETTINGS: &[Node] = &[Node::Component {
    name: "Button",
    props: &[
        ("variant", Value::Text("outline")),
        ("tone", Value::Text("neutral")),
        ("hide_label", Value::Bool(true)),
        ("icon", Value::Text("settings")),
        ("label", Value::Message(Key::SettingsLabel)),
    ],
    children: &[],
}];

/// Every `Badge` example. 4 of them.
pub(crate) const EXAMPLES: &[Tree] = &[
    (
        r#"Badge { count: "…", max: "99", label: "…" }"#,
        &[Node::Element {
            tag: "div",
            attributes: &[("class", Value::Text("nd-example-row"))],
            children: &[
                Node::Component {
                    name: "Badge",
                    props: &[
                        ("count", Value::Text("3")),
                        ("label", Value::Message(Key::ExampleBadgeUnread)),
                    ],
                    children: &[Node::Fragment(INBOX)],
                },
                // THE LABEL IS THE COUNT THE BADGE STANDS FOR, NOT THE ONE IT
                // DRAWS. `max` caps the digits at `99+`; it does not cap what
                // happened, and a reader who cannot see the corner is owed the
                // real number.
                Node::Component {
                    name: "Badge",
                    props: &[
                        ("count", Value::Text("128")),
                        ("max", Value::Text("99")),
                        ("label", Value::Message(Key::ExampleBadgeUnread128)),
                    ],
                    children: &[Node::Fragment(INBOX)],
                },
            ],
        }],
    ),
    (
        r#"Badge { dot: true } / Badge { count: "0" } / Badge { count: "0", show_zero: true }"#,
        &[Node::Element {
            tag: "div",
            attributes: &[("class", Value::Text("nd-example-row"))],
            children: &[
                Node::Component {
                    name: "Badge",
                    props: &[
                        ("dot", Value::Bool(true)),
                        ("label", Value::Message(Key::ExampleBadgeChanged)),
                    ],
                    children: &[Node::Fragment(SETTINGS)],
                },
                Node::Component {
                    name: "Badge",
                    props: &[
                        ("count", Value::Text("0")),
                        ("label", Value::Message(Key::ExampleBadgeUnread0)),
                    ],
                    children: &[Node::Fragment(SETTINGS)],
                },
                Node::Component {
                    name: "Badge",
                    props: &[
                        ("count", Value::Text("0")),
                        ("show_zero", Value::Bool(true)),
                        ("label", Value::Message(Key::ExampleBadgeUnread0)),
                    ],
                    children: &[Node::Fragment(SETTINGS)],
                },
            ],
        }],
    ),
    (
        r#"Badge { tone: "…" }"#,
        &[Node::Element {
            tag: "div",
            attributes: &[("class", Value::Text("nd-example-row"))],
            // The last is a family from the colour system's long tail — the
            // whole point of a tone that is a name rather than a list.
            children: &[Node::Each {
                over: Each::Rows(&[
                    &[Value::Text("danger")],
                    &[Value::Text("accent")],
                    &[Value::Text("neutral")],
                    &[Value::Text("overdue")],
                ]),
                body: &[Node::Component {
                    name: "Badge",
                    props: &[
                        ("tone", Value::Bound(0)),
                        ("count", Value::Text("9")),
                        ("label", Value::Message(Key::ExampleBadgeUnread9)),
                    ],
                    children: &[Node::Fragment(INBOX)],
                }],
            }],
        }],
    ),
    (
        r"Badge { placement: BadgePlacement::… } / Badge { overlap: BadgeOverlap::… }",
        &[
            // A ROOMIER ROW, and the reason is the component: a badge draws
            // OUTSIDE the box of what it marks, so four of them at four corners
            // in an ordinary row collide with their neighbours. Nothing can
            // reserve that space — reserving it is what a badge is not.
            Node::Element {
                tag: "div",
                attributes: &[("class", Value::Text("nd-example-row nd-example-row-airy"))],
                children: &[Node::Each {
                    over: Each::Values {
                        component: "Badge",
                        prop: "placement",
                    },
                    body: &[Node::Component {
                        name: "Badge",
                        props: &[
                            ("placement", Value::Bound(0)),
                            ("count", Value::Text("2")),
                            ("tone", Value::Text("accent")),
                            ("label", Value::Message(Key::ExampleBadgeUnread2)),
                        ],
                        children: &[Node::Fragment(INBOX)],
                    }],
                }],
            },
            Node::Element {
                tag: "div",
                attributes: &[("class", Value::Text("nd-example-row"))],
                // The shape the tag takes follows the overlap it stands beside,
                // and the closure picked it with an `if`. A tree has no branch,
                // so the pairing is the table: `rectangle` with a square tag and
                // `circle` with a pill.
                children: &[Node::Each {
                    over: Each::Rows(&[
                        &[Value::Text("rectangle"), Value::Text("square")],
                        &[Value::Text("circle"), Value::Text("pill")],
                    ]),
                    body: &[Node::Component {
                        name: "Badge",
                        props: &[
                            ("overlap", Value::Bound(0)),
                            ("count", Value::Text("2")),
                            ("label", Value::Message(Key::ExampleBadgeUnread2)),
                        ],
                        children: &[Node::Component {
                            name: "Tag",
                            props: &[
                                ("shape", Value::Bound(1)),
                                ("variant", Value::Text("outline")),
                                ("label", Value::Message(Key::ExampleBadgeInbox)),
                            ],
                            children: &[],
                        }],
                    }],
                }],
            },
        ],
    ),
];
