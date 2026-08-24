//! The browser behaviour this design system **ships**, and where each file is.
//!
//! # What is here
//!
//! | File | What it does |
//! |---|---|
//! | `anchor.js` | Puts a fixed panel under the control it belongs to, and closes it when that control scrolls away |
//! | `color.js` | sRGB, HSV, hex, HSL and OKLCH, and a parser for the notations a reader may type |
//! | `messages.js` | Filling a caller's sentence, and choosing its plural form |
//! | `color-picker.js` | `enhancePickers()` — everything a `ColorPicker` does once a script is running |
//! | `inputs.js` | `enhanceInputs()` — the same, for the six input controls |
//! | `error-summary.js` | `enhanceErrorSummaries()` — the focus, the jump to the control, and the list a client-side refusal makes |
//! | `carousel.js` | `enhanceCarousels()` — the controls, the dots, the count, and the four rules autoplay carries |
//! | `table.js` | `enhanceTables()` — sorting, filtering, resizing, pinning, grouping, export, and the aggregates a filter recounts |
//! | `info.js` | `enhanceInfos()` — the note beside a word or beside an assistant's mark, on a press and never on a hover |
//! | `menu.js` | `enhanceMenus()` — the list of actions a button opens, and every key a menu owes |
//! | `message-bar.js` | `enhanceMessageBars()` — revealing the control that takes a bar away, and taking it |
//! | `date-picker.js` | `enhanceDatePickers()` — the month a reader presses days in, the arrows that walk it, and the panel it folds into |
//! | `dialog.js` | `enhanceDialogs()` — opening the platform's `<dialog>`, and the Escape a non-modal one is owed |
//! | `popover.js` | `enhancePopovers()` — the named surface a control opens, and every way out of it |
//! | `tooltip.js` | `enhanceTooltips()` — the hint a control gains on hover, on focus and under a held finger |
//! | `toast.js` | `enhanceToasts()` — the timer that pauses, and the live region a message arrives in |
//! | `panels.js` | Which single floating list on the page is open, whoever drew it |
//! | `roving.js` | `enhanceRoving()` — the one arrow walk a radio group, a toolbar and a tab list share |
//! | `select.js` | `enhanceSelects()` — the listbox, the filter and the type-ahead a `Select` gains |
//! | `tag-picker.js` | `enhanceTagPickers()` — the suggestions, and the tags a reader adds and takes back |
//! | `tree.js` | `enhanceTrees()` — the arrows, the type-ahead and the one tab stop a tree owes |
//! | `tabs.js` | `enhanceTabs()` — the panel that shows and the ones that stop being on the page |
//! | `date-input.js` | `enhanceDateInputs()` — the three boxes: stepping, paste, the locale's own order |
//!
//! # Why a crate holds JavaScript
//!
//! Because `xtask` already reads `crates/noctua-design-lineage/src/layout.js`
//! this way, and one arrangement for one job is worth more than a better second
//! one. What this crate adds over that precedent is the path itself: `xtask`
//! reads from these constants and `noctua-design-emit` writes them into the
//! generated header, so the two cannot spell a file differently.
//!
//! # Why any of it ships at all
//!
//! `NOCTUA.md §9.4`: a component whose consumer cannot use it is a component
//! that lies. `Lineage` was the first to ship its behaviour, for that reason
//! and in those words. `ColorPicker` is the second, and its case is stronger —
//! a colour picker that cannot pick a colour outside this repository's own
//! documentation site is not a colour picker.
//!
//! **The site-only controls are gone.** `Select`, `TagPicker`, `Tree`, `Tabs`
//! and `DateInput` were enhanced from `docs-site/assets/shell.js`, so a
//! consumer who installed the packages got their roles — `listbox`, `tree`,
//! `tablist` — and none of the keyboard those roles oblige. They are here now,
//! and the site imports them back. `Search` is the one that has not moved: its
//! index is fetched from a path the documentation site decides, and until that
//! is a prop rather than a convention it would ship half a component.

/// `anchor.js`, as written in generated headers and read by `xtask`.
pub const ANCHOR_SOURCE: &str = "crates/noctua-design-runtime/src/anchor.js";

/// `color.js`, likewise.
pub const COLOR_SOURCE: &str = "crates/noctua-design-runtime/src/color.js";

/// `color-picker.js`, likewise.
pub const PICKER_SOURCE: &str = "crates/noctua-design-runtime/src/color-picker.js";

/// `messages.js`, likewise.
pub const MESSAGES_SOURCE: &str = "crates/noctua-design-runtime/src/messages.js";

/// `inputs.js`, likewise.
pub const INPUTS_SOURCE: &str = "crates/noctua-design-runtime/src/inputs.js";

/// `carousel.js`, likewise.
pub const CAROUSEL_SOURCE: &str = "crates/noctua-design-runtime/src/carousel.js";

/// `error-summary.js`, likewise.
pub const SUMMARY_SOURCE: &str = "crates/noctua-design-runtime/src/error-summary.js";

/// `info.js`, likewise — the sixth caller of `anchor.js`.
pub const INFO_SOURCE: &str = "crates/noctua-design-runtime/src/info.js";

/// `menu.js`, likewise — the seventh caller, and the one that made
/// `Button { menu: true }` stop lying.
pub const MENU_SOURCE: &str = "crates/noctua-design-runtime/src/menu.js";

/// `table.js`, likewise — the largest of them, and the one a data grid is.
pub const TABLE_SOURCE: &str = "crates/noctua-design-runtime/src/table.js";

/// `message-bar.js`, likewise — the smallest of them, and the one that stops
/// `MessageBar { dismissible }` being a control that closes nothing.
pub const MESSAGE_BAR_SOURCE: &str = "crates/noctua-design-runtime/src/message-bar.js";

/// `date-picker.js`, likewise — the first date behaviour this crate ships.
pub const DATE_PICKER_SOURCE: &str = "crates/noctua-design-runtime/src/date-picker.js";

/// `dialog.js`, likewise — the one file here that opens a `<dialog>`, which is
/// the only way one opens at all.
pub const DIALOG_SOURCE: &str = "crates/noctua-design-runtime/src/dialog.js";

/// `popover.js`, likewise — the ninth caller of `anchor.js`.
pub const POPOVER_SOURCE: &str = "crates/noctua-design-runtime/src/popover.js";

/// `tooltip.js`, likewise — the tenth, and the one that writes the
/// `aria-describedby` a component cannot put on markup it was handed.
pub const TOOLTIP_SOURCE: &str = "crates/noctua-design-runtime/src/tooltip.js";

/// `toast.js`, likewise — the timer that pauses, and the live region rule that
/// a message has to be written into rather than arrive full.
pub const TOAST_SOURCE: &str = "crates/noctua-design-runtime/src/toast.js";

/// `panels.js`, likewise — the registry that keeps one floating list open.
pub const PANELS_SOURCE: &str = "crates/noctua-design-runtime/src/panels.js";

/// `roving.js`, likewise — the arrow walk a radio group, a toolbar and a tab
/// list all promise.
pub const ROVING_SOURCE: &str = "crates/noctua-design-runtime/src/roving.js";

/// `select.js`, likewise — the listbox a `Select` becomes.
pub const SELECT_SOURCE: &str = "crates/noctua-design-runtime/src/select.js";

/// `tag-picker.js`, likewise.
pub const TAG_PICKER_SOURCE: &str = "crates/noctua-design-runtime/src/tag-picker.js";

/// `tree.js`, likewise.
pub const TREE_SOURCE: &str = "crates/noctua-design-runtime/src/tree.js";

/// `tabs.js`, likewise.
pub const TABS_SOURCE: &str = "crates/noctua-design-runtime/src/tabs.js";

/// `date-input.js`, likewise — the three boxes, not the month behind them.
pub const DATE_INPUT_SOURCE: &str = "crates/noctua-design-runtime/src/date-input.js";

/// Every file this crate carries, as `(path, contents)`.
///
/// `include_str!` rather than a read at build time: a file renamed out from
/// under the emitter is then a compile error naming the file, instead of an
/// `xtask` that runs and writes an empty artifact. The paths above are what
/// name it; these are what it says.
#[must_use]
pub fn sources() -> [(&'static str, &'static str); 23] {
    [
        (ANCHOR_SOURCE, include_str!("anchor.js")),
        (COLOR_SOURCE, include_str!("color.js")),
        (MESSAGES_SOURCE, include_str!("messages.js")),
        (PICKER_SOURCE, include_str!("color-picker.js")),
        (INPUTS_SOURCE, include_str!("inputs.js")),
        (CAROUSEL_SOURCE, include_str!("carousel.js")),
        (SUMMARY_SOURCE, include_str!("error-summary.js")),
        (TABLE_SOURCE, include_str!("table.js")),
        (INFO_SOURCE, include_str!("info.js")),
        (MENU_SOURCE, include_str!("menu.js")),
        (MESSAGE_BAR_SOURCE, include_str!("message-bar.js")),
        (DATE_PICKER_SOURCE, include_str!("date-picker.js")),
        (DIALOG_SOURCE, include_str!("dialog.js")),
        (POPOVER_SOURCE, include_str!("popover.js")),
        (TOOLTIP_SOURCE, include_str!("tooltip.js")),
        (TOAST_SOURCE, include_str!("toast.js")),
        (PANELS_SOURCE, include_str!("panels.js")),
        (ROVING_SOURCE, include_str!("roving.js")),
        (SELECT_SOURCE, include_str!("select.js")),
        (TAG_PICKER_SOURCE, include_str!("tag-picker.js")),
        (TREE_SOURCE, include_str!("tree.js")),
        (TABS_SOURCE, include_str!("tabs.js")),
        (DATE_INPUT_SOURCE, include_str!("date-input.js")),
    ]
}

/// One file's contents, by the path that names it.
///
/// # Panics
///
/// On a path this crate does not carry, which is a programming error rather
/// than a condition: the three constants above are the only legal arguments.
#[must_use]
pub fn source(path: &str) -> &'static str {
    sources()
        .into_iter()
        .find(|(name, _)| *name == path)
        .map_or_else(
            || panic!("this crate carries no `{path}`"),
            |(_, text)| text,
        )
}

#[cfg(test)]
mod tests {
    use super::*;

    /// A file that failed to arrive would emit an empty artifact, and an empty
    /// `dist/color-picker.js` is a picker that silently does nothing.
    #[test]
    fn every_file_has_something_in_it() {
        for (path, text) in sources() {
            assert!(
                text.lines().count() > 10,
                "{path} is empty or nearly so: {} line(s)",
                text.lines().count()
            );
        }
    }

    /// Each one is a module, because each one is imported rather than run.
    #[test]
    fn every_file_exports_something() {
        for (path, text) in sources() {
            assert!(
                text.contains("export "),
                "{path} exports nothing, so nothing can import it"
            );
        }
    }

    #[test]
    fn a_path_this_crate_does_not_carry_is_a_panic() {
        assert!(std::panic::catch_unwind(|| source("nowhere.js")).is_err());
    }
}
