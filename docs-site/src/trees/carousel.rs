//! `Carousel`, as trees.
//!
//! One entry per example in `examples::CAROUSEL`, in the same order. The
//! caption is repeated here so the pairing is pinned to something a
//! reader can check: `every_tree_belongs_to_its_example` reads it against
//! the table's own, and a tree that slid a row apart fails there rather
//! than drawing the wrong picture.

use noctua_design_i18n::Key;

use crate::tree::{Node, Value};
use crate::trees::Tree;

/// The five slides every carousel example on this page is filled with.
///
/// One fragment rather than nine copies, because `examples.rs` wrote it as a
/// macro for that reason: the slides are the scenery, and nine copies of the
/// scenery stop agreeing.
pub(crate) const SLIDES: &[Node] = &[
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
    Node::Component {
        name: "CarouselSlide",
        props: &[("label", Value::Message(Key::ExampleCarouselSlideFive))],
        children: &[Node::Component {
            name: "Paragraph",
            props: &[],
            children: &[Node::Text(Value::Message(Key::ExampleCarouselSlideFive))],
        }],
    },
];

/// Every `Carousel` example. 5 of them.
pub(crate) const EXAMPLES: &[Tree] = &[
    (
        r"Carousel { name, label, snap: Center, previous_label, next_label, announce_label }",
        &[Node::Component {
            name: "Carousel",
            // NO `per_view: "1"` AND NO `controls: true`. Both are the declared
            // default, so the first example on the page opened on two props a
            // reader could delete without changing anything. Each is shown at a
            // value that differs below.
            props: &[
                ("name", Value::Text("gallery")),
                ("label", Value::Message(Key::ExampleCarouselLabel)),
                ("snap", Value::Text("center")),
                (
                    "previous_label",
                    Value::Message(Key::ExampleCarouselPrevious),
                ),
                ("next_label", Value::Message(Key::ExampleCarouselNext)),
                (
                    "announce_label",
                    Value::Message(Key::ExampleCarouselAnnounce),
                ),
            ],
            children: &[Node::Fragment(SLIDES)],
        }],
    ),
    (
        r#"Carousel { per_view: "auto" } — the shelf, which is the shape that survives a telephone"#,
        &[Node::Component {
            name: "Carousel",
            props: &[
                ("name", Value::Text("shelf")),
                ("label", Value::Message(Key::ExampleCarouselShelfLabel)),
                ("per_view", Value::Text("auto")),
                (
                    "previous_label",
                    Value::Message(Key::ExampleCarouselPrevious),
                ),
                ("next_label", Value::Message(Key::ExampleCarouselNext)),
                (
                    "announce_label",
                    Value::Message(Key::ExampleCarouselAnnounce),
                ),
            ],
            children: &[Node::Fragment(SLIDES)],
        }],
    ),
    (
        r"Carousel { dots, dot_label, counter, counter_label, glass }",
        &[Node::Component {
            name: "Carousel",
            props: &[
                ("name", Value::Text("counted")),
                ("label", Value::Message(Key::ExampleCarouselLabel)),
                ("per_view", Value::Text("2")),
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
                ("glass", Value::Bool(true)),
            ],
            children: &[Node::Fragment(SLIDES)],
        }],
    ),
    (
        r"Carousel { autoplay, pause_label, play_label } — and the four rules it carries",
        &[Node::Component {
            name: "Carousel",
            props: &[
                ("name", Value::Text("rotating")),
                ("label", Value::Message(Key::ExampleCarouselLabel)),
                ("autoplay", Value::Text("4000")),
                ("pause_label", Value::Message(Key::ExampleCarouselPause)),
                ("play_label", Value::Message(Key::ExampleCarouselPlay)),
                ("dots", Value::Bool(true)),
                ("dot_label", Value::Message(Key::ExampleCarouselDot)),
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
            children: &[Node::Fragment(SLIDES)],
        }],
    ),
    (
        r"Carousel { per_view, wrap, snap: None, counter, counter_label }
Carousel { orientation: Vertical, controls: false }",
        &[Node::Element {
            tag: "div",
            attributes: &[("class", Value::Text("nd-example-stack"))],
            children: &[
                Node::Component {
                    name: "Carousel",
                    props: &[
                        ("name", Value::Text("looping")),
                        ("label", Value::Message(Key::ExampleCarouselShelfLabel)),
                        ("per_view", Value::Text("3")),
                        ("wrap", Value::Bool(true)),
                        ("snap", Value::Text("none")),
                        ("counter", Value::Bool(true)),
                        ("counter_label", Value::Message(Key::ExampleCarouselCounter)),
                        (
                            "previous_label",
                            Value::Message(Key::ExampleCarouselPrevious),
                        ),
                        ("next_label", Value::Message(Key::ExampleCarouselNext)),
                        (
                            "announce_label",
                            Value::Message(Key::ExampleCarouselAnnounce),
                        ),
                    ],
                    children: &[Node::Fragment(SLIDES)],
                },
                // THE ONE CAROUSEL THAT CAN DO WITHOUT THE ARROWS. The contract
                // draws them because a mouse with no horizontal wheel cannot
                // move a sideways track. This track runs downwards, where every
                // wheel already works.
                Node::Component {
                    name: "Carousel",
                    props: &[
                        ("name", Value::Text("downwards")),
                        ("label", Value::Message(Key::ExampleCarouselLabel)),
                        ("orientation", Value::Text("vertical")),
                        ("controls", Value::Bool(false)),
                        (
                            "announce_label",
                            Value::Message(Key::ExampleCarouselAnnounce),
                        ),
                    ],
                    children: &[Node::Fragment(SLIDES)],
                },
            ],
        }],
    ),
];
