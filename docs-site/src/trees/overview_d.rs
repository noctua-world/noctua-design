//! Hero examples, group D, as trees.
//!
//! One entry per page, keyed by the page's slug — the hero is the one
//! example a page leads with, and `examples::OVERVIEWS` keys them the
//! same way. Split four ways only so the migration could be shared out;
//! nothing about a page decides which group it is in.

use noctua_design_i18n::Key;

use crate::tree::{Each, Node, Value};
use crate::trees::Hero;
use crate::trees::lineage::DOE;
use crate::trees::select::{AMBER_DETAILED, CLAY_DETAILED, OCHRE_DETAILED, TEAL_DETAILED};
use crate::trees::table::{AURORA, BRACKEN, CINTRA, INVOICE_ROW};

/// The invoice that closes the hero's table, in [`INVOICE_ROW`]'s columns.
///
/// Not `table::DELGADO`, which is refunded and smaller: the four amounts on
/// this table have to add up to the total its `<tfoot>` states.
const DELGADO_PAID: &[Value] = &[
    Value::Data("INV-0046"),
    Value::Data("Delgado SA"),
    Value::Data("28 Mar"),
    Value::Text("2026-03-28"),
    Value::Message(Key::ExampleTableStatusPaid),
    Value::Text("paid"),
    Value::Data("R$ 2.310,00"),
    Value::Text("2310"),
];

/// The heroes in this group. 15 of them.
pub(crate) const HEROES: &[Hero] = &[
    (
        "blockquote",
        r#"Blockquote { tone: Accent, cite: "…", attribution: "Edsger W. Dijkstra", Paragraph { … } }"#,
        &[Node::Component {
            name: "Blockquote",
            props: &[
                ("tone", Value::Text("accent")),
                ("cite", Value::Text("https://example.invalid/on-simplicity")),
                ("attribution", Value::Data("Edsger W. Dijkstra")),
            ],
            children: &[Node::Component {
                name: "Paragraph",
                props: &[],
                children: &[Node::Text(Value::Message(Key::ExampleQuote))],
            }],
        }],
    ),
    (
        "color-picker",
        r#"ColorPicker { name: "overview-label-colour", label: "…", hint: "…", value: "…", eyedropper: true, formats: "hex rgb oklch" }"#,
        &[Node::Component {
            name: "ColorPicker",
            props: &[
                ("name", Value::Text("overview-label-colour")),
                ("label", Value::Message(Key::ExampleColorPickerLabel)),
                ("hint", Value::Message(Key::ExampleColorPickerHint)),
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
                ("eyedropper", Value::Bool(true)),
                (
                    "eyedropper_label",
                    Value::Message(Key::ExampleColorPickerEyedropper),
                ),
                // THE NOTATIONS ARE FOLDED, AND THE COLOUR IS NOT. Three
                // formats rather than four: an overview shows that the codes
                // are there and what one looks like, and the full list is the
                // example that is about `formats`.
                ("codes_label", Value::Message(Key::ExampleColorPickerCodes)),
                ("formats", Value::Text("hex rgb oklch")),
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
        "fieldset",
        r#"Fieldset { name: "overview-apply", legend: "…", hint: "…" }
Checkbox { name: "overview-apply", value: "study", label: "…", hint: "…" }"#,
        &[Node::Component {
            name: "Fieldset",
            props: &[
                ("name", Value::Text("overview-apply")),
                ("legend", Value::Message(Key::ExampleFormsApplyLegend)),
                ("hint", Value::Message(Key::ExampleFormsApplyHint)),
            ],
            children: &[
                Node::Component {
                    name: "Checkbox",
                    props: &[
                        ("name", Value::Text("overview-apply")),
                        ("value", Value::Text("study")),
                        ("label", Value::Message(Key::ExampleFormsApplyStudy)),
                        ("hint", Value::Message(Key::ExampleFormsApplyStudyHint)),
                    ],
                    children: &[],
                },
                Node::Component {
                    name: "Checkbox",
                    props: &[
                        ("name", Value::Text("overview-apply")),
                        ("value", Value::Text("work")),
                        ("label", Value::Message(Key::ExampleFormsApplyWork)),
                        ("hint", Value::Message(Key::ExampleFormsApplyWorkHint)),
                        ("checked", Value::Bool(true)),
                    ],
                    children: &[],
                },
                Node::Component {
                    name: "Checkbox",
                    props: &[
                        ("name", Value::Text("overview-apply")),
                        ("value", Value::Text("care")),
                        ("label", Value::Message(Key::ExampleFormsApplyCare)),
                    ],
                    children: &[],
                },
            ],
        }],
    ),
    (
        "lineage",
        r#"Lineage { name: "overview-doe", label: "…", graph: "figure gran … bond u1 cara", pick: Both, camera_label, zoom_in_label, zoom_out_label, fit_label }"#,
        &[Node::Component {
            name: "Lineage",
            props: &[
                ("name", Value::Text("overview-doe")),
                ("label", Value::Message(Key::ExampleLineageDoe)),
                // THREE GENERATIONS, WHICH IS THE SMALLEST SCENE THAT IS A
                // FAMILY TREE. Two people and a child show the parts; a
                // grandparents' union carrying two children, one of whom holds
                // a union of their own, shows what the parts are for.
                ("graph", Value::Text(DOE)),
                // The scene explains itself and submits nothing, so a pick is a
                // thing on the screen and nowhere else — no `form`.
                ("pick", Value::Text("both")),
                ("camera_label", Value::Message(Key::ExampleLineageCamera)),
                ("zoom_in_label", Value::Message(Key::ExampleLineageCloser)),
                ("zoom_out_label", Value::Message(Key::ExampleLineageFurther)),
                ("fit_label", Value::Message(Key::ExampleLineageFit)),
            ],
            children: &[],
        }],
    ),
    (
        "meter",
        r#"Meter { name: "overview-storage", label: "…", value: "212", max: "256", unit: "GB", low: "128", high: "204", optimum: "0", value_label: "…" }
Meter { name: "overview-seats", label: "…", value: "16", max: "20", value_label: "…" }"#,
        &[Node::Element {
            tag: "div",
            attributes: &[("class", Value::Text("nd-example-stack"))],
            children: &[
                // A DISK AT 83%, PAST ITS OWN HIGH THRESHOLD. `optimum` sits
                // under `low` because empty is the good end of a disk, so the
                // platform paints this reading as trouble with nobody writing a
                // condition.
                //
                // A DIFFERENT DISK FROM THE ONE THE EXAMPLES BELOW USE, so no
                // two meters on this page are one reading twice.
                Node::Component {
                    name: "Meter",
                    props: &[
                        ("name", Value::Text("overview-storage")),
                        ("label", Value::Message(Key::ExampleMeterStorage)),
                        ("value", Value::Text("212")),
                        ("max", Value::Text("256")),
                        ("unit", Value::Data("GB")),
                        ("low", Value::Text("128")),
                        ("high", Value::Text("204")),
                        ("optimum", Value::Text("0")),
                        ("value_label", Value::Message(Key::ExampleMeterStorageValue)),
                    ],
                    children: &[],
                },
                // And one with no thresholds beside it, which makes no
                // judgement at all: sixteen seats of twenty is a fact.
                Node::Component {
                    name: "Meter",
                    props: &[
                        ("name", Value::Text("overview-seats")),
                        ("label", Value::Message(Key::ExampleMeterSeats)),
                        ("value", Value::Text("16")),
                        ("max", Value::Text("20")),
                        ("value_label", Value::Message(Key::ExampleMeterSeatsValue)),
                    ],
                    children: &[],
                },
            ],
        }],
    ),
    (
        "prose",
        r"Prose { Heading { … } Paragraph { size: Lead, … } Paragraph { … } Blockquote { … } }",
        &[Node::Component {
            name: "Prose",
            props: &[],
            children: &[
                Node::Component {
                    name: "Heading",
                    props: &[("level", Value::Text("3")), ("size", Value::Text("h3"))],
                    children: &[Node::Text(Value::Message(Key::ExampleOverviewProseHeading))],
                },
                Node::Component {
                    name: "Paragraph",
                    props: &[("size", Value::Text("lead"))],
                    children: &[Node::Text(Value::Message(Key::ExampleOverviewProseLead))],
                },
                Node::Component {
                    name: "Paragraph",
                    props: &[],
                    children: &[Node::Text(Value::Message(Key::ExampleOverviewProseBody))],
                },
                Node::Component {
                    name: "Blockquote",
                    props: &[],
                    children: &[Node::Component {
                        name: "Paragraph",
                        props: &[],
                        children: &[Node::Text(Value::Message(Key::ExampleQuoteShort))],
                    }],
                },
            ],
        }],
    ),
    (
        "select",
        r#"Select { name: "overview-palette", label: "…", hint: "…", placeholder: "…", searchable: true, clearable: true }
SelectGroup { label: "…" }  ·  SelectOption { value: "ochre-balanced", label: "Ochre", swatch: "ochre-balanced", detail: "…" }"#,
        &[Node::Component {
            name: "Select",
            props: &[
                ("name", Value::Text("overview-palette")),
                ("label", Value::Message(Key::ExampleSelectPaletteLabel)),
                ("hint", Value::Message(Key::ExampleSelectPaletteHint)),
                (
                    "placeholder",
                    Value::Message(Key::ExampleSelectPalettePlaceholder),
                ),
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
                Node::Component {
                    name: "SelectGroup",
                    props: &[("label", Value::Message(Key::ExampleSelectWarm))],
                    children: &[
                        Node::Fragment(OCHRE_DETAILED),
                        Node::Fragment(AMBER_DETAILED),
                        Node::Fragment(CLAY_DETAILED),
                    ],
                },
                Node::Component {
                    name: "SelectGroup",
                    props: &[("label", Value::Message(Key::ExampleSelectCool))],
                    children: &[
                        Node::Fragment(TEAL_DETAILED),
                        // The cool half has no fragment of its own: the page's
                        // own examples never draw these two with a detail line.
                        Node::Component {
                            name: "SelectOption",
                            props: &[
                                ("class", Value::Text("nd-identifier")),
                                ("value", Value::Text("azure-vivid")),
                                ("label", Value::Data("Azure")),
                                ("swatch", Value::Text("azure-vivid")),
                                ("detail", Value::Message(Key::ExampleSelectDetailVivid)),
                            ],
                            children: &[],
                        },
                        Node::Component {
                            name: "SelectOption",
                            props: &[
                                ("class", Value::Text("nd-identifier")),
                                ("value", Value::Text("indigo-sober")),
                                ("label", Value::Data("Indigo")),
                                ("swatch", Value::Text("indigo-sober")),
                                ("detail", Value::Message(Key::ExampleSelectDetailSober)),
                            ],
                            children: &[],
                        },
                    ],
                },
            ],
        }],
    ),
    (
        "switch",
        r#"Switch { name: "overview-grain", on: "on", off: "off", checked: true, label: "…", describedby: "overview-switch-note" }
Switch { name: "overview-pattern", on: "on", off: "off", label: "…" }"#,
        &[Node::Element {
            tag: "div",
            attributes: &[("class", Value::Text("nd-example-stack"))],
            children: &[
                Node::Component {
                    name: "Switch",
                    props: &[
                        ("name", Value::Text("overview-grain")),
                        ("on", Value::Text("on")),
                        ("off", Value::Text("off")),
                        ("checked", Value::Bool(true)),
                        ("describedby", Value::Text("overview-switch-note")),
                        ("label", Value::Message(Key::SettingsGrainLabel)),
                    ],
                    children: &[],
                },
                // THE SENTENCE IS POINTED AT RATHER THAN MERELY NEAR. A note
                // beside a control is a note a screen reader never reaches on
                // its way through the setting.
                Node::Element {
                    tag: "p",
                    attributes: &[
                        ("class", Value::Text("nd-note-line")),
                        ("id", Value::Text("overview-switch-note")),
                    ],
                    children: &[Node::Text(Value::Message(Key::SettingsGrainNote))],
                },
                Node::Component {
                    name: "Switch",
                    props: &[
                        ("name", Value::Text("overview-pattern")),
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
        "textarea",
        r#"Textarea { name: "overview-note", label: "…", hint: "…", rows: "3", autosize: true, max_rows: "8", maxlength: "400", counter: true, counter_label: "…" }"#,
        // WRITE IN IT. Three lines to begin with, growing to eight and then
        // scrolling, and the count says how much room is left — the box doing
        // all three at once is the component.
        &[Node::Component {
            name: "Textarea",
            props: &[
                ("name", Value::Text("overview-note")),
                ("label", Value::Message(Key::ExampleInputNoteLabel)),
                ("hint", Value::Message(Key::ExampleInputNoteHint)),
                ("rows", Value::Text("3")),
                ("autosize", Value::Bool(true)),
                ("max_rows", Value::Text("8")),
                ("maxlength", Value::Text("400")),
                ("counter", Value::Bool(true)),
                ("counter_label", Value::Message(Key::ExampleInputCounter)),
            ],
            children: &[],
        }],
    ),
    (
        "truncate",
        r#"Truncate { lines: TruncateLines::_2, title: "…", "…" }"#,
        // A COLUMN NARROW ENOUGH THAT THE CLIPPING HAPPENS. This component
        // demonstrates nothing in a column wide enough to hold the whole
        // string.
        &[Node::Element {
            tag: "div",
            attributes: &[("class", Value::Text("nd-example-narrow"))],
            children: &[Node::Component {
                name: "Truncate",
                props: &[
                    ("lines", Value::Text("2")),
                    ("title", Value::Message(Key::ExampleListLong)),
                ],
                children: &[Node::Text(Value::Message(Key::ExampleListLong))],
            }],
        }],
    ),
    (
        "link",
        r#"Link { href: "…", external: true, rel: "noreferrer", "…" }
Link { href: "…", "…" }"#,
        &[Node::Component {
            name: "Paragraph",
            props: &[],
            children: &[
                Node::Text(Value::Join(&[
                    Value::Message(Key::ExampleOverviewLinkLead),
                    Value::Text(" "),
                ])),
                Node::Component {
                    name: "Link",
                    props: &[
                        (
                            "href",
                            Value::Text("https://noctua-world.github.io/noctua-colors/"),
                        ),
                        ("external", Value::Bool(true)),
                        ("rel", Value::Text("noreferrer")),
                    ],
                    children: &[Node::Text(Value::Message(Key::ExampleOverviewLinkColors))],
                },
                Node::Text(Value::Join(&[
                    Value::Data(", "),
                    Value::Message(Key::ExampleOverviewLinkBetween),
                    Value::Text(" "),
                ])),
                Node::Component {
                    name: "Link",
                    props: &[("href", Value::Text("../spacing/"))],
                    children: &[Node::Text(Value::Message(Key::ExampleOverviewLinkSpacing))],
                },
                Node::Text(Value::Data(".")),
            ],
        }],
    ),
    (
        "tables",
        r"Table { name, caption, note, sort_ascending_label, sort_descending_label, sort_cleared_label }
TableColumn { name, width, sortable, sort_as }
TableSection { kind: Head } / TableSection / TableSection { kind: Foot }
TableRow / TableHeaderCell { scope: Row, sort } / TableCell { column, label, align: Numeric, sort_value }
TableRow { emphasis: Strong } / TableCell { aggregate: Sum }",
        &[Node::Component {
            name: "Table",
            props: &[
                ("name", Value::Text("overview-invoices")),
                ("caption", Value::Message(Key::ExampleTableInvoices)),
                ("note", Value::Message(Key::ExampleTableInvoicesNote)),
                (
                    "sort_ascending_label",
                    Value::Message(Key::ExampleTableSortAscending),
                ),
                (
                    "sort_descending_label",
                    Value::Message(Key::ExampleTableSortDescending),
                ),
                (
                    "sort_cleared_label",
                    Value::Message(Key::ExampleTableSortCleared),
                ),
            ],
            children: &[
                // THE COLUMNS ARE DECLARED BEFORE ANY ROW, and everything a
                // column decides is here: how wide it is, whether it sorts, and
                // what two of its values mean to each other.
                //
                // Written out rather than spliced from `table::INVOICE_COLUMNS`:
                // two of these sort and none of that fragment's do.
                Node::Component {
                    name: "TableColumn",
                    props: &[
                        ("name", Value::Text("invoice")),
                        ("width", Value::Text("fit")),
                    ],
                    children: &[],
                },
                Node::Component {
                    name: "TableColumn",
                    props: &[("name", Value::Text("client"))],
                    children: &[],
                },
                Node::Component {
                    name: "TableColumn",
                    props: &[
                        ("name", Value::Text("due")),
                        ("width", Value::Text("fit")),
                        ("sortable", Value::Bool(true)),
                        ("sort_as", Value::Text("date")),
                    ],
                    children: &[],
                },
                Node::Component {
                    name: "TableColumn",
                    props: &[
                        ("name", Value::Text("status")),
                        ("width", Value::Text("fit")),
                    ],
                    children: &[],
                },
                Node::Component {
                    name: "TableColumn",
                    props: &[
                        ("name", Value::Text("amount")),
                        ("width", Value::Text("fit")),
                        ("sortable", Value::Bool(true)),
                        ("sort_as", Value::Text("number")),
                    ],
                    children: &[],
                },
                Node::Component {
                    name: "TableSection",
                    props: &[("kind", Value::Text("head"))],
                    children: &[Node::Component {
                        name: "TableRow",
                        props: &[],
                        children: &[
                            Node::Component {
                                name: "TableHeaderCell",
                                props: &[("column", Value::Text("invoice"))],
                                children: &[Node::Text(Value::Message(
                                    Key::ExampleTableColInvoice,
                                ))],
                            },
                            Node::Component {
                                name: "TableHeaderCell",
                                props: &[("column", Value::Text("client"))],
                                children: &[Node::Text(Value::Message(Key::ExampleTableColClient))],
                            },
                            // The order the server already applied, drawn before
                            // any script runs.
                            Node::Component {
                                name: "TableHeaderCell",
                                props: &[
                                    ("column", Value::Text("due")),
                                    ("sort", Value::Text("ascending")),
                                ],
                                children: &[Node::Text(Value::Message(Key::ExampleTableColDue))],
                            },
                            Node::Component {
                                name: "TableHeaderCell",
                                props: &[("column", Value::Text("status"))],
                                children: &[Node::Text(Value::Message(Key::ExampleTableColStatus))],
                            },
                            Node::Component {
                                name: "TableHeaderCell",
                                props: &[
                                    ("column", Value::Text("amount")),
                                    ("align", Value::Text("numeric")),
                                ],
                                children: &[Node::Text(Value::Message(Key::ExampleTableColAmount))],
                            },
                        ],
                    }],
                },
                Node::Component {
                    name: "TableSection",
                    props: &[],
                    children: &[Node::Each {
                        over: Each::Rows(&[AURORA, BRACKEN, CINTRA, DELGADO_PAID]),
                        body: INVOICE_ROW,
                    }],
                },
                // A `<tfoot>` is what makes a total a total rather than one more
                // row of data, and it is the only band whose cells may carry an
                // `aggregate` — recounted from the rows showing.
                Node::Component {
                    name: "TableSection",
                    props: &[("kind", Value::Text("foot"))],
                    children: &[Node::Component {
                        name: "TableRow",
                        props: &[("emphasis", Value::Text("strong"))],
                        children: &[
                            Node::Component {
                                name: "TableHeaderCell",
                                props: &[
                                    ("scope", Value::Text("row")),
                                    ("column", Value::Text("invoice")),
                                    ("span", Value::Text("4")),
                                ],
                                children: &[Node::Text(Value::Message(Key::ExampleTableTotal))],
                            },
                            Node::Component {
                                name: "TableCell",
                                props: &[
                                    ("column", Value::Text("amount")),
                                    ("class", Value::Text("nd-identifier")),
                                    ("align", Value::Text("numeric")),
                                    ("aggregate", Value::Text("sum")),
                                ],
                                children: &[Node::Text(Value::Data("R$ 9.130,00"))],
                            },
                        ],
                    }],
                },
            ],
        }],
    ),
    (
        "label",
        r"Label { text, for_control, required, required_label }
Label { kind: Name, for_control } / Segmented { labelledby }",
        &[Node::Element {
            tag: "div",
            attributes: &[("class", Value::Text("nd-example-stack"))],
            children: &[
                Node::Component {
                    name: "Label",
                    props: &[
                        ("text", Value::Message(Key::ExampleOverviewLabelEmail)),
                        ("for_control", Value::Text("overview-label-email")),
                        ("required", Value::Bool(true)),
                        ("required_label", Value::Message(Key::ExampleInputRequired)),
                    ],
                    children: &[],
                },
                Node::Element {
                    tag: "input",
                    attributes: &[
                        ("class", Value::Text("nd-input-box")),
                        ("id", Value::Text("overview-label-email")),
                        ("type", Value::Text("email")),
                        ("name", Value::Text("overview-label-email")),
                        ("autocomplete", Value::Text("email")),
                        ("aria-describedby", Value::Text("overview-label-email-hint")),
                    ],
                    children: &[],
                },
                Node::Element {
                    tag: "p",
                    attributes: &[("id", Value::Text("overview-label-email-hint"))],
                    children: &[Node::Component {
                        name: "Text",
                        props: &[
                            ("size", Value::Text("small")),
                            ("tone", Value::Text("muted")),
                        ],
                        children: &[Node::Text(Value::Message(Key::ExampleOverviewLabelHint))],
                    }],
                },
                Node::Component {
                    name: "Divider",
                    props: &[("spacing", Value::Text("tight"))],
                    children: &[],
                },
                // AND THE ONE A `for` CANNOT REACH. A radiogroup is not
                // labelable, so its name is a `<p>` it points at.
                Node::Component {
                    name: "Label",
                    props: &[
                        ("kind", Value::Text("name")),
                        ("text", Value::Message(Key::ExampleLabelDensity)),
                        ("for_control", Value::Text("overview-label-density")),
                    ],
                    children: &[],
                },
                Node::Component {
                    name: "Segmented",
                    props: &[
                        ("name", Value::Text("overview-label-density")),
                        ("labelledby", Value::Text("overview-label-density-label")),
                    ],
                    children: &[Node::Each {
                        over: Each::Rows(&[
                            &[
                                Value::Text("compact"),
                                Value::Message(Key::ExampleLabelCompact),
                                Value::Bool(false),
                            ],
                            &[
                                Value::Text("cosy"),
                                Value::Message(Key::ExampleLabelCosy),
                                Value::Bool(true),
                            ],
                            &[
                                Value::Text("roomy"),
                                Value::Message(Key::ExampleLabelRoomy),
                                Value::Bool(false),
                            ],
                        ]),
                        body: &[Node::Component {
                            name: "SegmentedItem",
                            props: &[
                                ("value", Value::Bound(0)),
                                ("label", Value::Bound(1)),
                                ("selected", Value::Bound(2)),
                            ],
                            children: &[],
                        }],
                    }],
                },
            ],
        }],
    ),
    (
        "popover",
        r"Popover { name, trigger_label, heading, close_label, … Button { … } }",
        &[Node::Component {
            name: "Popover",
            props: &[
                ("name", Value::Text("overview-popover")),
                (
                    "trigger_label",
                    Value::Message(Key::ExamplePopoverShareOpen),
                ),
                ("heading", Value::Message(Key::ExamplePopoverShareHeading)),
                ("close_label", Value::Message(Key::ExamplePopoverClose)),
            ],
            children: &[
                Node::Component {
                    name: "Paragraph",
                    props: &[],
                    children: &[Node::Text(Value::Message(Key::ExamplePopoverShareBody))],
                },
                Node::Component {
                    name: "Button",
                    props: &[
                        ("variant", Value::Text("outline")),
                        ("tone", Value::Text("neutral")),
                        ("icon", Value::Text("copy")),
                        ("label", Value::Message(Key::ExamplePopoverCopy)),
                    ],
                    children: &[],
                },
            ],
        }],
    ),
    (
        "rating",
        r"Rating { readonly: true, value, max, value_label } / Rating { item_label, … }",
        &[Node::Element {
            tag: "div",
            attributes: &[("class", Value::Text("nd-example-stack"))],
            children: &[
                Node::Component {
                    name: "Rating",
                    props: &[
                        ("name", Value::Text("overview-rating-average")),
                        ("label", Value::Message(Key::ExampleOverviewRatingAverage)),
                        ("readonly", Value::Bool(true)),
                        ("value", Value::Text("4.3")),
                        ("max", Value::Text("5")),
                        ("value_label", Value::Message(Key::ExampleRatingValue)),
                    ],
                    children: &[],
                },
                Node::Component {
                    name: "Rating",
                    props: &[
                        ("name", Value::Text("overview-rating-stay")),
                        ("label", Value::Message(Key::ExampleOverviewRatingLabel)),
                        ("max", Value::Text("5")),
                        ("value_label", Value::Message(Key::ExampleRatingValue)),
                        ("item_label", Value::Message(Key::ExampleRatingItem)),
                        ("hint", Value::Message(Key::ExampleRatingHint)),
                    ],
                    children: &[],
                },
            ],
        }],
    ),
];
