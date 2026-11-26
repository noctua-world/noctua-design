//! `LogLine`, as trees.
//!
//! One entry per example in `examples::LOG_LINE`, in the same order. The
//! caption is repeated here so the pairing is pinned to something a
//! reader can check: `every_example_has_a_tree` reads it against
//! the table's own, and a tree that slid a row apart fails there rather
//! than drawing the wrong picture.

use noctua_design_i18n::Key;

use crate::tree::{Each, Node, Value};
use crate::trees::Tree;

/// Every `LogLine` example. 4 of them.
pub(crate) const EXAMPLES: &[Tree] = &[
    (
        r#"LogLine { time: "…", tone: "…", level: "…", source: "…", "…" }"#,
        // ONE LINE A LEVEL. The word says which, and the tone says it again.
        &[Node::Component {
            name: "Log",
            props: &[
                ("name", Value::Text("example-log-line-levels")),
                ("label", Value::Message(Key::ExampleLogLabel)),
                ("follow", Value::Bool(false)),
            ],
            children: &[Node::Each {
                over: Each::Rows(&[
                    &[
                        Value::Text("09:41:02.311"),
                        Value::Text("neutral"),
                        Value::Message(Key::ExampleLogLevelDebug),
                        Value::Message(Key::ExampleLogCatalog),
                    ],
                    &[
                        Value::Text("09:41:03.005"),
                        Value::Text("info"),
                        Value::Message(Key::ExampleLogLevelInfo),
                        Value::Message(Key::ExampleLogSocket),
                    ],
                    &[
                        Value::Text("09:41:07.420"),
                        Value::Text("warning"),
                        Value::Message(Key::ExampleLogLevelWarning),
                        Value::Message(Key::ExampleLogSlow),
                    ],
                    &[
                        Value::Text("09:41:12.002"),
                        Value::Text("danger"),
                        Value::Message(Key::ExampleLogLevelError),
                        Value::Message(Key::ExampleLogRefused),
                    ],
                ]),
                body: &[Node::Component {
                    name: "LogLine",
                    props: &[
                        ("time", Value::Bound(0)),
                        ("tone", Value::Bound(1)),
                        ("level", Value::Bound(2)),
                        ("source", Value::Text("nc-ipc")),
                    ],
                    children: &[Node::Text(Value::Bound(3))],
                }],
            }],
        }],
    ),
    (
        r#"LogLine { level: "…", "…" } / LogLine { "…" }"#,
        // A PART LEFT OUT LEAVES NO SPACE BEHIND, and a level with no tone is
        // drawn in the page's own shade.
        &[Node::Component {
            name: "Log",
            props: &[
                ("name", Value::Text("example-log-line-parts")),
                ("label", Value::Message(Key::ExampleLogLabel)),
                ("follow", Value::Bool(false)),
            ],
            children: &[
                Node::Component {
                    name: "LogLine",
                    props: &[("level", Value::Message(Key::ExampleLogLevelInfo))],
                    children: &[Node::Text(Value::Message(Key::ExampleLogStarted))],
                },
                Node::Component {
                    name: "LogLine",
                    props: &[],
                    children: &[Node::Text(Value::Message(Key::ExampleLogHeartbeat))],
                },
            ],
        }],
    ),
    (
        r"LogLine { … } with a word longer than the line, and a line break",
        // A PATH HAS NO SPACE TO BREAK AT, so it breaks inside itself, and the
        // second line of the message stays its own line.
        &[Node::Element {
            tag: "div",
            attributes: &[("class", Value::Text("nd-example-narrow"))],
            children: &[Node::Component {
                name: "Log",
                props: &[
                    ("name", Value::Text("example-log-line-long")),
                    ("label", Value::Message(Key::ExampleLogLabel)),
                    ("follow", Value::Bool(false)),
                ],
                children: &[Node::Component {
                    name: "LogLine",
                    props: &[
                        ("time", Value::Text("09:41:30.448")),
                        ("tone", Value::Text("danger")),
                        ("level", Value::Message(Key::ExampleLogLevelError)),
                        ("source", Value::Text("nc-store")),
                    ],
                    // A path and a code are names, the same in every language.
                    children: &[Node::Element {
                        tag: "span",
                        attributes: &[("class", Value::Text("nd-identifier"))],
                        children: &[Node::Text(Value::Data(
                            "/var/lib/noctua/development/store/journal-0007.sqlite\nSQLITE_FULL (13)",
                        ))],
                    }],
                }],
            }],
        }],
    ),
    (
        r#"LogLine { MatchText { text: "…", hits: "…" } }"#,
        // A LOG FILTERED FROM THE PROMPT LINE: the message is a `MatchText`,
        // each word of the query marked in its own tone.
        &[Node::Component {
            name: "Log",
            props: &[
                ("name", Value::Text("example-log-line-match")),
                ("label", Value::Message(Key::ExampleLogLabel)),
                ("follow", Value::Bool(false)),
            ],
            children: &[Node::Component {
                name: "LogLine",
                props: &[
                    ("time", Value::Text("09:41:09.877")),
                    ("tone", Value::Text("info")),
                    ("level", Value::Message(Key::ExampleLogLevelInfo)),
                    ("source", Value::Text("nc-kernel")),
                ],
                children: &[Node::Component {
                    name: "MatchText",
                    props: &[
                        ("class", Value::Text("nd-identifier")),
                        ("text", Value::Data("settings.mcp.read")),
                        ("hits", Value::Text("0-2|9-11")),
                    ],
                    children: &[],
                }],
            }],
        }],
    ),
];
