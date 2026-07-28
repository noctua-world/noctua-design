//! The Noctua design system — the Dioxus half.
//!
//! # There is exactly one component here, and it is not a widget
//!
//! [`ThemeScope`] is the whole surface. Not a Button, not a Card — Stage 6 adds
//! those, and `NOCTUA-PLAN.md` Stage 3 is explicit that a component built before
//! the machine that builds components gets rebuilt.
//!
//! `ThemeScope` is here anyway because it is not a widget: it is how a theme
//! reaches a subtree at all, and every component Stage 6 adds will be rendered
//! inside one. It is also **generated** from `spec/theme.toml` rather than
//! written, so it is a product of the machine rather than an exception to it.
//!
//! What else exists is everything a component will need: the token layer
//! ([`noctua_design_tokens`]), the theme model ([`noctua_design_theme`]), the
//! font layer, the source gates, and a single `cargo xtask check` that runs all
//! of it.
//!
//! # The parity rule, which arrives with the first component
//!
//! This crate and `packages/react` must stay **behaviourally identical**. A
//! component is four things that change together — the contract in
//! `spec/components/<name>.toml`, the Dioxus implementation, the React
//! implementation, and the shared behavioural cases in `<name>.cases.toml`.
//! Editing one implementation alone is a compile error or a failing conformance
//! test, by design. `NOCTUA.md §9.3`.

// GENERATED from spec/theme.toml by `cargo xtask build`. Do not edit
// src/theme.rs; edit the setting table and rebuild.
pub mod theme;

// Re-exported so a consumer takes one dependency rather than two, and so the
// token crate's version can never disagree with the components that use it.
pub use noctua_design_theme as model;
pub use noctua_design_tokens as tokens;

pub use theme::{ThemeScope, use_theme};
