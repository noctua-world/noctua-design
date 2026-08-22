//! `FileInput`, as trees.
//!
//! One entry per example in `examples::FILE_INPUT`, in the same order. The
//! caption is repeated here so the pairing is pinned to something a
//! reader can check: `every_example_has_a_tree` reads it against
//! the table's own, and a tree that slid a row apart fails there rather
//! than drawing the wrong picture.

use noctua_design_i18n::Key;

use crate::tree::{Node, Value};
use crate::trees::Tree;

/// Every `FileInput` example. 4 of them.
pub(crate) const EXAMPLES: &[Tree] = &[
    (
        r"FileInput { name, label, browse_label, drop_label, empty_label, status, hint, required, required_label }",
        // NO `drop: true` HERE. It is the default, so it showed nothing.
        // The example that turns it off is the last one on this page.
        &[Node::Element {
            tag: "form",
            attributes: &[
                ("class", Value::Text("nd-example-form")),
                ("method", Value::Text("get")),
                ("action", Value::Text("#")),
                ("novalidate", Value::Bool(true)),
            ],
            children: &[
                Node::Component {
                    name: "FileInput",
                    props: &[
                        ("name", Value::Text("one-attachment")),
                        ("label", Value::Message(Key::ExampleInputFilesLabel)),
                        ("browse_label", Value::Message(Key::ExampleInputBrowse)),
                        ("drop_label", Value::Message(Key::ExampleInputDrop)),
                        ("empty_label", Value::Message(Key::ExampleInputFileEmpty)),
                        ("status", Value::Message(Key::ExampleInputFileStatus)),
                        ("hint", Value::Message(Key::ExampleInputFilesHint)),
                        ("required", Value::Bool(true)),
                        ("required_label", Value::Message(Key::ExampleInputRequired)),
                    ],
                    children: &[],
                },
                Node::Component {
                    name: "Button",
                    props: &[
                        ("submit", Value::Text("submit")),
                        ("label", Value::Message(Key::ExampleInputSend)),
                    ],
                    children: &[],
                },
            ],
        }],
    ),
    (
        r"FileInput { multiple, max, max_label, remove_label, added_label, removed_label }",
        &[Node::Component {
            name: "FileInput",
            props: &[
                ("name", Value::Text("attachments")),
                ("label", Value::Message(Key::ExampleInputFilesLabel)),
                ("browse_label", Value::Message(Key::ExampleInputBrowse)),
                ("drop_label", Value::Message(Key::ExampleInputDrop)),
                ("multiple", Value::Bool(true)),
                ("max", Value::Text("3")),
                ("max_label", Value::Message(Key::ExampleInputFileTooMany)),
                ("remove_label", Value::Message(Key::ExampleInputFileRemove)),
                ("added_label", Value::Message(Key::ExampleInputFileAdded)),
                (
                    "removed_label",
                    Value::Message(Key::ExampleInputFileRemoved),
                ),
                ("empty_label", Value::Message(Key::ExampleInputFileEmpty)),
                ("status", Value::Message(Key::ExampleInputFileStatus)),
            ],
            children: &[],
        }],
    ),
    (
        r"FileInput { accept, accept_label, max_size, max_size_label } — help, never security",
        &[Node::Component {
            name: "FileInput",
            props: &[
                ("name", Value::Text("pictures")),
                ("label", Value::Message(Key::ExampleInputFilesLabel)),
                ("browse_label", Value::Message(Key::ExampleInputBrowse)),
                ("accept", Value::Text(".pdf,image/*")),
                ("accept_label", Value::Message(Key::ExampleInputFileType)),
                ("max_size", Value::Text("2097152")),
                (
                    "max_size_label",
                    Value::Message(Key::ExampleInputFileTooBig),
                ),
                ("multiple", Value::Bool(true)),
                ("remove_label", Value::Message(Key::ExampleInputFileRemove)),
                ("empty_label", Value::Message(Key::ExampleInputFileEmpty)),
                ("hint", Value::Message(Key::ExampleInputFilesHint)),
            ],
            children: &[],
        }],
    ),
    (
        r"FileInput { name, label, browse_label, drop: false, size, glass, error }
FileInput { name, label, browse_label, disabled, size }",
        &[Node::Element {
            tag: "div",
            attributes: &[("class", Value::Text("nd-example-stack"))],
            children: &[
                Node::Component {
                    name: "FileInput",
                    props: &[
                        ("name", Value::Text("one-file")),
                        ("label", Value::Message(Key::ExampleInputFilesLabel)),
                        ("browse_label", Value::Message(Key::ExampleInputBrowse)),
                        ("drop", Value::Bool(false)),
                        ("size", Value::Text("small")),
                        ("glass", Value::Bool(true)),
                        ("error", Value::Message(Key::ExampleInputError)),
                    ],
                    children: &[],
                },
                Node::Component {
                    name: "FileInput",
                    props: &[
                        ("name", Value::Text("refused-files")),
                        ("label", Value::Message(Key::ExampleInputFilesLabel)),
                        ("browse_label", Value::Message(Key::ExampleInputBrowse)),
                        ("disabled", Value::Bool(true)),
                        ("size", Value::Text("large")),
                    ],
                    children: &[],
                },
            ],
        }],
    ),
];
