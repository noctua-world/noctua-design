//! `ColorPicker`, as trees.
//!
//! One entry per example in `examples::COLOR_PICKER`, in the same order. The
//! caption is repeated here so the pairing is pinned to something a
//! reader can check: `every_tree_belongs_to_its_example` reads it against
//! the table's own, and a tree that slid a row apart fails there rather
//! than drawing the wrong picture.

use noctua_design_i18n::Key;

use crate::tree::{Node, Value};
use crate::trees::Tree;

// EVERY WORD IN A PANEL IS A PROP, which is the contract's own last paragraph
// made visible: a picker whose panel was written in English inside the
// component would ship that English into every locale a consumer has. The lists
// are written out per example rather than shared, because no two of them carry
// the same set — the one with `entry: false` has no entry labels at all.

/// Every `ColorPicker` example. 5 of them.
pub(crate) const EXAMPLES: &[Tree] = &[
    (
        r"ColorPicker { name, label, value, trigger_label, area_label, hue_label, entry_label, entry_invalid_label, codes_label, copy_label, copied_label, close_label }",
        &[Node::Component {
            name: "ColorPicker",
            // NO `size`, NO `shape`, NO `formats`. All three were set to
            // their own declared default here, so the plainest example on
            // the page carried three props a reader could delete without
            // changing a pixel. Each is shown at a value that differs
            // further down.
            props: &[
                ("name", Value::Text("label-colour")),
                ("label", Value::Message(Key::ExampleColorPickerLabel)),
                ("value", Value::Text("#7c3aed")), // allow-literal: the colour a ColorPicker opens on IS a colour, and a picker whose starting value came from a palette token would follow the theme instead of showing the reader their own
                (
                    "trigger_label",
                    Value::Message(Key::ExampleColorPickerTrigger),
                ),
                ("area_label", Value::Message(Key::ExampleColorPickerArea)),
                ("hue_label", Value::Message(Key::ExampleColorPickerHue)),
                ("entry_label", Value::Message(Key::ExampleColorPickerEntry)),
                (
                    "entry_invalid_label",
                    Value::Message(Key::ExampleColorPickerEntryInvalid),
                ),
                ("codes_label", Value::Message(Key::ExampleColorPickerCodes)),
                ("copy_label", Value::Message(Key::ExampleColorPickerCopy)),
                (
                    "copied_label",
                    Value::Message(Key::ExampleColorPickerCopied),
                ),
                ("close_label", Value::Message(Key::ExampleColorPickerClose)),
            ],
            children: &[],
        }],
    ),
    (
        r"ColorPicker { size: Large, shape: Pill, alpha: true, alpha_label, eyedropper: true, eyedropper_label, recent, recent_label, forget_label }",
        &[Node::Component {
            name: "ColorPicker",
            props: &[
                ("name", Value::Text("brand-colour")),
                ("label", Value::Message(Key::ExampleColorPickerBrandLabel)),
                ("value", Value::Text("#0ea5e9")), // allow-literal: the colour a ColorPicker opens on IS a colour, and a picker whose starting value came from a palette token would follow the theme instead of showing the reader their own
                ("size", Value::Text("large")),
                ("shape", Value::Text("pill")),
                ("alpha", Value::Bool(true)),
                ("alpha_label", Value::Message(Key::ExampleColorPickerAlpha)),
                ("eyedropper", Value::Bool(true)),
                (
                    "eyedropper_label",
                    Value::Message(Key::ExampleColorPickerEyedropper),
                ),
                // A key the CALLER names. Nothing is remembered without one,
                // which is what keeps the decision to write to a reader's
                // browser out of this component's hands.
                ("recent", Value::Text("noctua.docs.colours")),
                (
                    "recent_label",
                    Value::Message(Key::ExampleColorPickerRecent),
                ),
                (
                    "forget_label",
                    Value::Message(Key::ExampleColorPickerForget),
                ),
                (
                    "trigger_label",
                    Value::Message(Key::ExampleColorPickerTrigger),
                ),
                ("area_label", Value::Message(Key::ExampleColorPickerArea)),
                ("hue_label", Value::Message(Key::ExampleColorPickerHue)),
                ("entry_label", Value::Message(Key::ExampleColorPickerEntry)),
                (
                    "entry_invalid_label",
                    Value::Message(Key::ExampleColorPickerEntryInvalid),
                ),
                ("codes_label", Value::Message(Key::ExampleColorPickerCodes)),
                ("copy_label", Value::Message(Key::ExampleColorPickerCopy)),
                (
                    "copied_label",
                    Value::Message(Key::ExampleColorPickerCopied),
                ),
                ("close_label", Value::Message(Key::ExampleColorPickerClose)),
            ],
            children: &[],
        }],
    ),
    (
        r"ColorPicker { multiple: true, max, max_label, add_label, remove_label, added_label, removed_label, status }",
        &[Node::Component {
            name: "ColorPicker",
            props: &[
                ("name", Value::Text("tag-colours")),
                ("label", Value::Message(Key::ExampleColorPickerTagsLabel)),
                ("value", Value::Text("#7c3aed #0ea5e9 #22c55e")), // allow-literal: the colour a ColorPicker opens on IS a colour, and a picker whose starting value came from a palette token would follow the theme instead of showing the reader their own
                ("multiple", Value::Bool(true)),
                ("max", Value::Text("4")),
                ("max_label", Value::Message(Key::ExampleColorPickerMax)),
                ("add_label", Value::Message(Key::ExampleColorPickerAdd)),
                (
                    "remove_label",
                    Value::Message(Key::ExampleColorPickerRemove),
                ),
                ("added_label", Value::Message(Key::ExampleColorPickerAdded)),
                (
                    "removed_label",
                    Value::Message(Key::ExampleColorPickerRemoved),
                ),
                ("status", Value::Message(Key::ExampleColorPickerStatus)),
                (
                    "trigger_label",
                    Value::Message(Key::ExampleColorPickerTrigger),
                ),
                ("area_label", Value::Message(Key::ExampleColorPickerArea)),
                ("hue_label", Value::Message(Key::ExampleColorPickerHue)),
                ("entry_label", Value::Message(Key::ExampleColorPickerEntry)),
                (
                    "entry_invalid_label",
                    Value::Message(Key::ExampleColorPickerEntryInvalid),
                ),
                ("codes_label", Value::Message(Key::ExampleColorPickerCodes)),
                ("copy_label", Value::Message(Key::ExampleColorPickerCopy)),
                (
                    "copied_label",
                    Value::Message(Key::ExampleColorPickerCopied),
                ),
                ("close_label", Value::Message(Key::ExampleColorPickerClose)),
            ],
            children: &[],
        }],
    ),
    (
        r#"ColorPicker { palette: true, palette_label, spectrum_label, formats: "hex oklch token" }"#,
        &[Node::Component {
            name: "ColorPicker",
            props: &[
                ("name", Value::Text("noctua-colour")),
                ("label", Value::Message(Key::ExampleColorPickerLabel)),
                ("value", Value::Text("#55a45b")), // allow-literal: the colour a ColorPicker opens on IS a colour, and a picker whose starting value came from a palette token would follow the theme instead of showing the reader their own
                ("palette", Value::Bool(true)),
                (
                    "palette_label",
                    Value::Message(Key::ExampleColorPickerPalette),
                ),
                (
                    "spectrum_label",
                    Value::Message(Key::ExampleColorPickerSpectrum),
                ),
                // `token` says nothing about a colour the palette does not
                // contain, so it appears on the Noctua tab and nowhere else
                // however it is listed.
                ("formats", Value::Text("hex oklch token")),
                (
                    "trigger_label",
                    Value::Message(Key::ExampleColorPickerTrigger),
                ),
                ("area_label", Value::Message(Key::ExampleColorPickerArea)),
                ("hue_label", Value::Message(Key::ExampleColorPickerHue)),
                ("entry_label", Value::Message(Key::ExampleColorPickerEntry)),
                (
                    "entry_invalid_label",
                    Value::Message(Key::ExampleColorPickerEntryInvalid),
                ),
                ("codes_label", Value::Message(Key::ExampleColorPickerCodes)),
                ("copy_label", Value::Message(Key::ExampleColorPickerCopy)),
                (
                    "copied_label",
                    Value::Message(Key::ExampleColorPickerCopied),
                ),
                ("close_label", Value::Message(Key::ExampleColorPickerClose)),
            ],
            children: &[],
        }],
    ),
    (
        r"ColorPicker { hint, error, required: true, glass: true, entry: false, size: Small, shape: Square } / ColorPicker { disabled: true }",
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
                    name: "ColorPicker",
                    props: &[
                        ("name", Value::Text("page-colour")),
                        ("label", Value::Message(Key::ExampleColorPickerLabel)),
                        ("value", Value::Text("#f59e0b")), // allow-literal: the colour a ColorPicker opens on IS a colour, and a picker whose starting value came from a palette token would follow the theme instead of showing the reader their own
                        ("hint", Value::Message(Key::ExampleColorPickerHint)),
                        ("error", Value::Message(Key::ExampleColorPickerError)),
                        ("required", Value::Bool(true)),
                        ("glass", Value::Bool(true)),
                        // Off, so the panel is the square and the tracks and
                        // nothing else — the shape a product wants when the
                        // colours are a reader's own rather than a brand's.
                        ("entry", Value::Bool(false)),
                        ("size", Value::Text("small")),
                        ("shape", Value::Text("square")),
                        (
                            "trigger_label",
                            Value::Message(Key::ExampleColorPickerTrigger),
                        ),
                        ("area_label", Value::Message(Key::ExampleColorPickerArea)),
                        ("hue_label", Value::Message(Key::ExampleColorPickerHue)),
                        ("codes_label", Value::Message(Key::ExampleColorPickerCodes)),
                        ("copy_label", Value::Message(Key::ExampleColorPickerCopy)),
                        (
                            "copied_label",
                            Value::Message(Key::ExampleColorPickerCopied),
                        ),
                        ("close_label", Value::Message(Key::ExampleColorPickerClose)),
                    ],
                    children: &[],
                },
                Node::Component {
                    name: "ColorPicker",
                    props: &[
                        ("name", Value::Text("refused-colour")),
                        ("label", Value::Message(Key::ExampleColorPickerBrandLabel)),
                        ("value", Value::Text("#928f8e")), // allow-literal: the colour a ColorPicker opens on IS a colour, and a picker whose starting value came from a palette token would follow the theme instead of showing the reader their own
                        ("disabled", Value::Bool(true)),
                        (
                            "trigger_label",
                            Value::Message(Key::ExampleColorPickerTrigger),
                        ),
                        ("area_label", Value::Message(Key::ExampleColorPickerArea)),
                        ("hue_label", Value::Message(Key::ExampleColorPickerHue)),
                        ("entry_label", Value::Message(Key::ExampleColorPickerEntry)),
                        (
                            "entry_invalid_label",
                            Value::Message(Key::ExampleColorPickerEntryInvalid),
                        ),
                        ("codes_label", Value::Message(Key::ExampleColorPickerCodes)),
                        ("copy_label", Value::Message(Key::ExampleColorPickerCopy)),
                        (
                            "copied_label",
                            Value::Message(Key::ExampleColorPickerCopied),
                        ),
                        ("close_label", Value::Message(Key::ExampleColorPickerClose)),
                    ],
                    children: &[],
                },
                Node::Component {
                    name: "Button",
                    props: &[
                        ("submit", Value::Text("submit")),
                        ("label", Value::Message(Key::ExampleColorPickerSend)),
                    ],
                    children: &[],
                },
            ],
        }],
    ),
];
