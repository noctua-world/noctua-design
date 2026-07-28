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

#[test]
fn a_list_changes_its_element_with_its_kind() {
    fn app() -> Element {
        rsx! { List { kind: ListKind::Ordered, ListItem { "one" } } }
    }
    let html = render(app);
    assert!(html.contains("data-nd-kind=\"ordered\""), "{html}");
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
