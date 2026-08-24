//! The component rules this design system **ships**, and where each file is.
//!
//! # Why a crate holds CSS
//!
//! Because `noctua-design-runtime` already holds the JavaScript this way, and
//! one arrangement for one job is worth more than a better second one. What it
//! buys here is [`sources`]: **that array is the cascade**, so the order two
//! rules resolve in is one reviewable list rather than an accident of how a
//! directory happened to be read.
//!
//! Every file is a band of `docs-site/assets/shell.css` as it was written
//! there, in the order it was written. Nothing was reformatted when it moved,
//! and nothing should be: a rule whose specificity ties with another is decided
//! by which comes first, so a reordering has to be argued for, not made.
//!
//! # What a consumer's page inherits
//!
//! `base.css` reaches past this system's own components, and deliberately:
//!
//! - `box-sizing: border-box` on `*`, which every rule here is written in.
//! - `margin: 0` on `h1`–`h4`, `p`, `ul` and `figure`, on top of which each
//!   component sets its own spacing.
//! - `[hidden] { display: none !important }`, the one `!important` here.
//! - a `:focus-visible` ring, which is every component's focus ring.
//!
//! All four are low specificity and a consumer's own rules beat them. They are
//! written down because they are the price of the components looking right, and
//! a consumer should meet them here rather than discover them.
//!
//! `base.css` also declares the eight `--nd-*` custom properties that shipped
//! rules read. That makes `--nd-*` a public name after being an internal one;
//! moving them into `spec/tokens.toml` as `--noctua-*` is the tidier answer and
//! it is a rename, not a move, so it is not done here.
//!
//! # What is NOT here
//!
//! The documentation site's own chrome — its header, drawer, sidebar, page
//! shape, example frames and variant grids — stays in
//! `docs-site/assets/shell.css`, which the site links **after** this, so the
//! documentation can override a component and never the reverse.

/// The directory these files live in, as written into the generated header.
pub const SOURCE_DIR: &str = "crates/noctua-design-styles/src/";

/// Every stylesheet this crate carries, as `(path, contents)`.
///
/// The list is long because the system has fifty families of rules, and one
/// entry per family is the shortest honest way to write which order they load
/// in. Splitting it would hide the cascade in two places.
///
/// **The order of this array is the cascade.** It is the order the rules were
/// written in `shell.css`, band for band, because that is the only order they
/// have ever been proved in.
#[must_use]
#[allow(
    clippy::too_many_lines,
    reason = "one line per stylesheet, and the order is the point"
)]
pub fn sources() -> [(&'static str, &'static str); 50] {
    [
        (
            "crates/noctua-design-styles/src/base.css",
            include_str!("base.css"),
        ),
        (
            "crates/noctua-design-styles/src/icon-glyph.css",
            include_str!("icon-glyph.css"),
        ),
        (
            "crates/noctua-design-styles/src/toolbar-group.css",
            include_str!("toolbar-group.css"),
        ),
        (
            "crates/noctua-design-styles/src/switch.css",
            include_str!("switch.css"),
        ),
        (
            "crates/noctua-design-styles/src/checkbox.css",
            include_str!("checkbox.css"),
        ),
        (
            "crates/noctua-design-styles/src/select.css",
            include_str!("select.css"),
        ),
        (
            "crates/noctua-design-styles/src/date-input.css",
            include_str!("date-input.css"),
        ),
        (
            "crates/noctua-design-styles/src/date-picker.css",
            include_str!("date-picker.css"),
        ),
        (
            "crates/noctua-design-styles/src/segmented.css",
            include_str!("segmented.css"),
        ),
        (
            "crates/noctua-design-styles/src/surfaces.css",
            include_str!("surfaces.css"),
        ),
        (
            "crates/noctua-design-styles/src/visually-hidden.css",
            include_str!("visually-hidden.css"),
        ),
        (
            "crates/noctua-design-styles/src/heading.css",
            include_str!("heading.css"),
        ),
        (
            "crates/noctua-design-styles/src/button.css",
            include_str!("button.css"),
        ),
        (
            "crates/noctua-design-styles/src/button-group.css",
            include_str!("button-group.css"),
        ),
        (
            "crates/noctua-design-styles/src/swatch.css",
            include_str!("swatch.css"),
        ),
        (
            "crates/noctua-design-styles/src/tag.css",
            include_str!("tag.css"),
        ),
        (
            "crates/noctua-design-styles/src/badge.css",
            include_str!("badge.css"),
        ),
        (
            "crates/noctua-design-styles/src/avatar.css",
            include_str!("avatar.css"),
        ),
        (
            "crates/noctua-design-styles/src/avatar-group.css",
            include_str!("avatar-group.css"),
        ),
        (
            "crates/noctua-design-styles/src/tag-picker.css",
            include_str!("tag-picker.css"),
        ),
        (
            "crates/noctua-design-styles/src/tree.css",
            include_str!("tree.css"),
        ),
        (
            "crates/noctua-design-styles/src/text.css",
            include_str!("text.css"),
        ),
        (
            "crates/noctua-design-styles/src/code-block.css",
            include_str!("code-block.css"),
        ),
        (
            "crates/noctua-design-styles/src/pagination.css",
            include_str!("pagination.css"),
        ),
        (
            "crates/noctua-design-styles/src/info.css",
            include_str!("info.css"),
        ),
        (
            "crates/noctua-design-styles/src/menu.css",
            include_str!("menu.css"),
        ),
        (
            "crates/noctua-design-styles/src/ai-label.css",
            include_str!("ai-label.css"),
        ),
        (
            "crates/noctua-design-styles/src/progress.css",
            include_str!("progress.css"),
        ),
        (
            "crates/noctua-design-styles/src/table.css",
            include_str!("table.css"),
        ),
        (
            "crates/noctua-design-styles/src/search.css",
            include_str!("search.css"),
        ),
        (
            "crates/noctua-design-styles/src/disclosure.css",
            include_str!("disclosure.css"),
        ),
        (
            "crates/noctua-design-styles/src/lineage.css",
            include_str!("lineage.css"),
        ),
        (
            "crates/noctua-design-styles/src/color-picker.css",
            include_str!("color-picker.css"),
        ),
        (
            "crates/noctua-design-styles/src/field.css",
            include_str!("field.css"),
        ),
        (
            "crates/noctua-design-styles/src/textarea.css",
            include_str!("textarea.css"),
        ),
        (
            "crates/noctua-design-styles/src/number-input.css",
            include_str!("number-input.css"),
        ),
        (
            "crates/noctua-design-styles/src/file-input.css",
            include_str!("file-input.css"),
        ),
        (
            "crates/noctua-design-styles/src/time-input.css",
            include_str!("time-input.css"),
        ),
        (
            "crates/noctua-design-styles/src/carousel.css",
            include_str!("carousel.css"),
        ),
        (
            "crates/noctua-design-styles/src/error-summary.css",
            include_str!("error-summary.css"),
        ),
        (
            "crates/noctua-design-styles/src/divider.css",
            include_str!("divider.css"),
        ),
        (
            "crates/noctua-design-styles/src/label.css",
            include_str!("label.css"),
        ),
        (
            "crates/noctua-design-styles/src/image.css",
            include_str!("image.css"),
        ),
        (
            "crates/noctua-design-styles/src/message-bar.css",
            include_str!("message-bar.css"),
        ),
        (
            "crates/noctua-design-styles/src/rating.css",
            include_str!("rating.css"),
        ),
        (
            "crates/noctua-design-styles/src/otp-field.css",
            include_str!("otp-field.css"),
        ),
        (
            "crates/noctua-design-styles/src/dialog.css",
            include_str!("dialog.css"),
        ),
        (
            "crates/noctua-design-styles/src/popover.css",
            include_str!("popover.css"),
        ),
        (
            "crates/noctua-design-styles/src/tooltip.css",
            include_str!("tooltip.css"),
        ),
        (
            "crates/noctua-design-styles/src/toast.css",
            include_str!("toast.css"),
        ),
    ]
}

/// How many stylesheets ship.
pub const COUNT: usize = 50;

#[cfg(test)]
mod tests {
    use super::*;

    /// A file added to `src/` and left out of [`sources`] ships nowhere and
    /// reports nothing. `include_str!` catches a rename; only this catches an
    /// arrival.
    #[test]
    fn every_stylesheet_in_this_crate_is_carried() {
        let dir = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("src");
        let mut on_disk: Vec<String> = std::fs::read_dir(&dir)
            .expect("the crate has a src directory")
            .filter_map(Result::ok)
            .map(|entry| entry.file_name().to_string_lossy().into_owned())
            .filter(|name| {
                std::path::Path::new(name)
                    .extension()
                    .is_some_and(|e| e == "css")
            })
            .collect();
        on_disk.sort();
        let mut carried: Vec<String> = sources()
            .iter()
            .map(|(path, _)| {
                path.rsplit('/')
                    .next()
                    .expect("a path has a last segment")
                    .to_owned()
            })
            .collect();
        carried.sort();
        assert_eq!(on_disk, carried, "src/ and sources() disagree");
    }

    /// An empty file is a band that was lost on its way here.
    #[test]
    fn no_stylesheet_is_empty() {
        for (path, text) in sources() {
            assert!(text.trim().len() > 40, "{path} carries nothing");
        }
    }

    /// Balanced braces are the cheapest proof that a band arrived whole. The
    /// count is per file so the failure names the file rather than the sheet.
    #[test]
    fn every_stylesheet_closes_what_it_opens() {
        for (path, text) in sources() {
            let opened = text.matches('{').count();
            let closed = text.matches('}').count();
            assert_eq!(
                opened, closed,
                "{path} opens {opened} blocks and closes {closed}"
            );
        }
    }

    // Whether a shipped rule names a class only the documentation site draws is
    // asked where it can be asked properly, over selectors rather than over the
    // text: `noctua_design_emit::styles`, which reads both stylesheets and the
    // set of classes the components can actually emit.
}
