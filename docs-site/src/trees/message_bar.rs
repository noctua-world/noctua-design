//! `MessageBar`, as trees.
//!
//! One entry per example in `examples::MESSAGE_BAR`, in the same order. The
//! caption is repeated here so the pairing is pinned to something a
//! reader can check: `every_tree_belongs_to_its_example` reads it against
//! the table's own, and a tree that slid a row apart fails there rather
//! than drawing the wrong picture.

use noctua_design_i18n::Key;

use crate::tree::{Node, Value};
use crate::trees::Tree;

/// Every `MessageBar` example. 4 of them.
pub(crate) const EXAMPLES: &[Tree] = &[
    // THE DEFAULT, AND THE QUIET ONE. A status waits for a pause instead of
    // cutting a screen reader off, which is right about something that worked.
    (
        r#"MessageBar { tone: "success", icon: IconName::Check, title, … }"#,
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
        r#"MessageBar { alert: true, live: Assertive, tone: "danger", icon: IconName::Alert }"#,
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
        r#"MessageBar { dismissible: true, dismiss_label, tone: "warning", icon: IconName::Warning }"#,
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
];
