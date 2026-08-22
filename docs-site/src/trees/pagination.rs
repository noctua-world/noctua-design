//! `Pagination`, as trees.
//!
//! One entry per example in `examples::PAGINATION`, in the same order. The
//! caption is repeated here so the pairing is pinned to something a
//! reader can check: `every_example_has_a_tree` reads it against
//! the table's own, and a tree that slid a row apart fails there rather
//! than drawing the wrong picture.

use noctua_design_i18n::Key;

use crate::tree::{Node, Value};
use crate::trees::Tree;

/// Every `Pagination` example. 4 of them.
pub(crate) const EXAMPLES: &[Tree] = &[
    (
        r"Pagination { name, label, page, pages, href, previous_label, next_label, page_label, current_label }",
        &[Node::Component {
            name: "Pagination",
            props: &[
                ("name", Value::Text("palettes")),
                ("label", Value::Message(Key::ExamplePaginationLabel)),
                // THIRTY-NINE PALETTES, TEN A PAGE, FOUR PAGES. The overview
                // and the last example on this page count the same set, so a
                // reader moving between them is reading one story.
                ("page", Value::Text("1")),
                ("pages", Value::Text("4")),
                ("href", Value::Text("#page-{page}")),
                // BOTH WORDS ARE PASSED AND ONLY ONE CONTROL IS DRAWN, which
                // is the claim. Without `previous_label` the missing backward
                // link proves nothing, because a control with no name is not
                // rendered whatever page it sits on.
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
            ],
            children: &[],
        }],
    ),
    (
        r"Pagination { spread: PaginationSpread::Compact, more_label, previous_label, next_label }",
        &[Node::Component {
            name: "Pagination",
            props: &[
                ("name", Value::Text("palettes-compact")),
                ("label", Value::Message(Key::ExamplePaginationLabel)),
                ("page", Value::Text("12")),
                ("pages", Value::Text("40")),
                ("href", Value::Text("#page-{page}")),
                ("spread", Value::Text("compact")),
                // THE GAP IS A CONTROL HERE. Page 12 of 40 hides nine pages
                // behind the first `…` and twenty-five behind the second, and
                // `more_label` is what lets a reader reach one of them in a
                // press. With the script blocked both are the inert character
                // they were, which is what `menu.js` reveals the button from.
                ("more_label", Value::Message(Key::ExamplePaginationMore)),
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
            ],
            children: &[],
        }],
    ),
    (
        r"Pagination { spread: PaginationSpread::Wide, size: PaginationSize::Small }",
        &[Node::Component {
            name: "Pagination",
            props: &[
                ("name", Value::Text("palettes-wide")),
                ("label", Value::Message(Key::ExamplePaginationLabel)),
                ("page", Value::Text("20")),
                ("pages", Value::Text("40")),
                ("href", Value::Text("#page-{page}")),
                ("spread", Value::Text("wide")),
                ("size", Value::Text("small")),
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
            ],
            children: &[],
        }],
    ),
    (
        r"At the last page, with a summary",
        &[Node::Component {
            name: "Pagination",
            props: &[
                ("name", Value::Text("palettes-last")),
                ("label", Value::Message(Key::ExamplePaginationLabel)),
                // A CONTROL WHOSE NUMBERS DISAGREE WITH ITS OWN SENTENCE IS
                // ONE NOBODY TRUSTS. Page 9 of 9 under *showing 21 to 30 of
                // 39* is arithmetic a reader can do and the page cannot.
                ("page", Value::Text("4")),
                ("pages", Value::Text("4")),
                ("href", Value::Text("#page-{page}")),
                ("size", Value::Text("large")),
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
                ("summary", Value::Message(Key::ExamplePaginationSummaryLast)),
            ],
            children: &[],
        }],
    ),
];
