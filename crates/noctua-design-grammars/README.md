# noctua-design-grammars

Syntax grammars, and the hand-written scanner behind the
[Noctua design system](https://github.com/noctua-world/noctua-design)'s code
blocks.

**No dependencies.** A consumer showing a code block should not inherit a regex
engine, a parser generator or a grammar runtime — which is also why the scanner
is written against tables rather than compiled from a grammar format.

The tables are **generated** from `spec/` in that repository. The same tables are
emitted as JavaScript, so a page that highlights in the browser and a server that
highlights in Rust produce the same spans rather than nearly the same ones.

MIT OR Apache-2.0.
