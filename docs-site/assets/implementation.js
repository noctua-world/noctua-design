// Switching every example between the two implementations this system ships.
//
// # What this is for
//
// `NOCTUA.md §9.3` requires the Dioxus and the React halves to behave
// identically. Until now the site demonstrated that on one example, and the
// audit recorded what one example is worth: six React modules were broken, two
// of them did not parse, and the whole suite was green.
//
// Every example is data now, so every one of them can be drawn twice. This file
// is the control over that, and nothing else. It knows how to fetch, mount,
// unmount and remember; it knows nothing about any particular example.
//
// # Its own module, not part of `shell.js`
//
// `shell.js` is the page's behaviour and runs on every view. This runs only for
// a reader who asks, and it owns a preference, a lazy import, an observer and a
// failure mode — four things that are easier to reason about, and to switch
// off, in one place.

/// The stored preference. Site-local and document-level, exactly like the
/// locale: `spec/theme.toml` is the settings a `ThemeScope` can carry, and
/// which package renders an example is this site's problem rather than a
/// consumer's.
const KEY = "noctua-impl";

/// The two answers. Anything else in storage is not one of them, and
/// "whatever was in localStorage" is not a thing to act on.
const OFFERED = ["dioxus", "react"];

/// How far outside the viewport a frame is prepared.
///
/// `assets/model.js` already does this shape of lazy mount for the WebGL
/// stages, at the same distance and for the same reason: nothing is fetched
/// until a stage is near, and nothing is drawn while none is.
const REACH = "200px 0px"; // allow-literal: an observer margin, not a drawn length — it never reaches the page and no token describes "how early to prepare"

/// How many roots commit in one frame.
///
/// A page can hold forty-seven examples. Committing them in one task measured
/// long enough to drop frames, so they go four at a time and the page stays
/// answerable while it fills in.
const BATCH = 4;

/// What the page is showing, now.
///
/// **Separate from what is stored**, and that separation is load-bearing: a
/// preference is only written once a root has actually committed, so a reader
/// whose bundle 404s once does not carry a broken answer to every page they
/// visit afterwards. Without this the press had nowhere to be recorded until it
/// had already succeeded, and nothing was ever drawn.
let intent = null;

let arriving = null;
let trees = null;
let behaviours = null;
let refused = false;

/// What a frame is showing, when the reader has said so for that frame alone.
///
/// Deliberately not stored. A reader who compared one button on Tuesday should
/// not find that frame in React on Thursday with no memory of asking.
const chosen = new WeakMap();

/// Frames waiting to be drawn, in the order they came into reach.
const pending = [];
let draining = false;

function base() {
  return document.documentElement.dataset.noctuaBase ?? "";
}

function locale() {
  return document.documentElement.dataset.noctuaLocale ?? "en";
}

/// What the page should be showing.
///
/// The press if there has been one, and what was stored otherwise.
export function preference() {
  return intent ?? stored();
}

/// The reader's standing answer, or the honest default.
function stored() {
  try {
    const stored = localStorage.getItem(KEY);
    return OFFERED.includes(stored) ? stored : "dioxus";
  } catch {
    // Private browsing. A reader with no storage gets the server-rendered
    // answer, which is the one already on the screen.
    return "dioxus";
  }
}

function remember(value) {
  try {
    localStorage.setItem(KEY, value);
  } catch {
    // Nothing to do and nothing to report: the page still works.
  }
}

/// The bundle and the trees, fetched together.
///
/// Together, not one after the other, so the first switch waits for the slower
/// of the two rather than for both. Cached as a promise so a reader pressing
/// twice fetches once.
function arrive() {
  if (arriving === null) {
    arriving = Promise.all([
      import("./react/example-react.js"),
      fetch(`${base()}assets/examples-${locale()}.json`).then((response) => {
        if (!response.ok) throw new Error(`examples-${locale()}.json: ${response.status}`);
        return response.json();
      }),
    ]).then(([module, data]) => {
      trees = data;
      return module;
    });
  }
  return arriving;
}

/// Every stage in a frame that has a tree, and no deeper frame's.
///
/// Frames nest on the example-frame page, and the nearest frame owns its
/// stages. This is `shell.js`'s own containment rule, applied here.
function stages(frame) {
  return [...frame.querySelectorAll("[data-nd-example]")].filter(
    (stage) => stage.closest("[data-nd-frame]") === frame,
  );
}

function slotIn(stage) {
  let slot = stage.querySelector(":scope > [data-nd-react-slot]");
  if (!slot) {
    slot = document.createElement("div");
    slot.dataset.ndReactSlot = "";
    stage.append(slot);
  }
  return slot;
}

/// Which caption is showing, of the two a switchable example carries.
function caption(frame, language) {
  for (const half of frame.querySelectorAll("[data-nd-impl-source]")) {
    if (half.closest("[data-nd-frame]") !== frame) continue;
    half.hidden = half.dataset.ndImplSource !== language;
  }
}

/// Marks a frame's own control.
function mark(frame, language) {
  for (const toggle of frame.querySelectorAll('[data-nd-action="frame-impl"]')) {
    if (toggle.closest("[data-nd-frame]") !== frame) continue;
    say(toggle, language === "react");
  }
}

/// A toggle's state and its name, which change together.
///
/// `Button` writes the unpressed name into `aria-label` and the pressed one
/// into `data-nd-label-alt`. Without swapping them a toggle keeps one name and
/// announces the wrong state from the first press onward, which is the defect
/// `label_alt` exists for.
///
/// The unpressed name is kept on first use rather than read from a second
/// attribute: the markup already carries it, and a second copy is a second
/// thing to keep in step.
const named = new WeakMap();

function say(toggle, on) {
  if (!named.has(toggle)) named.set(toggle, toggle.getAttribute("aria-label") ?? "");
  if (on) toggle.setAttribute("aria-pressed", "true");
  else toggle.removeAttribute("aria-pressed");
  const name = on ? toggle.dataset.ndLabelAlt : named.get(toggle);
  if (name) {
    toggle.setAttribute("aria-label", name);
    toggle.setAttribute("title", name);
  }
}

async function draw(frame) {
  const module = await arrive();
  for (const stage of stages(frame)) {
    const tree = trees?.[stage.dataset.ndExample];
    if (!tree) continue;
    module.mount(slotIn(stage), tree, behaviours);
    stage.dataset.ndShowing = "react";
  }
  caption(frame, "React");
  mark(frame, "react");
}

/// Puts a frame back, and takes React off the page rather than hiding it.
export function undraw(frame) {
  for (const stage of stages(frame)) {
    if (stage.dataset.ndShowing !== "react") continue;
    delete stage.dataset.ndShowing;
    const slot = stage.querySelector(":scope > [data-nd-react-slot]");
    if (slot) {
      // Release before unmount, while the elements the behaviours are holding
      // are still in the document. Afterwards there is nothing to release from.
      behaviours?.release(slot);
      arriving?.then((module) => module.unmount(slot)).catch(() => {});
    }
  }
  caption(frame, "Dioxus");
  mark(frame, "dioxus");
}

/// Which implementation a frame should be showing.
///
/// A frame shows its own answer where the reader gave one, and the page's
/// otherwise. Setting the page's answer clears every frame's — the alternative,
/// where a frame the reader touched is pinned and a later page-wide press
/// silently skips it, is the version nobody can explain from the screen.
function wanted(frame) {
  return chosen.get(frame) ?? preference();
}

function queue(frame) {
  if (!pending.includes(frame)) pending.push(frame);
  if (draining) return;
  draining = true;
  requestAnimationFrame(function drain() {
    const batch = pending.splice(0, BATCH);
    for (const frame of batch) {
      if (wanted(frame) === "react") void attempt(frame);
    }
    if (pending.length) requestAnimationFrame(drain);
    else draining = false;
  });
}

async function attempt(frame) {
  if (refused) return;
  // BUSY IS AN ATTRIBUTE, NOT A REPLACEMENT FOR THE CONTROL'S CONTENT.
  //
  // This wrote the loading message into `textContent`, which was harmless while
  // the control was a stop with a word in it. On an icon it **destroyed both
  // SVG glyphs**, and restoring the text afterwards put back a text node where
  // two drawings had been: a pressed button with nothing in it.
  //
  // `aria-busy` says the same thing without touching the tree, and the message
  // goes where an icon-only control keeps its words — its accessible name.
  const control = frame.querySelector('[data-nd-action="frame-impl"]');
  const said = control?.getAttribute("aria-label");
  if (control) {
    control.setAttribute("aria-busy", "true");
    if (frame.dataset.ndLoading) {
      control.setAttribute("aria-label", frame.dataset.ndLoading);
      control.setAttribute("title", frame.dataset.ndLoading);
    }
  }
  try {
    await draw(frame);
    // Only now, and only for a page-wide press. Two rules, both learned:
    //
    // A preference written on the press, before anything committed, would
    // follow a reader whose bundle 404s once onto every page afterwards and
    // fail on each.
    //
    // And a preference written by a SINGLE frame's press is not a preference.
    // It made this frame's own Reset re-draw it in React instead of putting it
    // back, because the page's answer had quietly become React underneath.
    if (!chosen.has(frame)) remember("react");
  } catch (error) {
    refuse(error);
  } finally {
    if (control) {
      control.removeAttribute("aria-busy");
      // `say` puts the right one of the two names back, so this only has to
      // undo the loading message where the state did not change.
      if (said) {
        control.setAttribute("aria-label", said);
        control.setAttribute("title", said);
      }
      mark(frame, wanted(frame));
    }
  }
}

/// What happens when the bundle cannot be had.
///
/// Once, and visibly. The old version logged and restored one label, which was
/// enough for one control on one page. A page-wide control that appears to do
/// nothing needs a sentence, and every stop that cannot be honoured has to
/// refuse rather than sit there looking available.
function refuse(error) {
  refused = true;
  console.error("[noctua] the React examples could not be loaded", error);
  for (const frame of document.querySelectorAll("[data-nd-frame]")) undraw(frame);
  for (const toggle of document.querySelectorAll('[data-nd-action="frame-impl"]')) {
    toggle.setAttribute("aria-disabled", "true");
    toggle.removeAttribute("aria-pressed");
  }
  const toggle = document.querySelector('[data-nd-action="impl"]');
  if (toggle) {
    toggle.setAttribute("aria-disabled", "true");
    toggle.removeAttribute("aria-pressed");
  }
  const bar = document.querySelector("[data-nd-impl-failed]");
  if (bar) bar.hidden = false;
}

/// Sets the page's answer, and clears every frame's.
export function choose(language) {
  const going = OFFERED.includes(language) ? language : "dioxus";
  intent = going;
  for (const frame of document.querySelectorAll("[data-nd-frame]")) chosen.delete(frame);
  if (going === "dioxus") {
    remember("dioxus");
    for (const frame of document.querySelectorAll("[data-nd-frame]")) undraw(frame);
  } else {
    for (const frame of watching) queue(frame);
  }
  pressed(going === "react");
}

/// The page control's own state, said the same way.
function pressed(on) {
  const toggle = document.querySelector('[data-nd-action="impl"]');
  if (toggle) say(toggle, on);
}

/// Frames near enough to be worth drawing.
const watching = new Set();

const approaching = new IntersectionObserver(
  (entries) => {
    for (const entry of entries) {
      if (!entry.isIntersecting) continue;
      watching.add(entry.target);
      if (wanted(entry.target) === "react") queue(entry.target);
    }
  },
  { rootMargin: REACH },
);

/// Starts watching every frame that can be switched.
///
/// Idempotent, and called again when a fold opens: a frame inside a closed
/// `<details>` has no size, so it never comes into reach, which is correct
/// until the reader opens it.
export function observe(root = document) {
  for (const frame of root.querySelectorAll("[data-nd-frame]")) {
    if (frame.querySelector("[data-nd-example]")) approaching.observe(frame);
  }
}

/// Wires the controls and honours what the reader asked for last time.
///
/// `hooks` carries `enhance` and `release`. They are injected rather than
/// imported because the runtime behaviours are 208 KB the page has already
/// loaded, and importing them from the React bundle would inline a second copy.
export function install(hooks) {
  behaviours = hooks;
  if (!document.querySelector("[data-nd-example]")) return;

  document.addEventListener("click", (event) => {
    const toggle = event.target.closest?.('[data-nd-action="impl"]');
    if (toggle) {
      if (toggle.getAttribute("aria-disabled") === "true") return;
      choose(toggle.getAttribute("aria-pressed") === "true" ? "dioxus" : "react");
      return;
    }

    // Reset undoes this frame's own answer too, and hands it back to the
    // page's. A frame that stayed in React after a reset would be a frame the
    // reset button had lied about.
    const reset = event.target.closest?.('[data-nd-action="frame-reset"]');
    if (reset) {
      const frame = reset.closest("[data-nd-frame]");
      if (frame) {
        chosen.delete(frame);
        if (preference() === "react") queue(frame);
        else undraw(frame);
      }
      return;
    }

    const stop = event.target.closest?.('[data-nd-action="frame-impl"]');
    if (!stop || stop.getAttribute("aria-disabled") === "true") return;
    const frame = stop.closest("[data-nd-frame]");
    const language = stop.getAttribute("aria-pressed") === "true" ? "dioxus" : "react";
    chosen.set(frame, language);
    if (language === "react") queue(frame);
    else undraw(frame);
  });

  // A frame inside a closed fold has never been measured. `toggle` does not
  // bubble, so this listens in the capture phase.
  document.addEventListener(
    "toggle",
    (event) => {
      if (event.target instanceof HTMLDetailsElement && event.target.open) {
        observe(event.target);
      }
    },
    true,
  );

  observe();
  pressed(preference() === "react");
}
