// The Noctua design system — the React half, in types.
//
// It mirrors index.js, and the component index it re-exports is generated from
// spec/components/ exactly as the components are. Kept here rather than
// generated because index.js is hand-written too, and one of the pair being
// generated would be the confusing arrangement.

export * from "./components/index.js";
export { ThemeScope, useTheme } from "./theme.js";
