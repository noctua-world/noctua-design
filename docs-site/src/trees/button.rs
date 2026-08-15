//! `Button`, as trees.
//!
//! One entry per example in `examples::BUTTON`, in the same order. The
//! caption is repeated here so the pairing is pinned to something a
//! reader can check: `every_tree_belongs_to_its_example` reads it against
//! the table's own, and a tree that slid a row apart fails there rather
//! than drawing the wrong picture.

use noctua_design_i18n::Key;

use crate::tree::{Each, Node, Value};
use crate::trees::Tree;

/// Every `Button` example. 17 of them.
pub(crate) const EXAMPLES: &[Tree] = &[
    (
        r#"Button { label: "…" }"#,
        &[Node::Component {
            name: "Button",
            props: &[("label", Value::Message(Key::ExampleButtonSave))],
            children: &[],
        }],
    ),
    (
        r"Button { variant: ButtonVariant::… }",
        &[Node::Element {
            tag: "div",
            attributes: &[("class", Value::Text("nd-example-row"))],
            // Read from the contract, not listed: a seventh variant added to
            // `spec/components/button.toml` draws itself here.
            children: &[Node::Each {
                over: Each::Values {
                    component: "Button",
                    prop: "variant",
                },
                body: &[Node::Component {
                    name: "Button",
                    props: &[
                        ("variant", Value::Bound(0)),
                        ("label", Value::Message(Key::ExampleButtonSave)),
                    ],
                    children: &[],
                }],
            }],
        }],
    ),
    (
        r"Button { tone: ButtonTone::… }",
        &[Node::Element {
            tag: "div",
            attributes: &[("class", Value::Text("nd-example-row"))],
            children: &[Node::Each {
                over: Each::Values {
                    component: "Button",
                    prop: "tone",
                },
                body: &[Node::Component {
                    name: "Button",
                    props: &[
                        ("tone", Value::Bound(0)),
                        ("label", Value::Message(Key::ExampleButtonPublish)),
                    ],
                    children: &[],
                }],
            }],
        }],
    ),
    (
        r"Button { size: ButtonSize::… }",
        &[Node::Element {
            tag: "div",
            attributes: &[("class", Value::Text("nd-example-row"))],
            children: &[Node::Each {
                over: Each::Values {
                    component: "Button",
                    prop: "size",
                },
                body: &[Node::Component {
                    name: "Button",
                    props: &[
                        ("size", Value::Bound(0)),
                        ("label", Value::Message(Key::ExampleButtonSave)),
                    ],
                    children: &[],
                }],
            }],
        }],
    ),
    (
        r"Button { shape: ButtonShape::… }",
        &[Node::Element {
            tag: "div",
            attributes: &[("class", Value::Text("nd-example-row"))],
            children: &[Node::Each {
                over: Each::Values {
                    component: "Button",
                    prop: "shape",
                },
                body: &[Node::Component {
                    name: "Button",
                    props: &[
                        ("shape", Value::Bound(0)),
                        ("label", Value::Message(Key::ExampleButtonSave)),
                    ],
                    children: &[],
                }],
            }],
        }],
    ),
    (
        r"Button { icon: IconName::Check, … }",
        &[Node::Element {
            tag: "div",
            attributes: &[("class", Value::Text("nd-example-row"))],
            children: &[
                Node::Component {
                    name: "Button",
                    props: &[
                        ("icon", Value::Text("check")),
                        ("label", Value::Message(Key::ExampleButtonPublish)),
                    ],
                    children: &[],
                },
                Node::Component {
                    name: "Button",
                    props: &[
                        ("icon", Value::Text("copy")),
                        ("variant", Value::Text("outline")),
                        ("tone", Value::Text("neutral")),
                        ("label", Value::Message(Key::ExampleCopy)),
                    ],
                    children: &[],
                },
            ],
        }],
    ),
    (
        r"Button { icon_end: IconName::ExternalLink | ::ChevronRight, … }",
        &[Node::Element {
            tag: "div",
            attributes: &[("class", Value::Text("nd-example-row"))],
            children: &[
                Node::Component {
                    name: "Button",
                    props: &[
                        ("icon_end", Value::Text("external-link")),
                        ("variant", Value::Text("outline")),
                        ("tone", Value::Text("neutral")),
                        ("label", Value::Message(Key::AppRepository)),
                    ],
                    children: &[],
                },
                // A second, because the prop is a CHOICE: a destination and a
                // direction are the two things a trailing icon is ever for, and
                // one button cannot show a choice.
                Node::Component {
                    name: "Button",
                    props: &[
                        ("icon_end", Value::Text("chevron-right")),
                        ("variant", Value::Text("subtle")),
                        ("label", Value::Message(Key::ExampleButtonMore)),
                    ],
                    children: &[],
                },
            ],
        }],
    ),
    (
        r"Button { hide_label: true, icon: …, … }",
        &[Node::Element {
            tag: "div",
            attributes: &[("class", Value::Text("nd-example-row"))],
            children: &[Node::Each {
                over: Each::Values {
                    component: "Button",
                    prop: "size",
                },
                body: &[Node::Component {
                    name: "Button",
                    props: &[
                        ("size", Value::Bound(0)),
                        ("hide_label", Value::Bool(true)),
                        ("icon", Value::Text("settings")),
                        ("variant", Value::Text("transparent")),
                        ("tone", Value::Text("neutral")),
                        ("label", Value::Message(Key::SettingsLabel)),
                    ],
                    children: &[],
                }],
            }],
        }],
    ),
    (
        r#"Button { detail: "…", justify: ButtonJustify::Start, … }"#,
        &[Node::Element {
            tag: "div",
            attributes: &[("class", Value::Text("nd-example-narrow"))],
            children: &[Node::Component {
                name: "Button",
                props: &[
                    ("icon", Value::Text("languages")),
                    ("label", Value::Message(Key::ExampleButtonPublish)),
                    ("detail", Value::Message(Key::ExampleButtonDetail)),
                    ("justify", Value::Text("start")),
                    ("variant", Value::Text("outline")),
                    ("tone", Value::Text("neutral")),
                    ("fill", Value::Bool(true)),
                ],
                children: &[],
            }],
        }],
    ),
    (
        r"Button { wrap: true, fill: true, … }",
        &[Node::Element {
            tag: "div",
            attributes: &[("class", Value::Text("nd-example-narrow"))],
            children: &[Node::Component {
                name: "Button",
                props: &[
                    ("wrap", Value::Bool(true)),
                    ("fill", Value::Bool(true)),
                    ("label", Value::Message(Key::ExampleButtonLong)),
                ],
                children: &[],
            }],
        }],
    ),
    (
        r#"Button { menu: true, expanded, controls: "…" }"#,
        &[Node::Element {
            tag: "div",
            attributes: &[],
            children: &[
                // IT OPENS SOMETHING NOW. This shipped as a button carrying
                // `aria-haspopup="menu"` over a hidden paragraph — announcing a
                // menu to a screen reader and doing nothing on every press,
                // because the system had no menu for it to open. `Menu` is that
                // component, and `dist/menu.js` is what makes the press work.
                Node::Component {
                    name: "Button",
                    props: &[
                        ("menu", Value::Bool(true)),
                        ("expanded", Value::Bool(false)),
                        ("controls", Value::Text("example-button-actions")),
                        ("variant", Value::Text("outline")),
                        ("tone", Value::Text("neutral")),
                        ("label", Value::Message(Key::ExampleButtonMore)),
                    ],
                    children: &[],
                },
                Node::Component {
                    name: "Menu",
                    props: &[
                        ("name", Value::Text("example-button-actions")),
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
                    ],
                },
            ],
        }],
    ),
    (
        r#"Button { href: "…", rel: "noreferrer", target: "_blank" }"#,
        &[Node::Element {
            tag: "div",
            attributes: &[("class", Value::Text("nd-example-row"))],
            children: &[
                Node::Component {
                    name: "Button",
                    props: &[
                        (
                            "href",
                            Value::Text("https://github.com/noctua-world/noctua-design"),
                        ),
                        ("rel", Value::Text("noreferrer")),
                        ("target", Value::Text("_blank")),
                        ("icon_end", Value::Text("external-link")),
                        ("variant", Value::Text("outline")),
                        ("tone", Value::Text("neutral")),
                        ("label", Value::Message(Key::AppRepository)),
                    ],
                    children: &[],
                },
                Node::Component {
                    name: "Button",
                    props: &[
                        ("href", Value::Text("#")),
                        ("variant", Value::Text("link")),
                        ("label", Value::Message(Key::ExampleLinkText)),
                    ],
                    children: &[],
                },
            ],
        }],
    ),
    (
        r#"Button { pressed: true, icon_alt: …, label_alt: "…", action: "…" }"#,
        &[Node::Element {
            tag: "div",
            attributes: &[("class", Value::Text("nd-example-row"))],
            children: &[
                Node::Component {
                    name: "Button",
                    props: &[
                        ("hide_label", Value::Bool(true)),
                        ("icon", Value::Text("moon")),
                        ("icon_alt", Value::Text("sun")),
                        ("label", Value::Message(Key::ThemeModeDark)),
                        ("label_alt", Value::Message(Key::ThemeModeLight)),
                        ("variant", Value::Text("transparent")),
                        ("tone", Value::Text("neutral")),
                    ],
                    children: &[],
                },
                Node::Component {
                    name: "Button",
                    props: &[
                        ("pressed", Value::Bool(true)),
                        ("hide_label", Value::Bool(true)),
                        ("icon", Value::Text("moon")),
                        ("icon_alt", Value::Text("sun")),
                        // A TOGGLE IS THE BUTTON MOST LIKELY TO NEED FINDING. It
                        // has no form to post to and no address to go to, so the
                        // only thing that can act on a press is a script — and this
                        // is how the script finds it.
                        ("action", Value::Text("example-mode")),
                        ("label", Value::Message(Key::ThemeModeDark)),
                        ("label_alt", Value::Message(Key::ThemeModeLight)),
                        ("variant", Value::Text("transparent")),
                        ("tone", Value::Text("neutral")),
                    ],
                    children: &[],
                },
            ],
        }],
    ),
    (
        r"Button { disabled: true, … }",
        &[Node::Element {
            tag: "div",
            attributes: &[("class", Value::Text("nd-example-row"))],
            children: &[
                Node::Component {
                    name: "Button",
                    props: &[
                        ("disabled", Value::Bool(true)),
                        ("label", Value::Message(Key::ExampleButtonSave)),
                    ],
                    children: &[],
                },
                Node::Component {
                    name: "Button",
                    props: &[
                        ("disabled", Value::Bool(true)),
                        ("variant", Value::Text("outline")),
                        ("tone", Value::Text("neutral")),
                        ("label", Value::Message(Key::ExampleButtonCancel)),
                    ],
                    children: &[],
                },
            ],
        }],
    ),
    (
        r#"Button { submit: ButtonSubmit::Submit, name: "…", value: "…" }"#,
        // A REAL FORM, BECAUSE `type` HAS NO APPEARANCE. Three buttons with
        // one label differing only in an attribute that does not paint is
        // three identical pictures — the prop looked like it did nothing.
        // Put them in a form with a field and the difference is the whole
        // point: `reset` empties it, `submit` sends it, `button` waits.
        &[Node::Element {
            tag: "form",
            attributes: &[
                ("class", Value::Text("nd-example-stack")),
                // Nothing leaves the page: the example is what the buttons do
                // to the field beside them, not where a real form would go.
                ("action", Value::Text("#")),
            ],
            children: &[
                Node::Component {
                    name: "Input",
                    props: &[
                        ("name", Value::Text("example-button-subject")),
                        ("label", Value::Message(Key::ExampleButtonSubject)),
                        ("value", Value::Message(Key::ExampleButtonPublish)),
                    ],
                    children: &[],
                },
                Node::Element {
                    tag: "div",
                    attributes: &[("class", Value::Text("nd-example-row"))],
                    children: &[Node::Each {
                        over: Each::Rows(&[
                            &[
                                Value::Text("submit"),
                                Value::Message(Key::ExampleButtonSend),
                                Value::Text("solid"),
                                Value::Text("accent"),
                            ],
                            &[
                                Value::Text("button"),
                                Value::Message(Key::ExampleButtonSave),
                                Value::Text("outline"),
                                Value::Text("neutral"),
                            ],
                            &[
                                Value::Text("reset"),
                                Value::Message(Key::ExampleButtonClear),
                                Value::Text("subtle"),
                                Value::Text("danger"),
                            ],
                        ]),
                        body: &[Node::Component {
                            name: "Button",
                            props: &[
                                ("submit", Value::Bound(0)),
                                // WHICH BUTTON WAS PRESSED, and the only way to ask
                                // it with no script: the browser posts the name and
                                // value of the one that was used and of no other.
                                ("name", Value::Text("action")),
                                ("value", Value::Bound(0)),
                                ("variant", Value::Bound(2)),
                                ("tone", Value::Bound(3)),
                                ("label", Value::Bound(1)),
                            ],
                            children: &[],
                        }],
                    }],
                },
            ],
        }],
    ),
    (
        r"Button { variant: ButtonVariant::Nav, current: true, … }",
        &[Node::Element {
            tag: "div",
            attributes: &[("class", Value::Text("nd-example-stack"))],
            children: &[
                Node::Component {
                    name: "Button",
                    props: &[
                        ("variant", Value::Text("nav")),
                        ("href", Value::Text("#")),
                        ("current", Value::Bool(true)),
                        ("label", Value::Message(Key::ExampleButtonSave)),
                    ],
                    children: &[],
                },
                Node::Component {
                    name: "Button",
                    props: &[
                        ("variant", Value::Text("nav")),
                        ("href", Value::Text("#")),
                        ("label", Value::Message(Key::ExampleButtonPublish)),
                    ],
                    children: &[],
                },
            ],
        }],
    ),
    (
        r"Button { glass: true, … }",
        &[Node::Element {
            tag: "div",
            attributes: &[("class", Value::Text("nd-example-row"))],
            children: &[Node::Each {
                over: Each::Values {
                    component: "Button",
                    prop: "tone",
                },
                body: &[Node::Component {
                    name: "Button",
                    props: &[
                        ("glass", Value::Bool(true)),
                        ("tone", Value::Bound(0)),
                        ("label", Value::Message(Key::ExampleButtonSave)),
                    ],
                    children: &[],
                }],
            }],
        }],
    ),
];
