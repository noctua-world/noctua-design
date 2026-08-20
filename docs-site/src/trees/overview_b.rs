//! Hero examples, group B, as trees.
//!
//! One entry per page, keyed by the page's slug — the hero is the one
//! example a page leads with, and `examples::OVERVIEWS` keys them the
//! same way. Split four ways only so the migration could be shared out;
//! nothing about a page decides which group it is in.

use noctua_design_i18n::Key;

use crate::tree::{Each, Node, Value};
use crate::trees::Hero;

/// The heroes in this group. 15 of them.
pub(crate) const HEROES: &[Hero] = &[
    (
        "avatars",
        r#"Avatar { decorative: true, initials: "AL", src: "…", label: "…" }
AvatarGroup { label: "Reviewers", more: "+5", more_label: "…", more_href: "…" }"#,
        &[Node::Element {
            tag: "div",
            attributes: &[("class", Value::Text("nd-example-stack"))],
            children: &[
                // One person, beside the name that is already on the screen —
                // which is the whole of `decorative`.
                Node::Element {
                    tag: "span",
                    attributes: &[("class", Value::Text("nd-example-byline"))],
                    children: &[
                        // A PORTRAIT, NOT THE SITE'S OWN MARK. The favicon is a
                        // ring around a dot, so it drew a second circle inside
                        // the avatar's own — two rings where a face belongs.
                        Node::Component {
                            name: "Avatar",
                            props: &[
                                ("decorative", Value::Bool(true)),
                                ("initials", Value::Data("AL")),
                                ("src", Value::Asset("assets/portraits/ada.svg")),
                                ("label", Value::Message(Key::ExampleAvatarAna)),
                            ],
                            children: &[],
                        },
                        Node::Component {
                            name: "Text",
                            props: &[],
                            children: &[Node::Text(Value::Message(Key::ExampleAvatarAna))],
                        },
                    ],
                },
                Node::Component {
                    name: "AvatarGroup",
                    props: &[
                        ("label", Value::Message(Key::ExampleAvatarReviewers)),
                        ("more", Value::Data("+5")),
                        ("more_label", Value::Message(Key::ExampleAvatarMore)),
                        ("more_href", Value::Text("../avatars/")),
                    ],
                    children: &[
                        // ONE PERSON, ONE PORTRAIT — Ana is `ada`, Bruno is
                        // `bob`, Carla is `cara`, on this page and on
                        // `/avatars/`. The reviewers used to be one face and
                        // two sets of letters, which is not a picture of a
                        // room with people in it.
                        Node::Element {
                            tag: "li",
                            attributes: &[],
                            children: &[Node::Component {
                                name: "Avatar",
                                props: &[
                                    ("label", Value::Message(Key::ExampleAvatarBruno)),
                                    ("initials", Value::Data("BS")),
                                    ("src", Value::Asset("assets/portraits/bob.svg")),
                                    ("tone", Value::Text("accent")),
                                ],
                                children: &[],
                            }],
                        },
                        Node::Element {
                            tag: "li",
                            attributes: &[],
                            children: &[Node::Component {
                                name: "Avatar",
                                props: &[
                                    ("label", Value::Message(Key::ExampleAvatarCarla)),
                                    ("initials", Value::Data("CM")),
                                    ("src", Value::Asset("assets/portraits/cara.svg")),
                                    ("tone", Value::Text("success")),
                                ],
                                children: &[],
                            }],
                        },
                        Node::Element {
                            tag: "li",
                            attributes: &[],
                            children: &[Node::Component {
                                name: "Avatar",
                                props: &[
                                    ("label", Value::Message(Key::ExampleAvatarPending)),
                                    ("icon", Value::Text("user")),
                                ],
                                children: &[],
                            }],
                        },
                    ],
                },
            ],
        }],
    ),
    (
        "checkbox",
        r#"Fieldset { legend: "…", hint: "…" }
Checkbox { name: "apply", value: "work", label: "…", hint: "…", checked: true }
Checkbox { value: "none", divider: "or", exclusive: true }"#,
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
                Node::Component {
                    name: "Checkbox",
                    props: &[
                        ("name", Value::Text("overview-apply")),
                        ("value", Value::Text("none")),
                        ("label", Value::Message(Key::ExampleFormsApplyNone)),
                        ("divider", Value::Message(Key::ExampleFormsOr)),
                        ("exclusive", Value::Bool(true)),
                    ],
                    children: &[],
                },
            ],
        }],
    ),
    (
        "error-summary",
        r#"ErrorSummary { name: "…", title: "There is a problem", description: "…" }
ErrorSummaryItem { control: "nd-input-overview-name", "Enter your full name" }
ErrorSummaryItem { control: "nd-overview-terms-yes", "…" }"#,
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
                    name: "ErrorSummary",
                    props: &[
                        ("name", Value::Text("overview")),
                        ("title", Value::Message(Key::ExampleSummaryTitle)),
                        (
                            "description",
                            Value::Message(Key::ExampleSummaryDescription),
                        ),
                        // Off, and only because this is a page of examples. A
                        // summary that is in the markup takes the focus as the
                        // page paints — which is the whole point of it on a
                        // form, and unreadable on a page of six other things.
                        ("focus", Value::Bool(false)),
                    ],
                    children: &[
                        Node::Component {
                            name: "ErrorSummaryItem",
                            props: &[("control", Value::Text("nd-input-overview-name"))],
                            children: &[Node::Text(Value::Message(Key::ExampleSummaryNameError))],
                        },
                        Node::Component {
                            name: "ErrorSummaryItem",
                            props: &[("control", Value::Text("nd-input-overview-email"))],
                            children: &[Node::Text(Value::Message(Key::ExampleSummaryEmailError))],
                        },
                        // ONE ENTRY FOR ONE QUESTION, whatever it is asked with:
                        // it points at the first box of the group, which is
                        // where a reader fixing it starts.
                        Node::Component {
                            name: "ErrorSummaryItem",
                            props: &[("control", Value::Text("nd-overview-terms-yes"))],
                            children: &[Node::Text(Value::Message(Key::ExampleFormsTermsError))],
                        },
                    ],
                },
                Node::Component {
                    name: "Input",
                    props: &[
                        ("name", Value::Text("overview-name")),
                        ("label", Value::Message(Key::ExampleSummaryNameLabel)),
                        ("error", Value::Message(Key::ExampleSummaryNameError)),
                        ("autocomplete", Value::Text("name")),
                    ],
                    children: &[],
                },
                Node::Component {
                    name: "Input",
                    props: &[
                        ("name", Value::Text("overview-email")),
                        ("label", Value::Message(Key::ExampleSummaryEmailLabel)),
                        ("kind", Value::Text("email")),
                        ("error", Value::Message(Key::ExampleSummaryEmailError)),
                        ("autocomplete", Value::Text("email")),
                    ],
                    children: &[],
                },
                Node::Component {
                    name: "Fieldset",
                    props: &[
                        ("name", Value::Text("overview-terms")),
                        ("legend", Value::Message(Key::ExampleFormsTermsLegend)),
                        ("error", Value::Message(Key::ExampleFormsTermsError)),
                        ("error_label", Value::Message(Key::ExampleFormsErrorWord)),
                    ],
                    children: &[Node::Component {
                        name: "Checkbox",
                        props: &[
                            ("name", Value::Text("overview-terms")),
                            ("value", Value::Text("yes")),
                            ("label", Value::Message(Key::ExampleFormsTermsAgree)),
                            ("invalid", Value::Bool(true)),
                        ],
                        children: &[],
                    }],
                },
                Node::Component {
                    name: "Button",
                    props: &[
                        ("submit", Value::Text("submit")),
                        ("label", Value::Message(Key::ExampleSummarySend)),
                    ],
                    children: &[],
                },
            ],
        }],
    ),
    (
        "info",
        r#"Info { name: "…", label: "What counts as a token", heading: "…", close_label: "…" }"#,
        &[Node::Component {
            name: "Text",
            props: &[],
            children: &[
                Node::Text(Value::Message(Key::ExampleInfoUsage)),
                Node::Component {
                    name: "Info",
                    props: &[
                        ("name", Value::Text("overview-usage")),
                        ("label", Value::Message(Key::ExampleInfoUsageLabel)),
                        ("heading", Value::Message(Key::ExampleInfoUsageHeading)),
                        ("close_label", Value::Message(Key::ExampleInfoClose)),
                    ],
                    children: &[
                        // Two sentences, so two runs of text. One `Join` over
                        // both would build a sentence from two catalogue
                        // lookups, and `a_join_carries_at_most_one_message`
                        // refuses exactly that.
                        Node::Text(Value::Message(Key::ExampleInfoUsageNote)),
                        Node::Text(Value::Join(&[
                            Value::Text(" "),
                            Value::Message(Key::ExampleLinkLead),
                            Value::Text(" "),
                        ])),
                        Node::Component {
                            name: "Link",
                            props: &[("href", Value::Text("#"))],
                            children: &[Node::Text(Value::Message(Key::ExampleLinkText))],
                        },
                        Node::Text(Value::Data(".")),
                    ],
                },
            ],
        }],
    ),
    // `list-item` had a hero here, and `/list/`'s own already draws the
    // description list it showed. `ListItem` is documented on `/list/` now,
    // and a page has one hero.
    (
        "paragraph",
        r"Paragraph { size: ParagraphSize::Lead, … }
Paragraph { … }
Paragraph { size: ParagraphSize::Small, tone: ParagraphTone::Muted, … }",
        &[Node::Element {
            tag: "div",
            attributes: &[],
            children: &[
                Node::Component {
                    name: "Paragraph",
                    props: &[("size", Value::Text("lead"))],
                    children: &[Node::Text(Value::Message(
                        Key::ExampleOverviewParagraphLead,
                    ))],
                },
                Node::Component {
                    name: "Paragraph",
                    props: &[],
                    children: &[Node::Text(Value::Message(
                        Key::ExampleOverviewParagraphBody,
                    ))],
                },
                Node::Component {
                    name: "Paragraph",
                    props: &[
                        ("size", Value::Text("small")),
                        ("tone", Value::Text("muted")),
                    ],
                    children: &[Node::Text(Value::Message(
                        Key::ExampleOverviewParagraphNote,
                    ))],
                },
            ],
        }],
    ),
    (
        "segmented",
        r#"Segmented { name: "…", label: "Mode", fill: SegmentedFill::Equal }
SegmentedItem { value: "light", icon: "sun", label: "Light" }
SegmentedItem { value: "system", icon: "monitor", label: "…", selected: true }"#,
        &[Node::Component {
            name: "Segmented",
            props: &[
                ("name", Value::Text("overview-mode")),
                ("label", Value::Message(Key::ThemeModeLabel)),
                ("fill", Value::Text("equal")),
            ],
            children: &[Node::Each {
                over: Each::Rows(&[
                    &[
                        Value::Text("light"),
                        Value::Text("sun"),
                        Value::Message(Key::ThemeModeLight),
                        Value::Bool(false),
                    ],
                    &[
                        Value::Text("dark"),
                        Value::Text("moon"),
                        Value::Message(Key::ThemeModeDark),
                        Value::Bool(false),
                    ],
                    &[
                        Value::Text("system"),
                        Value::Text("monitor"),
                        Value::Message(Key::ThemeModeSystem),
                        Value::Bool(true),
                    ],
                ]),
                body: &[Node::Component {
                    name: "SegmentedItem",
                    props: &[
                        ("value", Value::Bound(0)),
                        ("icon", Value::Bound(1)),
                        ("label", Value::Bound(2)),
                        ("selected", Value::Bound(3)),
                    ],
                    children: &[],
                }],
            }],
        }],
    ),
    (
        "slider",
        r#"Slider { name: "…", label: "Volume", value: "35", min: "0", max: "100", step: "5" }
Slider { marks: "0 50 100", mark_labels: "Silent|Loud", output_label: "{value}%" }"#,
        &[Node::Component {
            name: "Slider",
            props: &[
                ("name", Value::Text("overview-volume")),
                ("label", Value::Message(Key::ExampleInputVolumeLabel)),
                ("value", Value::Text("35")),
                ("min", Value::Text("0")),
                ("max", Value::Text("100")),
                ("step", Value::Text("5")),
                ("marks", Value::Text("0 50 100")),
                ("mark_labels", Value::Message(Key::ExampleInputVolumeMarks)),
                (
                    "output_label",
                    Value::Message(Key::ExampleInputPercentValue),
                ),
            ],
            children: &[],
        }],
    ),
    (
        "tag-picker",
        r#"TagPicker { name: "topics", label: "…", placeholder: "…", hint: "…", status, create_label, empty_label, remove_label, added_label, removed_label }
SelectGroup { label: "…", SelectOption { value: "docs", label: "docs" } }"#,
        &[Node::Element {
            tag: "div",
            attributes: &[(
                "class",
                Value::Text("nd-example-stack nd-example-stack-wide"),
            )],
            children: &[Node::Component {
                name: "TagPicker",
                props: &[
                    ("name", Value::Text("overview-topics")),
                    ("label", Value::Message(Key::ExamplePickerLabel)),
                    ("placeholder", Value::Message(Key::ExamplePickerPlaceholder)),
                    ("hint", Value::Message(Key::ExamplePickerHint)),
                    ("status", Value::Message(Key::ExamplePickerStatus)),
                    ("create_label", Value::Message(Key::ExamplePickerCreate)),
                    ("empty_label", Value::Message(Key::ExamplePickerEmpty)),
                    ("remove_label", Value::Message(Key::ExamplePickerRemove)),
                    ("added_label", Value::Message(Key::ExamplePickerAdded)),
                    ("removed_label", Value::Message(Key::ExamplePickerRemoved)),
                ],
                // A suggestion's `value` is its label, because a reader
                // can produce the label by typing it and two ways to
                // make one tag must not submit two different strings.
                children: &[
                    Node::Component {
                        name: "SelectGroup",
                        props: &[("label", Value::Message(Key::ExamplePickerWriting))],
                        children: &[
                            Node::Component {
                                name: "SelectOption",
                                props: &[
                                    ("class", Value::Text("nd-identifier")),
                                    ("value", Value::Text("docs")),
                                    ("label", Value::Data("docs")),
                                ],
                                children: &[],
                            },
                            Node::Component {
                                name: "SelectOption",
                                props: &[
                                    ("class", Value::Text("nd-identifier")),
                                    ("value", Value::Text("spec")),
                                    ("label", Value::Data("spec")),
                                ],
                                children: &[],
                            },
                        ],
                    },
                    Node::Component {
                        name: "SelectGroup",
                        props: &[("label", Value::Message(Key::ExamplePickerBuilding))],
                        children: &[
                            Node::Component {
                                name: "SelectOption",
                                props: &[
                                    ("class", Value::Text("nd-identifier")),
                                    ("value", Value::Text("rust")),
                                    ("label", Value::Data("rust")),
                                ],
                                children: &[],
                            },
                            Node::Component {
                                name: "SelectOption",
                                props: &[
                                    ("class", Value::Text("nd-identifier")),
                                    ("value", Value::Text("css")),
                                    ("label", Value::Data("css")),
                                ],
                                children: &[],
                            },
                        ],
                    },
                ],
            }],
        }],
    ),
    (
        "toolbar-group",
        r#"Toolbar { label: "…" }
ToolbarGroup { … }
ToolbarGroup { align: ToolbarGroupAlign::End, … }"#,
        &[Node::Component {
            name: "Toolbar",
            props: &[("label", Value::Message(Key::ExampleToolbarLabel))],
            children: &[
                Node::Component {
                    name: "ToolbarGroup",
                    props: &[],
                    children: &[
                        Node::Component {
                            name: "Button",
                            props: &[
                                ("variant", Value::Text("transparent")),
                                ("tone", Value::Text("neutral")),
                                ("size", Value::Text("small")),
                                ("hide_label", Value::Bool(true)),
                                ("icon", Value::Text("search")),
                                ("label", Value::Message(Key::SearchLabel)),
                            ],
                            children: &[],
                        },
                        Node::Component {
                            name: "Button",
                            props: &[
                                ("variant", Value::Text("transparent")),
                                ("tone", Value::Text("neutral")),
                                ("size", Value::Text("small")),
                                ("hide_label", Value::Bool(true)),
                                ("icon", Value::Text("columns")),
                                ("label", Value::Message(Key::ExampleTableColumns)),
                            ],
                            children: &[],
                        },
                        Node::Component {
                            name: "Button",
                            props: &[
                                ("variant", Value::Text("transparent")),
                                ("tone", Value::Text("neutral")),
                                ("size", Value::Text("small")),
                                ("hide_label", Value::Bool(true)),
                                ("icon", Value::Text("copy")),
                                ("label", Value::Message(Key::ExampleCopy)),
                            ],
                            children: &[],
                        },
                    ],
                },
                // NEITHER GROUP IS NAMED, which is the ordinary answer: the
                // bar carries the name, and two named clusters inside it is
                // a screen reader saying `group` twice on the way past.
                Node::Component {
                    name: "ToolbarGroup",
                    props: &[("align", Value::Text("end"))],
                    children: &[
                        Node::Component {
                            name: "Segmented",
                            props: &[
                                ("name", Value::Text("overview-toolbar-view")),
                                ("size", Value::Text("small")),
                                ("selection", Value::Text("quiet")),
                                ("label", Value::Message(Key::ExampleToolbarView)),
                            ],
                            children: &[Node::Each {
                                over: Each::Rows(&[
                                    &[
                                        Value::Text("light"),
                                        Value::Message(Key::ThemeModeLight),
                                        Value::Bool(true),
                                    ],
                                    &[
                                        Value::Text("dark"),
                                        Value::Message(Key::ThemeModeDark),
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
                        Node::Component {
                            name: "Button",
                            props: &[
                                ("variant", Value::Text("outline")),
                                ("tone", Value::Text("neutral")),
                                ("size", Value::Text("small")),
                                ("label", Value::Message(Key::ExampleReset)),
                            ],
                            children: &[],
                        },
                    ],
                },
            ],
        }],
    ),
    (
        "date-input",
        r"DateInput { name, legend, hint, day_label, month_label, year_label, pad: true, required: true, required_label, missing_label, number_label, invalid_label, year_digits_label, error_label }",
        // `novalidate`, so an empty answer is refused in the
        // component's own sentence rather than the browser's bubble.
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
                    name: "DateInput",
                    props: &[
                        ("name", Value::Text("overview-expires")),
                        (
                            "legend",
                            Value::Message(Key::ExampleOverviewDateInputLegend),
                        ),
                        ("hint", Value::Message(Key::ExampleOverviewDateInputHint)),
                        ("day_label", Value::Message(Key::ExampleDateInputDay)),
                        ("month_label", Value::Message(Key::ExampleDateInputMonth)),
                        ("year_label", Value::Message(Key::ExampleDateInputYear)),
                        ("pad", Value::Bool(true)),
                        ("required", Value::Bool(true)),
                        (
                            "error_label",
                            Value::Message(Key::ExampleDateInputErrorWord),
                        ),
                        (
                            "required_label",
                            Value::Message(Key::ExampleOverviewDateInputRequired),
                        ),
                        (
                            "missing_label",
                            Value::Message(Key::ExampleDateInputMissing),
                        ),
                        ("number_label", Value::Message(Key::ExampleDateInputNumber)),
                        ("invalid_label", Value::Message(Key::ExampleDateInputReal)),
                        (
                            "year_digits_label",
                            Value::Message(Key::ExampleDateInputYearDigits),
                        ),
                    ],
                    children: &[],
                },
                Node::Component {
                    name: "Button",
                    props: &[
                        ("submit", Value::Text("submit")),
                        ("label", Value::Message(Key::ExampleDateInputSend)),
                    ],
                    children: &[],
                },
            ],
        }],
    ),
    (
        "radio",
        r#"Fieldset { name, legend, hint }
Radio { name: "overview-delivery", value, label, hint, checked, size: Medium }"#,
        &[Node::Component {
            name: "Fieldset",
            props: &[
                ("name", Value::Text("overview-delivery")),
                ("legend", Value::Message(Key::ExampleOverviewRadioLegend)),
                ("hint", Value::Message(Key::ExampleOverviewRadioHint)),
            ],
            children: &[
                Node::Component {
                    name: "Radio",
                    props: &[
                        ("name", Value::Text("overview-delivery")),
                        ("value", Value::Text("standard")),
                        ("label", Value::Message(Key::ExampleOverviewRadioStandard)),
                        (
                            "hint",
                            Value::Message(Key::ExampleOverviewRadioStandardHint),
                        ),
                        ("size", Value::Text("medium")),
                        ("checked", Value::Bool(true)),
                    ],
                    children: &[],
                },
                Node::Component {
                    name: "Radio",
                    props: &[
                        ("name", Value::Text("overview-delivery")),
                        ("value", Value::Text("express")),
                        ("label", Value::Message(Key::ExampleOverviewRadioExpress)),
                        ("hint", Value::Message(Key::ExampleOverviewRadioExpressHint)),
                        ("size", Value::Text("medium")),
                    ],
                    children: &[],
                },
                Node::Component {
                    name: "Radio",
                    props: &[
                        ("name", Value::Text("overview-delivery")),
                        ("value", Value::Text("collect")),
                        ("label", Value::Message(Key::ExampleOverviewRadioCollect)),
                        ("size", Value::Text("medium")),
                    ],
                    children: &[],
                },
            ],
        }],
    ),
    (
        "visually-hidden",
        r#"Link { href: "…", "Read more" VisuallyHidden { " — Typography" } }"#,
        // THREE LINKS THAT READ THE SAME AND ANNOUNCE DIFFERENTLY. The
        // term beside each one is what a sighted reader has and a list
        // of links, read out of context, does not.
        &[Node::Component {
            name: "List",
            props: &[("kind", Value::Text("description"))],
            children: &[
                Node::Component {
                    name: "ListItem",
                    props: &[("term", Value::Message(Key::PageTypographyTitle))],
                    children: &[Node::Component {
                        name: "Link",
                        props: &[("href", Value::Text("../typography/"))],
                        children: &[
                            Node::Text(Value::Message(Key::ExampleOverviewVisuallyHiddenMore)),
                            Node::Component {
                                name: "VisuallyHidden",
                                props: &[],
                                children: &[Node::Text(Value::Join(&[
                                    Value::Text(" — "),
                                    Value::Message(Key::PageTypographyTitle),
                                ]))],
                            },
                        ],
                    }],
                },
                Node::Component {
                    name: "ListItem",
                    props: &[("term", Value::Message(Key::PageSpacingTitle))],
                    children: &[Node::Component {
                        name: "Link",
                        props: &[("href", Value::Text("../spacing/"))],
                        children: &[
                            Node::Text(Value::Message(Key::ExampleOverviewVisuallyHiddenMore)),
                            Node::Component {
                                name: "VisuallyHidden",
                                props: &[],
                                children: &[Node::Text(Value::Join(&[
                                    Value::Text(" — "),
                                    Value::Message(Key::PageSpacingTitle),
                                ]))],
                            },
                        ],
                    }],
                },
                Node::Component {
                    name: "ListItem",
                    props: &[("term", Value::Message(Key::PageIconsTitle))],
                    children: &[Node::Component {
                        name: "Link",
                        props: &[("href", Value::Text("../icons/"))],
                        children: &[
                            Node::Text(Value::Message(Key::ExampleOverviewVisuallyHiddenMore)),
                            Node::Component {
                                name: "VisuallyHidden",
                                props: &[],
                                children: &[Node::Text(Value::Join(&[
                                    Value::Text(" — "),
                                    Value::Message(Key::PageIconsTitle),
                                ]))],
                            },
                        ],
                    }],
                },
            ],
        }],
    ),
    (
        "message-bar",
        r#"MessageBar { tone: "warning", icon: IconName::Warning, title, dismissible: true }"#,
        &[Node::Component {
            name: "MessageBar",
            props: &[
                ("tone", Value::Text("warning")),
                ("icon", Value::Text("warning")),
                ("title", Value::Message(Key::ExampleOverviewMessageBarTitle)),
                ("dismissible", Value::Bool(true)),
                (
                    "dismiss_label",
                    Value::Message(Key::ExampleMessageBarDismiss),
                ),
            ],
            children: &[Node::Text(Value::Message(
                Key::ExampleOverviewMessageBarBody,
            ))],
        }],
    ),
    (
        "toast",
        r#"Toast { tone: "success", title, duration: "6000", dismiss_label }"#,
        &[Node::Element {
            tag: "div",
            attributes: &[("class", Value::Text("nd-example-toasts"))],
            children: &[Node::Component {
                name: "Toast",
                props: &[
                    ("name", Value::Text("overview-toast")),
                    ("tone", Value::Text("success")),
                    ("duration", Value::Text("6000")),
                    ("title", Value::Message(Key::ExampleOverviewToastTitle)),
                    ("dismiss_label", Value::Message(Key::ExampleToastDismiss)),
                ],
                children: &[Node::Text(Value::Message(Key::ExampleOverviewToastBody))],
            }],
        }],
    ),
    (
        "tabs",
        r"Tabs { name, label } + Tab { value, label, count } + TabPanel { value, … }",
        &[Node::Element {
            tag: "div",
            attributes: &[("class", Value::Text("nd-example-stack"))],
            children: &[
                Node::Component {
                    name: "Tabs",
                    props: &[
                        ("name", Value::Text("overview-tabs")),
                        ("label", Value::Message(Key::ExampleOverviewTabsLabel)),
                    ],
                    children: &[
                        Node::Component {
                            name: "Tab",
                            props: &[
                                ("name", Value::Text("overview-tabs")),
                                ("value", Value::Text("items")),
                                ("label", Value::Message(Key::ExampleTabsItems)),
                                ("count", Value::Text("3")),
                                ("selected", Value::Bool(true)),
                            ],
                            children: &[],
                        },
                        Node::Component {
                            name: "Tab",
                            props: &[
                                ("name", Value::Text("overview-tabs")),
                                ("value", Value::Text("delivery")),
                                ("label", Value::Message(Key::ExampleTabsDelivery)),
                            ],
                            children: &[],
                        },
                        Node::Component {
                            name: "Tab",
                            props: &[
                                ("name", Value::Text("overview-tabs")),
                                ("value", Value::Text("payment")),
                                ("label", Value::Message(Key::ExampleTabsPayment)),
                            ],
                            children: &[],
                        },
                    ],
                },
                Node::Component {
                    name: "TabPanel",
                    props: &[
                        ("name", Value::Text("overview-tabs")),
                        ("value", Value::Text("items")),
                    ],
                    children: &[Node::Component {
                        name: "Paragraph",
                        props: &[],
                        children: &[Node::Text(Value::Message(Key::ExampleTabsItemsBody))],
                    }],
                },
                Node::Component {
                    name: "TabPanel",
                    props: &[
                        ("name", Value::Text("overview-tabs")),
                        ("value", Value::Text("delivery")),
                    ],
                    children: &[Node::Component {
                        name: "Paragraph",
                        props: &[],
                        children: &[Node::Text(Value::Message(Key::ExampleTabsDeliveryBody))],
                    }],
                },
                Node::Component {
                    name: "TabPanel",
                    props: &[
                        ("name", Value::Text("overview-tabs")),
                        ("value", Value::Text("payment")),
                    ],
                    children: &[Node::Component {
                        name: "Paragraph",
                        props: &[],
                        children: &[Node::Text(Value::Message(Key::ExampleTabsPaymentBody))],
                    }],
                },
            ],
        }],
    ),
];
