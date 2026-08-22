//! `Dialog`, as trees.
//!
//! One entry per example in `examples::DIALOG`, in the same order. The
//! caption is repeated here so the pairing is pinned to something a
//! reader can check: `every_example_has_a_tree` reads it against
//! the table's own, and a tree that slid a row apart fails there rather
//! than drawing the wrong picture.

use noctua_design_i18n::Key;

use crate::tree::{Node, Value};
use crate::trees::Tree;

/// Every `Dialog` example. 5 of them.
pub(crate) const EXAMPLES: &[Tree] = &[
    // THE ORDINARY ONE, AND EVERY NAMING PROP AT ONCE: a visible heading names
    // the surface, a sentence describes it, and the way out has a name.
    (
        r"Dialog { name, trigger_label, heading, description, close_label, … }",
        &[Node::Component {
            name: "Dialog",
            props: &[
                ("name", Value::Text("dialog-remove")),
                ("trigger_label", Value::Message(Key::ExampleDialogOpen)),
                ("heading", Value::Message(Key::ExampleDialogHeading)),
                ("description", Value::Message(Key::ExampleDialogDescription)),
                ("close_label", Value::Message(Key::ExampleDialogClose)),
            ],
            children: &[
                Node::Component {
                    name: "Paragraph",
                    props: &[],
                    children: &[Node::Text(Value::Message(Key::ExampleDialogBody))],
                },
                Node::Element {
                    tag: "div",
                    attributes: &[("class", Value::Text("nd-example-row"))],
                    children: &[
                        Node::Component {
                            name: "Button",
                            props: &[
                                ("variant", Value::Text("outline")),
                                ("tone", Value::Text("neutral")),
                                ("label", Value::Message(Key::ExampleDialogCancel)),
                            ],
                            children: &[],
                        },
                        Node::Component {
                            name: "Button",
                            props: &[
                                ("variant", Value::Text("solid")),
                                ("tone", Value::Text("danger")),
                                ("label", Value::Message(Key::ExampleDialogConfirm)),
                            ],
                            children: &[],
                        },
                    ],
                },
            ],
        }],
    ),
    // BESIDE THE PAGE RATHER THAN OVER IT. `show()` leaves the document
    // reachable, so the reader can keep reading the thing the notes are about.
    (
        r"Dialog { modal: false, size: DialogSize::Small, … }",
        &[Node::Component {
            name: "Dialog",
            props: &[
                ("name", Value::Text("dialog-notes")),
                ("modal", Value::Bool(false)),
                ("size", Value::Text("small")),
                ("trigger_label", Value::Message(Key::ExampleDialogNotesOpen)),
                ("heading", Value::Message(Key::ExampleDialogNotesHeading)),
                ("close_label", Value::Message(Key::ExampleDialogClose)),
            ],
            children: &[Node::Component {
                name: "Paragraph",
                props: &[],
                children: &[Node::Text(Value::Message(Key::ExampleDialogNotesBody))],
            }],
        }],
    ),
    (
        r"Dialog { size: DialogSize::Large, dismissible: false, … }",
        &[Node::Component {
            name: "Dialog",
            props: &[
                ("name", Value::Text("dialog-terms")),
                ("size", Value::Text("large")),
                ("dismissible", Value::Bool(false)),
                ("trigger_label", Value::Message(Key::ExampleDialogTermsOpen)),
                ("heading", Value::Message(Key::ExampleDialogTermsHeading)),
            ],
            children: &[Node::Component {
                name: "Paragraph",
                props: &[],
                children: &[Node::Text(Value::Message(Key::ExampleDialogTermsBody))],
            }],
        }],
    ),
    // NAMED BY `label`, because there is no title to name it from.
    (
        r#"Dialog { label: "…", … }"#,
        &[Node::Component {
            name: "Dialog",
            props: &[
                ("name", Value::Text("dialog-rename")),
                ("label", Value::Message(Key::ExampleDialogRenameLabel)),
                (
                    "trigger_label",
                    Value::Message(Key::ExampleDialogRenameOpen),
                ),
                ("close_label", Value::Message(Key::ExampleDialogClose)),
            ],
            children: &[Node::Component {
                name: "Paragraph",
                props: &[],
                children: &[Node::Text(Value::Message(Key::ExampleDialogRenameBody))],
            }],
        }],
    ),
    (
        r"Dialog { ai_mark, ai_label, ai_note }",
        &[Node::Component {
            name: "Dialog",
            props: &[
                ("ai_mark", Value::Message(Key::ExampleAiMark)),
                ("ai_label", Value::Message(Key::ExampleAiName)),
                ("ai_note", Value::Message(Key::ExampleAiNote)),
                ("name", Value::Text("dialog-remove-ai")),
                ("trigger_label", Value::Message(Key::ExampleDialogOpen)),
                ("heading", Value::Message(Key::ExampleDialogHeading)),
                ("description", Value::Message(Key::ExampleDialogDescription)),
                ("close_label", Value::Message(Key::ExampleDialogClose)),
            ],
            children: &[
                Node::Component {
                    name: "Paragraph",
                    props: &[],
                    children: &[Node::Text(Value::Message(Key::ExampleDialogBody))],
                },
                Node::Element {
                    tag: "div",
                    attributes: &[("class", Value::Text("nd-example-row"))],
                    children: &[
                        Node::Component {
                            name: "Button",
                            props: &[
                                ("variant", Value::Text("outline")),
                                ("tone", Value::Text("neutral")),
                                ("label", Value::Message(Key::ExampleDialogCancel)),
                            ],
                            children: &[],
                        },
                        Node::Component {
                            name: "Button",
                            props: &[
                                ("variant", Value::Text("solid")),
                                ("tone", Value::Text("danger")),
                                ("label", Value::Message(Key::ExampleDialogConfirm)),
                            ],
                            children: &[],
                        },
                    ],
                },
            ],
        }],
    ),
];
