//! Hero examples, group C, as trees.
//!
//! One entry per page, keyed by the page's slug — the hero is the one
//! example a page leads with, and `examples::OVERVIEWS` keys them the
//! same way. Split four ways only so the migration could be shared out;
//! nothing about a page decides which group it is in.

use noctua_design_i18n::Key;

use crate::tree::{Each, Node, Value};
use crate::trees::Hero;
use crate::trees::menu::menu_button;

/// The sample the `code-block` hero highlights.
///
/// A copy of the one `trees::code_block` carries, because that one is private
/// to its own module and a hero cannot reach it. Two copies of a sample are two
/// copies that stop agreeing, so this wants one shared home.
const SAMPLE_RUST: &str = r"/// Every attribute a scope stamps, or none.
pub fn scope_attributes(request: &ScopeRequest) -> Option<Attributes> {
    let resolved = resolve(request)?;
    if resolved == inherited {
        return None; // a no-op scope is 1,767 declarations for nothing
    }
    Some(Attributes { palette: resolved.palette, mode: resolved.mode })
}";

/// The heroes in this group. 13 of them.
pub(crate) const HEROES: &[Hero] = &[
    (
        "badge",
        r#"Badge { count: "3", label: "3 unread messages", Button { … } }
Badge { count: "128", max: "99", tone: "accent", … }
Badge { dot: true, tone: "neutral", … }"#,
        // A ROOMIER ROW, for the reason the placement example below
        // gives: a badge draws OUTSIDE the box of what it marks, and
        // three of them in an ordinary row sit on their neighbours.
        &[Node::Element {
            tag: "div",
            attributes: &[("class", Value::Text("nd-example-row nd-example-row-airy"))],
            children: &[
                Node::Component {
                    name: "Badge",
                    props: &[
                        ("count", Value::Text("3")),
                        ("label", Value::Message(Key::ExampleBadgeUnread)),
                    ],
                    children: &[Node::Component {
                        name: "Button",
                        props: &[
                            ("variant", Value::Text("outline")),
                            ("tone", Value::Text("neutral")),
                            ("label", Value::Message(Key::ExampleBadgeInbox)),
                        ],
                        children: &[],
                    }],
                },
                Node::Component {
                    name: "Badge",
                    props: &[
                        ("count", Value::Text("128")),
                        ("max", Value::Text("99")),
                        ("tone", Value::Text("accent")),
                        ("label", Value::Message(Key::ExampleOverviewBadgeWaiting)),
                    ],
                    children: &[Node::Component {
                        name: "Button",
                        props: &[
                            ("variant", Value::Text("outline")),
                            ("tone", Value::Text("neutral")),
                            ("label", Value::Message(Key::ExampleOverviewBadgeReviews)),
                        ],
                        children: &[],
                    }],
                },
                Node::Component {
                    name: "Badge",
                    props: &[
                        ("dot", Value::Bool(true)),
                        ("tone", Value::Text("neutral")),
                        ("label", Value::Message(Key::ExampleBadgeChanged)),
                    ],
                    children: &[Node::Component {
                        name: "Button",
                        props: &[
                            ("variant", Value::Text("transparent")),
                            ("tone", Value::Text("neutral")),
                            ("hide_label", Value::Bool(true)),
                            ("icon", Value::Text("settings")),
                            ("label", Value::Message(Key::SettingsLabel)),
                        ],
                        children: &[],
                    }],
                },
            ],
        }],
    ),
    (
        "code-block",
        r#"CodeBlock { code, language: "rust", filename: "scope.rs", numbered: true, mark_lines: "4-6", copy: "…", copied: "…", caption: "…" }"#,
        &[Node::Component {
            name: "CodeBlock",
            props: &[
                ("code", Value::Data(SAMPLE_RUST)),
                ("language", Value::Text("rust")),
                ("filename", Value::Text("scope.rs")),
                ("numbered", Value::Bool(true)),
                ("mark_lines", Value::Text("4-6")),
                ("copy", Value::Message(Key::ExampleCopy)),
                ("copied", Value::Message(Key::ExampleCopied)),
                (
                    "caption",
                    Value::Message(Key::ExampleOverviewCodeBlockCaption),
                ),
            ],
            children: &[],
        }],
    ),
    (
        "field",
        r"Field { control, label, hint, required, required_label } / Field { kind: Group, label, hint }",
        &[Node::Element {
            tag: "div",
            attributes: &[("class", Value::Text("nd-example-stack"))],
            children: &[
                Node::Component {
                    name: "Input",
                    props: &[
                        ("name", Value::Text("overview-field-email")),
                        ("label", Value::Message(Key::ExampleInputEmailLabel)),
                        ("kind", Value::Text("email")),
                        ("hint", Value::Message(Key::ExampleInputHint)),
                        ("required", Value::Bool(true)),
                        ("required_label", Value::Message(Key::ExampleInputRequired)),
                        ("autocomplete", Value::Text("email")),
                    ],
                    children: &[],
                },
                // The other `kind`: a `<legend>` over three boxes, because a
                // `<label>` names one element and a set of controls needs a
                // name of its own.
                Node::Component {
                    name: "TimeInput",
                    props: &[
                        ("name", Value::Text("overview-field-time")),
                        ("legend", Value::Message(Key::ExampleInputTimeLegend)),
                        ("hour_label", Value::Message(Key::ExampleInputHour)),
                        ("minute_label", Value::Message(Key::ExampleInputMinute)),
                        ("hint", Value::Message(Key::ExampleInputTimeHint)),
                    ],
                    children: &[],
                },
            ],
        }],
    ),
    (
        "inline-text",
        r#"Paragraph {
    Strong { … } Time { datetime: "2026-07-29", … } Code { … } Del { label: "removed:", … } Ins { label: "added:", … }
    Abbr { title: "…", … } Em { … } Mark { … } Kbd { … } Text { numeric: true, … }
}"#,
        &[Node::Component {
            name: "Paragraph",
            props: &[],
            children: &[
                Node::Component {
                    name: "Strong",
                    props: &[],
                    children: &[Node::Text(Value::Message(
                        Key::ExampleOverviewInlineTextRelease,
                    ))],
                },
                Node::Text(Value::Join(&[
                    Value::Text(" "),
                    Value::Message(Key::ExampleOverviewInlineTextShipped),
                    Value::Text(" "),
                ])),
                Node::Component {
                    name: "Time",
                    props: &[("datetime", Value::Text("2026-07-29"))],
                    children: &[Node::Text(Value::Message(Key::ExampleTimeDate))],
                },
                Node::Text(Value::Join(&[
                    Value::Message(Key::ExampleOverviewInlineTextInside),
                    Value::Text(" "),
                ])),
                Node::Component {
                    name: "Code",
                    props: &[("class", Value::Text("nd-identifier"))],
                    children: &[Node::Text(Value::Data("ThemeScope"))],
                },
                Node::Text(Value::Join(&[
                    Value::Message(Key::ExampleOverviewInlineTextOld),
                    Value::Text(" "),
                ])),
                Node::Component {
                    name: "Del",
                    props: &[
                        ("class", Value::Text("nd-identifier")),
                        ("label", Value::Message(Key::ExampleEditRemoved)),
                    ],
                    children: &[Node::Text(Value::Data("--noctua-accent"))],
                },
                Node::Text(Value::Join(&[
                    Value::Text(" "),
                    Value::Message(Key::ExampleOverviewInlineTextBecame),
                    Value::Text(" "),
                ])),
                Node::Component {
                    name: "Ins",
                    props: &[
                        ("class", Value::Text("nd-identifier")),
                        ("label", Value::Message(Key::ExampleEditAdded)),
                    ],
                    children: &[Node::Text(Value::Data("--nc-accent-solid"))],
                },
                Node::Text(Value::Join(&[
                    Value::Message(Key::ExampleOverviewInlineTextFloor),
                    Value::Text(" "),
                ])),
                Node::Component {
                    name: "Abbr",
                    props: &[
                        ("class", Value::Text("nd-identifier")),
                        ("title", Value::Data("Web Content Accessibility Guidelines")),
                    ],
                    children: &[Node::Text(Value::Data("WCAG"))],
                },
                Node::Text(Value::Join(&[
                    Value::Text(" "),
                    Value::Message(Key::ExampleOverviewInlineTextFloorEnd),
                    Value::Text(" "),
                ])),
                Node::Component {
                    name: "Em",
                    props: &[],
                    children: &[Node::Text(Value::Message(
                        Key::ExampleOverviewInlineTextEvery,
                    ))],
                },
                Node::Text(Value::Join(&[
                    Value::Text(" "),
                    Value::Message(Key::ExampleOverviewInlineTextSearch),
                    Value::Text(" "),
                ])),
                Node::Component {
                    name: "Mark",
                    props: &[],
                    children: &[Node::Text(Value::Message(Key::ExampleMarkWord))],
                },
                Node::Text(Value::Join(&[
                    Value::Text(" "),
                    Value::Message(Key::ExampleOverviewInlineTextAgain),
                    Value::Text(" "),
                ])),
                Node::Component {
                    name: "Kbd",
                    props: &[("class", Value::Text("nd-identifier"))],
                    children: &[Node::Text(Value::Data("Esc"))],
                },
                Node::Text(Value::Join(&[
                    Value::Text(" "),
                    Value::Message(Key::ExampleOverviewInlineTextCloses),
                    Value::Text(" "),
                ])),
                Node::Component {
                    name: "Text",
                    props: &[
                        ("class", Value::Text("nd-identifier")),
                        ("numeric", Value::Bool(true)),
                    ],
                    children: &[Node::Text(Value::Data("1 767"))],
                },
                Node::Text(Value::Join(&[
                    Value::Text(" "),
                    Value::Message(Key::ExampleOverviewInlineTextDeclarations),
                ])),
            ],
        }],
    ),
    (
        "menu",
        r#"Button { menu: true, controls: "overview-menu-actions", label: "Actions" }
Menu { name: "overview-menu-actions", label: "…",
    MenuItem { icon: "copy", label: "…" }
    MenuItem { icon: "close", tone: "danger", label: "…", detail: "…" }
}"#,
        &[Node::Element {
            tag: "div",
            attributes: &[],
            children: &[
                menu_button!("overview-menu-actions"),
                Node::Component {
                    name: "Menu",
                    props: &[
                        ("name", Value::Text("overview-menu-actions")),
                        ("label", Value::Message(Key::ExampleMenuLabel)),
                    ],
                    children: &[
                        Node::Component {
                            name: "MenuItem",
                            props: &[
                                ("icon", Value::Text("copy")),
                                ("label", Value::Message(Key::ExampleMenuDuplicate)),
                            ],
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
                                ("icon", Value::Text("download")),
                                ("label", Value::Message(Key::ExampleMenuArchive)),
                            ],
                            children: &[],
                        },
                        Node::Component {
                            name: "MenuItem",
                            props: &[
                                ("icon", Value::Text("close")),
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
        "progress",
        r"Progress { name, label, value, max, value_label }
Progress { name, label, busy_label } — no value at all",
        &[Node::Element {
            tag: "div",
            attributes: &[("class", Value::Text("nd-example-stack"))],
            children: &[
                Node::Component {
                    name: "Progress",
                    props: &[
                        ("name", Value::Text("overview-upload")),
                        ("label", Value::Message(Key::ExampleProgressUpload)),
                        ("value", Value::Text("7.4")),
                        ("max", Value::Text("12")),
                        (
                            "value_label",
                            Value::Message(Key::ExampleProgressUploadValue),
                        ),
                    ],
                    children: &[],
                },
                Node::Component {
                    name: "Progress",
                    props: &[
                        ("name", Value::Text("overview-import")),
                        ("label", Value::Message(Key::ExampleProgressImport)),
                        ("busy_label", Value::Message(Key::ExampleProgressImportBusy)),
                    ],
                    children: &[],
                },
            ],
        }],
    ),
    // `segmented-item` had a hero here, and `/segmented/`'s own draws the same
    // three modes with the same icons. `SegmentedItem` is documented on
    // `/segmented/` now, and a page has one hero.
    (
        "swatch",
        r"SegmentedItem { value, label, Swatch { palette: value, size: SwatchSize::Small } }",
        &[Node::Component {
            name: "Segmented",
            props: &[
                ("name", Value::Text("overview-swatch-accent")),
                ("label", Value::Message(Key::ThemeAccentLabel)),
                ("appearance", Value::Text("spread")),
            ],
            children: &[Node::Each {
                over: Each::Rows(&[
                    &[
                        Value::Text("amber-vivid"),
                        Value::Message(Key::ThemeAccentAmber),
                        Value::Bool(false),
                    ],
                    &[
                        Value::Text("jade-balanced"),
                        Value::Message(Key::ThemeAccentJade),
                        Value::Bool(true),
                    ],
                    &[
                        Value::Text("teal-sober"),
                        Value::Message(Key::ThemeAccentTeal),
                        Value::Bool(false),
                    ],
                    &[
                        Value::Text("rose-vivid"),
                        Value::Message(Key::ThemeAccentRose),
                        Value::Bool(false),
                    ],
                ]),
                // The chip carries the palette and takes its colour from
                // it, so no colour is written here — and the label is
                // what a reader who cannot see the chip chooses by.
                body: &[Node::Component {
                    name: "SegmentedItem",
                    props: &[
                        ("value", Value::Bound(0)),
                        ("label", Value::Bound(1)),
                        ("selected", Value::Bound(2)),
                    ],
                    children: &[Node::Component {
                        name: "Swatch",
                        props: &[("palette", Value::Bound(0)), ("size", Value::Text("small"))],
                        children: &[],
                    }],
                }],
            }],
        }],
    ),
    (
        "input",
        r#"Input { name, label, kind: Email, placeholder: "…", hint, required, required_label, autocomplete }
Input { kind: Password, reveal_label, conceal_label }"#,
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
                    name: "Input",
                    props: &[
                        ("name", Value::Text("overview-email")),
                        ("label", Value::Message(Key::ExampleInputEmailLabel)),
                        ("kind", Value::Text("email")),
                        ("placeholder", Value::Data("ada@example.org")),
                        ("hint", Value::Message(Key::ExampleInputHint)),
                        ("required", Value::Bool(true)),
                        ("required_label", Value::Message(Key::ExampleInputRequired)),
                        ("autocomplete", Value::Text("email")),
                    ],
                    children: &[],
                },
                Node::Component {
                    name: "Input",
                    props: &[
                        ("name", Value::Text("overview-password")),
                        ("label", Value::Message(Key::ExampleInputPasswordLabel)),
                        ("kind", Value::Text("password")),
                        ("hint", Value::Message(Key::ExampleInputPasswordHint)),
                        ("required", Value::Bool(true)),
                        ("required_label", Value::Message(Key::ExampleInputRequired)),
                        ("autocomplete", Value::Text("new-password")),
                        ("reveal_label", Value::Message(Key::ExampleInputReveal)),
                        ("conceal_label", Value::Message(Key::ExampleInputConceal)),
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
        "tree",
        r#"Tree { name, label, guides: true, expand_label, collapse_label,
    TreeItem { label, icon, badge: "3", expanded: true, TreeItem { href, label } }
    TreeItem { label, icon, branch: true, href }
}"#,
        &[Node::Component {
            name: "Tree",
            props: &[
                ("name", Value::Text("overview-pages")),
                ("label", Value::Message(Key::ExampleTreeLabel)),
                ("guides", Value::Bool(true)),
                ("expand_label", Value::Message(Key::ExampleTreeExpand)),
                ("collapse_label", Value::Message(Key::ExampleTreeCollapse)),
            ],
            children: &[
                Node::Component {
                    name: "TreeItem",
                    props: &[
                        ("label", Value::Message(Key::ExampleTreeFoundations)),
                        ("icon", Value::Text("palette")),
                        ("badge", Value::Text("3")),
                        ("expanded", Value::Bool(true)),
                    ],
                    children: &[
                        Node::Component {
                            name: "TreeItem",
                            props: &[
                                ("href", Value::Text("../typography/")),
                                ("label", Value::PageTitle("typography")),
                            ],
                            children: &[],
                        },
                        Node::Component {
                            name: "TreeItem",
                            props: &[
                                ("href", Value::Text("../spacing/")),
                                ("label", Value::PageTitle("spacing")),
                            ],
                            children: &[],
                        },
                        Node::Component {
                            name: "TreeItem",
                            props: &[
                                ("href", Value::Text("../icons/")),
                                ("label", Value::PageTitle("icons")),
                            ],
                            children: &[],
                        },
                    ],
                },
                Node::Component {
                    name: "TreeItem",
                    props: &[
                        ("label", Value::Message(Key::ExampleTreeControls)),
                        ("icon", Value::Text("settings")),
                        ("expanded", Value::Bool(true)),
                    ],
                    children: &[
                        Node::Component {
                            name: "TreeItem",
                            props: &[
                                ("class", Value::Text("nd-identifier")),
                                ("href", Value::Text("../button/")),
                                ("label", Value::Data("Button")),
                            ],
                            children: &[],
                        },
                        Node::Component {
                            name: "TreeItem",
                            props: &[
                                ("class", Value::Text("nd-identifier")),
                                ("href", Value::Text("../tree/")),
                                ("label", Value::Data("Tree")),
                                ("current", Value::Bool(true)),
                            ],
                            children: &[],
                        },
                    ],
                },
                // Children that were not passed: the lazily-loaded shape.
                // Nothing is hidden, because nothing is there.
                Node::Component {
                    name: "TreeItem",
                    props: &[
                        ("label", Value::Message(Key::ExampleTreeEverything)),
                        ("icon", Value::Text("search")),
                        ("branch", Value::Bool(true)),
                        ("href", Value::Text("../search/")),
                    ],
                    children: &[],
                },
            ],
        }],
    ),
    (
        "file-input",
        r#"FileInput { name, label, hint, browse_label, drop_label, multiple: true, max: "3", max_label, accept: ".pdf,image/*", accept_label, max_size, max_size_label, remove_label, empty_label, added_label, removed_label, status }"#,
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
                        ("name", Value::Text("overview-attachments")),
                        ("label", Value::Message(Key::ExampleInputFilesLabel)),
                        ("hint", Value::Message(Key::ExampleInputFilesHint)),
                        ("browse_label", Value::Message(Key::ExampleInputBrowse)),
                        ("drop_label", Value::Message(Key::ExampleInputDrop)),
                        ("multiple", Value::Bool(true)),
                        ("max", Value::Text("3")),
                        ("max_label", Value::Message(Key::ExampleInputFileTooMany)),
                        ("accept", Value::Text(".pdf,image/*")),
                        ("accept_label", Value::Message(Key::ExampleInputFileType)),
                        ("max_size", Value::Text("2097152")),
                        (
                            "max_size_label",
                            Value::Message(Key::ExampleInputFileTooBig),
                        ),
                        ("remove_label", Value::Message(Key::ExampleInputFileRemove)),
                        ("empty_label", Value::Message(Key::ExampleInputFileEmpty)),
                        ("added_label", Value::Message(Key::ExampleInputFileAdded)),
                        (
                            "removed_label",
                            Value::Message(Key::ExampleInputFileRemoved),
                        ),
                        ("status", Value::Message(Key::ExampleInputFileStatus)),
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
    // `select-group` had a hero here, and it was the same two groups over the
    // same six palettes that `/select/`'s own hero draws. `SelectGroup` is
    // documented on `/select/` now, and a page has one hero.
    (
        "divider",
        r#"Divider { spacing: Loose }
Divider { label: "or", spacing: Loose }"#,
        &[Node::Element {
            tag: "div",
            attributes: &[],
            children: &[
                Node::Component {
                    name: "Heading",
                    props: &[("level", Value::Text("3")), ("size", Value::Text("h4"))],
                    children: &[Node::Text(Value::Message(Key::ExampleOverviewDividerPay))],
                },
                Node::Component {
                    name: "Paragraph",
                    props: &[],
                    children: &[Node::Text(Value::Message(
                        Key::ExampleOverviewDividerPayBody,
                    ))],
                },
                // A WORD ON THE RULE, because these two are alternatives
                // rather than one section after another.
                Node::Component {
                    name: "Divider",
                    props: &[
                        ("label", Value::Message(Key::ExampleDividerOr)),
                        ("spacing", Value::Text("loose")),
                    ],
                    children: &[],
                },
                Node::Component {
                    name: "Heading",
                    props: &[("level", Value::Text("3")), ("size", Value::Text("h4"))],
                    children: &[Node::Text(Value::Message(
                        Key::ExampleOverviewDividerInvoice,
                    ))],
                },
                Node::Component {
                    name: "Paragraph",
                    props: &[],
                    children: &[Node::Text(Value::Message(
                        Key::ExampleOverviewDividerInvoiceBody,
                    ))],
                },
            ],
        }],
    ),
    (
        "dialog",
        r"Dialog { name, trigger_label, heading, description, close_label, … }",
        &[Node::Component {
            name: "Dialog",
            props: &[
                ("name", Value::Text("overview-dialog")),
                (
                    "trigger_label",
                    Value::Message(Key::ExampleOverviewDialogOpen),
                ),
                ("heading", Value::Message(Key::ExampleOverviewDialogHeading)),
                (
                    "description",
                    Value::Message(Key::ExampleOverviewDialogBody),
                ),
                ("close_label", Value::Message(Key::ExampleDialogClose)),
            ],
            children: &[
                Node::Component {
                    name: "Paragraph",
                    props: &[],
                    children: &[Node::Text(Value::Message(Key::ExampleOverviewDialogBody))],
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
                                ("tone", Value::Text("accent")),
                                ("label", Value::Message(Key::ExampleOverviewDialogConfirm)),
                            ],
                            children: &[],
                        },
                    ],
                },
            ],
        }],
    ),
    (
        "icon",
        r#"Icon { icon: "check", label: "…" } / Icon { icon: "star", size: Large }"#,
        &[Node::Element {
            tag: "div",
            attributes: &[("class", Value::Text("nd-example-row"))],
            children: &[
                // Named, because the drawing is the whole message here.
                Node::Component {
                    name: "Icon",
                    props: &[
                        ("icon", Value::Text("check")),
                        ("label", Value::Message(Key::ExampleOverviewIconStatus)),
                    ],
                    children: &[],
                },
                Node::Component {
                    name: "Text",
                    props: &[],
                    children: &[Node::Text(Value::Message(Key::ExampleOverviewIconStatus))],
                },
                Node::Component {
                    name: "Icon",
                    props: &[
                        ("icon", Value::Text("star")),
                        ("size", Value::Text("small")),
                    ],
                    children: &[],
                },
                Node::Component {
                    name: "Icon",
                    props: &[("icon", Value::Text("star"))],
                    children: &[],
                },
                Node::Component {
                    name: "Icon",
                    props: &[
                        ("icon", Value::Text("star")),
                        ("size", Value::Text("large")),
                    ],
                    children: &[],
                },
            ],
        }],
    ),
];
