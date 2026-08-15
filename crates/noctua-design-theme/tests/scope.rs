//! The scope resolver, tested against the three ways it can be silently wrong.
//!
//! These run without a browser and without either UI framework, because the
//! logic they cover is the same in both packages — it is generated once and
//! called by two thin wrappers. A bug here is a bug in both implementations at
//! the same time, which is exactly why it is worth testing here rather than
//! twice in two conformance suites.

use noctua_design_theme::{Glass, Mode, Palette, Radius, Resolved, ScopeRequest, scope_attributes};

/// A page on a non-default palette, forced dark.
fn page() -> Resolved {
    Resolved {
        palette: Palette::BlueVivid,
        mode: Mode::Dark,
        ..Resolved::default()
    }
}

/// **Gotcha 1.** Every attribute belongs to one element.
///
/// The color system's forced-mode selector is compound and same-element:
/// `[data-palette="x"][data-theme="dark"]`. Split across a wrapper and a child
/// it matches nothing, and the child falls through to the default palette's
/// unanchored `[data-theme="dark"]` — silently painting the wrong palette.
///
/// The resolver returns one struct precisely so there is no shape in which the
/// attributes could be split.
#[test]
fn every_attribute_belongs_to_one_element() {
    let attributes = scope_attributes(
        ScopeRequest {
            palette: Some(Palette::AmberVivid),
            mode: Some(Mode::Light),
            ..ScopeRequest::default()
        },
        page(),
    )
    .expect("a scope that changes something emits attributes");

    assert_eq!(attributes.palette, "amber-vivid");
    assert_eq!(attributes.mode, Some("light"));
    assert_eq!(attributes.scope, "data-noctua-scope");
}

/// **Gotcha 2.** A mode is never stamped without a palette.
///
/// A scope that asks only to flip light/dark still needs the palette on the
/// same element, or the compound selector misses and the subtree gets the
/// *default* palette's dark values rather than its own palette's.
#[test]
fn a_mode_change_alone_still_stamps_the_inherited_palette() {
    let attributes = scope_attributes(
        ScopeRequest {
            mode: Some(Mode::Light),
            ..ScopeRequest::default()
        },
        page(),
    )
    .expect("changing the mode is a change");

    assert_eq!(
        attributes.palette, "blue-vivid",
        "the inherited palette must travel with the mode, or the compound \
         selector matches nothing and the subtree paints the default palette"
    );
    assert_eq!(attributes.mode, Some("light"));
}

/// **Gotcha 3.** `system` stamps no attribute at all.
///
/// `data-theme="system"` is a value no stylesheet understands. The cascade
/// resolves the system preference on its own, and stamping anything would
/// override it with nothing.
#[test]
fn following_the_system_stamps_no_mode() {
    let attributes = scope_attributes(
        ScopeRequest {
            palette: Some(Palette::JadeSober),
            mode: Some(Mode::System),
            ..ScopeRequest::default()
        },
        page(),
    )
    .expect("a palette change is a change");

    assert_eq!(attributes.palette, "jade-sober");
    assert_eq!(
        attributes.mode, None,
        "system is the absence of the attribute, not a value of it"
    );
}

/// A scope that changes nothing costs nothing.
///
/// The semantic contract is 1,767 declarations. Re-resolving them for a subtree
/// that asked for exactly what it already had is pure cost, and the marker
/// would also make the element look themed to anything inspecting the DOM.
#[test]
fn a_scope_that_matches_its_surroundings_emits_nothing() {
    assert!(
        scope_attributes(ScopeRequest::default(), page()).is_none(),
        "an all-inherit scope is a no-op"
    );
    assert!(
        scope_attributes(
            ScopeRequest {
                palette: Some(Palette::BlueVivid),
                mode: Some(Mode::Dark),
                ..ScopeRequest::default()
            },
            page(),
        )
        .is_none(),
        "asking for exactly what you inherit is also a no-op"
    );
}

/// Inheritance copies the resolved value, not the request.
#[test]
fn an_inheriting_field_copies_what_the_ancestor_resolved_to() {
    let resolved = ScopeRequest {
        glass: Some(Glass::Full),
        ..ScopeRequest::default()
    }
    .resolve(page());

    assert_eq!(resolved.palette, Palette::BlueVivid);
    assert_eq!(resolved.mode, Mode::Dark);
    assert_eq!(resolved.glass, Glass::Full);
}

/// Scopes nest, and the inner one resolves against the outer one's result —
/// not against the page.
#[test]
fn a_nested_scope_resolves_against_its_parent_scope() {
    let outer = ScopeRequest {
        palette: Some(Palette::AmberVivid),
        mode: Some(Mode::Light),
        radius: Some(Radius::Round),
        ..ScopeRequest::default()
    }
    .resolve(page());

    let inner = scope_attributes(
        ScopeRequest {
            palette: Some(Palette::OchreBalanced),
            ..ScopeRequest::default()
        },
        outer,
    )
    .expect("a palette change");

    assert_eq!(inner.palette, "ochre-balanced");
    assert_eq!(
        inner.mode,
        Some("light"),
        "the inner scope inherits the OUTER scope's mode, not the page's"
    );
    assert_eq!(
        inner.radius, "round",
        "and the outer scope's radius, which it did not ask to change"
    );
}

/// The default palette must be scopable like any other.
///
/// It was not, until `noctua-colors` 0.2.1: it was emitted `:root`-only, so
/// `data-palette="ochre-balanced"` matched nothing and the element inherited
/// the page. Thirty-eight of thirty-nine worked.
#[test]
fn the_default_palette_can_be_scoped() {
    let attributes = scope_attributes(
        ScopeRequest {
            palette: Some(Palette::OchreBalanced),
            ..ScopeRequest::default()
        },
        page(),
    )
    .expect("switching to the default from another palette is a change");

    assert_eq!(attributes.palette, "ochre-balanced");
    assert_eq!(
        noctua_design_theme::stylesheet_href(Palette::OchreBalanced),
        "tokens/css/theme-ochre-balanced.css",
        "the default needs a scopable stylesheet like every other palette"
    );
}

/// Every palette has an href, and no two share one.
#[test]
fn every_palette_has_its_own_stylesheet() {
    let mut seen: Vec<&str> = Vec::new();
    for palette in Palette::ALL {
        let href = noctua_design_theme::stylesheet_href(palette);
        assert!(
            std::path::Path::new(href)
                .extension()
                .is_some_and(|ext| ext.eq_ignore_ascii_case("css")),
            "{href}"
        );
        assert!(
            href.contains(palette.as_str()),
            "{href} is not {}",
            palette.as_str()
        );
        assert!(!seen.contains(&href), "two palettes share {href}");
        seen.push(href);
    }
    assert_eq!(seen.len(), 39);
}

/// A stored value is never trusted: it reaches an attribute and an `href`.
#[test]
fn an_unrecognised_stored_value_is_rejected() {
    assert_eq!(Palette::parse("blue-vivid"), Some(Palette::BlueVivid));
    for hostile in ["../../etc/passwd", "blue-vivid\"", "", "BLUE-VIVID"] {
        assert_eq!(Palette::parse(hostile), None, "{hostile} was accepted");
    }
    assert_eq!(Mode::parse("system"), Some(Mode::System));
    assert_eq!(Mode::parse("sytem"), None);
}
