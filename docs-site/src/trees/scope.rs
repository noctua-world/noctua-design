//! The `ThemeScope` demonstration, as a tree.
//!
//! # Why this one is written by hand and the other 499 were transcribed
//!
//! This is not a worked example out of `examples.rs`. It is the frame on
//! `/theme-scope/` that mounts the same scope twice, once from each
//! implementation, so a reader can see that `NOCTUA.md §9.3` Leg 4 holds rather
//! than read that it does.
//!
//! It was the **only** frame on the site that could do that, and it did it from
//! a hand-written mirror of itself in `docs-site/js/example-react.js` — a copy,
//! and a copy of an example is an example that stops agreeing with the one it
//! copies. Every example is data now, so this one is too, and the copy is gone.
//!
//! `ThemeScope` comes from `spec/theme.toml` rather than `spec/components/`, so
//! the generated interpreter has no arm for it. `tree::theme_scope` is that arm.

use noctua_design_i18n::Key;

use crate::tree::{Node, Value};

/// The scope, and the sample inside it.
///
/// The sample mirrors nothing: it **is** the markup, and both implementations
/// draw it from here. `pages::Sample` is what this replaces.
pub(crate) const SAMPLE: &[Node] = &[Node::Component {
    name: "ThemeScope",
    props: &[
        ("class", Value::Text("nd-scope")),
        ("palette", Value::Text("azure-vivid")),
        ("mode", Value::Text("light")),
        ("glass", Value::Text("full")),
        ("radius", Value::Text("round")),
    ],
    children: &[Node::Element {
        tag: "div",
        attributes: &[("class", Value::Text("nd-sample"))],
        children: &[
            Node::Element {
                tag: "h4",
                attributes: &[("class", Value::Text("nd-sample-heading"))],
                children: &[Node::Text(Value::Message(Key::ScopeSampleHeading))],
            },
            Node::Element {
                tag: "p",
                attributes: &[("class", Value::Text("nd-prose"))],
                children: &[
                    // The trailing space is part of the markup: the sentence and
                    // the link are one run of prose, and the byte comparison
                    // sees a dropped space.
                    Node::Text(Value::Join(&[
                        Value::Message(Key::ScopeSampleBody),
                        Value::Text(" "),
                    ])),
                    Node::Element {
                        tag: "a",
                        attributes: &[
                            ("class", Value::Text("nd-link")),
                            ("href", Value::Text("#")),
                        ],
                        children: &[Node::Text(Value::Message(Key::ScopeSampleLink))],
                    },
                ],
            },
            Node::Element {
                tag: "div",
                attributes: &[("class", Value::Text("nd-sample-actions"))],
                children: &[
                    Node::Component {
                        name: "Button",
                        props: &[("label", Value::Message(Key::ScopeSampleAction))],
                        children: &[],
                    },
                    Node::Component {
                        name: "Button",
                        props: &[
                            ("variant", Value::Text("outline")),
                            ("tone", Value::Text("neutral")),
                            ("label", Value::Message(Key::ScopeSampleQuiet)),
                        ],
                        children: &[],
                    },
                ],
            },
        ],
    }],
}];
