// The React half of every example on the site, loaded on demand.
//
// # What this proves
//
// This system ships two implementations of the same components, and
// `NOCTUA.md §9.3` requires them to behave identically. Generating both from
// one spec makes their prop surfaces agree by construction, but "the same
// props" is not "the same behaviour", and the only way to show the second is to
// mount the thing and look at it.
//
// Until now exactly one example on the site could be mounted. That is how six
// modules of the React package came to be broken — two of them did not even
// parse — with the whole test suite green. Every example can be mounted now,
// which is the difference between a claim and a demonstration.
//
// # Why there is no code for any particular example here
//
// The old version of this file hand-mirrored one example, class for class, from
// `pages.rs`. That is a copy, and a copy of an example is an example that stops
// agreeing with the one it copies.
//
// An example is data now — see `docs-site/src/tree.rs` — so this file is an
// interpreter for that data and knows nothing about any example in particular.
// The Rust side walks the same tree through the generated components. Neither
// side can drift, because there is nothing for them to drift from.
//
// # It is NEVER fetched on a normal page view
//
// `shell.js` imports it the first time a reader asks for React, and that is
// asserted by a test, because a lazy path that loads eagerly is a lazy path
// with a bug nobody notices.

import { Fragment, createElement as h, useLayoutEffect } from "react";
import { createRoot } from "react-dom/client";
import { ThemeScope } from "@noctua-world/design-react/theme.js";

import { COMPONENTS, HEAVY } from "./registry.js";

// `ThemeScope` is not in `spec/components/`, so the generated registry does not
// carry it. It is the one component a tree can name that comes from the theme
// model instead.
const registry = { ...COMPONENTS, ThemeScope };

/**
 * Fetches whatever a page's trees need beyond the first chunk.
 *
 * Called with every component name the page's trees mention. A page that shows
 * no code block never asks for the grammars, and one that shows no lineage
 * never asks for the layout engine.
 */
export async function preload(names) {
  const wanted = new Set(names.filter((name) => name in HEAVY && !(name in registry)));
  await Promise.all(
    [...wanted].map(async (name) => {
      Object.assign(registry, await HEAVY[name]());
    }),
  );
}

/**
 * One node of a tree, as React.
 *
 * The three shapes are the serialised form of `crate::tree::Node`: `c` for a
 * component of the design system, `e` for the plain markup an example arranges
 * itself with, and `t` for a run of text. Everything else — the loops, the
 * fragments, the message lookups — was resolved on the Rust side, because the
 * page is built per locale and there is nothing left to decide here.
 */
function build(node, at) {
  if (node.k === "t") return node.v;

  const kind = node.k === "c" ? registry[node.n] : node.n;
  if (!kind) {
    // Loud rather than an empty box. `every_component_a_tree_names_exists`
    // makes this unreachable from a build that passed, so reaching it means
    // the registry and the catalogue have come apart.
    throw new Error(`no component named ${node.n}`);
  }

  const props = { ...node.p, key: at };
  const children = (node.c ?? []).map(build);
  // Children as arguments rather than as an array: React asks for keys on an
  // array and does not on arguments, and a key here would be a key the Dioxus
  // half does not have.
  return children.length ? h(kind, props, ...children) : h(kind, props);
}

/**
 * A whole example, with the design system's behaviours over it.
 *
 * # Why the behaviours are a layout effect and not a call after `render`
 *
 * `root.render` is asynchronous, so `shell.js` has no moment at which it knows
 * the DOM is there. A layout effect does: it runs after the mutation phase and
 * before paint, and a parent's runs after every child's, so this one sees the
 * whole subtree.
 *
 * Before paint is not a nicety. Several components ship **hidden** and are
 * revealed by the script — a menu button carries `hidden` until `menu.js`
 * reveals it, and a table's tools likewise — so a passive effect would show the
 * reader a frame of an empty example before the controls appeared.
 *
 * It runs on mount and on a changed tree, and **not** on every render. An
 * effect with no dependencies would run its cleanup on any reconciliation, and
 * the cleanup closes whatever surface is open — so a reader with a menu open
 * would have it shut under them. A mounted example never changes anyway: the
 * tree is static data and these components hold no state.
 */
function Drawn({ tree, host, hooks }) {
  useLayoutEffect(() => {
    hooks.enhance(host);
    return () => hooks.release(host);
  }, [tree, host, hooks]);
  return h(Fragment, null, ...tree.map(build));
}

const roots = new WeakMap();

/**
 * Mounts an example into `slot`.
 *
 * `hooks` carries `enhance` and `release` from `shell.js` rather than this
 * module importing them. The runtime behaviours are 208 KB that the page has
 * already loaded, and importing them here would make esbuild inline a second
 * copy into this bundle.
 */
export function mount(slot, tree, hooks) {
  let root = roots.get(slot);
  if (!root) {
    root = createRoot(slot);
    roots.set(slot, root);
  }
  root.render(h(Drawn, { tree, host: slot, hooks }));
  return root;
}

/**
 * Unmounts it.
 *
 * Unmounted rather than hidden, so switching back to Dioxus leaves no React
 * behind — which a test asserts, because a hidden React tree is still a React
 * tree holding a duplicate of the page's DOM.
 */
export function unmount(slot) {
  const root = roots.get(slot);
  if (root) {
    root.unmount();
    roots.delete(slot);
  }
}
