// The React half of the example frame, loaded on demand.
//
// This module exists to prove one claim that a screenshot cannot: the React
// `ThemeScope` and the Dioxus one behave identically. Both are generated from
// `spec/theme.toml`, so their prop surfaces agree by construction — but "the
// same props" is not "the same behaviour", and the only way to show the second
// is to mount the thing and look at it.
//
// It is NEVER fetched on a normal page view. `shell.js` imports it the first
// time a reader switches an example to React, which is the one moment the cost
// is worth paying.

import { createElement as h } from "react";
import { createRoot } from "react-dom/client";
import { ThemeScope } from "@noctua-world/design-react/theme.js";
import { Button } from "@noctua-world/design-react/components/index.js";

/**
 * Renders the same sample the Dioxus example renders, into the same slot.
 *
 * The markup below mirrors `docs-site/src/pages.rs::Sample` class for class,
 * because the point of the comparison is that nothing but the framework
 * changes. A different sample would prove nothing.
 */
function Sample({ text }) {
  return h("div", { className: "nd-sample" }, [
    h("h4", { className: "nd-sample-heading", key: "h" }, text.heading),
    h("p", { className: "nd-prose", key: "p" }, [
      text.body + " ",
      h("a", { className: "nd-link", href: "#", key: "a" }, text.link),
    ]),
    h("div", { className: "nd-sample-actions", key: "b" }, [
      h(Button, { key: "1", label: text.action }),
      h(Button, { key: "2", variant: "outline", tone: "neutral", label: text.quiet }),
    ]),
  ]);
}

const roots = new WeakMap();

/**
 * Mounts the React example into `slot`.
 *
 * `scope` is the same request object the Dioxus example was built from, read
 * straight off the server-rendered element's attributes — so the two are
 * demonstrably showing the same theme rather than two themes that happen to
 * look alike.
 */
export function mount(slot, scope, text) {
  let root = roots.get(slot);
  if (!root) {
    root = createRoot(slot);
    roots.set(slot, root);
  }
  root.render(
    h(
      ThemeScope,
      { ...scope, className: "nd-scope" },
      h(Sample, { text }),
    ),
  );
  return root;
}

/** Unmounts it, so switching back to Dioxus leaves no React behind. */
export function unmount(slot) {
  const root = roots.get(slot);
  if (root) {
    root.unmount();
    roots.delete(slot);
  }
}
