// The JavaScript half of the `Lineage` layout, against the same expectation the
// Rust half is held to.
//
// THE HOLE THIS FILLS, AND THE TEST IT REFUSES TO BE.
//
// There are two engines: `crates/noctua-design-lineage/src/layout.rs`, which
// runs during SSR, and its twin `layout.js`, which runs in a browser for the
// React package. They must agree, and the obvious way to check that — lay the
// same graph out with both and compare — **passes whenever they are both wrong
// in the same way**. That is the failure that actually happens: one shared
// misreading of the specification, transcribed twice.
//
// So neither is compared to the other. Both are compared to
// `dist/lineage.cases.json`, emitted from `spec/lineage/cases.toml` by one
// function in one emitter. A disagreement names which half moved.
//
// This spec runs in Node rather than in a page, which is why it takes no `page`
// fixture: `packages/tokens/lineage.js` is a plain ES module with no DOM in it,
// and loading a page to test a pure function would only add ways for the test
// to fail for reasons that are not about the layout.

import { readFileSync } from "node:fs";
import { fileURLToPath } from "node:url";

import { expect, test } from "@playwright/test";

import { fold, matches, parse, scene } from "../../packages/tokens/lineage.js";

const root = fileURLToPath(new URL("../..", import.meta.url));
const FIXTURE = JSON.parse(readFileSync(`${root}dist/lineage.cases.json`, "utf8"));

/** Three decimals, which is the precision the two halves promise one another. */
const round = (value) => {
  const text = Number(value).toFixed(3);
  return text === "-0.000" ? "0.000" : text;
};

/** A row with every number re-rounded, so two spellings compare equal. */
const normalise = (row) =>
  row
    .split(/\s+/u)
    .map((field) => (Number.isNaN(Number(field)) || field === "" ? field : round(field)))
    .join(" ");

const options = (kase) => ({ view: kase.view, route: kase.route, shy: kase.reach === "shy" });

const placesOf = (laid) =>
  laid.places.map(
    (place) =>
      `${laid.graph.nodes[place.node].id} ${round(place.x)} ${round(place.y)} ${round(place.z)}`,
  );

const segmentsOf = (laid) =>
  laid.segments.map((run) => {
    const basis = run.axis === "free" ? ` ${run.basis.map(round).join(" ")}` : "";
    return `${run.bond} ${round(run.x)} ${round(run.y)} ${round(run.z)} ${round(run.len)} ${run.axis}${basis}`;
  });

test("the fixture reaches this file at all", () => {
  // A fixture that failed to load would make every test below vacuously true,
  // which is the one way a parity suite can be worse than no parity suite.
  expect(FIXTURE.cases.length).toBeGreaterThan(10);
  expect(parse("figure ada").nodes.length).toBe(1);
});

test("every case is placed exactly where the specification says", () => {
  for (const kase of FIXTURE.cases) {
    const laid = scene(kase.graph, options(kase));
    expect(placesOf(laid).map(normalise), `\`${kase.name}\` was placed differently`).toEqual(
      kase.places.map(normalise),
    );
    expect(laid.segments.length, `\`${kase.name}\` drew a different number of runs`).toBe(
      kase.runs,
    );
    expect(laid.crossings, `\`${kase.name}\` crossings`).toBe(kase.crossings);
    expect(laid.skipped, `\`${kase.name}\` skipped`).toBe(kase.skipped);
    expect(laid.broken, `\`${kase.name}\` broken`).toBe(kase.broken);
    if (kase.segments.length > 0) {
      // Both sides through `normalise`, which is not a formality: it rounds the
      // bond index too, and comparing a rounded row against an unrounded one
      // fails on every case while the geometry agrees to the digit.
      expect(segmentsOf(laid).map(normalise), `\`${kase.name}\` was routed differently`).toEqual(
        kase.segments.map(normalise),
      );
    }
  }
});

test("the same graph twice is the same scene twice", () => {
  // PURITY. An engine that consulted a clock, a locale or a random number would
  // still pass every case above on the run that blessed them.
  for (const kase of FIXTURE.cases) {
    expect(placesOf(scene(kase.graph, options(kase)))).toEqual(
      placesOf(scene(kase.graph, options(kase))),
    );
  }
});

test("nothing hangs on a graph written to break it", () => {
  // TERMINATION. A test that hangs is a suite that hangs, so these are here
  // rather than discovered by a reader who pasted a generated genealogy.
  const chain = [];
  for (let index = 0; index < 80; index += 1) {
    chain.push(`figure f${index}`, `bond f${index} f${index + 1}`);
  }
  const hostile = [
    "bond a b\nbond b a\nfigure a\nfigure b",
    "figure a\n".repeat(200),
    chain.join("\n"),
    "figure a rank=999\nfigure b rank=-999\nbond a b",
    "mark m\nbond m m\nfigure a\nbond a m kind=union\nbond m a kind=union",
  ];
  for (const graph of hostile) {
    expect(scene(graph, {}).places.length).toBeLessThanOrEqual(201);
  }
});

test("no identifier can carry anything but a word", () => {
  // THE INJECTION THE PARSER EXISTS FOR, asserted in both languages because
  // both build a custom property name out of an id. The claim is not that these
  // lines are refused — a quote is a delimiter, so `a"b` becomes the perfectly
  // safe `ab` — it is that nothing but a word ever reaches an id.
  const hostile = [
    "figure a);color:red;--x:(",
    'figure a"b',
    "figure a b",
    "figure ",
    "figure url(evil)",
    "bond a);x:( b",
    "bond a b);x:(",
  ];
  for (const line of hostile) {
    const graph = parse(line);
    const ids = graph.nodes
      .map((node) => node.id)
      .concat(graph.bonds.flatMap((bond) => [bond.from, bond.to]));
    for (const id of ids) {
      expect(id, `\`${line}\` produced the id \`${id}\``).toMatch(/^[A-Za-z0-9_-]+$/u);
    }
  }
});

test("a javascript url is dropped and a relative one is kept", () => {
  const graph = parse(
    "figure a href=javascript:alert(1)\n" +
      "figure b href=https://example.test/x image=../p.avif\n" +
      "figure c image=data:image/svg+xml;base64,AAAA",
  );
  expect(graph.nodes[0].href).toBe("");
  expect(graph.nodes[1].href).toBe("https://example.test/x");
  expect(graph.nodes[1].image).toBe("../p.avif");
  expect(graph.nodes[2].image).toBe("");
});

test("the fold is one code point in and one out", () => {
  // Length-preserving, because the matched positions are the only thing a
  // highlight has — and because the Rust half folds the same way.
  for (const text of ["Ada", "ÀÉÎÕÜ", "coração", "ÿ", "Doe-Smith"]) {
    expect([...fold(text)].length, `${text} changed length`).toBe([...text].length);
  }
  expect(fold("Coração")).toBe("coracao");
});

test("an empty filter matches everything and a term reaches the metadata", () => {
  const [node] = parse('figure ada label="Ada" meta="1815" terms="doe founder"').nodes;
  expect(matches(node, "")).toBe(true);
  expect(matches(node, "  ")).toBe(true);
  expect(matches(node, "1815")).toBe(true);
  expect(matches(node, "FOUNDER")).toBe(true);
  expect(matches(node, "smith")).toBe(false);
});
