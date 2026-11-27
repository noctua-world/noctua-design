# noctua-design-fonts

Typefaces of the [Noctua design system](https://github.com/noctua-world/noctua-design),
embedded, with one call that writes them out.

Every face that ships whole: Noto Sans, Noto Serif and Noto Sans Display, Noctua Iosevka,
the two Symbols Nerd Font faces, and Noto Sans for Arabic, Hebrew, Devanagari, Thai,
Ethiopic, Georgian, Armenian and Mongolian. About 3 MB, most of it the symbols.

```toml
[build-dependencies]
noctua-design-dist = "0.3.0"
noctua-design-fonts = "0.3.0"
```

```rust
// build.rs
fn main() -> std::io::Result<()> {
    noctua_design_dist::write_to("static/dist")?;
    noctua_design_fonts::write_to("static/assets/fonts")
}
```

## Where the faces go

`dist/fonts.css`, from `noctua-design-dist`, declares every face and asks for each
at `../assets/fonts/<file>`: in `assets/fonts/`, beside the directory that holds <!-- allow-reference: a URL relative to wherever the consumer put the sheet, not a path in this repository -->
`dist/`. So a page that links `static/dist/fonts.css` needs this crate's files at
`static/assets/fonts/`. Every crate of faces writes flat, by the names the
stylesheet uses, so they all write into that one directory.

A host that serves from memory needs no directory: `FILES` is every face as
`(name, bytes)`, and `SERVED` is the path a page asks for them under.

A face that is not there costs nothing but itself: the stylesheet's stacks fall
back to the system's fonts, and a face is fetched only when a page draws a
character in its range.

## The other faces

The faces are split across crates because a registry refuses a package over
10 MB and together they are 17 MB.

| Crate | Faces |
|---|---|
| `noctua-design-fonts` | the Latin faces, the code face, the symbols and eight other scripts |
| `noctua-design-fonts-jp` | Noto Sans JP |
| `noctua-design-fonts-kr` | Noto Sans KR |
| `noctua-design-fonts-sc` | Noto Sans SC |

On npm, `@noctua-world/design-fonts` holds all of them.

## Licence

The library is MIT OR Apache-2.0, at your option. The Noto faces and Noctua Iosevka are under the SIL Open Font License 1.1. Symbols Nerd
Font ships under the MIT license, and the icon sets it gathers keep their own: among
them CC BY 4.0, Apache 2.0, the OFL and the Unlicense. `ATTRIBUTION.md`, written out with
the faces, says where each set's terms are.

`LICENCES` holds each text, and `write_to` writes them beside the faces, because
the Open Font License requires its text to travel with the fonts.

**Generated** from `spec/fonts.toml` in that repository, and committed there.
Nothing in `src/` or `fonts/` is written by hand.
