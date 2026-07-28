//! The Noctua design system — the Dioxus half.
//!
//! # Every component here is generated
//!
//! [`ThemeScope`] comes from `spec/theme.toml`; every text component comes from
//! its own `spec/components/<name>.toml`. Nothing in this crate is written by
//! hand, and that is the parity mechanism rather than a convenience: the React
//! package is generated from the same files, so the two cannot declare
//! different props, different defaults or different values.
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

// GENERATED. Do not edit src/theme.rs or src/components/; edit the spec and
// rebuild. `cargo xtask check` fails if either drifts.
pub mod components;
pub mod script;
pub mod theme;

// Re-exported so a consumer takes one dependency rather than two, and so the
// token crate's version can never disagree with the components that use it.
pub use noctua_design_theme as model;
pub use noctua_design_tokens as tokens;

pub use components::*;
pub use script::{Script, dominant_script, font_override};
pub use theme::{ThemeScope, use_theme};
