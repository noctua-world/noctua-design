// The Noctua design system — the React half.
//
// There is deliberately nothing here. This package exports no component, for
// the same reason its Dioxus counterpart does not: NOCTUA-PLAN.md Stage 3
// builds the machine that builds components, and a component built before the
// machine gets rebuilt. Stage 6 adds the first ones.
//
// When they arrive, this package and crates/noctua-design must stay
// BEHAVIOURALLY IDENTICAL. One contract, two implementations, one shared set of
// behavioural cases run against both — NOCTUA.md §9.3. Adding a component here
// without adding it there is a failing conformance test, by design.

export {};
