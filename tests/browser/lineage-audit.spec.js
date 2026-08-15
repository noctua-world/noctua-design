// Every scene on the page, measured against the things a reader can see.
//
// WHY THIS FILE EXISTS, IN THE DEVELOPER'S OWN WORDS: *these are basic, obvious
// UI problems, and they will keep happening unless the way you work changes.*
// They were right. Ten defects were reported in one message — labels written
// over cards, a line ending in mid air, a union dot drawn on top of a person,
// figures touching each other in most examples, a tooltip behind the scene —
// and every single one of them is a MEASUREMENT: two boxes intersecting, a gap
// of zero, an endpoint that lands on nothing. None of it was being measured.
// Screenshots were being looked at instead, a few of them, at one width.
//
// So this walks every scene on the page, at four widths, in both locales, and
// reports what it finds by name. It is deliberately not a snapshot: a snapshot
// tells you something changed, and what was wanted here is something that tells
// you what is WRONG, in a sentence, in a picture nobody has looked at yet.
//
// WHAT IT DOES NOT CHECK. Whether a scene is pleasant, whether the slab reads
// as a solid, whether the owl looks alive. Those still need eyes. What this
// removes is the class of defect that never needed them.

import { expect, test } from "@playwright/test";

import { LOCALES, PAGE, url, showParts } from "./pages.js";

/** The widths a reader actually has. 1024 is on the list because three pages
 *  scrolled sideways there for weeks, in the gap between 768 and 1440. */
const WIDTHS = [320, 768, 1024, 1440];

/** Two boxes may share this many pixels before it counts as an overlap: a
 *  hairline border, and the half-pixel a transform lands on. */
const TOUCH = 1.5;

/** How much air two figures on one row must have between them. Below this they
 *  read as one object with a line down the middle. */
const AIR = 5;

/** How near a connection's end must come to the piece it claims to reach. */
const REACH = 6;

/** No text a reader is meant to read may render smaller than this. */
const TYPE = 12;

/** No portrait may render smaller than this. */
const FACE = 16;

/** A connection's label may stand at most this far from its own line. */
const NEAR = 90;

/**
 * Every finding in one page, as sentences.
 *
 * Runs entirely inside the page: one round trip per scene set rather than one
 * per box, because there are about four hundred boxes on this page and a
 * Playwright call each would take longer than the rest of the suite.
 */
async function audit(page) {
  return page.evaluate(
    ({ TOUCH, AIR, REACH, TYPE, FACE, NEAR }) => {
      const findings = [];
      const px = (value) => Number.parseFloat(value) || 0;
      const seen = (el) => {
        const style = getComputedStyle(el);
        if (style.display === "none" || style.visibility === "hidden") return false;
        if (el.hidden) return false;
        const box = el.getBoundingClientRect();
        return box.width > 1 && box.height > 1;
      };
      const box = (el) => el.getBoundingClientRect();
      const overlap = (a, b) => {
        const x = Math.min(a.right, b.right) - Math.max(a.left, b.left);
        const y = Math.min(a.bottom, b.bottom) - Math.max(a.top, b.top);
        return x > TOUCH && y > TOUCH ? { x, y } : null;
      };
      const name = (el) => {
        const piece = el.closest("[data-nd-id]");
        if (piece) return piece.dataset.ndId;
        if (el.dataset.ndAt) return `figure at ${el.dataset.ndAt}`;
        return (el.textContent ?? "").trim().slice(0, 20) || el.className.split(" ")[0];
      };

      // EVERY SCENE, AND THE VARIANT SAMPLES ARE SCENES. They have no `name`
      // — a specimen is markup rather than a component call — so selecting on
      // `[data-nd-lineage]` skipped the whole grid, which is where three of the
      // ten reported defects were.
      for (const scene of document.querySelectorAll(".nd-lineage")) {
        const label =
          scene.dataset.ndLineage ||
          (scene.getAttribute("aria-labelledby") || "").replace("nd-variant-lineage-", "") ||
          "?";
        // The CLASS, not the attribute: a specimen is raw markup and carries
        // no `data-nd-lineage-field`, so the attribute skipped every sample in
        // the variant grid — and the grid is where three of the ten defects
        // were reported from.
        const field = scene.querySelector(".nd-lineage-field");
        if (!field || !seen(field)) continue;
        const say = (text) => findings.push(`${label}: ${text}`);

        // A DEEP SCENE IS A PROJECTION. Cards on different planes overlap on
        // screen by design and the resting turn swings them outside flat box
        // arithmetic, so the geometric rules below keep to flat scenes; a
        // deep scene's clipping and text are still checked, and its geometry
        // belongs to the eyes pass.
        const projected = scene.dataset.ndView === "deep";

        // What a reader sees as a thing standing in the scene.
        const solids = [...scene.querySelectorAll(".nd-lineage-card, .nd-lineage-figure")]
          .filter(seen)
          .filter((el) => !el.closest("[hidden]"));
        const tags = [...scene.querySelectorAll(".nd-lineage-tag")].filter(seen);

        // --- Nothing is drawn on top of anything else ----------------------
        for (let i = 0; !projected && i < solids.length; i += 1) {
          for (let j = i + 1; j < solids.length; j += 1) {
            // A figure standing at a slot sits exactly where its piece is, and
            // that is the mechanism rather than a defect.
            const a = solids[i];
            const b = solids[j];
            if (name(a) === name(b)) continue;
            const hit = overlap(box(a), box(b));
            if (hit) {
              say(
                `${name(a)} and ${name(b)} overlap by ` +
                  `${Math.round(hit.x)}x${Math.round(hit.y)} pixels`,
              );
            }
          }
        }

        // --- And two figures on one row have air between them --------------
        for (let i = 0; !projected && i < solids.length; i += 1) {
          for (let j = i + 1; j < solids.length; j += 1) {
            const a = box(solids[i]);
            const b = box(solids[j]);
            const shares = Math.min(a.bottom, b.bottom) - Math.max(a.top, b.top) > a.height / 2;
            if (!shares) continue;
            const gap = a.left < b.left ? b.left - a.right : a.left - b.right;
            if (gap >= 0 && gap < AIR) {
              say(
                `${name(solids[i])} and ${name(solids[j])} are ` +
                  `${Math.round(gap)} pixels apart on the same row`,
              );
            }
          }
        }

        // --- A label belongs in the air, never over a face -----------------
        for (const tag of tags) {
          for (const solid of solids) {
            const hit = overlap(box(tag), box(solid));
            if (hit) {
              say(`the label "${(tag.textContent ?? "").trim()}" is over ${name(solid)}`);
            }
          }
        }

        const window_ = box(field);

        // --- Every connection reaches something at both ends ---------------
        //
        // A SHY SCENE STOPS ON PURPOSE: `reach: shy` trims every figure-bound
        // end by 0.35 of the down-pitch, so for those scenes the landing
        // tolerance is that trim plus the ordinary reach — a shy line must
        // still stop NEAR its figure, and a rule that skipped these scenes
        // would let one stop nowhere near it.
        const give =
          scene.dataset.ndReach === "shy"
            ? 0.35 * (Number.parseFloat(getComputedStyle(field).getPropertyValue("--nd-celly")) || 160) + REACH
            : REACH;
        for (const bond of projected ? [] : scene.querySelectorAll(".nd-lineage-bond")) {
          if (!seen(bond) && bond.getClientRects().length === 0) continue;
          if (getComputedStyle(bond).display === "none" || bond.hidden) continue;
          const runs = [...bond.querySelectorAll(".nd-lineage-run")].filter(seen);
          if (runs.length === 0) continue;
          // The ends of a run, for the two axes whose box IS the run. A `z` run
          // and a free one are foreshortened, and their box says nothing about
          // where they start.
          const ends = [];
          for (const run of runs) {
            const r = box(run);
            const axis = run.dataset.ndAxis;
            if (axis === "x") {
              ends.push({ x: r.left, y: (r.top + r.bottom) / 2 });
              ends.push({ x: r.right, y: (r.top + r.bottom) / 2 });
            } else if (axis === "y") {
              ends.push({ x: (r.left + r.right) / 2, y: r.top });
              ends.push({ x: (r.left + r.right) / 2, y: r.bottom });
            }
          }
          if (ends.length === 0) continue;
          // A BOND THAT TRAVELS IN DEPTH IS NOT MEASURED HERE. A `z` run and a
          // free one are foreshortened: their bounding box is the projection of
          // a line rather than the line, so an end computed from it is a guess.
          // What this rule is for — a connection drawn to a place where nothing
          // is — is a flat-scene defect, and every flat scene is still checked.
          // EVERY run, not the visible ones: a `z` run pointing straight at the
          // camera is nought pixels wide, so filtering by "can be seen" threw
          // away the very run that says this bond travels in depth.
          if (
            [...bond.querySelectorAll(".nd-lineage-run")].some(
              (run) => run.dataset.ndAxis === "z" || run.dataset.ndAxis === "free",
            )
          ) {
            continue;
          }
          // An end that touches another run of the same bond is a joint rather
          // than a loose end — a corner where two runs meet at their tips, and
          // a T where a stem meets the middle of the channel. Both are how a
          // bracket is drawn, and neither is a connection to nothing.
          const boxes = runs.map(box);
          const joint = (end) =>
            boxes.filter(
              (r) =>
                end.x >= r.left - REACH &&
                end.x <= r.right + REACH &&
                end.y >= r.top - REACH &&
                end.y <= r.bottom + REACH,
            ).length > 1;
          const loose = ends.filter((end) => !joint(end));
          for (const end of loose) {
            const lands = solids.some((solid) => {
              // A piece may hold lines at its own distance (`reach=<cells>`
              // in the graph): its give is its own. A slotted figure is NOT
              // nested in its piece — it stands at the anchor independently —
              // so its piece is found by the `at` it names. The scene ROOT
              // also carries `data-nd-reach` as the mode word, which parses
              // to NaN and falls through, deliberately.
              const anchor = solid.dataset.ndAt
                ? scene.querySelector(`.nd-lineage-piece[data-nd-id="${solid.dataset.ndAt}"]`)
                : solid.closest(".nd-lineage-piece");
              const own = Number.parseFloat(anchor?.getAttribute("data-nd-reach"));
              const celly =
                Number.parseFloat(getComputedStyle(field).getPropertyValue("--nd-celly")) || 160;
              const bespoke = Number.isFinite(own) ? own * celly + REACH : give;
              const s = box(solid);
              return (
                end.x >= s.left - bespoke &&
                end.x <= s.right + bespoke &&
                end.y >= s.top - bespoke &&
                end.y <= s.bottom + bespoke
              );
            });
            if (!lands) {
              // NAME WHAT IT NEARLY REACHED. Two page coordinates tell whoever
              // reads the failure nothing at all about which picture to open.
              let near = { who: "nothing", far: Infinity };
              for (const solid of solids) {
                const s_ = box(solid);
                const far = Math.hypot(
                  Math.max(s_.left - end.x, 0, end.x - s_.right),
                  Math.max(s_.top - end.y, 0, end.y - s_.bottom),
                );
                if (far < near.far) near = { who: name(solid), far };
              }
              say(
                `a connection ends ${Math.round(near.far)} pixels short of ` +
                  `${near.who} and reaches nothing ` +
                  `(at ${Math.round(end.x - window_.left)},${Math.round(end.y - window_.top)} ` +
                  `in a ${bond.dataset.ndKind ?? "descent"} bond)`,
              );
            }
          }
        }

        // --- A line never passes through a card that is not its destination -
        //
        // The reader's own rule, in their own words, after a union line from
        // Ada ran straight through her sister-in-law's card on its way to
        // Eve — which reads as a different marriage. A run ENDS in the card
        // it belongs to — every stem is drawn to the piece's centre, under
        // the card — so a run with an end inside a card is that card's own
        // leg, and a DRIVE-THROUGH is a run whose penetration is deep while
        // both of its ends are outside. Marks are exempt — a union point
        // sits on its own line by design — and a diagonal (`free`) run is
        // not measured because its box is not the line.
        const figures = solids.filter(
          (solid) => !solid.closest('[data-nd-piece="mark"]'),
        );
        for (const bond of projected ? [] : scene.querySelectorAll(".nd-lineage-bond")) {
          if (getComputedStyle(bond).display === "none" || bond.hidden) continue;
          for (const run of bond.querySelectorAll(".nd-lineage-run")) {
            if (!seen(run)) continue;
            const axis = run.dataset.ndAxis;
            if (axis !== "x" && axis !== "y") continue;
            const r = box(run);
            const tips =
              axis === "x"
                ? [
                    { x: r.left, y: (r.top + r.bottom) / 2 },
                    { x: r.right, y: (r.top + r.bottom) / 2 },
                  ]
                : [
                    { x: (r.left + r.right) / 2, y: r.top },
                    { x: (r.left + r.right) / 2, y: r.bottom },
                  ];
            for (const solid of figures) {
              const s = box(solid);
              const inside = tips.some(
                (tip) =>
                  tip.x >= s.left - TOUCH &&
                  tip.x <= s.right + TOUCH &&
                  tip.y >= s.top - TOUCH &&
                  tip.y <= s.bottom + TOUCH,
              );
              if (inside) continue;
              const along =
                axis === "x"
                  ? Math.min(r.right, s.right) - Math.max(r.left, s.left)
                  : Math.min(r.bottom, s.bottom) - Math.max(r.top, s.top);
              const across =
                axis === "x"
                  ? Math.min(r.bottom, s.bottom) - Math.max(r.top, s.top)
                  : Math.min(r.right, s.right) - Math.max(r.left, s.left);
              if (along > 8 && across > 0) {
                say(
                  `a ${bond.dataset.ndKind ?? "descent"} line drives ` +
                    `${Math.round(along)} pixels through ${name(solid)}`,
                );
              }
            }
          }
        }

        // --- Nothing a reader reads is microscopic -------------------------
        //
        // The old model scaled type with a shrinking cell, and a milestone
        // scene rendered at seven pixels. The sizing model now makes this
        // structurally impossible; the rule is here so it STAYS impossible.
        for (const text of scene.querySelectorAll(
          ".nd-lineage-name, .nd-lineage-meta, .nd-lineage-tag, .nd-lineage-count, .nd-lineage-figure .nd-text",
        )) {
          if (!seen(text)) continue;
          const size = px(getComputedStyle(text).fontSize);
          if (size < TYPE) {
            say(`"${(text.textContent ?? "").trim().slice(0, 16)}" renders at ${Math.round(size)}px`);
          }
        }

        for (const face of scene.querySelectorAll(".nd-lineage-portrait, .nd-lineage .nd-avatar")) {
          if (!seen(face)) continue;
          const wide = box(face).width;
          if (wide < FACE) say(`a portrait renders ${Math.round(wide)}px wide`);
        }

        // --- A label stands by its own line --------------------------------
        //
        // "parceira" once floated in empty space, nowhere near the dotted
        // line it named. A label belongs within arm's reach of one of its own
        // bond's runs.
        for (const tag of tags) {
          const bond = tag.closest(".nd-lineage-bond");
          if (!bond) continue;
          const runs = [...bond.querySelectorAll(".nd-lineage-run")].filter(seen);
          if (runs.length === 0) continue;
          const t = box(tag);
          const cx = (t.left + t.right) / 2;
          const cy = (t.top + t.bottom) / 2;
          let nearest = Infinity;
          for (const run of runs) {
            const r = box(run);
            const far = Math.hypot(
              Math.max(r.left - cx, 0, cx - r.right),
              Math.max(r.top - cy, 0, cy - r.bottom),
            );
            if (far < nearest) nearest = far;
          }
          if (nearest > NEAR) {
            say(
              `the label "${(tag.textContent ?? "").trim()}" floats ` +
                `${Math.round(nearest)}px from its own line`,
            );
          }
        }

        // --- Nothing is cut off by its own box -----------------------------
        //
        // A box whose overflow is VISIBLE cuts nothing: a bare figure lets
        // its content stand past its anchor rectangle on purpose, and
        // `scrollHeight` reports that overhang whether or not anything is
        // lost. Only a box that actually clips is measured.
        for (const solid of solids) {
          const flow = getComputedStyle(solid);
          if (flow.overflowX === "visible" && flow.overflowY === "visible") continue;
          const cut = Math.max(
            solid.scrollHeight - solid.clientHeight,
            solid.scrollWidth - solid.clientWidth,
          );
          if (cut > 1) say(`${name(solid)} clips ${cut} pixels of its own content`);
        }

        // --- And nothing is beyond the window's REACH ----------------------
        //
        // The field pans: a scene wider than the window overflows to the
        // right and the platform's own scrollbar reaches it, so "outside the
        // client box" is the model working. A defect is what the scroll
        // cannot reach: left of the scrollable origin, above it, or past the
        // scrollable extent.
        for (const solid of projected ? [] : solids) {
          const s = box(solid);
          const originX = window_.left - field.scrollLeft;
          const originY = window_.top - field.scrollTop;
          const out = Math.max(
            originX - s.left,
            s.right - (originX + field.scrollWidth),
            originY - s.top,
            s.bottom - (originY + field.scrollHeight),
          );
          if (out > TOUCH) {
            say(`${name(solid)} is ${Math.round(out)} pixels beyond the window's reach`);
          }
        }
      }
      return findings;
    },
    { TOUCH, AIR, REACH, TYPE, FACE, NEAR },
  );
}

for (const locale of LOCALES.filter((one) => one !== "qps")) {
  for (const width of WIDTHS) {
    test(`nothing in a scene collides, at ${width} in ${locale}`, async ({ page }) => {
      await page.setViewportSize({ width, height: 900 });
      await page.goto(url(PAGE.lineage, locale));
      await showParts(page);
      // The scenes below the fold are laid out too — nothing here is lazy — but
      // the fonts decide how wide a name is, and a name decides how tall a card
      // is.
      await page.evaluate(() => document.fonts.ready);
      await page.waitForTimeout(250);

      const findings = await audit(page);
      expect(findings, `${findings.length} things collide`).toEqual([]);
    });
  }
}
