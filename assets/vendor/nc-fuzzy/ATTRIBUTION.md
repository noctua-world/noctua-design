# The fuzzy core's corpus, vendored

`fuzzy-corpus.tsv` is the matching rule of `NOCTUA.md §12.4` written as rows:
what each character folds to, how a query splits into words, how a query matches
an item, and in what order a list comes back. Its own header describes the
format.

| | |
|---|---|
| Project | `noctua-central`, the `nc-fuzzy` crate |
| File there | `fuzzy-corpus.tsv`, in the crate's test directory |
| Taken from | branch `dev`, commit `3c0e749fb08400025ebf94b96c12a90d2fc98155` |
| Checksum | `SHA256SUMS` beside this file, as `sha256sum` writes it |
| Licence | this repository's own, MIT or Apache-2.0: the file is the same project's, copied here by its author |

## Why it is vendored rather than read in place

`nc-fuzzy` is the rule and `dist/search-match.js` is its second implementation.
Neither is tested against the other, which would pass whenever both were wrong
the same way: each replays this one file in its own suite, the crate in Rust and
`tests/browser/fuzzy-corpus.spec.js` here. CI clones this repository alone, so
the rows have to be in it.

## It is never edited here

A row is a claim about the rule, and the rule is `nc-fuzzy`'s. `cargo xtask
check` holds the file to the recorded checksum and fails on any other byte. A
change to the rule is made in the crate, and then this copy and its record are
replaced together, as `AGENTS.md` says under "The fuzzy matcher".
