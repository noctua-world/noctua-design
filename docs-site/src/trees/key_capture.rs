//! `KeyCapture`, as trees.
//!
//! One entry per example in `examples::KEY_CAPTURE`, in the same order. The
//! caption is repeated here so the pairing is pinned to something a
//! reader can check: `every_example_has_a_tree` reads it against
//! the table's own, and a tree that slid a row apart fails there rather
//! than drawing the wrong picture.

use noctua_design_i18n::Key;

use crate::tree::{Node, Value};
use crate::trees::Tree;

/// Every `KeyCapture` example. 6 of them.
pub(crate) const EXAMPLES: &[Tree] = &[
    (
        r#"KeyCapture { name: "…", label: "…", value: "Alt+Space", capture_label: "…", empty_label: "…" }"#,
        &[Node::Component {
            name: "KeyCapture",
            props: &[
                ("name", Value::Text("example-key-capture-basic")),
                ("label", Value::Message(Key::ExampleKeyCaptureLeader)),
                ("value", Value::Text("Alt+Space")),
                (
                    "capture_label",
                    Value::Message(Key::ExampleKeyCaptureCapture),
                ),
                ("empty_label", Value::Message(Key::ExampleKeyCaptureEmpty)),
            ],
            children: &[],
        }],
    ),
    (
        r#"KeyCapture { keyboard: KeyCaptureKeyboard::Linux | Windows | Mac, value: "Alt+Super+Space", … }"#,
        // ONE CHORD, THREE KEYBOARDS. The chord written is the same in all
        // three and only the labels on the modifiers change.
        &[Node::Element {
            tag: "div",
            attributes: &[("class", Value::Text("nd-example-stack"))],
            children: &[
                Node::Component {
                    name: "KeyCapture",
                    props: &[
                        ("name", Value::Text("example-key-capture-linux")),
                        ("label", Value::Message(Key::ExampleKeyCaptureLeader)),
                        ("value", Value::Text("Alt+Super+Space")),
                        ("keyboard", Value::Text("linux")),
                        (
                            "capture_label",
                            Value::Message(Key::ExampleKeyCaptureCapture),
                        ),
                        ("empty_label", Value::Message(Key::ExampleKeyCaptureEmpty)),
                    ],
                    children: &[],
                },
                Node::Component {
                    name: "KeyCapture",
                    props: &[
                        ("name", Value::Text("example-key-capture-windows")),
                        ("label", Value::Message(Key::ExampleKeyCaptureLeader)),
                        ("value", Value::Text("Alt+Super+Space")),
                        ("keyboard", Value::Text("windows")),
                        (
                            "capture_label",
                            Value::Message(Key::ExampleKeyCaptureCapture),
                        ),
                        ("empty_label", Value::Message(Key::ExampleKeyCaptureEmpty)),
                    ],
                    children: &[],
                },
                Node::Component {
                    name: "KeyCapture",
                    props: &[
                        ("name", Value::Text("example-key-capture-mac")),
                        ("label", Value::Message(Key::ExampleKeyCaptureLeader)),
                        ("value", Value::Text("Alt+Super+Space")),
                        ("keyboard", Value::Text("mac")),
                        (
                            "capture_label",
                            Value::Message(Key::ExampleKeyCaptureCapture),
                        ),
                        ("empty_label", Value::Message(Key::ExampleKeyCaptureEmpty)),
                    ],
                    children: &[],
                },
            ],
        }],
    ),
    (
        r#"KeyCapture { clear_label: "…", … }"#,
        &[Node::Component {
            name: "KeyCapture",
            props: &[
                ("name", Value::Text("example-key-capture-removable")),
                ("label", Value::Message(Key::ExampleKeyCaptureSettings)),
                ("value", Value::Text("KeyS")),
                (
                    "capture_label",
                    Value::Message(Key::ExampleKeyCaptureCapture),
                ),
                ("empty_label", Value::Message(Key::ExampleKeyCaptureEmpty)),
                ("clear_label", Value::Message(Key::ExampleKeyCaptureClear)),
            ],
            children: &[],
        }],
    ),
    (
        r"KeyCapture { … } with no value",
        &[Node::Component {
            name: "KeyCapture",
            props: &[
                ("name", Value::Text("example-key-capture-unset")),
                ("label", Value::Message(Key::ExampleKeyCaptureHelp)),
                (
                    "capture_label",
                    Value::Message(Key::ExampleKeyCaptureCapture),
                ),
                ("empty_label", Value::Message(Key::ExampleKeyCaptureEmpty)),
            ],
            children: &[],
        }],
    ),
    (
        r#"KeyCapture { refusal: "…", … }"#,
        &[Node::Component {
            name: "KeyCapture",
            props: &[
                ("name", Value::Text("example-key-capture-refused")),
                ("label", Value::Message(Key::ExampleKeyCaptureLeader)),
                ("value", Value::Text("Alt+Space")),
                (
                    "capture_label",
                    Value::Message(Key::ExampleKeyCaptureCapture),
                ),
                ("empty_label", Value::Message(Key::ExampleKeyCaptureEmpty)),
                ("refusal", Value::Message(Key::ExampleKeyCaptureRefusal)),
            ],
            children: &[],
        }],
    ),
    (
        r"KeyCapture { disabled: true, … }",
        &[Node::Component {
            name: "KeyCapture",
            props: &[
                ("name", Value::Text("example-key-capture-locked")),
                ("label", Value::Message(Key::ExampleKeyCaptureLeader)),
                ("value", Value::Text("Alt+Super+Space")),
                (
                    "capture_label",
                    Value::Message(Key::ExampleKeyCaptureCapture),
                ),
                ("empty_label", Value::Message(Key::ExampleKeyCaptureEmpty)),
                ("clear_label", Value::Message(Key::ExampleKeyCaptureClear)),
                ("disabled", Value::Bool(true)),
            ],
            children: &[],
        }],
    ),
];
