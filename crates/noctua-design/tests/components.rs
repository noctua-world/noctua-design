//! What the generated components actually render.
//!
//! The emitter's own tests prove both languages *declare* the same props. These
//! prove the Dioxus side produces the markup the spec describes — the element,
//! the class, the attributes, and the defaults applied by the component rather
//! than by the caller.
//!
//! The React side is checked against the same expectations by the browser
//! suite, which renders both and compares the DOM.

use dioxus::prelude::*;
use noctua_design::components::*;

fn render(app: fn() -> Element) -> String {
    let mut dom = VirtualDom::new(app);
    dom.rebuild_in_place();
    dioxus_ssr::render(&dom)
}

/// A heading's outline level and its visual size are separate decisions, and
/// this is the assertion that keeps them separate: level 2 at display size.
#[test]
fn a_heading_separates_its_level_from_its_size() {
    fn app() -> Element {
        rsx! {
            Heading { level: HeadingLevel::_2, size: HeadingSize::Display, "Title" }
        }
    }
    let html = render(app);
    assert!(html.starts_with("<h2"), "{html}");
    assert!(html.contains("data-nd-size=\"display\""), "{html}");
}

/// A prop the caller omits gets its default from the COMPONENT, which is what
/// stops the two implementations disagreeing about it.
#[test]
fn a_default_comes_from_the_component() {
    fn app() -> Element {
        rsx! { Text { "plain" } }
    }
    let html = render(app);
    assert!(html.contains("data-nd-size=\"body\""), "{html}");
    assert!(html.contains("data-nd-weight=\"regular\""), "{html}");
}

#[test]
fn every_component_carries_its_class_and_the_callers() {
    fn app() -> Element {
        rsx! { Paragraph { class: "extra", "text" } }
    }
    let html = render(app);
    assert!(html.contains("nd-paragraph"), "{html}");
    assert!(html.contains("extra"), "{html}");
}

/// The measure lives on the prose, never on a layout container.
#[test]
fn prose_owns_the_measure() {
    fn app() -> Element {
        rsx! { Prose { Paragraph { "long text" } } }
    }
    let html = render(app);
    assert!(html.contains("nd-prose-block"), "{html}");
    assert!(html.contains("data-nd-measure=\"default\""), "{html}");
}

/// A truncated value stays whole in the accessible tree.
#[test]
fn truncate_keeps_the_full_value() {
    fn app() -> Element {
        rsx! {
            Truncate { lines: TruncateLines::_2, title: "the whole thing", "the whole thing" }
        }
    }
    let html = render(app);
    assert!(html.contains("title=\"the whole thing\""), "{html}");
    assert!(html.contains("data-nd-lines=\"2\""), "{html}");
}

/// THE ELEMENT, not an attribute.
///
/// This test existed under this name and asserted `data-nd-kind="ordered"` —
/// which the component did emit, on a `<ul>`. An ordered list that is a `<ul>`
/// has no numbers and tells a screen reader the wrong thing, and the test that
/// was meant to catch that was checking the attribute the bug left behind.
#[test]
fn a_list_changes_its_element_with_its_kind() {
    fn app() -> Element {
        rsx! { List { kind: ListKind::Ordered, ListItem { "one" } } }
    }
    let html = render(app);
    assert!(html.contains("<ol"), "{html}");
    assert!(!html.contains("<ul"), "{html}");
    assert!(
        !html.contains("data-nd-kind"),
        "the element already says it: {html}"
    );
}

#[test]
fn a_description_list_is_a_dl_of_dt_and_dd() {
    fn app() -> Element {
        rsx! {
            List { kind: ListKind::Description,
                ListItem { term: "Noctua", "an owl" }
            }
        }
    }
    let html = render(app);
    assert!(html.contains("<dl"), "{html}");
    assert!(html.contains("<dt"), "{html}");
    assert!(html.contains("<dd"), "{html}");
    // The pair has to be a DIRECT child of the <dl>, or the association a
    // screen reader reads is broken by whatever sits between them.
    assert!(!html.contains("<li"), "{html}");
}

/// A heading's level is document structure, and it was inert.
///
/// Every `Heading` rendered `<h2>` whatever `level` said: the prop had no
/// attribute and no element to switch, so it was accepted, documented, and
/// dropped. A screen-reader user navigating by heading fell through a flat
/// outline and nothing reported it.
#[test]
fn a_heading_renders_the_level_it_is_given() {
    fn first() -> Element {
        rsx! { Heading { level: HeadingLevel::_1, "a heading" } }
    }
    fn third() -> Element {
        rsx! { Heading { level: HeadingLevel::_3, "a heading" } }
    }
    fn sixth() -> Element {
        rsx! { Heading { level: HeadingLevel::_6, "a heading" } }
    }
    for (app, element) in [
        (first as fn() -> Element, "<h1"),
        (third, "<h3"),
        (sixth, "<h6"),
    ] {
        let html = render(app);
        assert!(html.contains(element), "expected {element}: {html}");
    }
}

/// The level and the size are independent, which is the whole reason there are
/// two props: an `<h3>` that has to look like an `<h1>` must not become one.
#[test]
fn a_heading_sizes_itself_apart_from_its_level() {
    fn app() -> Element {
        rsx! {
            Heading { level: HeadingLevel::_3, size: HeadingSize::Display, "loud but third" }
        }
    }
    let html = render(app);
    assert!(html.contains("<h3"), "{html}");
    assert!(html.contains("data-nd-size=\"display\""), "{html}");
}

/// A strikethrough is not announced, so the word has to be.
#[test]
fn an_edit_announces_itself_when_it_is_given_the_word() {
    fn labelled() -> Element {
        rsx! { Del { label: "removed", "the old text" } }
    }
    fn plain() -> Element {
        rsx! { Ins { "the new text" } }
    }

    let html = render(labelled);
    assert!(html.contains("nd-visually-hidden"), "{html}");
    assert!(html.contains("removed"), "{html}");

    let bare = render(plain);
    assert!(!bare.contains("nd-visually-hidden"), "{bare}");
}

/// `cite` is a URL for machines. The line a reader sees is `attribution`.
#[test]
fn a_blockquote_can_name_its_source() {
    fn app() -> Element {
        rsx! {
            Blockquote {
                attribution: "A. Person",
                cite: "https://example.invalid/x",
                "said a thing"
            }
        }
    }
    let html = render(app);
    assert!(html.contains("<footer"), "{html}");
    assert!(html.contains("<cite"), "{html}");
    assert!(html.contains("A. Person"), "{html}");
    assert!(
        html.contains("cite=\"https://example.invalid/x\""),
        "{html}"
    );
}

/// Every enum's `ALL` is what the documentation page iterates, so an empty one
/// would silently document nothing.
#[test]
fn every_enum_lists_its_values() {
    assert_eq!(HeadingLevel::ALL.len(), 6);
    assert_eq!(HeadingSize::ALL.len(), 6);
    assert_eq!(TextWeight::ALL.len(), 4);
    assert_eq!(ListKind::ALL.len(), 3);
    assert_eq!(TruncateLines::ALL.len(), 4);
    assert_eq!(HeadingLevel::_1.as_str(), "1");
    assert_eq!(ListKind::Description.as_str(), "description");
}
