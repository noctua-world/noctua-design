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

/// Every `Dialog` example. 7 of them.
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
    // AN ANSWER THAT CANNOT BE TAKEN BACK: the alert dialog, its mark and its
    // two answers, the focus on the one that changes nothing.
    (
        r"Dialog { alert: true, irreversible_label, confirm_label, decline_label, … }",
        &[Node::Component {
            name: "Dialog",
            props: &[
                ("name", Value::Text("dialog-delete")),
                ("alert", Value::Bool(true)),
                ("trigger_label", Value::Message(Key::ExampleDialogOpen)),
                ("heading", Value::Message(Key::ExampleDialogHeading)),
                ("description", Value::Message(Key::ExampleDialogDescription)),
                (
                    "irreversible_label",
                    Value::Message(Key::ExampleDialogIrreversible),
                ),
                ("confirm_label", Value::Message(Key::ExampleDialogConfirm)),
                ("decline_label", Value::Message(Key::ExampleDialogCancel)),
                ("close_label", Value::Message(Key::ExampleDialogClose)),
            ],
            children: &[Node::Component {
                name: "Paragraph",
                props: &[],
                children: &[Node::Text(Value::Message(Key::ExampleDialogFiles))],
            }],
        }],
    ),
    // THE INTERFACE'S CONFIRMATION, opened because the program asked. `shown`
    // is the caller's, and on this site the caller is the page: a control
    // writes it, and the answer the dialog reports is written under it.
    (
        r"Dialog { alert: true, shown, requester, dismissible: false, … } — shown by the page",
        &[Node::Element {
            tag: "div",
            attributes: &[("class", Value::Text("nd-example-confirm"))],
            children: &[
                Node::Component {
                    name: "Button",
                    props: &[
                        ("label", Value::Message(Key::ExampleDialogAsk)),
                        ("action", Value::Text("dialog-show")),
                        ("variant", Value::Text("outline")),
                        ("tone", Value::Text("neutral")),
                        ("size", Value::Text("small")),
                    ],
                    children: &[],
                },
                Node::Component {
                    name: "Dialog",
                    props: &[
                        ("name", Value::Text("dialog-confirm")),
                        ("alert", Value::Bool(true)),
                        ("shown", Value::Bool(false)),
                        ("dismissible", Value::Bool(false)),
                        ("heading", Value::Message(Key::ExampleDialogRealmHeading)),
                        (
                            "description",
                            Value::Message(Key::ExampleDialogRealmDescription),
                        ),
                        (
                            "requester",
                            Value::Message(Key::ExampleDialogRealmRequester),
                        ),
                        (
                            "irreversible_label",
                            Value::Message(Key::ExampleDialogIrreversible),
                        ),
                        (
                            "confirm_label",
                            Value::Message(Key::ExampleDialogRealmConfirm),
                        ),
                        ("decline_label", Value::Message(Key::ExampleDialogCancel)),
                    ],
                    children: &[Node::Component {
                        name: "Paragraph",
                        props: &[],
                        children: &[Node::Component {
                            name: "Code",
                            props: &[("class", Value::Text("nd-identifier"))],
                            children: &[Node::Text(Value::Data("realm.delete name=home"))],
                        }],
                    }],
                },
                Node::Element {
                    tag: "p",
                    attributes: &[
                        ("class", Value::Text("nd-example-answer")),
                        ("role", Value::Text("status")),
                        ("data-nd-sentence", Value::Message(Key::ExampleDialogAnswer)),
                    ],
                    children: &[],
                },
            ],
        }],
    ),
];
