//! `Disclosure`, as trees.
//!
//! One entry per example in `examples::DISCLOSURE`, in the same order. The
//! caption is repeated here so the pairing is pinned to something a
//! reader can check: `every_example_has_a_tree` reads it against
//! the table's own, and a tree that slid a row apart fails there rather
//! than drawing the wrong picture.

use noctua_design_i18n::Key;

use crate::tree::{Node, Value};
use crate::trees::Tree;

/// The paragraph six of the seven examples open on.
const BODY: &[Node] = &[Node::Component {
    name: "Paragraph",
    props: &[],
    children: &[Node::Text(Value::Message(Key::ExampleDisclosureBody))],
}];

/// Every `Disclosure` example. 7 of them.
pub(crate) const EXAMPLES: &[Tree] = &[
    (
        r#"Disclosure { summary: "…", … }"#,
        &[Node::Component {
            name: "Disclosure",
            props: &[("summary", Value::Message(Key::ExampleDisclosureSummary))],
            children: &[Node::Fragment(BODY)],
        }],
    ),
    (
        r"Disclosure { open: true, … }",
        &[Node::Component {
            name: "Disclosure",
            props: &[
                ("summary", Value::Message(Key::ExampleDisclosureSummary)),
                ("open", Value::Bool(true)),
            ],
            children: &[Node::Fragment(BODY)],
        }],
    ),
    (
        r"Disclosure { marker: false, … }",
        &[Node::Component {
            name: "Disclosure",
            props: &[
                ("summary", Value::Message(Key::ExampleDisclosureSummary)),
                ("marker", Value::Bool(false)),
            ],
            children: &[Node::Fragment(BODY)],
        }],
    ),
    (
        r#"Disclosure { icon: "settings", … }"#,
        &[Node::Component {
            name: "Disclosure",
            props: &[
                ("summary", Value::Message(Key::SettingsLabel)),
                ("icon", Value::Text("settings")),
            ],
            children: &[Node::Fragment(BODY)],
        }],
    ),
    (
        r#"Disclosure { name: "…", … }"#,
        &[Node::Component {
            name: "Disclosure",
            props: &[
                ("summary", Value::Message(Key::ExampleDisclosureSummary)),
                ("name", Value::Text("example")),
            ],
            children: &[Node::Component {
                name: "List",
                props: &[
                    ("kind", Value::Text("unordered")),
                    ("spacing", Value::Text("tight")),
                ],
                children: &[
                    Node::Component {
                        name: "ListItem",
                        props: &[],
                        children: &[Node::Text(Value::Message(Key::ExampleListOne))],
                    },
                    Node::Component {
                        name: "ListItem",
                        props: &[],
                        children: &[Node::Text(Value::Message(Key::ExampleListTwo))],
                    },
                ],
            }],
        }],
    ),
    (
        r"Disclosure { heading: DisclosureHeading::H3, … }",
        &[Node::Component {
            name: "Disclosure",
            props: &[
                ("summary", Value::Message(Key::ExampleDisclosureSection)),
                ("heading", Value::Text("h3")),
            ],
            children: &[Node::Fragment(BODY)],
        }],
    ),
    (
        r"Disclosure { glass: true, … }",
        &[Node::Component {
            name: "Disclosure",
            props: &[
                ("summary", Value::Message(Key::ExampleDisclosureSummary)),
                ("glass", Value::Bool(true)),
                ("open", Value::Bool(true)),
            ],
            children: &[Node::Fragment(BODY)],
        }],
    ),
];
