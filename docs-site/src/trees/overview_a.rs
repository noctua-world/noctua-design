//! Hero examples, group A, as trees.
//!
//! One entry per page, keyed by the page's slug — the hero is the one
//! example a page leads with, and `examples::OVERVIEWS` keys them the
//! same way. Split four ways only so the migration could be shared out;
//! nothing about a page decides which group it is in.

use noctua_design_i18n::Key;

use crate::tree::{Each, Node, Value};
use crate::trees::Hero;

/// The heroes in this group. 16 of them.
pub(crate) const HEROES: &[Hero] = &[
    (
        "button",
        r#"Button { variant: Solid, tone: Accent, icon: "check", label: "Publish" }
Button { variant: Outline, label: "Save" }
Button { menu: true, controls: "overview-button-menu", label: "More actions" }"#,
        &[Node::Element {
            tag: "div",
            attributes: &[("class", Value::Text("nd-example-row"))],
            children: &[
                Node::Component {
                    name: "Button",
                    props: &[
                        ("variant", Value::Text("solid")),
                        ("tone", Value::Text("accent")),
                        ("icon", Value::Text("check")),
                        ("label", Value::Message(Key::ExampleButtonPublish)),
                    ],
                    children: &[],
                },
                Node::Component {
                    name: "Button",
                    props: &[
                        ("variant", Value::Text("outline")),
                        ("tone", Value::Text("neutral")),
                        ("label", Value::Message(Key::ExampleButtonSave)),
                    ],
                    children: &[],
                },
                Node::Component {
                    name: "Button",
                    props: &[
                        ("variant", Value::Text("transparent")),
                        ("tone", Value::Text("neutral")),
                        ("icon", Value::Text("copy")),
                        ("label", Value::Message(Key::ExampleCopy)),
                    ],
                    children: &[],
                },
                Node::Component {
                    name: "Button",
                    props: &[
                        ("menu", Value::Bool(true)),
                        ("controls", Value::Text("overview-button-menu")),
                        ("variant", Value::Text("outline")),
                        ("tone", Value::Text("neutral")),
                        ("label", Value::Message(Key::ExampleButtonMore)),
                    ],
                    children: &[],
                },
                Node::Component {
                    name: "Menu",
                    props: &[
                        ("name", Value::Text("overview-button-menu")),
                        ("label", Value::Message(Key::ExampleMenuLabel)),
                    ],
                    children: &[
                        Node::Component {
                            name: "MenuItem",
                            props: &[("label", Value::Message(Key::ExampleMenuDuplicate))],
                            children: &[],
                        },
                        Node::Component {
                            name: "MenuItem",
                            props: &[("label", Value::Message(Key::ExampleMenuRename))],
                            children: &[],
                        },
                        Node::Component {
                            name: "MenuItem",
                            props: &[
                                ("tone", Value::Text("danger")),
                                ("label", Value::Message(Key::ExampleMenuDelete)),
                                ("detail", Value::Message(Key::ExampleMenuDeleteDetail)),
                            ],
                            children: &[],
                        },
                    ],
                },
            ],
        }],
    ),
    (
        "carousel",
        r#"Carousel { name, label, per_view: "1", snap: Center, dots: true, counter: true }
CarouselSlide { label: "…" }"#,
        &[Node::Component {
            name: "Carousel",
            props: &[
                ("name", Value::Text("overview-carousel")),
                ("label", Value::Message(Key::ExampleCarouselLabel)),
                ("per_view", Value::Text("1")),
                ("snap", Value::Text("center")),
                ("dots", Value::Bool(true)),
                ("dot_label", Value::Message(Key::ExampleCarouselDot)),
                ("counter", Value::Bool(true)),
                ("counter_label", Value::Message(Key::ExampleCarouselCounter)),
                (
                    "announce_label",
                    Value::Message(Key::ExampleCarouselAnnounce),
                ),
                (
                    "previous_label",
                    Value::Message(Key::ExampleCarouselPrevious),
                ),
                ("next_label", Value::Message(Key::ExampleCarouselNext)),
            ],
            // FOUR SLIDES, where `carousel::SLIDES` carries five. The hero
            // writes its own set rather than calling the page's macro, so a
            // fragment here would draw a slide the closure never did.
            children: &[
                Node::Component {
                    name: "CarouselSlide",
                    props: &[("label", Value::Message(Key::ExampleCarouselSlideOne))],
                    children: &[Node::Component {
                        name: "Paragraph",
                        props: &[],
                        children: &[Node::Text(Value::Message(Key::ExampleCarouselSlideOne))],
                    }],
                },
                Node::Component {
                    name: "CarouselSlide",
                    props: &[("label", Value::Message(Key::ExampleCarouselSlideTwo))],
                    children: &[Node::Component {
                        name: "Paragraph",
                        props: &[],
                        children: &[Node::Text(Value::Message(Key::ExampleCarouselSlideTwo))],
                    }],
                },
                Node::Component {
                    name: "CarouselSlide",
                    props: &[("label", Value::Message(Key::ExampleCarouselSlideThree))],
                    children: &[Node::Component {
                        name: "Paragraph",
                        props: &[],
                        children: &[Node::Text(Value::Message(Key::ExampleCarouselSlideThree))],
                    }],
                },
                Node::Component {
                    name: "CarouselSlide",
                    props: &[("label", Value::Message(Key::ExampleCarouselSlideFour))],
                    children: &[Node::Component {
                        name: "Paragraph",
                        props: &[],
                        children: &[Node::Text(Value::Message(Key::ExampleCarouselSlideFour))],
                    }],
                },
            ],
        }],
    ),
    (
        "disclosure",
        r#"Disclosure { summary: "…", heading: DisclosureHeading::H3, open: true }
Disclosure { summary: "…", heading: DisclosureHeading::H3 }"#,
        &[Node::Element {
            tag: "div",
            attributes: &[("class", Value::Text("nd-example-stack"))],
            children: &[
                Node::Component {
                    name: "Disclosure",
                    props: &[
                        ("summary", Value::Message(Key::ExampleDisclosureSummary)),
                        ("heading", Value::Text("h3")),
                        ("open", Value::Bool(true)),
                    ],
                    children: &[Node::Component {
                        name: "Paragraph",
                        props: &[],
                        children: &[Node::Text(Value::Message(Key::ExampleDisclosureBody))],
                    }],
                },
                Node::Component {
                    name: "Disclosure",
                    props: &[
                        ("summary", Value::Message(Key::ExampleDisclosureSection)),
                        ("heading", Value::Text("h3")),
                    ],
                    children: &[Node::Component {
                        name: "Paragraph",
                        props: &[],
                        children: &[Node::Text(Value::Message(
                            Key::ExampleOverviewDisclosureA11y,
                        ))],
                    }],
                },
            ],
        }],
    ),
    (
        "heading",
        r"Heading { level: _2, size: HeadingSize::Display, balance: true }
Heading { level: _3, size: HeadingSize::H3, tone: HeadingTone::Muted }",
        &[Node::Component {
            name: "Prose",
            props: &[("measure", Value::Text("default"))],
            children: &[
                Node::Component {
                    name: "Heading",
                    props: &[
                        ("level", Value::Text("2")),
                        ("size", Value::Text("display")),
                        ("balance", Value::Bool(true)),
                    ],
                    children: &[Node::Text(Value::Message(Key::ExampleHeadingLong))],
                },
                Node::Component {
                    name: "Paragraph",
                    props: &[
                        ("size", Value::Text("lead")),
                        ("tone", Value::Text("muted")),
                    ],
                    children: &[Node::Text(Value::Message(Key::ExampleListLong))],
                },
                Node::Component {
                    name: "Heading",
                    props: &[
                        ("level", Value::Text("3")),
                        ("size", Value::Text("h3")),
                        ("tone", Value::Text("muted")),
                    ],
                    children: &[Node::Text(Value::Message(Key::ExampleHeadingSection))],
                },
            ],
        }],
    ),
    (
        "list",
        r#"List { kind: ListKind::Ordered } with ListItem { "…" }
List { kind: ListKind::Description, spacing: Tight } with ListItem { term: "…", "…" }"#,
        &[Node::Element {
            tag: "div",
            attributes: &[("class", Value::Text("nd-example-stack"))],
            children: &[
                Node::Component {
                    name: "List",
                    props: &[("kind", Value::Text("ordered"))],
                    children: &[
                        Node::Component {
                            name: "ListItem",
                            props: &[],
                            children: &[Node::Text(Value::Message(
                                Key::ExampleOverviewListDeclare,
                            ))],
                        },
                        Node::Component {
                            name: "ListItem",
                            props: &[],
                            children: &[Node::Text(Value::Message(Key::ExampleOverviewListBuild))],
                        },
                        Node::Component {
                            name: "ListItem",
                            props: &[],
                            children: &[Node::Text(Value::Message(
                                Key::ExampleOverviewListExamples,
                            ))],
                        },
                    ],
                },
                Node::Component {
                    name: "List",
                    props: &[
                        ("kind", Value::Text("description")),
                        ("spacing", Value::Text("tight")),
                    ],
                    children: &[
                        Node::Component {
                            name: "ListItem",
                            props: &[("term", Value::Message(Key::ThemeAccentLabel))],
                            children: &[Node::Text(Value::Message(Key::ExampleTermAccent))],
                        },
                        Node::Component {
                            name: "ListItem",
                            props: &[("term", Value::Message(Key::ThemeGlassLabel))],
                            children: &[Node::Text(Value::Message(Key::ExampleTermGlass))],
                        },
                    ],
                },
            ],
        }],
    ),
    (
        "pagination",
        r#"Pagination { name, label, page: "3", pages: "4", href, previous_label,
next_label, page_label, current_label, summary }"#,
        &[Node::Component {
            name: "Pagination",
            props: &[
                ("name", Value::Text("overview-pagination")),
                ("label", Value::Message(Key::ExamplePaginationLabel)),
                // Ten palettes a page, thirty-nine of them, and the summary
                // under the control says exactly that — a control whose
                // numbers disagree with its own sentence is one nobody trusts.
                ("page", Value::Text("3")),
                ("pages", Value::Text("4")),
                ("href", Value::Text("#page-{page}")),
                (
                    "previous_label",
                    Value::Message(Key::ExamplePaginationPrevious),
                ),
                ("next_label", Value::Message(Key::ExamplePaginationNext)),
                ("page_label", Value::Message(Key::ExamplePaginationPage)),
                (
                    "current_label",
                    Value::Message(Key::ExamplePaginationCurrent),
                ),
                ("summary", Value::Message(Key::ExamplePaginationSummary)),
            ],
            children: &[],
        }],
    ),
    (
        "search",
        r#"Search { name, label, placeholder, status, empty_label, clear_label }
SearchGroup { label: "…" }
SearchResult { href: "…", label: "…", detail: "…", keywords: "…", current: true }"#,
        &[Node::Component {
            name: "Search",
            props: &[
                ("name", Value::Text("overview-search")),
                ("label", Value::Message(Key::SearchLabel)),
                ("placeholder", Value::Message(Key::SearchPlaceholder)),
                ("status", Value::Message(Key::SearchResults)),
                ("empty_label", Value::Message(Key::SearchEmpty)),
                ("clear_label", Value::Message(Key::SearchClear)),
            ],
            children: &[
                Node::Component {
                    name: "SearchGroup",
                    props: &[("label", Value::Message(Key::SearchGroupPages))],
                    children: &[
                        Node::Component {
                            name: "SearchResult",
                            props: &[
                                ("href", Value::Text("../spacing/")),
                                ("label", Value::PageTitle("spacing")),
                                ("detail", Value::Message(Key::PageSpacingSummary)),
                                // Letters a reader might type that the name does
                                // not contain: `gap` reaches this row and nothing
                                // else.
                                ("keywords", Value::Text("gap inset step scale rhythm")),
                            ],
                            children: &[],
                        },
                        Node::Component {
                            name: "SearchResult",
                            props: &[
                                ("href", Value::Text("../typography/")),
                                ("label", Value::PageTitle("typography")),
                                ("detail", Value::Message(Key::PageTypographySummary)),
                            ],
                            children: &[],
                        },
                    ],
                },
                Node::Component {
                    name: "SearchGroup",
                    props: &[("label", Value::Message(Key::SearchGroupComponents))],
                    children: &[
                        Node::Component {
                            name: "SearchResult",
                            props: &[
                                ("class", Value::Text("nd-identifier")),
                                ("href", Value::Text("../button/")),
                                ("label", Value::Data("Button")),
                            ],
                            children: &[],
                        },
                        Node::Component {
                            name: "SearchResult",
                            props: &[
                                ("class", Value::Text("nd-identifier")),
                                ("href", Value::Text("../select/")),
                                ("label", Value::Data("Select")),
                            ],
                            children: &[],
                        },
                        // The page this is on, so `current` is a claim the
                        // document can back rather than a decoration.
                        Node::Component {
                            name: "SearchResult",
                            props: &[
                                ("class", Value::Text("nd-identifier")),
                                ("href", Value::Text("../search/")),
                                ("label", Value::PageTitle("search")),
                                ("current", Value::Bool(true)),
                            ],
                            children: &[],
                        },
                    ],
                },
            ],
        }],
    ),
    (
        "select-option",
        r#"SelectOption { value: "…", label: "…", swatch: "…", detail: "…", selected: true }"#,
        &[Node::Component {
            name: "Select",
            props: &[
                ("name", Value::Text("overview-select-option")),
                ("label", Value::Message(Key::ExampleSelectPaletteLabel)),
                ("hint", Value::Message(Key::ExampleSelectPaletteHint)),
            ],
            children: &[
                Node::Fragment(crate::trees::select::AMBER_DETAILED),
                // The one option the page's own fragments cannot supply, since
                // `selected` is what this hero is showing.
                Node::Component {
                    name: "SelectOption",
                    props: &[
                        ("class", Value::Text("nd-identifier")),
                        ("value", Value::Text("ochre-balanced")),
                        ("label", Value::Data("Ochre")),
                        ("swatch", Value::Text("ochre-balanced")),
                        ("detail", Value::Message(Key::ExampleSelectDetailBalanced)),
                        ("selected", Value::Bool(true)),
                    ],
                    children: &[],
                },
                Node::Fragment(crate::trees::select::CLAY_DETAILED),
            ],
        }],
    ),
    (
        "tag",
        r#"Tag { icon: IconName::Check, tone: "shipped", label: "Shipped" }
Tag { tone: "accent", variant: TagVariant::Solid, label: "In review" }
Tag { tone: "danger", variant: TagVariant::Outline, label: "Blocked" }"#,
        &[Node::Element {
            tag: "div",
            attributes: &[("class", Value::Text("nd-example-row"))],
            children: &[
                Node::Component {
                    name: "Tag",
                    props: &[
                        ("icon", Value::Text("check")),
                        ("tone", Value::Text("shipped")),
                        ("label", Value::Message(Key::ExampleTagShipped)),
                    ],
                    children: &[],
                },
                Node::Component {
                    name: "Tag",
                    props: &[
                        ("tone", Value::Text("accent")),
                        ("variant", Value::Text("solid")),
                        ("label", Value::Message(Key::ExampleTagReview)),
                    ],
                    children: &[],
                },
                Node::Component {
                    name: "Tag",
                    props: &[("label", Value::Message(Key::ExampleTagDraft))],
                    children: &[],
                },
                Node::Component {
                    name: "Tag",
                    props: &[
                        ("tone", Value::Text("danger")),
                        ("variant", Value::Text("outline")),
                        ("label", Value::Message(Key::ExampleTagBlocked)),
                    ],
                    children: &[],
                },
            ],
        }],
    ),
    (
        "toolbar",
        r#"Toolbar { label: "…" }
ToolbarGroup { … } / ToolbarGroup { align: ToolbarGroupAlign::End, … }"#,
        &[Node::Component {
            name: "Toolbar",
            props: &[("label", Value::Message(Key::ExampleToolbarLabel))],
            children: &[
                Node::Component {
                    name: "ToolbarGroup",
                    props: &[],
                    children: &[Node::Fragment(crate::trees::toolbar::CONTROLS)],
                },
                Node::Component {
                    name: "ToolbarGroup",
                    props: &[("align", Value::Text("end"))],
                    children: &[
                        Node::Component {
                            name: "Segmented",
                            props: &[
                                ("name", Value::Text("overview-toolbar-view")),
                                ("size", Value::Text("small")),
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
                                ("variant", Value::Text("transparent")),
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
        "button-group",
        r#"ButtonGroup { label: "Publishing", … }
Button { label: "Publish" }
Button { menu: true, hide_label: true, icon: "chevron-down", controls: "overview-button-group-menu" }"#,
        &[Node::Element {
            tag: "div",
            attributes: &[("class", Value::Text("nd-example-row"))],
            children: &[
                Node::Component {
                    name: "ButtonGroup",
                    props: &[("label", Value::Message(Key::ExampleGroupSplitLabel))],
                    children: &[
                        Node::Component {
                            name: "Button",
                            props: &[("label", Value::Message(Key::ExampleButtonPublish))],
                            children: &[],
                        },
                        Node::Component {
                            name: "Button",
                            props: &[
                                ("menu", Value::Bool(true)),
                                ("hide_label", Value::Bool(true)),
                                ("icon", Value::Text("chevron-down")),
                                ("controls", Value::Text("overview-button-group-menu")),
                                ("label", Value::Message(Key::ExampleButtonMore)),
                            ],
                            children: &[],
                        },
                    ],
                },
                Node::Component {
                    name: "Menu",
                    props: &[
                        ("name", Value::Text("overview-button-group-menu")),
                        ("label", Value::Message(Key::ExampleButtonMore)),
                    ],
                    children: &[
                        Node::Component {
                            name: "MenuItem",
                            props: &[(
                                "label",
                                Value::Message(Key::ExampleOverviewButtonGroupSchedule),
                            )],
                            children: &[],
                        },
                        Node::Component {
                            name: "MenuItem",
                            props: &[(
                                "label",
                                Value::Message(Key::ExampleOverviewButtonGroupDraft),
                            )],
                            children: &[],
                        },
                        Node::Component {
                            name: "MenuItem",
                            props: &[("label", Value::Message(Key::ExampleMenuArchive))],
                            children: &[],
                        },
                    ],
                },
            ],
        }],
    ),
    (
        "number-input",
        r#"NumberInput { name, label, hint, value, min, max, step, big_step, prefix: "R$", group: true, increment_label, decrement_label, max_label }"#,
        &[Node::Component {
            name: "NumberInput",
            props: &[
                ("name", Value::Text("overview-price")),
                ("label", Value::Message(Key::ExampleInputPriceLabel)),
                ("hint", Value::Message(Key::ExampleOverviewNumberInputHint)),
                ("value", Value::Text("1499.9")),
                ("min", Value::Text("0")),
                ("max", Value::Text("9999.99")),
                ("step", Value::Text("0.01")),
                ("big_step", Value::Text("10")),
                ("prefix", Value::Data("R$")),
                ("group", Value::Bool(true)),
                (
                    "increment_label",
                    Value::Message(Key::ExampleOverviewNumberInputUp),
                ),
                (
                    "decrement_label",
                    Value::Message(Key::ExampleOverviewNumberInputDown),
                ),
                (
                    "max_label",
                    Value::Message(Key::ExampleOverviewNumberInputTooBig),
                ),
            ],
            children: &[],
        }],
    ),
    (
        "time-input",
        r"TimeInput { name, legend, hint, hour_label, minute_label, meridiem_label, am_label, pm_label, value, min, max, step, pad, now_label, required, required_label, missing_label, invalid_label, min_label, max_label }",
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
                    name: "TimeInput",
                    props: &[
                        ("name", Value::Text("overview-slot")),
                        ("legend", Value::Message(Key::ExampleInputTimeLegend)),
                        ("hint", Value::Message(Key::ExampleInputTimeHint)),
                        ("hour_label", Value::Message(Key::ExampleInputHour)),
                        ("minute_label", Value::Message(Key::ExampleInputMinute)),
                        ("meridiem_label", Value::Message(Key::ExampleInputMeridiem)),
                        ("am_label", Value::Message(Key::ExampleInputAm)),
                        ("pm_label", Value::Message(Key::ExampleInputPm)),
                        ("value", Value::Text("09:30")),
                        ("min", Value::Text("08:00")),
                        ("max", Value::Text("18:00")),
                        ("step", Value::Text("15")),
                        ("pad", Value::Bool(true)),
                        ("now_label", Value::Message(Key::ExampleInputNow)),
                        ("required", Value::Bool(true)),
                        (
                            "required_label",
                            Value::Message(Key::ExampleInputTimeNeeded),
                        ),
                        (
                            "missing_label",
                            Value::Message(Key::ExampleInputTimeMissing),
                        ),
                        (
                            "invalid_label",
                            Value::Message(Key::ExampleInputTimeInvalid),
                        ),
                        ("min_label", Value::Message(Key::ExampleInputTimeEarly)),
                        ("max_label", Value::Message(Key::ExampleInputTimeLate)),
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
        "image",
        r"Image { src, alt, ratio: Landscape, caption, width, height }",
        &[Node::Element {
            tag: "div",
            attributes: &[("class", Value::Text("nd-example-picture"))],
            children: &[Node::Component {
                name: "Image",
                props: &[
                    ("src", Value::Asset("assets/portraits/gran.svg")),
                    ("alt", Value::Message(Key::ExampleImageAlt)),
                    ("caption", Value::Message(Key::ExampleOverviewImageCaption)),
                    ("ratio", Value::Text("landscape")),
                    ("width", Value::Text("480")),
                    ("height", Value::Text("360")),
                ],
                children: &[],
            }],
        }],
    ),
    (
        "tooltip",
        r#"Tooltip { name, text: "…", Button { icon, label } }"#,
        &[Node::Element {
            tag: "div",
            attributes: &[("class", Value::Text("nd-example-row"))],
            children: &[
                Node::Component {
                    name: "Tooltip",
                    props: &[
                        ("name", Value::Text("overview-tooltip-copy")),
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
                },
                Node::Component {
                    name: "Tooltip",
                    props: &[
                        ("name", Value::Text("overview-tooltip-export")),
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
                },
            ],
        }],
    ),
    (
        "otp-field",
        r"OtpField { name, legend, box_label, length, hint, invalid_label }",
        &[Node::Component {
            name: "OtpField",
            props: &[
                ("name", Value::Text("overview-otp")),
                ("legend", Value::Message(Key::ExampleOverviewOtpFieldLegend)),
                ("box_label", Value::Message(Key::ExampleOtpFieldBox)),
                ("length", Value::Text("6")),
                ("hint", Value::Message(Key::ExampleOverviewOtpFieldHint)),
                ("invalid_label", Value::Message(Key::ExampleOtpFieldInvalid)),
            ],
            children: &[],
        }],
    ),
];
