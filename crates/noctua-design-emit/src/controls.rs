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

use std::collections::BTreeSet;
use std::fmt::Write as _;

use noctua_design_spec::component::Component;
use noctua_design_spec::icons::Resolved as Icon;

/// The icon import for a body that asks for a size as well as a name.
///
/// A `const` rather than a literal in the arm: three components want the same
/// line, and the arm that holds them is one line long because of it.
const SIZED_ICON: &str =
    "use noctua_design_icons::IconDrawing;\n\nuse crate::components::{Icon, IconSize};\n";

/// A drawing and its name, for a body that needs no size.
///
/// `Icon` is a component now, so it arrives from `crate::components` like every
/// other one. `IconDrawing` stays with the drawings, because it is a fact about
/// the icon set rather than about the component.
const PLAIN_ICON: &str = "use noctua_design_icons::IconDrawing;\n\nuse crate::components::Icon;\n";

/// The same, in JavaScript, where an icon is only ever an icon.
const REACT_ICON: &str = "import { Icon } from \"./icon.js\";\n";

/// A field that is a `<fieldset>`, for the two controls that are groups.
const FIELD_GROUP: &str = "use crate::components::{Field, FieldKind, FieldSize};\n";

/// The same, in JavaScript, where the field draws the group either way.
const REACT_FIELD: &str = "import { Field } from \"./field.js\";\n";

/// The choice between two named things, for the field that asks one.
const REACT_SEGMENTED: &str = concat!(
    "import { Segmented } from \"./segmented.js\";\n",
    "import { SegmentedItem } from \"./segmented_item.js\";\n"
);

/// The one label in the system, for the one component that composes it.
///
/// `Field` drew its own `<label>`, its own `<legend>` and its own required
/// marker. That was the second drawing of a thing that now has a component of
/// its own, so `Field` renders `Label` and draws none of the three.
const LABEL_IMPORT: &str = "use crate::components::{Label, LabelKind, LabelSize};\n";

/// Whether this component's body is written here rather than derived.
#[must_use]
pub fn is_custom(component: &Component) -> bool {
    matches!(
        component.meta.name.as_str(),
        "Button"
            | "Swatch"
            | "Switch"
            | "Segmented"
            | "SegmentedItem"
            | "Link"
            | "Disclosure"
            | "Search"
            | "SearchGroup"
            | "SearchResult"
            | "CodeBlock"
            | "List"
            | "ListItem"
            | "Blockquote"
            | "Del"
            | "Ins"
            | "Checkbox"
            | "Radio"
            | "Fieldset"
            | "Select"
            | "SelectGroup"
            | "SelectOption"
            | "Table"
            | "TableColumn"
            | "TableSection"
            | "TableRow"
            | "TableCell"
            | "TableHeaderCell"
            | "Info"
            | "Menu"
            | "MenuItem"
            | "Meter"
            | "Progress"
            | "Pagination"
            | "Tag"
            | "Badge"
            | "Avatar"
            | "AvatarGroup"
            | "TagPicker"
            | "ColorPicker"
            | "Field"
            | "Input"
            | "Textarea"
            | "NumberInput"
            | "Slider"
            | "FileInput"
            | "TimeInput"
            | "Carousel"
            | "CarouselSlide"
            | "ErrorSummary"
            | "ErrorSummaryItem"
            | "Tree"
            | "TreeItem"
            | "Lineage"
            | "LineageFigure"
            | "DateInput"
            | "Divider"
            | "Label"
            | "Image"
            | "MessageBar"
            | "Icon"
            | "Rating"
            | "OtpField"
            | "Tabs"
            | "Tab"
            | "TabPanel"
            | "Dialog"
            | "Popover"
            | "Tooltip"
            | "Toast"
    )
}

/// The Dioxus body, from `let root = …` to the closing brace of `rsx!`.
pub fn dioxus_body(out: &mut String, component: &Component) {
    match component.meta.name.as_str() {
        "Button" => button_dioxus(out),
        "Lineage" => out.push_str(LINEAGE_DIOXUS),
        "LineageFigure" => out.push_str(LINEAGE_FIGURE_DIOXUS),
        "DateInput" => out.push_str(DATE_INPUT_DIOXUS),
        "Swatch" => swatch_dioxus(out),
        "Switch" => switch_dioxus(out),
        "Segmented" => segmented_dioxus(out),
        "SegmentedItem" => segmented_item_dioxus(out),
        "Link" => link_dioxus(out),
        "Disclosure" => disclosure_dioxus(out),
        "Search" => out.push_str(SEARCH_DIOXUS),
        "SearchGroup" => out.push_str(SEARCH_GROUP_DIOXUS),
        "SearchResult" => out.push_str(SEARCH_RESULT_DIOXUS),
        "CodeBlock" => out.push_str(CODE_BLOCK_DIOXUS),
        "List" => list_dioxus(out),
        "ListItem" => list_item_dioxus(out),
        "Blockquote" => blockquote_dioxus(out),
        "Del" => edit_dioxus(out, "del", "nd-del"),
        "Ins" => edit_dioxus(out, "ins", "nd-ins"),
        "Checkbox" => choice_dioxus(out, CHECKBOX),
        "Radio" => choice_dioxus(out, RADIO),
        "Fieldset" => fieldset_dioxus(out),
        "Table" => out.push_str(TABLE_DIOXUS),
        "TableColumn" => out.push_str(TABLE_COLUMN_DIOXUS),
        "TableSection" => out.push_str(TABLE_SECTION_DIOXUS),
        "TableRow" => out.push_str(TABLE_ROW_DIOXUS),
        "TableCell" => out.push_str(TABLE_CELL_DIOXUS),
        "TableHeaderCell" => out.push_str(TABLE_HEADER_CELL_DIOXUS),
        "Info" => out.push_str(INFO_DIOXUS),
        "Menu" => out.push_str(MENU_DIOXUS),
        "MenuItem" => out.push_str(MENU_ITEM_DIOXUS),
        "Meter" => out.push_str(METER_DIOXUS),
        "Progress" => out.push_str(PROGRESS_DIOXUS),
        "Pagination" => out.push_str(PAGINATION_DIOXUS),
        "Select" => out.push_str(SELECT_DIOXUS),
        "SelectGroup" => out.push_str(SELECT_GROUP_DIOXUS),
        "SelectOption" => out.push_str(SELECT_OPTION_DIOXUS),
        "Tag" => out.push_str(TAG_DIOXUS),
        "Badge" => out.push_str(BADGE_DIOXUS),
        "Avatar" => out.push_str(AVATAR_DIOXUS),
        "AvatarGroup" => out.push_str(AVATAR_GROUP_DIOXUS),
        "TagPicker" => out.push_str(TAG_PICKER_DIOXUS),
        "ColorPicker" => out.push_str(COLOR_PICKER_DIOXUS),
        "Field" => out.push_str(FIELD_DIOXUS),
        "Input" => out.push_str(INPUT_DIOXUS),
        "Textarea" => out.push_str(TEXTAREA_DIOXUS),
        "NumberInput" => out.push_str(NUMBER_INPUT_DIOXUS),
        "Slider" => out.push_str(SLIDER_DIOXUS),
        "FileInput" => out.push_str(FILE_INPUT_DIOXUS),
        "TimeInput" => out.push_str(TIME_INPUT_DIOXUS),
        "Carousel" => out.push_str(CAROUSEL_DIOXUS),
        "CarouselSlide" => out.push_str(CAROUSEL_SLIDE_DIOXUS),
        "ErrorSummary" => out.push_str(ERROR_SUMMARY_DIOXUS),
        "ErrorSummaryItem" => out.push_str(ERROR_SUMMARY_ITEM_DIOXUS),
        "Tree" => out.push_str(TREE_DIOXUS),
        "TreeItem" => out.push_str(TREE_ITEM_DIOXUS),
        "Divider" => out.push_str(DIVIDER_DIOXUS),
        "Label" => out.push_str(LABEL_DIOXUS),
        "Image" => out.push_str(IMAGE_DIOXUS),
        "MessageBar" => out.push_str(MESSAGE_BAR_DIOXUS),
        "Icon" => out.push_str(ICON_DIOXUS),
        "Rating" => out.push_str(RATING_DIOXUS),
        "OtpField" => out.push_str(OTP_FIELD_DIOXUS),
        "Tabs" => out.push_str(TABS_DIOXUS),
        "Tab" => out.push_str(TAB_DIOXUS),
        "TabPanel" => out.push_str(TAB_PANEL_DIOXUS),
        "Dialog" => out.push_str(DIALOG_DIOXUS),
        "Popover" => out.push_str(POPOVER_DIOXUS),
        "Tooltip" => out.push_str(TOOLTIP_DIOXUS),
        "Toast" => out.push_str(TOAST_DIOXUS),
        other => unreachable!("no custom body for `{other}`"),
    }
}

/// The React body, from the first statement to the `return`.
pub fn react_body(out: &mut String, component: &Component) {
    match component.meta.name.as_str() {
        "Button" => button_react(out),
        "Lineage" => out.push_str(LINEAGE_REACT),
        "LineageFigure" => out.push_str(LINEAGE_FIGURE_REACT),
        "DateInput" => out.push_str(DATE_INPUT_REACT),
        "Swatch" => swatch_react(out),
        "Switch" => switch_react(out),
        "Segmented" => segmented_react(out),
        "SegmentedItem" => segmented_item_react(out),
        "Link" => link_react(out),
        "Disclosure" => disclosure_react(out),
        "Search" => out.push_str(SEARCH_REACT),
        "SearchGroup" => out.push_str(SEARCH_GROUP_REACT),
        "SearchResult" => out.push_str(SEARCH_RESULT_REACT),
        "CodeBlock" => out.push_str(CODE_BLOCK_REACT),
        "List" => list_react(out),
        "ListItem" => list_item_react(out),
        "Blockquote" => blockquote_react(out),
        "Del" => edit_react(out, "del", "nd-del"),
        "Ins" => edit_react(out, "ins", "nd-ins"),
        "Checkbox" => choice_react(out, CHECKBOX),
        "Radio" => choice_react(out, RADIO),
        "Fieldset" => fieldset_react(out),
        "Table" => out.push_str(TABLE_REACT),
        "TableColumn" => out.push_str(TABLE_COLUMN_REACT),
        "TableSection" => out.push_str(TABLE_SECTION_REACT),
        "TableRow" => out.push_str(TABLE_ROW_REACT),
        "TableCell" => out.push_str(TABLE_CELL_REACT),
        "TableHeaderCell" => out.push_str(TABLE_HEADER_CELL_REACT),
        "Info" => out.push_str(INFO_REACT),
        "Menu" => out.push_str(MENU_REACT),
        "MenuItem" => out.push_str(MENU_ITEM_REACT),
        "Meter" => out.push_str(METER_REACT),
        "Progress" => out.push_str(PROGRESS_REACT),
        "Pagination" => out.push_str(PAGINATION_REACT),
        "Select" => out.push_str(SELECT_REACT),
        "SelectGroup" => out.push_str(SELECT_GROUP_REACT),
        "SelectOption" => out.push_str(SELECT_OPTION_REACT),
        "Tag" => out.push_str(TAG_REACT),
        "Badge" => out.push_str(BADGE_REACT),
        "Avatar" => out.push_str(AVATAR_REACT),
        "AvatarGroup" => out.push_str(AVATAR_GROUP_REACT),
        "TagPicker" => out.push_str(TAG_PICKER_REACT),
        "ColorPicker" => out.push_str(COLOR_PICKER_REACT),
        "Field" => out.push_str(FIELD_REACT),
        "Input" => out.push_str(INPUT_REACT),
        "Textarea" => out.push_str(TEXTAREA_REACT),
        "NumberInput" => out.push_str(NUMBER_INPUT_REACT),
        "Slider" => out.push_str(SLIDER_REACT),
        "FileInput" => out.push_str(FILE_INPUT_REACT),
        "TimeInput" => out.push_str(TIME_INPUT_REACT),
        "Carousel" => out.push_str(CAROUSEL_REACT),
        "CarouselSlide" => out.push_str(CAROUSEL_SLIDE_REACT),
        "ErrorSummary" => out.push_str(ERROR_SUMMARY_REACT),
        "ErrorSummaryItem" => out.push_str(ERROR_SUMMARY_ITEM_REACT),
        "Tree" => out.push_str(TREE_REACT),
        "TreeItem" => out.push_str(TREE_ITEM_REACT),
        "Divider" => out.push_str(DIVIDER_REACT),
        "Label" => out.push_str(LABEL_REACT),
        "Image" => out.push_str(IMAGE_REACT),
        "MessageBar" => out.push_str(MESSAGE_BAR_REACT),
        "Icon" => out.push_str(ICON_REACT),
        "Rating" => out.push_str(RATING_REACT),
        "OtpField" => out.push_str(OTP_FIELD_REACT),
        "Tabs" => out.push_str(TABS_REACT),
        "Tab" => out.push_str(TAB_REACT),
        "TabPanel" => out.push_str(TAB_PANEL_REACT),
        "Dialog" => out.push_str(DIALOG_REACT),
        "Popover" => out.push_str(POPOVER_REACT),
        "Tooltip" => out.push_str(TOOLTIP_REACT),
        "Toast" => out.push_str(TOAST_REACT),
        other => unreachable!("no custom body for `{other}`"),
    }
}

// --- Button -----------------------------------------------------------------

/// The control for an action, in every shape an interface needs.
///
/// # Why the accessible name is not written when the text is visible
///
/// `aria-label` overrides the content. A button whose visible text says one
/// thing and whose label says another announces the label, and the two drift
/// the moment either is edited — which is why this writes it **only** for the
/// icon-only form, where there is no visible text to compute a name from. The
/// `label` prop is required either way, so a nameless button is not a thing a
/// caller can construct.
///
/// # Why `rel` defaults to something rather than to nothing
///
/// A `target` that opens a new browsing context hands the opened page a handle
/// on this one unless `rel` says otherwise. Browsers now imply `noopener` for
/// `_blank`, and "now" is doing a lot of work in that sentence — the guarantee
/// is a browser version, not a specification anyone can rely on. The caller may
/// still say something else; what they may not do is say nothing. Default deny.
///
/// # Why three ARIA attributes are absent rather than `"false"`
///
/// `aria-pressed`, `aria-expanded` and `aria-current` all mean something by
/// their absence. `aria-pressed="false"` turns a plain button into a toggle
/// that is off, which is a different control from the one it is.
///
/// `menu` IS THE ONE EXCEPTION, AND IT IS THE SAME ARGUMENT. A button that
/// opens a menu is a control that opens something whether or not it is open
/// right now, so `aria-expanded="false"` is the true statement and the absence
/// of the attribute is the false one. So the rule is not "write it only when
/// true" — it is "say what is so": absent where there is nothing to open,
/// `"false"` where there is and it is shut.
fn button_dioxus(out: &mut String) {
    button_dioxus_content(out);
    button_dioxus_elements(out);
}

/// The prelude and the shared content: everything before the element is chosen.
fn button_dioxus_content(out: &mut String) {
    out.push_str(
        "    let root = format!(\"nd-button {class}\");\n\
         \x20   let root = root.trim().to_owned();\n\
         \n\
         \x20   // Only the icon-only form names itself: everywhere else the label IS\n\
         \x20   // the visible text, and an `aria-label` beside it is a second name\n\
         \x20   // that can drift from the first.\n\
         \x20   let named = hide_label.then(|| label.clone());\n\
         \x20   // A NEW BROWSING CONTEXT KEEPS `noopener` WHATEVER ELSE IS ASKED FOR.\n\
         \x20   //\n\
         \x20   // `rel` unset is the easy half. The half worth writing down is a\n\
         \x20   // caller who sets `rel=\"nofollow\"` beside `target`: they said\n\
         \x20   // something, so a defaulting rule keyed on absence steps aside and the\n\
         \x20   // opened page gets a handle on this one. `noreferrer` already implies\n\
         \x20   // `noopener`, so it is left alone; anything else is added to.\n\
         \x20   let rel = if target.is_some() {\n\
         \x20       match rel {\n\
         \x20           None => Some(\"noopener noreferrer\".to_owned()),\n\
         \x20           Some(value) if value.contains(\"noopener\") || value.contains(\"noreferrer\") => {\n\
         \x20               Some(value)\n\
         \x20           }\n\
         \x20           Some(value) => Some(format!(\"{value} noopener\")),\n\
         \x20       }\n\
         \x20   } else {\n\
         \x20       rel\n\
         \x20   };\n\
         \n\
         \x20   // ONE CHEVRON, WHOEVER DREW IT.\n\
         \x20   //\n\
         \x20   // `menu` draws the chevron that makes a menu control look like\n\
         \x20   // one, and it already steps aside for an `icon_end`. A LEADING\n\
         \x20   // `icon` NAMING THE SAME DRAWING WAS NOT COVERED, so a split\n\
         \x20   // button asking for `icon: \"chevron-down\"` beside `menu` drew two\n\
         \x20   // of them side by side. That shipped on two pages of this site.\n\
         \x20   // A button with two chevrons is not a control anybody designs, so\n\
         \x20   // the component refuses to draw the second rather than trusting\n\
         \x20   // every caller to notice.\n\
         \x20   let chevron = menu && icon != Some(IconDrawing::CHEVRON_DOWN);\n\
         \n\
         \x20   let content = rsx! {\n\
         \x20       // A TOGGLE SHIPS BOTH GLYPHS AND LETS THE STATE PICK ONE.\n\
         \x20       //\n\
         \x20       // The alternative is a script that swaps the icon, which means\n\
         \x20       // shipping a drawing to the script — and this package ships no\n\
         \x20       // runtime at all. `CodeBlock` already does this with its copy and\n\
         \x20       // check marks; this is the same trick, generalised.\n\
         \x20       if let Some(icon) = icon {\n\
         \x20           if let Some(icon_alt) = icon_alt {\n\
         \x20               span { class: \"nd-button-swap\",\n\
         \x20                   Icon { icon }\n\
         \x20                   Icon { icon: icon_alt }\n\
         \x20               }\n\
         \x20           } else {\n\
         \x20               Icon { icon }\n\
         \x20           }\n\
         \x20       }\n\
         \x20       if !hide_label {\n\
         \x20           span { class: \"nd-button-text\",\n\
         \x20               span { class: \"nd-button-label\", \"{label}\" }\n\
         \x20               if let Some(detail) = detail {\n\
         \x20                   span { class: \"nd-button-detail\", \"{detail}\" }\n\
         \x20               }\n\
         \x20           }\n\
         \x20       }\n\
         \x20       // The chevron is what makes a menu control look like one, and it\n\
         \x20       // steps aside for an `icon_end` the caller chose deliberately.\n\
         \x20       if let Some(icon_end) = icon_end {\n\
         \x20           Icon { icon: icon_end }\n\
         \x20       } else if chevron {\n\
         \x20           Icon { icon: IconDrawing::CHEVRON_DOWN, size: crate::components::IconSize::Small }\n\
         \x20       }\n\
         \x20   };\n",
    );
}

/// The `<a>` form and the `<button>` form, in that order.
fn button_dioxus_elements(out: &mut String) {
    out.push_str(
        "\n\
         \x20   // A control that navigates is a link, whatever it is painted like. The\n\
         \x20   // middle mouse button, the context menu and a screen reader's list of\n\
         \x20   // links all expect one, and none of them can be given by a handler.\n\
         \x20   if let Some(href) = href {\n\
         \x20       return rsx! {\n\
         \x20           a {\n\
         \x20               class: \"{root}\",\n\
         \x20               href: \"{href}\",\n\
         \x20               rel,\n\
         \x20               target,\n\
         \x20               \"data-nd-variant\": variant.as_str(),\n\
         \x20               \"data-nd-tone\": tone.as_str(),\n\
         \x20               \"data-nd-size\": size.as_str(),\n\
         \x20               \"data-nd-shape\": shape.as_str(),\n\
         \x20               \"data-nd-glass\": if glass { \"true\" } else { \"false\" },\n\
         \x20               \"data-nd-justify\": justify.as_str(),\n\
         \x20               \"data-nd-wrap\": if wrap { \"true\" } else { \"false\" },\n\
         \x20               \"data-nd-fill\": if fill { \"true\" } else { \"false\" },\n\
         \x20               \"data-nd-hide-label\": if hide_label { \"true\" } else { \"false\" },\n\
         \x20               \"data-nd-label-alt\": label_alt,\n\
         \x20               \"data-nd-action\": action,\n\
         \x20               \"aria-label\": named.clone(),\n\
         \x20               title: named,\n\
         \x20               \"aria-haspopup\": menu.then_some(\"menu\"),\n\
         \x20               \"aria-expanded\": expanded.then_some(\"true\").or(menu.then_some(\"false\")),\n\
         \x20               \"aria-controls\": controls,\n\
         \x20               \"aria-current\": current.then_some(\"page\"),\n\
         \x20               {content}\n\
         \x20           }\n\
         \x20       };\n\
         \x20   }\n\
         \n\
         \x20   rsx! {\n\
         \x20       button {\n\
         \x20           class: \"{root}\",\n\
         \x20           // Never left unset. An unset `type` inside a form is `submit`,\n\
         \x20           // which is how a button that meant to do something else submits\n\
         \x20           // the page instead.\n\
         \x20           r#type: submit.as_str(),\n\
         \x20           // WHICH BUTTON WAS PRESSED, AND THE ONLY WAY TO ASK IT WITH NO\n\
         \x20           // SCRIPT. A form with several submits posts the name and value\n\
         \x20           // of the one that was used and of no other, which is how\n\
         \x20           // *remove this tag* is answerable by the server alone. They are\n\
         \x20           // on the button and not on the link, because a link posts\n\
         \x20           // nothing.\n\
         \x20           name,\n\
         \x20           value,\n\
         \x20           \"data-nd-variant\": variant.as_str(),\n\
         \x20           \"data-nd-tone\": tone.as_str(),\n\
         \x20           \"data-nd-size\": size.as_str(),\n\
         \x20           \"data-nd-shape\": shape.as_str(),\n\
         \x20           \"data-nd-glass\": if glass { \"true\" } else { \"false\" },\n\
         \x20           \"data-nd-justify\": justify.as_str(),\n\
         \x20           \"data-nd-wrap\": if wrap { \"true\" } else { \"false\" },\n\
         \x20           \"data-nd-fill\": if fill { \"true\" } else { \"false\" },\n\
         \x20           \"data-nd-hide-label\": if hide_label { \"true\" } else { \"false\" },\n\
         \x20           \"data-nd-label-alt\": label_alt,\n\
         \x20           \"data-nd-action\": action,\n\
         \x20           \"aria-label\": named.clone(),\n\
         \x20           title: named,\n\
         \x20           \"aria-haspopup\": menu.then_some(\"menu\"),\n\
         \x20           // Absent rather than \"false\": all three mean something by\n\
         \x20           // their absence, and \"false\" announces a control this is not.\n\
         \x20           \"aria-pressed\": pressed.then_some(\"true\"),\n\
         \x20           \"aria-expanded\": expanded.then_some(\"true\").or(menu.then_some(\"false\")),\n\
         \x20           \"aria-controls\": controls,\n\
         \x20           \"aria-current\": current.then_some(\"page\"),\n\
         \x20           disabled,\n\
         \x20           // A MENU BUTTON IS REVEALED, NEVER BUILT, which is\n\
         \x20           // `Dialog`'s and `Popover`'s arrangement and what\n\
         \x20           // `menu.toml` has always claimed: \"neither the menu\n\
         \x20           // nor its button renders without a script\". Only the\n\
         \x20           // menu was hidden. The button shipped visible,\n\
         \x20           // focusable and carrying `aria-haspopup=\"menu\"` with\n\
         \x20           // `aria-expanded=\"false\"` beside it — announced to a\n\
         \x20           // screen reader as a menu button that nothing in the\n\
         \x20           // page could open. `dist/menu.js` reveals it, and\n\
         \x20           // only once it has found the menu it names.\n\
         \x20           //\n\
         \x20           // A false bool is omitted, so every other button is\n\
         \x20           // untouched.\n\
         \x20           hidden: menu,\n\
         \x20           {content}\n\
         \x20       }\n\
         \x20   }\n",
    );
}

fn button_react(out: &mut String) {
    button_react_content(out);
    button_react_elements(out);
}

/// The prelude and the shared content: everything before the element is chosen.
fn button_react_content(out: &mut String) {
    out.push_str(
        "  const variant_ = variant ?? \"solid\";\n\
         \x20 const tone_ = tone ?? \"accent\";\n\
         \x20 const size_ = size ?? \"medium\";\n\
         \x20 const shape_ = shape ?? \"rounded\";\n\
         \x20 const justify_ = justify ?? \"center\";\n\
         \x20 const submit_ = submit ?? \"button\";\n\
         \x20 const wrap_ = wrap ?? false;\n\
         \x20 const fill_ = fill ?? false;\n\
         \x20 const hideLabel_ = hideLabel ?? false;\n\
         \x20 const root = [\"nd-button\", className].filter(Boolean).join(\" \");\n\
         \x20 // A new browsing context keeps `noopener` whatever else is asked for.\n\
         \x20 // `noreferrer` already implies it; anything else is added to, so a\n\
         \x20 // caller who wrote `rel=\"nofollow\"` does not silently lose it.\n\
         \x20 const rel_ = !target\n\
         \x20   ? rel\n\
         \x20   : rel == null\n\
         \x20     ? \"noopener noreferrer\"\n\
         \x20     : /noopener|noreferrer/.test(rel)\n\
         \x20       ? rel\n\
         \x20       : `${rel} noopener`;\n\
         \x20 // Only the icon-only form names itself: everywhere else the label IS the\n\
         \x20 // visible text, and an `aria-label` beside it is a second name.\n\
         \x20 const named = hideLabel_ ? label : undefined;\n\
         \x20 // One chevron, whoever drew it: see the Dioxus half.\n\
         \x20 // A drawing rather than a name, so what is compared is the name it\n\
         \x20 // carries. See the Dioxus half, which compares the constants.\n\
         \x20 const chevron = menu && icon?.name !== \"chevron-down\";\n\
         \n\
         \x20 const content = [\n\
         \x20   // A toggle ships BOTH glyphs and lets the state pick one: swapping a\n\
         \x20   // glyph from script means shipping a drawing to the script, and this\n\
         \x20   // package ships no runtime at all.\n\
         \x20   icon\n\
         \x20     ? iconAlt\n\
         \x20       ? h(\n\
         \x20           \"span\",\n\
         \x20           { key: \"i\", className: \"nd-button-swap\" },\n\
         \x20           h(Icon, { key: \"a\", icon }),\n\
         \x20           h(Icon, { key: \"b\", icon: iconAlt }),\n\
         \x20         )\n\
         \x20       : h(Icon, { key: \"i\", icon })\n\
         \x20     : null,\n\
         \x20   hideLabel_\n\
         \x20     ? null\n\
         \x20     : h(\n\
         \x20         \"span\",\n\
         \x20         { key: \"t\", className: \"nd-button-text\" },\n\
         \x20         h(\"span\", { key: \"l\", className: \"nd-button-label\" }, label),\n\
         \x20         detail ? h(\"span\", { key: \"d\", className: \"nd-button-detail\" }, detail) : null,\n\
         \x20       ),\n\
         \x20   // The chevron steps aside for an `iconEnd` the caller chose.\n\
         \x20   iconEnd\n\
         \x20     ? h(Icon, { key: \"e\", icon: iconEnd })\n\
         \x20     : chevron\n\
         \x20       ? h(Icon, { key: \"m\", icon: chevronDown, size: \"small\" })\n\
         \x20       : null,\n\
         \x20 ];\n",
    );
}

/// The attribute bag, the `<a>` form and the `<button>` form, in that order.
fn button_react_elements(out: &mut String) {
    out.push_str(
        "\n\
         \x20 const shared = {\n\
         \x20   className: root,\n\
         \x20     \"data-nd-glass\": String(glass ?? false),\n\
         \x20   \"data-nd-variant\": variant_,\n\
         \x20   \"data-nd-tone\": tone_,\n\
         \x20   \"data-nd-size\": size_,\n\
         \x20   \"data-nd-shape\": shape_,\n\
         \x20   \"data-nd-justify\": justify_,\n\
         \x20   \"data-nd-wrap\": String(wrap_),\n\
         \x20   \"data-nd-fill\": String(fill_),\n\
         \x20   \"data-nd-hide-label\": String(hideLabel_),\n\
         \x20   \"data-nd-label-alt\": labelAlt,\n\
         \x20   \"data-nd-action\": action,\n\
         \x20   \"aria-label\": named,\n\
         \x20   title: named,\n\
         \x20   \"aria-haspopup\": menu ? \"menu\" : undefined,\n\
         \x20   // Absent rather than false: all three mean something by their absence.\n\
         \x20   \"aria-expanded\": expanded ? \"true\" : menu ? \"false\" : undefined,\n\
         \x20   \"aria-controls\": controls,\n\
         \x20   \"aria-current\": current ? \"page\" : undefined,\n\
         \x20   ...rest,\n\
         \x20 };\n\
         \n\
         \x20 // A control that navigates is a link, whatever it is painted like.\n\
         \x20 if (href) return h(\"a\", { ...shared, href, rel: rel_, target }, content);\n\
         \n\
         \x20 return h(\n\
         \x20   \"button\",\n\
         \x20   {\n\
         \x20     ...shared,\n\
         \x20     type: submit_,\n\
         \x20     // See the Dioxus half: which submit was pressed, with no script.\n\
         \x20     name,\n\
         \x20     value,\n\
         \x20     \"aria-pressed\": pressed ? \"true\" : undefined,\n\
         \x20     disabled: disabled || undefined,\n\
         \x20     // See the Dioxus half: a menu button is revealed by\n\
         \x20     // `dist/menu.js`, never built, because a control announcing\n\
         \x20     // `aria-haspopup=\"menu\"` over nothing that can open is worse\n\
         \x20     // than no control at all.\n\
         \x20     hidden: menu || undefined,\n\
         \x20   },\n\
         \x20   content,\n\
         \x20 );\n",
    );
}

// --- Swatch -----------------------------------------------------------------

/// A chip of one palette's accent, and the two lines that make it decoration.
///
/// It is `aria-hidden` unconditionally. Whatever contains it owes the name —
/// `SegmentedItem` requires one — and a chip that announced its palette would
/// say the same word twice. It takes no colour: `data-palette` is what makes
/// `--nc-accent-solid` resolve inside the chip's own scope, which is the same
/// mechanism `ThemeScope` uses and the reason no colour literal is needed.
fn swatch_dioxus(out: &mut String) {
    out.push_str(
        "    let root = format!(\"nd-swatch {class}\");\n\
         \x20   let root = root.trim().to_owned();\n\
         \n\
         \x20   rsx! {\n\
         \x20       span {\n\
         \x20           class: \"{root}\",\n\
         \x20           \"data-palette\": palette,\n\
         \x20           \"data-nd-size\": size.as_str(),\n\
         \x20           \"data-nd-shape\": shape.as_str(),\n\
         \x20           // Decoration, always. The name is on whatever contains it.\n\
         \x20           \"aria-hidden\": \"true\",\n\
         \x20       }\n\
         \x20   }\n",
    );
}

fn swatch_react(out: &mut String) {
    out.push_str(
        "  const size_ = size ?? \"medium\";\n\
         \x20 const shape_ = shape ?? \"pill\";\n\
         \x20 const root = [\"nd-swatch\", className].filter(Boolean).join(\" \");\n\
         \x20 return h(\"span\", {\n\
         \x20   className: root,\n\
         \x20   \"data-palette\": palette,\n\
         \x20   \"data-nd-size\": size_,\n\
         \x20   \"data-nd-shape\": shape_,\n\
         \x20   // Decoration, always. The name is on whatever contains it.\n\
         \x20   \"aria-hidden\": \"true\",\n\
         \x20   ...rest,\n\
         \x20 });\n",
    );
}

// --- ListItem ---------------------------------------------------------------

/// The three list elements, each restating the role it is about to lose.
///
/// # Why a role is written on an element that already has one
///
/// An engine computes a list's role from the layout object, so a `<ul>` that is
/// `display: flex` is not a list any more, and neither is one whose
/// `list-style` is `none`. This stylesheet does both — every list is a flex
/// column and `marker: none` takes the marker off — so a reader was told
/// "three items" by the markup and nothing by the browser. The contract has
/// always said the role is restated; this is the line that restates it.
///
/// A `<dl>` is left alone. `list` is the role of a `<ul>` and an `<ol>`, and a
/// description list is a different structure with a different mapping.
/// `ListItem` restates `listitem` on the `<li>` for the same reason, because a
/// list whose items lost their role announces a length of zero.
fn list_dioxus(out: &mut String) {
    out.push_str(
        "    let root = format!(\"nd-list {class}\");\n\
         \x20   let root = root.trim().to_owned();\n\
         \n\
         \x20   // One arm per value: `rsx!` takes a literal element name, and\n\
         \x20   // this prop chooses the element rather than an attribute.\n\
         \x20   // The levels flag is a fact about the OUTERMOST list, and the\n\
         \x20   // stylesheet counts the lists under it. Written on every kind,\n\
         \x20   // because a description list nests like any other.\n\
         \x20   let levels = if levels { \"true\" } else { \"false\" };\n\
         \n\
         \x20   rsx! {\n\
         \x20       match kind {\n\
         \x20           ListKind::Unordered => rsx! {\n\
         \x20               ul {\n\
         \x20                   class: \"{root}\",\n\
         \x20                   role: \"list\",\n\
         \x20                   \"data-nd-marker\": marker.as_str(),\n\
         \x20                   \"data-nd-spacing\": spacing.as_str(),\n\
         \x20                   \"data-nd-levels\": levels,\n\
         \x20                   {children}\n\
         \x20               }\n\
         \x20           },\n\
         \x20           ListKind::Ordered => rsx! {\n\
         \x20               ol {\n\
         \x20                   class: \"{root}\",\n\
         \x20                   role: \"list\",\n\
         \x20                   \"data-nd-marker\": marker.as_str(),\n\
         \x20                   \"data-nd-spacing\": spacing.as_str(),\n\
         \x20                   \"data-nd-levels\": levels,\n\
         \x20                   {children}\n\
         \x20               }\n\
         \x20           },\n\
         \x20           // A description list keeps the element's own mapping.\n\
         \x20           ListKind::Description => rsx! {\n\
         \x20               dl {\n\
         \x20                   class: \"{root}\",\n\
         \x20                   \"data-nd-marker\": marker.as_str(),\n\
         \x20                   \"data-nd-spacing\": spacing.as_str(),\n\
         \x20                   \"data-nd-levels\": levels,\n\
         \x20                   {children}\n\
         \x20               }\n\
         \x20           },\n\
         \x20       }\n\
         \x20   }\n",
    );
}

fn list_react(out: &mut String) {
    out.push_str(
        "  const kind_ = kind ?? \"unordered\";\n\
         \x20 const root = [\"nd-list\", className].filter(Boolean).join(\" \");\n\
         \x20 // This prop chooses the element rather than an attribute.\n\
         \x20 const element = { unordered: \"ul\", ordered: \"ol\", description: \"dl\" }[kind_] ?? \"ul\";\n\
         \x20 return h(\n\
         \x20   element,\n\
         \x20   {\n\
         \x20     className: root,\n\
         \x20     // See the Dioxus half: flex and `list-style: none` each take\n\
         \x20     // the role away, and a `<dl>` never had this one.\n\
         \x20     role: element === \"dl\" ? undefined : \"list\",\n\
         \x20     \"data-nd-marker\": marker ?? \"default\",\n\
         \x20     \"data-nd-spacing\": spacing ?? \"default\",\n\
         \x20     // See the Dioxus half: the flag belongs to the outermost\n\
         \x20     // list, and the stylesheet counts the lists under it.\n\
         \x20     \"data-nd-levels\": String(levels ?? false),\n\
         \x20     ...rest,\n\
         \x20   },\n\
         \x20   children,\n\
         \x20 );\n",
    );
}

/// One item, or one term and its description.
///
/// Two sibling elements rather than a wrapper, because a `<dt>` and its `<dd>`
/// have to be direct children of the `<dl>`: anything between them breaks the
/// association a screen reader reads.
///
/// The `<li>` restates `listitem`, because the rules that take a list's role
/// away take its items' role with it. A `<dd>` is left alone: it is not a
/// `listitem` and never was.
fn list_item_dioxus(out: &mut String) {
    out.push_str(
        "    let root = format!(\"nd-list-item {class}\");\n\
         \x20   let root = root.trim().to_owned();\n\
         \n\
         \x20   rsx! {\n\
         \x20       if let Some(term) = term {\n\
         \x20           dt { class: \"nd-list-term\", \"{term}\" }\n\
         \x20           dd { class: \"{root}\", {children} }\n\
         \x20       } else {\n\
         \x20           li { class: \"{root}\", role: \"listitem\", {children} }\n\
         \x20       }\n\
         \x20   }\n",
    );
}

fn list_item_react(out: &mut String) {
    out.push_str(
        "  const root = [\"nd-list-item\", className].filter(Boolean).join(\" \");\n\
         \x20 if (term) {\n\
         \x20   // A fragment, so the two land as siblings inside the <dl>.\n\
         \x20   return h(\n\
         \x20     Fragment,\n\
         \x20     null,\n\
         \x20     h(\"dt\", { key: \"t\", className: \"nd-list-term\" }, term),\n\
         \x20     h(\"dd\", { key: \"d\", className: root, ...rest }, children),\n\
         \x20   );\n\
         \x20 }\n\
         \x20 // See the Dioxus half: the role is restated, because the rules\n\
         \x20 // that take the list's role away take its items' with it.\n\
         \x20 return h(\"li\", { className: root, role: \"listitem\", ...rest }, children);\n",
    );
}

// --- Blockquote -------------------------------------------------------------

/// A quotation, and the line that says who said it.
///
/// `cite` is a URL for machines and is not shown; without `attribution` the
/// only way to name a source was to write the `<footer>` by hand, which is a
/// component the system did not have wearing a component it did.
fn blockquote_dioxus(out: &mut String) {
    out.push_str(
        "    let root = format!(\"nd-blockquote {class}\");\n\
         \x20   let root = root.trim().to_owned();\n\
         \n\
         \x20   rsx! {\n\
         \x20       blockquote {\n\
         \x20           class: \"{root}\",\n\
         \x20           \"data-nd-glass\": if glass { \"true\" } else { \"false\" },\n\
         \x20           cite,\n\
         \x20           \"data-nd-tone\": tone.as_str(),\n\
         \x20           {children}\n\
         \x20           if let Some(attribution) = attribution {\n\
         \x20               footer { class: \"nd-blockquote-attribution\",\n\
         \x20                   cite { \"{attribution}\" }\n\
         \x20               }\n\
         \x20           }\n\
         \x20       }\n\
         \x20   }\n",
    );
}

fn blockquote_react(out: &mut String) {
    out.push_str(
        "  const tone_ = tone ?? \"default\";\n\
         \x20 const root = [\"nd-blockquote\", className].filter(Boolean).join(\" \");\n\
         \x20 return h(\n\
         \x20   \"blockquote\",\n\
         \x20   {\n\
         \x20     className: root,\n\
         \x20     cite,\n\
         \x20     \"data-nd-tone\": tone_,\n\
         \x20     \"data-nd-glass\": String(glass ?? false),\n\
         \x20     ...rest,\n\
         \x20   },\n\
         \x20   children,\n\
         \x20   attribution\n\
         \x20     ? h(\n\
         \x20         \"footer\",\n\
         \x20         { key: \"a\", className: \"nd-blockquote-attribution\" },\n\
         \x20         h(\"cite\", null, attribution),\n\
         \x20       )\n\
         \x20     : null,\n\
         \x20 );\n",
    );
}

// --- Del and Ins ------------------------------------------------------------

/// An edit to the document, and the word that announces it.
///
/// One emitter for both, because they differ only in the element and the class:
/// a strikethrough and a wavy underline are both invisible to a screen reader,
/// and both therefore read as though the text were current.
///
/// The label is the caller's. A design system that hardcodes "deleted" ships
/// that English into every locale its consumers have, which is the one thing
/// `NOCTUA.md §9.9` is about.
fn edit_dioxus(out: &mut String, element: &str, class: &str) {
    let _ = write!(
        out,
        "    let root = format!(\"{class} {{class}}\");\n\
         \x20   let root = root.trim().to_owned();\n\
         \n\
         \x20   rsx! {{\n\
         \x20       {element} {{\n\
         \x20           class: \"{{root}}\",\n\
         \x20           cite,\n\
         \x20           datetime,\n\
         \x20           if let Some(label) = label {{\n\
         \x20               span {{ class: \"nd-visually-hidden\", \"{{label}} \" }}\n\
         \x20           }}\n\
         \x20           {{children}}\n\
         \x20       }}\n\
         \x20   }}\n"
    );
}

fn edit_react(out: &mut String, element: &str, class: &str) {
    let _ = write!(
        out,
        "  const root = [\"{class}\", className].filter(Boolean).join(\" \");\n\
         \x20 return h(\n\
         \x20   \"{element}\",\n\
         \x20   {{ className: root, cite, dateTime: datetime, ...rest }},\n\
         \x20   label ? h(\"span\", {{ key: \"l\", className: \"nd-visually-hidden\" }}, `${{label}} `) : null,\n\
         \x20   children,\n\
         \x20 );\n"
    );
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
         \x20           rel,\n\
         \x20           \"data-nd-external\": if external { \"true\" } else { \"false\" },\n\
         \x20           \"data-nd-tone\": tone.as_str(),\n\
         \x20           {children}\n\
         \x20           if external {\n\
         \x20               Icon { icon: IconDrawing::EXTERNAL_LINK, size: crate::components::IconSize::Small }\n\
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
         \x20     rel,\n\
         \x20     \"data-nd-external\": external ? \"true\" : \"false\",\n\
         \x20     \"data-nd-tone\": tone_,\n\
         \x20     ...rest,\n\
         \x20   },\n\
         \x20   children,\n\
         \x20   external ? h(Icon, { icon: externalLink, size: \"small\" }) : null,\n\
         \x20 );\n",
    );
}

// --- Switch -----------------------------------------------------------------

/// One setting that is on or off, and the track the state slides along.
///
/// The two halves are a track and a knob, both drawn by the stylesheet from
/// `aria-checked` — so the state a screen reader hears and the state a reader
/// sees are the same attribute rather than two things kept in step.
///
/// The label is beside the control and inside the button, which is what makes
/// the whole row the hit target: a 24px knob is the minimum WCAG asks for and a
/// poor thing to ask a thumb to find.
fn switch_dioxus(out: &mut String) {
    out.push_str(
        "    let root = format!(\"nd-switch {class}\");\n\
         \x20   let root = root.trim().to_owned();\n\
         \n\
         \x20   rsx! {\n\
         \x20       button {\n\
         \x20           class: \"{root}\",\n\
         \x20           r#type: \"button\",\n\
         \x20           role: \"switch\",\n\
         \x20           \"data-nd-group\": \"{name}\",\n\
         \x20           \"data-nd-on\": \"{on}\",\n\
         \x20           \"data-nd-off\": \"{off}\",\n\
         \x20           // Always written. There is no such thing as a switch\n\
         \x20           // whose state is unknown.\n\
         \x20           \"aria-checked\": if checked { \"true\" } else { \"false\" },\n\
         \x20           \"aria-disabled\": if disabled { \"true\" } else { \"false\" },\n\
         \x20           \"aria-describedby\": describedby,\n\
         \x20           \"data-nd-size\": size.as_str(),\n\
         \x20           span { class: \"nd-switch-label\", \"{label}\" }\n\
         \x20           // The track and the knob are one element and its\n\
         \x20           // pseudo-element, so there is nothing here a caller can\n\
         \x20           // put the wrong thing inside.\n\
         \x20           //\n\
         \x20           // THE GLASS GOES ON THE TRACK, not on the button. The\n\
         \x20           // button's own box paints nothing, so frosting it would\n\
         \x20           // frost nothing — and the knob has to stay solid: its\n\
         \x20           // position is what carries the state, and a translucent\n\
         \x20           // one is a state the reader has to work out.\n\
         \x20           span {\n\
         \x20               class: \"nd-switch-track\",\n\
         \x20               \"aria-hidden\": \"true\",\n\
         \x20               \"data-nd-glass\": if glass { \"true\" } else { \"false\" },\n\
         \x20           }\n\
         \x20       }\n\
         \x20   }\n",
    );
}

fn switch_react(out: &mut String) {
    out.push_str(
        "  const checked_ = checked ?? false;\n\
         \x20 const disabled_ = disabled ?? false;\n\
         \x20 const size_ = size ?? \"medium\";\n\
         \x20 const root = [\"nd-switch\", className].filter(Boolean).join(\" \");\n\
         \x20 return h(\n\
         \x20   \"button\",\n\
         \x20   {\n\
         \x20     className: root,\n\
         \x20     type: \"button\",\n\
         \x20     role: \"switch\",\n\
         \x20     \"data-nd-group\": name,\n\
         \x20     \"data-nd-on\": on,\n\
         \x20     \"data-nd-off\": off,\n\
         \x20     // Always written. There is no such thing as a switch whose\n\
         \x20     // state is unknown.\n\
         \x20     \"aria-checked\": String(checked_),\n\
         \x20     \"aria-disabled\": String(disabled_),\n\
         \x20     \"aria-describedby\": describedby,\n\
         \x20     \"data-nd-size\": size_,\n\
         \x20     ...rest,\n\
         \x20   },\n\
         \x20   h(\"span\", { key: \"l\", className: \"nd-switch-label\" }, label),\n\
         \x20   h(\"span\", {\n\
         \x20     key: \"t\",\n\
         \x20     className: \"nd-switch-track\",\n\
         \x20     \"aria-hidden\": \"true\",\n\
         \x20     // The glass goes on the track, as it does in the Dioxus half.\n\
         \x20     \"data-nd-glass\": String(glass ?? false),\n\
         \x20   }),\n\
         \x20 );\n",
    );
}

// --- Segmented --------------------------------------------------------------

/// A radiogroup, and the keyboard contract that makes it one.
///
/// # The role is the promise; the tab stop is the proof
///
/// `role="radiogroup"` tells assistive technology three things at once: exactly
/// one member is in force, the group is entered once rather than once per
/// member, and the arrows move *and choose*. Markup that claims the role and
/// leaves every member focusable has promised something it does not do — and
/// the cost is not theoretical. A settings panel of twelve groups of three is
/// forty-two tab stops instead of twelve.
///
/// The roving `tabindex` is therefore rendered rather than applied: the group is
/// one stop with the script blocked, and it does not become forty-two if the
/// script never runs at all.
///
/// # Why the rendered stop is on the GROUP and not on the chosen item
///
/// It was on the item — `tabindex="0"` when `selected` and `-1` otherwise — and
/// that is one stop for a group with an answer and **zero** for a group without
/// one. A radiogroup nothing has chosen yet was unreachable by keyboard with
/// the script blocked, which is the opposite of what this contract promises,
/// and the first `SegmentedItem` example on the site was exactly that shape.
///
/// An item cannot fix it, because an item cannot see its siblings: whether the
/// group has an answer is a fact about the group, and no amount of rendering
/// order lets the first item know that the fourth is chosen. So the group
/// carries the stop, unconditionally, and `shell.js` takes it away the moment
/// it has placed a real one on a radio. One stop either way, and the count
/// never depends on whether a script ran.
///
/// # What lives here and what lives in the site's script
///
/// This package ships no runtime. The component owes the role, the state on
/// every member, the orientation the arrows follow, and the one tab stop. The
/// arrow handling itself is `docs-site/assets/shell.js`, keyed on the role —
/// which is why the role is on the element and not on a wrapper.
fn segmented_dioxus(out: &mut String) {
    out.push_str(
        "    let root = format!(\"nd-segmented {class}\");\n\
         \x20   let root = root.trim().to_owned();\n\
         \n\
         \x20   rsx! {\n\
         \x20       div {\n\
         \x20           class: \"{root}\",\n\
         \x20           \"data-nd-glass\": if glass { \"true\" } else { \"false\" },\n\
         \x20           role: \"radiogroup\",\n\
         \x20           \"data-nd-group\": \"{name}\",\n\
         \x20           \"data-nd-appearance\": appearance.as_str(),\n\
         \x20           \"data-nd-selection\": selection.as_str(),\n\
         \x20           \"data-nd-size\": size.as_str(),\n\
         \x20           \"data-nd-shape\": shape.as_str(),\n\
         \x20           \"data-nd-fill\": fill.as_str(),\n\
         \x20           // Announced as well as drawn: a reader needs to know which\n\
         \x20           // arrows move before pressing one.\n\
         \x20           \"aria-orientation\": orientation.as_str(),\n\
         \x20           // ONE TAB STOP, AND THE GROUP IS WHAT HOLDS IT UNTIL A\n\
         \x20           // SCRIPT MOVES IT. See the emitter note above for why\n\
         \x20           // it cannot be an item: an item cannot see its siblings,\n\
         \x20           // so no item can know whether the group has an answer.\n\
         \x20           tabindex: \"0\",\n\
         \x20           // One or the other names the group. A radiogroup with\n\
         \x20           // neither announces its role and nothing else.\n\
         \x20           \"aria-label\": label,\n\
         \x20           \"aria-labelledby\": labelledby,\n\
         \x20           {children}\n\
         \x20       }\n\
         \x20   }\n",
    );
}

fn segmented_react(out: &mut String) {
    out.push_str(
        "  const appearance_ = appearance ?? \"joined\";\n\
         \x20 const size_ = size ?? \"medium\";\n\
         \x20 const shape_ = shape ?? \"rounded\";\n\
         \x20 const fill_ = fill ?? \"hug\";\n\
         \x20 const orientation_ = orientation ?? \"horizontal\";\n\
         \x20 const root = [\"nd-segmented\", className].filter(Boolean).join(\" \");\n\
         \x20 return h(\n\
         \x20   \"div\",\n\
         \x20   {\n\
         \x20     className: root,\n\
         \x20     \"data-nd-glass\": String(glass ?? false),\n\
         \x20     role: \"radiogroup\",\n\
         \x20     \"data-nd-group\": name,\n\
         \x20     \"data-nd-appearance\": appearance_,\n\
         \x20     \"data-nd-selection\": selection ?? \"accent\",\n\
         \x20     \"data-nd-size\": size_,\n\
         \x20     \"data-nd-shape\": shape_,\n\
         \x20     \"data-nd-fill\": fill_,\n\
         \x20     \"aria-orientation\": orientation_,\n\
         \x20     // See the Dioxus half: the group holds the one stop until a\n\
         \x20     // script places it on a radio.\n\
         \x20     tabIndex: 0,\n\
         \x20     \"aria-label\": label,\n\
         \x20     \"aria-labelledby\": labelledby,\n\
         \x20     ...rest,\n\
         \x20   },\n\
         \x20   children,\n\
         \x20 );\n",
    );
}

/// One member of the group.
///
/// `aria-checked` is written whatever its value. That is the opposite of
/// `aria-pressed` on a button, where absence means "not a toggle" — a radio
/// with no `aria-checked` is a radio whose state is unknown, and a group of
/// those announces nothing chosen at all.
///
/// `aria-disabled` rather than `disabled`, so a refused item keeps its place in
/// the roving order. `disabled` removes it from the tab order, and an option a
/// keyboard cannot reach is an option a keyboard user cannot be told about.
fn segmented_item_dioxus(out: &mut String) {
    out.push_str(
        "    let root = format!(\"nd-segmented-item {class}\");\n\
         \x20   let root = root.trim().to_owned();\n\
         \n\
         \x20   rsx! {\n\
         \x20       button {\n\
         \x20           class: \"{root}\",\n\
         \x20           \"data-nd-glass\": if glass { \"true\" } else { \"false\" },\n\
         \x20           r#type: \"button\",\n\
         \x20           role: \"radio\",\n\
         \x20           \"data-nd-value\": \"{value}\",\n\
         \x20           // Always written. A radio with no `aria-checked` has no state.\n\
         \x20           \"aria-checked\": if selected { \"true\" } else { \"false\" },\n\
         \x20           \"aria-disabled\": if disabled { \"true\" } else { \"false\" },\n\
         \x20           \"data-nd-hide-label\": if hide_label { \"true\" } else { \"false\" },\n\
         \x20           // NEVER A STOP OF ITS OWN BEFORE A SCRIPT RUNS. This used\n\
         \x20           // to rove on the item, which is one stop for a group\n\
         \x20           // that has an answer and ZERO for one that\n\
         \x20           // does not — a radiogroup no keyboard could reach at all\n\
         \x20           // with the script blocked, under a contract promising one\n\
         \x20           // stop, server-rendered. The group carries it instead,\n\
         \x20           // because the group is the only element that can see\n\
         \x20           // whether anything inside it is chosen.\n\
         \x20           tabindex: \"-1\",\n\
         \x20           // The one thing an icon cannot be, and the label still names\n\
         \x20           // the item: a colour.\n\
         \x20           {children}\n\
         \x20           if let Some(icon) = icon {\n\
         \x20               Icon { icon }\n\
         \x20           }\n\
         \x20           if hide_label {\n\
         \x20               // The name stays; only the text goes. An icon that\n\
         \x20               // repeats the word beside it says the same thing twice,\n\
         \x20               // and an icon with no word at all says nothing.\n\
         \x20               span { class: \"nd-visually-hidden\", \"{label}\" }\n\
         \x20           } else {\n\
         \x20               span { class: \"nd-segmented-text\", \"{label}\" }\n\
         \x20           }\n\
         \x20           // The second line, and it is part of the NAME. A stop\n\
         \x20           // that reads `320` over `px` announces `320 px`, which\n\
         \x20           // is the whole value — a reader who cannot see the\n\
         \x20           // stack is choosing from the number alone otherwise.\n\
         \x20           if let Some(detail) = detail {\n\
         \x20               span { class: \"nd-segmented-detail\", \"{detail}\" }\n\
         \x20           }\n\
         \x20       }\n\
         \x20   }\n",
    );
}

fn segmented_item_react(out: &mut String) {
    out.push_str(
        "  const selected_ = selected ?? false;\n\
         \x20 const disabled_ = disabled ?? false;\n\
         \x20 const hideLabel_ = hideLabel ?? false;\n\
         \x20 const root = [\"nd-segmented-item\", className].filter(Boolean).join(\" \");\n\
         \x20 return h(\n\
         \x20   \"button\",\n\
         \x20   {\n\
         \x20     className: root,\n\
         \x20     \"data-nd-glass\": String(glass ?? false),\n\
         \x20     type: \"button\",\n\
         \x20     role: \"radio\",\n\
         \x20     \"data-nd-value\": value,\n\
         \x20     // Always written. A radio with no `aria-checked` has no state.\n\
         \x20     \"aria-checked\": String(selected_),\n\
         \x20     \"aria-disabled\": String(disabled_),\n\
         \x20     \"data-nd-hide-label\": String(hideLabel_),\n\
         \x20     // See the Dioxus half: the group holds the stop until `roving`\n\
         \x20     // moves it onto a radio.\n\
         \x20     tabIndex: -1,\n\
         \x20     ...rest,\n\
         \x20   },\n\
         \x20   children,\n\
         \x20   icon ? h(Icon, { key: \"i\", icon }) : null,\n\
         \x20   hideLabel_\n\
         \x20     ? h(\"span\", { key: \"l\", className: \"nd-visually-hidden\" }, label)\n\
         \x20     : h(\"span\", { key: \"l\", className: \"nd-segmented-text\" }, label),\n\
         \x20   // The second line, and it is part of the name.\n\
         \x20   detail ? h(\"span\", { key: \"d\", className: \"nd-segmented-detail\" }, detail) : null,\n\
         \x20 );\n",
    );
}

// --- Disclosure -------------------------------------------------------------

/// `<details>`/`<summary>`, and not a hand-built pair.
///
/// The browser gives the role, the state and the keyboard for free and gets
/// them right in every engine. It is also not the `popover` attribute: the top
/// layer is always `position: fixed`, so a popover cannot anchor to a sticky
/// header without being repositioned on every scroll frame.
///
/// Nothing here moves focus on open and nothing is trapped. A reader who tabs
/// past an open disclosure carries on into the page, which is correct for
/// something that is not a dialog.
fn disclosure_dioxus(out: &mut String) {
    out.push_str(
        "    let root = format!(\"nd-disclosure {class}\");\n\
         \x20   let root = root.trim().to_owned();\n\
         \n\
         \x20   rsx! {\n\
         \x20       details {\n\
         \x20           class: \"{root}\",\n\
         \x20           \"data-nd-glass\": if glass { \"true\" } else { \"false\" },\n\
         \x20           open,\n\
         \x20           // The hook a script binds to, written from a prop rather\n\
         \x20           // than left to the caller: an arbitrary-attribute escape\n\
         \x20           // hatch on one component is an escape hatch on all of\n\
         \x20           // them. `None` writes nothing at all.\n\
         \x20           \"data-nd-disclosure\": name,\n\
         \x20           summary {\n\
         \x20               class: \"nd-disclosure-summary\",\n\
         \x20               // The summary is the accessible name whether it is\n\
         \x20               // shown or replaced by an icon, so an icon-only\n\
         \x20               // disclosure is still named.\n\
         \x20               \"aria-label\": if icon.is_some() { Some(summary.clone()) } else { None },\n\
         \x20               title: if icon.is_some() { Some(summary.clone()) } else { None },\n\
         \x20               if let Some(icon) = icon {\n\
         \x20                   Icon { icon }\n\
         \x20               } else {\n\
         \x20                   // A REAL HEADING, INSIDE THE SUMMARY. The HTML\n\
         \x20                   // spec allows summary one heading, and a folded\n\
         \x20                   // section whose title is a `<span>` is a section\n\
         \x20                   // missing from the outline a screen reader\n\
         \x20                   // navigates by — which is the only thing left of\n\
         \x20                   // it while it is closed.\n\
         \x20                   match heading {\n\
         \x20                       DisclosureHeading::H2 => rsx! { h2 { class: \"nd-disclosure-text\", \"{summary}\" } },\n\
         \x20                       DisclosureHeading::H3 => rsx! { h3 { class: \"nd-disclosure-text\", \"{summary}\" } },\n\
         \x20                       DisclosureHeading::H4 => rsx! { h4 { class: \"nd-disclosure-text\", \"{summary}\" } },\n\
         \x20                       DisclosureHeading::None => rsx! { span { class: \"nd-disclosure-text\", \"{summary}\" } },\n\
         \x20                   }\n\
         \x20                   // NAMED, BECAUSE THE RULE THAT TURNS IT USED TO\n\
         \x20                   // ASK FOR THE LAST GLYPH. In the icon-only form\n\
         \x20                   // the caller\'s own icon is the last glyph, so\n\
         \x20                   // opening a `Disclosure { icon: \"settings\" }`\n\
         \x20                   // turned the gear upside down.\n\
         \x20                   if marker {\n\
         \x20                       Icon {\n\
         \x20                           icon: IconDrawing::CHEVRON_DOWN,\n\
         \x20                           size: crate::components::IconSize::Small,\n\
         \x20                           class: \"nd-disclosure-marker\",\n\
         \x20                       }\n\
         \x20                   }\n\
         \x20               }\n\
         \x20           }\n\
         \x20           {children}\n\
         \x20       }\n\
         \x20   }\n",
    );
}

fn disclosure_react(out: &mut String) {
    out.push_str(
        "  const marker_ = marker ?? true;\n\
         \x20 const heading_ = heading ?? \"none\";\n\
         \x20 // The summary's own tag: a real heading when this disclosure is a\n\
         \x20 // section of the page, because folded away its title is all that is\n\
         \x20 // left and a `<span>` leaves the section out of the outline.\n\
         \x20 const tag_ = heading_ === \"none\" ? \"span\" : heading_;\n\
         \x20 const root = [\"nd-disclosure\", className].filter(Boolean).join(\" \");\n\
         \x20 return h(\n\
         \x20   \"details\",\n\
         \x20   {\n\
         \x20     className: root,\n\
         \x20     \"data-nd-glass\": String(glass ?? false),\n\
         \x20     open: open || undefined,\n\
         \x20     \"data-nd-disclosure\": name,\n\
         \x20     ...rest,\n\
         \x20   },\n\
         \x20   h(\n\
         \x20     \"summary\",\n\
         \x20     {\n\
         \x20       className: \"nd-disclosure-summary\",\n\
         \x20       \"aria-label\": icon ? summary : undefined,\n\
         \x20       title: icon ? summary : undefined,\n\
         \x20     },\n\
         \x20     icon\n\
         \x20       ? h(Icon, { icon })\n\
         \x20       : [\n\
         \x20           h(tag_, { key: \"t\", className: \"nd-disclosure-text\" }, summary),\n\
         \x20           // See the Dioxus half: named, because the rule that turns\n\
         \x20           // it used to ask for the last glyph in the summary.\n\
         \x20           marker_\n\
         \x20             ? h(Icon, {\n\
         \x20                 key: \"m\",\n\
         \x20                 icon: chevronDown,\n\
         \x20                 size: \"small\",\n\
         \x20                 className: \"nd-disclosure-marker\",\n\
         \x20               })\n\
         \x20             : null,\n\
         \x20         ],\n\
         \x20   ),\n\
         \x20   children,\n\
         \x20 );\n",
    );
}

// --- Checkbox and Radio -----------------------------------------------------

/// What separates a checkbox from a radio, and it is not much.
///
/// Both are drawn at one size and laid out the same way, because the
/// two appear in the same form one under the other and two controls asking the
/// same kind of question at two sizes read as two kinds of question. So the row,
/// the ids, the hint, the divider and the conditional reveal are written once
/// below and this is everything that differs: the second class, the input's own
/// `type`, the mark drawn inside the box, and the one attribute a checkbox has
/// that a radio can have no use for.
#[derive(Clone, Copy)]
struct Choice {
    /// The class beside `nd-choice`, which is what picks the shape.
    class: &'static str,
    /// `checkbox` or `radio`. The whole accessibility contract is this word.
    input: &'static str,
    /// The mark inside the box, in Dioxus and in React.
    mark_rust: &'static str,
    mark_js: &'static str,
    /// The none-of-the-above attribute, or nothing. A whole line each, so
    /// removing it removes the line rather than leaving a blank one.
    exclusive_rust: &'static str,
    exclusive_js: &'static str,
}

/// A tick from the icon set, never a shape drawn out of borders.
///
/// A tick drawn out of borders is a 23x12 box with two 5px edges rotated 45
/// degrees: six raw lengths for one glyph. `NOCTUA.md §9.5` has an answer for that
/// already: it is an icon, it is vendored, and the size gate has nothing to
/// complain about because there is no length here at all.
const CHECKBOX: Choice = Choice {
    class: "nd-checkbox",
    input: "checkbox",
    mark_rust: "Icon { icon: IconDrawing::CHECK }",
    mark_js: "h(Icon, { key: \"m\", icon: check })",
    exclusive_rust: "                \"data-nd-exclusive\": if exclusive { \"true\" } else { \"false\" },\n",
    exclusive_js: "        \"data-nd-exclusive\": String(exclusive ?? false),\n",
};

/// A dot, which is a shape rather than a glyph and has no icon to be.
const RADIO: Choice = Choice {
    class: "nd-radio",
    input: "radio",
    mark_rust: "span { class: \"nd-choice-dot\" }",
    mark_js: "h(\"span\", { key: \"m\", className: \"nd-choice-dot\" })",
    exclusive_rust: "",
    exclusive_js: "",
};

/// The row: an input nobody can see, the box drawn for it, a label and a hint.
///
/// # Why the input is transparent rather than hidden
///
/// `display: none` and `visibility: hidden` both take the control out of the
/// accessibility tree along with the drawing — and the drawing is not a
/// control. The input is `opacity: 0`, 44 pixels square, exactly over the box,
/// which is what makes it survive a screen magnifier. It is also why the hit target is the AAA 44 and not the AA 24 at
/// every one of the three sizes: the box shrinks and the input does not.
///
/// # Why the id is derived
///
/// A `<label for>` needs one, `aria-describedby` needs one, and the revealed
/// block needs one. A caller who writes three ids writes two of them the same
/// eventually, and the failure is silent: the label points at the first control
/// with that id and the second is nameless. `{name}` and `{value}` are already
/// required, already unique within a group, and already what the form submits.
fn choice_dioxus(out: &mut String, choice: Choice) {
    out.push_str(
        &CHOICE_DIOXUS
            .replace("«class»", choice.class)
            .replace("«input»", choice.input)
            .replace("«mark»", choice.mark_rust)
            .replace("«exclusive»\n", choice.exclusive_rust),
    );
}

fn choice_react(out: &mut String, choice: Choice) {
    out.push_str(
        &CHOICE_REACT
            .replace("«class»", choice.class)
            .replace("«input»", choice.input)
            .replace("«mark»", choice.mark_js)
            .replace("«exclusive»\n", choice.exclusive_js),
    );
}

const CHOICE_DIOXUS: &str = r#"    let root = format!("nd-choice «class» {class}");
    let root = root.trim().to_owned();
    // DERIVED, so a caller never writes an id and never writes two the same.
    // Three things point at this one: the label, the hint and the block the
    // choice reveals.
    let id = format!("nd-{name}-{value}");
    let hint_id = format!("{id}-hint");
    let reveal_id = format!("{id}-reveal");
    // Only what exists is named. `aria-describedby` pointing at an id nothing
    // carries is read as silence by some engines and as the whole document by
    // others, and neither of those is the hint.
    let described = hint.as_ref().map(|_| hint_id.clone());

    rsx! {
        // BEFORE the row and outside it: the divider separates this choice
        // from the ones above, so it is a sibling of the row rather than
        // something inside the row it introduces.
        if let Some(divider) = divider {
            p { class: "nd-choice-divider", "aria-hidden": "true", "{divider}" }
        }
        div {
            class: "{root}",
            "data-nd-size": size.as_str(),
            // Transparent and 44px square over the box, never `display: none`
            // — that would hide it from the accessibility tree as well as from
            // the page, and what is left is a drawing rather than a control.
            input {
                class: "nd-choice-input",
                id: "{id}",
                r#type: "«input»",
                name: "{name}",
                value: "{value}",
                checked,
                disabled,
                // Absent rather than "false": `aria-invalid="false"` is the
                // default state said out loud.
                "aria-invalid": invalid.then_some("true"),
                "aria-describedby": described,
«exclusive»
                "data-nd-reveal": if reveal { "true" } else { "false" },
            }
            // The box and its mark are decoration. The state a reader hears is
            // the input's, and a second copy of it here could only disagree.
            span { class: "nd-choice-box", "aria-hidden": "true",
                «mark»
            }
            // HIDDEN, NOT ABSENT. The label is still the accessible name — the
            // recipe clips it rather than removing it — so a box in a table's
            // heading column announces what checking it does while taking no
            // room to say so.
            label {
                class: "nd-choice-label",
                r#for: "{id}",
                "data-nd-hide-label": if hide_label { "true" } else { "false" },
                "{label}"
            }
            if let Some(hint) = hint {
                p { class: "nd-choice-hint", id: "{hint_id}", "{hint}" }
            }
        }
        // VISIBLE WITH NO SCRIPT AT ALL. The script is what hides it, which is
        // the only order of those two that degrades: a block hidden by CSS and
        // revealed by script is a question a reader without script never sees.
        if reveal {
            div { class: "nd-choice-reveal", id: "{reveal_id}", {children} }
        }
    }
"#;

const CHOICE_REACT: &str = r#"  const size_ = size ?? "large";
  const root = ["nd-choice «class»", className].filter(Boolean).join(" ");
  // Derived, as in the Dioxus half: three things point at this id and a caller
  // who writes it writes two the same eventually.
  const id = `nd-${name}-${value}`;
  const hintId = `${id}-hint`;
  const revealId = `${id}-reveal`;
  return h(
    Fragment,
    null,
    // Before the row and outside it, so it separates this choice from the ones
    // above rather than sitting inside the one it introduces.
    divider
      ? h("p", { key: "v", className: "nd-choice-divider", "aria-hidden": "true" }, divider)
      : null,
    h(
      "div",
      { key: "c", className: root, "data-nd-size": size_ },
      // `...rest` lands on the INPUT rather than on the row: everything a
      // caller passes through here — `onChange`, `form`, `required` — belongs
      // to the control, and the row is a grid this component owns.
      h("input", {
        key: "i",
        className: "nd-choice-input",
        id,
        type: "«input»",
        name,
        value,
        defaultChecked: checked ?? false,
        disabled: disabled || undefined,
        // Absent rather than "false", which is the default state said aloud.
        "aria-invalid": invalid ? "true" : undefined,
        "aria-describedby": hint ? hintId : undefined,
«exclusive»
        "data-nd-reveal": String(reveal ?? false),
        ...rest,
      }),
      // Decoration. The state a reader hears is the input's.
      h("span", { key: "b", className: "nd-choice-box", "aria-hidden": "true" }, «mark»),
      // Hidden, not absent: the recipe clips the words and leaves them as the
      // control's accessible name.
      h(
        "label",
        {
          key: "l",
          className: "nd-choice-label",
          htmlFor: id,
          "data-nd-hide-label": String(hideLabel ?? false),
        },
        label,
      ),
      hint ? h("p", { key: "h", className: "nd-choice-hint", id: hintId }, hint) : null,
    ),
    // Visible with no script. The script hides it; nothing here reveals it.
    reveal
      ? h("div", { key: "r", className: "nd-choice-reveal", id: revealId }, children)
      : null,
  );
"#;

// --- Fieldset ---------------------------------------------------------------

/// The question a group answers, and the two sentences that belong to it.
///
/// # It owns the wiring so a caller cannot get it wrong
///
/// The hint is `nd-{name}-hint`, the error is `nd-{name}-error`, and
/// `aria-describedby` names whichever of the two exists. A caller writes
/// neither id, so a caller cannot mistype one — which is the failure that
/// leaves a hint on the page and out of the announcement.
///
/// # `data-nd-invalid` is derived from `error`, never declared beside it
///
/// One fact with two switches is one fact and one bug waiting: a red rule with
/// no sentence, or a sentence with nothing marking the group. There is one
/// switch, and it is the sentence itself.
fn fieldset_dioxus(out: &mut String) {
    out.push_str(FIELDSET_DIOXUS);
}

fn fieldset_react(out: &mut String) {
    out.push_str(FIELDSET_REACT);
}

const FIELDSET_DIOXUS: &str = r#"    let root = format!("nd-fieldset {class}");
    let root = root.trim().to_owned();
    let hint_id = format!("nd-{name}-hint");
    let error_id = format!("nd-{name}-error");
    // Two optional halves, so this is a list joined rather than a string with
    // a hole in it: a describedby naming an id nothing carries is worse than
    // one naming nothing.
    let described: Vec<String> = [
        hint.as_ref().map(|_| hint_id.clone()),
        error.as_ref().map(|_| error_id.clone()),
    ]
    .into_iter()
    .flatten()
    .collect();
    let described = (!described.is_empty()).then(|| described.join(" "));
    let question = legend;
    // Read before the sentence is moved into the markup below, and derived
    // from it rather than declared beside it: one fact, one switch.
    let flagged = if error.is_some() { "true" } else { "false" };

    rsx! {
        fieldset {
            class: "{root}",
            "data-nd-group": "{name}",
            "data-nd-heading": if heading { "true" } else { "false" },
            "data-nd-inline": if inline { "true" } else { "false" },
            "data-nd-invalid": flagged,
            disabled,
            "aria-describedby": described,
            legend { class: "nd-fieldset-legend",
                // WRAPPED, not replaced. A page that asks one question should
                // have that question as its heading — and a <legend> swapped
                // for an <h1> stops being attached to the controls at all,
                // which is the only thing a legend is for.
                if heading {
                    h1 { class: "nd-fieldset-heading", "{question}" }
                } else {
                    "{question}"
                }
            }
            if let Some(hint) = hint {
                p { class: "nd-fieldset-hint", id: "{hint_id}", "{hint}" }
            }
            // `role="alert"`, because this appears after a submission and a
            // message that appears silently is one a reader never learns about.
            if let Some(error) = error {
                p { class: "nd-fieldset-error", id: "{error_id}", role: "alert",
                    if let Some(error_label) = error_label {
                        span { class: "nd-visually-hidden", "{error_label} " }
                    }
                    "{error}"
                }
            }
            div { class: "nd-choices", {children} }
        }
    }
"#;

const FIELDSET_REACT: &str = r#"  const root = ["nd-fieldset", className].filter(Boolean).join(" ");
  const hintId = `nd-${name}-hint`;
  const errorId = `nd-${name}-error`;
  // A list joined, not a string with a hole in it: two optional halves.
  const described = [hint ? hintId : null, error ? errorId : null].filter(Boolean).join(" ");
  return h(
    "fieldset",
    {
      className: root,
      "data-nd-group": name,
      "data-nd-heading": String(heading ?? false),
      "data-nd-inline": String(inline ?? false),
      // Derived from the sentence rather than declared beside it.
      "data-nd-invalid": String(Boolean(error)),
      disabled: disabled || undefined,
      "aria-describedby": described || undefined,
      ...rest,
    },
    h(
      "legend",
      { key: "g", className: "nd-fieldset-legend" },
      // Wrapped, not replaced: a legend swapped for a heading stops being
      // attached to the controls, which is the only thing a legend is for.
      heading ? h("h1", { className: "nd-fieldset-heading" }, legend) : legend,
    ),
    hint ? h("p", { key: "h", className: "nd-fieldset-hint", id: hintId }, hint) : null,
    error
      ? h(
          "p",
          { key: "e", className: "nd-fieldset-error", id: errorId, role: "alert" },
          errorLabel
            ? h("span", { key: "w", className: "nd-visually-hidden" }, `${errorLabel} `)
            : null,
          error,
        )
      : null,
    h("div", { key: "c", className: "nd-choices" }, children),
  );
"#;

// --- Select, SelectGroup and SelectOption -----------------------------------

/// A field: the label, the two sentences about it, and a real `<select>`.
///
/// # This ships a working control and adds to it, rather than replacing it
///
/// The markup below is submittable on its own. `docs-site/assets/shell.js`
/// builds a `role="combobox"` over it, reads its state from it and writes every
/// choice back into it — so the value that reaches a server is the browser's,
/// the touch keyboard is the platform's, and a page whose script failed to load
/// still works. The usual alternative is a `<div role="listbox">` beside a
/// hidden `<input>`, which is where this whole category of component gets its
/// signature bug: a control that looks right and submits nothing.
///
/// # Every word is a prop
///
/// A searchable listbox has to say four things — name the filter, say nothing
/// matched, offer to add what was typed, name the control that clears the
/// field — and `NOCTUA.md §9.9` forbids this package from carrying a word in
/// any language. They ride on `data-nd-*` for the script to read, which is the
/// mechanism `CodeBlock { copy, copied }` already established.
const SELECT_DIOXUS: &str = r#"    let root = format!("nd-select {class}");
    let root = root.trim().to_owned();
    // Derived, so the label's target, the hint and the error cannot be typed
    // wrong — there is nothing to type. `Field` derives the same two ids from
    // `control` by the rule its own documentation states.
    let id = format!("nd-select-{name}");
    let hint_id = format!("{id}-hint");
    let error_id = format!("{id}-error");
    // A REQUIRED CONTROL NEEDS SOMEWHERE TO PUT THE BROWSER'S OWN COMPLAINT.
    //
    // The native `<select>` is hidden once the script enhances this, and a
    // `display: none` control cannot be reported: the browser has to focus an
    // invalid control to anchor its bubble, cannot, and gives up — Chrome logs
    // "An invalid form control with name=… is not focusable" and the submit
    // silently does nothing. So the slot is rendered whenever `required` is
    // set, empty, and `shell.js` writes `validationMessage` into it. Empty, it
    // is `display: none` and announces nothing; the moment it has words it is
    // a `role="alert"` like any other.
    let error_slot = error.is_some() || required;
    let described: Vec<String> = [
        hint.as_ref().map(|_| hint_id.clone()),
        error_slot.then(|| error_id.clone()),
    ]
    .into_iter()
    .flatten()
    .collect();
    let described = (!described.is_empty()).then(|| described.join(" "));
    // Read before the sentence is moved into the markup, and derived from it:
    // one fact, one switch.
    let flagged = if error.is_some() { "true" } else { "false" };
    // Read before the option below consumes it: the attribute and the empty
    // `<option>` are two renderings of one word, and only one of them exists on
    // a `multiple` select.
    let empty_text = placeholder.clone();
    // `label` is a prop AND an element here, so the prop gets a name of its own
    // rather than relying on which one the macro means.
    let field = label;

    rsx! {
        Field {
            control: "{id}",
            label: field,
            hint: hint.clone(),
            error: error.clone(),
            // THE SLOT THIS COMPONENT ARRIVED AT FIRST, and `DateInput`
            // second, which is why it lives in `Field` now: the native
            // `<select>` is hidden once the script runs, and a hidden control
            // cannot be reported — the browser has to focus an invalid control
            // to anchor its bubble, cannot, and the submit silently does
            // nothing. `shell.js` writes `validationMessage` in here.
            error_slot: required,
            error_label: error_label,
            size: match size {
                SelectSize::Small => FieldSize::Small,
                SelectSize::Medium => FieldSize::Medium,
                SelectSize::Large => FieldSize::Large,
            },
            disabled,

        div {
            class: "{root}",
            // The hook the enhancement binds to, and every switch it reads.
            "data-nd-select": "",
            "data-nd-size": size.as_str(),
            "data-nd-searchable": if searchable { "true" } else { "false" },
            "data-nd-clearable": if clearable { "true" } else { "false" },
            "data-nd-loading": if loading { "true" } else { "false" },
            "data-nd-creatable": if creatable { "true" } else { "false" },
            "data-nd-invalid": flagged,
            // The sentences the script has to say, in the reader's language,
            // because this package ships none of its own. The placeholder is
            // among them: on a `multiple` select there is no empty `<option>`
            // to read it off, and a control that says nothing at all while it
            // is unanswered reads as one that is broken.
            "data-nd-placeholder": empty_text,
            "data-nd-search-label": search_label,
            "data-nd-empty-label": empty_label,
            "data-nd-loading-label": loading_label,
            "data-nd-create-label": create_label,
            "data-nd-selected-label": selected_label,
            // The control and the mark over it, in a box of their own: the
            // chevron is positioned against THIS rather than against the field,
            // which also holds a label and two sentences it would otherwise be
            // measured from.
            div { class: "nd-select-field",
                select {
                    class: "nd-select-native",
                    id: "{id}",
                    name: "{name}",
                    "aria-describedby": described,
                    required,
                    multiple,
                    disabled,
                    // The empty answer FIRST, so a field nobody has touched
                    // submits nothing rather than whatever happened to be at
                    // the top of the list — which is the quietest way a form
                    // collects an answer nobody gave.
                    //
                    // A `multiple` select needs none: nothing chosen already
                    // submits nothing, and a blank row inside a list box is a
                    // row a reader can select and learn nothing from.
                    if let Some(placeholder) = placeholder.filter(|_| !multiple) {
                        option { value: "", "{placeholder}" }
                    }
                    {children}
                }
                // The affordance the browser draws for itself and this hides
                // with `appearance: none`. It is an icon from the set rather
                // than a background image, which is what `NOCTUA.md §9.5` asks
                // — and the enhancement clones this node onto the control it
                // builds, so the script never carries a drawing.
                span { class: "nd-select-chevron", "aria-hidden": "true",
                    Icon { icon: IconDrawing::CHEVRON_DOWN, size: crate::components::IconSize::Small }
                }
                // The control that empties the field. Rendered here rather than
                // built by the script for the same reason as the chevron — and
                // because it is a real control with a real name, which the
                // script has no business inventing. Hidden until the script has
                // something to clear with it.
                if clearable {
                    button {
                        class: "nd-select-clear",
                        r#type: "button",
                        "data-nd-select-clear": "",
                        "aria-label": clear_label.clone(),
                        title: clear_label.clone(),
                        Icon { icon: IconDrawing::CLOSE, size: crate::components::IconSize::Small }
                    }
                }
                // THE TICK THE ENHANCED LIST MARKS A CHOSEN OPTION WITH.
                //
                // In a `<template>`, whose content is inert and unrendered, for
                // the script to clone. The behaviour lives in the site's script
                // and every drawing lives in the markup: a script that carried
                // an SVG would be a second place icons come from, which is what
                // `NOCTUA.md §9.5` exists to prevent.
                template { "data-nd-glyph": "check",
                    Icon { icon: IconDrawing::CHECK, size: crate::components::IconSize::Small }
                }
            }
        }
        }
    }
"#;

const SELECT_REACT: &str = r#"  const size_ = size ?? "medium";
  const root = ["nd-select", className].filter(Boolean).join(" ");
  const id = `nd-select-${name}`;
  const hintId = `${id}-hint`;
  const errorId = `${id}-error`;
  // See the Dioxus half: a required control needs somewhere to put the
  // browser's own complaint, because the native select is hidden by then and a
  // `display: none` control cannot be reported.
  const errorSlot = Boolean(error) || Boolean(required);
  const described = [hint ? hintId : null, errorSlot ? errorId : null].filter(Boolean).join(" ");
  return h(
    Field,
    {
      control: id,
      label,
      hint,
      error,
      errorSlot: Boolean(required),
      errorLabel,
      size: size_,
      disabled,
    },
  h(
    "div",
    {
      key: "s",
      className: root,
      "data-nd-select": "",
      "data-nd-size": size_,
      "data-nd-searchable": String(searchable ?? false),
      "data-nd-clearable": String(clearable ?? false),
      "data-nd-loading": String(loading ?? false),
      "data-nd-creatable": String(creatable ?? false),
      // Derived from the sentence rather than declared beside it.
      "data-nd-invalid": String(Boolean(error)),
      // The placeholder is one of the words the script needs: a `multiple`
      // select carries no empty `<option>` to read it off.
      "data-nd-placeholder": placeholder,
      "data-nd-search-label": searchLabel,
      "data-nd-empty-label": emptyLabel,
      "data-nd-loading-label": loadingLabel,
      "data-nd-create-label": createLabel,
      "data-nd-selected-label": selectedLabel,
    },

    h(
      "div",
      { key: "f", className: "nd-select-field" },
      h(
        "select",
        {
          key: "s",
          className: "nd-select-native",
          id,
          name,
          "aria-describedby": described || undefined,
          required: required || undefined,
          multiple: multiple || undefined,
          disabled: disabled || undefined,
          // `...rest` lands on the SELECT, not on the field: `onChange`, `form`
          // and `autoComplete` all belong to the control, and the field around
          // it is a label and two sentences this component owns.
          ...rest,
        },
        // Not on a `multiple` select: nothing chosen already submits nothing,
        // and a blank row inside a list box is a row that teaches nobody.
        placeholder && !multiple ? h("option", { key: "p", value: "" }, placeholder) : null,
        children,
      ),
      // The affordance `appearance: none` takes away, from the icon set rather
      // than from a background image.
      h(
        "span",
        { key: "c", className: "nd-select-chevron", "aria-hidden": "true" },
        h(Icon, { icon: chevronDown, size: "small" }),
      ),
      // A real control with a real name, rendered rather than built by a
      // script, and hidden until the script has something to clear with it.
      clearable
        ? h(
            "button",
            {
              key: "x",
              className: "nd-select-clear",
              type: "button",
              "data-nd-select-clear": "",
              "aria-label": clearLabel,
              title: clearLabel,
            },
            h(Icon, { icon: close, size: "small" }),
          )
        : null,
      // The tick the enhanced list marks a chosen option with, in a template
      // for the script to clone: every drawing lives in the markup.
      h(
        "template",
        { key: "t", "data-nd-glyph": "check" },
        h(Icon, { icon: check, size: "small" }),
      ),
    ),
  ),
  );
"#;

/// A named run of options, and nothing but `<optgroup>`.
///
/// The browser draws the heading, indents the run, refuses to let the heading
/// be chosen and announces the group's name with the first option inside it —
/// on every engine, including the ones whose popup is a native sheet nothing
/// here can style.
const SELECT_GROUP_DIOXUS: &str = r#"    let root = format!("nd-select-group {class}");
    let root = root.trim().to_owned();

    rsx! {
        optgroup {
            class: "{root}",
            label: "{label}",
            // The browser's own cascade: nothing here walks the options, so
            // nothing here can miss one.
            disabled,
            {children}
        }
    }
"#;

const SELECT_GROUP_REACT: &str = r#"  const root = ["nd-select-group", className].filter(Boolean).join(" ");
  return h(
    "optgroup",
    { className: root, label, disabled: disabled || undefined, ...rest },
    children,
  );
"#;

/// One value, and the richer row the enhanced list draws from data.
///
/// # Why the icon and the second line are attributes
///
/// An `<option>`'s content model is a string. An icon cannot be a child of one,
/// and a library that renders one anyway has stopped rendering a `<select>` —
/// which is the trade every rich select makes and the reason so few of them
/// submit anything. So the extra rides as `data-nd-*`: the native control shows
/// the label, which is the whole answer, and the enhanced list reads the rest.
///
/// # A note about `selected` in React
///
/// React logs a development warning for `selected` on an `<option>` and asks
/// for `defaultValue` on the `<select>` instead. It is not taken here, because
/// the alternative is a `Select` that has to be told its own answer separately
/// from the option that *is* the answer — two places for one fact, and the two
/// disagree the first time an option is inserted. The markup both halves
/// produce is identical, which is what `NOCTUA.md §9.3` asks of them.
const SELECT_OPTION_DIOXUS: &str = r#"    let root = format!("nd-select-option {class}");
    let root = root.trim().to_owned();

    rsx! {
        option {
            class: "{root}",
            value: "{value}",
            selected,
            disabled,
            // Data, because an <option> holds text and nothing else. The
            // native control shows the label; the enhanced list draws these.
            "data-nd-detail": detail,
            "data-nd-icon": icon.map(|icon| icon.name),
            "data-nd-swatch": swatch,
            "{label}"
        }
        // THE GLYPH, IN A `<template>` BESIDE THE OPTION.
        //
        // An `<option>` holds text, so the drawing cannot go inside it — and a
        // `<select>` holds options, optgroups and *script-supporting elements*,
        // which is `<script>` and `<template>`. A template's content is inert,
        // unrendered and invisible to the native control, which makes it the
        // one legal place a drawing can sit next to an option that cannot
        // contain one. The enhanced list clones it; the script still ships no
        // drawings of its own.
        if let Some(icon) = icon {
            template { "data-nd-glyph": icon.name,
                Icon { icon, size: crate::components::IconSize::Small }
            }
        }
    }
"#;

const SELECT_OPTION_REACT: &str = r#"  const root = ["nd-select-option", className].filter(Boolean).join(" ");
  return h(
    Fragment,
    null,
    h(
      "option",
      {
        key: "o",
        className: root,
        value,
        selected: selected || undefined,
        disabled: disabled || undefined,
        // Data, because an <option> holds text and nothing else.
        "data-nd-detail": detail,
        "data-nd-icon": icon,
        "data-nd-swatch": swatch,
        ...rest,
      },
      label,
    ),
    // The glyph, in a `<template>` beside the option — the one legal place a
    // drawing can sit next to an element that cannot contain one.
    icon
      ? h(
          "template",
          { key: "g", "data-nd-glyph": icon },
          h(Icon, { icon, size: "small" }),
        )
      : null,
  );
"#;

// --- Table, and its five parts ----------------------------------------------

/// Rows and columns, inside the box that scrolls them and under the tools that
/// drive them.
///
/// # Three jobs one declaration used to do
///
/// `.nd-table` said `display: block; overflow-x: auto` on the `<table>` itself.
/// That is overflow, semantics and presentation in one line, and the middle one
/// was a defect: every engine computes the table roles from the **layout
/// object**, not from the tag, so a `<table>` laid out as a block is announced
/// as a run of unrelated text.
///
/// So the overflow moves to a wrapper, the roles are restated in the markup on
/// every part, and nothing here changes a `display` without putting the role
/// back — including the stacked layout, where the caption and the note are
/// given a display of their own rather than left as table boxes with no table.
///
/// # Why the scrolling box is focusable before anyone measures it
///
/// A scroll container that answers only a pointer cannot be read with a
/// keyboard (WCAG 2.1.1). The tab stop and the region role are therefore
/// **rendered**, and the script's job is to take them away again on a table
/// that does not overflow — an enhancement may remove a way in, and must never
/// be the thing that adds one.
///
/// # Why the tools are rendered hidden
///
/// The columns menu needs one row per hideable column, and only the script can
/// count them; the copy control needs a clipboard, and only the script has one.
/// Both are rendered and hidden, and the script reveals what it can honour —
/// `Carousel`'s rule, for `Carousel`'s reason: a control that does nothing is
/// worse than no control.
const TABLE_DIOXUS: &str = r#"    let root = format!("nd-table {class}");
    let root = root.trim().to_owned();
    let caption_id = format!("nd-table-{name}-caption");
    let note_id = format!("nd-table-{name}-note");
    let columns_id = format!("nd-table-{name}-columns");
    let search_id = format!("nd-table-{name}-search");
    let search_input_id = format!("{search_id}-input");
    // Only what exists is pointed at. A describedby naming an id nothing
    // carries is read as silence by some engines and as the document by others.
    let described = note.as_ref().map(|_| note_id.clone());
    // `caption` is a prop AND an element here, so the prop gets a name of its
    // own rather than relying on which one the macro means.
    let title = caption;
    let columns_name = columns_label.clone().unwrap_or_default();

    rsx! {
        div { class: "nd-table-frame", "data-nd-table-frame": "{name}",

            // THE TOOLBAR, hidden until the script has something to put in it.
            div { class: "nd-table-tools", "data-nd-table-tools": "", hidden: true,
                // ONE SEARCH, BEHIND A GLASS. A box in every filterable heading
                // is a heading row twice as tall and a row of controls a reader
                // visits one at a time.
                if search_label.is_some() {
                    div { class: "nd-table-search", "data-nd-table-search": "",
                        button {
                            class: "nd-table-search-open",
                            r#type: "button",
                            "aria-expanded": "false",
                            "aria-controls": "{search_id}",
                            "aria-label": search_label.clone().unwrap_or_default(),
                            Icon { icon: IconDrawing::SEARCH, size: IconSize::Small }
                        }
                        div { class: "nd-table-search-field", id: "{search_id}", hidden: true,
                            span { class: "nd-table-search-glyph", "aria-hidden": "true",
                                Icon { icon: IconDrawing::SEARCH, size: IconSize::Small }
                            }
                            label {
                                class: "nd-visually-hidden",
                                r#for: "{search_input_id}",
                                {search_label.clone().unwrap_or_default()}
                            }
                            input {
                                class: "nd-table-search-input",
                                id: "{search_input_id}",
                                r#type: "search",
                                "data-nd-table-search-input": "",
                                autocomplete: "off",
                                spellcheck: "false",
                                placeholder: search_placeholder.clone(),
                            }
                        }
                    }
                }
                if columns_label.is_some() {
                    details { class: "nd-table-columns", "data-nd-table-columns": "",
                        summary { class: "nd-table-columns-summary",
                            Icon { icon: IconDrawing::COLUMNS_2, size: IconSize::Small }
                            span { "{columns_name}" }
                        }
                        // ITS OWN SURFACE, not `.nd-panel`. That class is the site header's
                        // sheet — `position: fixed`, full width, and its material
                        // applied in the markup — and borrowing it drew this menu
                        // as a transparent box with the table showing through.
                        div { class: "nd-table-columns-panel", id: "{columns_id}",
                            ul { class: "nd-table-columns-list", "data-nd-table-columns-list": "" }
                        }
                    }
                }
                if copy {
                    Button {
                        label: copy_label.clone().unwrap_or_default(),
                        icon: IconDrawing::COPY,
                        variant: ButtonVariant::Outline,
                        tone: ButtonTone::Neutral,
                        size: ButtonSize::Small,
                        class: "nd-table-copy",
                    }
                }
                // THE TABLE, TAKEN AWAY WITH THE READER. Four entries and one
                // menu, because four buttons in a toolbar is four buttons.
                if export {
                    details { class: "nd-table-export", "data-nd-table-export": "",
                        summary { class: "nd-table-export-summary",
                            Icon { icon: IconDrawing::DOWNLOAD, size: IconSize::Small }
                            span { {export_label.clone().unwrap_or_default()} }
                        }
                        div { class: "nd-table-export-panel",
                            ul { class: "nd-table-export-list",
                                if let Some(words) = csv_label {
                                    li {
                                        button {
                                            class: "nd-table-export-item",
                                            r#type: "button",
                                            "data-nd-table-export-as": "csv",
                                            "{words}"
                                        }
                                    }
                                }
                                if let Some(words) = json_label {
                                    li {
                                        button {
                                            class: "nd-table-export-item",
                                            r#type: "button",
                                            "data-nd-table-export-as": "json",
                                            "{words}"
                                        }
                                    }
                                }
                                if let Some(words) = copy_label.clone() {
                                    li {
                                        button {
                                            class: "nd-table-export-item",
                                            r#type: "button",
                                            "data-nd-table-export-as": "clipboard",
                                            "{words}"
                                        }
                                    }
                                }
                                if let Some(words) = print_label {
                                    li {
                                        button {
                                            class: "nd-table-export-item",
                                            r#type: "button",
                                            "data-nd-table-export-as": "print",
                                            "{words}"
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }

            div {
                class: "nd-table-scroll",
                role: "region",
                tabindex: "0",
                "aria-labelledby": "{caption_id}",
                table {
                    class: "{root}",
                    // RESTATED, and this is the point of the whole component. A
                    // `display` that is not a table display drops the implicit
                    // role in Blink, Gecko and WebKit alike.
                    role: "table",
                    "data-nd-table": "{name}",
                    "data-nd-density": density.as_str(),
                    "data-nd-rules": rules.as_str(),
                    "data-nd-zebra": if zebra { "true" } else { "false" },
                    "data-nd-hover": if hover { "true" } else { "false" },
                    "data-nd-width": width.as_str(),
                    "data-nd-layout": layout.as_str(),
                    "data-nd-sticky-header": if sticky_header { "true" } else { "false" },
                    "data-nd-height": height.as_str(),
                    "data-nd-caption-hidden": if hide_caption { "true" } else { "false" },
                    "data-nd-copy": if copy { "true" } else { "false" },
                    "data-nd-export": if export { "true" } else { "false" },
                    "data-nd-search-label": search_label,
                    "data-nd-search-placeholder": search_placeholder,
                    "data-nd-export-label": export_label,
                    "data-nd-exported-label": exported_label,
                    "data-nd-more-src": more_src,
                    "data-nd-more-label": more_label,
                    "data-nd-more-status-label": more_status_label,
                    "data-nd-total": total,
                    // Every sentence the script says, in the reader's language,
                    // because this package ships none of its own.
                    "data-nd-selected-label": selected_label,
                    "data-nd-sort-ascending-label": sort_ascending_label,
                    "data-nd-sort-descending-label": sort_descending_label,
                    "data-nd-sort-cleared-label": sort_cleared_label,
                    "data-nd-filtered-label": filtered_label,
                    "data-nd-resize-label": resize_label,
                    "data-nd-resized-label": resized_label,
                    "data-nd-columns-label": columns_label,
                    "data-nd-column-shown-label": column_shown_label,
                    "data-nd-column-hidden-label": column_hidden_label,
                    "data-nd-copy-label": copy_label,
                    "data-nd-copied-label": copied_label,
                    "data-nd-summary-label": summary_label,
                    "aria-describedby": described,
                    caption { class: "nd-table-caption",
                        span { id: "{caption_id}", "{title}" }
                        if let Some(note) = note {
                            span { class: "nd-table-note", id: "{note_id}", "{note}" }
                        }
                    }
                    {children}
                }
            }

            // MORE ROWS, AND A REAL CONTROL FOR THEM. The script presses it
            // when it scrolls into view; endless scrolling with no button is a
            // table a keyboard never reaches the end of.
            if more_label.is_some() {
                div { class: "nd-table-more", "data-nd-table-more": "", hidden: true,
                    Button {
                        label: more_label.clone().unwrap_or_default(),
                        variant: ButtonVariant::Outline,
                        tone: ButtonTone::Neutral,
                        size: ButtonSize::Small,
                        class: "nd-table-more-button",
                    }
                }
            }

            // What is showing of what there is. Drawn only when the two differ,
            // which is the script's decision because only it has filtered.
            if summary_label.is_some() {
                p { class: "nd-table-summary", "data-nd-table-summary": "", hidden: true }
            }

            // RENDERED, never built by the script. A live region has to be in
            // the document before text is written into it, or the first
            // announcement — the one that matters — is dropped.
            p {
                class: "nd-visually-hidden",
                role: "status",
                "data-nd-table-status": "",
            }

            // One row of the columns menu, cloned per hideable column. Drawn
            // here rather than in the script, so a script that draws a control
            // is never a second place that knows what a control looks like.
            template { "data-nd-table-column-template": "",
                li { class: "nd-table-columns-item",
                    label { class: "nd-table-columns-choice",
                        input { r#type: "checkbox", "data-nd-table-column-toggle": "" }
                        span { "data-nd-table-column-name": "" }
                    }
                }
            }
        }
    }
"#;

const TABLE_REACT: &str = r#"  const density_ = density ?? "default";
  const rules_ = rules ?? "rows";
  const width_ = width ?? "full";
  const layout_ = layout ?? "auto";
  const height_ = height ?? "auto";
  // `export` IS A KEYWORD, so the generator binds this prop to `_export` — see
  // `props.rs`. The prop a caller writes is still `export`; only the local
  // identifier moved, and this is it under the spelling the rest of the file
  // uses for a resolved value.
  const export_ = _export ?? false;
  const root = ["nd-table", className].filter(Boolean).join(" ");
  const captionId = `nd-table-${name}-caption`;
  const noteId = `nd-table-${name}-note`;
  const columnsId = `nd-table-${name}-columns`;
  const searchId = `nd-table-${name}-search`;
  const searchInputId = `${searchId}-input`;
  return h(
    "div",
    { className: "nd-table-frame", "data-nd-table-frame": name },
    // See the Dioxus half: rendered hidden, revealed by the script.
    h(
      "div",
      { key: "tools", className: "nd-table-tools", "data-nd-table-tools": "", hidden: true },
      // See the Dioxus half: one search, behind a glass.
      searchLabel
        ? h(
            "div",
            { key: "s", className: "nd-table-search", "data-nd-table-search": "" },
            h(
              "button",
              {
                key: "o",
                className: "nd-table-search-open",
                type: "button",
                "aria-expanded": "false",
                "aria-controls": searchId,
                "aria-label": searchLabel,
              },
              h(Icon, { icon: search, size: "small" }),
            ),
            h(
              "div",
              { key: "f", className: "nd-table-search-field", id: searchId, hidden: true },
              h(
                "span",
                { key: "g", className: "nd-table-search-glyph", "aria-hidden": "true" },
                h(Icon, { icon: search, size: "small" }),
              ),
              h("label", { key: "l", className: "nd-visually-hidden", htmlFor: searchInputId }, searchLabel),
              h("input", {
                key: "i",
                className: "nd-table-search-input",
                id: searchInputId,
                type: "search",
                "data-nd-table-search-input": "",
                autoComplete: "off",
                spellCheck: false,
                placeholder: searchPlaceholder,
              }),
            ),
          )
        : null,
      columnsLabel
        ? h(
            "details",
            { key: "c", className: "nd-table-columns", "data-nd-table-columns": "" },
            h(
              "summary",
              { key: "s", className: "nd-table-columns-summary" },
              h(Icon, { key: "i", icon: columns2, size: "small" }),
              h("span", { key: "w" }, columnsLabel),
            ),
            h(
              "div",
              { key: "p", className: "nd-table-columns-panel", id: columnsId },
              h("ul", {
                key: "l",
                className: "nd-table-columns-list",
                "data-nd-table-columns-list": "",
              }),
            ),
          )
        : null,
      copy
        ? h(Button, {
            key: "y",
            label: copyLabel ?? "",
            icon: copy,
            variant: "outline",
            tone: "neutral",
            size: "small",
            className: "nd-table-copy",
          })
        : null,
      export_
        ? h(
            "details",
            { key: "e", className: "nd-table-export", "data-nd-table-export": "" },
            h(
              "summary",
              { key: "s", className: "nd-table-export-summary" },
              h(Icon, { key: "i", icon: download, size: "small" }),
              h("span", { key: "w" }, exportLabel),
            ),
            h(
              "div",
              { key: "p", className: "nd-table-export-panel" },
              h(
                "ul",
                { key: "l", className: "nd-table-export-list" },
                [
                  ["csv", csvLabel],
                  ["json", jsonLabel],
                  ["clipboard", copyLabel],
                  ["print", printLabel],
                ]
                  .filter(([, words]) => Boolean(words))
                  .map(([as, words]) =>
                    h(
                      "li",
                      { key: as },
                      h(
                        "button",
                        {
                          className: "nd-table-export-item",
                          type: "button",
                          "data-nd-table-export-as": as,
                        },
                        words,
                      ),
                    ),
                  ),
              ),
            ),
          )
        : null,
    ),
    h(
      "div",
      {
        key: "scroll",
        className: "nd-table-scroll",
        // Focusable before anything measures it: the script's job is to take
        // the stop away on a table that does not overflow, never to add one.
        role: "region",
        tabIndex: 0,
        "aria-labelledby": captionId,
      },
      h(
        "table",
        {
          key: "t",
          className: root,
          // Restated. A non-table `display` drops the implicit role.
          role: "table",
          "data-nd-table": name,
          "data-nd-density": density_,
          "data-nd-rules": rules_,
          "data-nd-zebra": String(zebra ?? false),
          "data-nd-hover": String(hover ?? false),
          "data-nd-width": width_,
          "data-nd-layout": layout_,
          "data-nd-sticky-header": String(stickyHeader ?? false),
          "data-nd-height": height_,
          "data-nd-caption-hidden": String(hideCaption ?? false),
          "data-nd-copy": String(copy ?? false),
          "data-nd-export": String(export_),
          "data-nd-search-label": searchLabel,
          "data-nd-search-placeholder": searchPlaceholder,
          "data-nd-export-label": exportLabel,
          "data-nd-exported-label": exportedLabel,
          "data-nd-more-src": moreSrc,
          "data-nd-more-label": moreLabel,
          "data-nd-more-status-label": moreStatusLabel,
          "data-nd-total": total,
          "data-nd-selected-label": selectedLabel,
          "data-nd-sort-ascending-label": sortAscendingLabel,
          "data-nd-sort-descending-label": sortDescendingLabel,
          "data-nd-sort-cleared-label": sortClearedLabel,
          "data-nd-filtered-label": filteredLabel,
          "data-nd-resize-label": resizeLabel,
          "data-nd-resized-label": resizedLabel,
          "data-nd-columns-label": columnsLabel,
          "data-nd-column-shown-label": columnShownLabel,
          "data-nd-column-hidden-label": columnHiddenLabel,
          "data-nd-copy-label": copyLabel,
          "data-nd-copied-label": copiedLabel,
          "data-nd-summary-label": summaryLabel,
          "aria-describedby": note ? noteId : undefined,
          ...rest,
        },
        h(
          "caption",
          { key: "c", className: "nd-table-caption" },
          h("span", { key: "n", id: captionId }, caption),
          note ? h("span", { key: "d", className: "nd-table-note", id: noteId }, note) : null,
        ),
        children,
      ),
    ),
    // See the Dioxus half: a real control, pressed by the script when it
    // scrolls into view.
    moreLabel
      ? h(
          "div",
          { key: "more", className: "nd-table-more", "data-nd-table-more": "", hidden: true },
          h(Button, {
            key: "b",
            label: moreLabel,
            variant: "outline",
            tone: "neutral",
            size: "small",
            className: "nd-table-more-button",
          }),
        )
      : null,
    summaryLabel
      ? h("p", {
          key: "sum",
          className: "nd-table-summary",
          "data-nd-table-summary": "",
          hidden: true,
        })
      : null,
    h("p", {
      key: "status",
      className: "nd-visually-hidden",
      role: "status",
      "data-nd-table-status": "",
    }),
    h(
      "template",
      { key: "tpl", "data-nd-table-column-template": "" },
      h(
        "li",
        { key: "i", className: "nd-table-columns-item" },
        h(
          "label",
          { key: "l", className: "nd-table-columns-choice" },
          h("input", { key: "b", type: "checkbox", "data-nd-table-column-toggle": "" }),
          h("span", { key: "n", "data-nd-table-column-name": "" }),
        ),
      ),
    ),
  );
"#;

/// One column, as the `<colgroup>` HTML already has for the purpose.
///
/// It is written rather than derived for one reason: a `<colgroup>` may only
/// contain `<col>`, so the generated body's `{children}` would be markup the
/// parser throws away. This renders the one `<col>` a single-column group is,
/// and takes no children at all.
const TABLE_COLUMN_DIOXUS: &str = r#"    let root = format!("nd-table-column {class}");
    let root = root.trim().to_owned();

    rsx! {
        colgroup {
            class: "{root}",
            "data-nd-column": "{name}",
            "data-nd-width": width.as_str(),
            "data-nd-sortable": if sortable { "true" } else { "false" },
            "data-nd-sort-as": sort_as.as_str(),
            // The bands, and the three tones a value earns by being in one.
            "data-nd-low": low,
            "data-nd-high": high,
            "data-nd-tone-below": tone_below,
            "data-nd-tone-within": tone_within,
            "data-nd-tone-above": tone_above,
            "data-nd-filterable": if filterable { "true" } else { "false" },
            "data-nd-resizable": if resizable { "true" } else { "false" },
            "data-nd-hideable": if hideable { "true" } else { "false" },
            "data-nd-hidden": if hidden { "true" } else { "false" },
            "data-nd-pinned": if pinned { "true" } else { "false" },
            // A `<colgroup>` with one `<col>` in it is one column, which is what
            // this is. Written out rather than left implicit so the resize
            // handle has an element with a width to write on.
            col { class: "nd-table-column-box" }
        }
    }
"#;

const TABLE_COLUMN_REACT: &str = r#"  const width_ = width ?? "auto";
  const sortAs_ = sortAs ?? "text";
  const root = ["nd-table-column", className].filter(Boolean).join(" ");
  return h(
    "colgroup",
    {
      className: root,
      "data-nd-column": name,
      "data-nd-width": width_,
      "data-nd-sortable": String(sortable ?? false),
      "data-nd-sort-as": sortAs_,
      // See the Dioxus half: the bands, and the tones a value earns.
      "data-nd-low": low,
      "data-nd-high": high,
      "data-nd-tone-below": toneBelow,
      "data-nd-tone-within": toneWithin,
      "data-nd-tone-above": toneAbove,
      "data-nd-filterable": String(filterable ?? false),
      "data-nd-resizable": String(resizable ?? false),
      "data-nd-hideable": String(hideable ?? false),
      "data-nd-hidden": String(hidden ?? false),
      "data-nd-pinned": String(pinned ?? false),
      ...rest,
    },
    // See the Dioxus half: one `<col>`, because one group of one is one column.
    h("col", { key: "c", className: "nd-table-column-box" }),
  );
"#;

/// One record, and the two leading cells nobody should wire by hand.
///
/// # Why a detail row is a row of its own
///
/// The tempting shape is `ListItem`'s — one component rendering a `<tr>` and
/// the `<tr>` that opens under it. It is impossible: the row's cells are
/// `children`, the detail's content would be a second `children`, and at most
/// one prop may be that. So a detail names the row it belongs to and both ids
/// come out of that one identifier, which is `Checkbox { name, value }` again.
///
/// # Why the choice is declared here and not on the table
///
/// HTML puts a cell inside a row. A table cannot reach in and add one, and a
/// script that did would be a script the form stops working without.
const TABLE_ROW_DIOXUS: &str = r#"    let root = format!("nd-table-row {class}");
    let root = root.trim().to_owned();
    // Both sides of the pair, from one identifier, so the `aria-controls` and
    // the row it names cannot disagree.
    let id = detail_for.as_ref().map(|owner| format!("nd-row-{owner}-detail"));
    let controls = name.as_ref().map(|name| format!("nd-row-{name}-detail"));
    let field = select_name.clone().unwrap_or_default();
    let value = name.clone().unwrap_or_default();
    let choice = select_label.clone().unwrap_or_default();

    rsx! {
        tr {
            class: "{root}",
            // Restated, so the row survives a stacked layout.
            role: "row",
            id,
            "data-nd-row": name,
            "data-nd-tone": tone,
            "data-nd-emphasis": emphasis.as_str(),
            "data-nd-select": select.as_str(),
            "data-nd-selected": if selected { "true" } else { "false" },
            "data-nd-disabled": if disabled { "true" } else { "false" },
            "data-nd-detail-for": detail_for,
            // AND IT IS READ. `table.js` asks the table whether any row opens
            // — which is half of what makes the table a `role="grid"` — with
            // `[data-nd-expandable='true']`, and nothing wrote it. Every table
            // of expandable rows on the site was a plain table.
            "data-nd-expandable": if expandable { "true" } else { "false" },
            "data-nd-empty": if empty { "true" } else { "false" },
            // THE CHOICE CELL, which is a `<td>` in the body and a `<th>` in
            // the head. The row cannot see which section it is in, so the value
            // says which — and the two cells are not interchangeable: a heading
            // that is a `<td>` heads nothing.
            if select == TableRowSelect::Many {
                td { class: "nd-table-cell nd-table-select", role: "cell",
                    Checkbox {
                        name: field.clone(),
                        value: value.clone(),
                        label: choice.clone(),
                        hide_label: true,
                        size: CheckboxSize::Small,
                        checked: selected,
                        disabled,
                    }
                }
            } else if select == TableRowSelect::One {
                td { class: "nd-table-cell nd-table-select", role: "cell",
                    Radio {
                        name: field.clone(),
                        value: value.clone(),
                        label: choice.clone(),
                        hide_label: true,
                        size: RadioSize::Small,
                        checked: selected,
                        disabled,
                    }
                }
            } else if select == TableRowSelect::All {
                th {
                    class: "nd-table-header-cell nd-table-select nd-table-select-all",
                    role: "columnheader",
                    scope: "col",
                    Checkbox {
                        name: field,
                        value: "all",
                        label: choice,
                        hide_label: true,
                        size: CheckboxSize::Small,
                    }
                }
            }
            // The toggle for the detail row. A real button with `aria-controls`
            // and never a handler on the `<tr>`, which is not focusable,
            // announces nothing actionable and answers no key.
            if expandable {
                td { class: "nd-table-cell nd-table-expand", role: "cell",
                    Button {
                        label: expand_label.unwrap_or_default(),
                        hide_label: true,
                        icon: IconDrawing::CHEVRON_DOWN,
                        variant: ButtonVariant::Transparent,
                        size: ButtonSize::Small,
                        controls,
                        class: "nd-table-expand-button",
                    }
                }
            }
            {children}
        }
    }
"#;

const TABLE_ROW_REACT: &str = r#"  const tone_ = tone ?? "default";
  const select_ = select ?? "none";
  const root = ["nd-table-row", className].filter(Boolean).join(" ");
  const controls = name ? `nd-row-${name}-detail` : undefined;
  const box = {
    name: selectName ?? "",
    label: selectLabel ?? "",
    hideLabel: true,
    size: "small",
  };
  return h(
    "tr",
    {
      className: root,
      role: "row",
      id: detailFor ? `nd-row-${detailFor}-detail` : undefined,
      "data-nd-row": name,
      "data-nd-tone": tone,
      "data-nd-emphasis": emphasis ?? "none",
      "data-nd-select": select_,
      "data-nd-selected": String(selected ?? false),
      "data-nd-disabled": String(disabled ?? false),
      "data-nd-detail-for": detailFor,
      // See the Dioxus half: `table.js` reads this to derive `role="grid"`.
      "data-nd-expandable": String(expandable ?? false),
      "data-nd-empty": String(empty ?? false),
      ...rest,
    },
    // A `<td>` in the body and a `<th>` in the head: a heading that is a `<td>`
    // heads nothing, and the row cannot see which section it is in.
    select_ === "many"
      ? h(
          "td",
          { key: "s", className: "nd-table-cell nd-table-select", role: "cell" },
          h(Checkbox, {
            ...box,
            value: name ?? "",
            checked: selected ?? false,
            disabled: disabled ?? false,
          }),
        )
      : select_ === "one"
        ? h(
            "td",
            { key: "s", className: "nd-table-cell nd-table-select", role: "cell" },
            h(Radio, {
              ...box,
              value: name ?? "",
              checked: selected ?? false,
              disabled: disabled ?? false,
            }),
          )
        : select_ === "all"
          ? h(
              "th",
              {
                key: "s",
                className: "nd-table-header-cell nd-table-select nd-table-select-all",
                role: "columnheader",
                scope: "col",
              },
              h(Checkbox, { ...box, value: "all" }),
            )
          : null,
    expandable
      ? h(
          "td",
          { key: "x", className: "nd-table-cell nd-table-expand", role: "cell" },
          h(Button, {
            label: expandLabel ?? "",
            hideLabel: true,
            icon: chevronDown,
            variant: "transparent",
            size: "small",
            controls,
            className: "nd-table-expand-button",
          }),
        )
      : null,
    children,
  );
"#;

/// One value, and the column name the stacked layout draws beside it.
///
/// `colspan` and `rowspan` are omitted rather than written empty, which is one
/// reason this body is written rather than derived: `colspan=""` is a number
/// the parser reads as one and a reader reads as a mistake. The other is the
/// stacked label, which is real text and `aria-hidden` — a screen reader
/// already gets the column from the heading.
const TABLE_CELL_DIOXUS: &str = r#"    let root = format!("nd-table-cell {class}");
    let root = root.trim().to_owned();

    rsx! {
        td {
            class: "{root}",
            // Restated. A `<td>` whose implicit role has been dropped is a
            // generic child, and every column index after it in the row shifts.
            role: "cell",
            colspan: span,
            rowspan: rows,
            "data-nd-align": align.as_str(),
            "data-nd-tone": tone,
            "data-nd-wrap": wrap.as_str(),
            "data-nd-column": column,
            "data-nd-sort-value": sort_value,
            "data-nd-aggregate": aggregate.as_str(),
            "data-nd-aggregate-over": aggregate_over.as_str(),
            if let Some(label) = label {
                // DRAWN, AND HIDDEN FROM THE SCREEN READER. The stacked layout
                // has no heading row left, so the value carries its own name —
                // and hearing it again after the heading is that name twice.
                span { class: "nd-table-label", "aria-hidden": "true", "{label}" }
            }
            // A `<div>`, NOT A `<span>`. A cell may hold a `Meter`, a `Tag`, an
            // `Info` or a table of its own, and flow content inside phrasing
            // content is invalid — it renders, and it is markup nobody can
            // defend.
            div { class: "nd-table-value", {children} }
        }
    }
"#;

const TABLE_CELL_REACT: &str = r#"  const align_ = align ?? "start";
  const wrap_ = wrap ?? "wrap";
  const aggregate_ = aggregate ?? "none";
  const root = ["nd-table-cell", className].filter(Boolean).join(" ");
  return h(
    "td",
    {
      className: root,
      role: "cell",
      colSpan: span,
      rowSpan: rows,
      "data-nd-align": align_,
      "data-nd-tone": tone,
      "data-nd-wrap": wrap_,
      "data-nd-column": column,
      "data-nd-sort-value": sortValue,
      "data-nd-aggregate": aggregate_,
      "data-nd-aggregate-over": aggregateOver ?? "table",
      ...rest,
    },
    // See the Dioxus half: drawn for the stacked layout, hidden from a screen
    // reader that already has the column from its heading.
    label ? h("span", { key: "l", className: "nd-table-label", "aria-hidden": "true" }, label) : null,
    // See the Dioxus half: a `<div>`, so any component may live in a cell.
    h("div", { key: "v", className: "nd-table-value" }, children),
  );
"#;

/// The words that name a column, and the controls its column allows.
///
/// The sort control is a real `<button>` inside the heading rather than a
/// handler on the `<th>`, which is not focusable, announces nothing actionable
/// and answers no key. The filter box and the resize handle are added by the
/// script, because a control that cannot work without one should not be in the
/// markup claiming it can.
///
/// # Why `aria-sort` is absent rather than `"none"`
///
/// `aria-sort` describes a column a reader can sort by. Written on every
/// heading it says that of the record names down the leading edge as well —
/// 403 of them on one page for 13 sortable columns, 214 on `scope="row"`
/// headings that name a record and can never be sorted by. Sortability is
/// declared on the `TableColumn`, which this component cannot see, so the
/// server writes the attribute for the one case it does know: a heading the
/// caller stated a direction on, which is a column somebody sorted. The
/// runtime adds `aria-sort="none"` to the rest as it draws each sort control,
/// so the attribute and the control arrive together.
const TABLE_HEADER_CELL_DIOXUS: &str = r#"    let root = format!("nd-table-header-cell {class}");
    let root = root.trim().to_owned();
    // A DIRECTION, OR NOTHING. `None` here means "say nothing", not
    // `aria-sort="none"` — see the emitter's note above.
    let sorted = match sort {
        TableHeaderCellSort::None => None,
        TableHeaderCellSort::Ascending => Some("ascending"),
        TableHeaderCellSort::Descending => Some("descending"),
    };
    // A heading that names a record or a band of rows is not a column, so no
    // direction it was given can be true of it.
    let row_scope = matches!(
        scope,
        TableHeaderCellScope::Row | TableHeaderCellScope::Rowgroup
    );
    let sorted = if row_scope { None } else { sorted };

    rsx! {
        th {
            class: "{root}",
            // `columnheader` or `rowheader`, following `scope` — restated, so
            // a stacked table does not lose what every cell was named by.
            role: if scope == TableHeaderCellScope::Row { "rowheader" } else { "columnheader" },
            scope: scope.as_str(),
            colspan: span,
            rowspan: rows,
            "aria-sort": sorted,
            "data-nd-column": column,
            "data-nd-align": align.as_str(),
            "data-nd-tone": tone,
            "data-nd-wrap": wrap.as_str(),
            div { class: "nd-table-heading",
                span { class: "nd-table-heading-words", "data-nd-table-heading": "", {children} }
            }
        }
    }
"#;

const TABLE_HEADER_CELL_REACT: &str = r#"  const scope_ = scope ?? "col";
  const align_ = align ?? "start";
  // `nowrap`, THE OPPOSITE OF A CELL'S DEFAULT, which is the whole point of the
  // prop's doc: the head stays one line tall. This said `wrap`, so every column
  // heading in a React table wrapped where a Dioxus one did not.
  const wrap_ = wrap ?? "nowrap";
  // See the Dioxus half: a direction, or nothing. `undefined` is how React
  // leaves an attribute off, and `aria-sort="none"` on a heading nobody can
  // sort by announces a control that is not there.
  const sort_ = sort ?? "none";
  const sorted =
    scope_ === "row" || scope_ === "rowgroup" || sort_ === "none" ? undefined : sort_;
  const root = ["nd-table-header-cell", className].filter(Boolean).join(" ");
  return h(
    "th",
    {
      className: root,
      // See the Dioxus half: the role follows `scope`.
      role: scope_ === "row" ? "rowheader" : "columnheader",
      scope: scope_,
      colSpan: span,
      rowSpan: rows,
      "aria-sort": sorted,
      "data-nd-column": column,
      "data-nd-align": align_,
      "data-nd-tone": tone,
      "data-nd-wrap": wrap_,
      ...rest,
    },
    h(
      "div",
      { key: "h", className: "nd-table-heading" },
      h(
        "span",
        { key: "w", className: "nd-table-heading-words", "data-nd-table-heading": "" },
        children,
      ),
    ),
  );
"#;

// --- Info -------------------------------------------------------------------

/// The small `i`, and the note it opens.
///
/// # Why the note is rendered rather than built
///
/// A live region's content has to be in the document before it is written into,
/// and a note is content a reader chose to open — building it on the first
/// press is a note some engines never announce. It is rendered hidden and the
/// script reveals the control, which is `Carousel`'s rule: a control that does
/// nothing is worse than no control.
const INFO_DIOXUS: &str = r#"    let root = format!("nd-info {class}");
    let root = root.trim().to_owned();
    let id = format!("nd-info-{name}");
    let note_id = format!("{id}-note");
    let title_id = format!("{id}-title");
    let described = heading.as_ref().map(|_| title_id.clone());

    rsx! {
        span {
            class: "{root}",
            "data-nd-info": "{name}",
            "data-nd-placement": placement.as_str(),
            "data-nd-close-label": close_label,
            button {
                class: "nd-info-marker",
                id: "{id}",
                r#type: "button",
                "aria-label": "{label}",
                "aria-expanded": "false",
                "aria-controls": "{note_id}",
                // RENDERED HIDDEN. Revealed by the script, because a marker
                // that opens nothing is worse than no marker.
                hidden: true,
                Icon { icon: IconDrawing::INFO, size: IconSize::Small }
            }
            span {
                class: "nd-info-note",
                id: "{note_id}",
                role: "note",
                "aria-labelledby": described,
                hidden: true,
                if let Some(heading) = heading {
                    span { class: "nd-info-title", id: "{title_id}", "{heading}" }
                }
                span { class: "nd-info-body", {children} }
            }
        }
    }
"#;

const INFO_REACT: &str = r#"  const placement_ = placement ?? "auto";
  const root = ["nd-info", className].filter(Boolean).join(" ");
  const id = `nd-info-${name}`;
  const noteId = `${id}-note`;
  const titleId = `${id}-title`;
  return h(
    "span",
    {
      className: root,
      "data-nd-info": name,
      "data-nd-placement": placement_,
      "data-nd-close-label": closeLabel,
      ...rest,
    },
    h(
      "button",
      {
        key: "b",
        className: "nd-info-marker",
        id,
        type: "button",
        "aria-label": label,
        "aria-expanded": "false",
        "aria-controls": noteId,
        // See the Dioxus half: revealed by the script.
        hidden: true,
      },
      h(Icon, { icon: info, size: "small" }),
    ),
    h(
      "span",
      {
        key: "n",
        className: "nd-info-note",
        id: noteId,
        role: "note",
        "aria-labelledby": heading ? titleId : undefined,
        hidden: true,
      },
      heading ? h("span", { key: "t", className: "nd-info-title", id: titleId }, heading) : null,
      h("span", { key: "y", className: "nd-info-body" }, children),
    ),
  );
"#;

// --- Menu and MenuItem ------------------------------------------------------

/// The surface a button opens, and the actions on it.
///
/// # Why the id is the caller's own word
///
/// A button ties itself to a menu with `aria-controls`, which takes an id — so
/// the two have to agree on one string. `Info` derives its ids from `name`
/// because nothing outside it ever needs to say them; here the caller writes
/// `controls` on one component and `name` on the other, and a derivation they
/// cannot see is a derivation they will get wrong. So `name` IS the id.
///
/// # Why it renders hidden
///
/// `Info`'s reason: with no script there is no control and no surface, and a
/// menu a reader can see but not close is worse than a menu that never opened.
const MENU_DIOXUS: &str = r#"    let root = format!("nd-menu {class}");
    let root = root.trim().to_owned();

    rsx! {
        div {
            class: "{root}",
            id: "{name}",
            role: "menu",
            "aria-label": "{label}",
            "data-nd-menu": "{name}",
            "data-nd-placement": placement.as_str(),
            hidden: true,
            // A SCROLLER INSIDE THE SURFACE, BECAUSE A SCROLLABLE ELEMENT
            // CARRIES NO RADIUS. A menu longer than the window has to scroll,
            // and a rounded box that scrolls paints a square scrollbar through
            // its own corner — `.nd-panel` + `.nd-panel-body` and
            // `.nd-codeblock` + `.nd-codeblock-code` are the same split for
            // the same reason. The surface clips; this is what moves.
            //
            // `role="none"` so the items are still the menu's own children in
            // the accessibility tree. A generic box between a `role="menu"`
            // and its `role="menuitem"` items is the `<ul role="menu"><li
            // role="none">` arrangement, said with the elements this
            // component already renders.
            div { class: "nd-menu-scroll", role: "none",
                {children}
            }
        }
    }
"#;

const MENU_REACT: &str = r#"  const placement_ = placement ?? "auto";
  const root = ["nd-menu", className].filter(Boolean).join(" ");
  return h(
    "div",
    {
      className: root,
      id: name,
      role: "menu",
      "aria-label": label,
      "data-nd-menu": name,
      "data-nd-placement": placement_,
      // See the Dioxus half: revealed by the script, never by the markup.
      hidden: true,
      ...rest,
    },
    // See the Dioxus half: the surface clips and this scrolls, because a
    // scrollable element carries no radius.
    h("div", { key: "s", className: "nd-menu-scroll", role: "none" }, children),
  );
"#;

/// One action, as a real button or a real link.
///
/// # Why `aria-disabled` and not `disabled`
///
/// The contract says it at length: a disabled `<button>` leaves the tab order,
/// which in a menu means the arrow walk skips it — and an item that is skipped
/// is an action a reader concludes does not exist. Refused and reachable is the
/// honest pair, and it is the one place this disagrees with `Button`.
const MENU_ITEM_DIOXUS: &str = r#"    let root = format!("nd-menu-item {class}");
    let root = root.trim().to_owned();
    let refused = disabled.then_some("true");
    // A relationship the caller did not state, for a window they did open. The
    // rule `Button` keeps, kept the same way rather than a second way.
    let relation = rel.or_else(|| target.as_ref().map(|_| "noopener noreferrer".to_owned()));

    // Built once and used by whichever element this turns out to be, so the two
    // branches cannot come to draw different things.
    let content = rsx! {
        if let Some(icon) = icon {
            Icon { icon, size: IconSize::Small }
        }
        span { class: "nd-menu-item-text",
            span { class: "nd-menu-item-label", "{label}" }
            if let Some(detail) = detail.clone() {
                span { class: "nd-menu-item-detail", "{detail}" }
            }
        }
    };

    rsx! {
        if let Some(href) = href.clone() {
            a {
                class: "{root}",
                role: "menuitem",
                tabindex: "-1",
                href: "{href}",
                rel: relation,
                target,
                "data-nd-action": action,
                "data-nd-value": value,
                "data-nd-tone": tone,
                "aria-disabled": refused,
                {content}
            }
        } else {
            button {
                class: "{root}",
                r#type: "button",
                role: "menuitem",
                tabindex: "-1",
                "data-nd-action": action,
                "data-nd-value": value,
                "data-nd-tone": tone,
                "aria-disabled": refused,
                {content}
            }
        }
    }
"#;

const MENU_ITEM_REACT: &str = r#"  const root = ["nd-menu-item", className].filter(Boolean).join(" ");
  const relation = rel ?? (target ? "noopener noreferrer" : undefined);
  const inside = [
    icon ? h(Icon, { key: "i", icon, size: "small" }) : null,
    h(
      "span",
      { key: "t", className: "nd-menu-item-text" },
      h("span", { key: "l", className: "nd-menu-item-label" }, label),
      detail ? h("span", { key: "d", className: "nd-menu-item-detail" }, detail) : null,
    ),
  ];
  const shared = {
    className: root,
    role: "menuitem",
    tabIndex: -1,
    "data-nd-action": action,
    "data-nd-value": value,
    "data-nd-tone": tone,
    "aria-disabled": disabled ? "true" : undefined,
    ...rest,
  };
  return href
    ? h("a", { ...shared, href, rel: relation, target }, inside)
    : h("button", { ...shared, type: "button" }, inside);
"#;

// --- Meter and Progress -----------------------------------------------------

/// A reading inside a range, as the element HTML already has for it.
///
/// # Why the value sentence is computed here and not by a script
///
/// `{percent}` is arithmetic over three numbers a caller already gave, so it is
/// done where they are — at render, in both languages, once. A script filling
/// it would be a bar that says nothing until a script arrives, on a component
/// that otherwise needs none at all.
///
/// # Why the band is written into the markup
///
/// The platform paints from `low`/`high`/`optimum` and tells nobody which band
/// it chose. `data-nd-level` is the same comparison, made visible — so a test
/// can assert what a reader is seeing and a caller can hang a word beside the
/// bar without doing the arithmetic again.
const METER_DIOXUS: &str = r#"    let root = format!("nd-meter {class}");
    let root = root.trim().to_owned();
    let id = format!("nd-meter-{name}");
    let reading: f64 = value.parse().unwrap_or_default();
    let floor: f64 = min.as_deref().and_then(|one| one.parse().ok()).unwrap_or(0.0);
    let ceiling: f64 = max.as_deref().and_then(|one| one.parse().ok()).unwrap_or(1.0);
    let span = ceiling - floor;
    let share = if span.abs() < f64::EPSILON {
        0.0
    } else {
        ((reading - floor) / span).clamp(0.0, 1.0)
    };
    // Rounded to whole percent: a bar reading 82.4% is a bar whose sentence
    // says something the eye cannot check against it.
    // No cast: the share is clamped to 0..=1, so this is 0..=100 with one
    // decimal place asked for and none kept — which is the same number without
    // a lint about a truncation that cannot happen.
    let percent = format!("{:.0}%", share * 100.0);
    // The same comparison the platform makes, said out loud.
    let below = low.as_deref().and_then(|one| one.parse::<f64>().ok());
    let above = high.as_deref().and_then(|one| one.parse::<f64>().ok());
    // WHERE IN THE RANGE, NOT WHETHER IT IS GOOD. Which of the three the
    // platform paints depends on where `optimum` sits, and a value between the
    // thresholds is not necessarily the best one — on a disk it is the middle
    // of a range whose good end is empty.
    let level = match (below, above) {
        (Some(low), _) if reading < low => "low",
        (_, Some(high)) if reading > high => "high",
        _ => "middle",
    };
    let sentence = value_label.as_ref().map(|words| {
        words
            .replace("{value}", &value)
            .replace("{max}", max.as_deref().unwrap_or("1"))
            .replace("{unit}", unit.as_deref().unwrap_or(""))
            .replace("{percent}", &percent)
    });

    rsx! {
        div {
            class: "{root}",
            "data-nd-meter": "{name}",
            "data-nd-size": size.as_str(),
            "data-nd-tone": tone,
            "data-nd-unit": unit,
            "data-nd-level": "{level}",
            "data-nd-label-hidden": if hide_label { "true" } else { "false" },
            "data-nd-value-shown": if show_value { "true" } else { "false" },
            div { class: "nd-meter-line",
                label { class: "nd-meter-label", r#for: "{id}", "{label}" }
                if show_value {
                    if let Some(sentence) = sentence.clone() {
                        span { class: "nd-meter-value", "{sentence}" }
                    }
                }
            }
            meter {
                class: "nd-meter-bar",
                id: "{id}",
                value: "{value}",
                min,
                max,
                low,
                high,
                optimum,
                // THE CALLER'S SENTENCE, so what is heard is what is read. A
                // meter with none is announced as a bare fraction.
                "aria-valuetext": sentence,
                // What a browser too old for the element shows instead.
                "{percent}"
            }
        }
    }
"#;

const METER_REACT: &str = r#"  const size_ = size ?? "medium";
  const root = ["nd-meter", className].filter(Boolean).join(" ");
  const id = `nd-meter-${name}`;
  const reading = Number.parseFloat(value) || 0;
  const floor = Number.parseFloat(min ?? "0") || 0;
  const ceiling = Number.parseFloat(max ?? "1");
  const span = ceiling - floor;
  const share = span === 0 ? 0 : Math.min(1, Math.max(0, (reading - floor) / span));
  const percent = `${Math.round(share * 100)}%`;
  // See the Dioxus half: the platform paints from the thresholds and tells
  // nobody which band it chose.
  const below = low === undefined ? null : Number.parseFloat(low);
  const above = high === undefined ? null : Number.parseFloat(high);
  // See the Dioxus half: where in the range, not whether it is good.
  const level =
    below !== null && reading < below ? "low" : above !== null && reading > above ? "high" : "middle";
  const sentence = valueLabel
    ? valueLabel
        .split("{value}").join(value)
        .split("{max}").join(max ?? "1")
        .split("{unit}").join(unit ?? "")
        .split("{percent}").join(percent)
    : undefined;
  return h(
    "div",
    {
      className: root,
      "data-nd-meter": name,
      "data-nd-size": size_,
      "data-nd-tone": tone,
      "data-nd-unit": unit,
      "data-nd-level": level,
      "data-nd-label-hidden": String(hideLabel ?? false),
      "data-nd-value-shown": String(showValue ?? true),
      ...rest,
    },
    h(
      "div",
      { key: "l", className: "nd-meter-line" },
      h("label", { key: "n", className: "nd-meter-label", htmlFor: id }, label),
      (showValue ?? true) && sentence
        ? h("span", { key: "v", className: "nd-meter-value" }, sentence)
        : null,
    ),
    h(
      "meter",
      {
        key: "m",
        className: "nd-meter-bar",
        id,
        value,
        min,
        max,
        low,
        high,
        optimum,
        "aria-valuetext": sentence,
      },
      percent,
    ),
  );
"#;

/// A task advancing, and the indeterminate bar an absent value is.
///
/// `value` is passed through rather than defaulted, because `undefined` is the
/// state: `<progress>` with no `value` is the platform's own indeterminate
/// animation, which obeys a reader's motion setting because it is the
/// browser's rather than ours.
const PROGRESS_DIOXUS: &str = r#"    let root = format!("nd-progress {class}");
    let root = root.trim().to_owned();
    let id = format!("nd-progress-{name}");
    let busy = value.is_none();
    let ceiling: f64 = max.as_deref().and_then(|one| one.parse().ok()).unwrap_or(1.0);
    let percent = value.as_deref().and_then(|one| one.parse::<f64>().ok()).map(|reading| {
        let share = if ceiling.abs() < f64::EPSILON {
            0.0
        } else {
            (reading / ceiling).clamp(0.0, 1.0)
        };
        format!("{:.0}%", share * 100.0)
    });
    let sentence = value_label
        .as_ref()
        .filter(|_| !busy)
        .map(|words| {
            words
                .replace("{value}", value.as_deref().unwrap_or(""))
                .replace("{max}", max.as_deref().unwrap_or("1"))
                .replace("{percent}", percent.as_deref().unwrap_or(""))
        })
        .or_else(|| busy.then(|| busy_label.clone().unwrap_or_default()).filter(|one| !one.is_empty()));

    rsx! {
        div {
            class: "{root}",
            "data-nd-progress": "{name}",
            "data-nd-size": size.as_str(),
            "data-nd-tone": tone,
            "data-nd-busy": busy.then_some(""),
            "data-nd-label-hidden": if hide_label { "true" } else { "false" },
            div { class: "nd-progress-line",
                label { class: "nd-progress-label", r#for: "{id}", "{label}" }
                // THE CALLER'S SENTENCE, not the bare percent: what is read
                // and what is heard have to be the same words, and *7.4 of 12
                // MB* is what somebody watching an upload wants.
                if let Some(sentence) = sentence.clone() {
                    span { class: "nd-progress-value", "{sentence}" }
                }
            }
            progress {
                class: "nd-progress-bar",
                id: "{id}",
                // ABSENT, NOT ZERO. An empty attribute is a bar at nothing;
                // no attribute at all is the indeterminate one.
                value,
                max,
                "aria-valuetext": sentence,
                if let Some(percent) = percent { "{percent}" }
            }
        }
    }
"#;

const PROGRESS_REACT: &str = r#"  const size_ = size ?? "medium";
  const root = ["nd-progress", className].filter(Boolean).join(" ");
  const id = `nd-progress-${name}`;
  const busy = value === undefined || value === null;
  const ceiling = Number.parseFloat(max ?? "1");
  const percent = busy
    ? undefined
    : `${Math.round(Math.min(1, Math.max(0, (Number.parseFloat(value) || 0) / (ceiling || 1))) * 100)}%`;
  const sentence = busy
    ? busyLabel || undefined
    : valueLabel
      ? valueLabel
          .split("{value}").join(value)
          .split("{max}").join(max ?? "1")
          .split("{percent}").join(percent ?? "")
      : undefined;
  return h(
    "div",
    {
      className: root,
      "data-nd-progress": name,
      "data-nd-size": size_,
      "data-nd-tone": tone,
      "data-nd-busy": busy ? "" : undefined,
      "data-nd-label-hidden": String(hideLabel ?? false),
      ...rest,
    },
    h(
      "div",
      { key: "l", className: "nd-progress-line" },
      h("label", { key: "n", className: "nd-progress-label", htmlFor: id }, label),
      // See the Dioxus half: the sentence, not the bare percent.
      sentence ? h("span", { key: "v", className: "nd-progress-value" }, sentence) : null,
    ),
    // See the Dioxus half: absent, not zero — no attribute is the
    // indeterminate bar.
    h("progress", { key: "p", className: "nd-progress-bar", id, value, max, "aria-valuetext": sentence }, percent),
  );
"#;

/// One band, and the heading a collapsible group hangs from.
///
/// Written rather than derived because `kind` switches the element AND a
/// collapsible group needs a row of its own above its rows — which a generic
/// body has no way to put there.
const TABLE_SECTION_DIOXUS: &str = r#"    let root = format!("nd-table-section {class}");
    let root = root.trim().to_owned();
    let id = name.as_ref().map(|name| format!("nd-table-group-{name}"));
    let group = label.clone();

    // THE GROUP HEADING IS A ROW, because everything inside a `<tbody>` is. It
    // spans the table with a `colspan` nothing has to keep in step, because the
    // browser's own default for a lone cell in a row is to take the width.
    let heading = rsx! {
        if let Some(group) = group {
            tr {
                class: "nd-table-row nd-table-group",
                role: "row",
                "data-nd-table-group": name.clone(),
                th {
                    class: "nd-table-header-cell nd-table-group-cell",
                    role: "columnheader",
                    scope: "colgroup",
                    span { class: "nd-table-group-label", "{group}" }
                }
            }
        }
    };

    match kind {
        TableSectionKind::Head => rsx! {
            thead {
                class: "{root}",
                role: "rowgroup",
                "data-nd-section": name,
                {children}
            }
        },
        TableSectionKind::Foot => rsx! {
            tfoot {
                class: "{root}",
                role: "rowgroup",
                "data-nd-section": name,
                {children}
            }
        },
        TableSectionKind::Body => rsx! {
            tbody {
                class: "{root}",
                role: "rowgroup",
                id,
                "data-nd-section": name,
                "data-nd-collapsible": if collapsible { "true" } else { "false" },
                {heading}
                {children}
            }
        },
    }
"#;

const TABLE_SECTION_REACT: &str = r#"  const kind_ = kind ?? "body";
  const root = ["nd-table-section", className].filter(Boolean).join(" ");
  const element = kind_ === "head" ? "thead" : kind_ === "foot" ? "tfoot" : "tbody";
  // See the Dioxus half: a group's heading is a row, because everything inside
  // a `<tbody>` is.
  const heading = label
    ? h(
        "tr",
        {
          key: "g",
          className: "nd-table-row nd-table-group",
          role: "row",
          "data-nd-table-group": name,
        },
        h(
          "th",
          {
            key: "c",
            className: "nd-table-header-cell nd-table-group-cell",
            role: "columnheader",
            scope: "colgroup",
          },
          h("span", { key: "l", className: "nd-table-group-label" }, label),
        ),
      )
    : null;
  return h(
    element,
    {
      className: root,
      role: "rowgroup",
      id: name && kind_ === "body" ? `nd-table-group-${name}` : undefined,
      "data-nd-section": name,
      "data-nd-collapsible": kind_ === "body" ? String(collapsible ?? false) : undefined,
      ...rest,
    },
    kind_ === "body" ? heading : null,
    children,
  );
"#;

// --- Pagination -------------------------------------------------------------

/// The way through a set of pages, as real links.
///
/// # The window is the one piece of arithmetic in this file
///
/// Everything else here is markup with holes in it. This decides which numbers
/// to draw: a run around the page in force, the first and the last always
/// reachable, and a gap character where numbers were left out. It is written
/// twice on purpose — once in each language, side by side — because that is
/// what makes "the two implementations agree" a thing a reader can check
/// rather than a thing a test hopes for.
///
/// # Links, not buttons
///
/// A page is a URL. A reader will try to open one in a new tab, bookmark it and
/// find it again in their history, and a `<button>` with a handler offers none
/// of that. So every page is an `<a href>` built from one template — which is
/// also why nothing here needs a script.
const PAGINATION_DIOXUS: &str = r#"    let root = format!("nd-pagination {class}");
    let root = root.trim().to_owned();
    // Clamped rather than trusted. A control that reads `page 12 of 9` has
    // already lost the reader, and the two values arrive as text because every
    // prop that crosses the two languages does.
    let last = pages.parse::<u32>().unwrap_or(1).max(1);
    let current = page.parse::<u32>().unwrap_or(1).clamp(1, last);
    let siblings = match spread {
        PaginationSpread::Compact => 1,
        PaginationSpread::Default => 2,
        PaginationSpread::Wide => 3,
    };
    let from = current.saturating_sub(siblings).max(1);
    let to = (current + siblings).min(last);
    // `None` is the gap. Both ends are always in the list: a reader who wants
    // the end of a log should not have to walk there.
    let mut window: Vec<Option<u32>> = Vec::new();
    if from > 1 {
        window.push(Some(1));
    }
    if from > 2 {
        window.push(None);
    }
    for number in from..=to {
        window.push(Some(number));
    }
    if to + 1 < last {
        window.push(None);
    }
    if to < last {
        window.push(Some(last));
    }
    let address = |number: u32| href.replace("{page}", &number.to_string());
    let named = |template: &Option<String>, number: u32| {
        template
            .as_ref()
            .map(|words| words.replace("{page}", &number.to_string()))
    };
    // WHICH PAGES A GAP STANDS FOR, AND THE INDEX IS WHAT SAYS WHICH GAP IT IS.
    //
    // A leading gap can only ever sit at index 1: page one is pushed first
    // whenever `from > 1`, and `from > 2` is the only thing that puts a gap
    // after it. So every other gap is the trailing one, and no second list has
    // to be carried alongside the window to tell them apart.
    let hidden = |index: usize| {
        if index == 1 {
            (2, from - 1)
        } else {
            (to + 1, last - 1)
        }
    };

    rsx! {
        nav {
            class: "{root}",
            "aria-label": "{label}",
            "data-nd-pagination": "{name}",
            "data-nd-page": "{current}",
            "data-nd-pages": "{last}",
            "data-nd-spread": spread.as_str(),
            "data-nd-size": size.as_str(),
            ul { class: "nd-pagination-list",
                // Absent rather than disabled at the ends: a link to nowhere is
                // a control a keyboard still stops at and a reader still tries.
                if let Some(previous_label) = previous_label {
                    if current > 1 {
                        li {
                            a {
                                class: "nd-pagination-link nd-pagination-step",
                                href: address(current - 1),
                                rel: "prev",
                                "{previous_label}"
                            }
                        }
                    }
                }
                for (index, spot) in window.iter().enumerate() {
                    li { key: "{index}",
                        // A number, or the gap between two runs of them.
                        //
                        // A `match` and not an `if let`, which clippy's pedantic
                        // `single_match_else` would prefer: the gap arm needs three
                        // `let` bindings, and `rsx!`'s own `if let` takes rsx content
                        // rather than statements. The allow rides on the component,
                        // with this reason, instead of the body being bent to a lint.
                        match spot {
                            Some(number) => rsx! {
                                a {
                                    class: "nd-pagination-link",
                                    href: address(*number),
                                    // `page`, not `true`: the value is what says
                                    // this is a step in a sequence rather than a
                                    // selected thing.
                                    "aria-current": (*number == current).then_some("page"),
                                    "aria-label": if *number == current {
                                        named(&current_label, *number)
                                    } else {
                                        named(&page_label, *number)
                                    },
                                    "{number}"
                                }
                            },
                            // A character, not a word — and hidden, because an
                            // ellipsis read out between page three and page nine
                            // is noise, and both ends are drawn anyway.
                            //
                            // WITH `more_label` IT IS ALSO A CONTROL, and the
                            // control ships `hidden` for `Button`'s reason:
                            // `dist/menu.js` reveals it once it has found the
                            // menu it names, so a page with no script carries
                            // the same inert character it always carried. The
                            // stylesheet takes the character away when the
                            // button beside it is no longer hidden.
                            None => {
                                let (first, past) = hidden(index);
                                let panel = format!("nd-pagination-{name}-gap-{first}");
                                let opens = more_label.clone();
                                rsx! {
                                    span { class: "nd-pagination-gap", "aria-hidden": "true", "…" }
                                    if let Some(opens) = opens {
                                        button {
                                            class: "nd-pagination-link nd-pagination-more",
                                            r#type: "button",
                                            "aria-label": "{opens}",
                                            "aria-haspopup": "menu",
                                            // "false" is TRUE on a menu button:
                                            // it says a menu is there and shut.
                                            "aria-expanded": "false",
                                            "aria-controls": "{panel}",
                                            "data-nd-pagination-more": "",
                                            hidden: true,
                                            "…"
                                        }
                                        Menu {
                                            name: "{panel}",
                                            label: "{opens}",
                                            class: "nd-pagination-menu",
                                            // The pages this gap stands for, in
                                            // order. A number is an identifier,
                                            // so nothing here is translated.
                                            for number in first..=past {
                                                MenuItem {
                                                    key: "{number}",
                                                    class: "nd-identifier",
                                                    label: "{number}",
                                                    href: address(number),
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
                if let Some(next_label) = next_label {
                    if current < last {
                        li {
                            a {
                                class: "nd-pagination-link nd-pagination-step",
                                href: address(current + 1),
                                rel: "next",
                                "{next_label}"
                            }
                        }
                    }
                }
            }
            // A live region, because after a page turns this is the only thing
            // that says the set did not change under the reader.
            if let Some(summary) = summary {
                p { class: "nd-pagination-summary", role: "status", "{summary}" }
            }
        }
    }
"#;

const PAGINATION_REACT: &str = r#"  const spread_ = spread ?? "default";
  const size_ = size ?? "medium";
  const root = ["nd-pagination", className].filter(Boolean).join(" ");
  // Clamped rather than trusted: `page 12 of 9` has already lost the reader.
  const last = Math.max(1, Number.parseInt(pages, 10) || 1);
  const current = Math.min(last, Math.max(1, Number.parseInt(page, 10) || 1));
  const siblings = { compact: 1, default: 2, wide: 3 }[spread_] ?? 2;
  const from = Math.max(1, current - siblings);
  const to = Math.min(last, current + siblings);
  // `null` is the gap. Both ends are always in the list.
  const window = [];
  if (from > 1) window.push(1);
  if (from > 2) window.push(null);
  for (let n = from; n <= to; n += 1) window.push(n);
  if (to + 1 < last) window.push(null);
  if (to < last) window.push(last);
  const address = (n) => href.replace("{page}", String(n));
  const named = (template, n) => (template ? template.replace("{page}", String(n)) : undefined);
  // See the Dioxus half: a leading gap can only sit at index 1, so the index is
  // what says which of the two gaps this is.
  const hidden = (index) => (index === 1 ? [2, from - 1] : [to + 1, last - 1]);
  const pagesIn = ([first, past]) => {
    const numbers = [];
    for (let n = first; n <= past; n += 1) numbers.push(n);
    return numbers;
  };
  return h(
    "nav",
    {
      className: root,
      "aria-label": label,
      "data-nd-pagination": name,
      "data-nd-page": String(current),
      "data-nd-pages": String(last),
      "data-nd-spread": spread_,
      "data-nd-size": size_,
      ...rest,
    },
    h(
      "ul",
      { key: "l", className: "nd-pagination-list" },
      // Absent rather than disabled at the ends: a link to nowhere is a control
      // a keyboard still stops at.
      previousLabel && current > 1
        ? h(
            "li",
            { key: "p" },
            h(
              "a",
              {
                className: "nd-pagination-link nd-pagination-step",
                href: address(current - 1),
                rel: "prev",
              },
              previousLabel,
            ),
          )
        : null,
      window.map((spot, index) =>
        h(
          "li",
          { key: `n${index}` },
          spot === null
            ? // A character, not a word, and hidden: both ends are drawn anyway.
              // See the Dioxus half for the control beside it, which ships
              // hidden and is revealed by `dist/menu.js`.
              [
                h("span", { key: "g", className: "nd-pagination-gap", "aria-hidden": "true" }, "…"),
                moreLabel
                  ? h(
                      "button",
                      {
                        key: "m",
                        className: "nd-pagination-link nd-pagination-more",
                        type: "button",
                        "aria-label": moreLabel,
                        "aria-haspopup": "menu",
                        // "false" is TRUE on a menu button.
                        "aria-expanded": "false",
                        "aria-controls": `nd-pagination-${name}-gap-${hidden(index)[0]}`,
                        "data-nd-pagination-more": "",
                        hidden: true,
                      },
                      "…",
                    )
                  : null,
                moreLabel
                  ? h(
                      Menu,
                      {
                        key: "p",
                        name: `nd-pagination-${name}-gap-${hidden(index)[0]}`,
                        label: moreLabel,
                        className: "nd-pagination-menu",
                      },
                      // The pages this gap stands for. A number is an
                      // identifier, so nothing here is translated.
                      pagesIn(hidden(index)).map((n) =>
                        h(MenuItem, {
                          key: `p${n}`,
                          className: "nd-identifier",
                          label: String(n),
                          href: address(n),
                        }),
                      ),
                    )
                  : null,
              ]
            : h(
                "a",
                {
                  className: "nd-pagination-link",
                  href: address(spot),
                  // `page`, not `true`: a step in a sequence, not a selection.
                  "aria-current": spot === current ? "page" : undefined,
                  "aria-label":
                    spot === current ? named(currentLabel, spot) : named(pageLabel, spot),
                },
                String(spot),
              ),
        ),
      ),
      nextLabel && current < last
        ? h(
            "li",
            { key: "x" },
            h(
              "a",
              {
                className: "nd-pagination-link nd-pagination-step",
                href: address(current + 1),
                rel: "next",
              },
              nextLabel,
            ),
          )
        : null,
    ),
    summary
      ? h("p", { key: "s", className: "nd-pagination-summary", role: "status" }, summary)
      : null,
  );
"#;

/// A `Field` and a drawing inside it, for the two that need both.
///
/// The four controls that predate `Field` draw their own question no longer.
/// `DateInput` needs `FieldKind` besides, because a date is a group of three
/// boxes and a group is named by a `<legend>`.
fn field_and_icon(name: &str) -> String {
    let kind = if name == "DateInput" {
        "FieldKind, FieldSize"
    } else {
        "FieldSize"
    };
    format!(
        "use noctua_design_icons::IconDrawing;\n\nuse crate::components::{{Field, {kind}, Icon}};\n"
    )
}

/// The drawings, then a list of components, in the two groups rustfmt keeps.
///
/// A helper rather than a `concat!` per arm: four bodies want an icon beside
/// something else, and four copies of the same two lines is four places to
/// change when a path moves.
fn beside_icon(components: &str) -> String {
    format!("use noctua_design_icons::IconDrawing;\n\nuse crate::components::{{{components}}};\n")
}

/// Any lint a custom body legitimately trips, with the reason it trips it.
///
/// A body written by hand can be right and still lose an argument with a
/// pedantic lint. The rule this repository keeps is not "no allows" but "no
/// allow without a written reason", so the reason lives here, beside the body
/// it is about, and reaches the generated file with it.
#[must_use]
pub fn allows(component: &Component) -> String {
    match component.meta.name.as_str() {
        // `Pagination` draws either a page number or the gap between two runs
        // of them. Clippy would rather that were an `if let`, and it cannot be:
        // the gap arm needs three `let` bindings to name the pages it hides,
        // and `rsx!`'s own `if let` takes rsx content rather than statements.
        "Pagination" => "#[allow(\n    clippy::single_match_else,\n    reason = \"the gap arm \
             binds three names, and rsx's `if let` takes no statements\"\n)]\n"
            .to_owned(),
        _ => String::new(),
    }
}

/// Anything a custom body needs imported, in Rust.
#[must_use]
pub fn dioxus_imports(component: &Component) -> String {
    match component.meta.name.as_str() {
        // THE ONE COMPONENT THAT IMPORTS NO COMPONENT. Its body reads the
        // geometry off `IconDrawing` and the weight off the spec, and it renders
        // an element rather than anything from this crate.
        "Icon" => "use noctua_design_icons::{IconDrawing, STROKE_WIDTH};\n".to_owned(),
        // A field, a fieldset and the one drawing a rating is made of. A tab
        // wants the same drawing at a size it names.
        "Rating" => beside_icon("Field, FieldKind, FieldSize, Icon"),
        "Tab" => beside_icon("Icon, IconSize"),
        // `SelectOption` needs both for a reason worth writing down: the name
        // becomes `data-nd-icon` on the option, and the drawing goes in a
        // `<template>` beside it for the enhanced list to clone.
        "Button" | "Link" | "SegmentedItem" | "Disclosure" | "Checkbox" | "SelectOption"
        | "Search" | "SearchResult" | "Avatar" | "Carousel" => PLAIN_ICON.to_owned(),
        // A drawing and the field around it — see `field_and_icon`.
        "Select" | "DateInput" => field_and_icon(&component.meta.name),
        // A COMPONENT THAT RENDERS COMPONENTS. A tag's colour chip is a real
        // `Swatch` and its remove control is a real `Button` — a pill is not
        // the place to grow a second chip and a second button.
        "Tag" => concat!(
            "use noctua_design_icons::IconDrawing;\n",
            "\n",
            "use crate::components::{Button, ButtonShape, ButtonSize, ButtonTone, ButtonVariant, Icon, Swatch, SwatchSize};\n"
        )
        .to_owned(),
        // THE ENGINE, NOT A SCRIPT. The scene is laid out where the component
        // renders — here during pre-rendering, and in `layout.js` for React —
        // because this package ships no runtime and a consumer has none to
        // borrow. `shown_name_of` is aliased so the two bodies can say the same
        // word without shadowing the `label` prop.
        "Lineage" => concat!(
            "use std::fmt::Write as _;\n",
            "\n",
            "use noctua_design_lineage::{\n",
            "    Options, Route, View, axis_name, draw, nameable, set_parts, some,\n",
            "};\n"
        )
        .to_owned(),
        "LineageFigure" => concat!(
            "use std::fmt::Write as _;\n",
            "\n",
            "use noctua_design_lineage::{nameable, set_parts};\n"
        )
        .to_owned(),
        // A NODE RENDERS A COMPONENT: its badge is a real `Tag`, so there is
        // one pill in this system rather than a second shaped like it. The
        // toggle is a bare `<button>` and not a `Button`, because it ships
        // hidden and nameless — its name arrives from the tree's own
        // `expand_label` once the script knows which state it is in.
        "TreeItem" => beside_icon("Icon, Tag, TagSize, TagVariant"),
        // IT SHIPS A `Tag` FOR THE SCRIPT TO CLONE. A second place that knows
        // what a tag's markup looks like would drift from the emitter, which
        // is the whole thing this generator exists to prevent — so the picker
        // renders one inside a `<template>` and `shell.js` copies it.
        // Everything whose body draws one glyph at a size it names: the `i`,
        // a menu row's icon, a bar's dismiss control, and the three floating
        // surfaces whose way out is a close control.
        "Info" | "MenuItem" | "MessageBar" | "Dialog" | "Popover" | "Toast" => {
            SIZED_ICON.to_owned()
        }
        "Field" => LABEL_IMPORT.to_owned(),
        // THE GAP OPENS THE MENU THIS SYSTEM ALREADY HAS. A second surface with
        // its own arrow walk would be a second thing to keep in step with
        // `dist/menu.js`, which already ships the opening, the walk, Escape and
        // the focus coming back.
        "Pagination" => "use crate::components::{Menu, MenuItem};\n".to_owned(),
        "Table" => concat!(
            "use noctua_design_icons::IconDrawing;\n",
            "\n",
            "use crate::components::{Button, ButtonSize, ButtonTone, ButtonVariant, Icon, IconSize};\n"
        )
        .to_owned(),
        "TableRow" => concat!(
            "use noctua_design_icons::IconDrawing;\n",
            "\n",
            "use crate::components::{\n",
            "    Button, ButtonSize, ButtonVariant, Checkbox, CheckboxSize, Radio, RadioSize,\n",
            "};\n"
        )
        .to_owned(),
        "TagPicker" => beside_icon("Field, FieldSize, Icon, Tag, TagSize"),
        // A `Field` and the controls inside it. Every one of these is the
        // same arrangement — the question, the help, what went wrong — with
        // one control in it, which is why one component draws all three.
        // A field, and a drawing inside it: an icon before the text, or the
        // two steppers.
        "Input" | "NumberInput" => beside_icon("Field, FieldSize, Icon"),
        // A field and nothing else. `Slider` and `OtpField` need `FieldKind`
        // besides, because a range and a row of boxes are both groups.
        "Textarea" => "use crate::components::{Field, FieldSize};\n".to_owned(),
        "Slider" | "OtpField" => FIELD_GROUP.to_owned(),
        "FileInput" => concat!(
            "use noctua_design_icons::IconDrawing;\n",
            "\n",
            "use crate::components::{Button, ButtonSize, ButtonVariant, Field, FieldSize, Icon};\n"
        )
        .to_owned(),
        // A FIELD AND THE HALF OF THE DAY. The morning and afternoon control
        // is a real `Segmented`, not a copy of its markup built in a script —
        // see the body.
        "TimeInput" => concat!(
            "use crate::components::{\n",
            "    Field, FieldKind, FieldSize, Segmented, SegmentedItem, SegmentedSize,\n",
            "};\n"
        )
        .to_owned(),
        // FIVE DRAWINGS, A DISCLOSURE, A SEGMENTED AND FOUR BUTTONS. The
        // panel is built out of components this system already has rather than
        // out of markup that looks like them — a second thing that draws a
        // tab strip is a second thing to keep in step with `Segmented`.
        "ColorPicker" => concat!(
            "use crate::components::{\n",
            "    Button, ButtonSize, ButtonVariant, Disclosure, Field, FieldSize, Icon, Label,\n",
            "    LabelKind, LabelSize, Segmented, SegmentedItem, SegmentedSize,\n",
            "};\n",
            "\n",
            "use noctua_design_icons::IconDrawing;\n"
        )
        .to_owned(),
        // THE SCANNER, NOT A SCRIPT. Highlighting happens where the component
        // renders, because this package ships no runtime and a consumer has no
        // `shell.js` to fall back on.
        "CodeBlock" => concat!(
            "use noctua_design_grammars::{Language, Role, lines, scan};\n",
            "use noctua_design_icons::IconDrawing;\n",
            "\n",
            "use crate::components::Icon;\n"
        )
        .to_owned(),
        // A COMPONENT THAT RENDERS COMPONENTS. The selection box is a
        // `Checkbox` and the expand toggle is a `Button`, because a table is
        // not the place to grow a second checkbox and a second button.
        _ => String::new(),
    }
}

/// The same, in JavaScript.
#[must_use]
pub fn react_imports(component: &Component) -> String {
    match component.meta.name.as_str() {
        // The drawings and the weight, from the framework-free package. See the
        // Dioxus half: this is the component every other one imports, and it
        // imports none of them.
        "Icon" => {
            // THE WEIGHT, AND NOTHING ELSE. This imported `ICONS` too — every
            // drawing in the system, so that one component could index it by
            // name. The drawing arrives as a prop now.
            "import { ICON_STROKE_WIDTH } from \"@noctua-world/design-tokens/icons.js\";\n"
                .to_owned()
        }
        // A fragment, because one step of a rating is three SIBLINGS and the
        // stylesheet reads the two adjacencies between them.
        "Rating" => format!("import {{ Fragment }} from \"react\";\n{REACT_FIELD}\n{REACT_ICON}"),
        // Everything whose body draws an icon and nothing else. The Dioxus half
        // splits these across several arms because `IconDrawing` and `IconSize`
        // are imported unevenly there; in JavaScript an icon is an icon.
        "Button" | "Link" | "SegmentedItem" | "Disclosure" | "Search" | "SearchResult"
        | "Avatar" | "Carousel" | "Info" | "MenuItem" | "MessageBar" | "Tab" | "Dialog"
        | "Popover" | "Toast" => REACT_ICON.to_owned(),
        // The chip is a `Swatch` and the remove control is a `Button`; see the
        // Dioxus half.
        "Tag" => concat!(
            "import { Button } from \"./button.js\";\n",
            "import { Swatch } from \"./swatch.js\";\n",
            "\n",
            "import { Icon } from \"./icon.js\";\n"
        )
        .to_owned(),
        // Two components that hold a `Tag` and a drawing, for two reasons: a
        // node's badge IS a tag, and a picker ships an empty one inside a
        // `<template>` for the script to clone. Same imports, one arm — see the
        // Dioxus halves, which differ.
        "TreeItem" => concat!(
            "import { Tag } from \"./tag.js\";\n",
            "\n",
            "import { Icon } from \"./icon.js\";\n"
        )
        .to_owned(),
        // The same menu in the other language — see the Dioxus half.
        "Pagination" => concat!(
            "import { Menu } from \"./menu.js\";\n",
            "import { MenuItem } from \"./menu_item.js\";\n"
        )
        .to_owned(),
        "Table" => concat!(
            "import { Button } from \"./button.js\";\n",
            "\n",
            "import { Icon } from \"./icon.js\";\n"
        )
        .to_owned(),
        "TableRow" => concat!(
            "import { Button } from \"./button.js\";\n",
            "import { Checkbox } from \"./checkbox.js\";\n",
            "import { Radio } from \"./radio.js\";\n"
        )
        .to_owned(),
        "TagPicker" => concat!(
            "import { Field } from \"./field.js\";\n",
            "import { Tag } from \"./tag.js\";\n",
            "\n",
            "import { Icon } from \"./icon.js\";\n"
        )
        .to_owned(),
        // The same arrangement in the other language — see the Dioxus halves.
        // The same two groupings as the Dioxus half, and for its reasons.
        //
        // `DateInput` AND `Select` ARE HERE BECAUSE THEIR BODIES RENDER A
        // `Field`, and both were one arm up with the icon-only components — so
        // `date_input.js` and `select.js` shipped `h(Field, …)` with no `Field`
        // in scope and threw on first render. Found by the guard in `props.rs`
        // that asks what each body names, rather than by the older one that
        // asks about a fixed list of six names.
        "Input" | "NumberInput" | "DateInput" | "Select" => concat!(
            "import { Field } from \"./field.js\";\n",
            "\n",
            "import { Icon } from \"./icon.js\";\n"
        )
        .to_owned(),
        "Textarea" | "Slider" | "OtpField" => REACT_FIELD.to_owned(),
        // The same composition in the other language — see the Dioxus half.
        "Field" => "import { Label } from \"./label.js\";\n".to_owned(),
        "FileInput" => concat!(
            "import { Button } from \"./button.js\";\n",
            "import { Field } from \"./field.js\";\n",
            "\n",
            "import { Icon } from \"./icon.js\";\n"
        )
        .to_owned(),
        // A field, and a real `Segmented` for the half of the day.
        "TimeInput" => format!("{REACT_FIELD}{REACT_SEGMENTED}"),
        // The same panel, the same reasoning — see the Dioxus half.
        "ColorPicker" => concat!(
            "import { Button } from \"./button.js\";\n",
            "import { Disclosure } from \"./disclosure.js\";\n",
            "import { Field } from \"./field.js\";\n",
            "import { Label } from \"./label.js\";\n",
            "import { Segmented } from \"./segmented.js\";\n",
            "import { SegmentedItem } from \"./segmented_item.js\";\n",
            "\n",
            "import { Icon } from \"./icon.js\";\n"
        )
        .to_owned(),
        // The engine, from the tokens package, where its twin is copied to.
        "Lineage" => concat!(
            "import {\n",
            "  draw,\n",
            "  nameable,\n",
            "  setParts,\n",
            "  shownName,\n",
            "  some,\n",
            "} from \"@noctua-world/design-tokens/lineage.js\";\n"
        )
        .to_owned(),
        "LineageFigure" => {
            "import { nameable, setParts } from \"@noctua-world/design-tokens/lineage.js\";\n"
                .to_owned()
        }
        "CodeBlock" => concat!(
            "import { ALIASES, GRAMMARS, lines, scan } from \"@noctua-world/design-tokens/grammars.js\";\n",
            "\n",
            "import { Icon } from \"./icon.js\";\n"
        )
        .to_owned(),
        // The selection box is a `Checkbox` and the expand toggle is a
        // `Button`; a table grows neither of its own.
        // A fragment, for a component that returns siblings rather than one
        // root: a `<dt>` and its `<dd>`, which have to be direct children of
        // the `<dl>`; and a choice's divider, row and revealed block, which
        // are three because the block is not inside the row it belongs to.
        //
        // The checkbox adds the icon, because its mark is the vendored tick.
        // The radio's is a dot — a shape, not a glyph — so it needs no icon.
        "ListItem" | "Radio" => "import { Fragment } from \"react\";\n".to_owned(),
        "Checkbox" | "SelectOption" => {
            "import { Fragment } from \"react\";\nimport { Icon } from \"./icon.js\";\n".to_owned()
        }
        _ => String::new(),
    }
}

/// Every component whose `icon`-shaped props are a drawing rather than a word.
///
/// A LIST RATHER THAN THREE LISTS. The Rust type, the TypeScript type and the
/// `.d.ts` import all have to name the same set, and they were three separate
/// match arms with the same contents — which is how `MessageBar` came to be the
/// one component whose `icon` crossed as an unparsed string. Dioxus ran it
/// through `IconDrawing::parse` and rendered nothing for a name nothing declares;
/// React handed the string to `Icon`, which throws. One list, so a component
/// cannot be in two of the three.
const TYPED_ICONS: [&str; 12] = [
    "Button",
    // IT WAS THE LAST ONE HOLDING A NAME. `MenuItem.icon` was a `String` run
    // through `IconDrawing::parse` at render time, so a name nothing declares
    // drew nothing and said nothing — the exact failure the type exists to make
    // impossible. It was also the only caller of `parse` outside the gallery,
    // and `parse` is behind the `catalogue` feature now: keeping it here would
    // have put every drawing in the system into every consumer of a menu.
    "MenuItem",
    "SegmentedItem",
    "Disclosure",
    "SelectOption",
    "Tag",
    "Avatar",
    "TreeItem",
    "Input",
    "Icon",
    "Tab",
    "MessageBar",
];

/// Whether this prop of this component is one of those drawings.
fn typed_icon(component: &Component, prop: &str) -> bool {
    TYPED_ICONS.contains(&component.meta.name.as_str())
        && matches!(prop, "icon" | "icon_end" | "icon_alt" | "icon_expanded")
}

/// The Rust type a `text` prop takes, when it is not a `String`.
///
/// `icon` is an `IconDrawing`, which is the whole reason a name that is not an
/// icon does not compile.
#[must_use]
pub fn dioxus_type(component: &Component, prop: &str) -> Option<&'static str> {
    // `Option<IconDrawing>` where the prop is optional: the generator wraps
    // whatever this returns.
    typed_icon(component, prop).then_some("IconDrawing")
}

/// The same, for the TypeScript declaration.
#[must_use]
pub fn typescript_type(component: &Component, prop: &str) -> Option<&'static str> {
    typed_icon(component, prop).then_some("IconDrawing")
}

/// Extra imports a `.d.ts` needs.
#[must_use]
pub fn typescript_imports(component: &Component) -> String {
    if TYPED_ICONS.contains(&component.meta.name.as_str()) {
        "import type { IconDrawing } from \"@noctua-world/design-tokens/icons.js\";\n".to_owned()
    } else {
        String::new()
    }
}

/// The drawings a custom React body reaches for, as `import` statements.
///
/// # Derived from the body, never listed beside it
///
/// Each drawing is its own module now, so a body that renders one has to import
/// it — and a list of which component wants which icon would be a second place
/// the same fact is written. What it produces when the two disagree is not a
/// build error either: an identifier with no import is a `ReferenceError` at
/// render time, which is the exact class of defect that shipped when
/// `react_imports` existed and was called from nowhere.
///
/// So the body is generated, then read. An icon appears in a body as
/// `icon: <export>`, and an export name that the icon set declares gets an
/// import; `icon: icon`, `icon: iconEnd` and the rest pass a prop through and
/// match nothing, because no drawing is called `icon` or `icon-end`.
///
/// Position matters and a bare word will not do: `menu`, `check`, `search` and
/// `columns` are all real drawings *and* real local variables in these bodies.
#[must_use]
pub fn react_icon_imports(component: &Component, drawings: &[Icon]) -> String {
    if !is_custom(component) {
        return String::new();
    }
    let mut body = String::new();
    react_body(&mut body, component);

    let mut wanted: BTreeSet<(&str, &str)> = BTreeSet::new();
    for at in body
        .match_indices("icon: ")
        .map(|(at, _)| at + "icon: ".len())
    {
        let identifier: String = body[at..]
            .chars()
            .take_while(char::is_ascii_alphanumeric)
            .collect();
        if let Some(icon) = drawings.iter().find(|icon| icon.export == identifier) {
            wanted.insert((icon.export.as_str(), icon.name.as_str()));
        }
    }

    let mut out = String::new();
    for (export, name) in wanted {
        let _ = writeln!(
            out,
            "import {{ {export} }} from \"@noctua-world/design-tokens/icons/{name}.js\";"
        );
    }
    out
}

// --- Tag and Badge ----------------------------------------------------------

/// The pill, in every shape it takes.
///
/// # One component, three elements
///
/// `href` makes it an `<a>`, `pressed` makes it a `<button>`, and neither makes
/// it a `<span>`. That is not a convenience: a tag that changes the address is
/// a link however it is painted, and one carrying a state a reader can turn off
/// is a button — the rules `Button` and `Link` already state, applied to a
/// smaller box. Painting a `<span>` to look like either is how middle-click,
/// the context menu and a screen reader's list of links all stop working.
///
/// # The remove control is a sibling, never a wrapper
///
/// A `<button>` inside an `<a>` is invalid, and a `<button>` inside a
/// `<button>` is unreachable in several engines. So an interactive tag is one
/// control and its remove control is a second beside it, inside a wrapper that
/// is neither. A tag that is both a link and removable is two tab stops,
/// because it is two things a reader can do.
const TAG_DIOXUS: &str = r#"    let root = format!("nd-tag {class}");
    let root = root.trim().to_owned();

    // Filled once, here, rather than twice in two branches. `{label}` is the
    // hole `NOCTUA.md §9.9` requires: one sentence, one key, one translation.
    let remove = remove_label
        .as_ref()
        .map(|words| words.replace("{label}", &label));

    // The word and everything drawn beside it. The same run whichever element
    // ends up holding it, so the three branches below cannot drift apart.
    let inside = rsx! {
        if let Some(palette) = swatch.clone() {
            Swatch { palette, size: SwatchSize::Small, class: "nd-tag-swatch" }
        }
        if let Some(source) = image.clone() {
            // EMPTY ALTERNATIVE TEXT, ON PURPOSE. The word beside it is the
            // name; a second one would be the same thing said twice, and a
            // broken URL with a non-empty alt draws a broken-image glyph and
            // the text, where an empty one collapses to nothing.
            img { class: "nd-tag-image", src: "{source}", alt: "", loading: "lazy", decoding: "async" }
        }
        if let Some(icon) = icon {
            span { class: "nd-tag-glyph", "aria-hidden": "true",
                Icon { icon, size: crate::components::IconSize::Small }
            }
        }
        span { class: "nd-tag-label", "{label}" }
    };

    rsx! {
        span {
            class: "{root}",
            "data-nd-tone": tone,
            "data-nd-variant": variant.as_str(),
            "data-nd-size": size.as_str(),
            "data-nd-shape": shape.as_str(),
            "data-nd-glass": if glass { "true" } else { "false" },
            "data-nd-value": value,
            "data-nd-tag": "",
            if let Some(href) = href {
                a { class: "nd-tag-body", href: "{href}", {inside} }
            } else if let Some(pressed) = pressed {
                button {
                    class: "nd-tag-body",
                    r#type: "button",
                    // The role that already means "a state you can turn off",
                    // so nothing here invents one. Written whichever way it
                    // reads, unlike `Button`'s: there the absence means "not a
                    // toggle", and here the absence is a different ELEMENT.
                    //
                    // `as_str()` is the spec's own spelling — `on` and `off`,
                    // which is what a caller types — and `aria-pressed` takes
                    // `true` and `false`. Two vocabularies, translated here, in
                    // the one place that knows both.
                    "aria-pressed": if pressed == TagPressed::On { "true" } else { "false" },
                    disabled,
                    {inside}
                }
            } else {
                // NO ROLE AND NO TAB STOP. A pill that is only a label and
                // claims a role is the promise the markup does not keep.
                span { class: "nd-tag-body", {inside} }
            }
            if let Some(remove) = remove {
                Button {
                    class: "nd-tag-remove",
                    // A real submit when the caller said what field it posts
                    // under, so taking a tag off works with no script at all.
                    submit: if remove_name.is_some() {
                        crate::components::ButtonSubmit::Submit
                    } else {
                        crate::components::ButtonSubmit::Button
                    },
                    name: remove_name,
                    value: value.clone().unwrap_or_else(|| label.clone()),
                    variant: ButtonVariant::Transparent,
                    tone: ButtonTone::Neutral,
                    size: ButtonSize::Small,
                    shape: ButtonShape::Pill,
                    hide_label: true,
                    icon: IconDrawing::CLOSE,
                    label: remove,
                    disabled,
                }
            }
        }
    }
"#;

/// The same tag, in React.
const TAG_REACT: &str = r#"  const variant_ = variant ?? "soft";
  const size_ = size ?? "medium";
  const shape_ = shape ?? "pill";
  const root = ["nd-tag", className].filter(Boolean).join(" ");
  const remove = removeLabel ? removeLabel.replace("{label}", label) : null;

  // The word and everything drawn beside it, built once — see the Dioxus half.
  const inside = [
    swatch ? h(Swatch, { key: "s", palette: swatch, size: "small", className: "nd-tag-swatch" }) : null,
    image
      ? h("img", {
          key: "p",
          className: "nd-tag-image",
          src: image,
          alt: "",
          loading: "lazy",
          decoding: "async",
        })
      : null,
    icon
      ? h(
          "span",
          { key: "g", className: "nd-tag-glyph", "aria-hidden": "true" },
          h(Icon, { icon, size: "small" }),
        )
      : null,
    h("span", { key: "l", className: "nd-tag-label" }, label),
  ];

  const body = href
    ? h("a", { key: "b", className: "nd-tag-body", href }, ...inside)
    : pressed
      ? h(
          "button",
          {
            key: "b",
            className: "nd-tag-body",
            type: "button",
            // See the Dioxus half: the absence is a different element, so the
            // attribute is written whichever way it reads.
            "aria-pressed": pressed === "on" ? "true" : "false",
            disabled,
          },
          ...inside,
        )
      : h("span", { key: "b", className: "nd-tag-body" }, ...inside);

  return h(
    "span",
    {
      className: root,
      "data-nd-tone": tone,
      "data-nd-variant": variant_,
      "data-nd-size": size_,
      "data-nd-shape": shape_,
      "data-nd-glass": String(glass ?? false),
      "data-nd-value": value,
      "data-nd-tag": "",
      ...rest,
    },
    body,
    remove
      ? h(Button, {
          key: "x",
          className: "nd-tag-remove",
          submit: removeName ? "submit" : "button",
          name: removeName,
          value: value ?? label,
          variant: "transparent",
          tone: "neutral",
          size: "small",
          shape: "pill",
          hideLabel: true,
          icon: close,
          label: remove,
          disabled,
        })
      : null,
  );
"#;

/// The count on a corner.
///
/// # The number is decoration and the sentence is the message
///
/// `3` is not a sentence, and a reader who cannot see which control it is stuck
/// to has been told nothing. The mark is `aria-hidden` and `label` is a
/// visually hidden text node inside the wrapper, so the control's own name is
/// announced and then the sentence.
///
/// # And the wrapper adds no role
///
/// What is inside keeps whatever it already was. Wrapping a control in
/// something with a role of its own is how a button comes to announce as a
/// group containing a button.
const BADGE_DIOXUS: &str = r#"    let root = format!("nd-badge {class}");
    let root = root.trim().to_owned();

    // `99+` rather than `4216`, and only when both are numbers — the difference
    // between the two is not what the corner of a button is for, and a
    // four-digit badge is wider than the control it marks.
    let capped = match (count.as_deref(), max.as_deref()) {
        (Some(count), Some(max)) => match (count.trim().parse::<u64>(), max.trim().parse::<u64>()) {
            (Ok(count), Ok(max)) if count > max => Some(format!("{max}+")),
            _ => None,
        },
        _ => None,
    };
    let shown = capped.or_else(|| count.clone());

    // Nothing to report is best reported by an absence: a badge reading `0` is
    // a page telling a reader about something that did not happen.
    let zero = count.as_deref().map(str::trim) == Some("0");
    let mark = dot || (shown.is_some() && (!zero || show_zero));

    rsx! {
        span {
            class: "{root}",
            "data-nd-tone": tone,
            "data-nd-placement": placement.as_str(),
            "data-nd-overlap": overlap.as_str(),
            "data-nd-dot": if dot { "true" } else { "false" },
            // The count as it was GIVEN, not as it is drawn. `data-nd-marked`
            // below is the drawn answer; this is the question, and a stylesheet
            // or a test that wants to know what was asked for has to be able
            // to ask. The spec declares the attribute, so it is written.
            "data-nd-count": count.clone(),
            "data-nd-marked": if mark { "true" } else { "false" },
            {children}
            if mark {
                span { class: "nd-badge-mark", "aria-hidden": "true",
                    if !dot {
                        if let Some(shown) = shown { "{shown}" }
                    }
                }
                // WHAT IS ACTUALLY ANNOUNCED. The digits above are a picture of
                // this, and a badge whose only content was digits would tell a
                // reader a number and nothing else.
                span { class: "nd-visually-hidden", "{label}" }
            }
        }
    }
"#;

/// The same badge, in React.
const BADGE_REACT: &str = r#"  const placement_ = placement ?? "top-end";
  const overlap_ = overlap ?? "rectangle";
  const dot_ = dot ?? false;
  const root = ["nd-badge", className].filter(Boolean).join(" ");

  // See the Dioxus half for both of these.
  const asNumber = (value) => (/^\d+$/.test((value ?? "").trim()) ? Number((value ?? "").trim()) : null);
  const counted = asNumber(count);
  const ceiling = asNumber(max);
  const shown = counted !== null && ceiling !== null && counted > ceiling ? `${ceiling}+` : count;

  const zero = (count ?? "").trim() === "0";
  const mark = dot_ || (shown !== undefined && shown !== null && (!zero || showZero));

  return h(
    "span",
    {
      className: root,
      "data-nd-tone": tone,
      "data-nd-placement": placement_,
      "data-nd-overlap": overlap_,
      "data-nd-dot": String(dot_),
      // See the Dioxus half: the count as given, beside the count as drawn.
      "data-nd-count": count,
      "data-nd-marked": String(Boolean(mark)),
      ...rest,
    },
    children,
    mark
      ? h("span", { key: "m", className: "nd-badge-mark", "aria-hidden": "true" }, dot_ ? null : shown)
      : null,
    mark ? h("span", { key: "n", className: "nd-visually-hidden" }, label) : null,
  );
"#;

// --- Avatar and AvatarGroup -------------------------------------------------

/// A person, in three layers.
///
/// # The fallback is a stacking order, not a branch
///
/// The icon is the floor, the initials sit over it, and the picture sits over
/// both — so a URL that never answers shows the letters and a slow one shows
/// them until it lands. There is no `onerror` here to take a branch with: this
/// package ships no runtime, so a fallback is CSS or it is nothing.
///
/// The `<img>`'s alternative text is empty for the same reason. A broken image
/// with a non-empty alt draws a broken-image glyph and the words in most
/// engines; with an empty one it draws nothing at all and what is underneath
/// shows through. The name lives on the wrapper, where one thing has one name.
const AVATAR_DIOXUS: &str = r#"    let root = format!("nd-avatar {class}");
    let root = root.trim().to_owned();
    let icon = icon.unwrap_or(IconDrawing::USER);

    // The three layers, deepest first. Written once so the two elements below
    // cannot come to disagree about what an avatar contains.
    let layers = rsx! {
        span { class: "nd-avatar-icon", "aria-hidden": "true",
            Icon { icon }
        }
        if let Some(initials) = initials {
            // `aria-hidden` like every other drawing here: they are a picture
            // of the name, and a reader who heard "Ana Lima" does not need
            // "AL" after it.
            // `nd-identifier` because that is what initials are: letters cut
            // from a name, the same in every locale. It is also what tells the
            // pseudo-locale sweep that an unbracketed `AL` on a page is
            // deliberate rather than a sentence somebody forgot to translate.
            span {
                class: "nd-avatar-initials nd-identifier",
                "aria-hidden": "true",
                "{initials}"
            }
        }
        if let Some(src) = src {
            img {
                class: "nd-avatar-image",
                src: "{src}",
                alt: "",
                loading: "lazy",
                decoding: "async",
            }
        }
    };

    // `decorative` is the whole thing hidden, so it takes no name and gets no
    // link — a focusable element inside `aria-hidden` is `aria-hidden-focus`,
    // a control a keyboard reaches and a screen reader cannot describe.
    let named = (!decorative).then(|| label.clone());

    rsx! {
        if let Some(href) = href.filter(|_| !decorative) {
            a {
                class: "{root}",
                href: "{href}",
                "data-nd-size": size.as_str(),
                "data-nd-shape": shape.as_str(),
                "data-nd-tone": tone.as_str(),
                "aria-label": named,
                {layers}
            }
        } else {
            span {
                class: "{root}",
                "data-nd-size": size.as_str(),
                "data-nd-shape": shape.as_str(),
                "data-nd-tone": tone.as_str(),
                "aria-hidden": decorative.then_some("true"),
                // A `<span>` needs a role to carry a name at all. `img` is the
                // right one: this IS a picture of a person, whatever it is
                // built from, and it is what makes the label reach a reader.
                role: (!decorative).then_some("img"),
                "aria-label": named,
                {layers}
            }
        }
    }
"#;

/// The same person, in React.
const AVATAR_REACT: &str = r#"  const size_ = size ?? "medium";
  const shape_ = shape ?? "circle";
  const tone_ = tone ?? "neutral";
  const decorative_ = decorative ?? false;
  const root = ["nd-avatar", className].filter(Boolean).join(" ");
  const named = decorative_ ? undefined : label;

  // The three layers, deepest first — see the Dioxus half.
  const layers = [
    h(
      "span",
      { key: "i", className: "nd-avatar-icon", "aria-hidden": "true" },
      h(Icon, { icon: icon ?? "user" }),
    ),
    initials
      ? h(
          "span",
          { key: "n", className: "nd-avatar-initials nd-identifier", "aria-hidden": "true" },
          initials,
        )
      : null,
    src
      ? h("img", {
          key: "p",
          className: "nd-avatar-image",
          src,
          alt: "",
          loading: "lazy",
          decoding: "async",
        })
      : null,
  ];

  const shared = {
    className: root,
    "data-nd-size": size_,
    "data-nd-shape": shape_,
    "data-nd-tone": tone_,
    "aria-label": named,
    ...rest,
  };

  if (href && !decorative_) return h("a", { ...shared, href }, ...layers);
  return h(
    "span",
    {
      ...shared,
      "aria-hidden": decorative_ ? "true" : undefined,
      role: decorative_ ? undefined : "img",
    },
    ...layers,
  );
"#;

/// Several of them, as a real list.
///
/// # It does not count its children
///
/// Dioxus receives `children` as an opaque node with no length, and `§9.3`
/// forbids React doing what Dioxus cannot — so there is no `max` that hides
/// some and no arithmetic that claims how many were hidden. The caller passes
/// what is shown and writes the word for the rest.
///
/// # And the counter is one of the items
///
/// A list of three faces and a chip beside it announces *list, 3 items* while
/// standing for eight people. Making the chip the fourth item is what makes the
/// number the browser says true.
const AVATAR_GROUP_DIOXUS: &str = r#"    let root = format!("nd-avatar-group {class}");
    let root = root.trim().to_owned();

    rsx! {
        ul {
            class: "{root}",
            "aria-label": "{label}",
            "data-nd-size": size.as_str(),
            "data-nd-shape": shape.as_str(),
            "data-nd-overlap": overlap.as_str(),
            {children}
            if let Some(more) = more {
                li { class: "nd-avatar-more",
                    if let Some(href) = more_href {
                        a { class: "nd-avatar-more-chip", href: "{href}", "aria-label": more_label,
                            span { "aria-hidden": "true", "{more}" }
                        }
                    } else {
                        // NOT FOCUSABLE WITH NOWHERE TO GO. A chip a reader can
                        // tab to and press, which does nothing, is the control
                        // that lies — so with no `more_href` it is a `<span>`
                        // carrying its own name and no role at all.
                        span { class: "nd-avatar-more-chip",
                            span { "aria-hidden": "true", "{more}" }
                            if let Some(more_label) = more_label {
                                span { class: "nd-visually-hidden", "{more_label}" }
                            }
                        }
                    }
                }
            }
        }
    }
"#;

/// The same group, in React.
const AVATAR_GROUP_REACT: &str = r#"  const size_ = size ?? "medium";
  const shape_ = shape ?? "circle";
  const overlap_ = overlap ?? "loose";
  const root = ["nd-avatar-group", className].filter(Boolean).join(" ");

  return h(
    "ul",
    {
      className: root,
      "aria-label": label,
      "data-nd-size": size_,
      "data-nd-shape": shape_,
      "data-nd-overlap": overlap_,
      ...rest,
    },
    children,
    more
      ? h(
          "li",
          { key: "m", className: "nd-avatar-more" },
          moreHref
            ? h(
                "a",
                { className: "nd-avatar-more-chip", href: moreHref, "aria-label": moreLabel },
                h("span", { "aria-hidden": "true" }, more),
              )
            : h(
                "span",
                { className: "nd-avatar-more-chip" },
                h("span", { key: "t", "aria-hidden": "true" }, more),
                moreLabel ? h("span", { key: "n", className: "nd-visually-hidden" }, moreLabel) : null,
              ),
        )
      : null,
  );
"#;

// --- TagPicker --------------------------------------------------------------

/// Several values a reader writes.
///
/// # It is a `<select multiple>`, and everything follows from that
///
/// The store is the browser's own — `selectedOptions`, with no mirror — so a
/// value cannot be committed twice, because the option set *is* the index. A
/// typed value becomes a real `<option selected>`, which is what `Select`'s
/// `create()` already does, and what reaches a server is what the browser
/// sends.
///
/// The alternative was one text field holding a separator-joined string, and it
/// dies on one question: what happens to a value containing the separator.
/// Escaping it ships a wire format the contract cannot express; forbidding it
/// refuses a legal string with no way to say why; splitting it corrupts data
/// silently. Here the separator is a **keystroke** and never an encoding.
///
/// # The composer ships a `name` and the script takes it away
///
/// So a reader with no script picks any number of suggestions from the native
/// listbox *and* types one free value, and both arrive under one field. That is
/// the honest half of `Select`'s doctrine — "a page whose script failed to load
/// is a page that still submits" — applied to the half a `<select>` cannot
/// hold.
///
/// # And it ships a `Tag` for the script to clone
///
/// `shell.js` may not know what a tag's markup is: a second place that knows
/// would drift from this generator, which is the whole thing it exists to
/// prevent. The picker renders one empty `Tag` inside a `<template>` and the
/// script copies it — the generalisation of the `<template data-nd-glyph>` that
/// `Select` already ships for its tick.
const TAG_PICKER_DIOXUS: &str = r#"    let root = format!("nd-tag-picker {class}");
    let root = root.trim().to_owned();
    // Derived, so the label's target, the hint, the error, the tag list, the
    // listbox and the status line cannot be typed wrong — there is nothing to
    // type. `Search` was rebuilt because the version before it derived its id
    // from an optional name.
    let id = format!("nd-tag-picker-{name}");
    let hint_id = format!("{id}-hint");
    let error_id = format!("{id}-error");
    let tags_id = format!("{id}-tags");
    let described: Vec<String> = [
        hint.as_ref().map(|_| hint_id.clone()),
        error.as_ref().map(|_| error_id.clone()),
    ]
    .into_iter()
    .flatten()
    .collect();
    let described = (!described.is_empty()).then(|| described.join(" "));
    // Both read before the sentence and the name are moved into the markup:
    // one fact, one switch, and no borrow of something the macro has taken.
    let flagged = if error.is_some() { "true" } else { "false" };
    let invalid = error.is_some().then_some("true");
    // The clear control's copy, taken here: `rsx!` moves a value into the
    // nested block before the root's attributes are read, so one binding per
    // use is the only order that borrow-checks. `Select` does the same.
    let clear = if clearable { clear_label.clone() } else { None };

    rsx! {
        Field {
            control: "{id}",
            label: label.clone(),
            hint: hint.clone(),
            error: error.clone(),
            size: match size {
                TagPickerSize::Small => FieldSize::Small,
                TagPickerSize::Medium => FieldSize::Medium,
                TagPickerSize::Large => FieldSize::Large,
            },
            disabled,

        div {
            class: "{root}",
            "data-nd-tag-picker": "{name}",
            "data-nd-size": size.as_str(),
            "data-nd-glass": if glass { "true" } else { "false" },
            "data-nd-invalid": flagged,
            "data-nd-creatable": if creatable { "true" } else { "false" },
            "data-nd-paste": if paste { "true" } else { "false" },
            "data-nd-duplicates": if duplicates { "true" } else { "false" },
            "data-nd-loading": if loading { "true" } else { "false" },
            "data-nd-separators": "{separators}",
            "data-nd-max": max,
            "data-nd-pattern": pattern,
            "data-nd-create-label": create_label,
            "data-nd-max-label": max_label,
            "data-nd-invalid-label": invalid_label,
            "data-nd-clear-label": clear_label.clone(),
            "data-nd-remove-label": remove_label,
            "data-nd-added-label": added_label,
            "data-nd-removed-label": removed_label,
            "data-nd-status": status,
            "data-nd-empty-label": empty_label,
            "data-nd-loading-label": loading_label,
            "data-nd-clearable": if clearable { "true" } else { "false" },

            div { class: "nd-tag-picker-field",
                // THE VALUE, AND THE WHOLE OF IT. Hidden once the script has
                // built the visible half; a hidden form control still submits,
                // which is the entire arrangement.
                select {
                    class: "nd-tag-picker-native",
                    id: "{id}-values",
                    name: "{name}",
                    multiple: true,
                    disabled,
                    "aria-label": "{label}",
                    "data-nd-tag-picker-native": "",
                    {children}
                }
                // The tags so far. A list rather than a run of spans, because
                // it is a list — and one tab stop rather than forty.
                ul {
                    class: "nd-tag-picker-tags",
                    id: "{tags_id}",
                    "aria-label": "{label}",
                    "data-nd-tag-picker-tags": "",
                }
                input {
                    class: "nd-tag-picker-input",
                    id: "{id}",
                    r#type: "text",
                    // THE COMPOSER SUBMITS TOO, UNTIL THE SCRIPT SAYS OTHERWISE.
                    // With no script this is the only way a reader enters a
                    // value that was not offered, and it posts under the same
                    // field as the select beside it.
                    name: "{name}",
                    autocomplete: "off",
                    autocorrect: "off",
                    autocapitalize: "none",
                    spellcheck: "false",
                    placeholder,
                    disabled,
                    // ON THE COMPOSER, NOT ON THE SELECT. A required control the
                    // script has hidden cannot be reported: the browser has to
                    // focus an invalid control to anchor its bubble, cannot, and
                    // the submit fails in silence.
                    required,
                    "aria-invalid": invalid,
                    "aria-describedby": described,
                    "data-nd-tag-picker-input": "",
                }
                if clearable {
                    button {
                        class: "nd-tag-picker-clear",
                        r#type: "button",
                        "aria-label": clear.clone(),
                        title: clear,
                        "data-nd-tag-picker-clear": "",
                        hidden: true,
                        Icon { icon: IconDrawing::CLOSE, size: crate::components::IconSize::Small }
                    }
                }
            }

            // One sentence at a time: how many there are, what was added, what
            // was taken off, what is refused. Inside the component so it is
            // beside what it describes.
            p {
                class: "nd-tag-picker-note",
                id: "{id}-status",
                role: "status",
                "aria-live": "polite",
                "aria-atomic": "true",
                "data-nd-tag-picker-status": "",
            }

            // THE SCRIPT DOES NOT KNOW WHAT A TAG LOOKS LIKE. It copies this.
            template { "data-nd-tag-template": "",
                // ONE STEP DOWN FROM THE FIELD, so the tags sit inside it
                // with room around them rather than filling it edge to edge —
                // a picker whose tags are exactly as tall as its box has no
                // field left to click into. Inline rather than a helper: the
                // generator emits one function per component and nothing else.
                Tag {
                    label: "",
                    size: match size {
                        TagPickerSize::Large => TagSize::Medium,
                        _ => TagSize::Small,
                    },
                    remove_label: "{{label}}",
                }
            }
        }
        }
    }
"#;

/// The same picker, in React.
const TAG_PICKER_REACT: &str = r#"  const size_ = size ?? "medium";
  const separators_ = separators ?? ",";
  const creatable_ = creatable ?? true;
  const paste_ = paste ?? true;
  const root = ["nd-tag-picker", className].filter(Boolean).join(" ");
  const id = `nd-tag-picker-${name}`;
  const hintId = `${id}-hint`;
  const errorId = `${id}-error`;
  const described = [hint ? hintId : null, error ? errorId : null].filter(Boolean).join(" ");
  // One step down from the field — see the Dioxus half.
  const tagSize = size_ === "large" ? "medium" : "small";

  return h(
    Field,
    {
      control: id,
      label,
      hint,
      error,
      size: size_,
      disabled,
    },
  h(
    "div",
    {
      key: "p",
      className: root,
      "data-nd-tag-picker": name,
      "data-nd-size": size_,
      "data-nd-glass": String(glass ?? false),
      "data-nd-invalid": String(Boolean(error)),
      "data-nd-creatable": String(creatable_),
      "data-nd-paste": String(paste_),
      "data-nd-duplicates": String(duplicates ?? false),
      "data-nd-loading": String(loading ?? false),
      "data-nd-separators": separators_,
      "data-nd-max": max,
      "data-nd-pattern": pattern,
      "data-nd-create-label": createLabel,
      "data-nd-max-label": maxLabel,
      "data-nd-invalid-label": invalidLabel,
      "data-nd-clear-label": clearLabel,
      "data-nd-remove-label": removeLabel,
      "data-nd-added-label": addedLabel,
      "data-nd-removed-label": removedLabel,
      "data-nd-status": status,
      "data-nd-empty-label": emptyLabel,
      "data-nd-loading-label": loadingLabel,
      "data-nd-clearable": String(clearable ?? false),
      ...rest,
    },
    h(
      "div",
      { key: "f", className: "nd-tag-picker-field" },
      h(
        "select",
        {
          key: "s",
          className: "nd-tag-picker-native",
          id: `${id}-values`,
          name,
          multiple: true,
          disabled,
          "aria-label": label,
          "data-nd-tag-picker-native": "",
        },
        children,
      ),
      h("ul", {
        key: "t",
        className: "nd-tag-picker-tags",
        id: `${id}-tags`,
        "aria-label": label,
        "data-nd-tag-picker-tags": "",
      }),
      h("input", {
        key: "i",
        className: "nd-tag-picker-input",
        id,
        type: "text",
        // See the Dioxus half: the composer submits until the script says
        // otherwise, so a reader with no script can still type one value.
        name,
        autoComplete: "off",
        autoCorrect: "off",
        autoCapitalize: "none",
        spellCheck: "false",
        placeholder,
        disabled,
        required,
        "aria-invalid": error ? "true" : undefined,
        "aria-describedby": described || undefined,
        "data-nd-tag-picker-input": "",
      }),
      clearable
        ? h(
            "button",
            {
              key: "c",
              className: "nd-tag-picker-clear",
              type: "button",
              "aria-label": clearLabel,
              title: clearLabel,
              "data-nd-tag-picker-clear": "",
              hidden: true,
            },
            h(Icon, { icon: close, size: "small" }),
          )
        : null,
    ),
    h("p", {
      key: "n",
      className: "nd-tag-picker-note",
      id: `${id}-status`,
      role: "status",
      "aria-live": "polite",
      "aria-atomic": "true",
      "data-nd-tag-picker-status": "",
    }),
    h(
      "template",
      { key: "p", "data-nd-tag-template": "" },
      h(Tag, { label: "", size: tagSize, removeLabel: "{label}" }),
    ),
  ),
  );
"#;

// --- Field -------------------------------------------------------------------

/// The question above a control, the help under it, and what went wrong.
///
/// # Why it is authored rather than derived
///
/// Its element switches — `<div>` or `<fieldset>` — and so does the element
/// that names it, which has to be a `<legend>` inside the second and a
/// `<label for>` inside the first. `props.rs` derives a body from the element,
/// the root class and the attribute each prop becomes; a second element that
/// changes with the first is not a fact it has.
///
/// # It renders no control, and that is the whole design
///
/// The control is `children`, which is what lets one component serve a select,
/// a text box, three date segments and a colour swatch. The cost is that this
/// cannot write `aria-describedby` on something it cannot reach — so the ids
/// are derived from `control` by the rule its own documentation states, and the
/// control computes the same two strings. `every_describedby_points_at_something`
/// is the assertion that they agreed.
///
/// # It no longer draws the question either
///
/// It drew a `<label>`, a `<legend>` and a required marker beside them, and the
/// settings panel drew a third thing shaped like both. `Label` is that
/// component now, and this renders it. What is left here is the arrangement —
/// the question, the hint, the sentence and the control — which is what a field
/// actually is.
const FIELD_DIOXUS: &str = r#"    let root = format!("nd-field {class}");
    let root = root.trim().to_owned();
    // The two the caller's control must point its `aria-describedby` at. One
    // rule, stated in `control`'s documentation, computed here and again in
    // every component that wraps itself in this one — except a group, which
    // is this element, so it points the description at itself.
    let hint_id = format!("{control}-hint");
    let error_id = format!("{control}-error");
    let flagged = if error.is_some() { "true" } else { "false" };
    let slot = error.is_some() || error_slot;
    // One scale, two enums. `Field` and `Label` step together on purpose, so
    // a large field is not answered by a medium question.
    let named = match size {
        FieldSize::Small => LabelSize::Small,
        FieldSize::Medium => LabelSize::Medium,
        FieldSize::Large => LabelSize::Large,
    };
    let group = matches!(kind, FieldKind::Group);
    let described: Vec<String> = [
        hint.as_ref().map(|_| hint_id.clone()),
        slot.then(|| error_id.clone()),
    ]
    .into_iter()
    .flatten()
    .collect();
    // Only a group writes it: a `<fieldset>` describes what is inside it, and
    // a reader entering one hears the hint once rather than once per box.
    let described =
        (group && !described.is_empty()).then(|| described.join(" "));

    // Everything below the question, built once. Both arms move it and only
    // one of them runs, which is what lets the element switch without the
    // body being written twice.
    let body = rsx! {
        if let Some(hint) = hint {
            p { class: "nd-field-hint", id: "{hint_id}", "{hint}" }
        }
        // RENDERED EMPTY WHERE A SCRIPT NEEDS SOMEWHERE TO WRITE. A control
        // whose real element the script hides cannot be reported by the
        // browser, so `error_slot` is where the browser's own complaint goes.
        // Empty, the stylesheet gives it no box and it announces nothing.
        if slot {
            p {
                class: "nd-field-error",
                id: "{error_id}",
                role: "alert",
                "data-nd-field-error": "",
                // THE WORD IS ALWAYS THERE, AND IT USED TO ARRIVE ONLY WITH A
                // CALLER'S OWN SENTENCE. The case this slot exists for is the
                // other one: `error_slot` with no `error`, where a script puts
                // the browser's own complaint here. That case never got its
                // prefix, and `Select { required }` always takes it.
                if let Some(error_label) = error_label {
                    span { class: "nd-visually-hidden", "{error_label} " }
                }
                // AND THE SENTENCE HAS AN ELEMENT OF ITS OWN, so a script can
                // replace what it says without deleting the word in front of
                // it. `messages.js` is the only thing that finds it, and the
                // stylesheet asks this element whether the slot has anything
                // to show.
                span { "data-nd-field-message": "",
                    if let Some(error) = error { "{error}" }
                }
            }
        }
        {children}
    };

    // A `<legend>` names a set and a `<label>` names one element, so which of
    // the two is the same question as which root element. Answering it in two
    // places is how a `<legend>` comes to sit in a `<div>`, which names
    // nothing at all and validates as nothing either.
    //
    // `Label` draws whichever it is, along with the word that marks a required
    // answer, and derives its own id from `control` by the rule stated there.
    match kind {
        FieldKind::Group => rsx! {
            fieldset {
                class: "{root}",
                "data-nd-size": size.as_str(),
                "data-nd-required": if required { "true" } else { "false" },
                "data-nd-disabled": if disabled { "true" } else { "false" },
                "data-nd-invalid": flagged,
                "aria-describedby": described,
                // THE ONE ATTRIBUTE IN HTML THAT REFUSES EVERYTHING UNDER IT.
                // On a group it is the real thing rather than a dimming.
                disabled,
                Label {
                    kind: LabelKind::Legend,
                    for_control: control.clone(),
                    text: label.clone(),
                    size: named,
                    required,
                    required_label: required_label.clone(),
                }
                {body}
            }
        },
        FieldKind::Single => rsx! {
            div {
                class: "{root}",
                "data-nd-size": size.as_str(),
                "data-nd-required": if required { "true" } else { "false" },
                "data-nd-disabled": if disabled { "true" } else { "false" },
                "data-nd-invalid": flagged,
                Label {
                    kind: LabelKind::Label,
                    for_control: control.clone(),
                    text: label.clone(),
                    size: named,
                    required,
                    required_label,
                }
                {body}
            }
        },
    }
"#;

const FIELD_REACT: &str = r#"  const kind_ = kind ?? "single";
  const size_ = size ?? "medium";
  const root = ["nd-field", className].filter(Boolean).join(" ");
  // See the Dioxus half: one rule for the two ids, and only a group points the
  // description at itself.
  const hintId = `${control}-hint`;
  const errorId = `${control}-error`;
  const group = kind_ === "group";
  const slot = Boolean(error) || errorSlot;
  const described = [hint ? hintId : null, slot ? errorId : null].filter(Boolean).join(" ");

  return h(
    group ? "fieldset" : "div",
    {
      className: root,
      "data-nd-size": size_,
      "data-nd-required": String(required ?? false),
      "data-nd-disabled": String(disabled ?? false),
      "data-nd-invalid": String(Boolean(error)),
      "aria-describedby": group && described ? described : undefined,
      disabled: group ? disabled : undefined,
      ...rest,
    },
    // The one label in the system, composed rather than repeated. It draws the
    // `<legend>` or the `<label for>`, the required word, and its own id.
    h(Label, {
      key: "l",
      kind: group ? "legend" : "label",
      forControl: control,
      text: label,
      size: size_,
      required,
      requiredLabel,
    }),
    hint ? h("p", { key: "h", className: "nd-field-hint", id: hintId }, hint) : null,
    slot
      ? h(
          "p",
          {
            key: "e",
            className: "nd-field-error",
            id: errorId,
            role: "alert",
            "data-nd-field-error": "",
          },
          // See the Dioxus half: the word is always there, and the sentence
          // has an element of its own so a script can replace it alone.
          errorLabel
            ? h("span", { key: "w", className: "nd-visually-hidden" }, `${errorLabel} `)
            : null,
          h("span", { key: "m", "data-nd-field-message": "" }, error ?? null),
        )
      : null,
    children,
  );
"#;

// --- ColorPicker -------------------------------------------------------------

/// A colour a reader chooses, and every notation for it.
///
/// # The value is the browser's, exactly as `TagPicker`'s is
///
/// One `<input type="color">` per colour, all under one `name`. That is a real
/// control with the platform's own picker behind it, it posts a real value, and
/// it needs no script whatsoever — so what the script adds is a *better* way to
/// set a value that already worked, rather than the only way. With `multiple`
/// there is one input per committed colour and one spare, because a browser
/// posts every input sharing a name and that is the whole store.
///
/// An absent `value` writes **no** `value` attribute rather than a colour this
/// repository picked. `#000000` typed here would be a colour literal in a
/// generated file, and the platform's own default is the platform's business.
///
/// # The trigger and the panel both ship hidden
///
/// A square that opens a panel nothing has built is a button that does nothing,
/// and a panel a reader can tab into before anything answers it is worse. Both
/// carry `hidden`, and revealing them is the script's first act — the
/// arrangement `Search` uses for its dialog trigger, for the reason it records.
///
/// # And the script is handed every drawing it needs
///
/// `NOCTUA.md §9.5` puts icons in one place, and this panel needs five of them
/// — a copy, a tick, a pipette, a plus and a close. Four ride on controls this
/// body renders; the fifth pair rides inside `<template>` elements the script
/// clones, one per repeated thing: a code row, a swatch, a committed colour and
/// a chip. `shell.js` may not know what any of them look like, for the reason
/// `TagPicker` states in its own header.
const COLOR_PICKER_DIOXUS: &str = r#"    let root = format!("nd-color-picker {class}");
    let root = root.trim().to_owned();
    // Derived, so the label's target, the hint, the error, the entry, both
    // tracks, the panel and the status line cannot be typed wrong — there is
    // nothing to type. This component puts more ids on a page than any other,
    // which is exactly why none of them is a prop.
    let id = format!("nd-color-picker-{name}");
    // THE SQUARE IS WHAT THE QUESTION NAMES. It is a real `<button>`, which
    // is a labelable element, so `Field` points a real `<label for>` at it —
    // and the ids the hint and the error take derive from that same id.
    let control = format!("{id}-trigger");
    let label_id = format!("{control}-label");
    let hint_id = format!("{control}-hint");
    let error_id = format!("{control}-error");
    let panel_id = format!("{id}-panel");
    let entry_id = format!("{id}-entry");
    let described: Vec<String> = [
        hint.as_ref().map(|_| hint_id.clone()),
        error.as_ref().map(|_| error_id.clone()),
    ]
    .into_iter()
    .flatten()
    .collect();
    let described = (!described.is_empty()).then(|| described.join(" "));
    // Both read before the sentence is moved into the markup: one fact, one
    // switch, and no borrow of something the macro has taken.
    let flagged = if error.is_some() { "true" } else { "false" };
    let invalid = error.is_some().then_some("true");

    // THE VALUES, AND THE WHOLE OF THEM. `multiple` renders one input per
    // committed colour plus a spare; a single picker renders exactly one,
    // holding the first value or none at all.
    let given: Vec<String> = value
        .as_deref()
        .unwrap_or_default()
        .split_whitespace()
        .map(str::to_owned)
        .collect();
    let natives: Vec<Option<String>> = if multiple {
        given.iter().cloned().map(Some).chain([None]).collect()
    } else {
        vec![given.first().cloned()]
    };
    // THE SPARE IS MARKED, AND IT HAS TO BE. It is the row a reader with no
    // script sets to add a colour; with a script it is a colour nobody chose,
    // and a browser reports an empty `<input type="color">` as opaque black.
    // The script read four values out of a three-colour picker until this
    // said which row was the spare — asking the `value` ATTRIBUTE instead
    // would have worked until the first restored form.
    let spare = multiple.then_some(natives.len() - 1);
    // A boolean attribute HTML has no Rust type for. `Some("")` writes
    // `alpha=""`, which is the spelling every engine reads as present.
    let transparent = alpha.then_some("");

    rsx! {
        Field {
            control: "{control}",
            label: label.clone(),
            hint: hint.clone(),
            error: error.clone(),
            required,
            size: match size {
                ColorPickerSize::Small => FieldSize::Small,
                ColorPickerSize::Medium => FieldSize::Medium,
                ColorPickerSize::Large => FieldSize::Large,
            },
            disabled,

        div {
            class: "{root}",
            "data-nd-color-picker": "{name}",
            "data-nd-size": size.as_str(),
            "data-nd-shape": shape.as_str(),
            "data-nd-glass": if glass { "true" } else { "false" },
            "data-nd-invalid": flagged,
            "data-nd-multiple": if multiple { "true" } else { "false" },
            "data-nd-alpha": if alpha { "true" } else { "false" },
            "data-nd-palette": if palette { "true" } else { "false" },
            "data-nd-entry": if entry { "true" } else { "false" },
            "data-nd-eyedropper": if eyedropper { "true" } else { "false" },
            "data-nd-formats": "{formats}",
            "data-nd-recent": recent,
            "data-nd-max": max,
            "data-nd-max-label": max_label,
            "data-nd-remove-label": remove_label,
            "data-nd-added-label": added_label,
            "data-nd-removed-label": removed_label,
            "data-nd-status": status,
            "data-nd-trigger-label": trigger_label,
            "data-nd-entry-invalid-label": entry_invalid_label,
            "data-nd-copy-label": copy_label,
            "data-nd-copied-label": copied_label,

            div { class: "nd-color-picker-values", "data-nd-color-picker-values": "",
                for (index, initial) in natives.iter().enumerate() {
                    label { class: "nd-color-picker-native-row", key: "{index}",
                        // The field's name, on every one of them. A row of
                        // nameless colour inputs is a row a screen reader
                        // reads as *colour, colour, colour*.
                        span { class: "nd-visually-hidden", "{label}" }
                        input {
                            class: "nd-color-picker-native",
                            r#type: "color",
                            name: "{name}",
                            value: initial.clone(),
                            "alpha": transparent,
                            disabled,
                            // NEVER VIOLATED, AND STILL WORTH WRITING: a
                            // colour input always has a value, so the platform
                            // cannot refuse the form over this. What it does is
                            // announce the field as required, which is the half
                            // a reader actually needs.
                            required,
                            "aria-describedby": described.clone(),
                            "data-nd-color-picker-native": "",
                            "data-nd-spare": if spare == Some(index) { Some("") } else { None },
                        }
                    }
                }
            }

            // THE SQUARE. Hidden until the script reveals it — see the header.
            button {
                class: "nd-color-picker-trigger",
                id: "{control}",
                r#type: "button",
                hidden: true,
                disabled,
                "aria-controls": "{panel_id}",
                "data-nd-color-picker-trigger": "",
                span { class: "nd-color-picker-chips", "data-nd-color-picker-chips": "",
                    // Rendered here as well as by the script, so the square is
                    // already right the instant it is revealed rather than a
                    // frame later.
                    for (index, initial) in natives.iter().enumerate() {
                        if let Some(colour) = initial {
                            span {
                                class: "nd-color-picker-chip",
                                key: "{index}",
                                style: "--nd-color-picker-value: {colour}",
                            }
                        }
                    }
                }
            }

            div {
                class: "nd-panel nd-color-picker-panel",
                id: "{panel_id}",
                hidden: true,
                "data-nd-color-picker-panel": "",

                div { class: "nd-color-picker-head",
                    if palette {
                        Segmented {
                            name: "{id}-tab",
                            labelledby: "{label_id}",
                            size: SegmentedSize::Small,
                            class: "nd-color-picker-tabs",
                            SegmentedItem {
                                value: "spectrum",
                                label: spectrum_label.clone().unwrap_or_default(),
                                selected: true,
                            }
                            SegmentedItem {
                                value: "palette",
                                label: palette_label.clone().unwrap_or_default(),
                            }
                        }
                    }
                    Button {
                        label: close_label.clone().unwrap_or_default(),
                        icon: IconDrawing::CLOSE,
                        hide_label: true,
                        variant: ButtonVariant::Transparent,
                        size: ButtonSize::Small,
                        action: "color-picker-close",
                    }
                }

                div { class: "nd-color-picker-tab", "data-nd-tab": "spectrum",
                    // TWO REAL SLIDERS IN ONE BOX. Visually the square is one
                    // field a thumb moves in; to the platform it is two
                    // `<input type="range">` elements, which is where the role,
                    // the keyboard, the value and the announcement all come
                    // from. Inventing a `role="slider"` on a `<div>` would mean
                    // re-implementing every one of those, and getting one of
                    // them wrong silently.
                    div { class: "nd-color-picker-area", "data-nd-color-picker-area": "",
                        span { class: "nd-color-picker-area-thumb", "aria-hidden": "true" }
                        input {
                            class: "nd-color-picker-axis",
                            r#type: "range",
                            id: "{id}-saturation",
                            min: "0",
                            max: "100",
                            step: "1",
                            disabled,
                            "aria-label": area_label.clone(),
                            "data-nd-color-picker-axis": "saturation",
                        }
                        input {
                            class: "nd-color-picker-axis",
                            r#type: "range",
                            id: "{id}-value",
                            min: "0",
                            max: "100",
                            step: "1",
                            disabled,
                            "aria-label": area_label.clone(),
                            "data-nd-color-picker-axis": "value",
                        }
                    }
                    input {
                        class: "nd-color-picker-track",
                        r#type: "range",
                        id: "{id}-hue",
                        min: "0",
                        max: "360",
                        step: "1",
                        disabled,
                        "aria-label": hue_label,
                        "data-nd-color-picker-hue": "",
                    }
                    if alpha {
                        input {
                            class: "nd-color-picker-track",
                            r#type: "range",
                            id: "{id}-alpha",
                            min: "0",
                            max: "100",
                            step: "1",
                            disabled,
                            "aria-label": alpha_label,
                            "data-nd-color-picker-alpha": "",
                        }
                    }
                }

                if palette {
                    div {
                        class: "nd-color-picker-tab",
                        "data-nd-tab": "palette",
                        hidden: true,
                        div { class: "nd-color-picker-grid", "data-nd-color-picker-grid": "" }
                    }
                }

                if entry {
                    div { class: "nd-color-picker-entry-row",
                        label {
                            class: "nd-color-picker-entry-label",
                            r#for: "{entry_id}",
                            {entry_label.clone().unwrap_or_default()}
                        }
                        input {
                            class: "nd-color-picker-entry",
                            id: "{entry_id}",
                            r#type: "text",
                            autocomplete: "off",
                            autocorrect: "off",
                            autocapitalize: "none",
                            spellcheck: "false",
                            disabled,
                            "aria-invalid": invalid,
                            "data-nd-color-picker-entry": "",
                        }
                        if eyedropper {
                            // ABSENT WHERE THE API IS, not dead. The script
                            // removes it when `EyeDropper` is missing, which is
                            // why it is rendered rather than script-built:
                            // taking a control away is something a script can
                            // do honestly, and drawing one is not.
                            Button {
                                label: eyedropper_label.clone().unwrap_or_default(),
                                icon: IconDrawing::PIPETTE,
                                hide_label: true,
                                variant: ButtonVariant::Outline,
                                size: ButtonSize::Small,
                                action: "color-picker-eyedropper",
                            }
                        }
                    }
                }

                if recent.is_some() {
                    div {
                        class: "nd-color-picker-recent",
                        "data-nd-color-picker-recent": "",
                        hidden: true,
                        // THE ONE LABEL, not a fourth `<p>` shaped like one.
                        // It names the row of swatches under it rather than a
                        // control, so it is `kind = "name"`.
                        Label {
                            kind: LabelKind::Name,
                            size: LabelSize::Small,
                            text: recent_label.clone().unwrap_or_default(),
                        }
                        ul { class: "nd-color-picker-swatches", "data-nd-color-picker-recent-row": "" }
                        Button {
                            label: forget_label.clone().unwrap_or_default(),
                            variant: ButtonVariant::Transparent,
                            size: ButtonSize::Small,
                            action: "color-picker-forget",
                        }
                    }
                }

                // THE ASK, AND IT IS A `Disclosure`. Closed until a reader
                // wants it: four rows of text under the square would put the
                // colour off the bottom of a telephone.
                Disclosure {
                    summary: codes_label.clone().unwrap_or_default(),
                    name: "color-picker-codes",
                    class: "nd-color-picker-codes",
                    ul { class: "nd-color-picker-code-list", "data-nd-color-picker-codes": "" }
                }

                if multiple {
                    div { class: "nd-color-picker-list",
                        ul { class: "nd-color-picker-committed", "data-nd-color-picker-committed": "" }
                        Button {
                            label: add_label.clone().unwrap_or_default(),
                            icon: IconDrawing::PLUS,
                            variant: ButtonVariant::Outline,
                            size: ButtonSize::Small,
                            action: "color-picker-add",
                        }
                    }
                }
            }

            // One sentence at a time: how many colours there are, what was
            // added, what was taken off, what was refused. Inside the component
            // so it is beside what it describes.
            p {
                class: "nd-color-picker-note",
                id: "{id}-status",
                role: "status",
                "aria-live": "polite",
                "aria-atomic": "true",
                "data-nd-color-picker-status": "",
            }

            // THE SCRIPT DOES NOT KNOW WHAT ANY OF THESE LOOK LIKE. It copies
            // them. Four repeated things, four templates, and every drawing in
            // all of them came from `spec/icons.toml`.
            template { "data-nd-chip-template": "",
                span { class: "nd-color-picker-chip" }
            }
            template { "data-nd-swatch-template": "",
                li {
                    button {
                        class: "nd-color-picker-swatch",
                        r#type: "button",
                        "data-nd-color-picker-swatch": "",
                    }
                }
            }
            template { "data-nd-committed-template": "",
                li { class: "nd-color-picker-committed-row",
                    span { class: "nd-color-picker-chip" }
                    code { class: "nd-code-inline nd-identifier", "data-nd-tone": "subtle" }
                    Button {
                        label: "",
                        icon: IconDrawing::CLOSE,
                        hide_label: true,
                        variant: ButtonVariant::Transparent,
                        size: ButtonSize::Small,
                        action: "color-picker-remove",
                    }
                }
            }
            template { "data-nd-code-template": "",
                li { class: "nd-color-picker-code",
                    span { class: "nd-color-picker-code-name nd-identifier" }
                    code { class: "nd-code-inline nd-identifier", "data-nd-tone": "subtle" }
                    // A raw button rather than a `Button`, for `CodeBlock`'s
                    // reason: both glyphs are rendered and the state picks one,
                    // because swapping a drawing from script is shipping a
                    // drawing to the script.
                    button {
                        class: "nd-color-picker-copy",
                        r#type: "button",
                        "data-nd-color-copy": "",
                        Icon { icon: IconDrawing::COPY, size: crate::components::IconSize::Small }
                        Icon { icon: IconDrawing::CHECK, size: crate::components::IconSize::Small }
                    }
                }
            }
        }
        }
    }
"#;

const COLOR_PICKER_REACT: &str = r#"  const size_ = size ?? "medium";
  const shape_ = shape ?? "rounded";
  const entry_ = entry ?? true;
  const formats_ = formats ?? "hex rgb hsl oklch";
  const root = ["nd-color-picker", className].filter(Boolean).join(" ");
  const id = `nd-color-picker-${name}`;
  // THE SQUARE IS WHAT THE QUESTION NAMES — see the Dioxus half. It is a real
  // `<button>`, so `Field` points a real `<label for>` at it, and the label,
  // the hint and the error all derive from THAT id rather than from `id`:
  // `Field` and `Label` build theirs from the control they were handed, so a
  // description built from anything else points at nothing.
  const control = `${id}-trigger`;
  const labelId = `${control}-label`;
  const hintId = `${control}-hint`;
  const errorId = `${control}-error`;
  const panelId = `${id}-panel`;
  const entryId = `${id}-entry`;
  const described = [hint ? hintId : null, error ? errorId : null].filter(Boolean).join(" ");
  // See the Dioxus half: one input per committed colour plus a spare, and an
  // absent value writes no attribute rather than a colour of ours.
  const given = (value ?? "").split(/\s+/).filter(Boolean);
  const natives = multiple ? [...given, null] : [given[0] ?? null];
  // See the Dioxus half: the spare is a colour nobody chose, and an empty
  // colour input reports as opaque black rather than as empty.
  const spare = multiple ? natives.length - 1 : -1;

  return h(
    Field,
    {
      control,
      label,
      hint,
      error,
      required,
      size: size_,
      disabled,
    },
  h(
    "div",
    {
      key: "p",
      className: root,
      "data-nd-color-picker": name,
      "data-nd-size": size_,
      "data-nd-shape": shape_,
      "data-nd-glass": String(glass ?? false),
      "data-nd-invalid": String(Boolean(error)),
      "data-nd-multiple": String(multiple ?? false),
      "data-nd-alpha": String(alpha ?? false),
      "data-nd-palette": String(palette ?? false),
      "data-nd-entry": String(entry_),
      "data-nd-eyedropper": String(eyedropper ?? false),
      "data-nd-formats": formats_,
      "data-nd-recent": recent,
      "data-nd-max": max,
      "data-nd-max-label": maxLabel,
      "data-nd-remove-label": removeLabel,
      "data-nd-added-label": addedLabel,
      "data-nd-removed-label": removedLabel,
      "data-nd-status": status,
      "data-nd-trigger-label": triggerLabel,
      "data-nd-entry-invalid-label": entryInvalidLabel,
      "data-nd-copy-label": copyLabel,
      "data-nd-copied-label": copiedLabel,
      ...rest,
    },
    h(
      "div",
      { key: "v", className: "nd-color-picker-values", "data-nd-color-picker-values": "" },
      natives.map((initial, index) =>
        h(
          "label",
          { key: index, className: "nd-color-picker-native-row" },
          h("span", { key: "n", className: "nd-visually-hidden" }, label),
          h("input", {
            key: "i",
            className: "nd-color-picker-native",
            type: "color",
            name,
            defaultValue: initial ?? undefined,
            alpha: alpha ? "" : undefined,
            disabled,
            required,
            "aria-describedby": described || undefined,
            "data-nd-color-picker-native": "",
            "data-nd-spare": index === spare ? "" : undefined,
          }),
        ),
      ),
    ),
    h(
      "button",
      {
        key: "t",
        className: "nd-color-picker-trigger",
        id: control,
        type: "button",
        hidden: true,
        disabled,
        "aria-controls": panelId,
        "data-nd-color-picker-trigger": "",
      },
      h(
        "span",
        { key: "c", className: "nd-color-picker-chips", "data-nd-color-picker-chips": "" },
        natives
          .filter(Boolean)
          .map((colour, index) =>
            h("span", {
              key: index,
              className: "nd-color-picker-chip",
              style: { "--nd-color-picker-value": colour },
            }),
          ),
      ),
    ),
    h(
      "div",
      {
        key: "p",
        className: "nd-panel nd-color-picker-panel",
        id: panelId,
        hidden: true,
        "data-nd-color-picker-panel": "",
      },
      h(
        "div",
        { key: "h", className: "nd-color-picker-head" },
        palette
          ? h(
              Segmented,
              {
                key: "s",
                name: `${id}-tab`,
                labelledby: labelId,
                size: "small",
                className: "nd-color-picker-tabs",
              },
              h(SegmentedItem, {
                key: "a",
                value: "spectrum",
                label: spectrumLabel ?? "",
                selected: true,
              }),
              h(SegmentedItem, { key: "b", value: "palette", label: paletteLabel ?? "" }),
            )
          : null,
        h(Button, {
          key: "c",
          label: closeLabel ?? "",
          icon: close,
          hideLabel: true,
          variant: "transparent",
          size: "small",
          action: "color-picker-close",
        }),
      ),
      h(
        "div",
        { key: "s", className: "nd-color-picker-tab", "data-nd-tab": "spectrum" },
        h(
          "div",
          { key: "a", className: "nd-color-picker-area", "data-nd-color-picker-area": "" },
          h("span", { key: "t", className: "nd-color-picker-area-thumb", "aria-hidden": "true" }),
          // Two real ranges — see the Dioxus half for why a `<div>` with a
          // hand-written `role="slider"` is not the same thing.
          h("input", {
            key: "s",
            className: "nd-color-picker-axis",
            type: "range",
            id: `${id}-saturation`,
            min: "0",
            max: "100",
            step: "1",
            disabled,
            "aria-label": areaLabel,
            "data-nd-color-picker-axis": "saturation",
          }),
          h("input", {
            key: "v",
            className: "nd-color-picker-axis",
            type: "range",
            id: `${id}-value`,
            min: "0",
            max: "100",
            step: "1",
            disabled,
            "aria-label": areaLabel,
            "data-nd-color-picker-axis": "value",
          }),
        ),
        h("input", {
          key: "h",
          className: "nd-color-picker-track",
          type: "range",
          id: `${id}-hue`,
          min: "0",
          max: "360",
          step: "1",
          disabled,
          "aria-label": hueLabel,
          "data-nd-color-picker-hue": "",
        }),
        alpha
          ? h("input", {
              key: "o",
              className: "nd-color-picker-track",
              type: "range",
              id: `${id}-alpha`,
              min: "0",
              max: "100",
              step: "1",
              disabled,
              "aria-label": alphaLabel,
              "data-nd-color-picker-alpha": "",
            })
          : null,
      ),
      palette
        ? h(
            "div",
            { key: "g", className: "nd-color-picker-tab", "data-nd-tab": "palette", hidden: true },
            h("div", {
              key: "r",
              className: "nd-color-picker-grid",
              "data-nd-color-picker-grid": "",
            }),
          )
        : null,
      entry_
        ? h(
            "div",
            { key: "e", className: "nd-color-picker-entry-row" },
            h(
              "label",
              { key: "l", className: "nd-color-picker-entry-label", htmlFor: entryId },
              entryLabel ?? "",
            ),
            h("input", {
              key: "i",
              className: "nd-color-picker-entry",
              id: entryId,
              type: "text",
              autoComplete: "off",
              autoCorrect: "off",
              autoCapitalize: "none",
              spellCheck: "false",
              disabled,
              "aria-invalid": error ? "true" : undefined,
              "data-nd-color-picker-entry": "",
            }),
            eyedropper
              ? h(Button, {
                  key: "d",
                  label: eyedropperLabel ?? "",
                  icon: pipette,
                  hideLabel: true,
                  variant: "outline",
                  size: "small",
                  action: "color-picker-eyedropper",
                })
              : null,
          )
        : null,
      recent
        ? h(
            "div",
            {
              key: "r",
              className: "nd-color-picker-recent",
              "data-nd-color-picker-recent": "",
              hidden: true,
            },
            // The one label — see the Dioxus half.
            h(Label, { key: "l", kind: "name", size: "small", text: recentLabel ?? "" }),
            h("ul", {
              key: "s",
              className: "nd-color-picker-swatches",
              "data-nd-color-picker-recent-row": "",
            }),
            h(Button, {
              key: "f",
              label: forgetLabel ?? "",
              variant: "transparent",
              size: "small",
              action: "color-picker-forget",
            }),
          )
        : null,
      h(
        Disclosure,
        {
          key: "c",
          summary: codesLabel ?? "",
          name: "color-picker-codes",
          className: "nd-color-picker-codes",
        },
        h("ul", {
          key: "l",
          className: "nd-color-picker-code-list",
          "data-nd-color-picker-codes": "",
        }),
      ),
      multiple
        ? h(
            "div",
            { key: "m", className: "nd-color-picker-list" },
            h("ul", {
              key: "l",
              className: "nd-color-picker-committed",
              "data-nd-color-picker-committed": "",
            }),
            h(Button, {
              key: "a",
              label: addLabel ?? "",
              icon: plus,
              variant: "outline",
              size: "small",
              action: "color-picker-add",
            }),
          )
        : null,
    ),
    h("p", {
      key: "n",
      className: "nd-color-picker-note",
      id: `${id}-status`,
      role: "status",
      "aria-live": "polite",
      "aria-atomic": "true",
      "data-nd-color-picker-status": "",
    }),
    h(
      "template",
      { key: "1", "data-nd-chip-template": "" },
      h("span", { key: "c", className: "nd-color-picker-chip" }),
    ),
    h(
      "template",
      { key: "2", "data-nd-swatch-template": "" },
      h(
        "li",
        { key: "l" },
        h("button", {
          key: "b",
          className: "nd-color-picker-swatch",
          type: "button",
          "data-nd-color-picker-swatch": "",
        }),
      ),
    ),
    h(
      "template",
      { key: "3", "data-nd-committed-template": "" },
      h(
        "li",
        { key: "l", className: "nd-color-picker-committed-row" },
        h("span", { key: "c", className: "nd-color-picker-chip" }),
        h("code", { key: "v", className: "nd-code-inline nd-identifier", "data-nd-tone": "subtle" }),
        h(Button, {
          key: "r",
          label: "",
          icon: close,
          hideLabel: true,
          variant: "transparent",
          size: "small",
          action: "color-picker-remove",
        }),
      ),
    ),
    h(
      "template",
      { key: "4", "data-nd-code-template": "" },
      h(
        "li",
        { key: "l", className: "nd-color-picker-code" },
        h("span", { key: "n", className: "nd-color-picker-code-name nd-identifier" }),
        h("code", { key: "v", className: "nd-code-inline nd-identifier", "data-nd-tone": "subtle" }),
        h(
          "button",
          { key: "c", className: "nd-color-picker-copy", type: "button", "data-nd-color-copy": "" },
          h(Icon, { key: "i", icon: copy, size: "small" }),
          h(Icon, { key: "k", icon: check, size: "small" }),
        ),
      ),
    ),
  ),
  );
"#;

// --- The input controls ------------------------------------------------------

// SEVEN COMPONENTS, ONE ARRANGEMENT. Each of them is a `Field` — the question,
// the help and what went wrong — with a control inside it, and each computes
// the same two ids `Field` derives from `control` so its own
// `aria-describedby` points at the sentences the field drew. That is the one
// piece of duplication the extraction could not remove: this component cannot
// reach into its own children to write an attribute on one of them, and a
// browser test asserts the two agreed rather than a comment asking them to.

const INPUT_DIOXUS: &str = r#"    let id = format!("nd-input-{name}");
    let counter_id = format!("{id}-counter");
    // The sentences this box points at, in the order a reader should hear
    // them: what to do, what went wrong, how much room is left.
    let described: Vec<String> = [
        hint.as_ref().map(|_| format!("{id}-hint")),
        error.as_ref().map(|_| format!("{id}-error")),
        counter.then(|| counter_id.clone()),
    ]
    .into_iter()
    .flatten()
    .collect();
    let described = (!described.is_empty()).then(|| described.join(" "));
    let invalid = error.is_some().then_some("true");
    let password = matches!(kind, InputKind::Password);
    let box_class = format!("nd-input {class}");
    let box_class = box_class.trim().to_owned();
    let clear = clear_label.clone();
    let reveal = reveal_label.clone();
    // WHAT THE COUNTER SAYS BEFORE ANY SCRIPT RUNS.
    //
    // The `<p>` shipped empty and `aria-describedby` pointed at it, so a box
    // with `maxlength` stopped taking characters with nothing on the page
    // saying why. That is the exact failure `counter`'s own doc exists to
    // prevent, committed by the component the doc is about.
    //
    // `other` IS THE FORM RENDERED. Which form a count needs is a question
    // about the reader's language — Portuguese and Polish do not agree with
    // English or with each other — and a component that ships no runtime and
    // is handed no locale cannot answer it. `other` is the one category every
    // language has, and `inputs.js` writes the exact form as soon as it runs,
    // with the region silenced first so nothing is said twice.
    let counted = if counter {
        let shape = counter_label.clone().unwrap_or_default();
        let form = if shape.contains(':') {
            shape
                .split('|')
                .filter_map(|part| part.split_once(':'))
                .find(|(name, _)| name.trim() == "other")
                .map_or(String::new(), |(_, text)| text.to_owned())
        } else {
            shape
        };
        let start = value
            .as_deref()
            .unwrap_or_default()
            .chars()
            .count()
            .to_string();
        form.replace("{count}", &start)
            .replace("{max}", maxlength.as_deref().unwrap_or_default())
    } else {
        String::new()
    };

    rsx! {
        Field {
            control: "{id}",
            label: label.clone(),
            hint: hint.clone(),
            error: error.clone(),
            required,
            required_label: required_label.clone(),
            // SOMEWHERE FOR A SENTENCE TO GO. A required control is the one
            // that may be refused, and `ErrorSummary`'s `collect` puts the
            // browser's own complaint here rather than in a bubble nobody
            // can read twice. Empty, the stylesheet gives it no box.
            error_slot: required,
            size: match size {
                InputSize::Small => FieldSize::Small,
                InputSize::Medium => FieldSize::Medium,
                InputSize::Large => FieldSize::Large,
            },
            disabled,

            div {
                class: "{box_class}",
                "data-nd-input": "{name}",
                "data-nd-size": size.as_str(),
                "data-nd-glass": if glass { "true" } else { "false" },
                "data-nd-clearable": if clearable { "true" } else { "false" },
                "data-nd-counter": if counter { "true" } else { "false" },
                "data-nd-prefix": prefix.clone(),
                "data-nd-suffix": suffix.clone(),
                "data-nd-clear-label": clear_label.clone(),
                "data-nd-counter-label": counter_label.clone(),
                "data-nd-reveal-label": reveal_label.clone(),
                "data-nd-conceal-label": conceal_label.clone(),

                if let Some(icon) = icon {
                    Icon { icon, size: crate::components::IconSize::Small }
                }
                // DRAWN, NEVER SUBMITTED, and `aria-hidden` — announcing it
                // would tell a reader their answer contains something it does
                // not.
                if let Some(prefix) = prefix.clone() {
                    span { class: "nd-input-affix nd-identifier", "aria-hidden": "true", "{prefix}" }
                }
                input {
                    class: "nd-input-box",
                    id: "{id}",
                    r#type: kind.as_str(),
                    name: "{name}",
                    value,
                    placeholder,
                    required,
                    disabled,
                    readonly,
                    maxlength,
                    autocomplete,
                    pattern,
                    spellcheck: if spellcheck { "true" } else { "false" },
                    "aria-invalid": invalid,
                    "aria-describedby": described,
                    "data-nd-input-box": "",
                }
                if let Some(suffix) = suffix.clone() {
                    span { class: "nd-input-affix nd-identifier", "aria-hidden": "true", "{suffix}" }
                }
                // HIDDEN UNTIL THE SCRIPT CAN HONOUR IT, like every other
                // control in this system that does nothing without one.
                if clearable {
                    button {
                        class: "nd-input-control",
                        r#type: "button",
                        hidden: true,
                        disabled,
                        "aria-label": clear.clone(),
                        title: clear,
                        "data-nd-input-clear": "",
                        Icon { icon: IconDrawing::CLOSE, size: crate::components::IconSize::Small }
                    }
                }
                if password {
                    button {
                        class: "nd-input-control",
                        r#type: "button",
                        hidden: true,
                        disabled,
                        "aria-pressed": "false",
                        "aria-label": reveal.clone(),
                        title: reveal,
                        "data-nd-input-reveal": "",
                        // Both drawings are rendered and the state picks one:
                        // swapping a glyph from script is shipping a drawing
                        // to the script (`NOCTUA.md §9.5`).
                        Icon { icon: IconDrawing::EYE, size: crate::components::IconSize::Small }
                        Icon { icon: IconDrawing::EYE_OFF, size: crate::components::IconSize::Small }
                    }
                }
            }
            if counter {
                p {
                    class: "nd-input-counter",
                    id: "{counter_id}",
                    role: "status",
                    "aria-live": "polite",
                    "data-nd-input-counter": "",
                    // See above: the sentence, rendered rather than waited for.
                    "{counted}"
                }
            }
        }
    }
"#;

const INPUT_REACT: &str = r#"  const kind_ = kind ?? "text";
  const size_ = size ?? "medium";
  const id = `nd-input-${name}`;
  const counterId = `${id}-counter`;
  const described = [
    hint ? `${id}-hint` : null,
    error ? `${id}-error` : null,
    counter ? counterId : null,
  ]
    .filter(Boolean)
    .join(" ");
  const password = kind_ === "password";
  const boxClass = ["nd-input", className].filter(Boolean).join(" ");
  // See the Dioxus half: the counter says its sentence before any script runs,
  // and `other` is the form a component handed no locale may choose.
  let counted = "";
  if (counter) {
    const shape = String(counterLabel ?? "");
    let form = shape;
    if (shape.includes(":")) {
      form = "";
      for (const part of shape.split("|")) {
        const at = part.indexOf(":");
        if (at >= 0 && part.slice(0, at).trim() === "other") form = part.slice(at + 1);
      }
    }
    counted = form
      .split("{count}")
      .join(String(String(value ?? "").length))
      .split("{max}")
      .join(String(maxlength ?? ""));
  }

  return h(
    Field,
    {
      control: id,
      label,
      hint,
      error,
      required,
      requiredLabel,
      // See the Dioxus half: somewhere for a refused control's sentence to go.
      errorSlot: required,
      size: size_,
      disabled,
    },
    h(
      "div",
      {
        key: "b",
        className: boxClass,
        "data-nd-input": name,
        "data-nd-size": size_,
        "data-nd-glass": String(glass ?? false),
        "data-nd-clearable": String(clearable ?? false),
        "data-nd-counter": String(counter ?? false),
        "data-nd-prefix": prefix,
        "data-nd-suffix": suffix,
        "data-nd-clear-label": clearLabel,
        "data-nd-counter-label": counterLabel,
        "data-nd-reveal-label": revealLabel,
        "data-nd-conceal-label": concealLabel,
        ...rest,
      },
      icon ? h(Icon, { key: "i", icon, size: "small" }) : null,
      prefix
        ? h("span", { key: "p", className: "nd-input-affix nd-identifier", "aria-hidden": "true" }, prefix)
        : null,
      h("input", {
        key: "x",
        className: "nd-input-box",
        id,
        type: kind_,
        name,
        defaultValue: value,
        placeholder,
        required,
        disabled,
        readOnly: readonly,
        maxLength: maxlength,
        autoComplete: autocomplete,
        pattern,
        spellCheck: String(spellcheck ?? false),
        "aria-invalid": error ? "true" : undefined,
        "aria-describedby": described || undefined,
        "data-nd-input-box": "",
      }),
      suffix
        ? h("span", { key: "s", className: "nd-input-affix nd-identifier", "aria-hidden": "true" }, suffix)
        : null,
      clearable
        ? h(
            "button",
            {
              key: "c",
              className: "nd-input-control",
              type: "button",
              hidden: true,
              disabled,
              "aria-label": clearLabel,
              title: clearLabel,
              "data-nd-input-clear": "",
            },
            h(Icon, { key: "i", icon: close, size: "small" }),
          )
        : null,
      password
        ? h(
            "button",
            {
              key: "r",
              className: "nd-input-control",
              type: "button",
              hidden: true,
              disabled,
              "aria-pressed": "false",
              "aria-label": revealLabel,
              title: revealLabel,
              "data-nd-input-reveal": "",
            },
            h(Icon, { key: "a", icon: eye, size: "small" }),
            h(Icon, { key: "b", icon: eyeOff, size: "small" }),
          )
        : null,
    ),
    counter
      ? h(
          "p",
          {
            key: "n",
            className: "nd-input-counter",
            id: counterId,
            role: "status",
            "aria-live": "polite",
            "data-nd-input-counter": "",
          },
          counted,
        )
      : null,
  );
"#;

const TEXTAREA_DIOXUS: &str = r#"    let id = format!("nd-textarea-{name}");
    let counter_id = format!("{id}-counter");
    let described: Vec<String> = [
        hint.as_ref().map(|_| format!("{id}-hint")),
        error.as_ref().map(|_| format!("{id}-error")),
        counter.then(|| counter_id.clone()),
    ]
    .into_iter()
    .flatten()
    .collect();
    let described = (!described.is_empty()).then(|| described.join(" "));
    let invalid = error.is_some().then_some("true");
    let box_class = format!("nd-textarea {class}");
    let box_class = box_class.trim().to_owned();
    // See `INPUT_DIOXUS`: the sentence is rendered rather than waited for, and
    // `other` is the form a component handed no locale may choose. Repeated
    // rather than shared for the reason at the head of this section — one
    // component cannot reach into another's markup to write on it.
    let counted = if counter {
        let shape = counter_label.clone().unwrap_or_default();
        let form = if shape.contains(':') {
            shape
                .split('|')
                .filter_map(|part| part.split_once(':'))
                .find(|(name, _)| name.trim() == "other")
                .map_or(String::new(), |(_, text)| text.to_owned())
        } else {
            shape
        };
        let start = value
            .as_deref()
            .unwrap_or_default()
            .chars()
            .count()
            .to_string();
        form.replace("{count}", &start)
            .replace("{max}", maxlength.as_deref().unwrap_or_default())
    } else {
        String::new()
    };

    rsx! {
        Field {
            control: "{id}",
            label: label.clone(),
            hint: hint.clone(),
            error: error.clone(),
            required,
            required_label: required_label.clone(),
            // SOMEWHERE FOR A SENTENCE TO GO. A required control is the one
            // that may be refused, and `ErrorSummary`'s `collect` puts the
            // browser's own complaint here rather than in a bubble nobody
            // can read twice. Empty, the stylesheet gives it no box.
            error_slot: required,
            size: match size {
                TextareaSize::Small => FieldSize::Small,
                TextareaSize::Medium => FieldSize::Medium,
                TextareaSize::Large => FieldSize::Large,
            },
            disabled,

            // A BOX AROUND THE BOX, AND IT IS `Input`'S ARRANGEMENT EXACTLY.
            //
            // `.nd-textarea` used to be the `<textarea>` itself, which made it
            // the one control on the site that ignored the reader's corner
            // preset: a `<textarea>` is a scroll container, a radius on a
            // scroller lets the bar paint through the corner, so it shipped
            // with no radius at all. The border, the fill, the corners and the
            // frost belong on something that does not scroll; the scrolling
            // belongs to the control inside it. `.nd-panel` and `.nd-codeblock`
            // are the same shape for the same reason.
            //
            // It also gives the frost somewhere to land. The one glass recipe
            // paints its sheen in an `::after`, and a `<textarea>` is a
            // replaced element with no pseudo-elements at all.
            div {
                class: "{box_class}",
                "data-nd-textarea": "{name}",
                "data-nd-size": size.as_str(),
                "data-nd-glass": if glass { "true" } else { "false" },
                "data-nd-autosize": if autosize { "true" } else { "false" },
                "data-nd-max-rows": max_rows,
                "data-nd-resize": resize.as_str(),
                "data-nd-counter": if counter { "true" } else { "false" },
                "data-nd-counter-label": counter_label,

                textarea {
                    class: "nd-textarea-box",
                    id: "{id}",
                    name: "{name}",
                    rows,
                    placeholder,
                    required,
                    disabled,
                    readonly,
                    maxlength,
                    spellcheck: if spellcheck { "true" } else { "false" },
                    "aria-invalid": invalid,
                    "aria-describedby": described,
                    "data-nd-textarea-box": "",
                    // THE VALUE IS THE CONTENT, not an attribute — which is why
                    // a value with a newline in it survives the round trip.
                    {value.unwrap_or_default()}
                }
            }
            if counter {
                p {
                    class: "nd-textarea-counter",
                    id: "{counter_id}",
                    role: "status",
                    "aria-live": "polite",
                    "data-nd-textarea-counter": "",
                    // See above: the sentence, rendered rather than waited for.
                    "{counted}"
                }
            }
        }
    }
"#;

const TEXTAREA_REACT: &str = r#"  const size_ = size ?? "medium";
  const resize_ = resize ?? "vertical";
  const id = `nd-textarea-${name}`;
  const counterId = `${id}-counter`;
  const described = [
    hint ? `${id}-hint` : null,
    error ? `${id}-error` : null,
    counter ? counterId : null,
  ]
    .filter(Boolean)
    .join(" ");
  const boxClass = ["nd-textarea", className].filter(Boolean).join(" ");
  // See the Dioxus half, and `INPUT_REACT`: the sentence is rendered, and
  // `other` is the form a component handed no locale may choose.
  let counted = "";
  if (counter) {
    const shape = String(counterLabel ?? "");
    let form = shape;
    if (shape.includes(":")) {
      form = "";
      for (const part of shape.split("|")) {
        const at = part.indexOf(":");
        if (at >= 0 && part.slice(0, at).trim() === "other") form = part.slice(at + 1);
      }
    }
    counted = form
      .split("{count}")
      .join(String(String(value ?? "").length))
      .split("{max}")
      .join(String(maxlength ?? ""));
  }

  return h(
    Field,
    {
      control: id,
      label,
      hint,
      error,
      required,
      requiredLabel,
      // See the Dioxus half: somewhere for a refused control's sentence to go.
      errorSlot: required,
      size: size_,
      disabled,
    },
    // See the Dioxus half: a box around the box, so the corners, the border
    // and the frost sit on something that does not scroll.
    h(
      "div",
      {
        key: "b",
        className: boxClass,
        "data-nd-textarea": name,
        "data-nd-size": size_,
        "data-nd-glass": String(glass ?? false),
        "data-nd-autosize": String(autosize ?? false),
        "data-nd-max-rows": maxRows,
        "data-nd-resize": resize_,
        "data-nd-counter": String(counter ?? false),
        "data-nd-counter-label": counterLabel,
        ...rest,
      },
      h("textarea", {
        key: "x",
        className: "nd-textarea-box",
        id,
        name,
        rows,
        placeholder,
        required,
        disabled,
        readOnly: readonly,
        maxLength: maxlength,
        spellCheck: String(spellcheck ?? true),
        "aria-invalid": error ? "true" : undefined,
        "aria-describedby": described || undefined,
        "data-nd-textarea-box": "",
        // See the Dioxus half: the value is the content.
        defaultValue: value,
      }),
    ),
    counter
      ? h(
          "p",
          {
            key: "n",
            className: "nd-textarea-counter",
            id: counterId,
            role: "status",
            "aria-live": "polite",
            "data-nd-textarea-counter": "",
          },
          counted,
        )
      : null,
  );
"#;

const NUMBER_INPUT_DIOXUS: &str = r#"    let id = format!("nd-number-input-{name}");
    // THE ERROR ID IS ALWAYS ONE OF THEM, because the slot below is always
    // drawn: what this control says about a value out of range is written
    // there, and a box that points at nothing carries none of it.
    let described: Vec<String> = [
        hint.as_ref().map(|_| format!("{id}-hint")),
        Some(format!("{id}-error")),
    ]
    .into_iter()
    .flatten()
    .collect();
    let described = described.join(" ");
    let invalid = error.is_some().then_some("true");
    let box_class = format!("nd-number-input {class}");
    let box_class = box_class.trim().to_owned();
    let up = increment_label.clone();
    let down = decrement_label.clone();
    // THE KEYBOARD `step` ASKS FOR. A decimal key on a field counting people
    // types a character the field refuses, and a field taking cents without
    // one asks for a keyboard change per value.
    let mode = if step.as_deref().unwrap_or_default().contains('.') {
        "decimal"
    } else {
        "numeric"
    };
    // THE PLATFORM'S OWN REFUSAL, AND IT NEEDS NO SCRIPT. Digits, a sign, a
    // decimal mark and the separators `group` draws are accepted. Anything
    // else is refused by the browser before a script exists to say more.
    let shape = "-?[0-9]+([.,\\u00A0\\u202F ][0-9]{3})*([.,][0-9]+)?";
    // The steppers move a value, so `readonly` refuses them as `disabled`
    // does. The contract says the same in as many words.
    let stuck = disabled || readonly;

    rsx! {
        Field {
            control: "{id}",
            label: label.clone(),
            hint: hint.clone(),
            error: error.clone(),
            required,
            required_label: required_label.clone(),
            // SOMEWHERE FOR A SENTENCE TO GO, ON EVERY NUMBER FIELD. This one
            // has three sentences of its own — `min_label`, `max_label` and
            // `invalid_label` — so the slot cannot wait for `required`, which
            // is what left a bounded field marking itself wrong in silence.
            // `ErrorSummary`'s `collect` writes here too. Empty, the
            // stylesheet gives it no box.
            error_slot: true,
            size: match size {
                NumberInputSize::Small => FieldSize::Small,
                NumberInputSize::Medium => FieldSize::Medium,
                NumberInputSize::Large => FieldSize::Large,
            },
            disabled,

            div {
                class: "{box_class}",
                "data-nd-number-input": "{name}",
                "data-nd-size": size.as_str(),
                "data-nd-glass": if glass { "true" } else { "false" },
                "data-nd-steppers": if steppers { "true" } else { "false" },
                // WHAT THE STEPPERS ARE FOR, AND THE STYLESHEET DOES THE REST.
                // `primary` puts one full control either side of the value and
                // `assist` keeps the two small ones at the end of the box. The
                // markup is the same either way, so the runtime finds the same
                // two controls and the arrangement is a question about
                // painting rather than about structure.
                "data-nd-stepping": stepping.as_str(),
                "data-nd-group": if group { "true" } else { "false" },
                "data-nd-min": min,
                "data-nd-max": max,
                "data-nd-step": step,
                "data-nd-big-step": big_step,
                "data-nd-prefix": prefix.clone(),
                "data-nd-suffix": suffix.clone(),
                "data-nd-min-label": min_label,
                "data-nd-max-label": max_label,
                "data-nd-invalid-label": invalid_label,
                "data-nd-increment-label": increment_label.clone(),
                "data-nd-decrement-label": decrement_label.clone(),

                if let Some(prefix) = prefix.clone() {
                    span { class: "nd-number-input-affix nd-identifier", "aria-hidden": "true", "{prefix}" }
                }
                // `type="text"` WITH A NUMERIC KEYBOARD, never `type="number"`
                // — the contract says why at length. The `pattern` is what
                // keeps the platform's own refusal working with no script.
                input {
                    class: "nd-number-input-box",
                    id: "{id}",
                    r#type: "text",
                    name: "{name}",
                    value,
                    placeholder,
                    required,
                    disabled,
                    readonly,
                    inputmode: mode,
                    pattern: shape,
                    autocomplete: "off",
                    "aria-invalid": invalid,
                    "aria-describedby": described,
                    "data-nd-number-input-box": "",
                }
                if let Some(suffix) = suffix.clone() {
                    span { class: "nd-number-input-affix nd-identifier", "aria-hidden": "true", "{suffix}" }
                }
                if steppers {
                    // HIDDEN UNTIL THE SCRIPT CAN HONOUR THEM, exactly like
                    // `Input`'s clear and reveal controls. Two buttons that
                    // move nothing are two buttons a reader presses twice.
                    span {
                        class: "nd-number-input-steppers",
                        hidden: true,
                        "data-nd-number-input-steppers": "",
                        // NOT TAB STOPS. The arrow keys on the field already do
                        // this, and two extra stops per number field is a form
                        // that takes three times as many presses to cross.
                        button {
                            class: "nd-number-input-step",
                            r#type: "button",
                            tabindex: "-1",
                            disabled: stuck,
                            "aria-label": up.clone(),
                            title: up,
                            "data-nd-number-input-step": "up",
                            Icon { icon: IconDrawing::PLUS, size: crate::components::IconSize::Small }
                        }
                        button {
                            class: "nd-number-input-step",
                            r#type: "button",
                            tabindex: "-1",
                            disabled: stuck,
                            "aria-label": down.clone(),
                            title: down,
                            "data-nd-number-input-step": "down",
                            Icon { icon: IconDrawing::MINUS, size: crate::components::IconSize::Small }
                        }
                    }
                }
            }
        }
    }
"#;

const NUMBER_INPUT_REACT: &str = r#"  const size_ = size ?? "medium";
  const stepping_ = stepping ?? "assist";
  const id = `nd-number-input-${name}`;
  // See the Dioxus half: the error id is always one of them, because the slot
  // is always drawn.
  const described = [hint ? `${id}-hint` : null, `${id}-error`].filter(Boolean).join(" ");
  const boxClass = ["nd-number-input", className].filter(Boolean).join(" ");
  // See the Dioxus half: the keyboard `step` asks for, the platform's own
  // refusal, and the two states that refuse a stepper.
  const mode = String(step ?? "").includes(".") ? "decimal" : "numeric";
  const shape = "-?[0-9]+([.,\\u00A0\\u202F ][0-9]{3})*([.,][0-9]+)?";
  const stuck = Boolean(disabled) || Boolean(readonly);

  return h(
    Field,
    {
      control: id,
      label,
      hint,
      error,
      required,
      requiredLabel,
      // See the Dioxus half: this control has three sentences of its own, so
      // the slot is drawn for every number field rather than for a required
      // one.
      errorSlot: true,
      size: size_,
      disabled,
    },
    h(
      "div",
      {
        key: "b",
        className: boxClass,
        "data-nd-number-input": name,
        "data-nd-size": size_,
        "data-nd-glass": String(glass ?? false),
        "data-nd-steppers": String(steppers ?? true),
        // See the Dioxus half: the arrangement is painting, not structure, so
        // both forms ship the same two controls.
        "data-nd-stepping": stepping_,
        "data-nd-group": String(group ?? false),
        "data-nd-min": min,
        "data-nd-max": max,
        "data-nd-step": step,
        "data-nd-big-step": bigStep,
        "data-nd-prefix": prefix,
        "data-nd-suffix": suffix,
        "data-nd-min-label": minLabel,
        "data-nd-max-label": maxLabel,
        "data-nd-invalid-label": invalidLabel,
        "data-nd-increment-label": incrementLabel,
        "data-nd-decrement-label": decrementLabel,
        ...rest,
      },
      prefix
        ? h("span", { key: "p", className: "nd-number-input-affix nd-identifier", "aria-hidden": "true" }, prefix)
        : null,
      h("input", {
        key: "x",
        className: "nd-number-input-box",
        id,
        type: "text",
        name,
        defaultValue: value,
        placeholder,
        required,
        disabled,
        readOnly: readonly,
        inputMode: mode,
        pattern: shape,
        autoComplete: "off",
        "aria-invalid": error ? "true" : undefined,
        "aria-describedby": described || undefined,
        "data-nd-number-input-box": "",
      }),
      suffix
        ? h("span", { key: "s", className: "nd-number-input-affix nd-identifier", "aria-hidden": "true" }, suffix)
        : null,
      (steppers ?? true)
        ? h(
            "span",
            {
              key: "t",
              className: "nd-number-input-steppers",
              // See the Dioxus half: hidden until the script can honour them.
              hidden: true,
              "data-nd-number-input-steppers": "",
            },
            h(
              "button",
              {
                key: "u",
                className: "nd-number-input-step",
                type: "button",
                tabIndex: -1,
                disabled: stuck,
                "aria-label": incrementLabel,
                title: incrementLabel,
                "data-nd-number-input-step": "up",
              },
              h(Icon, { key: "i", icon: plus, size: "small" }),
            ),
            h(
              "button",
              {
                key: "d",
                className: "nd-number-input-step",
                type: "button",
                tabIndex: -1,
                disabled: stuck,
                "aria-label": decrementLabel,
                title: decrementLabel,
                "data-nd-number-input-step": "down",
              },
              h(Icon, { key: "i", icon: minus, size: "small" }),
            ),
          )
        : null,
    ),
  );
"#;

const SLIDER_DIOXUS: &str = r#"    let id = format!("nd-slider-{name}");
    let described: Vec<String> = [
        hint.as_ref().map(|_| format!("{id}-hint")),
        error.as_ref().map(|_| format!("{id}-error")),
    ]
    .into_iter()
    .flatten()
    .collect();
    let described = (!described.is_empty()).then(|| described.join(" "));
    let invalid = error.is_some().then_some("true");
    let track_class = format!("nd-slider {class}");
    let track_class = track_class.trim().to_owned();
    // A RANGE IS TWO NAMED CONTROLS. The `<legend>` is the question they share
    // and each thumb takes its own name from the end it is, because *slider*
    // and *slider* is a pair a reader cannot tell apart.
    let start_id = if range { format!("{id}-start") } else { id.clone() };
    let end_id = format!("{id}-end");
    // Two thumbs read the same three bounds and the same sentences, and `rsx!`
    // moves each into the first that names it. One binding per use.
    let (min_end, max_end, step_end) = (min.clone(), max.clone(), step.clone());
    let described_end = described.clone();

    // THE VALUE AND THE SCALE ARE DRAWN BY THE COMPONENT NOW.
    //
    // Both shipped as empty boxes that `inputs.js` filled. `output`'s own doc
    // says it is on by default "so users can read the value they have set",
    // and with the script blocked it was an empty `<output>` saying nothing at
    // all — the prop's single promise, unkept. The marks were the same: five
    // ticks the caller asked for and no ticks on the screen.
    //
    // The runtime still writes both on every move. What changes is that the
    // first state is rendered rather than waited for, so nothing arrives late
    // and nothing is missing when nothing arrives.
    let shape = output_label.clone();
    let shown = |number: &str| match shape.as_deref() {
        Some(text) if !text.is_empty() => text.replace("{value}", number),
        _ => number.to_owned(),
    };
    let read = |text: Option<&str>, fallback: f64| {
        text.and_then(|value| value.trim().parse::<f64>().ok())
            .unwrap_or(fallback)
    };
    // THE PLATFORM'S OWN DEFAULTS, because they are what the browser will put
    // in the box: a `<input type="range">` with no value sits in the middle of
    // a range that runs from zero to a hundred unless told otherwise.
    let low = read(min.as_deref(), 0.0);
    let high = read(max.as_deref(), 100.0);
    let middle = low + (high - low) / 2.0;
    let tidy = |number: f64| {
        if number.fract() == 0.0 {
            format!("{number:.0}")
        } else {
            format!("{number}")
        }
    };
    let start_value = value.clone().unwrap_or_else(|| tidy(middle));
    let end_value = end.clone().unwrap_or_else(|| tidy(middle));
    let reading = if range {
        format!("{} – {}", shown(&start_value), shown(&end_value))
    } else {
        shown(&start_value)
    };
    // The words, read position by position against the values, which is
    // exactly what `inputs.js` does with the same two props.
    let words = mark_labels.clone().unwrap_or_default();
    let ticks: Vec<String> = marks
        .as_deref()
        .unwrap_or_default()
        .split_whitespace()
        .enumerate()
        .map(|(index, tick)| {
            words
                .split('|')
                .nth(index)
                .map(str::trim)
                .filter(|word| !word.is_empty())
                .unwrap_or(tick)
                .to_owned()
        })
        .collect();

    rsx! {
        Field {
            control: "{start_id}",
            label: label.clone(),
            hint: hint.clone(),
            error: error.clone(),
            required,
            required_label: required_label.clone(),
            kind: if range { FieldKind::Group } else { FieldKind::Single },
            size: match size {
                SliderSize::Small => FieldSize::Small,
                SliderSize::Medium => FieldSize::Medium,
                SliderSize::Large => FieldSize::Large,
            },
            disabled,

            div {
                class: "{track_class}",
                "data-nd-slider": "{name}",
                "data-nd-size": size.as_str(),
                "data-nd-tone": tone.as_str(),
                "data-nd-glass": if glass { "true" } else { "false" },
                "data-nd-range": if range { "true" } else { "false" },
                "data-nd-output": if output { "true" } else { "false" },
                "data-nd-output-label": output_label,
                "data-nd-marks": marks.clone(),
                "data-nd-mark-labels": mark_labels,

                div { class: "nd-slider-rails", "data-nd-slider-rails": "",
                    input {
                        class: "nd-slider-track",
                        id: "{start_id}",
                        r#type: "range",
                        name: if range { format!("{name}-start") } else { name.clone() },
                        value,
                        min,
                        max,
                        step,
                        required,
                        disabled,
                        "aria-invalid": invalid,
                        "aria-describedby": described.clone(),
                        "data-nd-slider-thumb": "start",
                    }
                    if range {
                        input {
                            class: "nd-slider-track",
                            id: "{end_id}",
                            r#type: "range",
                            name: "{name}-end",
                            value: end,
                            min: min_end,
                            max: max_end,
                            step: step_end,
                            disabled,
                            "aria-describedby": described_end,
                            "data-nd-slider-thumb": "end",
                        }
                    }
                    // A DRAWING OF THE SCALE, and `aria-hidden` because the
                    // scale is already announced by the range's own min and
                    // max — five tick numbers before every value is noise.
                    if marks.is_some() {
                        span { class: "nd-slider-marks", "aria-hidden": "true", "data-nd-slider-marks": "",
                            for word in ticks {
                                span { class: "nd-slider-mark", "{word}" }
                            }
                        }
                    }
                }
                if output {
                    // NOT A LIVE REGION. The range announces its own value on
                    // every change; a second announcement is the same number
                    // said twice.
                    output { class: "nd-slider-output", "for": "{start_id}", "data-nd-slider-output": "", "{reading}" }
                }
            }
        }
    }
"#;

const SLIDER_REACT: &str = r#"  const size_ = size ?? "medium";
  const tone_ = tone ?? "accent";
  const id = `nd-slider-${name}`;
  const described = [hint ? `${id}-hint` : null, error ? `${id}-error` : null]
    .filter(Boolean)
    .join(" ");
  const trackClass = ["nd-slider", className].filter(Boolean).join(" ");
  // See the Dioxus half: a range is two named controls under one legend.
  const startId = range ? `${id}-start` : id;
  // See the Dioxus half: the value and the scale are drawn by the component,
  // because an empty `<output>` is the one thing `output` promises not to be.
  const shown = (number) =>
    outputLabel ? String(outputLabel).split("{value}").join(String(number)) : String(number);
  const read = (text, fallback) => {
    const number = Number(text);
    return String(text ?? "").trim() === "" || !Number.isFinite(number) ? fallback : number;
  };
  const low = read(min, 0);
  const high = read(max, 100);
  const middle = low + (high - low) / 2;
  const startValue = value ?? String(middle);
  const endValue = end ?? String(middle);
  const reading = range ? `${shown(startValue)} – ${shown(endValue)}` : shown(startValue);
  const ticks = String(marks ?? "")
    .split(/\s+/)
    .filter(Boolean)
    .map((tick, index) => String(markLabels ?? "").split("|")[index]?.trim() || tick);

  return h(
    Field,
    {
      control: startId,
      label,
      hint,
      error,
      required,
      requiredLabel,
      kind: range ? "group" : "single",
      size: size_,
      disabled,
    },
    h(
      "div",
      {
        key: "b",
        className: trackClass,
        "data-nd-slider": name,
        "data-nd-size": size_,
        "data-nd-tone": tone_,
        "data-nd-glass": String(glass ?? false),
        "data-nd-range": String(range ?? false),
        "data-nd-output": String(output ?? true),
        "data-nd-output-label": outputLabel,
        "data-nd-marks": marks,
        "data-nd-mark-labels": markLabels,
        ...rest,
      },
      h(
        "div",
        { key: "r", className: "nd-slider-rails", "data-nd-slider-rails": "" },
        h("input", {
          key: "s",
          className: "nd-slider-track",
          id: startId,
          type: "range",
          name: range ? `${name}-start` : name,
          defaultValue: value,
          min,
          max,
          step,
          required,
          disabled,
          "aria-invalid": error ? "true" : undefined,
          "aria-describedby": described || undefined,
          "data-nd-slider-thumb": "start",
        }),
        range
          ? h("input", {
              key: "e",
              className: "nd-slider-track",
              id: `${id}-end`,
              type: "range",
              name: `${name}-end`,
              defaultValue: end,
              min,
              max,
              step,
              disabled,
              "aria-describedby": described || undefined,
              "data-nd-slider-thumb": "end",
            })
          : null,
        marks
          ? h(
              "span",
              {
                key: "m",
                className: "nd-slider-marks",
                "aria-hidden": "true",
                "data-nd-slider-marks": "",
              },
              ticks.map((word, index) =>
                h("span", { key: `t${index}`, className: "nd-slider-mark" }, word),
              ),
            )
          : null,
      ),
      (output ?? true)
        ? h(
            "output",
            {
              key: "o",
              className: "nd-slider-output",
              htmlFor: startId,
              "data-nd-slider-output": "",
            },
            reading,
          )
        : null,
    ),
  );
"#;

const FILE_INPUT_DIOXUS: &str = r#"    let id = format!("nd-file-input-{name}");
    let list_id = format!("{id}-files");
    let described: Vec<String> = [
        hint.as_ref().map(|_| format!("{id}-hint")),
        error.as_ref().map(|_| format!("{id}-error")),
    ]
    .into_iter()
    .flatten()
    .collect();
    let described = (!described.is_empty()).then(|| described.join(" "));
    let invalid = error.is_some().then_some("true");
    let area_class = format!("nd-file-input {class}");
    let area_class = area_class.trim().to_owned();
    // One binding per use: `rsx!` moves a value into the nested block before
    // the root's attributes are read, so a second copy is the only order that
    // borrow-checks. `Select` and `TagPicker` do the same.
    let empty = empty_label.clone();

    rsx! {
        Field {
            control: "{id}",
            label: label.clone(),
            hint: hint.clone(),
            error: error.clone(),
            required,
            required_label: required_label.clone(),
            // SOMEWHERE FOR A SENTENCE TO GO. A required control is the one
            // that may be refused, and `ErrorSummary`'s `collect` puts the
            // browser's own complaint here rather than in a bubble nobody
            // can read twice. Empty, the stylesheet gives it no box.
            error_slot: required,
            size: match size {
                FileInputSize::Small => FieldSize::Small,
                FileInputSize::Medium => FieldSize::Medium,
                FileInputSize::Large => FieldSize::Large,
            },
            disabled,

            div {
                class: "{area_class}",
                "data-nd-file-input": "{name}",
                "data-nd-size": size.as_str(),
                "data-nd-glass": if glass { "true" } else { "false" },
                "data-nd-drop": if drop { "true" } else { "false" },
                "data-nd-max": max,
                "data-nd-max-size": max_size,
                "data-nd-max-label": max_label,
                "data-nd-max-size-label": max_size_label,
                "data-nd-accept-label": accept_label,
                "data-nd-remove-label": remove_label,
                "data-nd-empty-label": empty_label.clone(),
                "data-nd-status": status,
                "data-nd-added-label": added_label,
                "data-nd-removed-label": removed_label,

                // THE AREA IS THE `<label>` OF A REAL FILE INPUT. That one
                // decision is what makes it focusable, pressable, announced
                // and usable by somebody who has never dragged a file —
                // because all four are the label's, not ours.
                label {
                    class: "nd-file-input-area",
                    r#for: "{id}",
                    "data-nd-file-input-area": "",
                    Icon { icon: IconDrawing::UPLOAD }
                    span { class: "nd-file-input-browse", "{browse_label}" }
                    if let Some(drop_label) = drop_label {
                        span { class: "nd-file-input-drop", "{drop_label}" }
                    }
                }
                // CLIPPED, NOT `display: none` — that takes it out of the
                // accessibility tree and out of the tab order, and what is
                // left is a picture of a file field.
                input {
                    class: "nd-file-input-native",
                    id: "{id}",
                    r#type: "file",
                    name: "{name}",
                    accept,
                    multiple,
                    required,
                    disabled,
                    "aria-invalid": invalid,
                    "aria-describedby": described,
                    "data-nd-file-input-native": "",
                }
                ul {
                    class: "nd-file-input-list",
                    id: "{list_id}",
                    "data-nd-file-input-list": "",
                    if let Some(empty) = empty {
                        li { class: "nd-file-input-empty", "data-nd-file-input-empty": "", "{empty}" }
                    }
                }
                p {
                    class: "nd-file-input-note",
                    id: "{id}-status",
                    role: "status",
                    "aria-live": "polite",
                    "aria-atomic": "true",
                    "data-nd-file-input-status": "",
                }
            }
            // The script does not know what a row looks like. It copies this.
            template { "data-nd-file-template": "",
                li { class: "nd-file-input-row",
                    span { class: "nd-file-input-name nd-identifier" }
                    span { class: "nd-file-input-size nd-identifier" }
                    Button {
                        label: "",
                        icon: IconDrawing::CLOSE,
                        hide_label: true,
                        variant: ButtonVariant::Transparent,
                        size: ButtonSize::Small,
                        action: "file-input-remove",
                    }
                }
            }
        }
    }
"#;

const FILE_INPUT_REACT: &str = r#"  const size_ = size ?? "medium";
  const id = `nd-file-input-${name}`;
  const described = [hint ? `${id}-hint` : null, error ? `${id}-error` : null]
    .filter(Boolean)
    .join(" ");
  const areaClass = ["nd-file-input", className].filter(Boolean).join(" ");

  return h(
    Field,
    {
      control: id,
      label,
      hint,
      error,
      required,
      requiredLabel,
      // See the Dioxus half: somewhere for a refused control's sentence to go.
      errorSlot: required,
      size: size_,
      disabled,
    },
    h(
      "div",
      {
        key: "b",
        className: areaClass,
        "data-nd-file-input": name,
        "data-nd-size": size_,
        "data-nd-glass": String(glass ?? false),
        "data-nd-drop": String(drop ?? true),
        "data-nd-max": max,
        "data-nd-max-size": maxSize,
        "data-nd-max-label": maxLabel,
        "data-nd-max-size-label": maxSizeLabel,
        "data-nd-accept-label": acceptLabel,
        "data-nd-remove-label": removeLabel,
        "data-nd-empty-label": emptyLabel,
        "data-nd-status": status,
        "data-nd-added-label": addedLabel,
        "data-nd-removed-label": removedLabel,
        ...rest,
      },
      h(
        "label",
        { key: "a", className: "nd-file-input-area", htmlFor: id, "data-nd-file-input-area": "" },
        h(Icon, { key: "i", icon: upload }),
        h("span", { key: "b", className: "nd-file-input-browse" }, browseLabel),
        dropLabel ? h("span", { key: "d", className: "nd-file-input-drop" }, dropLabel) : null,
      ),
      h("input", {
        key: "x",
        className: "nd-file-input-native",
        id,
        type: "file",
        name,
        accept,
        multiple,
        required,
        disabled,
        "aria-invalid": error ? "true" : undefined,
        "aria-describedby": described || undefined,
        "data-nd-file-input-native": "",
      }),
      h(
        "ul",
        { key: "l", className: "nd-file-input-list", id: `${id}-files`, "data-nd-file-input-list": "" },
        emptyLabel
          ? h(
              "li",
              { key: "e", className: "nd-file-input-empty", "data-nd-file-input-empty": "" },
              emptyLabel,
            )
          : null,
      ),
      h("p", {
        key: "n",
        className: "nd-file-input-note",
        id: `${id}-status`,
        role: "status",
        "aria-live": "polite",
        "aria-atomic": "true",
        "data-nd-file-input-status": "",
      }),
    ),
    h(
      "template",
      { key: "t", "data-nd-file-template": "" },
      h(
        "li",
        { key: "r", className: "nd-file-input-row" },
        h("span", { key: "n", className: "nd-file-input-name nd-identifier" }),
        h("span", { key: "s", className: "nd-file-input-size nd-identifier" }),
        h(Button, {
          key: "x",
          label: "",
          icon: close,
          hideLabel: true,
          variant: "transparent",
          size: "small",
          action: "file-input-remove",
        }),
      ),
    ),
  );
"#;

const TIME_INPUT_DIOXUS: &str = r#"    let id = format!("nd-time-input-{name}");
    // FROM THE SAME `control` THE FIELD GETS, and that is the whole reason the
    // rule is stated in `Field.control`'s own documentation: this pointed at
    // `{id}-hint` while the field drew `{id}-hour-hint`, and every sentence
    // under a `TimeInput` was described by an element that did not exist.
    // Nothing reports a dangling `aria-describedby` — not a browser, not a
    // validator — which is why `every_describedby_points_at_something` is a
    // browser test rather than a comment asking for care.
    let control = format!("{id}-hour");
    // THE GROUP DESCRIBES ITSELF NOW. `Field { kind: Group }` is the
    // `<fieldset>`, and a reader entering one hears the hint once rather than
    // once per box — which is what these three used to do, because the field
    // did not yet write it.
    let invalid = error.is_some().then_some("true");
    let root = format!("nd-time-input {class}");
    let root = root.trim().to_owned();
    // SERVED AS TWENTY-FOUR HOURS, WHATEVER `clock` SAYS. The re-lay to the
    // reader's own is the script's, and it MOVES the boxes rather than
    // restyling them — so what the eyes read, what Tab walks and what the form
    // posts are always one order.
    //
    // AND THE VALUE IS RENDERED INTO THEM. It used to reach the boxes from
    // `data-nd-value` at enhance time and nowhere else, so a field given
    // `14:05` was three empty boxes until a script ran — under a prop whose
    // whole promise is the time to start with.
    let given: Vec<&str> = value.as_deref().unwrap_or("").split(':').collect();
    let served = |at: usize| -> Option<String> {
        given
            .get(at)
            .map(|piece| piece.trim())
            .filter(|piece| !piece.is_empty())
            .map(|piece| {
                if pad && piece.len() == 1 {
                    format!("0{piece}")
                } else {
                    (*piece).to_owned()
                }
            })
    };
    let parts: Vec<(&str, String, Option<String>, Option<String>)> = {
        let mut all = vec![
            (
                "hour",
                format!("{id}-hour"),
                Some(hour_label.clone()),
                served(0),
            ),
            (
                "minute",
                format!("{id}-minute"),
                Some(minute_label.clone()),
                served(1),
            ),
        ];
        if seconds {
            all.push((
                "second",
                format!("{id}-second"),
                second_label.clone(),
                served(2),
            ));
        }
        all
    };
    // WHICH HALF OF THE DAY THE SERVED VALUE IS IN, decided here rather than
    // guessed by the script. An empty field chooses NEITHER: a control that
    // ships with the morning already marked has answered for the reader.
    let hour24 = given
        .first()
        .and_then(|piece| piece.trim().parse::<u32>().ok());
    let morning = hour24.map(|hour| hour < 12);
    // A NAME AND BOTH WORDS, OR NO CONTROL AT ALL. The script used to fall
    // back to the raw values, so a field that named neither drew `am` and
    // `pm` — two identifiers, untranslated, on the page. A radiogroup with no
    // name is the other half of the same failure, so the name is required
    // here too and the field simply stays on twenty-four hours without it.
    let has_meridiem = meridiem_label.is_some() && am_label.is_some() && pm_label.is_some();

    rsx! {
        Field {
            control: "{control}",
            label: legend.clone(),
            hint: hint.clone(),
            error: error.clone(),
            required,
            required_label: required_label.clone(),
            kind: FieldKind::Group,
            size: match size {
                TimeInputSize::Small => FieldSize::Small,
                TimeInputSize::Medium => FieldSize::Medium,
                TimeInputSize::Large => FieldSize::Large,
            },
            disabled,

            div {
                class: "{root}",
                "data-nd-time-input": "{name}",
                "data-nd-size": size.as_str(),
                "data-nd-glass": if glass { "true" } else { "false" },
                "data-nd-clock": clock.as_str(),
                "data-nd-seconds": if seconds { "true" } else { "false" },
                "data-nd-pad": if pad { "true" } else { "false" },
                "data-nd-value": value,
                "data-nd-min": min,
                "data-nd-max": max,
                "data-nd-step": step,
                "data-nd-required-label": required_label.clone(),
                "data-nd-missing-label": missing_label,
                "data-nd-invalid-label": invalid_label,
                "data-nd-min-label": min_label,
                "data-nd-max-label": max_label,

                div { class: "nd-time-input-row", "data-nd-time-input-row": "",
                    for (part, part_id, part_label, part_value) in parts {
                        // EVERY BOX IS NAMED, and the name survives the
                        // twelve-hour re-lay because the boxes move rather
                        // than being restyled.
                        span { class: "nd-time-input-part", key: "{part}",
                            label {
                                class: "nd-time-input-part-label",
                                r#for: "{part_id}",
                                {part_label.clone().unwrap_or_default()}
                            }
                            input {
                                class: "nd-time-input-segment",
                                id: "{part_id}",
                                r#type: "text",
                                name: "{name}-{part}",
                                inputmode: "numeric",
                                autocomplete: "off",
                                value: part_value,
                                required,
                                disabled,
                                "aria-invalid": invalid,
                                "data-nd-time-input-segment": "{part}",
                            }
                        }
                    }
                    // THE MORNING AND AFTERNOON CONTROL, AND IT IS A REAL
                    // `Segmented` NOW.
                    //
                    // It was a `<div class="nd-segmented">` of two
                    // `<button class="nd-segmented-item">` that `inputs.js`
                    // built with `createElement` — a copy of this component's
                    // markup, in a script, already drifted from it: no
                    // `appearance`, no `shape`, no `fill`, no
                    // `aria-orientation`, so every compound rule that draws a
                    // joined control matched nothing and the pair rendered as
                    // two detached boxes. A control in this system is a
                    // component, never a class on a `<button>`.
                    //
                    // The WRAPPER carries `hidden`, because a component cannot
                    // be handed the attribute, and it ships hidden for the
                    // reason every script-owned control does: the twelve-hour
                    // reading is the script's, so with nothing running there
                    // is no half of the day to name.
                    span {
                        class: "nd-time-input-meridiem",
                        "data-nd-time-input-meridiem": "",
                        hidden: true,
                        {meridiem_label.clone().map(|word| rsx! {
                            span { class: "nd-visually-hidden", id: "{id}-meridiem-label", "{word}" }
                        })}
                        if has_meridiem {
                            Segmented {
                                name: "{name}-meridiem",
                                labelledby: "{id}-meridiem-label",
                                // THE FIELD'S OWN SIZE. Siblings in one row
                                // share their padding and their height, and a
                                // control fixed at `small` beside a `large`
                                // field reads as broken.
                                size: match size {
                                    TimeInputSize::Small => SegmentedSize::Small,
                                    TimeInputSize::Medium => SegmentedSize::Medium,
                                    TimeInputSize::Large => SegmentedSize::Large,
                                },
                                class: "nd-time-input-meridiem-group",
                                SegmentedItem {
                                    value: "am",
                                    label: am_label.clone().unwrap_or_default(),
                                    // NEITHER, when nothing is known. See
                                    // `morning` above.
                                    selected: morning == Some(true),
                                    disabled,
                                }
                                SegmentedItem {
                                    value: "pm",
                                    label: pm_label.clone().unwrap_or_default(),
                                    selected: morning == Some(false),
                                    disabled,
                                }
                            }
                        }
                    }
                }
                p {
                    class: "nd-time-input-note",
                    id: "{id}-status",
                    role: "status",
                    "aria-live": "polite",
                    "aria-atomic": "true",
                    "data-nd-time-input-status": "",
                }
            }
        }
    }
"#;

const TIME_INPUT_REACT: &str = r#"  const size_ = size ?? "medium";
  const clock_ = clock ?? "24";
  const pad_ = pad ?? true;
  const id = `nd-time-input-${name}`;
  // See the Dioxus half: the same `control` the field gets, or the sentences
  // are described by elements that do not exist.
  const control = `${id}-hour`;
  // See the Dioxus half: the fieldset describes itself now.
  const root = ["nd-time-input", className].filter(Boolean).join(" ");
  // See the Dioxus half: served as twenty-four hours, re-laid by the script,
  // and the value RENDERED into the boxes rather than read out of an
  // attribute once a script runs.
  const given = String(value ?? "").split(":");
  const served = (at) => {
    const piece = (given[at] ?? "").trim();
    if (piece === "") return undefined;
    return pad_ && piece.length === 1 ? `0${piece}` : piece;
  };
  const parts = [
    ["hour", hourLabel, served(0)],
    ["minute", minuteLabel, served(1)],
    ...(seconds ? [["second", secondLabel, served(2)]] : []),
  ];
  // See the Dioxus half: which half of the day the served value is in, and
  // NEITHER when nothing is known.
  const hour24 = /^\d+$/.test((given[0] ?? "").trim()) ? Number(given[0]) : null;
  const morning = hour24 === null ? null : hour24 < 12;
  // See the Dioxus half: a name and both words, or no control at all.
  const hasMeridiem = Boolean(meridiemLabel) && Boolean(amLabel) && Boolean(pmLabel);

  return h(
    Field,
    {
      control,
      label: legend,
      hint,
      error,
      required,
      requiredLabel,
      kind: "group",
      size: size_,
      disabled,
    },
    h(
      "div",
      {
        key: "b",
        className: root,
        "data-nd-time-input": name,
        "data-nd-size": size_,
        "data-nd-glass": String(glass ?? false),
        "data-nd-clock": clock_,
        "data-nd-seconds": String(seconds ?? false),
        "data-nd-pad": String(pad_),
        "data-nd-value": value,
        "data-nd-min": min,
        "data-nd-max": max,
        "data-nd-step": step,
        "data-nd-required-label": requiredLabel,
        "data-nd-missing-label": missingLabel,
        "data-nd-invalid-label": invalidLabel,
        "data-nd-min-label": minLabel,
        "data-nd-max-label": maxLabel,
        ...rest,
      },
      h(
        "div",
        { key: "r", className: "nd-time-input-row", "data-nd-time-input-row": "" },
        parts.map(([part, partLabel, partValue]) =>
          h(
            "span",
            { key: part, className: "nd-time-input-part" },
            h(
              "label",
              { key: "l", className: "nd-time-input-part-label", htmlFor: `${id}-${part}` },
              partLabel ?? "",
            ),
            h("input", {
              key: "i",
              className: "nd-time-input-segment",
              id: `${id}-${part}`,
              type: "text",
              name: `${name}-${part}`,
              inputMode: "numeric",
              autoComplete: "off",
              defaultValue: partValue,
              required,
              disabled,
              "aria-invalid": error ? "true" : undefined,
              "data-nd-time-input-segment": part,
            }),
          ),
        ),
        // See the Dioxus half: a real `Segmented`, hidden until the script
        // reads the clock, and never a copy of that component's markup built
        // with `createElement`.
        h(
          "span",
          {
            key: "m",
            className: "nd-time-input-meridiem",
            "data-nd-time-input-meridiem": "",
            hidden: true,
          },
          meridiemLabel
            ? h("span", { key: "l", className: "nd-visually-hidden", id: `${id}-meridiem-label` }, meridiemLabel)
            : null,
          hasMeridiem
            ? h(
                Segmented,
                {
                  key: "g",
                  name: `${name}-meridiem`,
                  labelledby: `${id}-meridiem-label`,
                  // See the Dioxus half: the field's own size.
                  size: size_,
                  className: "nd-time-input-meridiem-group",
                },
                h(SegmentedItem, {
                  key: "a",
                  value: "am",
                  label: amLabel ?? "",
                  selected: morning === true,
                  disabled,
                }),
                h(SegmentedItem, {
                  key: "p",
                  value: "pm",
                  label: pmLabel ?? "",
                  selected: morning === false,
                  disabled,
                }),
              )
            : null,
        ),
      ),
      h("p", {
        key: "n",
        className: "nd-time-input-note",
        id: `${id}-status`,
        role: "status",
        "aria-live": "polite",
        "aria-atomic": "true",
        "data-nd-time-input-status": "",
      }),
    ),
  );
"#;

// --- Carousel ----------------------------------------------------------------

/// A row a reader walks sideways, and one thing standing in it.
///
/// # It is a scroll container before it is anything else
///
/// The slides are a real `<ul>` in a real `overflow` box with `scroll-snap`, so
/// dragging, flicking, momentum, the trackpad, Shift-wheel, the scrollbar and
/// Home/End all work with no script at all. Every hand-built slideshow this
/// avoids is a control that gets one of those six subtly wrong.
///
/// # Everything the script owns ships hidden
///
/// The two controls, the dots, the counter and the live region are rendered and
/// carry `hidden`. A button that scrolls a track nothing has measured is a
/// button that does nothing, and `Search`'s dialog trigger records the reason
/// for the arrangement.
///
/// # The pause control is rendered only when it can be named
///
/// WCAG 2.2.2 wants a way to stop anything that moves for more than five
/// seconds. `pause_label` is that way, and the runtime refuses to rotate
/// without it — so the enforcement is structural rather than a note asking for
/// care.
const CAROUSEL_DIOXUS: &str = r#"    let root = format!("nd-carousel {class}");
    let root = root.trim().to_owned();
    let id = format!("nd-carousel-{name}");
    let track_id = format!("{id}-track");
    let rotates = autoplay.is_some() && pause_label.is_some();
    let previous = previous_label.clone();
    let next = next_label.clone();
    let pause = pause_label.clone();
    // A WIDTH IS A PROP, SO THE COMPONENT WRITES IT. `carousel.js` used to set
    // this custom property, which made `per_view` a thing that needed a script:
    // with JavaScript blocked a `per_view: "3"` carousel showed one slide,
    // under a contract that opens by promising the slides "scroll and snap
    // before any script runs". The stylesheet divides the track by it and
    // declares `1` as the fallback, so this is the only value that has to
    // arrive. `auto` is matched on `data-nd-per-view` instead and wants no
    // property at all.
    let per_view_width =
        (per_view != "auto").then(|| format!("--nd-carousel-per-view:{per_view}"));

    rsx! {
        section {
            class: "{root}",
            id: "{id}",
            "data-nd-carousel": "{name}",
            // IT SAYS WHAT KIND OF THING IT IS, AND WHAT IS IN IT. `carousel`
            // alone gives a reader the shape and none of the contents, which
            // is the half they need to decide whether to walk it.
            role: "group",
            "aria-roledescription": "carousel",
            "aria-label": "{label}",
            // DRAWN, NOT ANNOUNCED. ARIA 1.2 gives `role="group"` no axis, so
            // `aria-orientation` here was an attribute an engine may drop.
            "data-nd-orientation": orientation.as_str(),
            "data-nd-per-view": "{per_view}",
            "data-nd-snap": snap.as_str(),
            "data-nd-glass": if glass { "true" } else { "false" },
            "data-nd-controls": if controls { "true" } else { "false" },
            "data-nd-dots": if dots { "true" } else { "false" },
            "data-nd-counter": if counter { "true" } else { "false" },
            "data-nd-wrap": if wrap { "true" } else { "false" },
            "data-nd-autoplay": autoplay,
            "data-nd-previous-label": previous_label.clone(),
            "data-nd-next-label": next_label.clone(),
            "data-nd-dot-label": dot_label,
            "data-nd-counter-label": counter_label,
            "data-nd-announce-label": announce_label,
            "data-nd-pause-label": pause_label.clone(),
            "data-nd-play-label": play_label,

            // THE TRACK. A real list in a real scroll box: a screen reader
            // counts the items, and a reader with no script scrolls it with
            // everything their device already gives them.
            ul {
                class: "nd-carousel-track",
                id: "{track_id}",
                "data-nd-carousel-track": "",
                // See above: the share of the track one slide takes, rendered.
                style: per_view_width,
                // Polite while a reader is driving; the runtime turns it off
                // while autoplay is, because a region that speaks every four
                // seconds unasked makes the rest of the page unusable.
                "aria-live": if rotates { "off" } else { "polite" },
                {children}
            }

            div { class: "nd-carousel-controls",
                // FIRST, AND ONLY WHEN IT CAN BE NAMED. WCAG 2.2.2 made
                // structural: with no name there is no control, and the
                // runtime refuses to rotate without one.
                if rotates {
                    button {
                        class: "nd-carousel-control nd-carousel-rotation",
                        r#type: "button",
                        hidden: true,
                        "aria-pressed": "false",
                        "aria-label": pause.clone(),
                        title: pause,
                        "data-nd-carousel-rotation": "",
                        Icon { icon: IconDrawing::PAUSE, size: crate::components::IconSize::Small }
                        Icon { icon: IconDrawing::PLAY, size: crate::components::IconSize::Small }
                    }
                }
                if controls {
                    button {
                        class: "nd-carousel-control",
                        r#type: "button",
                        hidden: true,
                        "aria-controls": "{track_id}",
                        "aria-label": previous.clone(),
                        title: previous,
                        "data-nd-carousel-step": "previous",
                        Icon { icon: IconDrawing::CHEVRON_LEFT, size: crate::components::IconSize::Small }
                    }
                    button {
                        class: "nd-carousel-control",
                        r#type: "button",
                        hidden: true,
                        "aria-controls": "{track_id}",
                        "aria-label": next.clone(),
                        title: next,
                        "data-nd-carousel-step": "next",
                        Icon { icon: IconDrawing::CHEVRON_RIGHT, size: crate::components::IconSize::Small }
                    }
                }
                if dots {
                    // Filled by the runtime, one per slide it finds — a count
                    // written by a caller goes wrong the first time a slide is
                    // added.
                    div {
                        class: "nd-carousel-dots",
                        role: "group",
                        "aria-label": "{label}",
                        hidden: true,
                        "data-nd-carousel-dots": "",
                    }
                }
                if counter {
                    p { class: "nd-carousel-counter", hidden: true, "data-nd-carousel-counter": "" }
                }
            }

            // One sentence at a time, and never the slide's contents: what a
            // reader needs on arrival is which slide arrived.
            p {
                class: "nd-visually-hidden",
                id: "{id}-status",
                role: "status",
                "aria-live": "polite",
                "aria-atomic": "true",
                "data-nd-carousel-status": "",
            }

            // The runtime does not know what a dot looks like. It copies this.
            template { "data-nd-dot-template": "",
                button {
                    class: "nd-carousel-dot",
                    r#type: "button",
                    "data-nd-carousel-dot": "",
                }
            }
        }
    }
"#;

const CAROUSEL_REACT: &str = r#"  const perView_ = perView ?? "1";
  const snap_ = snap ?? "start";
  const orientation_ = orientation ?? "horizontal";
  const root = ["nd-carousel", className].filter(Boolean).join(" ");
  const id = `nd-carousel-${name}`;
  const trackId = `${id}-track`;
  // See the Dioxus half: no name, no control, and no rotation either.
  const rotates = Boolean(autoplay && pauseLabel);
  // See the Dioxus half: a width is a prop, so the component writes it rather
  // than the runtime, and `auto` wants no property at all.
  const perViewWidth =
    perView_ === "auto" ? undefined : { "--nd-carousel-per-view": perView_ };

  return h(
    "section",
    {
      className: root,
      id,
      "data-nd-carousel": name,
      role: "group",
      "aria-roledescription": "carousel",
      "aria-label": label,
      // See the Dioxus half: a group has no axis in ARIA, so the axis is drawn.
      "data-nd-orientation": orientation_,
      "data-nd-per-view": perView_,
      "data-nd-snap": snap_,
      "data-nd-glass": String(glass ?? false),
      "data-nd-controls": String(controls ?? true),
      "data-nd-dots": String(dots ?? false),
      "data-nd-counter": String(counter ?? false),
      "data-nd-wrap": String(wrap ?? false),
      "data-nd-autoplay": autoplay,
      "data-nd-previous-label": previousLabel,
      "data-nd-next-label": nextLabel,
      "data-nd-dot-label": dotLabel,
      "data-nd-counter-label": counterLabel,
      "data-nd-announce-label": announceLabel,
      "data-nd-pause-label": pauseLabel,
      "data-nd-play-label": playLabel,
      ...rest,
    },
    h(
      "ul",
      {
        key: "t",
        className: "nd-carousel-track",
        id: trackId,
        "data-nd-carousel-track": "",
        style: perViewWidth,
        "aria-live": rotates ? "off" : "polite",
      },
      children,
    ),
    h(
      "div",
      { key: "c", className: "nd-carousel-controls" },
      rotates
        ? h(
            "button",
            {
              key: "r",
              className: "nd-carousel-control nd-carousel-rotation",
              type: "button",
              hidden: true,
              "aria-pressed": "false",
              "aria-label": pauseLabel,
              title: pauseLabel,
              "data-nd-carousel-rotation": "",
            },
            h(Icon, { key: "a", icon: pause, size: "small" }),
            h(Icon, { key: "b", icon: play, size: "small" }),
          )
        : null,
      (controls ?? true)
        ? [
            h(
              "button",
              {
                key: "p",
                className: "nd-carousel-control",
                type: "button",
                hidden: true,
                "aria-controls": trackId,
                "aria-label": previousLabel,
                title: previousLabel,
                "data-nd-carousel-step": "previous",
              },
              h(Icon, { key: "i", icon: chevronLeft, size: "small" }),
            ),
            h(
              "button",
              {
                key: "n",
                className: "nd-carousel-control",
                type: "button",
                hidden: true,
                "aria-controls": trackId,
                "aria-label": nextLabel,
                title: nextLabel,
                "data-nd-carousel-step": "next",
              },
              h(Icon, { key: "i", icon: chevronRight, size: "small" }),
            ),
          ]
        : null,
      dots
        ? h("div", {
            key: "d",
            className: "nd-carousel-dots",
            role: "group",
            "aria-label": label,
            hidden: true,
            "data-nd-carousel-dots": "",
          })
        : null,
      counter
        ? h("p", {
            key: "o",
            className: "nd-carousel-counter",
            hidden: true,
            "data-nd-carousel-counter": "",
          })
        : null,
    ),
    h("p", {
      key: "s",
      className: "nd-visually-hidden",
      id: `${id}-status`,
      role: "status",
      "aria-live": "polite",
      "aria-atomic": "true",
      "data-nd-carousel-status": "",
    }),
    h(
      "template",
      { key: "m", "data-nd-dot-template": "" },
      h("button", {
        key: "b",
        className: "nd-carousel-dot",
        type: "button",
        "data-nd-carousel-dot": "",
      }),
    ),
  );
"#;

const CAROUSEL_SLIDE_DIOXUS: &str = r#"    let root = format!("nd-carousel-slide {class}");
    let root = root.trim().to_owned();

    rsx! {
        li {
            class: "{root}",
            // MOVING BETWEEN THESE IS ANNOUNCED AS MOVING BETWEEN SLIDES. The
            // role is on the `<li>` itself rather than on a wrapper: a list
            // item that is a group is one element to a screen reader, and two
            // would be one too many.
            role: "group",
            "aria-roledescription": "slide",
            // The NAME is the runtime's — `3 of 8`, from the slides that are
            // actually there. This is the caller's addition to it, kept as an
            // attribute so the runtime can join the two without a second copy
            // of either.
            "data-nd-slide-label": label,
            "data-nd-carousel-slide": "",
            {children}
        }
    }
"#;

const CAROUSEL_SLIDE_REACT: &str = r#"  const root = ["nd-carousel-slide", className].filter(Boolean).join(" ");

  return h(
    "li",
    {
      className: root,
      role: "group",
      "aria-roledescription": "slide",
      // See the Dioxus half: the position is the runtime's, this is the
      // caller's addition to it.
      "data-nd-slide-label": label,
      "data-nd-carousel-slide": "",
      ...rest,
    },
    children,
  );
"#;

// --- ErrorSummary -------------------------------------------------------------

/// Everything wrong with a form, at the top of it.
///
/// # The focus goes to the box and the alert is inside it
///
/// `tabindex="-1"` so focus can be *sent* here without this becoming a tab
/// stop, and `role="alert"` on the content within. Belt and braces on purpose:
/// focus alone is silent where a script could not move it, and an alert alone
/// leaves a reader at the bottom of a form being told about the top of it.
///
/// # The element exists before it speaks
///
/// It is in the document from the start and its **content** is what arrives,
/// which is what an alert region needs to be announced at all. A region
/// inserted whole is a region most screen readers never see — so the empty
/// state is an attribute the stylesheet acts on rather than markup that was
/// never there.
const ERROR_SUMMARY_DIOXUS: &str = r##"    let root = format!("nd-error-summary {class}");
    let root = root.trim().to_owned();
    let id = format!("nd-error-summary-{name}");
    let title_id = format!("{id}-title");

    rsx! {
        div {
            class: "{root}",
            id: "{id}",
            "data-nd-error-summary": "{name}",
            // FOCUSABLE, NOT A TAB STOP. The two are different things, and
            // this is the attribute that says which.
            tabindex: "-1",
            "aria-labelledby": "{title_id}",
            "data-nd-focus": if focus { "true" } else { "false" },
            "data-nd-collect": if collect { "true" } else { "false" },
            "data-nd-form": form,
            "data-nd-title-prefix": title_prefix,
            // Rendered empty only when the script is going to fill it. A
            // server that refused a submission already knows what is wrong.
            "data-nd-empty": collect.then_some(""),

            div { role: "alert", class: "nd-error-summary-alert",
                h2 { class: "nd-error-summary-title", id: "{title_id}", "{title}" }
                if let Some(description) = description {
                    p { class: "nd-error-summary-description", "{description}" }
                }
                ul { class: "nd-error-summary-list", "data-nd-error-summary-list": "",
                    {children}
                }
            }

            // The script does not know what an entry looks like. It copies
            // this — `NOCTUA.md §9.5`, applied to a link rather than a drawing.
            template { "data-nd-error-summary-template": "",
                li { class: "nd-error-summary-item",
                    a { class: "nd-error-summary-link", href: "#", "data-nd-error-summary-link": "" }
                }
            }
        }
    }
"##;

const ERROR_SUMMARY_REACT: &str = r##"  const root = ["nd-error-summary", className].filter(Boolean).join(" ");
  const id = `nd-error-summary-${name}`;
  const titleId = `${id}-title`;

  return h(
    "div",
    {
      className: root,
      id,
      "data-nd-error-summary": name,
      // See the Dioxus half: focusable, not a tab stop.
      tabIndex: -1,
      "aria-labelledby": titleId,
      "data-nd-focus": String(focus ?? true),
      "data-nd-collect": String(collect ?? false),
      "data-nd-form": form,
      "data-nd-title-prefix": titlePrefix,
      "data-nd-empty": collect ? "" : undefined,
      ...rest,
    },
    h(
      "div",
      { key: "a", role: "alert", className: "nd-error-summary-alert" },
      h("h2", { key: "t", className: "nd-error-summary-title", id: titleId }, title),
      description
        ? h("p", { key: "d", className: "nd-error-summary-description" }, description)
        : null,
      h(
        "ul",
        { key: "l", className: "nd-error-summary-list", "data-nd-error-summary-list": "" },
        children,
      ),
    ),
    h(
      "template",
      { key: "m", "data-nd-error-summary-template": "" },
      h(
        "li",
        { key: "i", className: "nd-error-summary-item" },
        h("a", {
          key: "a",
          className: "nd-error-summary-link",
          href: "#",
          "data-nd-error-summary-link": "",
        }),
      ),
    ),
  );
"##;

const ERROR_SUMMARY_ITEM_DIOXUS: &str = r##"    let root = format!("nd-error-summary-item {class}");
    let root = root.trim().to_owned();

    rsx! {
        li { class: "{root}",
            // A REAL LINK TO A REAL ID. It works with the script blocked, it
            // opens in a new tab when a reader asks, and it says where it
            // goes — none of which a button with a handler does.
            a {
                class: "nd-error-summary-link",
                href: "#{control}",
                "data-nd-error-summary-link": "",
                {children}
            }
        }
    }
"##;

const ERROR_SUMMARY_ITEM_REACT: &str = r#"  const root = ["nd-error-summary-item", className].filter(Boolean).join(" ");

  return h(
    "li",
    { className: root, ...rest },
    // See the Dioxus half: a real link, never a button.
    h(
      "a",
      { key: "a", className: "nd-error-summary-link", href: `#${control}`, "data-nd-error-summary-link": "" },
      children,
    ),
  );
"#;

// --- DateInput ---------------------------------------------------------------

/// One date, as the three numbers a reader already knows.
///
/// # It claims nothing the script must keep
///
/// With no script this is a `<fieldset>` with a legend, three labelled text
/// boxes that post `{name}-day/-month/-year`, and the browser's own
/// `required` refusal. The reorder to the reader's locale, the stepping, the
/// paste, the validation sentences and the whole suggestions panel are the
/// script's — and every ARIA attribute they need is written by the script,
/// never here.
///
/// # The error slot always renders
///
/// Empty until somebody has a sentence — the caller's at render, the
/// validator's later — and hidden by the stylesheet while empty, so the id
/// `aria-describedby` names always exists. `Select` invented the
/// arrangement for the same reason.
const DATE_INPUT_DIOXUS: &str = r#"    let root = format!("nd-date-input {class}");
    let root = root.trim().to_owned();
    // Derived, so the labels' targets cannot be typed wrong — there is nothing
    // to type. The hint and the error ids are `Field`'s now, and so is the
    // `aria-describedby` that points at them: a `<fieldset>` describes what is
    // inside it, and the field IS the fieldset.
    let id = format!("nd-date-input-{name}");
    let flagged = if error.is_some() { "true" } else { "false" };
    // The one wire format, split — not locale arithmetic. How a reader sees
    // the three numbers is the row's order, which is decided below.
    let mut parts = value.as_deref().unwrap_or("").splitn(3, '-');
    let value_year = parts.next().unwrap_or("").to_owned();
    let value_month = parts.next().unwrap_or("").to_owned();
    let value_day = parts.next().unwrap_or("").to_owned();
    let value_year = (!value_year.is_empty()).then_some(value_year);
    let value_month = (!value_month.is_empty()).then_some(value_month);
    let value_day = (!value_day.is_empty()).then_some(value_day);
    // Served in the declared order; `auto` serves day-month-year and the
    // script re-lays the row once it knows the reader.
    let resolved = match order {
        DateInputOrder::Auto => "dmy",
        other => other.as_str(),
    };
    // The three items as VALUES, laid out by the order in force — a declared
    // `ymd` is SERVED year-first, not restyled into it, so a scriptless page
    // and the stamp agree.
    let day_item = rsx! {
        div { class: "nd-date-input-item", "data-nd-part": "day",
            label { class: "nd-date-input-label", r#for: "{id}-day", "{day_label}" }
            input {
                class: "nd-date-input-segment nd-date-input-two",
                id: "{id}-day",
                r#type: "text",
                name: "{name}-day",
                inputmode: "numeric",
                autocomplete: bday.then_some("bday-day"),
                value: value_day,
                required,
            }
        }
    };
    let month_item = rsx! {
        div { class: "nd-date-input-item", "data-nd-part": "month",
            label { class: "nd-date-input-label", r#for: "{id}-month", "{month_label}" }
            input {
                class: "nd-date-input-segment nd-date-input-two",
                id: "{id}-month",
                r#type: "text",
                name: "{name}-month",
                inputmode: "numeric",
                autocomplete: bday.then_some("bday-month"),
                value: value_month,
                required,
            }
        }
    };
    let year_item = rsx! {
        div { class: "nd-date-input-item", "data-nd-part": "year",
            label { class: "nd-date-input-label", r#for: "{id}-year", "{year_label}" }
            input {
                class: "nd-date-input-segment nd-date-input-four",
                id: "{id}-year",
                r#type: "text",
                name: "{name}-year",
                inputmode: "numeric",
                autocomplete: bday.then_some("bday-year"),
                value: value_year,
                required,
            }
        }
    };
    let (first, second, third) = match resolved {
        "mdy" => (month_item, day_item, year_item),
        "ymd" => (year_item, month_item, day_item),
        _ => (day_item, month_item, year_item),
    };
    let has_suggest = suggest_label.is_some();

    rsx! {
        // THE GROUP IS THE FIELD NOW. `Field { kind: Group }` renders the
        // `<fieldset>`, writes the `<legend>`, points `aria-describedby` at
        // itself and carries the `disabled` that refuses everything under it —
        // so what is left here is the row of boxes and the hooks the script
        // reads, on a `<div>` of its own.
        Field {
            control: "{id}-day",
            label: legend.clone(),
            hint: hint.clone(),
            error: error.clone(),
            // The slot this component arrived at second, and `Select` first.
            error_slot: true,
            error_label: error_label,
            kind: FieldKind::Group,
            required,
            size: match size {
                DateInputSize::Small => FieldSize::Small,
                DateInputSize::Medium => FieldSize::Medium,
                DateInputSize::Large => FieldSize::Large,
            },
            disabled,

        div {
            class: "{root}",
            "data-nd-date-input": "{name}",
            "data-nd-size": size.as_str(),
            "data-nd-order": order.as_str(),
            "data-nd-resolved": resolved,
            "data-nd-invalid": flagged,
            "data-nd-bday": if bday { "true" } else { "false" },
            "data-nd-pad": if pad { "true" } else { "false" },
            "data-nd-expand": if expand { "true" } else { "false" },
            "data-nd-calendar": if calendar { "true" } else { "false" },
            "data-nd-glass": if glass { "true" } else { "false" },
            "data-nd-min": min,
            "data-nd-max": max,
            "data-nd-suggest-label": suggest_label,
            "data-nd-previous-label": previous_label,
            "data-nd-next-label": next_label,
            "data-nd-required-label": required_label,
            "data-nd-missing-label": missing_label,
            "data-nd-number-label": number_label,
            "data-nd-year-digits-label": year_digits_label,
            "data-nd-invalid-label": invalid_label,
            "data-nd-min-label": min_label,
            "data-nd-max-label": max_label,

            div { class: "nd-date-input-row",
                {first}
                {second}
                {third}
            }

            // THE QUICK PICKS, INERT. Served hidden, so with no script they
            // claim nothing; the script builds the listbox by reading them.
            div {
                class: "nd-date-input-suggest",
                hidden: true,
                "data-nd-date-input-suggest": "",
                {children}
            }
            // THE SCRIPT DOES NOT KNOW WHAT A CALENDAR LOOKS LIKE. It copies
            // this, so the drawing stays declared rather than drawn in a
            // script.
            if has_suggest {
                template { "data-nd-date-input-glyph": "",
                    Icon { icon: IconDrawing::CALENDAR, size: crate::components::IconSize::Small }
                }
            }
        }
        }
    }
"#;

/// The same field, in React.
const DATE_INPUT_REACT: &str = r#"  const size_ = size ?? "medium";
  const order_ = order ?? "auto";
  const root = ["nd-date-input", className].filter(Boolean).join(" ");
  const id = `nd-date-input-${name}`;
  const hintId = `${id}-hint`;
  const errorId = `${id}-error`;
  // The error id is always in it — the slot always renders; see the Dioxus
  // half.
  const described = [hint ? hintId : null, errorId].filter(Boolean).join(" ");
  // The one wire format, split — not locale arithmetic.
  const [valueYear = "", valueMonth = "", valueDay = ""] = (value ?? "").split("-");
  const resolved = order_ === "auto" ? "dmy" : order_;
  const segment = (part, width, partLabel, partValue, token) =>
    h(
      "div",
      { key: part, className: "nd-date-input-item", "data-nd-part": part },
      h(
        "label",
        { key: "l", className: "nd-date-input-label", htmlFor: `${id}-${part}` },
        partLabel,
      ),
      h("input", {
        key: "i",
        className: `nd-date-input-segment ${width}`,
        id: `${id}-${part}`,
        type: "text",
        name: `${name}-${part}`,
        inputMode: "numeric",
        autoComplete: bday ? token : undefined,
        defaultValue: partValue || undefined,
        required,
        // The one spread target, chosen by meaning rather than position —
        // "the first box" changes when the locale re-lays the row.
        ...(part === "day" ? rest : null),
      }),
    );

  return h(
    Field,
    {
      control: `${id}-day`,
      label: legend,
      hint,
      error,
      errorSlot: true,
      errorLabel,
      kind: "group",
      required,
      size: size_,
      disabled,
    },
  h(
    "div",
    {
      key: "d",
      className: root,
      "data-nd-date-input": name,
      "data-nd-size": size_,
      "data-nd-order": order_,
      "data-nd-resolved": resolved,
      "data-nd-invalid": String(Boolean(error)),
      "data-nd-bday": String(bday ?? false),
      "data-nd-pad": String(pad ?? false),
      "data-nd-expand": String(expand ?? false),
      "data-nd-calendar": String(calendar ?? false),
      "data-nd-glass": String(glass ?? false),
      "data-nd-min": min,
      "data-nd-max": max,
      "data-nd-suggest-label": suggestLabel,
      "data-nd-previous-label": previousLabel,
      "data-nd-next-label": nextLabel,
      "data-nd-required-label": requiredLabel,
      "data-nd-missing-label": missingLabel,
      "data-nd-number-label": numberLabel,
      "data-nd-year-digits-label": yearDigitsLabel,
      "data-nd-invalid-label": invalidLabel,
      "data-nd-min-label": minLabel,
      "data-nd-max-label": maxLabel,
    },
    h(
      "div",
      { key: "r", className: "nd-date-input-row" },
      // Laid out by the order in force — a declared `ymd` is SERVED
      // year-first; see the Dioxus half.
      ...(resolved === "mdy"
        ? ["month", "day", "year"]
        : resolved === "ymd"
          ? ["year", "month", "day"]
          : ["day", "month", "year"]
      ).map((part) =>
        part === "day"
          ? segment("day", "nd-date-input-two", dayLabel, valueDay, "bday-day")
          : part === "month"
            ? segment("month", "nd-date-input-two", monthLabel, valueMonth, "bday-month")
            : segment("year", "nd-date-input-four", yearLabel, valueYear, "bday-year"),
      ),
    ),
    h(
      "div",
      {
        key: "s",
        className: "nd-date-input-suggest",
        hidden: true,
        "data-nd-date-input-suggest": "",
      },
      children,
    ),
    suggestLabel
      ? h(
          "template",
          { key: "t", "data-nd-date-input-glyph": "" },
          h(Icon, { icon: calendar, size: "small" }),
        )
      : null,
  ),
  );
"#;

// --- Tree and TreeItem ------------------------------------------------------

/// A hierarchy a reader walks.
///
/// # It claims no roles
///
/// `role="tree"`, `role="treeitem"`, `role="group"`, `aria-expanded`,
/// `aria-selected` and every `tabindex` are written by `shell.js`. With no
/// script this is a nested `<ul>` of links, complete — and nothing on the page
/// claims a keyboard contract nothing is answering, which is the rule
/// `Segmented` states and `Table` follows in refusing `role="grid"`.
///
/// # And it hides nothing that cannot be brought back
///
/// A branch that ships closed is closed by a stylesheet rule keyed on
/// `data-nd-scripting`, which the inline bootstrap stamps before the first
/// paint — so there is no layout shift, and with no script the attribute never
/// appears and nothing is folded away. That is `CodeBlock { max_lines }`'s rule
/// applied to a tree.
const TREE_DIOXUS: &str = r#"    let root = format!("nd-tree {class}");
    let root = root.trim().to_owned();

    rsx! {
        ul {
            class: "{root}",
            "data-nd-tree": "{name}",
            "aria-label": "{label}",
            "data-nd-selection": selection.as_str(),
            "data-nd-size": size.as_str(),
            "data-nd-guides": if guides { "true" } else { "false" },
            "data-nd-form": form,
            "data-nd-expand-label": expand_label,
            "data-nd-collapse-label": collapse_label,
            "data-nd-empty-label": empty_label,
            {children}
        }
    }
"#;

/// The same tree, in React.
const TREE_REACT: &str = r#"  const selection_ = selection ?? "none";
  const size_ = size ?? "medium";
  const root = ["nd-tree", className].filter(Boolean).join(" ");

  return h(
    "ul",
    {
      className: root,
      "data-nd-tree": name,
      "aria-label": label,
      "data-nd-selection": selection_,
      "data-nd-size": size_,
      "data-nd-guides": String(guides ?? false),
      "data-nd-form": form,
      "data-nd-expand-label": expandLabel,
      "data-nd-collapse-label": collapseLabel,
      "data-nd-empty-label": emptyLabel,
      ...rest,
    },
    children,
  );
"#;

/// One node.
///
/// # The role goes on the row and the link is untouched
///
/// `role="treeitem"` on an `<a>` replaces *link* with *tree item* in what a
/// reader hears and takes the link's own behaviours with it — middle-click, the
/// context menu, *copy link address*, the status-bar preview. So the `<li>`
/// carries the role (written by the script) and the `<a>` inside it stays a
/// link.
///
/// # And the children are the nodes under it
///
/// Wrapped in a `<ul>` by this component rather than by the caller, because
/// that `<ul>` is what becomes `role="group"` and what a collapse hides. A
/// caller who had to remember it is a caller who will not.
const TREE_ITEM_DIOXUS: &str = r#"    let root = format!("nd-tree-item {class}");
    let root = root.trim().to_owned();

    // NEITHER LANGUAGE ASKS WHETHER THERE ARE CHILDREN, AND THAT IS THE POINT.
    //
    // Dioxus receives `children` as an `Element` — a value that always exists,
    // empty or not — and React could count its own, which `NOCTUA.md §9.3`
    // forbids one implementation doing alone. So both always render the group
    // and always render the toggle, hidden; the script looks at the DOM, which
    // is the only place the answer actually is, and stamps `data-nd-leaf`.
    // With no script an empty group draws nothing and a hidden toggle stays
    // hidden, which is the same page either way.

    // The row: everything on one line, whether or not it leads anywhere.
    let row = rsx! {
        if let Some(icon) = icon {
            span { class: "nd-tree-glyph", "aria-hidden": "true",
                Icon { icon, size: crate::components::IconSize::Small }
                // BOTH DRAWINGS SHIP AND THE STATE PICKS ONE, so nothing has to
                // send a drawing to the script — `Button`'s toggle already does
                // this, and this package ships no runtime to do it any other way.
                if let Some(icon_expanded) = icon_expanded {
                    Icon { icon: icon_expanded, size: crate::components::IconSize::Small }
                }
            }
        }
        span { class: "nd-tree-label", "{label}" }
        if let Some(badge) = badge {
            Tag {
                class: "nd-tree-badge",
                label: badge,
                size: TagSize::Small,
                variant: TagVariant::Outline,
            }
        }
    };

    rsx! {
        li {
            class: "{root}",
            "data-nd-value": value,
            "data-nd-expanded": if expanded { "true" } else { "false" },
            "data-nd-branch": if branch { "true" } else { "false" },
            "data-nd-selected": if selected { "true" } else { "false" },
            "aria-disabled": disabled.then_some("true"),
            div { class: "nd-tree-row",
                // Hidden until the script reveals it, and only on a node that
                // has something under it: with no script nothing is folded
                // away, so a control for unfolding it would do nothing.
                button {
                    class: "nd-tree-toggle",
                    r#type: "button",
                    hidden: true,
                    "data-nd-tree-toggle": "",
                    Icon { icon: IconDrawing::CHEVRON_DOWN, size: crate::components::IconSize::Small }
                }
                if let Some(href) = href {
                    a {
                        class: "nd-tree-body",
                        href: "{href}",
                        "aria-current": current.then_some("page"),
                        {row}
                    }
                } else {
                    span {
                        class: "nd-tree-body",
                        "aria-current": current.then_some("page"),
                        {row}
                    }
                }
            }
            // The `<ul>` that becomes `role="group"` and that a collapse hides.
            // Written here rather than by the caller, because a caller who has
            // to remember it is a caller who will not — and always, because
            // whether it is empty is a question only the DOM can answer.
            ul { class: "nd-tree-group", "data-nd-tree-group": "", {children} }
        }
    }
"#;

/// The scene, in Dioxus. Its twin is [`LINEAGE_REACT`] below.
///
/// Both walk `noctua_design_lineage::draw` and stamp what it says. Every
/// decision — which of a swappable set is showing, whether a figure answers the
/// filter, what a figure is joined to, where a connection's label sits — lives
/// there, in the one place that has a twin held to `spec/lineage/cases.toml`.
/// A difference between these two that is not a difference between `rsx!` and
/// `createElement` is a defect.
const LINEAGE_DIOXUS: &str = r##"    let root = format!("nd-lineage {class}");
    let root = root.trim().to_owned();

    let filter = filter.unwrap_or_default();
    let sets = sets.unwrap_or_default();
    let occasion = occasion.unwrap_or_default();
    let form = form.unwrap_or_default();
    let elided_label = elided_label.unwrap_or_default();
    let drawing = draw(
        &graph,
        Options {
            view: match view {
                LineageView::Flat => View::Flat,
                LineageView::Deep => View::Deep,
            },
            route: match route {
                LineageRoute::Elbow => Route::Elbow,
                LineageRoute::Taut => Route::Taut,
            },
            isolate: filter_mode == LineageFilterMode::Isolate,
            shy: reach == LineageReach::Shy,
        },
        &filter,
        &sets,
    );

    // THE PLACE OF EVERY PIECE, PUBLISHED AS THREE CUSTOM PROPERTIES.
    //
    // This is the only way a parent that cannot see its own children can put
    // one somewhere: `LineageFigure { at: "ada" }` reads `--nd-at-ada-x` and
    // stands there. The occasion and the active sets are published the same
    // way, so hiding resolves to `display: none` through a variable rather than
    // through a class the markup would have to carry — which is what makes both
    // correct before any script runs and changeable afterwards without
    // re-rendering anything.
    //
    // Every name interpolated here has been through `nameable` or through the
    // parser's identifier rule. One carrying `;` would end the declaration and
    // begin another.
    let mut style = format!(
        "--nd-w:{};--nd-h:{};--nd-d:{};",
        drawing.width, drawing.height, drawing.depth
    );
    for piece in &drawing.pieces {
        let _ = write!(
            style,
            "--nd-at-{0}-x:{1};--nd-at-{0}-y:{2};--nd-at-{0}-z:{3};--nd-at-{0}-span:{4};",
            piece.id, piece.x, piece.y, piece.z, piece.span
        );
    }
    if nameable(&occasion) {
        let _ = write!(style, "--nd-on-{occasion}:grid;");
    }
    for entry in sets.split_whitespace() {
        if let Some((group, chosen)) = set_parts(entry) {
            let _ = write!(style, "--nd-set-{group}-{chosen}:grid;");
        }
    }

    let stem = format!("nd-{name}");
    // UNSET, `line` FOLLOWS `thickness`: figures with volume get lines with
    // volume, and a deliberately flat scene keeps flat lines. Resolved here
    // once, so the attribute a consumer reads always carries a real answer.
    let line = line.unwrap_or(if thickness == LineageThickness::None {
        LineageLine::Ribbon
    } else {
        LineageLine::Beam
    });
    let filtering = !filter.is_empty();
    let picks_bonds = matches!(pick, LineagePick::Bond | LineagePick::Both);
    let picks_figures = matches!(pick, LineagePick::Figure | LineagePick::Both);
    let empty = drawing.pieces.is_empty();
    let empty_text = empty_label.clone().unwrap_or_default();

    // THE CARD IS BUILT HERE RATHER THAN INSIDE THE LOOP, because it is one
    // content in three elements: an `<a>` when the figure is a link, a `<label>`
    // when it can be picked, and a `<span>` otherwise. Written inline it would
    // be the same eight lines three times, and three copies of a card are three
    // cards that drift.
    let cards: Vec<(noctua_design_lineage::DrawnPiece, Element)> = drawing
        .pieces
        .iter()
        .cloned()
        .map(|piece| {
            let pickable = picks_figures && piece.href.is_empty() && !piece.slot;
            let inside = rsx! {
                if piece.elided > 0 {
                    // A NUMERAL IS THE SAME WORD IN EVERY LANGUAGE, which is
                    // why the marker carries one and no sentence: how many
                    // people are behind this gap. The words that go with it are
                    // the scene's `elided_label`, which names the marker.
                    span { class: "nd-lineage-count nd-identifier", "{piece.elided}" }
                } else {
                    if !piece.image.is_empty() {
                        img { class: "nd-lineage-portrait", src: "{piece.image}", alt: "" }
                    }
                    // SOLID LETTERING IS STACKED PLIES. Five aria-hidden
                    // copies recede behind the real text and turn with the
                    // scene — the same mechanism as the beam, at glyph scale.
                    span { class: "nd-lineage-name nd-identifier", "{piece.label}"
                        if lettering == LineageLettering::Solid {
                            for ply in 1..=5 {
                                i { "data-nd-ply": "{ply}", "aria-hidden": "true", "{piece.label}" }
                            }
                        }
                    }
                    if !piece.meta.is_empty() {
                        span { class: "nd-lineage-meta nd-identifier", "{piece.meta}"
                            if lettering == LineageLettering::Solid {
                                for ply in 1..=5 {
                                    i { "data-nd-ply": "{ply}", "aria-hidden": "true", "{piece.meta}" }
                                }
                            }
                        }
                    }
                }
            };
            let card = if piece.slot {
                rsx! {}
            } else if !piece.href.is_empty() {
                rsx! {
                    a {
                        class: "nd-lineage-card",
                        href: "{piece.href}",
                        {inside}
                    }
                }
            } else if pickable {
                rsx! {
                    label {
                        class: "nd-lineage-card",
                        r#for: "{stem}-p{piece.id}",
                        {inside}
                    }
                }
            } else {
                rsx! {
                    span { class: "nd-lineage-card", {inside} }
                }
            };
            (piece, card)
        })
        .collect();

    rsx! {
        div {
            class: "{root}",
            "data-nd-lineage": "{name}",
            "data-nd-view": view.as_str(),
            "data-nd-route": route.as_str(),
            "data-nd-size": size.as_str(),
            "data-nd-tilt": tilt.as_str(),
            "data-nd-thickness": thickness.as_str(),
            "data-nd-line": line.as_str(),
            "data-nd-reach": reach.as_str(),
            "data-nd-lettering": lettering.as_str(),
            "data-nd-height": height.as_str(),
            "data-nd-pick": pick.as_str(),
            "data-nd-filter-mode": filter_mode.as_str(),
            "data-nd-parallax": if parallax { "true" } else { "false" },
            "data-nd-filter": some(&filter),
            "data-nd-occasion": some(&occasion),
            "data-nd-sets": some(&sets),
            "data-nd-form": some(&form),
            "data-nd-empty-label": empty_label,
            "data-nd-camera-label": camera_label,
            "data-nd-zoom-in-label": zoom_in_label,
            "data-nd-zoom-out-label": zoom_out_label,
            "data-nd-fit-label": fit_label,
            "data-nd-matches-label": matches_label,
            // Counted rather than thrown, and reported rather than swallowed: a
            // graph with a typo in it draws everything that parsed and says how
            // much did not.
            "data-nd-skipped": (drawing.skipped > 0).then(|| drawing.skipped.to_string()),
            "data-nd-broken": (drawing.broken > 0).then(|| drawing.broken.to_string()),
            "data-nd-crossings": "{drawing.crossings}",
            "data-nd-matched": "{drawing.matched}",
            style: "{style}",
            div {
                class: "nd-lineage-field",
                id: "{stem}",
                role: "group",
                "aria-label": "{label}",
                tabindex: "0",
                "data-nd-lineage-field": "",
                div { class: "nd-lineage-space", "data-nd-lineage-space": "",
                    for bond in drawing.bonds {
                        div {
                            class: "nd-lineage-bond",
                            "data-nd-bond": "{bond.index}",
                            "data-nd-kind": some(&bond.kind),
                            "data-nd-tone": some(&bond.tone),
                            "data-nd-dash": some(&bond.dash),
                            "data-nd-set": some(&bond.set),
                            "data-nd-match": filtering.then_some(if bond.matched { "yes" } else { "no" }),
                            "data-nd-tag": bond.tag_below.then_some("below"),
                            hidden: !bond.shown,
                            style: "--nd-x:{bond.tag[0]};--nd-y:{bond.tag[1]};--nd-z:{bond.tag[2]};",
                            // A PICK IS A REAL CHECKBOX. It works with no script,
                            // it is reachable by keyboard with nothing simulated,
                            // it survives a form post, and `:checked` is what
                            // draws the connection heavier — the transport `Tree`
                            // and `Table` already chose.
                            if picks_bonds {
                                input {
                                    class: "nd-lineage-pick",
                                    r#type: "checkbox",
                                    id: "{stem}-b{bond.index}",
                                    name: some(&form),
                                    value: "{bond.from} {bond.to}",
                                }
                            }
                            // THE LINE IS WHAT A READER PRESSES.
                            //
                            // The runs go INSIDE the label, so the connection
                            // itself is the control — there is no separate
                            // affordance standing in front of it any more, and a
                            // reader who wants that connection points at that
                            // connection. The label has no box of its own
                            // (`display: contents`), so the runs keep the bond as
                            // their containing block and nothing moves.
                            {
                                let runs = rsx! {
                                    for run in bond.runs {
                                        i {
                                            class: "nd-lineage-run",
                                            "data-nd-axis": axis_name(run.axis),
                                            style: "--nd-x:{run.x};--nd-y:{run.y};--nd-z:{run.z};--nd-len:{run.len};--nd-basis:matrix3d({run.basis[0]},{run.basis[1]},{run.basis[2]},0,{run.basis[3]},{run.basis[4]},{run.basis[5]},0,{run.basis[6]},{run.basis[7]},{run.basis[8]},0,0,0,0,1);",
                                            // A beam is the run's own strip
                                            // plus a back and two folded
                                            // walls — the same anatomy as a
                                            // figure's body, at line scale.
                                            if line == LineageLine::Beam {
                                                i { "data-nd-face": "back" }
                                                i { "data-nd-face": "top" }
                                                i { "data-nd-face": "bottom" }
                                            }
                                        }
                                    }
                                };
                                if picks_bonds {
                                    rsx! {
                                        label { class: "nd-lineage-grip", r#for: "{stem}-b{bond.index}",
                                            {runs}
                                        }
                                    }
                                } else {
                                    rsx! {
                                        span { class: "nd-lineage-grip", {runs} }
                                    }
                                }
                            }
                            if !bond.label.is_empty() {
                                span { class: "nd-lineage-tag", "{bond.label}" }
                            }
                            if !bond.note.is_empty() {
                                span { class: "nd-lineage-note", "{bond.note}" }
                            }
                        }
                    }
                    for (piece, card) in cards {
                        div {
                            class: "nd-lineage-piece",
                            id: "{stem}-{piece.id}",
                            "data-nd-id": "{piece.id}",
                            "data-nd-piece": if piece.elided > 0 {
                                "elided"
                            } else if piece.mark {
                                "mark"
                            } else {
                                "figure"
                            },
                            "aria-label": (piece.elided > 0 && !elided_label.is_empty())
                                .then(|| elided_label.replace("{count}", &piece.elided.to_string())),
                            "data-nd-slot": piece.slot.then_some("true"),
                            "data-nd-kind": some(&piece.kind),
                            "data-nd-tone": some(&piece.tone),
                            "data-nd-shape": some(&piece.shape),
            "data-nd-reach": (piece.reach >= 0.0).then(|| piece.reach.to_string()),
                            "data-nd-set": some(&piece.set),
                            "data-nd-match": filtering.then_some(if piece.matched { "yes" } else { "no" }),
                            hidden: !piece.shown,
                            style: "--nd-x:{piece.x};--nd-y:{piece.y};--nd-z:{piece.z};--nd-span:{piece.span};",
                            if picks_figures && piece.href.is_empty() && !piece.slot {
                                input {
                                    class: "nd-lineage-pick",
                                    r#type: "checkbox",
                                    id: "{stem}-p{piece.id}",
                                    name: some(&form),
                                    value: "{piece.id}",
                                }
                            }
                            // A REAL BOX, NOT A CARD WITH A CARD BEHIND IT.
                            // Five faces — the back and the four walls — so a
                            // figure has volume from any angle a reader turns
                            // it to, and so the ones away from the middle of
                            // the view show their sides even head on, which is
                            // what perspective does to a solid. The first
                            // version was a second rectangle offset in depth,
                            // and it read as exactly that.
                            if !piece.slot && !piece.mark {
                                span { class: "nd-lineage-body", "aria-hidden": "true",
                                    i { "data-nd-face": "back" }
                                    i { "data-nd-face": "top" }
                                    i { "data-nd-face": "bottom" }
                                    i { "data-nd-face": "start" }
                                    i { "data-nd-face": "end" }
                                }
                            }
                            {card}
                            // THE RELATIONSHIPS, AS LINKS RATHER THAN AS PROSE.
                            // A sentence — *child of Ada and Bob* — cannot be
                            // translated and cannot be followed. A link named by
                            // its target is both.
                            if !piece.ties.is_empty() {
                                span { class: "nd-lineage-ties",
                                    for tie in piece.ties {
                                        a {
                                            class: "nd-lineage-tie",
                                            href: "#{stem}-{tie.id}",
                                            "data-nd-kind": some(&tie.kind),
                                            "data-nd-up": tie.upward.then_some("true"),
                                            // THE NAME IS A NAME, AND THE WORD
                                            // BESIDE IT IS A SENTENCE. Marking
                                            // the whole link an identifier
                                            // would exempt the word from the
                                            // check that every sentence went
                                            // through the catalogue, so only
                                            // the name carries the class.
                                            span { class: "nd-identifier", "{tie.label}" }
                                            if !tie.note.is_empty() {
                                                span { class: "nd-lineage-tie-note", " — {tie.note}" }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                    {children}
                }
            }
            if empty {
                p { class: "nd-lineage-empty", "{empty_text}" }
            }
            output {
                class: "nd-lineage-status",
                "aria-live": "polite",
                "data-nd-lineage-status": "",
            }
        }
    }
"##;

/// The same scene, in React.
///
/// # Every name here is one this file declares
///
/// This body was written against a naming convention the generator does not
/// use — `filter_`, `sets_`, `view_`, `root` and a dozen more arrived from
/// nowhere, and `const filter = filter_` redeclared a name that was already the
/// parameter. `lineage.js` did not parse, and had not for an unknown length of
/// time. The generator emits the camel-cased prop, and a body resolves its own
/// defaults into `x_`; both halves of that are asserted now.
///
/// # And `style` is an object
///
/// React's `style` prop takes a map, not a CSS string — a string throws on the
/// first render. Custom properties go in it by name, which is what publishes
/// every piece's position for a `LineageFigure` to stand at.
const LINEAGE_REACT: &str = r#"  const view_ = view ?? "flat";
  const route_ = route ?? "elbow";
  const reach_ = reach ?? "touch";
  const lettering_ = lettering ?? "flat";
  const size_ = size ?? "medium";
  const tilt_ = tilt ?? "level";
  const thickness_ = thickness ?? "slab";
  const filterMode_ = filterMode ?? "highlight";
  const pick_ = pick ?? "none";
  const height_ = height ?? "tall";
  const parallax_ = parallax ?? false;
  const root = ["nd-lineage", className].filter(Boolean).join(" ");
  // The five the Dioxus half unwraps into a plain string before it goes near
  // the engine. An `undefined` reaching `draw` is a filter nothing matches.
  const filter_ = filter ?? "";
  const sets_ = sets ?? "";
  const occasion_ = occasion ?? "";
  const form_ = form ?? "";
  const elidedLabel_ = elidedLabel ?? "";
  const drawing = draw(
    graph,
    { view: view_, route: route_, isolate: filterMode_ === "isolate", shy: reach_ === "shy" },
    filter_,
    sets_,
  );

  // The place of every piece, published as three custom properties — see the
  // Dioxus half for why, and for what has been through `nameable` before it
  // reaches a property name. An OBJECT, because that is what React's `style`
  // takes; a string throws.
  const style = {
    "--nd-w": drawing.width,
    "--nd-h": drawing.height,
    "--nd-d": drawing.depth,
  };
  for (const piece of drawing.pieces) {
    style[`--nd-at-${piece.id}-x`] = piece.x;
    style[`--nd-at-${piece.id}-y`] = piece.y;
    style[`--nd-at-${piece.id}-z`] = piece.z;
    style[`--nd-at-${piece.id}-span`] = piece.span;
  }
  if (nameable(occasion_)) style[`--nd-on-${occasion_}`] = "grid";
  for (const entry of sets_.split(/\s+/u)) {
    const parts = setParts(entry);
    if (parts) style[`--nd-set-${parts[0]}-${parts[1]}`] = "grid";
  }

  const stem = `nd-${name}`;
  // Unset, `line` follows `thickness` — see the Dioxus half.
  const line_ = line ?? (thickness_ === "none" ? "ribbon" : "beam");
  const filtering = filter_ !== "";
  const picksBonds = pick_ === "bond" || pick_ === "both";
  const picksFigures = pick_ === "figure" || pick_ === "both";
  const flag = (yes) => (filtering ? (yes ? "yes" : "no") : undefined);

  const runOf = (run) =>
    h(
      "i",
      {
        key: `${run.bond}-${run.x}-${run.y}-${run.z}-${run.axis}-${run.len}`,
        className: "nd-lineage-run",
        "data-nd-axis": run.axis,
        style: {
          "--nd-x": run.x,
          "--nd-y": run.y,
          "--nd-z": run.z,
          "--nd-len": run.len,
          "--nd-basis": `matrix3d(${run.basis[0]},${run.basis[1]},${run.basis[2]},0,${run.basis[3]},${run.basis[4]},${run.basis[5]},0,${run.basis[6]},${run.basis[7]},${run.basis[8]},0,0,0,0,1)`,
        },
      },
      line_ === "beam"
        ? [
            h("i", { key: "back", "data-nd-face": "back" }),
            h("i", { key: "top", "data-nd-face": "top" }),
            h("i", { key: "bottom", "data-nd-face": "bottom" }),
          ]
        : undefined,
    );

  const bonds = drawing.bonds.map((bond) => {
    const inside = [];
    if (picksBonds) {
      inside.push(
        h("input", {
          key: "pick",
          className: "nd-lineage-pick",
          type: "checkbox",
          id: `${stem}-b${bond.index}`,
          name: some(form_),
          value: `${bond.from} ${bond.to}`,
        }),
      );
    }
    // The line is what a reader presses — see the Dioxus half.
    const runs = bond.runs.map(runOf);
    inside.push(
      picksBonds
        ? h(
            "label",
            { key: "grip", className: "nd-lineage-grip", htmlFor: `${stem}-b${bond.index}` },
            runs,
          )
        : h("span", { key: "grip", className: "nd-lineage-grip" }, runs),
    );
    if (bond.label !== "") {
      inside.push(h("span", { key: "tag", className: "nd-lineage-tag" }, bond.label));
    }
    if (bond.note !== "") {
      inside.push(h("span", { key: "note", className: "nd-lineage-note" }, bond.note));
    }
    return h(
      "div",
      {
        key: `b${bond.index}`,
        className: "nd-lineage-bond",
        "data-nd-bond": String(bond.index),
        "data-nd-kind": some(bond.kind),
        "data-nd-tone": some(bond.tone),
        "data-nd-dash": some(bond.dash),
        "data-nd-set": some(bond.set),
        "data-nd-match": flag(bond.matched),
        "data-nd-tag": bond.tagBelow ? "below" : undefined,
        hidden: !bond.shown,
        style: { "--nd-x": bond.tag[0], "--nd-y": bond.tag[1], "--nd-z": bond.tag[2] },
      },
      inside,
    );
  });

  const pieces = drawing.pieces.map((piece) => {
    const pickable = picksFigures && piece.href === "" && !piece.slot;
    const inside = [];
    if (pickable) {
      inside.push(
        h("input", {
          key: "pick",
          className: "nd-lineage-pick",
          type: "checkbox",
          id: `${stem}-p${piece.id}`,
          name: some(form_),
          value: piece.id,
        }),
      );
    }
    if (!piece.slot && !piece.mark) {
      // A real box, not a card with a card behind it — see the Dioxus half.
      inside.push(
        h(
          "span",
          { key: "body", className: "nd-lineage-body", "aria-hidden": "true" },
          ["back", "top", "bottom", "start", "end"].map((face) =>
            h("i", { key: face, "data-nd-face": face }),
          ),
        ),
      );
    }
    if (!piece.slot) {
      const face = [];
      if (piece.elided > 0) {
        // A NUMERAL IS THE SAME WORD IN EVERY LANGUAGE — see the Dioxus half.
        // The marker carries the count and no sentence; the words that go with
        // it are the scene's `elided_label`, on the `aria-label` below.
        face.push(
          h(
            "span",
            { key: "count", className: "nd-lineage-count nd-identifier" },
            String(piece.elided),
          ),
        );
      } else {
        if (piece.image !== "") {
          face.push(
            h("img", {
              key: "portrait",
              className: "nd-lineage-portrait",
              src: piece.image,
              alt: "",
            }),
          );
        }
        // Solid lettering is stacked plies — see the Dioxus half.
        const plies = (text) =>
          lettering_ === "solid"
            ? [1, 2, 3, 4, 5].map((ply) =>
                h(
                  "i",
                  { key: `ply${ply}`, "data-nd-ply": String(ply), "aria-hidden": "true" },
                  text,
                ),
              )
            : [];
        face.push(
          h(
            "span",
            { key: "name", className: "nd-lineage-name nd-identifier" },
            piece.label,
            ...plies(piece.label),
          ),
        );
        if (piece.meta !== "") {
          face.push(
            h(
              "span",
              { key: "meta", className: "nd-lineage-meta nd-identifier" },
              piece.meta,
              ...plies(piece.meta),
            ),
          );
        }
      }
      // One content in three elements: a link, a pick target, or neither.
      if (piece.href !== "") {
        inside.push(
            h(
            "a",
            {
              key: "card",
              className: "nd-lineage-card",
              href: piece.href,
            },
            face,
          ),
        );
      } else if (pickable) {
        inside.push(
          h(
            "label",
            {
              key: "card",
              className: "nd-lineage-card",
              htmlFor: `${stem}-p${piece.id}`,
            },
            face,
          ),
        );
      } else {
        inside.push(
          h(
            "span",
            { key: "card", className: "nd-lineage-card" },
            face,
          ),
        );
      }
    }
    if (piece.ties.length > 0) {
      inside.push(
        h(
          "span",
          { key: "ties", className: "nd-lineage-ties" },
          piece.ties.map((tie, at) =>
            h(
              "a",
              {
                key: at,
                className: "nd-lineage-tie",
                href: `#${stem}-${tie.id}`,
                "data-nd-kind": some(tie.kind),
                "data-nd-up": tie.upward ? "true" : undefined,
              },
              // The name is a name; the word beside it is a sentence.
              h("span", { className: "nd-identifier" }, tie.label),
              tie.note === ""
                ? null
                : h("span", { className: "nd-lineage-tie-note" }, ` — ${tie.note}`),
            ),
          ),
        ),
      );
    }
    return h(
      "div",
      {
        key: piece.id,
        className: "nd-lineage-piece",
        id: `${stem}-${piece.id}`,
        "data-nd-id": piece.id,
        "data-nd-piece": piece.elided > 0 ? "elided" : piece.mark ? "mark" : "figure",
      "aria-label":
        piece.elided > 0 && elidedLabel_ !== ""
          ? elidedLabel_.replace("{count}", String(piece.elided))
          : undefined,
        "data-nd-slot": piece.slot ? "true" : undefined,
        "data-nd-kind": some(piece.kind),
        "data-nd-tone": some(piece.tone),
        "data-nd-shape": some(piece.shape),
        "data-nd-reach": piece.reach >= 0 ? String(piece.reach) : undefined,
        "data-nd-set": some(piece.set),
        "data-nd-match": flag(piece.matched),
        hidden: !piece.shown,
        style: { "--nd-x": piece.x, "--nd-y": piece.y, "--nd-z": piece.z, "--nd-span": piece.span },
      },
      inside,
    );
  });

  return h(
    "div",
    {
      className: root,
      "data-nd-lineage": name,
      "data-nd-view": view_,
      "data-nd-route": route_,
      "data-nd-size": size_,
      "data-nd-tilt": tilt_,
      "data-nd-thickness": thickness_,
      "data-nd-line": line_,
      "data-nd-reach": reach_,
      "data-nd-lettering": lettering_,
      "data-nd-height": height_,
      "data-nd-pick": pick_,
      "data-nd-filter-mode": filterMode_,
      "data-nd-parallax": parallax_ ? "true" : "false",
      "data-nd-filter": some(filter_),
      "data-nd-occasion": some(occasion_),
      "data-nd-sets": some(sets_),
      "data-nd-form": some(form_),
      "data-nd-empty-label": emptyLabel,
      "data-nd-camera-label": cameraLabel,
      "data-nd-zoom-in-label": zoomInLabel,
      "data-nd-zoom-out-label": zoomOutLabel,
      "data-nd-fit-label": fitLabel,
      "data-nd-matches-label": matchesLabel,
      "data-nd-skipped": drawing.skipped > 0 ? String(drawing.skipped) : undefined,
      "data-nd-broken": drawing.broken > 0 ? String(drawing.broken) : undefined,
      "data-nd-crossings": String(drawing.crossings),
      "data-nd-matched": String(drawing.matched),
      style,
      ...rest,
    },
    h(
      "div",
      {
        className: "nd-lineage-field",
        id: stem,
        role: "group",
        "aria-label": label,
        tabIndex: 0,
        "data-nd-lineage-field": "",
      },
      h(
        "div",
        { className: "nd-lineage-space", "data-nd-lineage-space": "" },
        bonds,
        pieces,
        children,
      ),
    ),
    drawing.pieces.length === 0
      ? h("p", { className: "nd-lineage-empty" }, emptyLabel ?? "")
      : null,
    h("output", {
      className: "nd-lineage-status",
      "aria-live": "polite",
      "data-nd-lineage-status": "",
    }),
  );
"#;

/// A figure standing at a named point, in Dioxus.
const LINEAGE_FIGURE_DIOXUS: &str = r#"    let root = format!("nd-lineage-figure {class}");
    let root = root.trim().to_owned();

    // THE PLACE COMES FROM THE SCENE, THROUGH THREE CUSTOM PROPERTIES, and the
    // visibility through a fourth. `at` is a word — the parser guarantees that
    // for a graph id and `nameable` for anything else — so nothing here can end
    // the declaration and start another one. An `at` naming nothing resolves to
    // the fallback and the figure stands at the origin, which is visible rather
    // than silent.
    let anchor = if nameable(&at) { at.clone() } else { String::new() };
    let mut style = format!(
        "--nd-x:var(--nd-at-{anchor}-x,0);--nd-y:var(--nd-at-{anchor}-y,0);\
         --nd-z:var(--nd-at-{anchor}-z,0);--nd-span:var(--nd-at-{anchor}-span,1);"
    );
    // ONE VISIBILITY VARIABLE, AND THE OCCASION WINS. Two independent reasons
    // to be hidden cannot both be expressed in one custom property, and a
    // figure that needed both would be two figures — which the contract says.
    if let Some(word) = occasion.as_deref().filter(|word| nameable(word)) {
        let _ = write!(style, "--nd-shown:var(--nd-on-{word},none);");
    } else if let Some((group, chosen)) = set.as_deref().and_then(set_parts) {
        let _ = write!(style, "--nd-shown:var(--nd-set-{group}-{chosen},none);");
    }

    let inside = rsx! { {children} };
    rsx! {
        match (href.clone(), action.clone()) {
            // A LINK OR A BUTTON OR NEITHER, NEVER TWO. `Tag` states the rule
            // and this keeps it: an href beside an action wins, and the action
            // is dropped rather than attached to something that is not a button.
            (Some(href), _) => rsx! {
                a {
                    class: "{root}",
                    href: "{href}",
                    "data-nd-scene": "{scene}",
                    "data-nd-at": "{at}",
                    "data-nd-shape": shape.as_str(),
                    "data-nd-tone": tone.as_str(),
                    "data-nd-parallax": parallax.as_str(),
                    "data-nd-occasion": occasion.clone(),
                    "data-nd-set": set.clone(),
                    "aria-label": label.clone(),
                    style: "{style}",
                    {inside}
                }
            },
            (None, Some(action)) => rsx! {
                button {
                    class: "{root}",
                    r#type: "button",
                    "data-nd-scene": "{scene}",
                    "data-nd-at": "{at}",
                    "data-nd-action": "{action}",
                    "data-nd-swaps": swaps.clone(),
                    // A toggle says its state from the start; the runtime
                    // keeps it true to the shown set.
                    "aria-pressed": swaps.as_deref().map(|_| "false"),
                    "data-nd-shape": shape.as_str(),
                    "data-nd-tone": tone.as_str(),
                    "data-nd-parallax": parallax.as_str(),
                    "data-nd-occasion": occasion.clone(),
                    "data-nd-set": set.clone(),
                    "aria-label": label.clone(),
                    style: "{style}",
                    {inside}
                }
            },
            (None, None) => rsx! {
                div {
                    class: "{root}",
                    "data-nd-scene": "{scene}",
                    "data-nd-at": "{at}",
                    "data-nd-shape": shape.as_str(),
                    "data-nd-tone": tone.as_str(),
                    "data-nd-parallax": parallax.as_str(),
                    "data-nd-occasion": occasion.clone(),
                    "data-nd-set": set.clone(),
                    "aria-label": label.clone(),
                    style: "{style}",
                    {inside}
                }
            },
        }
    }
"#;

/// The same figure, in React.
///
/// Its defaults and its root class are resolved here, the way every other
/// React body in this file resolves them — this one named `root`, `shape_`,
/// `tone_` and `parallax_` without declaring any of them, so the component
/// threw a `ReferenceError` on its first render. And `style` is an object,
/// because that is what React's `style` prop takes; the CSS string this used
/// to build threw on the same render.
const LINEAGE_FIGURE_REACT: &str = r#"  const shape_ = shape ?? "card";
  const tone_ = tone ?? "neutral";
  const parallax_ = parallax ?? "none";
  const root = ["nd-lineage-figure", className].filter(Boolean).join(" ");
  // The place comes from the scene, through three custom properties, and the
  // visibility through a fourth — see the Dioxus half for what has been
  // through `nameable` before it reaches a property name.
  const anchor = nameable(at) ? at : "";
  const style = {
    "--nd-x": `var(--nd-at-${anchor}-x,0)`,
    "--nd-y": `var(--nd-at-${anchor}-y,0)`,
    "--nd-z": `var(--nd-at-${anchor}-z,0)`,
    "--nd-span": `var(--nd-at-${anchor}-span,1)`,
  };
  // One visibility variable, and the occasion wins — see the Dioxus half.
  if (nameable(occasion)) {
    style["--nd-shown"] = `var(--nd-on-${occasion},none)`;
  } else {
    const parts = setParts(set);
    if (parts) style["--nd-shown"] = `var(--nd-set-${parts[0]}-${parts[1]},none)`;
  }

  // A link or a button or neither, never two.
  const element = href ? "a" : action ? "button" : "div";
  const props = {
    className: root,
    "data-nd-scene": scene,
    "data-nd-at": at,
    "data-nd-shape": shape_,
    "data-nd-tone": tone_,
    "data-nd-parallax": parallax_,
    "data-nd-occasion": occasion,
    "data-nd-set": set,
    "aria-label": label,
    style,
    ...rest,
  };
  if (href) props.href = href;
  if (!href && action) {
    props.type = "button";
    props["data-nd-action"] = action;
    props["data-nd-swaps"] = swaps;
    // A toggle says its state from the start — see the Dioxus half.
    if (swaps) props["aria-pressed"] = "false";
  }
  return h(element, props, children);
"#;

/// The same node, in React.
const TREE_ITEM_REACT: &str = r#"  const root = ["nd-tree-item", className].filter(Boolean).join(" ");

  // See the Dioxus half for every decision here.
  const row = [
    icon
      ? h(
          "span",
          { key: "g", className: "nd-tree-glyph", "aria-hidden": "true" },
          h(Icon, { key: "a", icon, size: "small" }),
          iconExpanded ? h(Icon, { key: "b", icon: iconExpanded, size: "small" }) : null,
        )
      : null,
    h("span", { key: "l", className: "nd-tree-label" }, label),
    badge
      ? h(Tag, {
          key: "t",
          className: "nd-tree-badge",
          label: badge,
          size: "small",
          variant: "outline",
        })
      : null,
  ];

  return h(
    "li",
    {
      className: root,
      "data-nd-value": value,
      "data-nd-expanded": String(expanded ?? false),
      "data-nd-branch": String(branch ?? false),
      "data-nd-selected": String(selected ?? false),
      "aria-disabled": disabled ? "true" : undefined,
      ...rest,
    },
    h(
      "div",
      { key: "r", className: "nd-tree-row" },
      h(
        "button",
        {
          key: "x",
          className: "nd-tree-toggle",
          type: "button",
          hidden: true,
          "data-nd-tree-toggle": "",
        },
        h(Icon, { icon: chevronDown, size: "small" }),
      ),
      href
        ? h(
            "a",
            {
              key: "b",
              className: "nd-tree-body",
              href,
              "aria-current": current ? "page" : undefined,
            },
            ...row,
          )
        : h(
            "span",
            { key: "b", className: "nd-tree-body", "aria-current": current ? "page" : undefined },
            ...row,
          ),
    ),
    h("ul", { key: "g", className: "nd-tree-group", "data-nd-tree-group": "" }, children),
  );
"#;

// --- CodeBlock --------------------------------------------------------------

/// The block that reads its own content.
///
/// # What changed, and why everything else follows from it
///
/// The component this replaces took the code as **children**, so it could not
/// look at it: no line to number, no line to mark, no token to colour, no diff
/// to find. Its own contract said so, in the note explaining why it had no
/// `numbered` prop — *"re-adding it needs a `code` text prop first, and that is
/// a different component"*. This is that component, and every feature below is
/// downstream of one word in the contract.
///
/// # Why the highlighting is here and not in `shell.js`
///
/// This package ships no runtime, and a consumer of the Dioxus crate or the
/// React package has no `shell.js` to fall back on. A highlighter in the script
/// would leave every one of them with grey code, and would repaint the page
/// after first paint on the one site that did have it. So the scan happens
/// where the component renders — twice, from one algorithm, which is what
/// `crates/noctua-design-grammars` exists for.
///
/// What the script still owns is the copy, the wrap toggle and the collapse,
/// and each degrades to something *correct* when it never runs: no button, no
/// wrapping, and the whole block visible.
///
/// # The three things the markup owes the stylesheet
///
/// One element per line, so a counter can number them and a rule can mark one.
/// One element per token, carrying `data-nd-tok`, which is the only thing
/// `dist/syntax.css` keys on. And a newline of real text after every line, so
/// `textContent` reproduces the code exactly — that is what the copy control
/// puts on the clipboard, and it is why the line numbers are drawn by a
/// pseudo-element rather than written into the markup.
const CODE_BLOCK_DIOXUS: &str = r#"    let named = Language::parse(language.as_deref().unwrap_or_default());
    let grammar = named.unwrap_or(Language::Plain).grammar();
    let mut rows = lines(&scan(grammar, &code));
    // A trailing newline makes a final empty group. Rendering it would put a
    // blank line at the bottom of every block that ends the way a file does.
    let trailing = code.ends_with('\n');
    if trailing && rows.len() > 1 {
        rows.pop();
    }

    // `3,7-9`. Anything unparseable is silently no mark rather than an error:
    // this is a presentation hint, and a build that stopped over one would be
    // stopping over the wrong thing.
    let mut marked: Vec<usize> = Vec::new();
    for part in mark_lines.as_deref().unwrap_or_default().split(',') {
        let part = part.trim();
        match part.split_once('-') {
            Some((from, to)) => {
                if let (Ok(from), Ok(to)) = (from.trim().parse(), to.trim().parse::<usize>()) {
                    for line in from..=to {
                        marked.push(line);
                    }
                }
            }
            None => {
                if let Ok(line) = part.parse() {
                    marked.push(line);
                }
            }
        }
    }

    let first: usize = start_line
        .as_deref()
        .and_then(|value| value.trim().parse().ok())
        .unwrap_or(1);
    let is_diff = named == Some(Language::Diff);

    // THE REGION IS NAMED OR IT IS NOT A REGION. An unnamed landmark is one a
    // screen-reader user arrives inside with nothing telling them where they
    // are, which is worse than no landmark at all.
    let name = filename
        .clone()
        .or_else(|| named.filter(|l| *l != Language::Plain).map(|l| l.display().to_owned()));
    let region = name.as_ref().map(|_| "region");
    let badge = named.filter(|l| *l != Language::Plain).map(Language::display);
    let bar = filename.is_some() || copy.is_some() || wrap_label.is_some() || badge.is_some();

    let mut style = format!("counter-reset: nd-code-line {};", first.saturating_sub(1));
    if let Some(max) = max_lines.as_deref().and_then(|v| v.trim().parse::<usize>().ok()) {
        style = format!("{style} --nd-code-max: {max};");
    }
    let collapsible = expand_label.is_some() && max_lines.is_some();
    let root = format!("nd-codeblock {class}");
    let root = root.trim().to_owned();

    rsx! {
        figure {
            class: "nd-codeblock-figure",
            "data-nd-collapsible": if collapsible { Some("true") } else { None },
            if bar {
                div { class: "nd-codeblock-bar",
                    if let Some(filename) = filename.clone() {
                        span { class: "nd-codeblock-name nd-identifier", "{filename}" }
                    }
                    if let Some(badge) = badge {
                        span { class: "nd-codeblock-badge nd-identifier", "{badge}" }
                    }
                    if let Some(wrap_label) = wrap_label.clone() {
                        button {
                            class: "nd-codeblock-control",
                            r#type: "button",
                            "aria-label": "{wrap_label}",
                            title: "{wrap_label}",
                            "aria-pressed": if wrap { "true" } else { "false" },
                            "data-nd-wrap-toggle": "",
                            Icon { icon: IconDrawing::TEXT_WRAP }
                        }
                    }
                    if let Some(copy) = copy.clone() {
                        button {
                            class: "nd-codeblock-control",
                            r#type: "button",
                            "aria-label": "{copy}",
                            title: "{copy}",
                            "data-nd-copy": "",
                            "data-nd-copied": copied.clone(),
                            Icon { icon: IconDrawing::COPY }
                            Icon { icon: IconDrawing::CHECK }
                        }
                        // Empty until the copy succeeds: a live region that
                        // already holds its text announces nothing when that
                        // text is replaced by the same text.
                        span {
                            class: "nd-visually-hidden",
                            role: "status",
                            "data-nd-copy-status": "",
                        }
                    }
                }
            }
            pre {
                class: "{root}",
                style: "{style}",
                "data-nd-code-theme": theme.as_str(),
                "data-nd-language": language.clone(),
                "data-nd-numbered": if numbered { "true" } else { "false" },
                "data-nd-wrap": if wrap { "true" } else { "false" },
                "data-nd-focus": if focus { "true" } else { "false" },
                "data-nd-glass": if glass { "true" } else { "false" },
                // THE CODE IS THE FOCUSABLE SCROLLER, not the <pre>: the bar
                // above holds buttons, and a reader tabbing in to READ should
                // not have to tab through them first. And the <pre> keeps the
                // radius while this keeps the overflow, because a scrollable
                // element never carries a radius — a classic scrollbar paints
                // square straight through a rounded corner.
                code {
                    class: "nd-codeblock-code",
                    tabindex: "0",
                    role: region,
                    "aria-label": name.clone(),
                    for (index, row) in rows.iter().enumerate() {
                        span {
                            class: "nd-codeblock-line",
                            "data-nd-mark": if marked.contains(&(first + index)) { Some("true") } else { None },
                            "data-nd-line": if is_diff { row.first().map(|t| match t.role {
                                Role::Inserted => "inserted",
                                Role::Deleted => "deleted",
                                Role::Comment | Role::Attribute => "meta",
                                _ => "context",
                            }) } else { None },
                            for token in row {
                                span { "data-nd-tok": token.role.as_str(), "{token.text}" }
                            }
                        }
                        // A REAL NEWLINE, BESIDE THE LINE AND NOT INSIDE IT.
                        //
                        // Inside, it would be a forced break at the end of a
                        // block box and every line would be followed by an
                        // empty one. Beside, it sits in the `<code>`, whose
                        // `white-space` is `normal` — so it collapses to
                        // nothing on screen and stays in `textContent`, which
                        // is what the copy control reads. That is the whole
                        // reason the line numbers are drawn by a counter
                        // rather than written into the markup.
                        if index + 1 < rows.len() || trailing {
                            "\n"
                        }
                    }
                }
            }
            if collapsible {
                button {
                    class: "nd-codeblock-expand",
                    r#type: "button",
                    "aria-expanded": "false",
                    "data-nd-expand": "",
                    "data-nd-collapse-label": collapse_label.clone(),
                    "{expand_label.clone().unwrap_or_default()}"
                }
            }
            if let Some(caption) = caption.clone() {
                figcaption { class: "nd-codeblock-caption", "{caption}" }
            }
        }
    }
"#;

/// The same component, in React. Read it beside `CODE_BLOCK_DIOXUS`.
const CODE_BLOCK_REACT: &str = r#"  const numbered_ = numbered ?? false;
  const wrap_ = wrap ?? false;
  const focus_ = focus ?? false;
  const glass_ = glass ?? false;
  const theme_ = theme ?? "auto";
  const grammar = GRAMMARS[language] ?? GRAMMARS[ALIASES[language]] ?? GRAMMARS.plain;
  const named = grammar !== GRAMMARS.plain ? grammar : null;
  const rows = lines(scan(grammar, code));
  const trailing = code.endsWith("\n");
  if (trailing && rows.length > 1) rows.pop();

  // `3,7-9`. Anything unparseable is silently no mark rather than an error.
  const marked = new Set();
  for (const part of (markLines ?? "").split(",")) {
    const [from, to] = part.trim().split("-");
    const start = Number.parseInt(from, 10);
    const end = to === undefined ? start : Number.parseInt(to, 10);
    if (Number.isNaN(start) || Number.isNaN(end)) continue;
    for (let line = start; line <= end; line += 1) marked.add(line);
  }

  const first = Number.parseInt(startLine ?? "", 10) || 1;
  const isDiff = grammar.name === "diff";
  const name = filename ?? named?.display ?? null;
  const badge = named?.display ?? null;
  const bar = Boolean(filename || copy || wrapLabel || badge);
  const maximum = Number.parseInt(maxLines ?? "", 10);
  const style = { counterReset: `nd-code-line ${Math.max(first - 1, 0)}` };
  if (!Number.isNaN(maximum)) style["--nd-code-max"] = maximum;
  const collapsible = Boolean(expandLabel && maxLines);
  const root = ["nd-codeblock", className].filter(Boolean).join(" ");

  const lineKind = (row) => {
    if (!isDiff || row.length === 0) return undefined;
    const role = row[0].role;
    if (role === "inserted" || role === "deleted") return role;
    if (role === "comment" || role === "attribute") return "meta";
    return "context";
  };

  return h(
    "figure",
    { className: "nd-codeblock-figure", "data-nd-collapsible": collapsible ? "true" : undefined, ...rest },
    bar
      ? h(
          "div",
          { key: "bar", className: "nd-codeblock-bar" },
          filename
            ? h("span", { key: "n", className: "nd-codeblock-name nd-identifier" }, filename)
            : null,
          badge
            ? h("span", { key: "l", className: "nd-codeblock-badge nd-identifier" }, badge)
            : null,
          wrapLabel
            ? h(
                "button",
                {
                  key: "w",
                  className: "nd-codeblock-control",
                  type: "button",
                  "aria-label": wrapLabel,
                  title: wrapLabel,
                  "aria-pressed": String(wrap_),
                  "data-nd-wrap-toggle": "",
                },
                h(Icon, { icon: textWrap }),
              )
            : null,
          copy
            ? h(
                "button",
                {
                  key: "c",
                  className: "nd-codeblock-control",
                  type: "button",
                  "aria-label": copy,
                  title: copy,
                  "data-nd-copy": "",
                  "data-nd-copied": copied,
                },
                h(Icon, { key: "i", icon: copy }),
                h(Icon, { key: "k", icon: check }),
              )
            : null,
          copy
            ? h("span", {
                key: "s",
                className: "nd-visually-hidden",
                role: "status",
                "data-nd-copy-status": "",
              })
            : null,
        )
      : null,
    h(
      "pre",
      {
        key: "p",
        className: root,
        style,
        "data-nd-code-theme": theme_,
        "data-nd-language": language,
        "data-nd-numbered": String(numbered_),
        "data-nd-wrap": String(wrap_),
        "data-nd-focus": String(focus_),
        "data-nd-glass": String(glass_),
      },
      // The code is the focusable scroller, not the <pre>. See the Dioxus half.
      h(
        "code",
        {
          className: "nd-codeblock-code",
          tabIndex: 0,
          role: name ? "region" : undefined,
          "aria-label": name ?? undefined,
        },
        // A REAL NEWLINE, BESIDE EACH LINE AND NOT INSIDE IT. See the Dioxus
        // half: inside, it is a forced break at the end of a block box and
        // every line gets an empty one after it.
        rows.flatMap((row, index) => {
          const line = h(
            "span",
            {
              key: index,
              className: "nd-codeblock-line",
              "data-nd-mark": marked.has(first + index) ? "true" : undefined,
              "data-nd-line": lineKind(row),
            },
            ...row.map((token, at) =>
              h("span", { key: at, "data-nd-tok": token.role }, token.text),
            ),
          );
          return index + 1 < rows.length || trailing ? [line, "\n"] : [line];
        }),
      ),
    ),
    collapsible
      ? h(
          "button",
          {
            key: "e",
            className: "nd-codeblock-expand",
            type: "button",
            "aria-expanded": "false",
            "data-nd-expand": "",
            "data-nd-collapse-label": collapseLabel,
          },
          expandLabel,
        )
      : null,
    caption ? h("figcaption", { key: "f", className: "nd-codeblock-caption" }, caption) : null,
  );
"#;

// --- Search -----------------------------------------------------------------

/// The box, the results, and the words the script needs to say anything.
///
/// # What it replaces, and the two defects it was built around
///
/// `SearchField` was an `<input>`, a hidden `<label>` and a live region. The
/// results were three hundred hidden `<a>` elements a *page* rendered, the
/// empty state was a stray paragraph beside them, and nothing tied the three
/// together — so a reader who could not see the list shrink heard *twelve
/// results* and had no announced way to reach one.
///
/// And it derived its id from an optional name:
/// `format!("nd-search-{}", name.unwrap_or_default())`, so four unnamed fields
/// on one page all carried `id="nd-search-"` with four `<label for>` pointing
/// at whichever the browser resolved first. `name` is **required** here, which
/// makes that unrepresentable rather than reviewable.
///
/// # What is deliberately not written here
///
/// `role="combobox"`, `aria-expanded`, `aria-controls`, `aria-autocomplete`,
/// `aria-activedescendant` and `role="listbox"` are the script's. A box
/// claiming to control a listbox that nothing can open is a promise the markup
/// does not keep — the rule `Segmented` already states — and with the script
/// blocked this is a labelled search box beside a list of real links.
const SEARCH_DIOXUS: &str = r#"    let id = format!("nd-search-{name}");
    let root = format!("nd-search {class}");
    let root = root.trim().to_owned();
    let dialog_form = presentation == SearchPresentation::Dialog;
    // `reset` inside a form empties the field in every engine with no script at
    // all; outside one there is nothing to reset, so the script does it.
    let clear_kind = if action.is_some() { "reset" } else { "button" };
    let clear = if clearable { clear_label.clone() } else { None };

    let row = rsx! {
        span { class: "nd-search-glyph", "aria-hidden": "true",
            Icon { icon: IconDrawing::SEARCH, size: crate::components::IconSize::Small }
        }
        label { class: "nd-visually-hidden", id: "{id}-label", r#for: "{id}", "{label}" }
        input {
            class: "nd-search-input",
            id: "{id}",
            r#type: "search",
            name: "{query}",
            // A search box is not a name, an address or a sentence, and every
            // one of these is a browser trying to be helpful about one of
            // those.
            autocomplete: "off",
            autocorrect: "off",
            autocapitalize: "none",
            spellcheck: "false",
            enterkeyhint: "search",
            placeholder,
            value,
            disabled,
            "data-nd-search-input": "",
        }
        if hotkey {
            // BOTH SPELLINGS SHIP AND THE STYLESHEET SHOWS ONE. Which modifier
            // a reader has is a fact about their hardware, and a pre-rendered
            // page does not know it — a hint corrected after paint is the
            // flicker `NOCTUA.md §9.6` forbids.
            span { class: "nd-search-hotkey", "aria-hidden": "true",
                kbd { class: "nd-kbd nd-identifier", "data-nd-size": "small", "data-nd-mod": "apple", "⌘" }
                kbd { class: "nd-kbd nd-identifier", "data-nd-size": "small", "data-nd-mod": "other", "Ctrl" }
                kbd { class: "nd-kbd nd-identifier", "data-nd-size": "small", "K" }
            }
        }
        if let Some(clear) = clear {
            button {
                class: "nd-search-clear",
                r#type: clear_kind,
                "aria-label": "{clear}",
                title: "{clear}",
                "data-nd-search-clear": "",
                Icon { icon: IconDrawing::CLOSE, size: crate::components::IconSize::Small }
            }
        }
    };

    let field = rsx! {
        if let Some(action) = action.clone() {
            form { class: "nd-search-field", method: "get", action: "{action}", {row} }
        } else {
            div { class: "nd-search-field", {row} }
        }
    };

    let panel = rsx! {
        div { class: "nd-search-panel", id: "{id}-panel", "data-nd-search-panel": "",
            // ONE ELEMENT, FIVE SENTENCES, ONE AT A TIME — the count, nothing
            // matched, still loading, never arrived, and the list was cut.
            // Shown and announced by the same node, so a count worth telling a
            // screen reader is a count a reader can see.
            p {
                class: "nd-search-note",
                id: "{id}-status",
                role: "status",
                "aria-live": "polite",
                "aria-atomic": "true",
                "data-nd-search-status": "",
            }
            if recent.is_some() {
                section {
                    class: "nd-search-recent",
                    id: "{id}-recent",
                    "aria-labelledby": "{id}-recent-label",
                    "data-nd-search-recent": "",
                    hidden: true,
                    p { class: "nd-search-recent-label", id: "{id}-recent-label",
                        "{recent_label.clone().unwrap_or_default()}"
                    }
                    ul { class: "nd-search-recent-list", "data-nd-search-recent-list": "" }
                    if let Some(forget) = forget_label.clone() {
                        button {
                            class: "nd-search-forget",
                            r#type: "button",
                            "data-nd-search-forget": "",
                            "{forget}"
                        }
                    }
                }
            }
            ul {
                class: "nd-search-results",
                id: "{id}-results",
                "aria-labelledby": "{id}-label",
                "data-nd-search-results": "",
                {children}
            }
        }
    };

    rsx! {
        div {
            class: "{root}",
            // `<div role="search">` and not `<search>`: dioxus-html has no such
            // element, and both implementations must render the same markup.
            role: "search",
            "aria-labelledby": "{id}-label",
            "data-nd-search": "{name}",
            "data-nd-presentation": presentation.as_str(),
            "data-nd-size": size.as_str(),
            "data-nd-glass": if glass { "true" } else { "false" },
            "data-nd-filter": if filter { "true" } else { "false" },
            "data-nd-hotkey": if hotkey { "true" } else { "false" },
            "data-nd-clearable": if clearable { "true" } else { "false" },
            "data-nd-loading": if loading { "true" } else { "false" },
            "data-nd-src": src,
            "data-nd-cap": cap,
            "data-nd-recent": recent,
            "data-nd-status": status,
            "data-nd-empty-label": empty_label,
            "data-nd-loading-label": loading_label,
            "data-nd-error-label": error_label,
            "data-nd-cap-label": cap_label,
            "data-nd-close-label": close_label,
            if dialog_form {
                // HIDDEN UNTIL THE SCRIPT ARRIVES, and that is the one place
                // this repository renders something hidden and unhides it.
                // The rule exists so a page with no script is complete; a
                // control that opens a modal nothing can fill is not complete,
                // it is a button that does nothing.
                button {
                    class: "nd-search-open",
                    r#type: "button",
                    "aria-haspopup": "dialog",
                    "data-nd-search-open": "",
                    hidden: true,
                    span { class: "nd-search-glyph", "aria-hidden": "true",
                        Icon { icon: IconDrawing::SEARCH, size: crate::components::IconSize::Small }
                    }
                    span { class: "nd-search-open-text", "{label}" }
                    if hotkey {
                        span { class: "nd-search-hotkey", "aria-hidden": "true",
                            kbd { class: "nd-kbd nd-identifier", "data-nd-size": "small", "data-nd-mod": "apple", "⌘" }
                            kbd { class: "nd-kbd nd-identifier", "data-nd-size": "small", "data-nd-mod": "other", "Ctrl" }
                            kbd { class: "nd-kbd nd-identifier", "data-nd-size": "small", "K" }
                        }
                    }
                }
                dialog {
                    class: "nd-search-dialog",
                    id: "{id}-dialog",
                    "aria-labelledby": "{id}-label",
                    "data-nd-search-dialog": "",
                    // THE CONTROL `close_label` NAMES, and it had none.
                    //
                    // The prop was declared, stamped as `data-nd-close-label`
                    // and read by nothing, so the only ways out of the palette
                    // were Escape and a tap on the backdrop — neither of which
                    // is a thing anyone can see. Its name is still the opt-in,
                    // like `CodeBlock { copy }`: no name, no control, because
                    // this package cannot name it in the reader's language.
                    div { class: "nd-search-head",
                        {field}
                        if let Some(close) = close_label.clone() {
                            button {
                                class: "nd-search-close",
                                r#type: "button",
                                "aria-label": "{close}",
                                title: "{close}",
                                "data-nd-search-close": "",
                                Icon { icon: IconDrawing::CLOSE, size: crate::components::IconSize::Small }
                            }
                        }
                    }
                    {panel}
                }
            } else {
                {field}
                {panel}
            }
        }
    }
"#;

/// A named run of results.
///
/// An `<li>` holding a `<p>` and a nested `<ul>`, which is the only shape that
/// works: an `<li>` cannot hold an `<li>`. It is also the shape that needs no
/// ARIA at all with the script blocked — a screen reader reads the outer item's
/// text before entering the nested list, so the run is named by being written
/// that way. Enhanced, the script adds `role="group"` here and `role="none"` on
/// the inner list, because a `listbox` holds groups and options and nothing else.
const SEARCH_GROUP_DIOXUS: &str = r#"    let root = format!("nd-search-group {class}");
    let root = root.trim().to_owned();

    rsx! {
        li { class: "{root}", "data-nd-search-group": "{label}",
            p { class: "nd-search-group-label", "{label}" }
            ul { class: "nd-search-group-list", {children} }
        }
    }
"#;

/// One result: a row that becomes an option, and a link that stays a link.
///
/// `role="option"` lands on the `<li>` and the `<a>` is never touched, so
/// middle-click, Command-click, *copy link address* and the status-bar preview
/// all survive. Putting the role on the anchor satisfies the same contract and
/// costs every one of them.
const SEARCH_RESULT_DIOXUS: &str = r#"    let root = format!("nd-search-result {class}");
    let root = root.trim().to_owned();

    rsx! {
        li { class: "{root}", "data-nd-keywords": keywords,
            a {
                class: "nd-search-result-link",
                href: "{href}",
                "aria-current": if current { "page" },
                // The label is its own element because the script rewrites it
                // into `<mark>` runs over the letters that matched, and it must
                // be able to find exactly the text it is allowed to touch.
                span { class: "nd-search-result-label", "data-nd-search-label": "", "{label}" }
                if let Some(badge) = badge {
                    span { class: "nd-search-result-badge", "{badge}" }
                }
                if let Some(detail) = detail {
                    span { class: "nd-search-result-detail", "{detail}" }
                }
                if current {
                    // The non-colour half of `aria-current`: a list of places
                    // owes a mark to a reader who cannot see which row is lit.
                    Icon { icon: IconDrawing::CHECK, size: crate::components::IconSize::Small }
                }
            }
        }
    }
"#;

/// The same component, in React. Read it beside `SEARCH_DIOXUS`.
const SEARCH_REACT: &str = r#"  const presentation_ = presentation ?? "inline";
  const size_ = size ?? "medium";
  const query_ = query ?? "q";
  const clearable_ = clearable ?? true;
  const id = `nd-search-${name}`;
  const dialogForm = presentation_ === "dialog";
  const clearKind = action ? "reset" : "button";
  const hotkeyHint = hotkey
    ? h(
        "span",
        { key: "h", className: "nd-search-hotkey", "aria-hidden": "true" },
        h("kbd", { key: "a", className: "nd-kbd nd-identifier", "data-nd-size": "small", "data-nd-mod": "apple" }, "⌘"),
        h("kbd", { key: "o", className: "nd-kbd nd-identifier", "data-nd-size": "small", "data-nd-mod": "other" }, "Ctrl"),
        h("kbd", { key: "k", className: "nd-kbd nd-identifier", "data-nd-size": "small" }, "K"),
      )
    : null;

  const row = [
    h(
      "span",
      { key: "g", className: "nd-search-glyph", "aria-hidden": "true" },
      h(Icon, { icon: search, size: "small" }),
    ),
    h("label", { key: "l", className: "nd-visually-hidden", id: `${id}-label`, htmlFor: id }, label),
    h("input", {
      key: "i",
      className: "nd-search-input",
      id,
      type: "search",
      name: query_,
      autoComplete: "off",
      autoCorrect: "off",
      autoCapitalize: "none",
      spellCheck: "false",
      enterKeyHint: "search",
      placeholder,
      defaultValue: value,
      disabled: disabled ?? false,
      "data-nd-search-input": "",
    }),
    hotkeyHint,
    clearable_ && clearLabel
      ? h(
          "button",
          {
            key: "c",
            className: "nd-search-clear",
            type: clearKind,
            "aria-label": clearLabel,
            title: clearLabel,
            "data-nd-search-clear": "",
          },
          h(Icon, { icon: close, size: "small" }),
        )
      : null,
  ];

  const field = action
    ? h("form", { key: "f", className: "nd-search-field", method: "get", action }, ...row)
    : h("div", { key: "f", className: "nd-search-field" }, ...row);

  const panel = h(
    "div",
    { key: "p", className: "nd-search-panel", id: `${id}-panel`, "data-nd-search-panel": "" },
    // One element, five sentences, one at a time. See the Dioxus half.
    h("p", {
      key: "s",
      className: "nd-search-note",
      id: `${id}-status`,
      role: "status",
      "aria-live": "polite",
      "aria-atomic": "true",
      "data-nd-search-status": "",
    }),
    recent
      ? h(
          "section",
          {
            key: "r",
            className: "nd-search-recent",
            id: `${id}-recent`,
            "aria-labelledby": `${id}-recent-label`,
            "data-nd-search-recent": "",
            hidden: true,
          },
          h("p", { key: "l", className: "nd-search-recent-label", id: `${id}-recent-label` }, recentLabel),
          h("ul", { key: "u", className: "nd-search-recent-list", "data-nd-search-recent-list": "" }),
          forgetLabel
            ? h(
                "button",
                { key: "f", className: "nd-search-forget", type: "button", "data-nd-search-forget": "" },
                forgetLabel,
              )
            : null,
        )
      : null,
    h(
      "ul",
      {
        key: "u",
        className: "nd-search-results",
        id: `${id}-results`,
        "aria-labelledby": `${id}-label`,
        "data-nd-search-results": "",
      },
      children,
    ),
  );

  return h(
    "div",
    {
      className: ["nd-search", className].filter(Boolean).join(" "),
      role: "search",
      "aria-labelledby": `${id}-label`,
      "data-nd-search": name,
      "data-nd-presentation": presentation_,
      "data-nd-size": size_,
      "data-nd-glass": String(glass ?? false),
      "data-nd-filter": String(filter ?? false),
      "data-nd-hotkey": String(hotkey ?? false),
      "data-nd-clearable": String(clearable_),
      "data-nd-loading": String(loading ?? false),
      "data-nd-src": src,
      "data-nd-cap": cap,
      "data-nd-recent": recent,
      "data-nd-status": status,
      "data-nd-empty-label": emptyLabel,
      "data-nd-loading-label": loadingLabel,
      "data-nd-error-label": errorLabel,
      "data-nd-cap-label": capLabel,
      "data-nd-close-label": closeLabel,
      ...rest,
    },
    dialogForm
      ? h(
          "button",
          {
            key: "o",
            className: "nd-search-open",
            type: "button",
            "aria-haspopup": "dialog",
            "data-nd-search-open": "",
            hidden: true,
          },
          h(
            "span",
            { key: "g", className: "nd-search-glyph", "aria-hidden": "true" },
            h(Icon, { icon: search, size: "small" }),
          ),
          h("span", { key: "t", className: "nd-search-open-text" }, label),
          hotkeyHint,
        )
      : null,
    dialogForm
      ? h(
          "dialog",
          {
            key: "d",
            className: "nd-search-dialog",
            id: `${id}-dialog`,
            "aria-labelledby": `${id}-label`,
            "data-nd-search-dialog": "",
          },
          // The control `closeLabel` names. See the Dioxus half: the prop was
          // stamped and read by nothing, so the palette's only exits were
          // Escape and the backdrop.
          h(
            "div",
            { key: "h", className: "nd-search-head" },
            field,
            closeLabel
              ? h(
                  "button",
                  {
                    key: "x",
                    className: "nd-search-close",
                    type: "button",
                    "aria-label": closeLabel,
                    title: closeLabel,
                    "data-nd-search-close": "",
                  },
                  h(Icon, { icon: close, size: "small" }),
                )
              : null,
          ),
          panel,
        )
      : field,
    dialogForm ? null : panel,
  );
"#;

/// The same group, in React.
const SEARCH_GROUP_REACT: &str = r#"  return h(
    "li",
    {
      className: ["nd-search-group", className].filter(Boolean).join(" "),
      "data-nd-search-group": label,
      ...rest,
    },
    h("p", { key: "l", className: "nd-search-group-label" }, label),
    h("ul", { key: "u", className: "nd-search-group-list" }, children),
  );
"#;

/// The same result, in React.
const SEARCH_RESULT_REACT: &str = r#"  const current_ = current ?? false;
  return h(
    "li",
    {
      className: ["nd-search-result", className].filter(Boolean).join(" "),
      "data-nd-keywords": keywords,
      ...rest,
    },
    h(
      "a",
      {
        className: "nd-search-result-link",
        href,
        "aria-current": current_ ? "page" : undefined,
      },
      // The label is its own element because the script rewrites it into
      // `<mark>` runs. See the Dioxus half.
      h("span", { key: "l", className: "nd-search-result-label", "data-nd-search-label": "" }, label),
      badge ? h("span", { key: "b", className: "nd-search-result-badge" }, badge) : null,
      detail ? h("span", { key: "d", className: "nd-search-result-detail" }, detail) : null,
      current_ ? h(Icon, { key: "c", icon: check, size: "small" }) : null,
    ),
  );
"#;

// --- Divider -----------------------------------------------------------------

/// The line between two blocks, and the word on it.
///
/// # Why it is authored rather than derived
///
/// Its element switches, and not from an enum: `label` is text, and giving one
/// turns an `<hr>` into a `<div>`. An `<hr>` is void, so a rule with a word on
/// it is not a rule with a word inside it — it is a different element with the
/// same job. `props.rs` derives a body from one element, so it cannot say that.
///
/// # A separator's children are presentational, so the word is written twice
///
/// ARIA gives `separator` children-presentational, exactly as it does `button`
/// and `img`: whatever is inside is a drawing, and no screen reader reads it.
/// A labelled divider that only put the word inside the element would be a
/// boundary announced as *separator* and nothing else. So the same string is
/// the `aria-label`, from the same prop, at the same render.
///
/// # And `decorative` takes the role with it
///
/// `aria-hidden` removes the element from the tree. A role on something nothing
/// can reach is a claim about an element that is not there, so the role, the
/// orientation and the name are all absent together rather than one by one.
const DIVIDER_DIOXUS: &str = r#"    let root = format!("nd-divider {class}");
    let root = root.trim().to_owned();
    let along = orientation.as_str();
    // The three that go together. A decorative rule is out of the tree, so it
    // carries no role, no orientation and no name — see the header.
    let hidden = decorative.then_some("true");
    let named = (!decorative).then_some("separator");
    let facing = (!decorative).then_some(along);

    rsx! {
        if let Some(label) = label {
            div {
                class: "{root}",
                "data-nd-orientation": along,
                "data-nd-spacing": spacing.as_str(),
                "data-nd-weight": weight.as_str(),
                "data-nd-labelled": "",
                role: named,
                "aria-orientation": facing,
                // THE WORD, AGAIN, BECAUSE THE ONE INSIDE IS A DRAWING.
                "aria-label": (!decorative).then(|| label.clone()),
                "aria-hidden": hidden,
                span { class: "nd-divider-line" }
                span { class: "nd-divider-text", "{label}" }
                span { class: "nd-divider-line" }
            }
        } else {
            hr {
                class: "{root}",
                "data-nd-orientation": along,
                "data-nd-spacing": spacing.as_str(),
                "data-nd-weight": weight.as_str(),
                role: named,
                "aria-orientation": facing,
                "aria-hidden": hidden,
            }
        }
    }
"#;

/// The same line, in React.
const DIVIDER_REACT: &str = r#"  const orientation_ = orientation ?? "horizontal";
  const spacing_ = spacing ?? "default";
  const weight_ = weight ?? "hairline";
  const decorative_ = decorative ?? false;
  const root = ["nd-divider", className].filter(Boolean).join(" ");
  // See the Dioxus half: a decorative rule carries no role, no orientation and
  // no name, because `aria-hidden` has already taken the element out.
  const shared = {
    className: root,
    "data-nd-orientation": orientation_,
    "data-nd-spacing": spacing_,
    "data-nd-weight": weight_,
    role: decorative_ ? undefined : "separator",
    "aria-orientation": decorative_ ? undefined : orientation_,
    "aria-hidden": decorative_ ? "true" : undefined,
    ...rest,
  };

  // GIVEN OR NOT GIVEN, NOT TRUTHY OR FALSY. The Dioxus half branches on
  // `Option`, so `label = Some("")` is a labelled divider there — a `<div
  // role="separator">` with an empty word in it. `!label` made the same call
  // render an `<hr>`, which is a different element with a different role from
  // the same props, and that is exactly the drift `NOCTUA.md §9.3` forbids.
  if (label === undefined || label === null) return h("hr", shared);
  return h(
    "div",
    {
      ...shared,
      "data-nd-labelled": "",
      // The word again: a separator's children are presentational.
      "aria-label": decorative_ ? undefined : label,
    },
    h("span", { key: "a", className: "nd-divider-line" }),
    h("span", { key: "t", className: "nd-divider-text" }, label),
    h("span", { key: "b", className: "nd-divider-line" }),
  );
"#;

// --- Label -------------------------------------------------------------------

/// The name of a control, a group of them, or a region.
///
/// # Why it is authored rather than derived
///
/// `kind` chooses the element, which `props.rs` can do on its own — and then
/// the `for` may only be written on one of the three, the id has to be derived
/// from a prop rather than taken from one, and the marker is a second element
/// beside the text. None of those three is a one-to-one mapping of a prop.
///
/// # One mark, never two
///
/// `required_label` and `optional_label` are the same slot from two directions.
/// A question is one or the other, so the body picks with `required` and draws
/// at most one word. Two words beside one name is two answers to one question.
///
/// # Why the word and not an asterisk
///
/// An asterisk means *required* only to a reader who already knows it does, and
/// it is a glyph a screen reader either skips or reads as *star*. The word is
/// the caller's, in the reader's language, and it is inside the label, so it is
/// read as part of the accessible name rather than beside it.
const LABEL_DIOXUS: &str = r#"    let root = format!("nd-label {class}");
    let root = root.trim().to_owned();
    // DERIVED, NEVER TAKEN. `Field` computed this same string before this
    // component existed, and every control that wraps itself in a field
    // computes it again — one rule, stated in `for_control`'s own
    // documentation, so nothing anywhere has an id to type.
    let id = for_control.as_ref().map(|control| format!("{control}-label"));
    // One mark or the other. See the header.
    let marker = if required { required_label } else { optional_label };

    // Built once, so the three elements below cannot come to draw different
    // things. The name is a bare text node rather than a wrapper, so markup a
    // script builds with this class is the same picture as markup this renders.
    let content = rsx! {
        "{text}"
        if let Some(marker) = marker {
            span { class: "nd-label-mark", "{marker}" }
        }
    };

    match kind {
        // The only attachment a browser acts on, and it acts on it only for a
        // labelable element.
        LabelKind::Label => rsx! {
            label {
                class: "{root}",
                id,
                r#for: for_control,
                "data-nd-size": size.as_str(),
                {content}
            }
        },
        // The one construct that attaches a question to every control in a
        // set, in every engine, with nothing to wire.
        LabelKind::Legend => rsx! {
            legend {
                class: "{root}",
                id,
                "data-nd-size": size.as_str(),
                {content}
            }
        },
        // NOT A `<label>` WITH NOTHING TO POINT AT. A `role="radiogroup"`, a
        // toolbar and a `<nav>` are not labelable, so a `for` aimed at one
        // names nothing whatsoever. They are named by reference, and the id
        // above is what `aria-labelledby` points at.
        LabelKind::Name => rsx! {
            p {
                class: "{root}",
                id,
                "data-nd-size": size.as_str(),
                {content}
            }
        },
    }
"#;

/// The same name, in React.
const LABEL_REACT: &str = r#"  const kind_ = kind ?? "label";
  const size_ = size ?? "medium";
  const required_ = required ?? false;
  const root = ["nd-label", className].filter(Boolean).join(" ");
  // See the Dioxus half: the id is derived and the mark is one or the other.
  const id = forControl ? `${forControl}-label` : undefined;
  const marker = required_ ? requiredLabel : optionalLabel;
  const element = kind_ === "legend" ? "legend" : kind_ === "name" ? "p" : "label";

  return h(
    element,
    {
      className: root,
      id,
      // Only a `<label>` may carry it, and only a labelable element answers it.
      htmlFor: kind_ === "label" ? forControl : undefined,
      "data-nd-size": size_,
      ...rest,
    },
    text,
    marker ? h("span", { key: "m", className: "nd-label-mark" }, marker) : null,
  );
"#;

// --- Image -------------------------------------------------------------------

/// A picture, in a box that was there before it was.
///
/// # Why it is authored rather than derived
///
/// A `caption` turns an `<img>` into a `<figure>` holding one, which is a
/// second element chosen by a text prop. And the two arrangements have to be
/// styled by one set of rules, so the shape rides the ROOT in both and the
/// picture reads it from there.
///
/// # The three `Avatar` established, kept
///
/// `loading="lazy"`, `decoding="async"`, and a box sized by the caller rather
/// than by the file. The last is the one that matters: an image whose height
/// arrives with its bytes moves everything under it, and a reader who was
/// about to press something presses something else.
///
/// # There is no `<picture>` and no `srcset`
///
/// Neither exists anywhere in this repository. Adding one is a change to both
/// languages at once or it is drift, so it waits for a caller who needs it.
const IMAGE_DIOXUS: &str = r#"    let root = format!("nd-image {class}");
    let root = root.trim().to_owned();
    // With no caption the picture IS the root, so it carries both classes. The
    // shape is read from `.nd-image` either way, which is what lets one rule
    // serve the bare picture and the captioned one.
    let alone = format!("{root} nd-image-media");
    let shape = ratio.map(ImageRatio::as_str);

    rsx! {
        if let Some(caption) = caption {
            figure {
                class: "{root}",
                "data-nd-ratio": shape,
                "data-nd-fit": fit.as_str(),
                "data-nd-rounded": if rounded { "true" } else { "false" },
                img {
                    class: "nd-image-media",
                    src: "{src}",
                    // NEVER DERIVED, AND AN EMPTY ONE IS AN ANSWER. The prop
                    // is required, so a picture with no words is not a thing a
                    // caller can construct by forgetting.
                    alt: "{alt}",
                    width,
                    height,
                    loading: loading.as_str(),
                    decoding: "async",
                }
                // A `<figcaption>` rather than a paragraph under the picture:
                // the words belong to the figure rather than to the flow.
                figcaption { class: "nd-image-caption", "{caption}" }
            }
        } else {
            img {
                class: "{alone}",
                "data-nd-ratio": shape,
                "data-nd-fit": fit.as_str(),
                "data-nd-rounded": if rounded { "true" } else { "false" },
                src: "{src}",
                alt: "{alt}",
                width,
                height,
                loading: loading.as_str(),
                decoding: "async",
            }
        }
    }
"#;

/// The same picture, in React.
const IMAGE_REACT: &str = r#"  const fit_ = fit ?? "cover";
  const loading_ = loading ?? "lazy";
  const rounded_ = rounded ?? false;
  const root = ["nd-image", className].filter(Boolean).join(" ");
  // See the Dioxus half: the shape rides the root, whichever element that is.
  const shape = {
    "data-nd-ratio": ratio,
    "data-nd-fit": fit_,
    "data-nd-rounded": String(rounded_),
  };
  const picture = { src, alt, width, height, loading: loading_, decoding: "async" };

  if (!caption)
    return h("img", { className: `${root} nd-image-media`, ...shape, ...picture, ...rest });
  return h(
    "figure",
    { className: root, ...shape, ...rest },
    h("img", { key: "p", className: "nd-image-media", ...picture }),
    h("figcaption", { key: "c", className: "nd-image-caption" }, caption),
  );
"#;

// --- MessageBar --------------------------------------------------------------

/// One message about something that happened, left where it happened.
///
/// # Why `role="status"` is the default and `alert` is a prop
///
/// An alert cuts a screen reader off mid-sentence. That is right about a
/// payment that failed while a reader was three fields further down, and rude
/// about a record that saved. A component whose default is the rude one gets
/// the rude one everywhere, because a default is what a caller writes when
/// they have not thought about it.
///
/// # Why `aria-live` is absent unless a caller writes it
///
/// `status` already implies polite and `alert` already implies assertive, so
/// writing the attribute at a default would be this component overruling the
/// role it just chose — `role="alert" aria-live="polite"` is an alert that
/// never interrupts. Absent means the role decides, which is the platform's
/// own answer, and `off` stays available for a bar that shipped with the page.
///
/// # And the tone is never the signal
///
/// `data-nd-tone` paints from `dist/tones.css` and nothing else depends on it.
/// The words, the title and the icon are what carry the meaning (`§8.8.6`),
/// which is why `icon` is the caller's choice rather than a map from the tone
/// name: this system has 351 tones and no opinion about what most of them mean.
///
/// # And the icon is a drawing, not a word
///
/// It is an `IconDrawing`, as `Input`'s and `Tab`'s are. It was a `String` run
/// through `IconDrawing::parse` here and handed straight to `Icon` in React — so a
/// name nothing declares rendered *nothing* in Dioxus and *threw* in React,
/// which is the one behaviour `NOCTUA.md §9.3` says the two may not have.
/// Typed, neither language can be given a name that is not an icon.
const MESSAGE_BAR_DIOXUS: &str = r#"    let root = format!("nd-message-bar {class}");
    let root = root.trim().to_owned();

    rsx! {
        div {
            class: "{root}",
            // A STATUS UNTIL A CALLER ASKS FOR THE RUDE ONE. See the header.
            role: if alert { "alert" } else { "status" },
            // Written only when the caller said something. An absent one is
            // the role's own value, which is the right one both times.
            "aria-live": live.map(MessageBarLive::as_str),
            "data-nd-tone": tone,
            "data-nd-dismissible": if dismissible { "true" } else { "false" },

            if let Some(icon) = icon {
                // `aria-hidden`, because it says what the words say. It is a
                // second signal for a reader who can see it, never the first.
                span { class: "nd-message-bar-icon", "aria-hidden": "true",
                    Icon { icon, size: IconSize::Small }
                }
            }

            div { class: "nd-message-bar-text",
                // A `<strong>` and not a heading: a message that arrives after
                // the page has loaded must not rewrite the outline under a
                // reader who is navigating by it.
                if let Some(title) = title {
                    strong { class: "nd-message-bar-title", "{title}" }
                }
                div { class: "nd-message-bar-body", {children} }
            }

            if dismissible {
                // RENDERED HIDDEN, revealed by `dist/message-bar.js` — `Info`'s
                // arrangement for `Info`'s reason. With no script there is
                // nothing to take the bar away, and a control that closes
                // nothing is worse than no control at all.
                button {
                    class: "nd-message-bar-dismiss",
                    r#type: "button",
                    "aria-label": dismiss_label,
                    "data-nd-dismiss": "",
                    hidden: true,
                    Icon { icon: IconDrawing::CLOSE, size: IconSize::Small }
                }
            }
        }
    }
"#;

/// The same message, in React.
const MESSAGE_BAR_REACT: &str = r#"  const alert_ = alert ?? false;
  const dismissible_ = dismissible ?? false;
  const root = ["nd-message-bar", className].filter(Boolean).join(" ");

  return h(
    "div",
    {
      className: root,
      // See the Dioxus half: a status until a caller asks for the rude one,
      // and no `aria-live` at all unless they said something.
      role: alert_ ? "alert" : "status",
      "aria-live": live,
      "data-nd-tone": tone,
      "data-nd-dismissible": String(dismissible_),
      ...rest,
    },
    icon
      ? h(
          "span",
          { key: "i", className: "nd-message-bar-icon", "aria-hidden": "true" },
          h(Icon, { icon, size: "small" }),
        )
      : null,
    h(
      "div",
      { key: "t", className: "nd-message-bar-text" },
      title ? h("strong", { key: "h", className: "nd-message-bar-title" }, title) : null,
      h("div", { key: "b", className: "nd-message-bar-body" }, children),
    ),
    dismissible_
      ? h(
          "button",
          {
            key: "x",
            className: "nd-message-bar-dismiss",
            type: "button",
            "aria-label": dismissLabel,
            "data-nd-dismiss": "",
            // See the Dioxus half: revealed by the script.
            hidden: true,
          },
          h(Icon, { icon: close, size: "small" }),
        )
      : null,
  );
"#;

// --- Icon ---------------------------------------------------------------------

/// One vendored drawing, in a box the text around it decides.
///
/// # Why it is authored rather than derived
///
/// Nothing about it is a one-to-one mapping of a prop. The geometry, the view
/// box and the paint are read from `IconDrawing`; `label` decides three attributes
/// at once and the absence of it decides two others; and the drawing goes in as
/// inner HTML rather than as children, because it is a string in the generated
/// module rather than markup a caller wrote.
///
/// # Why this component was MOVED here rather than added
///
/// It shipped from `crates/noctua-design-emit/src/icons.rs`, which put it
/// outside the catalogue: no props table, no variant grid, and absent from
/// search-by-prop on a site whose search is by prop. Writing a contract beside
/// that emitter would have produced two components called `Icon` in one crate,
/// with `lib.rs`'s explicit re-export silently choosing one. So the emitter
/// stopped writing the two component files and the body came here, where every
/// other authored body already is.
///
/// # `aria-hidden` by default, and the two states are exclusive
///
/// An icon is decoration until a caller names it. Unlabelled it carries
/// `aria-hidden="true"` and `focusable="false"`, which is right beside a word
/// that already says it. Labelled it is a `role="img"` with that name and no
/// `aria-hidden`, because an element cannot be both hidden and named.
const ICON_DIOXUS: &str = r#"    let root = format!("nd-icon-glyph {class}");
    let root = root.trim().to_owned();
    let named = label.is_some();

    rsx! {
        svg {
            class: "{root}",
            "data-nd-icon": icon.name,
            "data-nd-size": size.as_str(),
            view_box: icon.view_box,
            // `1em`, so the box comes from whatever contains it and one class
            // covers every context. `data-nd-size` sets that `em`.
            width: "1em",
            height: "1em",
            fill: if icon.filled { "currentColor" } else { "none" },
            stroke: if icon.filled { "none" } else { "currentColor" },
            // FROM THE SPEC, NOT FROM HERE. `spec/icons.toml` decides the
            // weight, and three places draw it: this body, the React one, and
            // the documentation page's variant grid.
            stroke_width: STROKE_WIDTH,
            stroke_linecap: "round",
            stroke_linejoin: "round",
            // Quoted, not identifiers: `dioxus_elements::svg` has no typed
            // `role`, `aria-*` or `focusable`, and an `Option` renders nothing
            // when it is `None`. That is how an unlabelled icon carries
            // `aria-hidden` and a labelled one carries a name instead.
            "role": named.then_some("img"),
            "aria-label": label,
            "aria-hidden": (!named).then_some("true"),
            "focusable": "false",
            dangerous_inner_html: "{icon.body}",
        }
    }
"#;

/// The same drawing, in React.
const ICON_REACT: &str = r#"  const size_ = size ?? "medium";
  const root = ["nd-icon-glyph", className].filter(Boolean).join(" ");
  // THE DRAWING ARRIVES AS A VALUE, NOT AS A NAME.
  //
  // This read `ICONS[icon]` out of a table of every drawing in the system,
  // which is what made one icon cost all of them. The caller imports the one
  // it wants and hands it over, so the only drawing in a bundle is the one
  // somebody imported. See the Dioxus half, where the same value is a `const`.
  if (!icon || !icon.body) throw new Error("Icon needs a drawing, imported from @noctua-world/design-tokens/icons/<name>.js");
  const named = label !== undefined && label !== null;

  return h("svg", {
    className: root,
    "data-nd-icon": icon.name,
    "data-nd-size": size_,
    viewBox: icon.viewBox,
    // See the Dioxus half: `1em`, and `data-nd-size` sets that `em`.
    width: "1em",
    height: "1em",
    fill: icon.filled ? "currentColor" : "none",
    stroke: icon.filled ? "none" : "currentColor",
    // See the Dioxus half: the weight is `spec/icons.toml`'s.
    strokeWidth: ICON_STROKE_WIDTH,
    strokeLinecap: "round",
    strokeLinejoin: "round",
    role: named ? "img" : undefined,
    "aria-label": named ? label : undefined,
    "aria-hidden": named ? undefined : "true",
    focusable: "false",
    dangerouslySetInnerHTML: { __html: icon.body },
    ...rest,
  });
"#;

// --- Rating ---------------------------------------------------------------------

/// A score out of a small whole number, read or given.
///
/// # Two shapes, and the prop that chooses between them
///
/// `readonly` is a reading: words, a row of stars, and the number written out.
/// Without it the control is a real `<fieldset>` of real radios, which is what
/// makes it work and submit with no script at all. One component rather than
/// two, because a caller switching a rating from editable to read-only is
/// changing one fact about it.
///
/// # Why the number is drawn beside the stars, always
///
/// `Meter` and `Progress` both record the defect this avoids: a shape a reader
/// has to estimate from is a value they were not given. Four and a bit stars is
/// 4.2 or 4.4 and nobody counting pixels can say which.
///
/// In the interactive form the sentence is filled ONCE PER STEP and every copy
/// is hidden but one. `input:checked + label + .nd-rating-count` is the whole
/// mechanism, so the number follows a reader's own choice with no script
/// running. The three elements are therefore SIBLINGS, in that order, in both
/// languages.
///
/// # A fraction is clipped, never rounded
///
/// The read-only picture is two copies of the same row, the second clipped to
/// `value / max`. There is no half-star drawing and there is no rounding, so
/// `4.3` draws four whole stars and three tenths of the fifth.
const RATING_DIOXUS: &str = r#"    let root = format!("nd-rating {class}");
    let root = root.trim().to_owned();
    // Derived, so the labels' targets cannot be typed wrong.
    let id = format!("nd-rating-{name}");
    let invalid = error.is_some().then_some("true");
    // A scale of no steps draws nothing, and one of forty is a choice nobody
    // can make at a glance. The clamp is the same judgement `max`'s doc states.
    let steps: u32 = max.parse().unwrap_or(5).clamp(1, 20);
    let reading: f64 = value
        .as_deref()
        .and_then(|one| one.parse().ok())
        .unwrap_or(0.0);
    // HOW MUCH OF THE ROW IS INK, not how many stars are whole. One decimal
    // place, because a reading to a tenth is what a fraction is for. A `u32`
    // rather than a `usize`, so the division needs no lossy cast.
    let share = (reading / f64::from(steps)).clamp(0.0, 1.0);
    let level = format!("{:.1}%", share * 100.0);
    // ONE SENTENCE WITH HOLES IN IT, filled here. Two words joined together
    // would be a sentence no translator can reorder.
    let filled = |words: &str, number: &str| {
        words.replace("{value}", number).replace("{max}", &max)
    };
    let sentence = value
        .as_deref()
        .zip(value_label.as_deref())
        .map(|(number, words)| filled(words, number));
    // Every step as values, so the loop below computes nothing. The count is
    // the same sentence at this step, which is what the stylesheet reveals
    // when this step is the one checked.
    let scale: Vec<(String, String, String, bool)> = (1..=steps)
        .map(|step| {
            let number = step.to_string();
            let named = item_label.as_deref().map(|words| filled(words, &number));
            let count = value_label.as_deref().map(|words| filled(words, &number));
            let chosen = value.as_deref() == Some(number.as_str());
            (
                number,
                named.unwrap_or_default(),
                count.unwrap_or_default(),
                chosen,
            )
        })
        .collect();

    rsx! {
        if readonly {
            div {
                class: "{root}",
                "data-nd-rating": "{name}",
                "data-nd-size": size.as_str(),
                "data-nd-tone": tone,
                "data-nd-readonly": "true",
                "data-nd-max": "{max}",
                "data-nd-value": value,
                span { class: "nd-rating-name", "{label}" }
                // THE PICTURE, AND ONLY THE PICTURE. Two copies of one row,
                // the second clipped to `share`. `aria-hidden`, because the
                // sentence after it says what it draws.
                span { class: "nd-rating-stars", "aria-hidden": "true",
                    span { class: "nd-rating-track",
                        for _ in 1..=steps {
                            Icon { icon: IconDrawing::STAR }
                        }
                    }
                    span {
                        class: "nd-rating-level",
                        "data-nd-level": "{level}",
                        style: "--nd-rating-share: {level}",
                        for _ in 1..=steps {
                            Icon { icon: IconDrawing::STAR }
                        }
                    }
                }
                if let Some(sentence) = sentence {
                    span { class: "nd-rating-value", "{sentence}" }
                }
            }
        } else {
            Field {
                control: "{id}-1",
                label: label.clone(),
                hint: hint.clone(),
                error: error.clone(),
                kind: FieldKind::Group,
                required,
                required_label: required_label.clone(),
                size: match size {
                    RatingSize::Small => FieldSize::Small,
                    RatingSize::Medium => FieldSize::Medium,
                    RatingSize::Large => FieldSize::Large,
                },
                disabled,

                div {
                    class: "{root}",
                    "data-nd-rating": "{name}",
                    "data-nd-size": size.as_str(),
                    "data-nd-tone": tone,
                    "data-nd-readonly": "false",
                    "data-nd-max": "{max}",
                    "data-nd-value": value,
                
                    div { class: "nd-rating-row",
                        for (number, named, count, chosen) in scale {
                            // THREE SIBLINGS PER STEP, AND THE ORDER IS
                            // LOAD-BEARING: the stylesheet reads
                            // `input:checked + label + .nd-rating-count` and
                            // nothing else puts the chosen number on the
                            // screen without a script.
                            input {
                                class: "nd-rating-input nd-visually-hidden",
                                id: "{id}-{number}",
                                r#type: "radio",
                                name: "{name}",
                                value: "{number}",
                                checked: chosen,
                                required,
                                disabled,
                                "aria-invalid": invalid,
                            }
                            label {
                                class: "nd-rating-star",
                                r#for: "{id}-{number}",
                                // A star is a drawing, so this is the only
                                // name this option has.
                                span { class: "nd-visually-hidden", "{named}" }
                                Icon { icon: IconDrawing::STAR }
                            }
                            span { class: "nd-rating-count", "{count}" }
                        }
                    }
                }
            }
        }
    }
"#;

/// The same score, in React.
const RATING_REACT: &str = r#"  const size_ = size ?? "medium";
  const max_ = max ?? "5";
  const readonly_ = readonly ?? false;
  const root = ["nd-rating", className].filter(Boolean).join(" ");
  const id = `nd-rating-${name}`;
  // See the Dioxus half: the clamp is the judgement `max`'s doc states.
  const steps = Math.min(20, Math.max(1, Number.parseInt(max_, 10) || 5));
  const reading = Number.parseFloat(value ?? "") || 0;
  const share = Math.min(1, Math.max(0, reading / steps));
  const level = `${(share * 100).toFixed(1)}%`;
  // See the Dioxus half: one sentence with holes, filled here.
  const filled = (words, number) =>
    words.split("{value}").join(number).split("{max}").join(max_);
  // GIVEN OR NOT GIVEN, NOT TRUTHY OR FALSY. The Dioxus half zips two
  // `Option`s, so `value = Some("")` reads the sentence out with an empty
  // number in it; `value && valueLabel` dropped the sentence entirely, and a
  // reading a sighted reader can see is one a screen reader was not given.
  const given = value !== undefined && value !== null;
  const sentence =
    given && valueLabel !== undefined && valueLabel !== null
      ? filled(valueLabel, value)
      : undefined;
  const scale = Array.from({ length: steps }, (_, at) => {
    const number = String(at + 1);
    return {
      number,
      named: itemLabel ? filled(itemLabel, number) : "",
      count: valueLabel ? filled(valueLabel, number) : "",
      chosen: value === number,
    };
  });
  const stars = (key) =>
    Array.from({ length: steps }, (_, at) => h(Icon, { key: `${key}${at}`, icon: star }));

  if (readonly_)
    return h(
      "div",
      {
        className: root,
        "data-nd-rating": name,
        "data-nd-size": size_,
        "data-nd-tone": tone,
        "data-nd-readonly": "true",
        "data-nd-max": max_,
        "data-nd-value": value,
        ...rest,
      },
      h("span", { key: "n", className: "nd-rating-name" }, label),
      // See the Dioxus half: two copies of one row, the second clipped.
      h(
        "span",
        { key: "s", className: "nd-rating-stars", "aria-hidden": "true" },
        h("span", { key: "t", className: "nd-rating-track" }, stars("t")),
        h(
          "span",
          {
            key: "l",
            className: "nd-rating-level",
            "data-nd-level": level,
            style: { "--nd-rating-share": level },
          },
          stars("l"),
        ),
      ),
      sentence === undefined
        ? null
        : h("span", { key: "v", className: "nd-rating-value" }, sentence),
    );

  return h(
    Field,
    {
      control: `${id}-1`,
      label,
      hint,
      error,
      kind: "group",
      required,
      requiredLabel,
      size: size_,
      disabled,
    },
    h(
      "div",
      {
        key: "r",
        className: root,
        "data-nd-rating": name,
        "data-nd-size": size_,
        "data-nd-tone": tone,
        "data-nd-readonly": "false",
        "data-nd-max": max_,
        "data-nd-value": value,
            ...rest,
      },
      h(
        "div",
        { key: "w", className: "nd-rating-row" },
        // A fragment, so the three stay SIBLINGS: see the Dioxus half, where
        // the stylesheet reads `input:checked + label + .nd-rating-count`.
        scale.map((step) =>
          h(
            Fragment,
            { key: step.number },
            h("input", {
              className: "nd-rating-input nd-visually-hidden",
              id: `${id}-${step.number}`,
              type: "radio",
              name,
              value: step.number,
              defaultChecked: step.chosen,
              required,
              disabled,
              "aria-invalid": error ? "true" : undefined,
            }),
            h(
              "label",
              { className: "nd-rating-star", htmlFor: `${id}-${step.number}` },
              h("span", { key: "n", className: "nd-visually-hidden" }, step.named),
              h(Icon, { key: "s", icon: star }),
            ),
            h("span", { className: "nd-rating-count" }, step.count),
          ),
        ),
      ),
    ),
  );
"#;

// --- OtpField -------------------------------------------------------------------

/// One short code, in a row of boxes.
///
/// # What it claims with no script
///
/// A `<fieldset>` with a legend, `length` labelled boxes of one character each,
/// and the browser's own `required` refusal. Every box posts under the SAME
/// name, so the code arrives as that many values in document order for a server
/// to join. A hidden field carrying the joined code would post nothing at all
/// when the script does not run, and one name per box would make a server take
/// apart something it never asked to be split.
///
/// # The walk is `TimeInput`'s
///
/// Auto-advance, Backspace in an empty box going back, and a paste splitting
/// across the row are the same three gestures a time input needs, so
/// `segmentWalk` in `crates/noctua-design-runtime/src/inputs.js` is the one
/// implementation and both controls call it.
///
/// # `autocomplete` on the first box only
///
/// `one-time-code` tells the platform *this field is the code*. Written on
/// every box it tells it that six times, and the platform offers the whole code
/// to each of them.
const OTP_FIELD_DIOXUS: &str = r#"    let root = format!("nd-otp-field {class}");
    let root = root.trim().to_owned();
    // Derived, so the labels' targets cannot be typed wrong. The hint and the
    // error ids are `Field`'s, and so is the `aria-describedby` that points at
    // them: a `<fieldset>` describes what is inside it.
    let id = format!("nd-otp-field-{name}");
    let flagged = if error.is_some() { "true" } else { "false" };
    let invalid = error.is_some().then_some("true");
    // A row of no boxes draws nothing, and a very long one is a code nobody was
    // sent. Six is the default and the clamp is the same judgement.
    let boxes: usize = length.parse().unwrap_or(6).clamp(1, 12);
    let numeric = kind == OtpFieldKind::Numeric;
    // ONE VALUE, SPLIT. The code is one string and the boxes are a picture of
    // it, so this is a split rather than an encoding.
    let given: Vec<String> = value
        .as_deref()
        .unwrap_or_default()
        .chars()
        .map(|one| one.to_string())
        .collect();
    // Every box as values, so the loop below computes nothing.
    let steps: Vec<(usize, String, String, Option<String>)> = (1..=boxes)
        .map(|index| {
            let named = box_label
                .replace("{index}", &index.to_string())
                .replace("{length}", &boxes.to_string());
            (
                index,
                format!("{id}-{index}"),
                named,
                given.get(index - 1).cloned(),
            )
        })
        .collect();

    rsx! {
        Field {
            control: "{id}-1",
            label: legend.clone(),
            hint: hint.clone(),
            error: error.clone(),
            // The paragraph a client-side refusal has somewhere to go into.
            // `Select` invented it and `DateInput` arrived at it second.
            error_slot: true,
            kind: FieldKind::Group,
            required,
            required_label: required_label.clone(),
            size: match size {
                OtpFieldSize::Small => FieldSize::Small,
                OtpFieldSize::Medium => FieldSize::Medium,
                OtpFieldSize::Large => FieldSize::Large,
            },
            disabled,

            div {
                class: "{root}",
                "data-nd-otp-field": "{name}",
                "data-nd-size": size.as_str(),
                "data-nd-kind": kind.as_str(),
                "data-nd-length": "{boxes}",
                "data-nd-value": value,
                "data-nd-invalid": flagged,
                "data-nd-autocomplete": if autocomplete { "true" } else { "false" },
                "data-nd-box-label": box_label,
                "data-nd-invalid-label": invalid_label,

                div { class: "nd-otp-field-row", "data-nd-otp-field-row": "",
                    for (index, box_id, box_name, filled) in steps {
                        span { class: "nd-otp-field-box", key: "{index}",
                            // HIDDEN, NOT ABSENT. The legend names the group
                            // and this names the box, so a reader who lands on
                            // the fourth one is told which it is.
                            label {
                                class: "nd-otp-field-label nd-visually-hidden",
                                r#for: "{box_id}",
                                "{box_name}"
                            }
                            input {
                                class: "nd-otp-field-input",
                                id: "{box_id}",
                                r#type: "text",
                                // EVERY BOX POSTS UNDER ONE NAME. See the
                                // header: the code is one value.
                                name: "{name}",
                                inputmode: if numeric { "numeric" } else { "text" },
                                // THE FIRST BOX ONLY. See the header.
                                autocomplete: (autocomplete && index == 1).then_some("one-time-code"),
                                maxlength: "1",
                                autocorrect: "off",
                                autocapitalize: "characters",
                                spellcheck: "false",
                                value: filled,
                                required,
                                disabled,
                                "aria-invalid": invalid,
                                "data-nd-otp-field-segment": "{index}",
                            }
                        }
                    }
                }
                // One sentence at a time: what is missing, and what the boxes
                // do not accept. Said on leaving rather than between two boxes.
                p {
                    class: "nd-otp-field-note",
                    id: "{id}-status",
                    role: "status",
                    "aria-live": "polite",
                    "aria-atomic": "true",
                    "data-nd-otp-field-status": "",
                }
            }
        }
    }
"#;

/// The same code, in React.
const OTP_FIELD_REACT: &str = r#"  const size_ = size ?? "medium";
  const kind_ = kind ?? "numeric";
  const length_ = length ?? "6";
  const autocomplete_ = autocomplete ?? true;
  const root = ["nd-otp-field", className].filter(Boolean).join(" ");
  const id = `nd-otp-field-${name}`;
  // See the Dioxus half: six is the default and the clamp is the judgement.
  const boxes = Math.min(12, Math.max(1, Number.parseInt(length_, 10) || 6));
  const numeric = kind_ === "numeric";
  // See the Dioxus half: one value, split.
  const given = [...(value ?? "")];
  const steps = Array.from({ length: boxes }, (_, at) => ({
    index: at + 1,
    id: `${id}-${at + 1}`,
    named: boxLabel
      .split("{index}").join(String(at + 1))
      .split("{length}").join(String(boxes)),
    filled: given[at],
  }));

  return h(
    Field,
    {
      control: `${id}-1`,
      label: legend,
      hint,
      error,
      errorSlot: true,
      kind: "group",
      required,
      requiredLabel,
      size: size_,
      disabled,
    },
    h(
      "div",
      {
        key: "o",
        className: root,
        "data-nd-otp-field": name,
        "data-nd-size": size_,
        "data-nd-kind": kind_,
        "data-nd-length": String(boxes),
        "data-nd-value": value,
        "data-nd-invalid": String(Boolean(error)),
        "data-nd-autocomplete": String(autocomplete_),
        "data-nd-box-label": boxLabel,
        "data-nd-invalid-label": invalidLabel,
        ...rest,
      },
      h(
        "div",
        { key: "r", className: "nd-otp-field-row", "data-nd-otp-field-row": "" },
        steps.map((step) =>
          h(
            "span",
            { key: step.index, className: "nd-otp-field-box" },
            h(
              "label",
              { key: "l", className: "nd-otp-field-label nd-visually-hidden", htmlFor: step.id },
              step.named,
            ),
            h("input", {
              key: "i",
              className: "nd-otp-field-input",
              id: step.id,
              type: "text",
              // See the Dioxus half: one name, and the first box only.
              name,
              inputMode: numeric ? "numeric" : "text",
              autoComplete: autocomplete_ && step.index === 1 ? "one-time-code" : undefined,
              maxLength: 1,
              autoCorrect: "off",
              autoCapitalize: "characters",
              spellCheck: "false",
              defaultValue: step.filled,
              required,
              disabled,
              "aria-invalid": error ? "true" : undefined,
              "data-nd-otp-field-segment": String(step.index),
            }),
          ),
        ),
      ),
      h("p", {
        key: "n",
        className: "nd-otp-field-note",
        id: `${id}-status`,
        role: "status",
        "aria-live": "polite",
        "aria-atomic": "true",
        "data-nd-otp-field-status": "",
      }),
    ),
  );
"#;

// --- Tabs, Tab and TabPanel -----------------------------------------------------

/// The strip, and it claims no role until a script can keep one.
///
/// # What is rendered and what the script writes
///
/// `Tree`'s rule: nothing claims a role it cannot honour. With no script there
/// is no `role="tablist"`, no `role="tab"` and no `role="tabpanel"`, every panel
/// is on the page, and each tab is a link down to the panel it names. That is a
/// working table of contents rather than a row of controls that do nothing.
///
/// The WIRING is rendered, because it costs nothing and cannot go stale: both
/// ids come from `name` and `value`, `aria-controls` points down and
/// `aria-labelledby` points back. `docs-site/assets/shell.js` adds the roles,
/// the selection and the hiding, and walks the strip with the same `stops()`
/// and `roving()` a toolbar and a radiogroup already use.
///
/// # Why this is not a `Segmented` prop
///
/// `Segmented` chooses a value and submits it. This reveals a panel and submits
/// nothing. A screen reader is told which of the two it is looking at, and
/// `segmented.toml` already refused `role="tablist"` on the grounds that it
/// promises panels that change.
const TABS_DIOXUS: &str = r#"    let root = format!("nd-tabs {class}");
    let root = root.trim().to_owned();

    rsx! {
        div {
            class: "{root}",
            // NO `role="tablist"` HERE. See the header: the role arrives with
            // the script that can honour it.
            "data-nd-tabs": "{name}",
            "data-nd-activation": activation.as_str(),
            "data-nd-appearance": appearance.as_str(),
            "data-nd-size": size.as_str(),
            "data-nd-fill": fill.as_str(),
            // Announced as well as drawn, and it decides which arrows move.
            "aria-orientation": orientation.as_str(),
            // One or the other names the strip once it has a role.
            "aria-label": label,
            "aria-labelledby": labelledby,
            {children}
        }
    }
"#;

/// The same strip, in React.
const TABS_REACT: &str = r#"  const activation_ = activation ?? "automatic";
  const appearance_ = appearance ?? "underline";
  const size_ = size ?? "medium";
  const fill_ = fill ?? "hug";
  const orientation_ = orientation ?? "horizontal";
  const root = ["nd-tabs", className].filter(Boolean).join(" ");

  return h(
    "div",
    {
      className: root,
      // See the Dioxus half: no role until a script can keep it.
      "data-nd-tabs": name,
      "data-nd-activation": activation_,
      "data-nd-appearance": appearance_,
      "data-nd-size": size_,
      "data-nd-fill": fill_,
      "aria-orientation": orientation_,
      "aria-label": label,
      "aria-labelledby": labelledby,
      ...rest,
    },
    children,
  );
"#;

/// One tab, which is a link until a script makes it a tab.
///
/// A `<button>` with no script does nothing at all, and a control that does
/// nothing is worse than no control. Every panel is visible without a script,
/// so a link reaches the one this names. `Tree` renders links and gains
/// `role="treeitem"` the same way.
const TAB_DIOXUS: &str = r##"    let root = format!("nd-tab {class}");
    let root = root.trim().to_owned();
    // DERIVED, BOTH WAYS, from the two words the panel also derives from. There
    // is nothing to type, so there is nothing to keep in step.
    let id = format!("nd-tabs-{name}-{value}");
    let panel = format!("{id}-panel");
    // A refused tab keeps its place and loses its destination: `aria-disabled`
    // announces it, and an anchor with no `href` is not followed.
    let href = (!disabled).then(|| format!("#{panel}"));

    rsx! {
        a {
            class: "{root}",
            id: "{id}",
            href,
            // NO `role="tab"` AND NO `aria-selected`. Both arrive with the
            // script, together with the panel it can hide.
            "aria-controls": "{panel}",
            "data-nd-value": "{value}",
            "data-nd-selected": if selected { "true" } else { "false" },
            "aria-disabled": if disabled { "true" } else { "false" },
            if let Some(icon) = icon {
                Icon { icon, size: IconSize::Small }
            }
            span { class: "nd-tab-text", "{label}" }
            // INSIDE THE TAB, so it is read as part of the name. A count beside
            // a tab rather than in it is a number a screen reader never reaches.
            if let Some(count) = count {
                span { class: "nd-tab-count", "{count}" }
            }
        }
    }
"##;

/// The same tab, in React.
const TAB_REACT: &str = r#"  const selected_ = selected ?? false;
  const disabled_ = disabled ?? false;
  const root = ["nd-tab", className].filter(Boolean).join(" ");
  // See the Dioxus half: derived both ways, from the same two words.
  const id = `nd-tabs-${name}-${value}`;
  const panel = `${id}-panel`;

  return h(
    "a",
    {
      className: root,
      id,
      href: disabled_ ? undefined : `#${panel}`,
      // See the Dioxus half: no role and no `aria-selected` until the script.
      "aria-controls": panel,
      "data-nd-value": value,
      "data-nd-selected": String(selected_),
      "aria-disabled": String(disabled_),
      ...rest,
    },
    icon ? h(Icon, { key: "i", icon, size: "small" }) : null,
    h("span", { key: "t", className: "nd-tab-text" }, label),
    // See the Dioxus half: inside the tab, so it is part of the name.
    count ? h("span", { key: "c", className: "nd-tab-count" }, count) : null,
  );
"#;

/// One panel, visible until a script has a reason to hide it.
///
/// It sits AFTER the strip rather than inside it, because a `role="tablist"`
/// may hold tabs and nothing else. A Dioxus component cannot look inside its own
/// children either, so a `Tabs` taking both could not have separated them.
const TAB_PANEL_DIOXUS: &str = r#"    let root = format!("nd-tab-panel {class}");
    let root = root.trim().to_owned();
    // The same two words the tab derived its own ids from.
    let id = format!("nd-tabs-{name}-{value}-panel");
    let tab = format!("nd-tabs-{name}-{value}");

    rsx! {
        div {
            class: "{root}",
            id: "{id}",
            // NO `role="tabpanel"` AND NO `hidden`. With no script every panel
            // is on the page, which is what makes the strip above it a table of
            // contents rather than a row of controls that do nothing.
            "aria-labelledby": "{tab}",
            "data-nd-value": "{value}",
            {children}
        }
    }
"#;

/// The same panel, in React.
const TAB_PANEL_REACT: &str = r#"  const root = ["nd-tab-panel", className].filter(Boolean).join(" ");
  // See the Dioxus half: the same two words the tab derived its ids from.
  const id = `nd-tabs-${name}-${value}-panel`;

  return h(
    "div",
    {
      className: root,
      id,
      // See the Dioxus half: no role and nothing hidden until the script.
      "aria-labelledby": `nd-tabs-${name}-${value}`,
      "data-nd-value": value,
      ...rest,
    },
    children,
  );
"#;

// --- Dialog -------------------------------------------------------------------

/// The platform's `<dialog>`, and the control that opens it.
///
/// # Why the trigger is part of the component
///
/// A `<dialog>` opens from `showModal()` or `show()` and from nothing else, so
/// shipping the surface alone hands every caller the same wiring and every copy
/// is a place the wrong element is opened. The control ships hidden and
/// `dist/dialog.js` reveals it, which is `Info`'s arrangement: a page whose
/// script never ran carries no button that opens nothing.
///
/// # Why the layout lives on `[open]` in the stylesheet
///
/// A closed `<dialog>` is `display: none` by the user-agent stylesheet, and
/// that declaration is the only thing keeping it off the page.
/// `Search { presentation: dialog }` paid for that lesson here: `display: flex`
/// on the class un-hid every closed dialog in the document.
///
/// # And the name comes from the visible title when there is one
///
/// `heading` renders a title inside the surface and names it through
/// `aria-labelledby`. `label` is written only when there is no heading, because
/// a name a reader can see and a name only a screen reader hears will drift the
/// moment either is edited.
const DIALOG_DIOXUS: &str = r#"    let root = format!("nd-dialog {class}");
    let root = root.trim().to_owned();
    let id = format!("nd-dialog-{name}");
    let surface_id = format!("{id}-surface");
    let title_id = format!("{id}-title");
    let note_id = format!("{id}-note");
    let titled = heading.as_ref().map(|_| title_id.clone());
    let named = if heading.is_some() { None } else { label.clone() };
    let described = description.as_ref().map(|_| note_id.clone());
    let head = heading.is_some() || dismissible;

    rsx! {
        div {
            class: "{root}",
            "data-nd-dialog": "{name}",
            "data-nd-modal": if modal { "true" } else { "false" },
            "data-nd-size": size.as_str(),
            "data-nd-dismissible": if dismissible { "true" } else { "false" },

            // RENDERED HIDDEN, revealed by `dist/dialog.js`. See the header,
            // and note that the stylesheet holds this box from the first paint
            // so revealing it moves nothing (`NOCTUA.md §9.6`).
            button {
                class: "nd-dialog-open",
                id: "{id}",
                r#type: "button",
                "aria-haspopup": "dialog",
                    hidden: true,
                "{trigger_label}"
            }

            dialog {
                class: "nd-dialog-surface",
                id: "{surface_id}",
                // One or the other, never both. See the header.
                "aria-label": named,
                "aria-labelledby": titled,
                "aria-describedby": described,
                if head {
                    div { class: "nd-dialog-head",
                        if let Some(heading) = heading {
                            h2 { class: "nd-dialog-title", id: "{title_id}", "{heading}" }
                        }
                        if dismissible {
                            button {
                                class: "nd-dialog-close",
                                r#type: "button",
                                "aria-label": close_label,
                                "data-nd-dialog-close": "",
                                Icon { icon: IconDrawing::CLOSE, size: IconSize::Small }
                            }
                        }
                    }
                }
                if let Some(description) = description {
                    p { class: "nd-dialog-note", id: "{note_id}", "{description}" }
                }
                // THE SCROLLER, AND IT IS ITS OWN ELEMENT. The surface clips and
                // carries the radius; this scrolls and carries none. `.nd-panel`
                // is two elements for exactly this reason.
                div { class: "nd-dialog-body", {children} }
            }
        }
    }
"#;

/// The same dialog, in React.
const DIALOG_REACT: &str = r#"  const modal_ = modal ?? true;
  const size_ = size ?? "medium";
  const dismissible_ = dismissible ?? true;
  const root = ["nd-dialog", className].filter(Boolean).join(" ");
  const id = `nd-dialog-${name}`;
  const titleId = `${id}-title`;
  const noteId = `${id}-note`;
  const head = Boolean(heading) || dismissible_;

  return h(
    "div",
    {
      className: root,
      "data-nd-dialog": name,
      "data-nd-modal": String(modal_),
      "data-nd-size": size_,
      "data-nd-dismissible": String(dismissible_),
      ...rest,
    },
    h(
      "button",
      {
        key: "o",
        className: "nd-dialog-open",
        id,
        type: "button",
        "aria-haspopup": "dialog",
          // See the Dioxus half: revealed by the script.
        hidden: true,
      },
      triggerLabel,
    ),
    h(
      "dialog",
      {
        key: "s",
        className: "nd-dialog-surface",
        id: `${id}-surface`,
        // See the Dioxus half: one name or the other, never both.
        "aria-label": heading ? undefined : label,
        "aria-labelledby": heading ? titleId : undefined,
        "aria-describedby": description ? noteId : undefined,
      },
      head
        ? h(
            "div",
            { key: "h", className: "nd-dialog-head" },
            heading ? h("h2", { key: "t", className: "nd-dialog-title", id: titleId }, heading) : null,
            dismissible_
              ? h(
                  "button",
                  {
                    key: "x",
                    className: "nd-dialog-close",
                    type: "button",
                    "aria-label": closeLabel,
                    "data-nd-dialog-close": "",
                  },
                  h(Icon, { icon: close, size: "small" }),
                )
              : null,
          )
        : null,
      description ? h("p", { key: "n", className: "nd-dialog-note", id: noteId }, description) : null,
      h("div", { key: "b", className: "nd-dialog-body" }, children),
    ),
  );
"#;

// --- Popover ------------------------------------------------------------------

/// A named surface hanging off a control, and the control it hangs off.
///
/// # Why this is a dialog and `Info` is a note
///
/// The two draw the same box. `Info` holds one sentence about the word beside
/// it, so it is a `role="note"` and is announced as an aside. This holds
/// controls a reader came to use, so it is a `role="dialog"` with a name and a
/// screen reader enters it. Neither is a menu, which owns the arrow keys.
///
/// # Why the trigger ships hidden
///
/// `Info`'s reason and `Dialog`'s: the surface cannot open without a script, so
/// a page that never ran one carries no control that opens nothing.
const POPOVER_DIOXUS: &str = r#"    let root = format!("nd-popover {class}");
    let root = root.trim().to_owned();
    let id = format!("nd-popover-{name}");
    let panel_id = format!("{id}-panel");
    let title_id = format!("{id}-title");
    let titled = heading.as_ref().map(|_| title_id.clone());
    let named = if heading.is_some() { None } else { label.clone() };
    let head = heading.is_some() || dismissible;

    rsx! {
        div {
            class: "{root}",
            "data-nd-popover": "{name}",
            "data-nd-placement": placement.as_str(),
            "data-nd-dismissible": if dismissible { "true" } else { "false" },

            // RENDERED HIDDEN. See the header.
            button {
                class: "nd-popover-trigger",
                id: "{id}",
                r#type: "button",
                "aria-haspopup": "dialog",
                "aria-expanded": "false",
                "aria-controls": "{panel_id}",
                hidden: true,
                "{trigger_label}"
            }

            div {
                class: "nd-popover-panel",
                id: "{panel_id}",
                role: "dialog",
                // One or the other, never both — `Dialog`'s rule, applied.
                "aria-label": named,
                "aria-labelledby": titled,
                hidden: true,
                if head {
                    div { class: "nd-popover-head",
                        if let Some(heading) = heading {
                            h2 { class: "nd-popover-title", id: "{title_id}", "{heading}" }
                        }
                        if dismissible {
                            button {
                                class: "nd-popover-close",
                                r#type: "button",
                                "aria-label": close_label,
                                "data-nd-popover-close": "",
                                Icon { icon: IconDrawing::CLOSE, size: IconSize::Small }
                            }
                        }
                    }
                }
                div { class: "nd-popover-body", {children} }
            }
        }
    }
"#;

/// The same surface, in React.
const POPOVER_REACT: &str = r#"  const placement_ = placement ?? "auto";
  const dismissible_ = dismissible ?? true;
  const root = ["nd-popover", className].filter(Boolean).join(" ");
  const id = `nd-popover-${name}`;
  const panelId = `${id}-panel`;
  const titleId = `${id}-title`;
  const head = Boolean(heading) || dismissible_;

  return h(
    "div",
    {
      className: root,
      "data-nd-popover": name,
      "data-nd-placement": placement_,
      "data-nd-dismissible": String(dismissible_),
      ...rest,
    },
    h(
      "button",
      {
        key: "t",
        className: "nd-popover-trigger",
        id,
        type: "button",
        "aria-haspopup": "dialog",
        "aria-expanded": "false",
        "aria-controls": panelId,
        // See the Dioxus half: revealed by the script.
        hidden: true,
      },
      triggerLabel,
    ),
    h(
      "div",
      {
        key: "p",
        className: "nd-popover-panel",
        id: panelId,
        role: "dialog",
        "aria-label": heading ? undefined : label,
        "aria-labelledby": heading ? titleId : undefined,
        hidden: true,
      },
      head
        ? h(
            "div",
            { key: "h", className: "nd-popover-head" },
            heading ? h("h2", { key: "i", className: "nd-popover-title", id: titleId }, heading) : null,
            dismissible_
              ? h(
                  "button",
                  {
                    key: "x",
                    className: "nd-popover-close",
                    type: "button",
                    "aria-label": closeLabel,
                    "data-nd-popover-close": "",
                  },
                  h(Icon, { icon: close, size: "small" }),
                )
              : null,
          )
        : null,
      h("div", { key: "b", className: "nd-popover-body" }, children),
    ),
  );
"#;

// --- Tooltip ------------------------------------------------------------------

/// One control, and the hint it gains.
///
/// # Why the bubble is rendered rather than built
///
/// `Info`'s reason: a bubble built on the first hover is one some engines never
/// announce, because `aria-describedby` has to point at an element that is
/// already in the document. It ships `hidden`, which is what keeps its text out
/// of the accessibility tree until a reader asks for it.
///
/// # Why the description is not written here
///
/// The control is the caller's own children, and a component cannot put an
/// attribute on markup it was handed. `dist/tooltip.js` writes the
/// `aria-describedby` on the first focusable descendant, once, and leaves a
/// control that already carries one alone.
const TOOLTIP_DIOXUS: &str = r#"    let root = format!("nd-tooltip {class}");
    let root = root.trim().to_owned();
    let id = format!("nd-tooltip-{name}");

    rsx! {
        span {
            class: "{root}",
            "data-nd-tooltip": "{name}",
            "data-nd-placement": placement.as_str(),
            "data-nd-delay": delay.as_str(),
            {children}
            // NOTHING FOCUSABLE GOES IN HERE, and the contract says so twice.
            // The bubble is not a tab stop and closes when the pointer leaves,
            // so a link inside it is a link that cannot be reached. `Popover`
            // is the component for a surface with controls in it.
            span {
                class: "nd-tooltip-bubble",
                id: "{id}",
                role: "tooltip",
                hidden: true,
                "{text}"
            }
        }
    }
"#;

/// The same hint, in React.
const TOOLTIP_REACT: &str = r#"  const placement_ = placement ?? "auto";
  const delay_ = delay ?? "short";
  const root = ["nd-tooltip", className].filter(Boolean).join(" ");

  return h(
    "span",
    {
      className: root,
      "data-nd-tooltip": name,
      "data-nd-placement": placement_,
      "data-nd-delay": delay_,
      ...rest,
    },
    children,
    // See the Dioxus half: rendered, hidden, and holding nothing focusable.
    h(
      "span",
      { key: "b", className: "nd-tooltip-bubble", id: `nd-tooltip-${name}`, role: "tooltip", hidden: true },
      text,
    ),
  );
"#;

// --- Toast --------------------------------------------------------------------

/// One message about something that just happened, floating clear of the work.
///
/// # Why it is a status and never an alert
///
/// `message_bar.toml` argues this at length and the corner it floats in does
/// not change the argument: an alert cuts a screen reader off mid-sentence, and
/// a component whose default is the rude one gets the rude one everywhere. A
/// caller who needs to interrupt writes `live` and has said so out loud.
///
/// # Why `aria-live` is absent unless a caller writes it
///
/// `role="status"` already implies polite, so writing the attribute at that
/// value would be this component overruling the role it just chose. Absent
/// means the role decides, and `dist/toast.js` puts back exactly what the
/// caller wrote after it has silenced the first paint.
///
/// # And the body carries a hook of its own
///
/// The live region is the root, and the sentence inside it is what the script
/// has to be able to find: a message enhanced after the page finished loading
/// is one that ARRIVED, and a region has to be written into rather than arrive
/// already full.
const TOAST_DIOXUS: &str = r#"    let root = format!("nd-toast {class}");
    let root = root.trim().to_owned();

    rsx! {
        div {
            class: "{root}",
            // A STATUS, NEVER AN ALERT. See the header.
            role: "status",
            // Written only when the caller said something.
            "aria-live": live.map(ToastLive::as_str),
            "data-nd-toast": "{name}",
            "data-nd-tone": tone,
            "data-nd-placement": placement.as_str(),
            "data-nd-duration": duration,
            "data-nd-countdown-label": countdown_label.clone(),
            "data-nd-dismissible": if dismissible { "true" } else { "false" },

            div { class: "nd-toast-text",
                // A `<strong>` and not a heading: a message that arrives after
                // the page has loaded must not rewrite the outline under a
                // reader who is navigating by it.
                if let Some(title) = title {
                    strong { class: "nd-toast-title", "{title}" }
                }
                div { class: "nd-toast-body", "data-nd-toast-body": "", {children} }

                // THE TIME LEFT, AND IT IS EMPTY UNTIL SOMETHING IS RUNNING
                // THE WAIT. `role="timer"` is a live region that does not
                // announce, so a screen reader is not read a new number every
                // second. It ships `hidden` for the dismiss control's reason:
                // a countdown that counts nothing is a number a reader watches
                // and learns nothing from, and with no script the message
                // never leaves at all.
                //
                // IT IS UNDER THE MESSAGE AND NOT BESIDE IT. A toast is about
                // twenty characters wide, so a line of digits in the row took
                // its width off the sentence and broke a message of nine words
                // over four lines.
                if countdown_label.is_some() {
                    span {
                        class: "nd-toast-countdown",
                        role: "timer",
                        "data-nd-toast-countdown": "",
                        hidden: true,
                    }
                }
            }

            if dismissible {
                // RENDERED HIDDEN, revealed by `dist/toast.js` — `MessageBar`'s
                // arrangement for `Info`'s reason. With no script there is
                // nothing to take the message away, and a control that closes
                // nothing is worse than no control at all.
                button {
                    class: "nd-toast-dismiss",
                    r#type: "button",
                    "aria-label": dismiss_label,
                    "data-nd-dismiss": "",
                    hidden: true,
                    Icon { icon: IconDrawing::CLOSE, size: IconSize::Small }
                }
            }
        }
    }
"#;

/// The same message, in React.
const TOAST_REACT: &str = r#"  const placement_ = placement ?? "bottom-end";
  const dismissible_ = dismissible ?? true;
  const root = ["nd-toast", className].filter(Boolean).join(" ");

  return h(
    "div",
    {
      className: root,
      // See the Dioxus half: a status, and no `aria-live` unless the caller
      // said something.
      role: "status",
      "aria-live": live,
      "data-nd-toast": name,
      "data-nd-tone": tone,
      "data-nd-placement": placement_,
      "data-nd-duration": duration,
      "data-nd-countdown-label": countdownLabel,
      "data-nd-dismissible": String(dismissible_),
      ...rest,
    },
    h(
      "div",
      { key: "t", className: "nd-toast-text" },
      title ? h("strong", { key: "h", className: "nd-toast-title" }, title) : null,
      h("div", { key: "b", className: "nd-toast-body", "data-nd-toast-body": "" }, children),
      // See the Dioxus half: a timer that does not announce, empty and hidden
      // until `dist/toast.js` is running the wait it reports, and under the
      // message rather than beside it.
      countdownLabel
        ? h("span", {
            key: "c",
            className: "nd-toast-countdown",
            role: "timer",
            "data-nd-toast-countdown": "",
            hidden: true,
          })
        : null,
    ),
    dismissible_
      ? h(
          "button",
          {
            key: "x",
            className: "nd-toast-dismiss",
            type: "button",
            "aria-label": dismissLabel,
            "data-nd-dismiss": "",
            // See the Dioxus half: revealed by the script.
            hidden: true,
          },
          h(Icon, { icon: close, size: "small" }),
        )
      : null,
  );
"#;

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

    /// The reason `Button.label` is required, asserted on the contract rather
    /// than on the prose: a control made of nothing but an icon must be
    /// nameable only by naming it. `IconButton` existed for this alone, and
    /// `hide_label` is what let one component do both jobs.
    #[test]
    fn a_button_cannot_be_built_without_a_name() {
        let component = shipped("Button");
        let label = &component.prop["label"];
        assert!(label.required, "`label` is optional, so the bug can return");
        assert!(
            label.default.is_none(),
            "a default name is a name nobody chose"
        );
        assert!(
            component.prop["hide_label"].default.as_deref() == Some("false"),
            "hiding the label must be the thing a caller asks for, never the default"
        );
    }

    /// Both bodies must decide the element the same way, or "behaviourally
    /// identical" is a claim nobody checks.
    #[test]
    fn both_bodies_render_a_link_when_given_an_href() {
        let component = shipped("Button");
        let (mut rust, mut js) = (String::new(), String::new());
        dioxus_body(&mut rust, &component);
        react_body(&mut js, &component);
        assert!(rust.contains("a {"), "{rust}");
        assert!(js.contains("h(\"a\""), "{js}");
        assert!(rust.contains("button {"));
        assert!(js.contains("\"button\""));
    }

    /// `aria-pressed="false"` makes a plain button announce itself as a toggle
    /// that is not pressed, which is a different control from the one it is.
    #[test]
    fn neither_body_writes_a_false_aria_state() {
        let component = shipped("Button");
        let (mut rust, mut js) = (String::new(), String::new());
        dioxus_body(&mut rust, &component);
        react_body(&mut js, &component);
        for body in [&rust, &js] {
            assert!(!body.contains("\"aria-pressed\": \"false\""), "{body}");
            assert!(!body.contains("\"aria-expanded\": \"false\""), "{body}");
            assert!(!body.contains("\"aria-current\": \"false\""), "{body}");
        }
    }

    /// A NEW browsing context gets `noopener` whether or not the caller asked.
    ///
    /// Both implementations, because "the two behave identically" is the claim
    /// this whole file exists to keep, and a security default that reached one
    /// language only would be the worst version of breaking it.
    #[test]
    fn both_bodies_default_a_targeted_link_to_noopener() {
        let component = shipped("Button");
        let (mut rust, mut js) = (String::new(), String::new());
        dioxus_body(&mut rust, &component);
        react_body(&mut js, &component);
        for body in [&rust, &js] {
            assert!(body.contains("noopener noreferrer"), "{body}");
        }
    }

    /// The state on every radio, whatever its value.
    ///
    /// A radio that omits `aria-checked` is a radio whose state is unknown, and
    /// a group of those announces nothing chosen. This is the one ARIA
    /// attribute on this page that must NOT be written conditionally, which is
    /// exactly why it is asserted beside the three that must.
    #[test]
    fn a_segmented_item_always_announces_its_state() {
        let component = shipped("SegmentedItem");
        let (mut rust, mut js) = (String::new(), String::new());
        dioxus_body(&mut rust, &component);
        react_body(&mut js, &component);
        assert!(
            rust.contains("if selected { \"true\" } else { \"false\" }"),
            "{rust}"
        );
        assert!(js.contains("String(selected_)"), "{js}");
    }

    /// A radio group is ONE tab stop, whether or not it has been answered.
    ///
    /// THIS ASSERTS THE GUARANTEE, NOT THE MECHANISM. It used to read
    /// `rust.contains("tabindex: if selected")` — the mechanism, and the
    /// mechanism was the defect: `0` on the chosen item and `-1` on the rest is
    /// one stop for a group with an answer and **zero** for a group without
    /// one, so a `Segmented` nothing had chosen was unreachable by keyboard
    /// with the script blocked. The test passed the whole time, because it was
    /// checking that the code said what it said.
    ///
    /// What is asserted instead is the count: exactly one element in the
    /// rendered group carries a `0`, in both languages and in both cases.
    #[test]
    fn a_segmented_group_is_one_tab_stop_answered_or_not() {
        let group = shipped("Segmented");
        let item = shipped("SegmentedItem");
        let (mut group_rust, mut group_js) = (String::new(), String::new());
        dioxus_body(&mut group_rust, &group);
        react_body(&mut group_js, &group);
        let (mut item_rust, mut item_js) = (String::new(), String::new());
        dioxus_body(&mut item_rust, &item);
        react_body(&mut item_js, &item);

        // The group holds it, unconditionally — no `if` anywhere near it.
        assert!(group_rust.contains("tabindex: \"0\","), "{group_rust}");
        assert!(group_js.contains("tabIndex: 0,"), "{group_js}");
        // And no item is ever a second one, whatever `selected` says.
        assert!(item_rust.contains("tabindex: \"-1\","), "{item_rust}");
        assert!(item_js.contains("tabIndex: -1,"), "{item_js}");
        for body in [&item_rust, &item_js] {
            assert!(
                !body.contains("selected { \"0\"") && !body.contains("selected_ ? 0"),
                "an item that is a stop of its own makes the count depend on the answer: {body}"
            );
        }
    }

    /// A refused item stays reachable.
    ///
    /// `disabled` removes it from the tab order, and an option a keyboard
    /// cannot reach is an option a keyboard user cannot be told about.
    #[test]
    fn a_refused_segmented_item_keeps_its_place() {
        let component = shipped("SegmentedItem");
        assert_eq!(
            component.prop["disabled"].attribute.as_deref(),
            Some("aria-disabled"),
            "`disabled` would take the item out of the roving order"
        );
    }
}
