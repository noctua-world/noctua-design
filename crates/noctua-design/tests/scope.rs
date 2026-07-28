//! What a `ThemeScope` actually renders.
//!
//! The theme crate's own tests prove the *resolver* — which values a scope
//! settles on. These prove the **markup**, which is a different thing and the
//! one that has silently broken in this fleet before: a resolver can be
//! perfectly right while the component stamps the answer onto two elements, or
//! stamps a word no stylesheet understands, and every token still resolves to
//! something so nothing looks broken.

use dioxus::prelude::*;
use noctua_design::ThemeScope;
use noctua_design::model::{Accent, Mode, Palette, Saturation};

/// Renders a component tree to HTML, the way the documentation site does.
fn render(app: fn() -> Element) -> String {
    let mut dom = VirtualDom::new(app);
    dom.rebuild_in_place();
    dioxus_ssr::render(&dom)
}

/// The compound selector `[data-palette="x"][data-theme="dark"]` matches only
/// when both halves are on the same element. A wrapper-plus-child split falls
/// through to the default palette's unanchored `[data-theme="dark"]` and paints
/// the wrong colours without erroring.
#[test]
fn every_attribute_lands_on_one_element() {
    fn app() -> Element {
        rsx! {
            ThemeScope {
                mode: Mode::Dark,
                accent: Accent::Amber,
                saturation: Saturation::Vivid,
                palette: Palette::AmberVivid,
                "content"
            }
        }
    }

    let html = render(app);
    let opening = html.split('>').next().expect("an opening tag");
    assert!(opening.contains("data-palette=\"amber-vivid\""), "{html}");
    assert!(opening.contains("data-theme=\"dark\""), "{html}");
    assert!(opening.contains("data-noctua-scope"), "{html}");
    // One element carries them, so there is exactly one opening `<div`.
    assert_eq!(html.matches("<div").count(), 1, "{html}");
}

/// `system` is the absence of `data-theme`, not a value of it. Stamping the
/// word would leave the page light while the OS asked for dark.
#[test]
fn following_the_system_stamps_no_mode() {
    fn app() -> Element {
        rsx! {
            ThemeScope { mode: Mode::System, palette: Palette::JadeSober, "content" }
        }
    }

    let html = render(app);
    assert!(!html.contains("data-theme"), "{html}");
    assert!(html.contains("data-palette=\"jade-sober\""), "{html}");
}

/// Changing only the mode must still stamp the palette, or the compound
/// selector misses.
#[test]
fn a_mode_change_alone_still_stamps_the_inherited_palette() {
    fn app() -> Element {
        rsx! {
            ThemeScope { mode: Mode::Dark, "content" }
        }
    }

    let html = render(app);
    assert!(html.contains("data-theme=\"dark\""), "{html}");
    assert!(
        html.contains(&format!("data-palette=\"{}\"", Palette::DEFAULT.as_str())),
        "a scope stamped a mode without a palette: {html}"
    );
}

/// Re-resolving 1,767 declarations for a subtree that asked for the theme it
/// already had is pure cost, and a wrapper `<div>` is a box every layout has to
/// step through.
#[test]
fn a_scope_that_changes_nothing_renders_no_element() {
    fn app() -> Element {
        rsx! {
            ThemeScope { mode: Mode::System, palette: Palette::DEFAULT, "content" }
        }
    }

    let html = render(app);
    assert_eq!(html, "content", "{html}");
}

/// A nested scope resolves against its parent scope, not against the document.
#[test]
fn a_nested_scope_inherits_from_the_scope_around_it() {
    fn app() -> Element {
        rsx! {
            ThemeScope {
                palette: Palette::TealVivid,
                mode: Mode::Dark,
                ThemeScope { mode: Mode::Light, "inner" }
            }
        }
    }

    let html = render(app);
    // The inner scope changed only the mode, so it must carry the *outer*
    // scope's palette — the document default would be wrong.
    let inner = html.rsplit("<div").next().expect("an inner div");
    assert!(inner.contains("data-theme=\"light\""), "{html}");
    assert!(inner.contains("data-palette=\"teal-vivid\""), "{html}");
}

/// The class lands on the scope element, so a caller can lay it out without a
/// second wrapper.
#[test]
fn the_class_reaches_the_scope_element() {
    fn app() -> Element {
        rsx! {
            ThemeScope { palette: Palette::RoseVivid, class: "nd-demo", "content" }
        }
    }

    assert!(render(app).contains("class=\"nd-demo\""));
}
