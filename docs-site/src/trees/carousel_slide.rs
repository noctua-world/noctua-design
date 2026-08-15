//! `CarouselSlide`, as trees.
//!
//! One entry per example in `examples::CAROUSEL_SLIDE`, in the same order. The
//! caption is repeated here so the pairing is pinned to something a
//! reader can check: `every_tree_belongs_to_its_example` reads it against
//! the table's own, and a tree that slid a row apart fails there rather
//! than drawing the wrong picture.

use noctua_design_i18n::Key;

use crate::tree::{Node, Value};
use crate::trees::Tree;

/// Every `CarouselSlide` example. 4 of them.
pub(crate) const EXAMPLES: &[Tree] = &[
    (
        r"CarouselSlide { label } — added to the position, never instead of it",
        &[Node::Component {
            name: "Carousel",
            props: &[
                ("name", Value::Text("named-slides")),
                ("label", Value::Message(Key::ExampleCarouselLabel)),
                ("per_view", Value::Text("2")),
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
            children: &[Node::Fragment(crate::trees::carousel::SLIDES)],
        }],
    ),
    (
        r"CarouselSlide { children } — anything focusable in one stays in the tab order",
        &[Node::Component {
            name: "Carousel",
            props: &[
                ("name", Value::Text("with-links")),
                ("label", Value::Message(Key::ExampleCarouselShelfLabel)),
                ("per_view", Value::Text("auto")),
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
            children: &[
                Node::Component {
                    name: "CarouselSlide",
                    props: &[],
                    children: &[Node::Component {
                        name: "Link",
                        props: &[("href", Value::Text("../button/"))],
                        children: &[Node::Text(Value::Message(Key::ExampleCarouselSlideOne))],
                    }],
                },
                Node::Component {
                    name: "CarouselSlide",
                    props: &[],
                    children: &[Node::Component {
                        name: "Link",
                        props: &[("href", Value::Text("../link/"))],
                        children: &[Node::Text(Value::Message(Key::ExampleCarouselSlideTwo))],
                    }],
                },
                Node::Component {
                    name: "CarouselSlide",
                    props: &[],
                    children: &[Node::Component {
                        name: "Link",
                        props: &[("href", Value::Text("../list/"))],
                        children: &[Node::Text(Value::Message(Key::ExampleCarouselSlideThree))],
                    }],
                },
            ],
        }],
    ),
    (
        r"CarouselSlide — with no label, the position alone is the name",
        &[Node::Component {
            name: "Carousel",
            props: &[
                ("name", Value::Text("unnamed-slides")),
                ("label", Value::Message(Key::ExampleCarouselLabel)),
                ("per_view", Value::Text("3")),
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
            children: &[
                Node::Component {
                    name: "CarouselSlide",
                    props: &[],
                    children: &[Node::Component {
                        name: "Paragraph",
                        props: &[],
                        children: &[Node::Text(Value::Message(Key::ExampleCarouselSlideOne))],
                    }],
                },
                Node::Component {
                    name: "CarouselSlide",
                    props: &[],
                    children: &[Node::Component {
                        name: "Paragraph",
                        props: &[],
                        children: &[Node::Text(Value::Message(Key::ExampleCarouselSlideTwo))],
                    }],
                },
                Node::Component {
                    name: "CarouselSlide",
                    props: &[],
                    children: &[Node::Component {
                        name: "Paragraph",
                        props: &[],
                        children: &[Node::Text(Value::Message(Key::ExampleCarouselSlideThree))],
                    }],
                },
                Node::Component {
                    name: "CarouselSlide",
                    props: &[],
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
        r"CarouselSlide — in a vertical carousel, the same markup",
        &[Node::Component {
            name: "Carousel",
            props: &[
                ("name", Value::Text("slides-down")),
                ("label", Value::Message(Key::ExampleCarouselLabel)),
                ("orientation", Value::Text("vertical")),
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
            children: &[Node::Fragment(crate::trees::carousel::SLIDES)],
        }],
    ),
];
