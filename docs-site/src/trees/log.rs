//! `Log`, as trees.
//!
//! One entry per example in `examples::LOG`, in the same order. The
//! caption is repeated here so the pairing is pinned to something a
//! reader can check: `every_example_has_a_tree` reads it against
//! the table's own, and a tree that slid a row apart fails there rather
//! than drawing the wrong picture.

use noctua_design_i18n::Key;

use crate::tree::{Each, Node, Value};
use crate::trees::Tree;

/// A daemon starting and running, one line a row: the time, the tone, the
/// level, the source and the message.
///
/// MORE LINES THAN A WINDOW HOLDS, so a log in one has an end to follow and a
/// reader has somewhere to scroll away to.
pub(crate) const DAEMON: &[Node] = &[Node::Each {
    over: Each::Rows(&[
        &[
            Value::Text("09:41:02.118"),
            Value::Text("info"),
            Value::Message(Key::ExampleLogLevelInfo),
            Value::Text("nc-daemon"),
            Value::Message(Key::ExampleLogStarted),
        ],
        &[
            Value::Text("09:41:02.140"),
            Value::Text("info"),
            Value::Message(Key::ExampleLogLevelInfo),
            Value::Text("nc-store"),
            Value::Message(Key::ExampleLogStore),
        ],
        &[
            Value::Text("09:41:02.311"),
            Value::Text("neutral"),
            Value::Message(Key::ExampleLogLevelDebug),
            Value::Text("nc-kernel"),
            Value::Message(Key::ExampleLogCatalog),
        ],
        &[
            Value::Text("09:41:03.005"),
            Value::Text("info"),
            Value::Message(Key::ExampleLogLevelInfo),
            Value::Text("nc-ipc"),
            Value::Message(Key::ExampleLogSocket),
        ],
        &[
            Value::Text("09:41:03.210"),
            Value::Text("neutral"),
            Value::Message(Key::ExampleLogLevelDebug),
            Value::Text("nc-ipc"),
            Value::Message(Key::ExampleLogAttached),
        ],
        &[
            Value::Text("09:41:07.420"),
            Value::Text("warning"),
            Value::Message(Key::ExampleLogLevelWarning),
            Value::Text("nc-ipc"),
            Value::Message(Key::ExampleLogSlow),
        ],
        &[
            Value::Text("09:41:09.877"),
            Value::Text("info"),
            Value::Message(Key::ExampleLogLevelInfo),
            Value::Text("nc-kernel"),
            Value::Message(Key::ExampleLogRan),
        ],
        &[
            Value::Text("09:41:12.002"),
            Value::Text("danger"),
            Value::Message(Key::ExampleLogLevelError),
            Value::Text("nc-plugin"),
            Value::Message(Key::ExampleLogRefused),
        ],
        &[
            Value::Text("09:41:12.003"),
            Value::Text("info"),
            Value::Message(Key::ExampleLogLevelInfo),
            Value::Text("nc-kernel"),
            Value::Message(Key::ExampleLogJournal),
        ],
        &[
            Value::Text("09:41:14.560"),
            Value::Text("neutral"),
            Value::Message(Key::ExampleLogLevelDebug),
            Value::Text("nc-store"),
            Value::Message(Key::ExampleLogSnapshot),
        ],
        &[
            Value::Text("09:41:18.031"),
            Value::Text("info"),
            Value::Message(Key::ExampleLogLevelInfo),
            Value::Text("nc-kernel"),
            Value::Message(Key::ExampleLogRan),
        ],
        &[
            Value::Text("09:41:21.774"),
            Value::Text("warning"),
            Value::Message(Key::ExampleLogLevelWarning),
            Value::Text("nc-store"),
            Value::Message(Key::ExampleLogFull),
        ],
        &[
            Value::Text("09:41:22.109"),
            Value::Text("neutral"),
            Value::Message(Key::ExampleLogLevelDebug),
            Value::Text("nc-ipc"),
            Value::Message(Key::ExampleLogAttached),
        ],
        &[
            Value::Text("09:41:25.000"),
            Value::Text("info"),
            Value::Message(Key::ExampleLogLevelInfo),
            Value::Text("nc-daemon"),
            Value::Message(Key::ExampleLogHeartbeat),
        ],
        &[
            Value::Text("09:41:30.448"),
            Value::Text("danger"),
            Value::Message(Key::ExampleLogLevelError),
            Value::Text("nc-ipc"),
            Value::Message(Key::ExampleLogLost),
        ],
        &[
            Value::Text("09:41:30.512"),
            Value::Text("info"),
            Value::Message(Key::ExampleLogLevelInfo),
            Value::Text("nc-ipc"),
            Value::Message(Key::ExampleLogAttached),
        ],
        &[
            Value::Text("09:41:35.000"),
            Value::Text("info"),
            Value::Message(Key::ExampleLogLevelInfo),
            Value::Text("nc-daemon"),
            Value::Message(Key::ExampleLogHeartbeat),
        ],
        &[
            Value::Text("09:41:39.262"),
            Value::Text("info"),
            Value::Message(Key::ExampleLogLevelInfo),
            Value::Text("nc-kernel"),
            Value::Message(Key::ExampleLogRan),
        ],
        &[
            Value::Text("09:41:44.918"),
            Value::Text("warning"),
            Value::Message(Key::ExampleLogLevelWarning),
            Value::Text("nc-plugin"),
            Value::Message(Key::ExampleLogMemory),
        ],
        &[
            Value::Text("09:41:45.000"),
            Value::Text("info"),
            Value::Message(Key::ExampleLogLevelInfo),
            Value::Text("nc-daemon"),
            Value::Message(Key::ExampleLogHeartbeat),
        ],
    ]),
    body: &[Node::Component {
        name: "LogLine",
        props: &[
            ("time", Value::Bound(0)),
            ("tone", Value::Bound(1)),
            ("level", Value::Bound(2)),
            ("source", Value::Bound(3)),
        ],
        children: &[Node::Text(Value::Bound(4))],
    }],
}];

/// The first three lines of [`DAEMON`], for a log with room for all of them.
const START: &[Node] = &[Node::Each {
    over: Each::Rows(&[
        &[
            Value::Text("09:41:02.118"),
            Value::Text("info"),
            Value::Message(Key::ExampleLogLevelInfo),
            Value::Text("nc-daemon"),
            Value::Message(Key::ExampleLogStarted),
        ],
        &[
            Value::Text("09:41:02.140"),
            Value::Text("info"),
            Value::Message(Key::ExampleLogLevelInfo),
            Value::Text("nc-store"),
            Value::Message(Key::ExampleLogStore),
        ],
        &[
            Value::Text("09:41:02.311"),
            Value::Text("neutral"),
            Value::Message(Key::ExampleLogLevelDebug),
            Value::Text("nc-kernel"),
            Value::Message(Key::ExampleLogCatalog),
        ],
    ]),
    body: &[Node::Component {
        name: "LogLine",
        props: &[
            ("time", Value::Bound(0)),
            ("tone", Value::Bound(1)),
            ("level", Value::Bound(2)),
            ("source", Value::Bound(3)),
        ],
        children: &[Node::Text(Value::Bound(4))],
    }],
}];

/// Every `Log` example. 4 of them.
pub(crate) const EXAMPLES: &[Tree] = &[
    (
        r#"Log { name: "…", label: "…", resume_label: "…", LogLine { … } … }"#,
        &[Node::Element {
            tag: "div",
            attributes: &[("class", Value::Text("nd-example-window"))],
            children: &[Node::Component {
                name: "Log",
                props: &[
                    ("name", Value::Text("example-log-basic")),
                    ("label", Value::Message(Key::ExampleLogLabel)),
                    ("resume_label", Value::Message(Key::ExampleLogResume)),
                ],
                children: DAEMON,
            }],
        }],
    ),
    (
        r"Log { follow: false, … }",
        &[Node::Element {
            tag: "div",
            attributes: &[("class", Value::Text("nd-example-window"))],
            children: &[Node::Component {
                name: "Log",
                props: &[
                    ("name", Value::Text("example-log-record")),
                    ("label", Value::Message(Key::ExampleLogRecord)),
                    ("follow", Value::Bool(false)),
                ],
                children: DAEMON,
            }],
        }],
    ),
    (
        r"Log { … } in a narrow window",
        &[Node::Element {
            tag: "div",
            attributes: &[("class", Value::Text("nd-example-window nd-example-narrow"))],
            children: &[Node::Component {
                name: "Log",
                props: &[
                    ("name", Value::Text("example-log-narrow")),
                    ("label", Value::Message(Key::ExampleLogLabel)),
                    ("resume_label", Value::Message(Key::ExampleLogResume)),
                ],
                children: DAEMON,
            }],
        }],
    ),
    (
        r"Log { … } with room for every line",
        &[Node::Element {
            tag: "div",
            attributes: &[("class", Value::Text("nd-example-window"))],
            children: &[Node::Component {
                name: "Log",
                props: &[
                    ("name", Value::Text("example-log-short")),
                    ("label", Value::Message(Key::ExampleLogLabel)),
                    ("resume_label", Value::Message(Key::ExampleLogResume)),
                ],
                children: START,
            }],
        }],
    ),
];
