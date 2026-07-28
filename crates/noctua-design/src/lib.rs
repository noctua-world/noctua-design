//! The Noctua design system — the Dioxus half.
//!
//! # There is deliberately nothing here
//!
//! This crate exports no component, and that is the point of the stage that
//! created it. `noctua-design` will grow to hundreds of components over many
//! sessions, and everything about how one is specified, generated, tested and
//! documented is decided *before* the first one exists — because a component
//! built before the machine that builds components gets rebuilt.
//!
//! `NOCTUA-PLAN.md` Stage 3, "Must NOT be touched": *No components. Not even a
//! Button.* Stage 6 adds the first ones.
//!
//! What does exist is everything a component will need: the token layer
//! ([`noctua_design_tokens`]), the font layer, the source gates, and a single
//! `cargo xtask check` that runs all of it.
//!
//! # The parity rule, which arrives with the first component
//!
//! This crate and `packages/react` must stay **behaviourally identical**. A
//! component is four things that change together — the contract in
//! `spec/components/<name>.toml`, the Dioxus implementation, the React
//! implementation, and the shared behavioural cases in `<name>.cases.toml`.
//! Editing one implementation alone is a compile error or a failing conformance
//! test, by design. `NOCTUA.md §9.3`.

// Re-exported so a consumer takes one dependency rather than two, and so the
// token crate's version can never disagree with the components that use it.
pub use noctua_design_tokens as tokens;
