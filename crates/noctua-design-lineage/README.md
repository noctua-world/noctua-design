# noctua-design-lineage

Reads a relationship graph and places it in three dimensions, for the
[Noctua design system](https://github.com/noctua-world/noctua-design)'s
`Lineage` component.

**No runtime dependencies.** A graph layout is arithmetic and sorting; a graph
library or a linear-algebra crate would be a permanent commitment to somebody
else's decisions in exchange for code that fits in one file.

It is not `no_std`, for one reason: `f64::sqrt` lives in `std`. It is the only
transcendental here, it is exactly rounded by IEEE-754 in Rust and in every
JavaScript engine, and that is what lets the browser half be a transcription of
this file rather than an approximation of it — checked against the same blessed
layouts, in both languages.

MIT OR Apache-2.0.
