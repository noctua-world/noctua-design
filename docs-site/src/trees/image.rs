//! `Image`, as trees.
//!
//! One entry per example in `examples::IMAGE`, in the same order. The
//! caption is repeated here so the pairing is pinned to something a
//! reader can check: `every_example_has_a_tree` reads it against
//! the table's own, and a tree that slid a row apart fails there rather
//! than drawing the wrong picture.

use noctua_design_i18n::Key;

use crate::tree::{Each, Node, Value};
use crate::trees::Tree;

/// Every `Image` example. 5 of them.
pub(crate) const EXAMPLES: &[Tree] = &[
    // THE BOX BEFORE THE BYTES, which is the whole of this component. `width`
    // and `height` are the file's own numbers, handed to the browser before any
    // stylesheet has loaded.
    (
        r"Image { src, alt, ratio, width, height }",
        &[Node::Element {
            tag: "div",
            attributes: &[("class", Value::Text("nd-example-picture"))],
            children: &[Node::Component {
                name: "Image",
                // NO `fit: Cover` AND NO `loading`. `cover` is the default and
                // `loading` was named in the caption without ever being passed.
                // Both are shown further down at a value that differs.
                props: &[
                    ("src", Value::Asset("assets/portraits/ada.svg")),
                    ("alt", Value::Message(Key::ExampleImageAlt)),
                    ("ratio", Value::Text("landscape")),
                    ("width", Value::Text("480")),
                    ("height", Value::Text("360")),
                ],
                children: &[],
            }],
        }],
    ),
    // A CAPTION MAKES IT A `<figure>`, which is what attaches the words to the
    // picture rather than leaving them under it.
    (
        r"Image { caption, ratio: Square }",
        &[Node::Element {
            tag: "div",
            attributes: &[("class", Value::Text("nd-example-picture"))],
            children: &[Node::Component {
                name: "Image",
                props: &[
                    ("src", Value::Asset("assets/portraits/eve.svg")),
                    ("alt", Value::Message(Key::ExampleImageAlt)),
                    ("caption", Value::Message(Key::ExampleImageCaption)),
                    ("ratio", Value::Text("square")),
                ],
                children: &[],
            }],
        }],
    ),
    // AN EMPTY `alt` IS AN ANSWER, and it is the right one here: the sentence
    // beside the picture already says who this is.
    (
        r#"Image { alt: "", rounded: true, ratio: Square, loading: Eager }"#,
        &[Node::Element {
            tag: "span",
            attributes: &[("class", Value::Text("nd-example-byline"))],
            children: &[
                Node::Element {
                    tag: "span",
                    attributes: &[("class", Value::Text("nd-example-face"))],
                    children: &[Node::Component {
                        name: "Image",
                        props: &[
                            ("src", Value::Asset("assets/portraits/cara.svg")),
                            ("alt", Value::Text("")),
                            ("rounded", Value::Bool(true)),
                            ("ratio", Value::Text("square")),
                            ("loading", Value::Text("eager")),
                        ],
                        children: &[],
                    }],
                },
                Node::Component {
                    name: "Text",
                    props: &[],
                    children: &[Node::Text(Value::Message(Key::ExampleImageBeside))],
                },
            ],
        }],
    ),
    // ROUNDED AND `contain`, for a drawing whose edges carry meaning. Cropping
    // a mark to fill a box is how a mark stops being the mark.
    (
        r"Image { rounded: true, fit: Contain, ratio: Wide }",
        &[Node::Element {
            tag: "div",
            attributes: &[("class", Value::Text("nd-example-picture"))],
            children: &[Node::Component {
                name: "Image",
                props: &[
                    ("src", Value::Asset("favicon.svg")),
                    ("alt", Value::Message(Key::ExampleImageMarkAlt)),
                    ("rounded", Value::Bool(true)),
                    ("fit", Value::Text("contain")),
                    ("ratio", Value::Text("wide")),
                ],
                children: &[],
            }],
        }],
    ),
    // THE FOUR SHAPES, ITERATED, over one file. What changes is the box.
    (
        r"Image { ratio } for ratio in ImageRatio::ALL",
        &[Node::Element {
            tag: "div",
            attributes: &[("class", Value::Text("nd-example-row"))],
            children: &[Node::Each {
                over: Each::Values {
                    component: "Image",
                    prop: "ratio",
                },
                body: &[Node::Element {
                    tag: "div",
                    attributes: &[("class", Value::Text("nd-example-thumb"))],
                    children: &[Node::Component {
                        name: "Image",
                        props: &[
                            ("src", Value::Asset("assets/portraits/bob.svg")),
                            ("alt", Value::Message(Key::ExampleImageAlt)),
                            ("ratio", Value::Bound(0)),
                        ],
                        children: &[],
                    }],
                }],
            }],
        }],
    ),
];
