//! `MessageBar`, as trees.
//!
//! One entry per example in `examples::MESSAGE_BAR`, in the same order. The
//! caption is repeated here so the pairing is pinned to something a
//! reader can check: `every_example_has_a_tree` reads it against
//! the table's own, and a tree that slid a row apart fails there rather
//! than drawing the wrong picture.

use noctua_design_i18n::Key;

use crate::tree::{Node, Value};
use crate::trees::Tree;

/// An instant long past, in milliseconds since the epoch: the deadline a tree
/// with no code in it can know. The page writes the real one (`shell.js`).
const PAST: &str = "1767225600000";

/// Every `MessageBar` example. 9 of them.
pub(crate) const EXAMPLES: &[Tree] = &[
    // THE DEFAULT, AND THE QUIET ONE. A status waits for a pause instead of
    // cutting a screen reader off, which is right about something that worked.
    (
        r#"MessageBar { tone: "success", icon: IconDrawing::CHECK, title, … }"#,
        &[Node::Component {
            name: "MessageBar",
            props: &[
                ("tone", Value::Text("success")),
                ("icon", Value::Text("check")),
                ("title", Value::Message(Key::ExampleMessageBarSavedTitle)),
            ],
            children: &[Node::Text(Value::Message(Key::ExampleMessageBarSaved))],
        }],
    ),
    // THE RUDE ONE, OPTED INTO. `alert` cuts in, and `live` says how loudly,
    // which is the one case worth overruling the role for.
    (
        r#"MessageBar { alert: true, live: Assertive, tone: "danger", icon: IconDrawing::ALERT }"#,
        &[Node::Component {
            name: "MessageBar",
            props: &[
                ("tone", Value::Text("danger")),
                ("icon", Value::Text("alert")),
                ("alert", Value::Bool(true)),
                ("live", Value::Text("assertive")),
                ("title", Value::Message(Key::ExampleMessageBarRefusedTitle)),
            ],
            children: &[Node::Text(Value::Message(Key::ExampleMessageBarRefused))],
        }],
    ),
    // ONE A READER CAN TAKE AWAY. The control ships hidden and the script
    // reveals it, so a page with no script carries no button that closes
    // nothing.
    (
        r#"MessageBar { dismissible: true, dismiss_label, tone: "warning", icon: IconDrawing::WARNING }"#,
        &[Node::Component {
            name: "MessageBar",
            props: &[
                ("tone", Value::Text("warning")),
                ("icon", Value::Text("warning")),
                ("dismissible", Value::Bool(true)),
                (
                    "dismiss_label",
                    Value::Message(Key::ExampleMessageBarDismiss),
                ),
                ("title", Value::Message(Key::ExampleMessageBarExpiringTitle)),
            ],
            children: &[Node::Text(Value::Message(Key::ExampleMessageBarExpiring))],
        }],
    ),
    // NO TONE AT ALL, which is the absence of the attribute rather than a name
    // called `default`. The words do the work on their own.
    (
        r"MessageBar { … }",
        &[Node::Component {
            name: "MessageBar",
            props: &[("icon", Value::Text("info"))],
            children: &[Node::Text(Value::Message(Key::ExampleMessageBarPlain))],
        }],
    ),
    (
        r"MessageBar { ai_mark, ai_label, ai_note }",
        &[Node::Component {
            name: "MessageBar",
            props: &[
                ("ai_mark", Value::Message(Key::ExampleAiMark)),
                ("ai_label", Value::Message(Key::ExampleAiName)),
                ("ai_note", Value::Message(Key::ExampleAiNote)),
                ("tone", Value::Text("success")),
                ("icon", Value::Text("check")),
                ("title", Value::Message(Key::ExampleMessageBarSavedTitle)),
            ],
            children: &[Node::Text(Value::Message(Key::ExampleMessageBarSaved))],
        }],
    ),
    // THE ALERT LINE (`NOCTUA.md §12.17`), last in its window, under the
    // footer. The deadline is the caller's, and on this site the caller is
    // the page: `shell.js` writes one ten seconds ahead when the page loads
    // and when the control in the window is pressed. The tree's own deadline
    // is an instant long past, which is all a tree with no code can know.
    (
        r#"MessageBar { presentation: Line, alert: true, tone: "danger", deadline, countdown_label, action_label, action, … } — the deadline written by the page"#,
        &[Node::Element {
            tag: "div",
            attributes: &[
                (
                    "class",
                    Value::Text("nd-example-window nd-example-countdown"),
                ),
                ("data-nd-seconds", Value::Text("10")),
            ],
            children: &[
                Node::Element {
                    tag: "div",
                    attributes: &[("class", Value::Text("nd-example-work"))],
                    children: &[Node::Component {
                        name: "Button",
                        props: &[
                            ("label", Value::Message(Key::ExampleMessageBarAgain)),
                            ("action", Value::Text("countdown-again")),
                            ("variant", Value::Text("outline")),
                            ("tone", Value::Text("neutral")),
                            ("size", Value::Text("small")),
                            ("icon", Value::Text("rotate-ccw")),
                        ],
                        children: &[],
                    }],
                },
                Node::Component {
                    name: "Footer",
                    props: &[("label", Value::Message(Key::ExampleFooterLabel))],
                    children: &[Node::Component {
                        name: "FooterLine",
                        props: &[("kind", Value::Text("modeline"))],
                        children: &[Node::Element {
                            tag: "span",
                            attributes: &[],
                            children: &[Node::Text(Value::Message(Key::ExampleFooterPage))],
                        }],
                    }],
                },
                Node::Component {
                    name: "MessageBar",
                    props: &[
                        ("presentation", Value::Text("line")),
                        ("alert", Value::Bool(true)),
                        ("tone", Value::Text("danger")),
                        ("icon", Value::Text("alert")),
                        ("deadline", Value::Text(PAST)),
                        (
                            "countdown_label",
                            Value::Message(Key::ExampleMessageBarRestoring),
                        ),
                        (
                            "action_label",
                            Value::Message(Key::ExampleMessageBarRestore),
                        ),
                        ("action", Value::Text("restore")),
                    ],
                    children: &[Node::Text(Value::Message(Key::ExampleMessageBarLost))],
                },
            ],
        }],
    ),
    // STOPPED ON PURPOSE. Nothing counts, and the action starts it again.
    (
        r#"MessageBar { presentation: Line, tone: "danger", action_label, action, … }"#,
        &[Node::Component {
            name: "MessageBar",
            props: &[
                ("presentation", Value::Text("line")),
                ("tone", Value::Text("danger")),
                ("icon", Value::Text("alert")),
                ("action_label", Value::Message(Key::ExampleMessageBarStart)),
                ("action", Value::Text("start")),
            ],
            children: &[Node::Text(Value::Message(Key::ExampleMessageBarStopped))],
        }],
    ),
    // NO TONE, AND A READER MAY TAKE IT AWAY: a line about the window that is
    // not an alarm, on the subtle surface, its control at the end of the row.
    (
        r"MessageBar { presentation: Line, dismissible: true, dismiss_label, … }",
        &[Node::Component {
            name: "MessageBar",
            props: &[
                ("presentation", Value::Text("line")),
                ("icon", Value::Text("wifi-off")),
                ("dismissible", Value::Bool(true)),
                (
                    "dismiss_label",
                    Value::Message(Key::ExampleMessageBarDismiss),
                ),
            ],
            children: &[Node::Text(Value::Message(Key::ExampleMessageBarOffline))],
        }],
    ),
    // A COUNTDOWN IN A BAR, under the message it is about.
    (
        r#"MessageBar { tone: "warning", deadline, countdown_label, action_label, … } — the deadline written by the page"#,
        &[Node::Element {
            tag: "div",
            attributes: &[
                ("class", Value::Text("nd-example-countdown")),
                ("data-nd-seconds", Value::Text("60")),
            ],
            children: &[Node::Component {
                name: "MessageBar",
                props: &[
                    ("tone", Value::Text("warning")),
                    ("icon", Value::Text("warning")),
                    ("title", Value::Message(Key::ExampleMessageBarExpiringTitle)),
                    ("deadline", Value::Text(PAST)),
                    (
                        "countdown_label",
                        Value::Message(Key::ExampleMessageBarEnds),
                    ),
                    ("action_label", Value::Message(Key::ExampleMessageBarStay)),
                    ("action", Value::Text("stay")),
                ],
                children: &[Node::Text(Value::Message(Key::ExampleMessageBarSave))],
            }],
        }],
    ),
];
