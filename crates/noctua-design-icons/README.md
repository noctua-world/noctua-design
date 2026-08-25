# noctua-design-icons

The [Noctua design system](https://github.com/noctua-world/noctua-design)'s icon
drawings, one associated `const` each.

**One icon costs one drawing.** A `const` is materialised at its use sites and
nowhere else, so an application that draws three icons carries three drawings —
not the catalogue. An array of all of them would be a use site for all of them,
which is why reaching the catalogue is behind the `catalogue` feature and off by
default.

```rust
use noctua_design_icons::Icon;

let menu = Icon::MENU;      // the drawing, and nothing else in the binary
```

**Generated** from `spec/` in that repository. The drawings are vendored from
Lucide and Material Symbols, whose terms (ISC and MIT) travel in `LICENSE` there.

This crate itself is MIT OR Apache-2.0.
