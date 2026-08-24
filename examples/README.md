# Two consumers, from outside the workspace

Both live outside the cargo workspace on purpose. A sibling project is not a
member of this one, and an example that compiled only because the workspace
resolved something for it would prove nothing about what an installed consumer
gets.

| | What it proves |
|---|---|
| `consumer-rust/` | The Dioxus crate builds and renders for a project that has only the crate — and every component's root class is painted by `dist/components.css` |
| `consumer-web/` | The stylesheets and the behaviour modules work in a plain HTML page: nine `<link>`s, two `import`s, and no build step of any kind |

```sh
cd examples/consumer-rust && cargo run     # also run by `cargo xtask check`
xdg-open examples/consumer-web/index.html  # or let the browser suite open it
```

`tests/browser/consumer.spec.js` opens the web one and measures it, because
whether a page is *styled* is a question only a rendering engine answers.
