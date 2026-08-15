//! `Switch`, as trees.
//!
//! One entry per example in `examples::SWITCH`, in the same order. The
//! caption is repeated here so the pairing is pinned to something a
//! reader can check: `every_tree_belongs_to_its_example` reads it against
//! the table's own, and a tree that slid a row apart fails there rather
//! than drawing the wrong picture.

use noctua_design_i18n::Key;

use crate::tree::{Each, Node, Value};
use crate::trees::Tree;

/// Every `Switch` example. 6 of them.
pub(crate) const EXAMPLES: &[Tree] = &[
    (
        r#"Switch { name: "…", on: "on", off: "off", label: "…" }"#,
        &[Node::Component {
            name: "Switch",
            props: &[
                ("name", Value::Text("example-grain")),
                ("on", Value::Text("on")),
                ("off", Value::Text("off")),
                ("label", Value::Message(Key::ExampleSwitchGrain)),
            ],
            children: &[],
        }],
    ),
    (
        r"Switch { checked: true, … }",
        &[Node::Element {
            tag: "div",
            attributes: &[("class", Value::Text("nd-example-stack"))],
            children: &[
                Node::Component {
                    name: "Switch",
                    props: &[
                        ("name", Value::Text("example-pattern")),
                        ("on", Value::Text("on")),
                        ("off", Value::Text("off")),
                        ("checked", Value::Bool(true)),
                        ("label", Value::Message(Key::ExampleSwitchPattern)),
                    ],
                    children: &[],
                },
                Node::Component {
                    name: "Switch",
                    props: &[
                        ("name", Value::Text("example-pattern-off")),
                        ("on", Value::Text("on")),
                        ("off", Value::Text("off")),
                        ("label", Value::Message(Key::ExampleSwitchPattern)),
                    ],
                    children: &[],
                },
            ],
        }],
    ),
    (
        r#"Switch { describedby: "…", … }"#,
        &[Node::Element {
            tag: "div",
            attributes: &[("class", Value::Text("nd-example-stack"))],
            children: &[
                Node::Component {
                    name: "Switch",
                    props: &[
                        ("name", Value::Text("example-described")),
                        ("on", Value::Text("on")),
                        ("off", Value::Text("off")),
                        ("checked", Value::Bool(true)),
                        ("describedby", Value::Text("example-switch-note")),
                        ("label", Value::Message(Key::SettingsGrainLabel)),
                    ],
                    children: &[],
                },
                Node::Element {
                    tag: "p",
                    attributes: &[
                        ("class", Value::Text("nd-note-line")),
                        ("id", Value::Text("example-switch-note")),
                    ],
                    children: &[Node::Text(Value::Message(Key::SettingsGrainNote))],
                },
            ],
        }],
    ),
    (
        r"Switch { size: SwitchSize::… }",
        &[Node::Element {
            tag: "div",
            attributes: &[("class", Value::Text("nd-example-stack"))],
            children: &[Node::Each {
                over: Each::Values {
                    component: "Switch",
                    prop: "size",
                },
                body: &[Node::Component {
                    name: "Switch",
                    props: &[
                        ("size", Value::Bound(0)),
                        (
                            "name",
                            Value::Join(&[Value::Text("example-size-"), Value::Bound(0)]),
                        ),
                        ("on", Value::Text("on")),
                        ("off", Value::Text("off")),
                        ("checked", Value::Bool(true)),
                        ("label", Value::Message(Key::ExampleSwitchPattern)),
                    ],
                    children: &[],
                }],
            }],
        }],
    ),
    (
        r"Switch { disabled: true, … }",
        &[Node::Component {
            name: "Switch",
            props: &[
                ("name", Value::Text("example-refused")),
                ("on", Value::Text("on")),
                ("off", Value::Text("off")),
                ("disabled", Value::Bool(true)),
                ("label", Value::Message(Key::SettingsGrainLabel)),
            ],
            children: &[],
        }],
    ),
    (
        r"Switch { glass: true, … }",
        &[Node::Component {
            name: "Switch",
            props: &[
                ("name", Value::Text("example-glass-switch")),
                ("on", Value::Text("on")),
                ("off", Value::Text("off")),
                ("glass", Value::Bool(true)),
                ("checked", Value::Bool(true)),
                ("label", Value::Message(Key::ExampleSwitchGrain)),
            ],
            children: &[],
        }],
    ),
];
