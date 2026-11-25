//! `Prompt`, as trees.
//!
//! One entry per example in `examples::PROMPT`, in the same order. The
//! caption is repeated here so the pairing is pinned to something a
//! reader can check: `every_example_has_a_tree` reads it against
//! the table's own, and a tree that slid a row apart fails there rather
//! than drawing the wrong picture.

use noctua_design_i18n::Key;

use crate::tree::{Node, Value};
use crate::trees::Tree;

/// Every `Prompt` example. 6 of them.
pub(crate) const EXAMPLES: &[Tree] = &[
    (
        r#"Prompt { name: "…", label: "…", send_label: "…", action: "…" }"#,
        &[Node::Component {
            name: "Prompt",
            props: &[
                ("name", Value::Text("example-prompt")),
                ("label", Value::Message(Key::ExamplePromptLabel)),
                ("send_label", Value::Message(Key::ExamplePromptSend)),
                ("action", Value::Text("example-prompt-send")),
            ],
            children: &[],
        }],
    ),
    (
        r#"Prompt { value: "…" }"#,
        &[Node::Component {
            name: "Prompt",
            props: &[
                ("class", Value::Text("nd-identifier")),
                ("name", Value::Text("example-prompt-long")),
                ("label", Value::Message(Key::ExamplePromptLabel)),
                (
                    "value",
                    Value::Data(
                        "window.close\nwindow.hide\nwindow.show\npage.close\npage.next\npage.previous\ndrawer.expand\ndrawer.collapse\nsidebar.show\nsidebar.hide\nsettings.open",
                    ),
                ),
            ],
            children: &[],
        }],
    ),
    (
        r#"Prompt { controls: "nd-option-list-…" } beside OptionList { focusable: false }"#,
        &[Node::Element {
            tag: "div",
            attributes: &[(
                "class",
                Value::Text("nd-example-stack nd-example-stack-wide"),
            )],
            children: &[
                Node::Component {
                    name: "Prompt",
                    props: &[
                        ("name", Value::Text("example-prompt-driven")),
                        ("label", Value::Message(Key::ExamplePromptLabel)),
                        ("controls", Value::Text("nd-option-list-prompt-commands")),
                    ],
                    children: &[],
                },
                Node::Component {
                    name: "OptionList",
                    props: &[
                        ("name", Value::Text("prompt-commands")),
                        ("label", Value::Message(Key::ExampleOptionListCommands)),
                        ("focusable", Value::Bool(false)),
                    ],
                    children: &[
                        Node::Component {
                            name: "OptionItem",
                            props: &[
                                ("value", Value::Text("settings")),
                                ("label", Value::Message(Key::ExampleOptionListSettings)),
                            ],
                            children: &[],
                        },
                        Node::Component {
                            name: "OptionItem",
                            props: &[
                                ("value", Value::Text("help")),
                                ("label", Value::Message(Key::ExampleOptionListHelp)),
                            ],
                            children: &[],
                        },
                        Node::Component {
                            name: "OptionItem",
                            props: &[
                                ("value", Value::Text("sidebar")),
                                ("label", Value::Message(Key::ExampleOptionListSidebar)),
                            ],
                            children: &[],
                        },
                    ],
                },
            ],
        }],
    ),
    (
        r#"Prompt { controls: "…", active: "…" } beside OptionItem { id: "…", active: true }"#,
        &[Node::Element {
            tag: "div",
            attributes: &[(
                "class",
                Value::Text("nd-example-stack nd-example-stack-wide"),
            )],
            children: &[
                Node::Component {
                    name: "Prompt",
                    props: &[
                        ("name", Value::Text("example-prompt-active")),
                        ("label", Value::Message(Key::ExamplePromptLabel)),
                        ("controls", Value::Text("nd-option-list-prompt-pages")),
                        ("active", Value::Text("example-prompt-option-help")),
                    ],
                    children: &[],
                },
                Node::Component {
                    name: "OptionList",
                    props: &[
                        ("name", Value::Text("prompt-pages")),
                        ("label", Value::Message(Key::ExampleOptionListCommands)),
                        ("focusable", Value::Bool(false)),
                    ],
                    children: &[
                        Node::Component {
                            name: "OptionItem",
                            props: &[
                                ("value", Value::Text("settings")),
                                ("label", Value::Message(Key::ExampleOptionListSettings)),
                                ("id", Value::Text("example-prompt-option-settings")),
                            ],
                            children: &[],
                        },
                        Node::Component {
                            name: "OptionItem",
                            props: &[
                                ("value", Value::Text("help")),
                                ("label", Value::Message(Key::ExampleOptionListHelp)),
                                ("id", Value::Text("example-prompt-option-help")),
                                ("active", Value::Bool(true)),
                            ],
                            children: &[],
                        },
                    ],
                },
            ],
        }],
    ),
    (
        r#"Prompt { readonly: true, value: "…", send_label: "…" }"#,
        &[Node::Component {
            name: "Prompt",
            props: &[
                ("name", Value::Text("example-prompt-readonly")),
                ("label", Value::Message(Key::ExamplePromptLabel)),
                ("value", Value::Message(Key::ExamplePromptRunning)),
                ("readonly", Value::Bool(true)),
                ("send_label", Value::Message(Key::ExamplePromptSend)),
            ],
            children: &[],
        }],
    ),
    (
        r#"Prompt { disabled: true, send_label: "…" }"#,
        &[Node::Component {
            name: "Prompt",
            props: &[
                ("name", Value::Text("example-prompt-disabled")),
                ("label", Value::Message(Key::ExamplePromptLabel)),
                ("disabled", Value::Bool(true)),
                ("send_label", Value::Message(Key::ExamplePromptSend)),
            ],
            children: &[],
        }],
    ),
];
