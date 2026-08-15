//! `Tooltip`, as trees.
//!
//! One entry per example in `examples::TOOLTIP`, in the same order. The
//! caption is repeated here so the pairing is pinned to something a
//! reader can check: `every_tree_belongs_to_its_example` reads it against
//! the table's own, and a tree that slid a row apart fails there rather
//! than drawing the wrong picture.

use noctua_design_i18n::Key;

use crate::tree::{Node, Value};
use crate::trees::Tree;

/// Every `Tooltip` example. 4 of them.
pub(crate) const EXAMPLES: &[Tree] = &[
    // THE HINT IS NEVER THE NAME. The button says `Copy` on it, and the hint
    // narrows what gets copied.
    (
        r#"Tooltip { name, text: "…", Button { … } }"#,
        &[Node::Component {
            name: "Tooltip",
            props: &[
                ("name", Value::Text("tooltip-copy")),
                ("text", Value::Message(Key::ExampleTooltipCopyHint)),
            ],
            children: &[Node::Component {
                name: "Button",
                props: &[
                    ("variant", Value::Text("outline")),
                    ("tone", Value::Text("neutral")),
                    ("icon", Value::Text("copy")),
                    ("label", Value::Message(Key::ExampleTooltipCopy)),
                ],
                children: &[],
            }],
        }],
    ),
    (
        r"Tooltip { delay: TooltipDelay::Long, … }",
        &[Node::Component {
            name: "Tooltip",
            props: &[
                ("name", Value::Text("tooltip-export")),
                ("delay", Value::Text("long")),
                ("text", Value::Message(Key::ExampleTooltipExportHint)),
            ],
            children: &[Node::Component {
                name: "Button",
                props: &[
                    ("variant", Value::Text("outline")),
                    ("tone", Value::Text("neutral")),
                    ("icon", Value::Text("download")),
                    ("label", Value::Message(Key::ExampleTooltipExport)),
                ],
                children: &[],
            }],
        }],
    ),
    (
        r"Tooltip { placement: TooltipPlacement::End, … }",
        &[Node::Element {
            tag: "div",
            attributes: &[("class", Value::Text("nd-example-end"))],
            children: &[Node::Component {
                name: "Tooltip",
                props: &[
                    ("name", Value::Text("tooltip-refresh")),
                    ("placement", Value::Text("end")),
                    ("text", Value::Message(Key::ExampleTooltipRefreshHint)),
                ],
                children: &[Node::Component {
                    name: "Button",
                    props: &[
                        ("variant", Value::Text("outline")),
                        ("tone", Value::Text("neutral")),
                        ("icon", Value::Text("rotate-ccw")),
                        ("label", Value::Message(Key::ExampleTooltipRefresh)),
                    ],
                    children: &[],
                }],
            }],
        }],
    ),
    // AN ICON-ONLY CONTROL, which is where a tooltip is most tempting and most
    // dangerous: `label` is still the name, and the hint only describes.
    (
        r"Tooltip { delay: TooltipDelay::None, … Button { hide_label: true } }",
        &[Node::Component {
            name: "Tooltip",
            props: &[
                ("name", Value::Text("tooltip-lock")),
                ("delay", Value::Text("none")),
                ("text", Value::Message(Key::ExampleTooltipLockHint)),
            ],
            children: &[Node::Component {
                name: "Button",
                props: &[
                    ("variant", Value::Text("outline")),
                    ("tone", Value::Text("neutral")),
                    ("icon", Value::Text("eye")),
                    ("hide_label", Value::Bool(true)),
                    ("label", Value::Message(Key::ExampleTooltipLock)),
                ],
                children: &[],
            }],
        }],
    ),
];
