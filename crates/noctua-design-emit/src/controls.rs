//! The bodies of the components the contract cannot describe.
//!
//! # Why some bodies are written here
//!
//! `props.rs` generates a component's body from three facts in its spec: the
//! element, the root class, and the attribute each prop becomes. That covers
//! every text component, and it is the parity mechanism — neither
//! implementation names a prop, a default or a value, so neither can disagree
//! with the other about one.
//!
//! It does not cover a control that renders another component inside itself,
//! switches its own element, or wires ARIA attributes that are not a
//! one-to-one mapping of a prop. `ThemeScope` and `Icon` already have emitters
//! of their own for the same reason; this is the third, not a new idea.
//!
//! **The prop surface is still generated from the spec.** Only the body is
//! written, and it is written ONCE — in this file, in both languages, side by
//! side — so the two implementations still cannot drift apart in a review.

use noctua_design_spec::component::Component;

/// Whether this component's body is written here rather than derived.
#[must_use]
pub fn is_custom(component: &Component) -> bool {
    matches!(
        component.meta.name.as_str(),
        "IconButton" | "Link" | "Choice" | "ChoiceOption"
    )
}

/// The Dioxus body, from `let root = …` to the closing brace of `rsx!`.
pub fn dioxus_body(out: &mut String, component: &Component) {
    match component.meta.name.as_str() {
        "IconButton" => icon_button_dioxus(out),
        "Link" => link_dioxus(out),
        "Choice" => choice_dioxus(out),
        "ChoiceOption" => choice_option_dioxus(out),
        other => unreachable!("no custom body for `{other}`"),
    }
}

/// The React body, from the first statement to the `return`.
pub fn react_body(out: &mut String, component: &Component) {
    match component.meta.name.as_str() {
        "IconButton" => icon_button_react(out),
        "Link" => link_react(out),
        "Choice" => choice_react(out),
        "ChoiceOption" => choice_option_react(out),
        other => unreachable!("no custom body for `{other}`"),
    }
}

// --- Link -------------------------------------------------------------------

/// A link, and the mark an external destination needs.
///
/// The affordance was `content: "↗"` — a text character standing in for an
/// icon. It is not one: it takes the reader's font rather than the system's, it
/// is absent from most monospaced faces, and it cannot be found by the gate
/// that keeps every other icon in one place. `NOCTUA.md §9.5` is explicit that
/// a glyph is not an icon.
///
/// The icon is `aria-hidden`: "opens in a new place" belongs to the link's
/// text, not to a decoration a screen reader would read out as a word.
fn link_dioxus(out: &mut String) {
    out.push_str(
        "    let root = format!(\"nd-a {class}\");\n\
         \x20   let root = root.trim().to_owned();\n\
         \n\
         \x20   rsx! {\n\
         \x20       a {\n\
         \x20           class: \"{root}\",\n\
         \x20           href,\n\
         \x20           \"data-nd-external\": if external { \"true\" } else { \"false\" },\n\
         \x20           \"data-nd-tone\": tone.as_str(),\n\
         \x20           {children}\n\
         \x20           if external {\n\
         \x20               Icon { icon: IconName::ExternalLink, size: crate::icon::IconSize::Small }\n\
         \x20           }\n\
         \x20       }\n\
         \x20   }\n",
    );
}

fn link_react(out: &mut String) {
    out.push_str(
        "  const tone_ = tone ?? \"accent\";\n\
         \x20 const root = [\"nd-a\", className].filter(Boolean).join(\" \");\n\
         \x20 return h(\n\
         \x20   \"a\",\n\
         \x20   {\n\
         \x20     className: root,\n\
         \x20     href,\n\
         \x20     \"data-nd-external\": external ? \"true\" : \"false\",\n\
         \x20     \"data-nd-tone\": tone_,\n\
         \x20     ...rest,\n\
         \x20   },\n\
         \x20   children,\n\
         \x20   external ? h(Icon, { icon: \"external-link\", size: \"small\" }) : null,\n\
         \x20 );\n",
    );
}

// --- Choice -----------------------------------------------------------------

/// A radiogroup, and the keyboard contract that makes it one.
///
/// The site had twelve of these written as raw markup, and they were forty-two
/// tab stops: every option was focusable, so a reader crossing the settings
/// panel with Tab pressed it forty-two times. A radio group is ONE tab stop,
/// and the arrows move within it — which is not a nicety, it is what the role
/// promises the moment the markup claims it.
///
/// The behaviour lives in `shell.js`, keyed on `[role="radiogroup"]`, because
/// this package ships no runtime. What the component owes is the markup that
/// behaviour binds to, and the contract in the spec that says what it must do.
fn choice_dioxus(out: &mut String) {
    out.push_str(
        "    let root = format!(\"nd-choice-group {class}\");\n\
         \x20   let root = root.trim().to_owned();\n\
         \n\
         \x20   rsx! {\n\
         \x20       div {\n\
         \x20           class: \"{root}\",\n\
         \x20           role: \"radiogroup\",\n\
         \x20           \"data-nd-setting\": \"{name}\",\n\
         \x20           \"data-nd-size\": size.as_str(),\n\
         \x20           \"aria-orientation\": orientation.as_str(),\n\
         \x20           // One or the other names the group. A radiogroup with\n\
         \x20           // neither announces its role and nothing else.\n\
         \x20           \"aria-label\": label,\n\
         \x20           \"aria-labelledby\": labelledby,\n\
         \x20           {children}\n\
         \x20       }\n\
         \x20   }\n",
    );
}

fn choice_react(out: &mut String) {
    out.push_str(
        "  const size_ = size ?? \"medium\";\n\
         \x20 const orientation_ = orientation ?? \"horizontal\";\n\
         \x20 const root = [\"nd-choice-group\", className].filter(Boolean).join(\" \");\n\
         \x20 return h(\n\
         \x20   \"div\",\n\
         \x20   {\n\
         \x20     className: root,\n\
         \x20     role: \"radiogroup\",\n\
         \x20     \"data-nd-setting\": name,\n\
         \x20     \"data-nd-size\": size_,\n\
         \x20     \"aria-orientation\": orientation_,\n\
         \x20     \"aria-label\": label,\n\
         \x20     \"aria-labelledby\": labelledby,\n\
         \x20     ...rest,\n\
         \x20   },\n\
         \x20   children,\n\
         \x20 );\n",
    );
}

/// One option.
///
/// `aria-checked` is written whatever its value, unlike `aria-pressed` on a
/// button where absence is meaningful: a radio that omits it is a radio whose
/// state is unknown, and a group of those announces nothing chosen.
///
/// The `tabindex` is the roving one — `0` on the chosen option, `-1` on the
/// rest — server-rendered rather than applied by script, so the group is one
/// tab stop before anything runs and does not become forty-two if the script
/// never does.
fn choice_option_dioxus(out: &mut String) {
    out.push_str(
        "    let root = format!(\"nd-choice-option {class}\");\n\
         \x20   let root = root.trim().to_owned();\n\
         \n\
         \x20   rsx! {\n\
         \x20       button {\n\
         \x20           class: \"{root}\",\n\
         \x20           r#type: \"button\",\n\
         \x20           role: \"radio\",\n\
         \x20           \"data-nd-value\": \"{value}\",\n\
         \x20           \"aria-checked\": if selected { \"true\" } else { \"false\" },\n\
         \x20           tabindex: if selected { \"0\" } else { \"-1\" },\n\
         \x20           if let Some(icon) = icon {\n\
         \x20               // The label is the accessible name and the icon is\n\
         \x20               // decoration: an icon-only option that also announced\n\
         \x20               // its icon would say the same thing twice.\n\
         \x20               span { class: \"nd-visually-hidden\", \"{label}\" }\n\
         \x20               Icon { icon }\n\
         \x20           } else {\n\
         \x20               \"{label}\"\n\
         \x20           }\n\
         \x20       }\n\
         \x20   }\n",
    );
}

fn choice_option_react(out: &mut String) {
    out.push_str(
        "  const root = [\"nd-choice-option\", className].filter(Boolean).join(\" \");\n\
         \x20 return h(\n\
         \x20   \"button\",\n\
         \x20   {\n\
         \x20     className: root,\n\
         \x20     type: \"button\",\n\
         \x20     role: \"radio\",\n\
         \x20     \"data-nd-value\": value,\n\
         \x20     // Always written. A radio with no `aria-checked` has no state.\n\
         \x20     \"aria-checked\": selected ? \"true\" : \"false\",\n\
         \x20     tabIndex: selected ? 0 : -1,\n\
         \x20     ...rest,\n\
         \x20   },\n\
         \x20   icon ? h(\"span\", { className: \"nd-visually-hidden\" }, label) : label,\n\
         \x20   icon ? h(Icon, { icon }) : null,\n\
         \x20 );\n",
    );
}

// --- IconButton -------------------------------------------------------------

/// A control whose only content is an icon.
///
/// Its `label` is required by the spec, so the type system refuses a control
/// with no accessible name — which is the reason it exists. The site shipped
/// one: a 44×44 header link with an empty icon and, for a screen reader,
/// nothing but "link".
///
/// The icon inside is deliberately `aria-hidden`. It repeats the label, and a
/// reader hearing both hears the same thing twice.
fn icon_button_dioxus(out: &mut String) {
    out.push_str(
        "    let root = format!(\"nd-icon-button {class}\");\n\
         \x20   let root = root.trim().to_owned();\n\
         \n\
         \x20   // A control that navigates is a link, whatever it looks like. The\n\
         \x20   // middle mouse button, the context menu and a screen reader's list of\n\
         \x20   // links all expect one, and none of them can be given by a handler.\n\
         \x20   if let Some(href) = href {\n\
         \x20       return rsx! {\n\
         \x20           a {\n\
         \x20               class: \"{root}\",\n\
         \x20               href: \"{href}\",\n\
         \x20               \"data-nd-size\": size.as_str(),\n\
         \x20               \"data-nd-tone\": tone.as_str(),\n\
         \x20               \"aria-label\": \"{label}\",\n\
         \x20               title: \"{label}\",\n\
         \x20               \"aria-expanded\": expanded.then_some(\"true\"),\n\
         \x20               \"aria-controls\": controls,\n\
         \x20               rel,\n\
         \x20               Icon { icon }\n\
         \x20           }\n\
         \x20       };\n\
         \x20   }\n\
         \n\
         \x20   rsx! {\n\
         \x20       button {\n\
         \x20           class: \"{root}\",\n\
         \x20           r#type: \"button\",\n\
         \x20           \"data-nd-size\": size.as_str(),\n\
         \x20           \"data-nd-tone\": tone.as_str(),\n\
         \x20           // The name is on the control, never on the icon: the icon is\n\
         \x20           // decoration and repeating the label would say it twice.\n\
         \x20           \"aria-label\": \"{label}\",\n\
         \x20           title: \"{label}\",\n\
         \x20           // Absent rather than \"false\": `aria-pressed=\"false\"` makes a\n\
         \x20           // plain button announce itself as an un-pressed toggle.\n\
         \x20           \"aria-pressed\": pressed.then_some(\"true\"),\n\
         \x20           \"aria-expanded\": expanded.then_some(\"true\"),\n\
         \x20           \"aria-controls\": controls,\n\
         \x20           disabled,\n\
         \x20           Icon { icon }\n\
         \x20       }\n\
         \x20   }\n",
    );
}

fn icon_button_react(out: &mut String) {
    out.push_str(
        "  const size_ = size ?? \"medium\";\n\
         \x20 const tone_ = tone ?? \"quiet\";\n\
         \x20 const root = [\"nd-icon-button\", className].filter(Boolean).join(\" \");\n\
         \x20 const shared = {\n\
         \x20   className: root,\n\
         \x20   \"data-nd-size\": size_,\n\
         \x20   \"data-nd-tone\": tone_,\n\
         \x20   // The name is on the control, never on the icon.\n\
         \x20   \"aria-label\": label,\n\
         \x20   title: label,\n\
         \x20   // Absent rather than false: `aria-expanded=\"false\"` on something that\n\
         \x20   // expands nothing announces a collapsed disclosure that is not there.\n\
         \x20   \"aria-expanded\": expanded ? \"true\" : undefined,\n\
         \x20   \"aria-controls\": controls,\n\
         \x20   ...rest,\n\
         \x20 };\n\
         \x20 const glyph = h(Icon, { icon });\n\
         \n\
         \x20 // A control that navigates is a link, whatever it looks like.\n\
         \x20 if (href) return h(\"a\", { ...shared, href, rel }, glyph);\n\
         \n\
         \x20 return h(\n\
         \x20   \"button\",\n\
         \x20   {\n\
         \x20     ...shared,\n\
         \x20     type: \"button\",\n\
         \x20     \"aria-pressed\": pressed ? \"true\" : undefined,\n\
         \x20     disabled: disabled || undefined,\n\
         \x20   },\n\
         \x20   glyph,\n\
         \x20 );\n",
    );
}

/// Anything a custom body needs imported, in Rust.
#[must_use]
pub fn dioxus_imports(component: &Component) -> String {
    match component.meta.name.as_str() {
        "IconButton" | "Link" | "ChoiceOption" => "use crate::icon::{Icon, IconName};\n".to_owned(),
        _ => String::new(),
    }
}

/// The same, in JavaScript.
#[must_use]
pub fn react_imports(component: &Component) -> String {
    match component.meta.name.as_str() {
        "IconButton" | "Link" | "ChoiceOption" => {
            "import { Icon } from \"../icon.js\";\n".to_owned()
        }
        _ => String::new(),
    }
}

/// The Rust type a `text` prop takes, when it is not a `String`.
///
/// `icon` is an `IconName`, which is the whole reason a name that is not an
/// icon does not compile.
#[must_use]
pub fn dioxus_type(component: &Component, prop: &str) -> Option<&'static str> {
    match (component.meta.name.as_str(), prop) {
        // `Option<IconName>` on `ChoiceOption`, because the prop is optional
        // there and the generator wraps whatever this returns.
        ("IconButton" | "ChoiceOption", "icon") => Some("IconName"),
        _ => None,
    }
}

/// The same, for the TypeScript declaration.
#[must_use]
pub fn typescript_type(component: &Component, prop: &str) -> Option<&'static str> {
    match (component.meta.name.as_str(), prop) {
        ("IconButton" | "ChoiceOption", "icon") => Some("IconName"),
        _ => None,
    }
}

/// Extra imports a `.d.ts` needs.
#[must_use]
pub fn typescript_imports(component: &Component) -> String {
    match component.meta.name.as_str() {
        "IconButton" | "ChoiceOption" => {
            "import type { IconName } from \"@noctua/tokens/icons.js\";\n".to_owned()
        }
        _ => String::new(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn shipped(name: &str) -> Component {
        let root = concat!(env!("CARGO_MANIFEST_DIR"), "/../../spec/components");
        noctua_design_spec::component::load_all(root)
            .expect("the shipped components")
            .into_iter()
            .find(|component| component.meta.name == name)
            .unwrap_or_else(|| panic!("{name} is not a component"))
    }

    /// The reason this component exists, asserted on the contract rather than
    /// on the prose: a control made of nothing but an icon must be nameable
    /// only by naming it.
    #[test]
    fn an_icon_button_cannot_be_built_without_a_name() {
        let component = shipped("IconButton");
        let label = &component.prop["label"];
        assert!(label.required, "`label` is optional, so the bug can return");
        assert!(
            label.default.is_none(),
            "a default name is a name nobody chose"
        );
    }

    /// Both bodies must decide the element the same way, or "behaviourally
    /// identical" is a claim nobody checks.
    #[test]
    fn both_bodies_render_a_link_when_given_an_href() {
        let component = shipped("IconButton");
        let (mut rust, mut js) = (String::new(), String::new());
        dioxus_body(&mut rust, &component);
        react_body(&mut js, &component);
        assert!(rust.contains("a {"), "{rust}");
        assert!(js.contains("h(\"a\""), "{js}");
        assert!(rust.contains("button {"));
        assert!(js.contains("h(\n         \"button\"") || js.contains("\"button\""));
    }

    /// `aria-pressed="false"` makes a plain button announce itself as a toggle
    /// that is not pressed, which is a different control from the one it is.
    #[test]
    fn neither_body_writes_a_false_aria_state() {
        let component = shipped("IconButton");
        let (mut rust, mut js) = (String::new(), String::new());
        dioxus_body(&mut rust, &component);
        react_body(&mut js, &component);
        for body in [&rust, &js] {
            assert!(!body.contains("\"aria-pressed\": \"false\""), "{body}");
            assert!(!body.contains("\"aria-expanded\": \"false\""), "{body}");
        }
    }
}
