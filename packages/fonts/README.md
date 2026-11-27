# @noctua-world/design-fonts

The typefaces of the [Noctua design system](https://github.com/noctua-world/noctua-design),
with their licences: the files `dist/fonts.css` asks for.

```sh
npm i @noctua-world/design-dist @noctua-world/design-fonts
cp -r node_modules/@noctua-world/design-dist/dist static/dist
mkdir -p static/assets
cp -r node_modules/@noctua-world/design-fonts/fonts static/assets/fonts
```

## Where the faces go

`dist/fonts.css` declares every face and asks for each at `../assets/fonts/<file>`: <!-- allow-reference: a URL relative to wherever the consumer put the sheet, not a path in this repository -->
in `assets/fonts/`, beside the directory that holds `dist/`. The two `cp` lines
above are that arrangement.

`fonts.json` lists every face by set, for a tool that copies some and not all:
`noctua-design-fonts` is the Latin faces, the code face, the symbols and eight
other scripts, about 3 MB, and each of `noctua-design-fonts-jp`, `-kr` and `-sc`
is one CJK family cut into slices. The sets are the crates of the same names on
crates.io, the same files byte for byte. A face that is not served costs nothing
but itself: the stacks fall back to the system's fonts, and a face is fetched
only when a page draws a character in its range.

## Licence

The Noto faces and Noctua Iosevka are under the SIL Open Font License 1.1, and
each licence is in `fonts/` beside the files it covers, because the licence
requires its text to travel with them. Symbols Nerd Font ships under the MIT
license, and the icon sets it gathers keep their own, among them CC BY 4.0,
Apache 2.0, the OFL and the Unlicense. `fonts/ATTRIBUTION.md` says where each
set's terms are.

**Copied** from `assets/fonts/` in that repository by its build, and committed
there. Nothing here is written by hand but this file and the manifest.
