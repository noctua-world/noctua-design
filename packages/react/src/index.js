// The Noctua design system — the React half.
//
// Every component here is GENERATED from spec/components/<name>.toml, the same
// file crates/noctua-design is generated from. That is what makes this package
// and its Dioxus counterpart behaviourally identical (NOCTUA.md §9.3) by
// construction rather than by review: neither names a prop, a default or a
// value, so neither can disagree with the other about one.
//
// Do not edit src/components/ or src/theme.js. Edit the spec and rebuild;
// `cargo xtask check` fails if either has drifted.

export * from "./components/index.js";
export { ThemeScope, useTheme } from "./theme.js";
