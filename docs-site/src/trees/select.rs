//! `Select`, as trees.
//!
//! One entry per example in `examples::SELECT`, in the same order. The
//! caption is repeated here so the pairing is pinned to something a
//! reader can check: `every_example_has_a_tree` reads it against
//! the table's own, and a tree that slid a row apart fails there rather
//! than drawing the wrong picture.

use noctua_design_i18n::Key;

use crate::tree::{Node, Value};
use crate::trees::Tree;

// THE OPTIONS ARE THIS SYSTEM'S OWN PALETTES.
//
// Thirty-nine of them, which is the only list on this site long enough to need
// a select at all — and the one place a chip beside an option is the answer
// rather than a decoration. The names are identifiers and carry
// `nd-identifier`, so the pseudo-locale test reads them as the palette names
// they are; everything around them is a sentence and goes through the
// catalogue like anything else.
//
// One fragment per palette option, since a dozen of them differ only in three
// strings. `examples::palette!` is the macro these replace, and the reason it
// exists is the reason these are shared rather than written out: the copies
// cannot drift.
//
// The value IS the palette, so the chip beside the label takes its colour from
// the palette it names and nothing here says a colour.

/// The `ochre-balanced` option.
pub(crate) const OCHRE: &[Node] = &[Node::Component {
    name: "SelectOption",
    props: &[
        ("class", Value::Text("nd-identifier")),
        ("value", Value::Text("ochre-balanced")),
        ("label", Value::Data("Ochre")),
        ("swatch", Value::Text("ochre-balanced")),
    ],
    children: &[],
}];

/// The `amber-vivid` option.
pub(crate) const AMBER: &[Node] = &[Node::Component {
    name: "SelectOption",
    props: &[
        ("class", Value::Text("nd-identifier")),
        ("value", Value::Text("amber-vivid")),
        ("label", Value::Data("Amber")),
        ("swatch", Value::Text("amber-vivid")),
    ],
    children: &[],
}];

/// The `clay-sober` option.
pub(crate) const CLAY: &[Node] = &[Node::Component {
    name: "SelectOption",
    props: &[
        ("class", Value::Text("nd-identifier")),
        ("value", Value::Text("clay-sober")),
        ("label", Value::Data("Clay")),
        ("swatch", Value::Text("clay-sober")),
    ],
    children: &[],
}];

/// The `teal-balanced` option.
pub(crate) const TEAL: &[Node] = &[Node::Component {
    name: "SelectOption",
    props: &[
        ("class", Value::Text("nd-identifier")),
        ("value", Value::Text("teal-balanced")),
        ("label", Value::Data("Teal")),
        ("swatch", Value::Text("teal-balanced")),
    ],
    children: &[],
}];

/// The `azure-vivid` option.
pub(crate) const AZURE: &[Node] = &[Node::Component {
    name: "SelectOption",
    props: &[
        ("class", Value::Text("nd-identifier")),
        ("value", Value::Text("azure-vivid")),
        ("label", Value::Data("Azure")),
        ("swatch", Value::Text("azure-vivid")),
    ],
    children: &[],
}];

/// The `indigo-sober` option.
pub(crate) const INDIGO: &[Node] = &[Node::Component {
    name: "SelectOption",
    props: &[
        ("class", Value::Text("nd-identifier")),
        ("value", Value::Text("indigo-sober")),
        ("label", Value::Data("Indigo")),
        ("swatch", Value::Text("indigo-sober")),
    ],
    children: &[],
}];

/// The `ochre-balanced` option, with the second line its family earns it.
pub(crate) const OCHRE_DETAILED: &[Node] = &[Node::Component {
    name: "SelectOption",
    props: &[
        ("class", Value::Text("nd-identifier")),
        ("value", Value::Text("ochre-balanced")),
        ("label", Value::Data("Ochre")),
        ("swatch", Value::Text("ochre-balanced")),
        ("detail", Value::Message(Key::ExampleSelectDetailBalanced)),
    ],
    children: &[],
}];

/// The `amber-vivid` option, with its second line.
pub(crate) const AMBER_DETAILED: &[Node] = &[Node::Component {
    name: "SelectOption",
    props: &[
        ("class", Value::Text("nd-identifier")),
        ("value", Value::Text("amber-vivid")),
        ("label", Value::Data("Amber")),
        ("swatch", Value::Text("amber-vivid")),
        ("detail", Value::Message(Key::ExampleSelectDetailVivid)),
    ],
    children: &[],
}];

/// The `clay-sober` option, with its second line.
pub(crate) const CLAY_DETAILED: &[Node] = &[Node::Component {
    name: "SelectOption",
    props: &[
        ("class", Value::Text("nd-identifier")),
        ("value", Value::Text("clay-sober")),
        ("label", Value::Data("Clay")),
        ("swatch", Value::Text("clay-sober")),
        ("detail", Value::Message(Key::ExampleSelectDetailSober)),
    ],
    children: &[],
}];

/// The `teal-balanced` option, with its second line.
pub(crate) const TEAL_DETAILED: &[Node] = &[Node::Component {
    name: "SelectOption",
    props: &[
        ("class", Value::Text("nd-identifier")),
        ("value", Value::Text("teal-balanced")),
        ("label", Value::Data("Teal")),
        ("swatch", Value::Text("teal-balanced")),
        ("detail", Value::Message(Key::ExampleSelectDetailBalanced)),
    ],
    children: &[],
}];

/// Every `Select` example. 9 of them.
pub(crate) const EXAMPLES: &[Tree] = &[
    (
        r#"Select { name: "palette", label: "…", hint: "…", placeholder: "…" }"#,
        &[Node::Component {
            name: "Select",
            props: &[
                ("name", Value::Text("palette")),
                ("label", Value::Message(Key::ExampleSelectPaletteLabel)),
                ("hint", Value::Message(Key::ExampleSelectPaletteHint)),
                (
                    "placeholder",
                    Value::Message(Key::ExampleSelectPalettePlaceholder),
                ),
            ],
            children: &[
                Node::Component {
                    name: "SelectGroup",
                    props: &[("label", Value::Message(Key::ExampleSelectWarm))],
                    children: &[
                        Node::Fragment(OCHRE),
                        Node::Fragment(AMBER),
                        Node::Fragment(CLAY),
                    ],
                },
                Node::Component {
                    name: "SelectGroup",
                    props: &[("label", Value::Message(Key::ExampleSelectCool))],
                    children: &[
                        Node::Fragment(TEAL),
                        Node::Fragment(AZURE),
                        Node::Fragment(INDIGO),
                    ],
                },
            ],
        }],
    ),
    (
        r"Select { searchable: true, search_label, empty_label, clearable: true, clear_label, size: SelectSize::Large }",
        &[Node::Component {
            name: "Select",
            props: &[
                ("name", Value::Text("palette-search")),
                ("label", Value::Message(Key::ExampleSelectPaletteLabel)),
                (
                    "placeholder",
                    Value::Message(Key::ExampleSelectPalettePlaceholder),
                ),
                ("size", Value::Text("large")),
                ("searchable", Value::Bool(true)),
                ("clearable", Value::Bool(true)),
                (
                    "search_label",
                    Value::Message(Key::ExampleSelectSearchLabel),
                ),
                ("empty_label", Value::Message(Key::ExampleSelectEmptyLabel)),
                ("clear_label", Value::Message(Key::ExampleSelectClearLabel)),
            ],
            children: &[
                Node::Fragment(OCHRE),
                Node::Fragment(AMBER),
                Node::Fragment(CLAY),
                Node::Fragment(TEAL),
                Node::Fragment(AZURE),
                Node::Fragment(INDIGO),
            ],
        }],
    ),
    (
        r"Select { multiple: true, required: true, selected_label }",
        &[Node::Component {
            name: "Select",
            props: &[
                ("name", Value::Text("palette-compare")),
                ("label", Value::Message(Key::ExampleSelectCompareLabel)),
                ("hint", Value::Message(Key::ExampleSelectCompareHint)),
                (
                    "placeholder",
                    Value::Message(Key::ExampleSelectPalettePlaceholder),
                ),
                ("multiple", Value::Bool(true)),
                ("required", Value::Bool(true)),
                // THE WORD IN FRONT OF A SENTENCE NOBODY HAS WRITTEN YET. This
                // field has no `error`: the sentence comes from the browser
                // when a reader submits without answering, and `Field` draws
                // this in front of whatever lands in the slot.
                ("error_label", Value::Message(Key::ExampleFormsErrorWord)),
                ("searchable", Value::Bool(true)),
                (
                    "search_label",
                    Value::Message(Key::ExampleSelectSearchLabel),
                ),
                ("empty_label", Value::Message(Key::ExampleSelectEmptyLabel)),
                (
                    "selected_label",
                    Value::Message(Key::ExampleSelectSelectedLabel),
                ),
            ],
            children: &[
                Node::Fragment(OCHRE_DETAILED),
                Node::Fragment(AMBER_DETAILED),
                Node::Fragment(CLAY_DETAILED),
                Node::Fragment(TEAL_DETAILED),
            ],
        }],
    ),
    (
        r"Select { creatable: true, create_label }",
        &[Node::Component {
            name: "Select",
            props: &[
                ("name", Value::Text("palette-new")),
                ("label", Value::Message(Key::ExampleSelectPaletteLabel)),
                (
                    "placeholder",
                    Value::Message(Key::ExampleSelectPalettePlaceholder),
                ),
                ("searchable", Value::Bool(true)),
                ("creatable", Value::Bool(true)),
                (
                    "search_label",
                    Value::Message(Key::ExampleSelectSearchLabel),
                ),
                ("empty_label", Value::Message(Key::ExampleSelectEmptyLabel)),
                (
                    "create_label",
                    Value::Message(Key::ExampleSelectCreateLabel),
                ),
            ],
            children: &[Node::Fragment(OCHRE), Node::Fragment(TEAL)],
        }],
    ),
    (
        r"Select { required: true, … } inside a <form>",
        // THE ONLY EXAMPLE ON THIS SITE THAT IS INSIDE A `<form>`, and its
        // absence is what hid two defects for a whole stage: a `required`
        // control that is hidden cannot be reported, so the submit failed in
        // silence, and a reset repainted nothing and left every created option
        // behind. Both are only reachable through a form, and there was none.
        //
        // Press it. The answer arrives in the address bar, which is the claim
        // this component makes about itself made rather than described.
        &[Node::Element {
            tag: "form",
            attributes: &[
                ("class", Value::Text("nd-example-form")),
                ("method", Value::Text("get")),
            ],
            children: &[
                Node::Component {
                    name: "Select",
                    props: &[
                        ("name", Value::Text("palette-form")),
                        ("label", Value::Message(Key::ExampleSelectPaletteLabel)),
                        (
                            "placeholder",
                            Value::Message(Key::ExampleSelectPalettePlaceholder),
                        ),
                        ("required", Value::Bool(true)),
                        ("searchable", Value::Bool(true)),
                        ("creatable", Value::Bool(true)),
                        (
                            "search_label",
                            Value::Message(Key::ExampleSelectSearchLabel),
                        ),
                        ("empty_label", Value::Message(Key::ExampleSelectEmptyLabel)),
                        (
                            "create_label",
                            Value::Message(Key::ExampleSelectCreateLabel),
                        ),
                    ],
                    children: &[
                        Node::Fragment(OCHRE),
                        Node::Fragment(TEAL),
                        Node::Fragment(INDIGO),
                    ],
                },
                Node::Element {
                    tag: "div",
                    attributes: &[("class", Value::Text("nd-example-row"))],
                    children: &[
                        Node::Component {
                            name: "Button",
                            props: &[
                                ("submit", Value::Text("submit")),
                                ("label", Value::Message(Key::ExampleSelectSend)),
                            ],
                            children: &[],
                        },
                        Node::Component {
                            name: "Button",
                            props: &[
                                ("submit", Value::Text("reset")),
                                ("variant", Value::Text("outline")),
                                ("tone", Value::Text("neutral")),
                                ("label", Value::Message(Key::ExampleSelectReset)),
                            ],
                            children: &[],
                        },
                    ],
                },
            ],
        }],
    ),
    (
        r#"Select { error: "…", error_label: "…" }"#,
        &[Node::Component {
            name: "Select",
            props: &[
                ("name", Value::Text("palette-error")),
                ("label", Value::Message(Key::ExampleSelectPaletteLabel)),
                (
                    "placeholder",
                    Value::Message(Key::ExampleSelectPalettePlaceholder),
                ),
                ("error", Value::Message(Key::ExampleSelectError)),
                ("error_label", Value::Message(Key::ExampleFormsErrorWord)),
            ],
            children: &[Node::Fragment(OCHRE), Node::Fragment(TEAL)],
        }],
    ),
    (
        r"Select { loading: true, loading_label, size: SelectSize::Small }",
        &[Node::Component {
            name: "Select",
            props: &[
                ("name", Value::Text("palette-loading")),
                ("label", Value::Message(Key::ExampleSelectPaletteLabel)),
                ("hint", Value::Message(Key::ExampleSelectLoadingHint)),
                (
                    "placeholder",
                    Value::Message(Key::ExampleSelectPalettePlaceholder),
                ),
                ("size", Value::Text("small")),
                ("loading", Value::Bool(true)),
                (
                    "loading_label",
                    Value::Message(Key::ExampleSelectLoadingLabel),
                ),
            ],
            children: &[],
        }],
    ),
    // NOT ALSO DISABLED, which the loading example used to be.
    //
    // A disabled control cannot be opened, so the one sentence the loading
    // state exists to say was in the page and unreachable — a demonstration of
    // a feature that demonstrated nothing. `disabled` gets an example of its
    // own instead, where it is the whole point.
    (
        r"Select { disabled: true }",
        &[Node::Component {
            name: "Select",
            props: &[
                ("name", Value::Text("palette-off")),
                ("label", Value::Message(Key::ExampleSelectPaletteLabel)),
                (
                    "placeholder",
                    Value::Message(Key::ExampleSelectPalettePlaceholder),
                ),
                ("disabled", Value::Bool(true)),
            ],
            children: &[Node::Fragment(OCHRE), Node::Fragment(TEAL)],
        }],
    ),
    (
        r"Select { ai_mark, ai_label, ai_note }",
        &[Node::Component {
            name: "Select",
            props: &[
                ("ai_mark", Value::Message(Key::ExampleAiMark)),
                ("ai_label", Value::Message(Key::ExampleAiName)),
                ("ai_note", Value::Message(Key::ExampleAiNote)),
                ("name", Value::Text("palette-ai")),
                ("label", Value::Message(Key::ExampleSelectPaletteLabel)),
                ("hint", Value::Message(Key::ExampleSelectPaletteHint)),
                (
                    "placeholder",
                    Value::Message(Key::ExampleSelectPalettePlaceholder),
                ),
            ],
            children: &[
                Node::Component {
                    name: "SelectGroup",
                    props: &[("label", Value::Message(Key::ExampleSelectWarm))],
                    children: &[
                        Node::Fragment(OCHRE),
                        Node::Fragment(AMBER),
                        Node::Fragment(CLAY),
                    ],
                },
                Node::Component {
                    name: "SelectGroup",
                    props: &[("label", Value::Message(Key::ExampleSelectCool))],
                    children: &[
                        Node::Fragment(TEAL),
                        Node::Fragment(AZURE),
                        Node::Fragment(INDIGO),
                    ],
                },
            ],
        }],
    ),
];
