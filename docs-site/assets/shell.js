// The documentation shell's behaviour.
//
// It knows almost nothing. Every value, default, storage key and DOM attribute
// comes from ./theme.js, which is generated from spec/theme.toml — the same
// table the Rust model and the inline bootstrap come from. So a setting added
// to the spec appears here without this file being touched, and this file
// cannot spell a key differently from the bootstrap that reads it.
//
// THREE THINGS IT DELIBERATELY DOES NOT DO:
//
// 1. It does not build the page. Everything is in the markup already; the
//    script adds settings to a page that is complete without it.
// 2. It does not read a color out of the DOM. `getComputedStyle(…).color`
//    returns whatever the engine felt like serialising, in whatever space, and
//    a test that compares two of those strings passes on a color it never
//    checked. Paint into a 1×1 canvas and read the pixel instead.
// 3. It does not trap focus or set `inert`. The drawer is not modal — widening
//    the window past `lg` undoes the whole open state in CSS, with no listener
//    to miss and no state to unwind.
//
// A FRAME OWNS WHAT IS INSIDE IT, AND THAT RULE IS WHY THERE IS NO TARGET PROP.
//
// A setting a frame STATES owns its whole subtree — a nested frame, its bar,
// its border, its own menu panel. A setting nobody stated comes from OUTSIDE,
// which on an ordinary page is the page and inside another frame is that frame.
// Reset CLEARS the request; it does not state the page. This replaces a rule
// that said the opposite, and the opposite is why the defect kept coming back:
// a frame that defaults its request to the page's whole theme has the resolver
// stamp the page back on and cancel the frame around it.
// `tests/browser/frame-isolation.spec.js` is named after this rule so a rewrite
// cannot lose it.
//
// Four things that make it look done when it is not:
//
//   * A FRAME THE RESOLVER NEVER VISITS CANNOT FOLLOW ANYTHING, so every frame
//     is registered at load whether or not it has been themed.
//   * `inheritedAt` MUST WALK REGISTERED SCOPES, NOT STAMPED ONES. Stamped
//     means resolution depends on which ancestors happen to have been visited
//     in the current pass — theme the inner frame first and it inherits the
//     page.
//   * A VALUE STAMPED AS THE ABSENCE OF ITS ATTRIBUTE means both "the machine
//     decides" AND "whatever is above me". One spelling for two facts, so that
//     one value is written out concretely and nothing else is.
//   * A NO-OP SCOPE CARRIES NO ATTRIBUTES AT ALL, so anything selecting
//     `[data-noctua-scope]` under a frame — a reset, an assertion — skips
//     exactly the frames that stated nothing. Never write `getAttribute` in a
//     test; ask the pixels, or ask what is in force.
//
// WHERE A SETTINGS VALUE LANDS IS DECIDED BY WHERE THE CONTROL SITS. Inside a
// themed scope it themes that scope; inside an example frame it themes that
// frame's stage; a setting the model knows themes the document; anything else
// moves and no more. And a scoped control inverts what is in force WHERE IT
// SITS: `settingAt(element, name)` walks every scope above the element and is
// the only correct answer once anything can nest.
//
// A FRAME'S PARTS ARE ITS OWN, AND A NESTED FRAME IS NOT PART OF ITS PARENT.
// Every one of a frame's controls is resolved through the nearest
// `[data-nd-frame]`, because the frame's own documentation page puts one inside
// another. A `frame.querySelectorAll(…)` there reaches into the nested one, and
// a control that marks a stop it did not set is a control that lies.
//
// THE FRAME'S CONTROLS ARE A RAIL, AND THEIR AXIS CHANGES. The bar is an
// ordinary STATIC grid item, in flow, so the floating-surface allow-list does
// not apply to it; below the `sm` step the grid collapses to one column and the
// rail lies down into a row. `Toolbar` writes `aria-orientation` and this file
// reads it to choose the arrow keys, so THE MARKUP CANNOT NAME THE AXIS: it
// ships `horizontal`, the ARIA default and true at the narrow width a
// script-free reader is likeliest to be at, and `railAxis()` upgrades it to
// `vertical` when the stylesheet stands the rail up — reading the breakpoint
// off `--noctua-breakpoint-sm` rather than writing the length a second time.
//
// THE FOLDS ON A PAGE SHIP CLOSED, AND ONE OF THEM IS NOT LIKE THE OTHERS. This
// file closes any open NAMED `[data-nd-disclosure]` the pointer lands outside
// of, which is right for the header's panels and wrong for a section a reader
// opened on purpose — so the reference sections are rendered without a `name`.
// In a browser spec, call `showParts(page)` from `tests/browser/pages.js` after
// `goto`: a spec about what a component DOES is not a spec about the fold in
// front of it. AND A TEST THAT SKIPS WHAT IS HIDDEN IS A TEST A FOLD SWITCHES
// OFF — a floating-surface sweep skipping on `offsetParent` and a 320px sweep
// skipping on `display: none` both did nothing for a whole stage when these
// controls were folded behind a gear. Before folding anything, check what stops
// looking at it.

import {
  modeValues,
  accentValues,
  saturationValues,
  paletteValues,
  fontValues,
  glassValues,
  radiusValues,
  backdropValues,
  patternValues,
  grainValues,
  resolveScope,
  scopeAttributes,
  scopeAttributeNames,
  scopableSettings,
  stylesheetHref,
  scopeAttribute,
} from "./theme.js";
// The matcher. DOM-free, word-free and dependency-free, which is what lets it
// be a file of its own — and what will let it move into a behaviour package
// unchanged on the day this repository has one.
import { prepare, rank } from "./search-match.js";

// The camera, the keyboard walk and the switches a `Lineage` gains. A `dist/`
// file rather than one of this site's, because a navigation component whose
// consumer cannot navigate is a control that lies — and it fetches three.js
// itself, on the first interaction rather than on load.
import { enhanceLineages, releaseLineages } from "./lineage.js";
// Putting a fixed panel under the control it belongs to, and closing it when
// that control scrolls away. A `dist/` file for the same reason: `ColorPicker`
// ships its behaviour and its panel floats, so the rule cannot live here.
import { keepAnchored, releaseAnchor, releaseAnchorsIn } from "./anchor.js";
// Where a field's refusal goes. `Field` renders a visually hidden word in
// front of the sentence, so nothing may write the paragraph's `textContent`.
import { saySentence } from "./messages.js";
// The picker's whole behaviour, and a `dist/` file for the same reason: a
// colour picker that only picks colours inside this site is not one.
import { enhancePickers, releasePickers } from "./color-picker.js";
// The six input controls, and a `dist/` file for the same reason: a text box
// that only accepts text inside this site is not a text box.
import { enhanceInputs, releaseInputs } from "./inputs.js";
// The carousel: the two controls, the dots, the count, and the four rules
// autoplay carries. A `dist/` file for the same reason as the rest.
import { enhanceCarousels, releaseCarousels } from "./carousel.js";
// The summary at the top of a refused form: the focus, the jump to the
// control, and the list a client-side refusal makes.
import { enhanceErrorSummaries, releaseErrorSummaries } from "./error-summary.js";
import { enhanceTables, releaseTables } from "./table.js";
import { enhanceInfos, releaseInfos } from "./info.js";
import { enhanceMenus, releaseMenus } from "./menu.js";
// The one thing a message bar does with a script: reveal the control that
// takes it away, and take it away.
import { enhanceMessageBars, releaseMessageBars } from "./message-bar.js";
// The four floating surfaces, and `dist/` files for the reason the rest are:
// a `<dialog>` opens from a script and from nothing else, a popover and a
// tooltip both hang off `anchor.js`, and a toast is a timer plus a live region.
import { enhanceDialogs, releaseDialogs } from "./dialog.js";
import { enhancePopovers, releasePopovers } from "./popover.js";
import { enhanceTooltips, releaseTooltips } from "./tooltip.js";
import { enhanceToasts, releaseToasts } from "./toast.js";
// These two are THIS SITE'S, never `dist/`'s: the modelled owl and the
// model stage are examples of what a consumer may put inside a figure, and
// a consumer of the packages never fetches a byte of either.
import { enhanceModels, releaseModels } from "./model.js";

const root = document.documentElement;
const base = root.getAttribute("data-noctua-base") || "";

/** Every disclosure this script owns, by the hook its `name` prop writes. */
const MENU = "[data-nd-disclosure]";

/** The header's drawer trigger, by the hook its `action` prop writes. */
const MENU_BUTTON = '[data-nd-action="menu"]';

/** The appearance panel. */
const settings = () => document.querySelector("[data-nd-disclosure='settings']");

// The language layer's three constants. The offered tags are read from the
// document rather than written here, so a locale added to `spec/i18n/` reaches
// this file with nothing to edit; the key and the sentinel are asserted against
// the Rust side by `the_locale_layer_agrees_with_the_script` in `main.rs`,
// because a storage key spelled two ways is a preference that never comes back.
const LOCALE_KEY = "noctua-locale";
const AUTOMATIC_LOCALE = "auto";
const offeredLocales = (root.getAttribute("data-noctua-locales") || "")
  .split(" ")
  .filter(Boolean);

/** Every setting, keyed by the name the markup uses. Generated, not written. */
const model = {
  mode: modeValues,
  accent: accentValues,
  saturation: saturationValues,
  palette: paletteValues,
  font: fontValues,
  glass: glassValues,
  radius: radiusValues,
  backdrop: backdropValues,
  pattern: patternValues,
  grain: grainValues,
};

/** The settings a reader picks directly. `palette` is derived from two of them. */
const chosen = Object.keys(model).filter((name) => name !== "palette");

// --- Storage ---------------------------------------------------------------

// Every read is validated against the generated value list before it becomes an
// attribute or an href. A stored value reaches both, and "whatever was in
// localStorage" is not a thing to interpolate into a URL.
function read(name) {
  try {
    const stored = localStorage.getItem(model[name].key);
    return model[name].values.includes(stored) ? stored : model[name].default;
  } catch {
    return model[name].default;
  }
}

function store(name, value) {
  try {
    localStorage.setItem(model[name].key, value);
  } catch {
    // Private browsing. The setting still applies to this page; it just will
    // not survive a reload, which is the correct degradation.
  }
}

// --- The document ----------------------------------------------------------

/** Applies one setting to `<html>` and remembers it. */
function apply(name, value) {
  // A `Segmented` the theme model knows nothing about is not this handler's
  // business — the example frame has two, and they are page state rather than
  // a stored preference. Without this guard `model[name]` is `undefined` and
  // every click on one throws.
  if (!model[name]?.values.includes(value)) return;
  store(name, value);
  stamp(root, name, value);

  // `palette` is stored resolved rather than derived at read time, because the
  // inline bootstrap runs before any module and cannot look up the grid. It
  // needs the answer, not the inputs.
  if (name === "accent" || name === "saturation") {
    const palette = `${read("accent")}-${read("saturation")}`;
    if (paletteValues.values.includes(palette)) {
      apply("palette", palette);
    }
  }
  if (name === "palette") {
    ensureStylesheet(value);
    // A PALETTE *IS* AN ACCENT AND A SATURATION, so anything that chooses one
    // directly has to set both. Storing only the palette leaves the appearance
    // panel marking an accent the page is not using, and the next click on a
    // saturation then resolves against that stale accent and throws the
    // reader's choice away.
    //
    // The split is unambiguous: no accent contains a hyphen, so `<accent>` is
    // everything before the first one. Both halves are validated against the
    // generated lists before they are stored.
    const cut = value.indexOf("-");
    const accent = value.slice(0, cut);
    const saturation = value.slice(cut + 1);
    if (
      accentValues.values.includes(accent) &&
      saturationValues.values.includes(saturation)
    ) {
      store("accent", accent);
      store("saturation", saturation);
    }
  }
  refresh();
}

function stamp(element, name, value) {
  const attribute = model[name].attribute;
  if (attribute == null) return;
  // `system` is the ABSENCE of `data-theme`, not a value of it: the cascade
  // resolves the system preference, and no stylesheet understands the word.
  if (model[name].omitWhen != null && value === model[name].omitWhen) {
    element.removeAttribute(attribute);
  } else {
    element.setAttribute(attribute, value);
  }
}

const loaded = new Set();

/**
 * Links a palette's stylesheet, once.
 *
 * `blocking="render"` is set BEFORE insertion, because a stylesheet inserted by
 * script is not render-blocking by default — set it after and the page paints
 * in the old palette and snaps over when the file lands. Appended last, which
 * is also what wins the (0,3,0) tie against the base sheet in system mode.
 */
function ensureStylesheet(palette) {
  if (loaded.has(palette) || !paletteValues.values.includes(palette)) return;
  loaded.add(palette);
  const link = document.createElement("link");
  link.rel = "stylesheet";
  link.dataset.ndPalette = palette;
  link.setAttribute("blocking", "render");
  link.href = base + stylesheetHref(palette);
  document.head.appendChild(link);
}

// --- The settings panel ----------------------------------------------------

/**
 * Marks the buttons that match what is currently in force.
 *
 * "In force" is answered where the group SITS, exactly as choosing is: a group
 * inside a themed scope shows that scope's value, not the document's. Without
 * this the demonstration page's controls reported the page — a scope built with
 * `mode: Light` and `radius: Round` showed `system` and `soft` marked, which is
 * a control lying about the thing it controls. It was invisible while those
 * controls were cycling icons that showed no state at all.
 */
function refresh() {
  for (const group of document.querySelectorAll("[data-nd-group]")) {
    const name = group.dataset.ndGroup;
    if (!model[name]) continue;
    const target = themedBy(group);
    const current = target ? settingAt(target, name) : read(name);
    // A switch IS the group — it carries the hook itself — so it marks its own
    // state; anything else marks the option that matches.
    if (group.getAttribute("role") === "switch") {
      group.setAttribute("aria-checked", String(current === group.dataset.ndOn));
      continue;
    }
    for (const button of group.querySelectorAll("[data-nd-value]")) {
      button.setAttribute(
        "aria-checked",
        String(button.dataset.ndValue === current),
      );
    }
  }
  // BEFORE ANYTHING IS READ. A sparse request means `settingAt` and
  // `inheritedAt` answer from what is stamped right now, so marking controls
  // against a half-stamped document reads stale state. It used to run last,
  // which was harmless only while every request was absolute.
  resolveScopes();

  // THE FRAME'S QUICK TOGGLE AND THE FRAME'S MENU SHOW ONE STATE.
  //
  // They drive the same request, so the toggle's `aria-pressed` is a question
  // about the stage rather than a flag the toggle keeps for itself: pressed
  // exactly when the stage is showing something other than what it inherits.
  for (const toggle of document.querySelectorAll('[data-nd-action="frame-mode"]')) {
    const frame = toggle.closest("[data-nd-frame]");
    const stage = frame && ownStage(frame);
    if (!stage) continue;
    // SHOWING SOMETHING OTHER THAN THE PAGE — not merely "has asked for a
    // mode". A reader who picks `system` in the frame's menu on a page that is
    // already following the system has asked for nothing, and a toggle that
    // reported itself pressed there would be reporting a difference that is not
    // on the screen.
    //
    // Against WHAT IT INHERITS, not against the page. A nested frame at rest
    // shows the frame around it, so a toggle comparing it with the page would
    // report a difference the reader cannot see — and would read as pressed on
    // every nested frame the moment its parent was themed. Both sides through
    // `concrete`, because `system` on a page already following the system is a
    // reader who has asked for nothing.
    const forced =
      concrete("mode", settingAt(stage, "mode")) !== concrete("mode", inheritedAt(stage).mode);
    // Against the BOOLEAN, not the string: `aria-pressed` is absent before the
    // first press, and `String(false) !== null` would read as a change and swap
    // the control's name on load.
    const was = toggle.getAttribute("aria-pressed") === "true";
    if (forced !== was) swapLabel(toggle, forced);
  }
  frostFrames();
  refreshRoving();
}

/**
 * A frame is frosted by whatever it is sitting on.
 *
 * ASKED AT THE LAYER, NOT AT THE PAGE. This read the page's `glass` for every
 * `.nd-frame-glass` in the document, so a frame nested in a stage whose reader
 * had turned glass off kept a frosted header — the second half of the defect
 * this file's scope model exists to fix, and the half that survives every fix
 * to the first. `settingAt` resolves at an arbitrary element, so for a layer
 * inside an inner frame it answers with the outer stage's choice and for a
 * frame on the page it still answers with the page's.
 *
 * A frame a RIG owns is left alone: `setGlassTest` remembers what it found and
 * has to be the only writer, or turning the rig off would hand the frame back
 * to a value this function had already overwritten.
 */
function frostFrames() {
  for (const layer of document.querySelectorAll(".nd-frame-glass")) {
    if (layer.dataset.ndGlassWas !== undefined) continue;
    if (settingAt(layer, "glass") !== GLASS_OFF) layer.setAttribute("data-nd-glass", "true");
    else layer.removeAttribute("data-nd-glass");
  }
}

/**
 * What one setting resolves to at a scope, asked and inherited together.
 *
 * `accent` is not a scope attribute — the palette is — so it is read back out
 * of whichever palette the scope ended up with, the same way `themeScope`
 * writes it.
 */
function settingAt(element, name) {
  const inherited = inheritedAt(element);
  const theme = resolveScope(requestAt(element, inherited), inherited);
  if (name === "accent") {
    const palette = theme.palette ?? read("palette");
    return palette.slice(0, palette.indexOf("-"));
  }
  return theme[name] ?? read(name);
}

/** Moves the chosen state within one group, and nothing else. */
function choose(group, button) {
  for (const other of group.querySelectorAll("[data-nd-value]")) {
    other.setAttribute("aria-checked", String(other === button));
  }
  refreshRoving();
}

/** Sends one setting wherever the control that changed it belongs. */
function route(control, name, value) {
  const target = themedBy(control);
  if (target) {
    themeScope(target, name, value);
    refresh();
  } else {
    apply(name, value);
  }
}

// ONE HANDLER FOR EVERY SETTINGS CONTROL, AND FOUR ANSWERS.
//
// Where a chosen value goes is decided by where the control SITS, which is the
// only rule that stays true as pages are written:
//
//   1. Inside a themed scope   → it themes that scope, and nothing outside it.
//   2. Inside an example frame → it themes that frame's stage, which becomes a
//                                scope the first time something asks.
//   3. A setting the model has → it themes the document, and is remembered.
//   4. Anything else           → it moves, and that is all.
//
// The second case is the theme menu inside a frame: it sits in the toolbar,
// which is deliberately OUTSIDE the stage it themes, so "inside a scope" cannot
// answer for it. The same rule the frame's own handlers use — the nearest
// `[data-nd-frame]` decides — answers it exactly.
//
// The fourth is not a fallback, it is a feature. Every radio group in an
// example used to be inert: `apply()` did not recognise the name, returned
// early, and the demo sat there refusing to respond to a reader trying it out.
// A control on a page about controls has to work.

/**
 * Which element a settings control themes, or `null` for the document.
 *
 * THE NEAREST OF THE TWO, NOT THE SCOPE FIRST.
 *
 * Asking for the nearest scope and only then for a frame reads the tree in the
 * wrong order the moment a frame sits inside a scope — which is exactly a frame
 * inside another frame's themed stage. The inner frame's own menu found the
 * OUTER stage first and themed it, so the nested example drove its parent.
 *
 * One query for both, so whichever is nearer decides, which is the rule as
 * stated: where the control sits.
 */
function themedBy(group) {
  const owner = group.closest(`[${scopeAttribute}], [data-nd-frame]`);
  if (!owner) return null;
  if (!owner.matches("[data-nd-frame]")) return owner;
  return ownStage(owner) ?? null;
}

document.addEventListener("click", (event) => {
  // A SWITCH IS THE SAME SETTING, SAID AS ONE CONTROL.
  //
  // It carries both values, so the script sends whichever is not in force and
  // never composes one. Everything else about it — where the value goes, how
  // the state is marked — is the routing below.
  const flip = event.target.closest?.('[role="switch"][data-nd-group]');
  if (flip) {
    if (flip.getAttribute("aria-disabled") === "true") return;
    const on = flip.getAttribute("aria-checked") === "true";
    const name = flip.dataset.ndGroup;
    // THE FOURTH ANSWER, WHICH THE SWITCH DID NOT HAVE. `route` sends a
    // setting somewhere; a switch the model knows nothing about has nowhere to
    // send it, so `themeScope` returned early, `apply` returned early and
    // `refresh` skipped it. Every one of the eight switches on the switch's
    // own page moved nothing at all, which is a control on a page about
    // controls that does not work. The `Segmented` below has had this branch
    // since it was written, for exactly the same reason.
    if (model[name]) {
      route(flip, name, on ? flip.dataset.ndOff : flip.dataset.ndOn);
    } else {
      flip.setAttribute("aria-checked", String(!on));
    }
    return;
  }

  const button = event.target.closest?.("[data-nd-value]");
  if (button) {
    const group = button.closest("[data-nd-group]");
    if (group) {
      // A refused item is reachable on purpose; it must still refuse.
      if (button.getAttribute("aria-disabled") === "true") return;
      const name = group.dataset.ndGroup;
      if (model[name]) {
        route(group, name, button.dataset.ndValue);
      } else {
        choose(group, button);
      }
      return;
    }
  }

  const cycle = event.target.closest?.("[data-nd-cycle]");
  if (cycle) {
    const name = cycle.dataset.ndCycle;
    const values = model[name].values;
    apply(name, values[(values.indexOf(read(name)) + 1) % values.length]);
    return;
  }

  if (event.target.closest?.('[data-nd-action="reset"]')) {
    for (const name of Object.keys(model)) {
      try {
        localStorage.removeItem(model[name].key);
      } catch {
        // As above: nothing to clear if nothing could be stored.
      }
      stamp(root, name, model[name].default);
    }
    refresh();
    return;
  }

  // Close any open disclosure the click landed outside of. `<details>` has no
  // such behaviour of its own, and a disclosure that stays open behind the page
  // you just clicked into reads as stuck.
  for (const panel of document.querySelectorAll(`${MENU}[open]`)) {
    if (!panel.contains(event.target)) panel.removeAttribute("open");
  }
});

// The accent swatches each need their own palette's stylesheet to show their
// own color. Fetched when the panel is first opened, and never on a page view
// that does not open it.
//
// THE HOOK IS A PROP, NOT A HAND-WRITTEN ATTRIBUTE. It used to be
// `[data-nd-groups]`, written directly on a hand-built `<details>`. When the
// panel became the generated `Disclosure` the attribute went with it, this
// `querySelector` returned `null`, and `?.` swallowed it without a word: no
// swatch was ever given a `data-palette`, so all thirteen painted the reader's
// own accent, and the panel stopped closing on an outside click at the same
// time. `Disclosure.name` is now the hook, the way `Choice.name` and
// the search field's own `name` already were.
/**
 * Gives every accent swatch its own palette, and fetches the sheet that
 * defines it.
 *
 * The swatch names an ACCENT; a stylesheet names a PALETTE, which is an accent
 * and a saturation. So the file a swatch needs changes with the reader's
 * saturation, and repainting on that change is not a refinement: `ochre-vivid`
 * is a different color from `ochre-balanced`, and a swatch left on the old one
 * offers a color the reader is no longer choosing between.
 */
function paintSwatches() {
  const saturation = read("saturation");
  // The chip is a `Swatch` inside the accent group's item, and the item's
  // `data-nd-value` IS the accent — so the accent is read from the control's
  // own contract rather than from a second attribute invented for the script.
  // It used to be `data-nd-swatch`, on hand-written markup, which is one more
  // place the same fact was written.
  for (const item of document.querySelectorAll("[data-nd-group='accent'] [data-nd-value]")) {
    const swatch = item.querySelector(".nd-swatch");
    if (!swatch) continue;
    const palette = `${item.dataset.ndValue}-${saturation}`;
    if (!paletteValues.values.includes(palette)) continue;
    swatch.setAttribute("data-palette", palette);
    ensureStylesheet(palette);
  }
}

// EVERY panel that has an accent row, not just the header's.
//
// This was bound to the settings disclosure alone, which was every panel there
// was. The example frame opens the same menu now, and its thirteen swatches
// painted the reader's own accent — thirteen identical chips — because the
// sheets that define them are fetched on open and its open was not this one.
for (const panel of document.querySelectorAll(MENU)) {
  if (!panel.querySelector("[data-nd-group='accent']")) continue;
  panel.addEventListener("toggle", (event) => {
    if (event.target.open) paintSwatches();
  });
}

document.addEventListener("click", (event) => {
  if (event.target.closest?.("[data-nd-group='saturation']")) paintSwatches();
});

// --- Anchoring a floating panel --------------------------------------------

// IT IS `dist/anchor.js` NOW, and the whole block moved unchanged. `ColorPicker`
// ships its behaviour, a shipped panel cannot import from this directory, and a
// second copy of the off-screen test is exactly what the note it carries warned
// about. The three call sites below did not move.

// The frame's theme menu is the first of the three call sites.
const FRAME_MENU = "[data-nd-disclosure='frame-theme']";

for (const menu of document.querySelectorAll(FRAME_MENU)) {
  menu.addEventListener("toggle", () => {
    const panel = menu.querySelector(".nd-panel");
    const summary = menu.querySelector("summary");
    if (!panel || !summary) return;
    if (menu.open) {
      keepAnchored(panel, summary, {}, () => menu.removeAttribute("open"));
    } else {
      releaseAnchor(panel);
    }
  });
}

// --- The controls on a code block ------------------------------------------

// The component renders the buttons and does the highlighting; this makes the
// buttons work, because the design system ships no runtime and what a consumer
// of the packages gets is the markup.
//
// EACH OF THE THREE DEGRADES TO SOMETHING CORRECT when this never runs: no copy
// button rather than a dead one, no wrap toggle rather than a stuck one, and
// the whole block visible rather than a truncated one with no way to open it.
// The last is why the cap is applied here rather than in the stylesheet — a CSS
// `max-block-size` would hide the tail of every long block on a page whose
// script was blocked, and nothing would say so.
//
// THE FIRST TWO ARE KEPT BY THE STYLESHEET, AND FOR A LONG TIME WERE NOT. Only
// the expand button was gated on `:root[data-nd-scripting]`; the copy and the
// wrap toggle shipped visible and focusable on every page, doing nothing, while
// this paragraph said otherwise. `.nd-codeblock-control` carries the gate now,
// which is the same one `.nd-lineage-camera`, `.nd-dialog` and `.nd-popover`
// use. A sentence about degradation is not a mechanism.

document.addEventListener("click", async (event) => {
  const button = event.target.closest?.("[data-nd-copy]");
  if (!button) return;

  // `textContent`, never markup, and never anything but the code the reader can
  // already see. The component renders a real newline beside each line and
  // draws the line numbers with a counter, so this is the source byte for byte.
  const code = button
    .closest(".nd-codeblock-figure")
    ?.querySelector(".nd-codeblock-code");
  if (!code) return;

  try {
    await navigator.clipboard.writeText(code.textContent);
  } catch {
    // No clipboard permission, or an insecure origin. Saying nothing is the
    // correct degradation: claiming a copy that did not happen is worse than a
    // button that appears not to have worked.
    return;
  }

  button.setAttribute("data-nd-done", "");
  // And the half a screen reader gets. The word is the component's `copied`
  // prop, because this package has no business inventing one in the reader's
  // language.
  const status = button.parentElement?.querySelector("[data-nd-copy-status]");
  if (status) status.textContent = button.dataset.ndCopied ?? "";

  setTimeout(() => {
    button.removeAttribute("data-nd-done");
    if (status) status.textContent = "";
  }, 2000);
});

document.addEventListener("click", (event) => {
  const button = event.target.closest?.("[data-nd-wrap-toggle]");
  if (!button) return;
  const block = button.closest(".nd-codeblock-figure")?.querySelector(".nd-codeblock");
  if (!block) return;
  const wrapped = block.getAttribute("data-nd-wrap") !== "true";
  block.setAttribute("data-nd-wrap", String(wrapped));
  button.setAttribute("aria-pressed", String(wrapped));
});

document.addEventListener("click", (event) => {
  const button = event.target.closest?.("[data-nd-expand]");
  if (!button) return;
  const block = button.closest(".nd-codeblock-figure")?.querySelector(".nd-codeblock");
  if (!block) return;
  const open = block.getAttribute("data-nd-collapsed") === "true";
  block.setAttribute("data-nd-collapsed", String(!open));
  button.setAttribute("aria-expanded", String(open));
  // The two names are two sentences, not one with a word swapped: "show all"
  // and "show less" are not one word apart in every language.
  const other = button.dataset.ndCollapseLabel;
  if (other) {
    button.dataset.ndCollapseLabel = button.textContent;
    button.textContent = other;
  }
  // Sending the reader back to the top of a block they just closed, and only
  // when it is off screen — scrolling a block that is already in view is motion
  // for nothing.
  if (!open && block.getBoundingClientRect().top < 0) {
    block.scrollIntoView({ block: "nearest", behavior: "auto" });
  }
});

// The cap itself. It is a class of enhancement this file is otherwise full of:
// read what the markup declared, and add the behaviour it described.
//
// IT TAKES A ROOT LIKE EVERY OTHER ENHANCER, AND THAT FIXES A DEFECT OF ITS
// OWN. Applying it per element is idempotent, but running it over the whole
// page a second time RE-COLLAPSES a block the reader had already expanded. The
// reader's answer to the control is undone by an enhancement pass somewhere
// else on the page. Scoping it to the subtree that changed means only that
// subtree is judged again.
function collapseBlocks(root = document) {
  for (const figure of root.querySelectorAll('[data-nd-collapsible="true"]')) {
    const block = figure.querySelector(".nd-codeblock");
    const button = figure.querySelector("[data-nd-expand]");
    if (!block || !button) continue;
    const lines = block.querySelectorAll(".nd-codeblock-line").length;
    const maximum = Number.parseInt(getComputedStyle(block).getPropertyValue("--nd-code-max"), 10);
    // A block already shorter than its cap is not collapsed, and gets no
    // control — offering to expand something fully visible is a control that
    // lies about what it does.
    if (!Number.isNaN(maximum) && lines > maximum) {
      block.setAttribute("data-nd-collapsed", "true");
    }
  }
}

// --- Checkboxes and radios -------------------------------------------------

// TWO BEHAVIOURS, AND NEITHER OF THEM IS THE CONTROL.
//
// The checking, the keyboard, the grouping and the value that gets submitted
// are the browser's, because these are real `<input>` elements. What is left is
// the pair of things HTML has no way to say: a *none of the above* that clears
// its neighbours, and a block that appears only when the choice above it is
// chosen. Both are behaviour, so both live here — the design system ships no
// runtime, and what the components ship is the attributes this keys on.
//
// Both degrade the right way. With this file blocked, the exclusive option is
// simply another checkbox and the conditional block is simply visible; nothing
// is unreachable and nothing lies about its state.

/** The attribute the stylesheet hides a conditional block by. */
const REVEALED = "data-nd-revealed";

/**
 * Every choice that answers the same question as this one.
 *
 * The group is the fieldset the question is asked in, or the form it is
 * submitted with — never the whole document. The Forms page renders nine groups
 * on one page and `name` alone would make several of them one.
 */
function siblingChoices(input) {
  const within = input.closest("fieldset, form") ?? document;
  return [...within.querySelectorAll(`.nd-choice-input[name="${CSS.escape(input.name)}"]`)];
}

/**
 * The none-of-the-above.
 *
 * Checking it clears every other box in the group; checking any other clears
 * it. That is the one combination a list of checkboxes cannot express by
 * itself, and the reason `Checkbox { exclusive }` exists at all.
 */
function applyExclusive(input) {
  const group = siblingChoices(input);
  if (input.dataset.ndExclusive === "true") {
    if (!input.checked) return;
    for (const other of group) if (other !== input) other.checked = false;
    return;
  }
  if (!input.checked) return;
  for (const other of group) {
    if (other.dataset.ndExclusive === "true") other.checked = false;
  }
}

/**
 * Shows or hides the block a choice reveals, and says so on the control.
 *
 * `aria-expanded` is written HERE and nowhere else, which is deliberate: with
 * this file blocked the attribute is absent, the block is visible, and a
 * control announcing `collapsed` over something a reader can see never happens.
 */
function refreshReveal(input) {
  if (input.dataset.ndReveal !== "true") return;
  const block = document.getElementById(`${input.id}-reveal`);
  if (!block) return;
  block.setAttribute(REVEALED, String(input.checked));
  input.setAttribute("aria-expanded", String(input.checked));
  input.setAttribute("aria-controls", block.id);
}

// The whole group, not only the control that changed: choosing one radio
// unchecks another and fires no event on it, so the block the old answer
// revealed would stay open under an answer nobody gave any more.
document.addEventListener("change", (event) => {
  const input = event.target.closest?.(".nd-choice-input");
  if (!input) return;
  applyExclusive(input);
  for (const other of siblingChoices(input)) refreshReveal(other);
});

/** The first pass, before anybody has touched anything. */
function refreshChoices() {
  for (const input of document.querySelectorAll(".nd-choice-input")) refreshReveal(input);
}

// --- The select, enhanced into a listbox -----------------------------------

// WHAT THIS ADDS, AND WHAT IT IS FORBIDDEN TO TAKE AWAY.
//
// The control under all of this is a real `<select>` with real `<option>`s in
// it. Everything below reads its state from that select and writes every choice
// back into it — so the value that reaches a server is the browser's own, and a
// page whose script failed to load is a page that still submits. The usual
// alternative, a `role="listbox"` beside a hidden `<input>`, is where this whole
// category of component gets its signature bug: a control that looks right and
// submits nothing.
//
// EVERY WORD comes off a `data-nd-*` attribute the component wrote. §9.9 forbids
// the design system from carrying text in any language and this file is not a
// loophole — it is the same file, one build later.
//
// EVERY DRAWING is cloned from a `<template>` the markup already carries. §9.5
// says icons come from one place; a script that carried an SVG would be a
// second one, and the second one is the one nobody audits.
//
// THE SECURITY QUESTION, ANSWERED IN WRITING (`NOCTUA.md §2.4).
//
// Three places here take a string that did not come from this build, and each
// is closed deliberately rather than by luck:
//
//   * `creatable` puts what a reader typed into the page. It goes in through
//     `value` and `textContent` on an element this file constructs — never
//     `innerHTML`, never a string of markup — so a typed `<script>` is a typed
//     `<script>`: eleven characters in a list, and nothing else.
//   * `data-nd-swatch` becomes a stylesheet request, and a request built from
//     an attribute is a request an attribute can aim. `ensureStylesheet` is
//     called only for a name the generated `paletteValues` already knows, so
//     the set of URLs this can fetch is the set the build shipped.
//   * `data-nd-icon` and `name` are interpolated into selectors, through
//     `CSS.escape`. A selector is not a sink on its own; an unescaped one is a
//     query that silently matches the wrong element, which is the quieter bug.

const SELECT_FIELD = "[data-nd-select]";

/** Everything one enhanced field needs to know about itself. */
const selects = new WeakMap();

/** The one open list. Two would be two things claiming the arrow keys. */
let openSelect = null;

/**
 * The one open floating list on the page, whatever kind it is.
 *
 * `openSelect` was a module-level global, which was right while `Select` was
 * the only thing with a list. A second one — a `TagPicker`'s suggestions — is a
 * second global, and then two panels are open at once, both claiming Escape and
 * both claiming the arrow keys. One registry, and whoever opens closes the
 * other.
 */
const openPanels = new Set();

/** Closes every open list except this one's. */
function claimOpenPanel(close) {
  for (const other of [...openPanels]) {
    if (other !== close) {
      openPanels.delete(other);
      other();
    }
  }
  openPanels.add(close);
}

/** Gives it up again. Safe to call when it was never held. */
function releaseOpenPanel(close) {
  openPanels.delete(close);
}

/** How long a run of typed letters counts as one word, with no filter box. */
const TYPE_AHEAD = 700;

function enhanceSelects(root = document) {
  for (const field of root.querySelectorAll(SELECT_FIELD)) enhanceSelect(field);
}

/**
 * Takes the behaviour back off every enhanced field under `root`.
 *
 * A BEHAVIOUR THAT CANNOT BE RELEASED IS A DEFECT FOR EVERY CONSUMER WITH A
 * CLIENT-SIDE ROUTER. What a `Select` leaves behind is not a listener on the
 * document, because the outside press is one delegated listener for the whole
 * page. It is three module-level references to a field no longer on it:
 * `openSelect`, the `openPanels` registry, and `anchor.js`'s own map.
 */
function releaseSelects(root = document) {
  for (const field of root.querySelectorAll(SELECT_FIELD)) {
    const state = selects.get(field);
    if (!state) continue;
    if (openSelect === state) openSelect = null;
    if (state.close) releaseOpenPanel(state.close);
    releaseAnchor(state.panel);
    state.binding.abort();
    selects.delete(field);
  }
}

function enhanceSelect(field) {
  if (selects.has(field)) return;
  const binding = new AbortController();
  const signal = binding.signal;
  const native = field.querySelector(".nd-select-native");
  const box = field.querySelector(".nd-select-field");
  // `Field` draws the question now, so it is a sibling of `.nd-select` rather
  // than a child of it — and it carries `{control}-label`, which is what the
  // enhanced control points `aria-labelledby` at.
  const label = field.closest(".nd-field")?.querySelector(".nd-label");
  if (!native || !box || !label || !native.id) return;

  const id = native.id;
  const searchable = field.dataset.ndSearchable === "true";
  // `creatable` NEEDS A FILTER BOX, AND USED TO ACCEPT ONE WITHOUT. The offer
  // to add is a row inside the filtered list and `create()` reads the box, so
  // without one the attribute reached nothing and would have thrown if it had.
  // Read here rather than at each use, so the dependency is stated once.
  const creatable = searchable && field.dataset.ndCreatable === "true";
  label.id = `${id}-label`;

  // THE NATIVE CONTROL IS HIDDEN, NOT `aria-hidden`.
  //
  // `hidden` takes it out of the accessibility tree AND out of the tab order in
  // one move, with no focusable element left claiming to be invisible — which
  // is what `aria-hidden` on a tabbable control is, and what axe calls
  // `aria-hidden-focus`. A hidden form control still submits: that is the whole
  // arrangement.
  native.hidden = true;

  const control = document.createElement("button");
  control.type = "button";
  control.id = `${id}-control`;
  control.className = "nd-select-control";
  control.disabled = native.disabled;
  control.setAttribute("aria-haspopup", "listbox");
  control.setAttribute("aria-expanded", "false");
  control.setAttribute("aria-controls", `${id}-listbox`);
  // Named by the field's own label and by its own text, which is the answer.
  // That is APG's select-only combobox, and it is why the reference to itself
  // is deliberate rather than a mistake.
  control.setAttribute("aria-labelledby", `${label.id} ${control.id}`);
  const described = native.getAttribute("aria-describedby");
  if (described) control.setAttribute("aria-describedby", described);
  // Only the button IS the combobox when there is no filter box. With one,
  // focus moves into the input and the input carries the role — a `combobox`
  // that does not have focus cannot own `aria-activedescendant`.
  if (!searchable) control.setAttribute("role", "combobox");
  // AN ANSWER IS COMPULSORY, AND THE ENHANCED CONTROL NEVER SAID SO. The
  // native `<select>` carries the real `required` and is `hidden`, so the only
  // thing a reader can reach was a plain button with nothing on it — the
  // string `aria-required` did not exist anywhere in this repository, under a
  // contract promising it follows from the prop. `required` is not an
  // attribute a `<button>` has, so this is the one way to state the fact.
  if (native.required) control.setAttribute("aria-required", "true");
  const value = document.createElement("span");
  value.className = "nd-select-value";
  control.append(value);
  // The label pointed at the control that is now hidden. A `<button>` is a
  // labelable element, so it can take the same label rather than needing a
  // second copy of the words.
  label.htmlFor = control.id;
  box.prepend(control);

  const panel = document.createElement("div");
  panel.className = "nd-panel nd-select-panel";
  panel.hidden = true;

  let search = null;
  if (searchable) {
    search = document.createElement("input");
    search.type = "text";
    search.className = "nd-select-search";
    search.setAttribute("role", "combobox");
    search.setAttribute("aria-expanded", "true");
    search.setAttribute("aria-controls", `${id}-listbox`);
    search.setAttribute("aria-autocomplete", "list");
    search.autocomplete = "off";
    search.spellcheck = false;
    if (field.dataset.ndSearchLabel) search.setAttribute("aria-label", field.dataset.ndSearchLabel);
    // The filter box is the combobox when there is one, so the fact moves with
    // the role rather than being stated on the element beside it.
    if (native.required) search.setAttribute("aria-required", "true");
    panel.append(search);
  }

  const list = document.createElement("ul");
  list.className = "nd-select-list";
  list.id = `${id}-listbox`;
  list.setAttribute("role", "listbox");
  if (native.multiple) list.setAttribute("aria-multiselectable", "true");
  panel.append(list);

  // One line for three opposite facts — nothing matched, nothing has arrived
  // yet, and here is what typing it would add — because they never co-occur and
  // three empty paragraphs would be three things to keep hidden.
  const note = document.createElement("p");
  note.className = "nd-select-note";
  panel.append(note);

  box.append(panel);
  box.dataset.ndEnhanced = "true";

  const state = {
    // CARRIED ON THE STATE, because every helper below already takes it. A
    // parallel argument threaded through a dozen call sites is a dozen places
    // for one of them to be forgotten.
    binding,
    signal,
    field,
    native,
    box,
    label,
    control,
    panel,
    list,
    note,
    search,
    creatable,
    id,
    rows: [],
    active: -1,
    typed: "",
    typedAt: 0,
    // Read once, before anything can change it: whether the sentence in the
    // error slot is the caller's. If it is, the script neither writes over it
    // nor clears it — a server's answer is not the browser's to withdraw.
    saidByCaller: field.dataset.ndInvalid === "true",
  };
  selects.set(field, state);

  control.addEventListener("click", () => (state.panel.hidden ? openList(state) : closeList(state)), { signal });
  control.addEventListener("keydown", (event) => onSelectKey(state, event), { signal });
  if (search) {
    search.addEventListener("keydown", (event) => onSelectKey(state, event), { signal });
    search.addEventListener("input", () => {
      applyFilter(state);
      moveTo(state, firstVisible(state));
    }, { signal });
  }
  list.addEventListener("click", (event) => {
    const item = event.target.closest?.(".nd-select-item");
    if (!item || item.getAttribute("aria-disabled") === "true") return;
    takeOption(state, state.rows.findIndex((row) => row.item === item));
  }, { signal });
  // A pointer moving over the list moves the highlight with it, so the keyboard
  // and the mouse never disagree about which option Enter would take.
  list.addEventListener("pointermove", (event) => {
    const item = event.target.closest?.(".nd-select-item");
    if (item) moveTo(state, state.rows.findIndex((row) => row.item === item), false);
  }, { signal });
  const clear = box.querySelector("[data-nd-select-clear]");
  if (clear) {
    clear.addEventListener("click", () => {
      for (const option of state.native.options) option.selected = false;
      // A single select always has SOMETHING selected as far as the DOM is
      // concerned, so clearing means choosing the empty option if there is one.
      if (!state.native.multiple) state.native.selectedIndex = -1;
      announce(state);
      paint(state);
      state.control.focus();
    }, { signal });
  }
  // Something else changed the answer — a second control, a test.
  native.addEventListener("change", () => paint(state), { signal });
  wireForm(state);

  rebuild(state);
  paint(state);
}

/**
 * The two things a form does to a control that it does not announce.
 *
 * A RESET FIRES NO `change`. `form.reset()` dispatches one `reset` event on the
 * form and nothing at all on the controls inside it, so the listener above
 * never runs and the enhanced control goes on showing an answer the native
 * select no longer holds. Measured: after a reset the control read
 * `ochre-balanced` while the select held nothing.
 *
 * And the event fires BEFORE the reset happens, and is cancelable — so the
 * repaint has to be deferred by a task, not done in the handler.
 *
 * A REQUIRED CONTROL THAT IS `display: none` CANNOT BE REPORTED. The browser
 * has to focus an invalid control to show its bubble; `native.hidden` means it
 * cannot, so Chrome logs *"An invalid form control with name='…' is not
 * focusable"* and the submit silently does nothing, and Firefox blocks it with
 * nothing said at all. `Select { multiple, required }` has shipped in the
 * examples the whole time and was never seen, because no example is inside a
 * `<form>`. The validity stays on the native control — it is the value, and it
 * is what a server-side check would agree with — and the REPORT moves to the
 * button, which is the focusable thing the reader can actually see.
 */
function wireForm(state) {
  const form = state.native.form;
  if (!form) return;

  form.addEventListener("reset", () => {
    setTimeout(() => {
      for (const option of [...state.native.options]) {
        if (option.dataset.ndCreated !== undefined) option.remove();
      }
      rebuild(state);
      paint(state);
    }, 0);
  }, { signal: state.signal });

  // `invalid` does not bubble, so it is caught on the way down.
  form.addEventListener(
    "invalid",
    (event) => {
      if (event.target !== state.native) return;
      // The browser's attempt to show a bubble is what logs the error, and it
      // was never going to succeed. Refuse it, and say the same sentence in the
      // place this component already says sentences — the browser's own
      // `validationMessage`, so it is in the reader's language without this
      // file carrying a word of it.
      event.preventDefault();
      if (state.saidByCaller) return;
      const slot = state.field.closest(".nd-field")?.querySelector("[data-nd-field-error]");
      saySentence(slot, state.native.validationMessage);
      state.field.dataset.ndInvalid = "true";
      state.control.setAttribute("aria-invalid", "true");
      // And focus lands on the control the reader can see, which is the whole
      // thing the browser could not do. Deferred by a task: the browser makes
      // its own attempt to focus the invalid control after this event returns,
      // fails, and leaves focus wherever it was — so a synchronous call here
      // is overwritten by a failure.
      //
      // The control, not `focusHolder`: the filter box is inside a panel that
      // is closed at this moment, and `.focus()` on a `display: none` element
      // is a silent no-op — which is the same class of failure this whole
      // repair is about.
      setTimeout(() => state.control.focus(), 0);
    },
    true, { signal: state.signal },
  );

  // Whatever is said stops being true the moment the answer changes.
  state.native.addEventListener("change", () => {
    if (state.native.checkValidity()) clearComplaint(state);
  }, { signal: state.signal });
  form.addEventListener("reset", () => setTimeout(() => clearComplaint(state), 0), { signal: state.signal });
}

/**
 * Takes back the browser's complaint, and the flags that went with it.
 *
 * Only ever what the script wrote. An `error` prop is the caller's sentence —
 * it is usually the one a server sent back — and clearing it because the
 * browser is now satisfied would delete an answer the browser never had.
 */
function clearComplaint(state) {
  if (state.saidByCaller) return;
  const slot = state.field.closest(".nd-field")?.querySelector("[data-nd-field-error]");
  saySentence(slot, "");
  state.field.dataset.ndInvalid = "false";
  state.control.removeAttribute("aria-invalid");
}

/** The rows, built from the options the native control actually has. */
function rebuild(state) {
  state.list.textContent = "";
  state.rows = [];
  const loading = state.field.dataset.ndLoading === "true";
  if (loading) {
    say(state, state.field.dataset.ndLoadingLabel);
    return;
  }
  let index = 0;
  const row = (option, into, groupDisabled) => {
    // The placeholder is not an answer, so it is not an option to choose. The
    // way back to nothing is `clearable`, which says so with a control.
    if (option.value === "") return;
    const item = document.createElement("li");
    item.className = "nd-select-item";
    item.id = `${state.id}-option-${index}`;
    item.setAttribute("role", "option");
    item.dataset.value = option.value;
    if (option.disabled || groupDisabled) item.setAttribute("aria-disabled", "true");

    if (option.dataset.ndSwatch) {
      // `Swatch`'s own mechanism: a chip that declares a palette and takes its
      // colour from it, so no colour is named here either.
      const swatch = document.createElement("span");
      swatch.className = "nd-swatch";
      swatch.dataset.palette = option.dataset.ndSwatch;
      swatch.dataset.ndSize = "small";
      swatch.dataset.ndShape = "pill";
      swatch.setAttribute("aria-hidden", "true");
      item.append(swatch);
    }
    const glyph = option.dataset.ndIcon && template(state, option.dataset.ndIcon);
    if (glyph) item.append(glyph);

    const text = document.createElement("span");
    text.className = "nd-select-item-text";
    const name = document.createElement("span");
    name.className = "nd-select-item-label";
    name.textContent = option.textContent;
    text.append(name);
    if (option.dataset.ndDetail) {
      // Inside the option and therefore part of its name: a row that shows two
      // lines and announces one has told a screen-reader user less than the
      // page shows, which is the failure a second line usually introduces.
      const detail = document.createElement("span");
      detail.className = "nd-select-item-detail";
      detail.textContent = option.dataset.ndDetail;
      text.append(detail);
    }
    item.append(text);
    const tick = template(state, "check");
    if (tick) {
      const mark = document.createElement("span");
      mark.className = "nd-select-item-mark";
      mark.setAttribute("aria-hidden", "true");
      mark.append(tick);
      item.append(mark);
    }
    into.append(item);
    state.rows.push({ item, option });
    index += 1;
  };

  for (const child of state.native.children) {
    if (child.tagName === "OPTGROUP") {
      // `role="group"` inside the listbox, with the run's own `<ul>` marked
      // `none` so its options are owned by the group rather than by a second
      // list. A `<li>` cannot hold a `<li>`; this is the shape that can.
      const group = document.createElement("li");
      group.className = "nd-select-group-row";
      group.setAttribute("role", "group");
      group.setAttribute("aria-label", child.label);
      const heading = document.createElement("span");
      heading.className = "nd-select-group-label";
      heading.setAttribute("aria-hidden", "true");
      heading.textContent = child.label;
      group.append(heading);
      const inner = document.createElement("ul");
      inner.className = "nd-select-group-list";
      inner.setAttribute("role", "none");
      group.append(inner);
      state.list.append(group);
      for (const option of child.children) row(option, inner, child.disabled);
    } else if (child.tagName === "OPTION") {
      row(child, state.list, false);
    }
  }
}

/** A drawing the markup already carries, cloned. This file ships none. */
function template(state, glyph) {
  const found = state.field.querySelector(`template[data-nd-glyph="${CSS.escape(glyph)}"]`);
  return found ? found.content.cloneNode(true) : null;
}

/** The one line the panel says when it has no list to show. */
function say(state, words) {
  state.note.textContent = words ?? "";
  state.note.hidden = !words;
}

/** What the closed control reads, and which rows are marked chosen. */
function paint(state) {
  const chosen = [...state.native.selectedOptions].filter((option) => option.value !== "");
  const value = state.control.querySelector(".nd-select-value");
  if (chosen.length === 0) {
    // FROM THE ATTRIBUTE, NOT FROM THE EMPTY OPTION.
    //
    // A `multiple` select carries no `<option value="">` — nothing chosen
    // already submits nothing — so reading the empty state off one left the
    // control saying nothing at all, which reads as broken rather than as
    // unanswered. The component writes the word where both kinds can find it.
    value.textContent = state.field.dataset.ndPlaceholder ?? "";
    value.dataset.ndEmpty = "true";
  } else if (chosen.length === 1) {
    value.textContent = chosen[0].textContent;
    delete value.dataset.ndEmpty;
  } else {
    // A list of eleven names does not fit in a control, and a control that
    // shows three and hides eight has lied about the answer.
    value.textContent = fillMessage(state.field.dataset.ndSelectedLabel ?? "", {
      count: chosen.length,
    });
    delete value.dataset.ndEmpty;
  }
  state.box.dataset.ndFilled = String(chosen.length > 0);
  for (const { item, option } of state.rows) {
    if (option) item.setAttribute("aria-selected", String(option.selected));
  }
}

/** Tells anything listening that the answer moved, as a real change would. */
function announce(state) {
  state.native.dispatchEvent(new Event("input", { bubbles: true }));
  state.native.dispatchEvent(new Event("change", { bubbles: true }));
}

function visibleRows(state) {
  return state.rows.filter((row) => !row.item.hidden);
}

function firstVisible(state) {
  const chosen = state.rows.findIndex((row) => row.option?.selected && !row.item.hidden);
  if (chosen >= 0) return chosen;
  return state.rows.findIndex((row) => !row.item.hidden);
}

/** Hides the rows the typed term does not reach, and says so if none is left. */
function applyFilter(state) {
  const term = (state.search?.value ?? "").trim().toLowerCase();
  // Last pass's offer, gone before this pass decides whether to make one.
  for (const row of state.rows.filter((row) => !row.option)) row.item.remove();
  state.rows = state.rows.filter((row) => row.option);
  for (const { item, option } of state.rows) {
    const haystack = `${option.textContent} ${option.dataset.ndDetail ?? ""}`.toLowerCase();
    item.hidden = term !== "" && !haystack.includes(term);
  }
  // A group with nothing left in it is a heading over a gap.
  for (const group of state.list.querySelectorAll(".nd-select-group-row")) {
    group.hidden = ![...group.querySelectorAll(".nd-select-item")].some((item) => !item.hidden);
  }
  // See `enhanceSelect`: a filter box is what makes this mean anything.
  const creatable = state.creatable;
  // BY VALUE AS WELL AS BY LABEL, AND THE VALUE IS THE ONE THAT MATTERS.
  //
  // This compared `textContent` only, while `create()` writes the term into
  // BOTH `value` and `textContent` — so wherever the two differ, and the
  // shipped example `SelectOption { value: "ochre-balanced", label: "Ochre" }`
  // is exactly that, typing the value found no label to match, offered to
  // create, and appended a SECOND option carrying a value the control already
  // had. The form then submitted `palette=ochre-balanced` twice from one
  // single-valued select. Measured, not theorised.
  const exact = state.rows.some(
    (row) => sameValue(row.option.textContent, term) || sameValue(row.option.value, term),
  );
  const offer = creatable && term !== "" && !exact;
  if (offer) {
    const item = document.createElement("li");
    item.className = "nd-select-item";
    item.id = `${state.id}-create`;
    item.setAttribute("role", "option");
    item.setAttribute("aria-selected", "false");
    item.dataset.ndCreate = "";
    const text = document.createElement("span");
    text.className = "nd-select-item-text";
    text.textContent = fillMessage(state.field.dataset.ndCreateLabel ?? "", {
      term: state.search.value.trim(),
    });
    item.append(text);
    state.list.append(item);
    // A row like any other, so the arrow keys reach it and Enter takes it.
    state.rows.push({ item, option: null });
  }
  say(state, visibleRows(state).length === 0 ? state.field.dataset.ndEmptyLabel : "");
}

/**
 * Whether two option strings are the same answer.
 *
 * `Intl.Collator` rather than `toLowerCase()`: that is locale-independent and
 * gets Turkish dotted and dotless I wrong, and it does not fold the difference
 * between `é` typed as one code point and `é` pasted as two — which are
 * different strings, identical on the screen, and would become two options
 * nobody can tell apart. `base` sensitivity ignores case and accent, which is
 * what "the reader meant the same thing" means here.
 */
const SAME = new Intl.Collator(undefined, { sensitivity: "base", usage: "search" });
const sameValue = (a, b) => SAME.compare((a ?? "").trim(), (b ?? "").trim()) === 0;

/** Adds what was typed to the REAL select, which is what makes it submittable. */
function create(state) {
  const term = state.search.value.trim().normalize("NFC");
  if (!term) return;
  // The offer should never have been drawn for a value that exists, and this
  // is the second half of that: `applyFilter` decides what to show, and the
  // store refuses a duplicate whatever anything upstream believed.
  const existing = [...state.native.options].find((option) => sameValue(option.value, term));
  if (existing) {
    existing.selected = true;
    state.search.value = "";
    rebuild(state);
    applyFilter(state);
    announce(state);
    paint(state);
    if (!state.native.multiple) closeList(state);
    return;
  }
  const option = document.createElement("option");
  option.value = term;
  option.textContent = term;
  // WHAT THE SCRIPT ADDED, MARKED AS SUCH — so a reset can take it away again.
  // `form.reset()` restores `defaultSelected`, which is `false` on anything
  // created here, so without this the option is deselected and then lives in
  // the list for ever, reappearing as a suggestion nobody declared and growing
  // by one on every reset.
  option.dataset.ndCreated = "";
  state.native.append(option);
  option.selected = true;
  state.search.value = "";
  rebuild(state);
  applyFilter(state);
  announce(state);
  paint(state);
  if (!state.native.multiple) closeList(state);
}

/** Takes an option, and closes unless several may be chosen. */
function takeOption(state, index) {
  const row = state.rows[index];
  if (!row || row.item.getAttribute("aria-disabled") === "true") return;
  // The offer to add what was typed is a row with nothing behind it yet.
  if (!row.option) {
    create(state);
    return;
  }
  if (state.native.multiple) {
    row.option.selected = !row.option.selected;
  } else {
    for (const option of state.native.options) option.selected = false;
    row.option.selected = true;
  }
  announce(state);
  paint(state);
  // Several may be chosen means several will be, and closing after each one is
  // a reader reopening the list for every answer they have.
  if (!state.native.multiple) closeList(state);
}

/** Moves the highlight, and says where it went. */
function moveTo(state, index, scroll = true) {
  if (index < 0 || index >= state.rows.length) return;
  for (const { item } of state.rows) delete item.dataset.ndActive;
  const item = state.rows[index].item;
  item.dataset.ndActive = "";
  state.active = index;
  focusHolder(state).setAttribute("aria-activedescendant", item.id);
  if (scroll) item.scrollIntoView({ block: "nearest" });
}

/** The element the keyboard is on, which is the one that owns the highlight. */
function focusHolder(state) {
  return state.search ?? state.control;
}

function step(state, delta) {
  const rows = visibleRows(state);
  if (rows.length === 0) return;
  const here = rows.findIndex((row) => row === state.rows[state.active]);
  const next = here < 0 ? (delta > 0 ? 0 : rows.length - 1) : (here + delta + rows.length) % rows.length;
  moveTo(state, state.rows.indexOf(rows[next]));
}

function openList(state) {
  // Through the shared registry, so a `TagPicker`'s list closes this one and
  // the other way round.
  state.close ??= () => closeList(state, false);
  claimOpenPanel(state.close);
  openSelect = state;
  state.panel.hidden = false;
  state.control.setAttribute("aria-expanded", "true");
  if (state.search) {
    state.search.value = "";
    applyFilter(state);
    state.search.focus();
  }
  paintSelectSwatches(state);
  anchorSelect(state);
  const at = firstVisible(state);
  if (at >= 0) moveTo(state, at);
}

function closeList(state, restore = true) {
  if (state.panel.hidden) return;
  releaseAnchor(state.panel);
  state.panel.hidden = true;
  state.control.setAttribute("aria-expanded", "false");
  focusHolder(state).removeAttribute("aria-activedescendant");
  if (openSelect === state) openSelect = null;
  if (state.close) releaseOpenPanel(state.close);
  if (restore) state.control.focus();
}

function onSelectKey(state, event) {
  const open = !state.panel.hidden;
  switch (event.key) {
    case "ArrowDown":
    case "ArrowUp":
      event.preventDefault();
      if (!open) openList(state);
      else step(state, event.key === "ArrowDown" ? 1 : -1);
      return;
    case "Home":
    case "End": {
      if (!open) return;
      // With something TYPED these are the caret's, not the list's: a reader
      // fixing a typo at the start of what they typed is not asking for the
      // first option. With the box empty there is no typo and no caret worth
      // keeping, so the keys reach the list — which is what the contract
      // promises and what an empty filter box used to swallow.
      if (state.search && state.search.value !== "") return;
      event.preventDefault();
      const rows = visibleRows(state);
      if (rows.length) moveTo(state, state.rows.indexOf(event.key === "Home" ? rows[0] : rows.at(-1)));
      return;
    }
    case "Enter":
      if (!open) {
        event.preventDefault();
        openList(state);
        return;
      }
      event.preventDefault();
      takeOption(state, state.active);
      return;
    case " ":
      // A FILTER BOX NEEDS ITS SPACES, AND THAT IS NOT NEGOTIABLE. Which
      // leaves a `multiple` searchable select with no Space, so Enter is what
      // chooses without closing there — `takeOption` keeps the list open
      // whenever several may be chosen, whichever key opened it. The keyboard
      // table says so now rather than promising a key this branch refuses.
      if (state.search) return;
      event.preventDefault();
      if (!open) openList(state);
      else takeOption(state, state.active);
      return;
    case "Escape":
      if (!open) return;
      event.preventDefault();
      closeList(state);
      return;
    case "Tab":
      // Closes and leaves. What was highlighted is NOT chosen by leaving, which
      // is the difference between a combobox and a radiogroup.
      if (open) closeList(state, false);
      return;
    default:
      break;
  }
  // Type-ahead, for the list with no filter box. One printable character at a
  // time, accumulated while they keep coming.
  if (state.search || event.key.length !== 1 || event.metaKey || event.ctrlKey || event.altKey) {
    return;
  }
  const now = performance.now();
  state.typed = now - state.typedAt > TYPE_AHEAD ? event.key : state.typed + event.key;
  state.typedAt = now;
  const term = state.typed.toLowerCase();
  const at = state.rows.findIndex((row) => row.option?.textContent.toLowerCase().startsWith(term));
  if (at < 0) return;
  event.preventDefault();
  if (!open) openList(state);
  moveTo(state, at);
}

/**
 * Fetches the stylesheet each chip's palette needs, on open.
 *
 * A `Swatch` takes its colour from the palette it declares, and a palette is a
 * stylesheet this site loads on demand — so a chip whose sheet has not arrived
 * paints the reader's own accent instead. Six identical chips is what that
 * looks like, and the theme menu's own accent row shipped thirteen of them
 * once for exactly this reason. On open rather than on build, because a page
 * with six selects on it would otherwise fetch every palette before a reader
 * had asked to see one.
 */
function paintSelectSwatches(state) {
  for (const { option } of state.rows) {
    const palette = option?.dataset.ndSwatch;
    if (palette && paletteValues.values.includes(palette)) ensureStylesheet(palette);
  }
}

/** The Select's panel: the second of `anchorPanel`'s three call sites. */
function anchorSelect(state) {
  keepAnchored(state.panel, state.control, { width: "--nd-select-width" }, () =>
    closeList(state, false),
  );
}

// A click anywhere else closes it. Not `blur`: a click on the panel's own
// scrollbar blurs the input, and a list that closed when you reached for its
// scrollbar would be unusable with a mouse.
document.addEventListener("pointerdown", (event) => {
  if (!openSelect) return;
  const field = event.target.closest?.(SELECT_FIELD);
  if (field !== openSelect.field) closeList(openSelect, false);
});

// The scroll and resize listeners the Select used to carry are gone: the
// `anchored` registry above holds one of each for every floating panel, and it
// is what closes this one when its control leaves the window.

// --- Search -----------------------------------------------------------------

// WHAT THIS REPLACES.
//
// One `input` listener that ran `dataset.ndIndex.toLowerCase().includes(needle)`
// over three hundred and forty-seven elements per keystroke and toggled `hidden`
// on each. No ranking, no reason a hit was a hit, no keyboard, no combobox
// semantics — the box and the results were unrelated in the accessibility tree,
// so a reader heard "twelve results" and had no announced way to reach one.
//
// WHAT THE COMPONENT OWES AND WHAT THIS OWES.
//
// The markup carries the box, the rows, the words and the ids. Every one of
// `role="combobox"`, `aria-expanded`, `aria-controls`, `aria-autocomplete`,
// `aria-activedescendant`, `role="listbox"`, `role="option"` and `role="group"`
// is written HERE, because a box claiming to control a listbox that nothing can
// open is a promise the markup does not keep.
//
// TWO SUPPLY FORMS. Children are rows already in the document — real links that
// work with this file blocked. `src` is a JSON index fetched on first open,
// filled into a fixed pool of rows. The pool is not an optimisation: rebuilding
// the list per keystroke is how `aria-activedescendant` comes to name a removed
// node, which is the commonest defect in this pattern and invisible without a
// screen reader.

/** Every enhanced search, by its root. */
const searches = new Map();

/** How long after the last keystroke the count is spoken. */
const ANNOUNCE_AFTER = 400;

/** How many recent queries are kept, whatever the reader typed. */
const RECENT_KEEP = 6;

function enhanceSearches(root = document) {
  for (const root_ of root.querySelectorAll("[data-nd-search]")) {
    if (searches.has(root_)) continue;
    const state = buildSearch(root_);
    if (state) searches.set(root_, state);
  }
}

/**
 * Takes the behaviour back off every enhanced search under `root`.
 *
 * A BEHAVIOUR THAT CANNOT BE RELEASED IS A DEFECT FOR EVERY CONSUMER WITH A
 * CLIENT-SIDE ROUTER, and this one holds its state in a STRONG `Map` keyed by
 * the element. Every search ever enhanced stays reachable from that map for the
 * life of the page, with its rows, its pool and its recent queries. A `WeakMap`
 * would not have fixed it either, because the state points back at the element.
 */
function releaseSearches(root = document) {
  for (const element of root.querySelectorAll("[data-nd-search]")) {
    const state = searches.get(element);
    if (!state) continue;
    releaseAnchor(state.panel);
    state.binding.abort();
    searches.delete(element);
  }
}

function buildSearch(root_) {
  const binding = new AbortController();
  const signal = binding.signal;
  const box = root_.querySelector("[data-nd-search-input]");
  const panel = root_.querySelector("[data-nd-search-panel]");
  const list = root_.querySelector("[data-nd-search-results]");
  if (!box || !panel || !list) return null;

  const name = root_.dataset.ndSearch;
  const id = `nd-search-${name}`;
  const state = {
    // CARRIED ON THE STATE, because every helper below already takes it. A
    // parallel argument threaded through a dozen call sites is a dozen places
    // for one of them to be forgotten.
    binding,
    signal,
    root: root_,
    name,
    id,
    mode: root_.dataset.ndPresentation ?? "inline",
    box,
    panel,
    list,
    note: root_.querySelector("[data-nd-search-status]"),
    dialog: root_.querySelector("[data-nd-search-dialog]"),
    trigger: root_.querySelector("[data-nd-search-open]"),
    clear: root_.querySelector("[data-nd-search-clear]"),
    recentBox: root_.querySelector("[data-nd-search-recent]"),
    recentList: root_.querySelector("[data-nd-search-recent-list]"),
    src: root_.dataset.ndSrc ?? "",
    cap: Number.parseInt(root_.dataset.ndCap ?? "", 10) || 0,
    filter: root_.dataset.ndFilter === "true",
    recentKey: root_.dataset.ndRecent ?? "",
    words: {
      status: root_.dataset.ndStatus ?? "",
      empty: root_.dataset.ndEmptyLabel ?? "",
      loading: root_.dataset.ndLoadingLabel ?? "",
      error: root_.dataset.ndErrorLabel ?? "",
      cap: root_.dataset.ndCapLabel ?? "",
    },
    candidates: null,
    pool: [],
    hits: [],
    active: -1,
    frame: 0,
    sayAt: 0,
    lastSaid: null,
    // THE CALLER'S CLAIM, and it was read by nothing until this line.
    //
    // `data-nd-loading` was stamped by the component, documented in the
    // contract and never looked at here — so `Search { loading: true }`
    // rendered a box indistinguishable from one whose results had all
    // arrived, which is the exact confusion the prop exists to prevent.
    //
    // It is separate from `fetching` below on purpose. This one is what the
    // caller says about their own data; that one is whether a request for
    // `src` is in the air. They produce the same sentence and they are not the
    // same fact — collapsing them into one field is what hid this, because the
    // fetch guard read the field as "a request is running" and the attribute
    // never got a chance to mean anything.
    loading: root_.dataset.ndLoading === "true",
    fetching: false,
    loaded: false,
    failed: false,
  };

  box.setAttribute("role", "combobox");
  box.setAttribute("aria-autocomplete", "list");
  box.setAttribute("aria-controls", `${id}-results`);
  box.setAttribute("aria-expanded", "false");
  list.setAttribute("role", "listbox");
  for (const group of list.querySelectorAll("[data-nd-search-group]")) {
    group.setAttribute("role", "group");
    group.setAttribute("aria-label", group.dataset.ndSearchGroup);
    // A `listbox` holds groups and options and nothing else, so the inner list
    // has to stop being a list as far as the tree is concerned.
    group.querySelector("ul")?.setAttribute("role", "none");
  }

  if (state.src) {
    state.pool = buildPool(state);
  } else {
    state.candidates = childCandidates(state);
  }

  box.addEventListener("input", () => runQuery(state), { signal });
  box.addEventListener("keydown", (event) => onSearchKey(state, event), { signal });
  box.addEventListener("focus", () => {
    if (state.mode === "popover" && (state.box.value || recentQueries(state).length)) {
      openSearch(state);
    }
  }, { signal });
  state.clear?.addEventListener("click", () => clearQuery(state), { signal });
  state.trigger?.addEventListener("click", () => openSearch(state), { signal });
  root_.querySelector("[data-nd-search-forget]")?.addEventListener("click", () => {
    forgetQueries(state);
    showRecent(state);
  }, { signal });

  // Choosing a result is what makes it recent, and it is the only thing that
  // does: a query somebody typed and abandoned is not one they meant.
  //
  // AND IT ANSWERS THE QUESTION, which is what closes and clears the palette.
  // Until `navigate.js` existed, following a result took the whole document
  // with it and both were free; with the document kept, the dialog stayed open
  // over the page the reader had just asked for, and the box stayed full — so
  // reopening the palette showed the results of the query they had already
  // answered instead of the recents that are the point of an empty box.
  //
  // Here rather than in the router, because the dialog is `Search`'s. A router
  // that reached in to close somebody else's surface would need a second line
  // for every surface a link can sit in.
  list.addEventListener("click", (event) => {
    if (!event.target.closest("a")) return;
    rememberQuery(state);
    if (!state.dialog) return;
    clearQuery(state);
    state.dialog.close();
  }, { signal });

  if (state.dialog) {
    // A trigger that opens a modal nothing can fill is a button that does
    // nothing, so the markup ships it hidden and this is what reveals it.
    if (state.trigger) state.trigger.hidden = false;
    state.dialog.addEventListener("cancel", (event) => {
      // THE FIRST ESCAPE CLEARS AND THE SECOND CLOSES, implemented WITH the
      // platform rather than around it: the browser's default here IS the
      // close, so preventing it is the feature.
      if (state.box.value !== "") {
        event.preventDefault();
        clearQuery(state);
      }
    }, { signal });
    state.dialog.addEventListener("close", () => {
      state.root.removeAttribute("data-nd-open");
      // A stale "twelve results" read out after the panel is gone is a lie.
      note(state, "");
    }, { signal });
    // The backdrop is the dialog element itself, which is where a click on it
    // lands.
    state.dialog.addEventListener("click", (event) => {
      if (event.target === state.dialog) state.dialog.close();
    }, { signal });
    // And the one exit a reader can see. Escape and the backdrop are the other
    // two, and neither of them is visible on a touch screen.
    state.root
      .querySelector("[data-nd-search-close]")
      ?.addEventListener("click", () => state.dialog.close(), { signal });
  }

  root_.dataset.ndEnhanced = "true";
  if (state.mode === "inline") runQuery(state);
  return state;
}

/** The rows the page already shipped, prepared once. */
function childCandidates(state) {
  return [...state.list.querySelectorAll(".nd-search-result")].map((row, index) => {
    const labelEl = row.querySelector("[data-nd-search-label]");
    const text = labelEl?.textContent ?? "";
    row.setAttribute("role", "option");
    row.id = `${state.id}-option-${index}`;
    row.querySelector("a")?.setAttribute("tabindex", "-1");
    return {
      row,
      labelEl,
      text,
      match: prepare(text),
      // Matched but never drawn, and a row matched only on these is
      // deliberately not highlighted: the letters that matched are not letters
      // on the screen, and a highlight over the wrong ones is worse than none.
      keywords: row.dataset.ndKeywords ? prepare(row.dataset.ndKeywords) : null,
      group: row.closest("[data-nd-search-group]") ?? null,
    };
  });
}

/** A fixed set of rows the fetched index is written into. */
function buildPool(state) {
  const size = state.cap > 0 ? state.cap : 20;
  const pool = [];
  for (let index = 0; index < size; index += 1) {
    const row = document.createElement("li");
    row.className = "nd-search-result";
    row.setAttribute("role", "option");
    row.id = `${state.id}-option-${index}`;
    row.hidden = true;
    const link = document.createElement("a");
    link.className = "nd-search-result-link";
    link.tabIndex = -1;
    const label = document.createElement("span");
    label.className = "nd-search-result-label";
    label.dataset.ndSearchLabel = "";
    const group = document.createElement("span");
    group.className = "nd-search-result-detail";
    link.append(label, group);
    row.append(link);
    state.list.append(row);
    pool.push({ row, link, labelEl: label, detailEl: group });
  }
  return pool;
}

/** The index at `src`, fetched once, the first time somebody searches. */
async function loadIndex(state) {
  if (state.loaded || state.fetching || !state.src) return;
  state.fetching = true;
  setLoading(state, true);
  note(state, state.words.loading);
  try {
    const response = await fetch(state.src);
    if (!response.ok) throw new Error(String(response.status));
    const data = await response.json();
    state.candidates = (data.entries ?? []).map((entry) => ({
      entry,
      text: entry.label ?? "",
      match: prepare(entry.label ?? ""),
      keywords: entry.keywords ? prepare(entry.keywords) : null,
    }));
    state.loaded = true;
    // And the claim is over: whatever the caller said was on its way is here.
    setLoading(state, false);
    // The children were the no-script fallback; the script has arrived.
    for (const row of state.list.querySelectorAll(".nd-search-result:not([role])")) {
      row.hidden = true;
    }
  } catch {
    // A failed fetch and an empty result look identical and mean opposite
    // things, which is why they are two sentences rather than one.
    state.failed = true;
    setLoading(state, false);
  } finally {
    state.fetching = false;
    runQuery(state);
  }
}

/**
 * Says it in the state and on the element, which are one fact in two places.
 *
 * The attribute is what the stylesheet reads for the progress cursor, and a
 * cursor that keeps spinning after the list arrived is the same lie in the
 * other direction.
 */
function setLoading(state, loading) {
  state.loading = loading;
  state.root.dataset.ndLoading = String(loading);
}

// --- Matching and drawing ---------------------------------------------------

function runQuery(state) {
  const query = state.box.value.trim();
  if (state.src && !state.loaded && !state.failed) {
    if (query) loadIndex(state);
    // AN EMPTY BOX IS EXACTLY WHEN THE RECENTS ARE WORTH SHOWING, and this
    // return swallowed them: `showRecent` is called from `drawResults`, which
    // is downstream of here, so a palette that had not fetched its index yet
    // — which is every palette, on the way in — drew nothing until a letter
    // was typed. The one moment the history is useful is the one moment it
    // was not on the screen.
    else showRecent(state);
    return;
  }
  if (!state.candidates) return;

  if (query === "") {
    state.hits = state.src ? [] : state.candidates.map((item) => ({ item, positions: [] }));
    state.total = state.hits.length;
  } else {
    const { hits, total } = rank(query, state.candidates, state.cap);
    // A row nothing matched by its label may still answer to its keywords, and
    // then it is shown without a highlight.
    const missed = [];
    if (state.cap === 0 || hits.length < state.cap) {
      const found = new Set(hits.map((hit) => hit.item));
      for (const item of state.candidates) {
        if (found.has(item) || !item.keywords) continue;
        const hit = rank(query, [{ match: item.keywords }], 0).hits[0];
        if (hit) missed.push({ item, positions: [], score: hit.score - 1_000_000 });
      }
    }
    missed.sort((a, b) => b.score - a.score);
    const room = state.cap > 0 ? Math.max(0, state.cap - hits.length) : missed.length;
    state.hits = [...hits, ...missed.slice(0, room)];
    state.total = total + missed.length;
  }

  state.active = -1;
  schedule(state);
}

/**
 * Render and anchor inside one coalesced frame.
 *
 * `requestAnimationFrame` and not a microtask: a microtask runs before layout,
 * so reading `panel.offsetWidth` for the anchor from one forces a synchronous
 * reflow on every keystroke. This puts the read and the write in the same
 * frame, once.
 */
function schedule(state) {
  if (state.frame) return;
  state.frame = requestAnimationFrame(() => {
    state.frame = 0;
    drawResults(state);
  });
}

function drawResults(state) {
  const query = state.box.value.trim();
  const shown = new Set(state.hits.map((hit) => hit.item));

  if (state.src) {
    state.hits.forEach((hit, index) => {
      const slot = state.pool[index];
      if (!slot) return;
      slot.row.hidden = false;
      slot.link.href = hit.item.entry.href ?? "";
      paintLabel(slot.labelEl, hit.item.match.chars, hit.positions);
      slot.detailEl.textContent = hit.item.entry.detail ?? hit.item.entry.group ?? "";
    });
    for (let index = state.hits.length; index < state.pool.length; index += 1) {
      state.pool[index].row.hidden = true;
    }
  } else {
    for (const item of state.candidates) {
      item.row.hidden = !shown.has(item);
      if (item.group) item.group.hidden = true;
    }
    // Reordered by moving rather than by cloning, and only the rows that are
    // shown: the rest keep the author's order, which is also the order they
    // have with no script.
    for (const hit of state.hits) {
      if (hit.positions.length) markInPlace(hit.item.labelEl, hit.positions);
      else clearMarks(hit.item.labelEl);
      if (hit.item.group) {
        hit.item.group.hidden = false;
      } else if (query) {
        state.list.append(hit.item.row);
      }
    }
  }

  const has = state.hits.length > 0;
  state.box.setAttribute("aria-expanded", String(has));
  state.root.toggleAttribute("data-nd-has-results", has);

  if (state.filter) filterAround(state, query);

  // The sentence, and the same one the live region says.
  let words = "";
  if (state.failed) words = state.words.error;
  else if (state.loading || state.fetching) words = state.words.loading;
  else if (query === "" && state.src) words = "";
  else if (!has && query) words = state.words.empty;
  else if (state.cap > 0 && state.total > state.hits.length) {
    words = fillMessage(
      state.words.cap,
      { shown: state.hits.length, total: state.total },
      state.total,
    );
  } else if (query) {
    words = fillMessage(state.words.status, { count: state.total }, state.total);
  }
  note(state, words);
  showRecent(state);
}

/**
 * The panel's one sentence — shown and announced by the same element, at most
 * once per pause, and never twice the same.
 *
 * ONE ELEMENT, BECAUSE A COUNT WORTH TELLING A SCREEN READER IS A COUNT A
 * READER CAN SEE. It is `role="status"`, so writing it *is* announcing it —
 * which is why the write is what gets debounced rather than a second hidden
 * copy of it. The line this replaces wrote on every `input` event, so typing
 * `button` queued six utterances, five of them stale before they were spoken.
 *
 * Clearing is immediate and appearing is not: a sentence that lingers 400ms
 * after it stopped being true is the one case where the delay is visible.
 */
function note(state, words) {
  if (!state.note) return;
  clearTimeout(state.sayAt);
  if (words === "") {
    state.lastSaid = "";
    state.note.textContent = "";
    return;
  }
  // THE FIRST SENTENCE IS NOT PACED. The delay exists to collapse a burst of
  // keystrokes into one utterance, and on the first write there is no burst to
  // collapse — only a panel that says nothing for four hundred milliseconds
  // while claiming to be still loading.
  if (state.lastSaid === null) {
    state.lastSaid = words;
    state.note.textContent = words;
    return;
  }
  state.sayAt = setTimeout(() => {
    if (words === state.lastSaid) return;
    state.lastSaid = words;
    state.note.textContent = words;
  }, ANNOUNCE_AFTER);
}

/** The rest of the page, filtered by the same query. Off unless asked for. */
function filterAround(state, query) {
  const scope = state.root.closest("nav, main, aside, section, body");
  if (!scope) return;
  for (const element of scope.querySelectorAll("[data-nd-index]")) {
    if (state.root.contains(element)) continue;
    const hit = query === "" || rank(query, [{ match: prepare(element.dataset.ndIndex) }], 0).hits.length > 0;
    const row = element.closest("li") ?? element;
    row.hidden = !hit;
  }
}

// --- The highlight ----------------------------------------------------------

// `<mark class="nd-mark">`, which is the component whose summary is literally
// "Text highlighted for the reader's current purpose, such as a search hit".
//
// The CSS Custom Highlight API would do this without touching the DOM and is
// rejected anyway: a highlight pseudo-element accepts no layout-affecting
// property, so it cannot set `font-weight` — and `Mark`'s own accessibility
// note says the weight is why the highlight is legible to a reader who cannot
// distinguish the colour. It would render a `Mark` that fails `Mark`'s contract.

/** A label the script owns, rebuilt with the matched runs marked. */
function paintLabel(into, chars, positions) {
  into.textContent = "";
  const wanted = new Set(positions);
  let run = [];
  let marked = false;
  const flush = () => {
    if (run.length === 0) return;
    if (marked) {
      const mark = document.createElement("mark");
      mark.className = "nd-mark";
      mark.dataset.ndHit = "";
      mark.textContent = run.join("");
      into.append(mark);
    } else {
      into.append(document.createTextNode(run.join("")));
    }
    run = [];
  };
  for (let index = 0; index < chars.length; index += 1) {
    const on = wanted.has(index);
    if (on !== marked) {
      flush();
      marked = on;
    }
    run.push(chars[index]);
  }
  flush();
}

/**
 * A label the PAGE shipped, wrapped where it matched and put back exactly.
 *
 * Rebuilding it would destroy classes, drawings and anything a consumer
 * attached. Splitting the text nodes and undoing with `normalize()` — which
 * merges them back — leaves every element untouched.
 */
function markInPlace(label, positions) {
  if (!label) return;
  clearMarks(label);
  if (positions.length === 0) return;
  const wanted = [...positions].sort((a, b) => a - b);
  const walker = document.createTreeWalker(label, NodeFilter.SHOW_TEXT);
  const jobs = [];
  let seen = 0;
  let next = 0;
  for (let node = walker.nextNode(); node && next < wanted.length; node = walker.nextNode()) {
    const chars = [...node.data];
    let unit = 0;
    let start = -1;
    let units = 0;
    for (let index = 0; index < chars.length; index += 1) {
      if (next < wanted.length && wanted[next] === seen + index) {
        if (start < 0) {
          start = unit;
          units = 0;
        }
        units += chars[index].length;
        next += 1;
      } else if (start >= 0) {
        jobs.push({ node, start, end: start + units });
        start = -1;
      }
      unit += chars[index].length;
    }
    if (start >= 0) jobs.push({ node, start, end: start + units });
    seen += chars.length;
  }
  // Back to front, so splitting an earlier run cannot move a later offset.
  for (const job of jobs.reverse()) {
    const range = document.createRange();
    range.setStart(job.node, job.start);
    range.setEnd(job.node, job.end);
    const mark = document.createElement("mark");
    mark.className = "nd-mark";
    mark.dataset.ndHit = "";
    range.surroundContents(mark);
  }
}

/** Puts a label back exactly as the page shipped it. */
function clearMarks(label) {
  if (!label) return;
  // `[data-nd-hit]`, not `mark`: a caller's children may hold a `Mark` of their
  // own, and unwrapping that would be this file editing the page's content
  // rather than annotating it.
  for (const mark of label.querySelectorAll("mark[data-nd-hit]")) {
    mark.replaceWith(...mark.childNodes);
  }
  label.normalize();
}

// --- Opening, closing and the keyboard --------------------------------------

function openSearch(state) {
  if (state.mode === "dialog") {
    if (state.dialog?.open) return;
    state.root.dataset.ndOpen = "true";
    // Whatever the box already holds, drawn before the modal is shown — which
    // for an empty box is the recents.
    runQuery(state);
    state.dialog?.showModal();
    // `showModal` moves focus; this only makes sure it lands in the box rather
    // than on whatever the browser picked first.
    state.box.focus();
    state.box.select();
    return;
  }
  if (state.mode !== "popover") return;
  state.root.dataset.ndOpen = "true";
  runQuery(state);
  keepAnchored(state.panel, state.box.closest(".nd-search-field") ?? state.box, { room: 320, width: "--nd-search-width" }, () =>
    closeSearch(state),
  );
}

function closeSearch(state) {
  if (state.mode === "dialog") {
    state.dialog?.close();
    return;
  }
  if (state.mode !== "popover") return;
  releaseAnchor(state.panel);
  state.root.removeAttribute("data-nd-open");
  setActive(state, -1);
}

function clearQuery(state) {
  state.box.value = "";
  runQuery(state);
  state.box.focus();
}

/** The rows a reader can move through, in the order they are drawn. */
function searchRows(state) {
  return state.src
    ? state.pool.filter((slot) => !slot.row.hidden).map((slot) => slot.row)
    : state.hits.map((hit) => hit.item.row).filter((row) => !row.hidden);
}

function setActive(state, index) {
  const rows = searchRows(state);
  for (const row of rows) {
    row.removeAttribute("aria-selected");
    row.removeAttribute("data-nd-active");
  }
  state.active = index;
  const row = rows[index];
  if (!row) {
    state.box.removeAttribute("aria-activedescendant");
    return;
  }
  // On the active option ONLY. `aria-selected="false"` on three hundred rows is
  // noise a screen reader reads out.
  row.setAttribute("aria-selected", "true");
  row.dataset.ndActive = "";
  state.box.setAttribute("aria-activedescendant", row.id);
  row.scrollIntoView({ block: "nearest" });
}

function moveActive(state, delta) {
  const rows = searchRows(state);
  if (rows.length === 0) return;
  const next = state.active < 0 ? (delta > 0 ? 0 : rows.length - 1) : state.active + delta;
  setActive(state, (next + rows.length) % rows.length);
}

/** How many rows fit in the panel, which is what a page means here. */
function pageSize(state) {
  const row = searchRows(state)[0];
  if (!row) return 1;
  const height = row.getBoundingClientRect().height || 1;
  return Math.max(1, Math.floor(state.list.clientHeight / height) - 1);
}

function onSearchKey(state, event) {
  const rows = searchRows(state);
  switch (event.key) {
    case "ArrowDown":
      event.preventDefault();
      if (state.mode === "popover" && !state.root.hasAttribute("data-nd-open")) openSearch(state);
      moveActive(state, 1);
      break;
    case "ArrowUp":
      event.preventDefault();
      if (state.mode === "popover" && !state.root.hasAttribute("data-nd-open")) openSearch(state);
      moveActive(state, -1);
      break;
    case "Home":
    case "End":
      // THE CARET'S, NOT THE LIST'S. A reader fixing a typo at the start of
      // what they typed is not asking for the first result — the same call
      // `onSelectKey` already made, for the same reason. The list keeps the
      // modified pair.
      if (event.metaKey || event.ctrlKey) {
        event.preventDefault();
        setActive(state, event.key === "Home" ? 0 : rows.length - 1);
      }
      break;
    case "PageDown":
    case "PageUp": {
      if (rows.length === 0) break;
      event.preventDefault();
      const step = pageSize(state) * (event.key === "PageDown" ? 1 : -1);
      const from = state.active < 0 ? 0 : state.active;
      setActive(state, Math.min(rows.length - 1, Math.max(0, from + step)));
      break;
    }
    case "Enter": {
      const row = rows[state.active];
      if (!row) return; // A form, if there is one, submits.
      event.preventDefault();
      const link = row.querySelector("a");
      if (!link) return;
      rememberQuery(state);
      if (event.metaKey || event.ctrlKey) window.open(link.href, "_blank", "noopener");
      else link.click();
      break;
    }
    case "Escape":
      event.preventDefault();
      if (state.box.value !== "") clearQuery(state);
      else closeSearch(state);
      break;
    case "Tab":
      // A POPOVER IS LEFT BY TABBING OUT OF IT. A DIALOG IS NOT.
      //
      // This closed on Tab whatever the presentation was, which threw a reader
      // out of the palette on the first press — and did it by fighting the
      // modality the browser was already providing. Inside a `<dialog>` the
      // browser cycles focus within the top layer, which is the behaviour the
      // contract promises and the reason no focus trap is written here.
      if (state.mode !== "dialog") closeSearch(state);
      break;
    default:
      break;
  }
}

// --- The hotkey -------------------------------------------------------------

/** Whether a key belongs to whatever has focus rather than to the page. */
function isTyping(target) {
  return Boolean(
    target?.closest?.(
      // AND ANYTHING THAT SAID IT LISTENS FOR LETTERS. A `role="tree"` node
      // with focus is not a text field, so `/` would have opened the palette
      // out from under a reader who was typing a node's name to jump to it —
      // and the hotkey listener is registered before the roving one, so it
      // would have won. A component that reads printable keys says so with
      // `data-nd-typeahead` and this stops taking them.
      "input, textarea, select, [contenteditable]:not([contenteditable='false']), [data-nd-typeahead]",
    ),
  );
}

/**
 * The first search in document order that asked for the chord.
 *
 * Two claimants would be two things answering one keystroke, so the first wins
 * and it wins deterministically.
 */
function hotkeyOwner() {
  const root_ = document.querySelector('[data-nd-search][data-nd-hotkey="true"]');
  return root_ ? searches.get(root_) : null;
}

document.addEventListener("keydown", (event) => {
  if (event.isComposing || event.defaultPrevented) return;
  const state = hotkeyOwner();
  if (!state) return;

  // Command-K on Apple hardware, Control-K everywhere else. It fires from
  // inside a text field on purpose — that is the convention a palette is judged
  // against — and it toggles, so the same keystroke puts it away.
  if (event.key.toLowerCase() === "k" && (event.metaKey || event.ctrlKey) && !event.altKey) {
    event.preventDefault();
    if (state.mode === "dialog" && state.dialog?.open) closeSearch(state);
    else {
      openSearch(state);
      state.box.focus();
    }
    return;
  }

  // `/` is the other convention, and it must never fire while somebody is
  // typing — including into this component's own box, where a slash is a slash.
  if (event.key === "/" && !event.metaKey && !event.ctrlKey && !event.altKey) {
    if (isTyping(event.target)) return;
    event.preventDefault();
    openSearch(state);
    state.box.focus();
  }
});

// --- Recent queries ---------------------------------------------------------

// Opt-in, by a prop carrying the key. Absent means nothing is stored — not a
// shorter history, none — because what somebody typed into a search is a
// different category of thing from the appearance they chose.

function recentQueries(state) {
  if (!state.recentKey) return [];
  try {
    const stored = JSON.parse(localStorage.getItem(state.recentKey) ?? "[]");
    // A hostile or stale value is not a reason to break the box. The theme
    // layer applies the same discipline to every setting it reads.
    if (!Array.isArray(stored)) return [];
    return stored.filter((entry) => typeof entry === "string").slice(0, RECENT_KEEP);
  } catch {
    return [];
  }
}

function rememberQuery(state) {
  const query = state.box.value.trim();
  if (!state.recentKey || query === "") return;
  const kept = [query, ...recentQueries(state).filter((entry) => entry !== query)];
  try {
    localStorage.setItem(state.recentKey, JSON.stringify(kept.slice(0, RECENT_KEEP)));
  } catch {
    // A full or refused store is not a reason to lose the search.
  }
}

function forgetQueries(state) {
  if (!state.recentKey) return;
  try {
    localStorage.removeItem(state.recentKey);
  } catch {
    // As above.
  }
}

function showRecent(state) {
  if (!state.recentBox || !state.recentList) return;
  const queries = state.box.value.trim() === "" ? recentQueries(state) : [];
  state.recentBox.hidden = queries.length === 0;
  state.recentList.textContent = "";
  for (const query of queries) {
    const item = document.createElement("li");
    const button = document.createElement("button");
    button.type = "button";
    button.className = "nd-search-recent-item";
    button.textContent = query;
    button.addEventListener("click", () => {
      state.box.value = query;
      state.box.focus();
      runQuery(state);
    }, { signal: state.signal });
    item.append(button);
    state.recentList.append(item);
  }
}

// --- The tag picker ----------------------------------------------------------

// SEVERAL VALUES A READER WRITES, ON A REAL `<select multiple>`.
//
// The store is the browser's own — `selectedOptions`, with no mirror — so a
// value cannot be committed twice: the option set IS the index. A typed value
// becomes a real `<option selected>`, exactly as `Select`'s `create()` does,
// and what reaches a server is what the browser sends.
//
// THE THREE THINGS THIS FILE MAY NOT DO, and each was a temptation here:
//
// 1. It may not know what a `Tag` looks like. The picker ships one inside a
//    `<template>` and this clones it — the generalisation of the
//    `<template data-nd-glyph>` `Select` already ships for its tick. A second
//    place that knew a component's markup would drift from the emitter, which
//    is the whole thing the emitter exists to prevent.
// 2. It may not carry a word. Every sentence is a prop with a `data-nd-*`
//    twin, filled through `fillMessage` like every other one here.
// 3. It may not be the only way this works. The composer ships a real `name`
//    and the first thing done below is to remove it, so a reader with no
//    script picks from the native listbox AND types one free value.

const PICKER = "[data-nd-tag-picker]";
const pickers = new WeakMap();

function enhanceTagPickers(root = document) {
  for (const root_ of root.querySelectorAll(PICKER)) {
    if (pickers.has(root_)) continue;
    const state = buildPicker(root_);
    if (state) pickers.set(root_, state);
  }
}

/**
 * Takes the behaviour back off every enhanced picker under `root`.
 *
 * A BEHAVIOUR THAT CANNOT BE RELEASED IS A DEFECT FOR EVERY CONSUMER WITH A
 * CLIENT-SIDE ROUTER. `buildPicker` registers a `pointerdown` on the DOCUMENT
 * per picker, so a page that mounts the same picker ten times carries ten of
 * them, each holding a detached composer alive through its closure.
 */
function releaseTagPickers(root = document) {
  for (const element of root.querySelectorAll(PICKER)) {
    const state = pickers.get(element);
    if (!state) continue;
    if (state.close) releaseOpenPanel(state.close);
    releaseAnchor(state.panel);
    state.binding.abort();
    pickers.delete(element);
    delete element.dataset.ndEnhanced;
  }
}

function buildPicker(root_) {
  const binding = new AbortController();
  const signal = binding.signal;
  const native = root_.querySelector("[data-nd-tag-picker-native]");
  const box = root_.querySelector("[data-nd-tag-picker-input]");
  const list = root_.querySelector("[data-nd-tag-picker-tags]");
  const shape = root_.querySelector("[data-nd-tag-template]");
  if (!native || !box || !list || !shape) return null;

  const name = root_.dataset.ndTagPicker;
  const id = `nd-tag-picker-${name}`;
  const state = {
    // CARRIED ON THE STATE, because every helper below already takes it. A
    // parallel argument threaded through a dozen call sites is a dozen places
    // for one of them to be forgotten.
    binding,
    signal,
    root: root_,
    native,
    box,
    list,
    shape,
    id,
    note: root_.querySelector("[data-nd-tag-picker-status]"),
    clear: root_.querySelector("[data-nd-tag-picker-clear]"),
    separators: [...(root_.dataset.ndSeparators ?? ",")],
    creatable: root_.dataset.ndCreatable === "true",
    paste: root_.dataset.ndPaste === "true",
    duplicates: root_.dataset.ndDuplicates === "true",
    max: Number.parseInt(root_.dataset.ndMax ?? "", 10) || 0,
    pattern: root_.dataset.ndPattern ?? "",
    // Read from the composer, which is where the component puts it, and read
    // ONCE — `paintTags` writes the same property and would otherwise be
    // asking a question it had already answered.
    required: box.required,
    words: {
      create: root_.dataset.ndCreateLabel ?? "",
      max: root_.dataset.ndMaxLabel ?? "",
      invalid: root_.dataset.ndInvalidLabel ?? "",
      remove: root_.dataset.ndRemoveLabel ?? "",
      added: root_.dataset.ndAddedLabel ?? "",
      removed: root_.dataset.ndRemovedLabel ?? "",
      status: root_.dataset.ndStatus ?? "",
      empty: root_.dataset.ndEmptyLabel ?? "",
      loading: root_.dataset.ndLoadingLabel ?? "",
    },
    rows: [],
    active: -1,
    said: 0,
  };

  // THE COMPOSER STOPS SUBMITTING, AND THE SELECT STARTS BEING INVISIBLE.
  //
  // Both in the same breath, because they are one arrangement: with no script
  // the reader answers through a listbox and a text field, and with one they
  // answer through this. Two fields under one name after enhancement would
  // post the composer's leftovers beside the committed values.
  box.removeAttribute("name");
  native.hidden = true;

  const panel = document.createElement("div");
  panel.className = "nd-panel nd-tag-picker-panel";
  panel.hidden = true;
  const rows = document.createElement("ul");
  rows.className = "nd-tag-picker-list";
  rows.id = `${id}-listbox`;
  rows.setAttribute("role", "listbox");
  rows.setAttribute(
    "aria-label",
    root_.closest(".nd-field")?.querySelector(".nd-label")?.textContent ?? "",
  );
  const note = document.createElement("p");
  note.className = "nd-tag-picker-empty";
  panel.append(rows, note);
  root_.querySelector(".nd-tag-picker-field").append(panel);
  state.panel = panel;
  state.list_ = rows;
  state.empty = note;
  state.close = () => closePicker(state);

  // Every combobox attribute, written here and never by the markup: with no
  // script there is no popup, and a box claiming to control a listbox nothing
  // can open is a promise the markup does not keep.
  box.setAttribute("role", "combobox");
  box.setAttribute("aria-autocomplete", "list");
  box.setAttribute("aria-expanded", "false");
  box.setAttribute("aria-controls", rows.id);

  box.addEventListener("input", () => {
    filterPicker(state);
    openPicker(state);
  }, { signal });
  box.addEventListener("keydown", (event) => onPickerKey(state, event), { signal });
  box.addEventListener("paste", (event) => onPickerPaste(state, event), { signal });
  // Whatever is half-typed when the form goes is a value the reader meant.
  // This is the single most-complained-about bug in every token input ever
  // shipped, and it costs two lines.
  box.addEventListener("blur", () => commit(state, box.value, { quiet: true }), { signal });
  native.form?.addEventListener("submit", () => commit(state, box.value, { quiet: true }), { signal });
  native.form?.addEventListener("reset", () => {
    setTimeout(() => {
      for (const option of [...native.options]) {
        if (option.dataset.ndCreated !== undefined) option.remove();
      }
      paintTags(state);
    }, 0);
  }, { signal });

  state.clear?.addEventListener("click", () => {
    for (const option of native.selectedOptions) option.selected = false;
    announcePicker(state);
    paintTags(state);
    box.focus();
  }, { signal });

  // Not `blur`: a click on the panel's own scrollbar blurs the composer, and a
  // list that closed when you reached for its scrollbar would be unusable.
  document.addEventListener("pointerdown", (event) => {
    if (!event.target.closest?.(PICKER)) closePicker(state);
    else if (event.target.closest(PICKER) !== root_) closePicker(state);
  }, { signal });

  root_.dataset.ndEnhanced = "true";
  rebuildPicker(state);
  paintTags(state);
  return state;
}

/** The suggestion rows, from the options the native control actually has. */
function rebuildPicker(state) {
  state.list_.textContent = "";
  state.rows = [];
  if (state.root.dataset.ndLoading === "true") {
    state.empty.textContent = state.words.loading;
    return;
  }
  let index = 0;
  for (const option of state.native.options) {
    if (option.value === "" || option.selected) continue;
    const item = document.createElement("li");
    item.className = "nd-tag-picker-item";
    item.id = `${state.id}-option-${index}`;
    item.setAttribute("role", "option");
    item.textContent = option.textContent;
    item.addEventListener("pointerdown", (event) => {
      // Before the composer can blur, so the click is not a race.
      event.preventDefault();
      commit(state, option.value);
    }, { signal: state.signal });
    state.list_.append(item);
    state.rows.push({ item, option });
    index += 1;
  }
}

/** Hides what does not match, and offers to add what was typed. */
function filterPicker(state) {
  rebuildPicker(state);
  const term = state.box.value.trim();
  const folded = term.toLocaleLowerCase();
  for (const { item, option } of state.rows) {
    item.hidden = term !== "" && !option.textContent.toLocaleLowerCase().includes(folded);
  }
  const shown = state.rows.filter((row) => !row.item.hidden);
  // The offer to add what was typed, when nothing already IS it. Compared with
  // the same collator the store uses, so the row a reader is offered and the
  // value the store refuses are the same question.
  const exact = [...state.native.options].some((option) => sameValue(option.value, term));
  if (state.creatable && term !== "" && !exact) {
    const item = document.createElement("li");
    item.className = "nd-tag-picker-item";
    item.id = `${state.id}-create`;
    item.setAttribute("role", "option");
    item.dataset.ndCreate = "";
    item.textContent = fillMessage(state.words.create, { term });
    item.addEventListener("pointerdown", (event) => {
      event.preventDefault();
      commit(state, term);
    }, { signal: state.signal });
    state.list_.append(item);
    state.rows.push({ item, option: null });
  }
  state.empty.textContent = shown.length === 0 && !state.creatable ? state.words.empty : "";
  state.active = -1;
  state.box.removeAttribute("aria-activedescendant");
}

/**
 * Adds one value to the store, which is the real `<select>`.
 *
 * Everything a value has to survive is here, and every one of them was a way
 * this goes wrong: NFC so `é` typed and `é` pasted are one tag rather than two
 * that look identical; a stricter emptiness test than `trim()`, which does not
 * catch a zero-width space; the pattern the caller asked for; the ceiling; and
 * a duplicate check against the option set rather than against a second list.
 */
function commit(state, raw, { quiet = false } = {}) {
  const term = (raw ?? "").normalize("NFC").replace(/^[\s​﻿]+|[\s​﻿]+$/gu, "");
  if (term === "") return false;

  if (state.pattern) {
    let ok = false;
    try {
      ok = new RegExp(`^(?:${state.pattern})$`, "u").test(term);
    } catch {
      // A pattern this engine will not compile refuses nothing rather than
      // refusing everything: a caller's typo must not make a field unusable.
      ok = true;
    }
    if (!ok) {
      sayPicker(state, fillMessage(state.words.invalid, { term }));
      return false;
    }
  }

  const chosen = [...state.native.selectedOptions].filter((option) => option.value !== "");
  if (state.max > 0 && chosen.length >= state.max) {
    sayPicker(state, fillMessage(state.words.max, { max: state.max }, state.max));
    return false;
  }

  const existing = [...state.native.options].find((option) => sameValue(option.value, term));
  if (existing) {
    if (existing.selected && !state.duplicates) {
      state.box.value = "";
      filterPicker(state);
      closePicker(state);
      return false;
    }
    existing.selected = true;
    // MOVED TO THE END, so the order is the order the reader chose in.
    //
    // `selectedOptions` is DOM order, and a declared suggestion sits above
    // everything typed — so choosing `rust` after typing three words put it
    // FIRST in the tags and first in what the form posts. Both are the reader's
    // list, and neither should be sorted by where a suggestion happened to be
    // declared. The native listbox is hidden by now, so nothing a reader can
    // see is reordered.
    state.native.append(existing);
  } else {
    const option = document.createElement("option");
    option.value = term;
    option.textContent = term;
    // Marked, so a reset can take it away again: `defaultSelected` is false on
    // anything created here, so without this the option is deselected and then
    // lives in the list for ever.
    option.dataset.ndCreated = "";
    state.native.append(option);
    option.selected = true;
  }

  state.box.value = "";
  if (!quiet) announcePicker(state);
  paintTags(state);
  filterPicker(state);
  closePicker(state);
  if (!quiet) sayPicker(state, fillMessage(state.words.added, { label: term }));
  return true;
}

/** Takes one value off, and puts focus somewhere a reader can find it. */
function remove(state, option, from) {
  const label = option.textContent;
  option.selected = false;
  if (option.dataset.ndCreated !== undefined) option.remove();
  announcePicker(state);
  paintTags(state);
  filterPicker(state);
  sayPicker(state, fillMessage(state.words.removed, { label }));

  // THE CONTROL THAT DID THIS HAS JUST LEFT THE PAGE. Focus falling to `<body>`
  // puts a screen-reader user back at the top of the document with no idea why,
  // so it goes to the next tag's control or to the composer.
  const buttons = [...state.list.querySelectorAll(".nd-tag-remove")];
  const next = buttons[Math.min(from, buttons.length - 1)];
  (next ?? state.box).focus();
}

/**
 * Draws the tags, by cloning the one the component shipped.
 *
 * This file does not know what a tag looks like — see the header. The template
 * holds one empty `Tag` with its remove control already named `{label}`, and
 * everything here is filling two text nodes in a copy of it.
 */
function paintTags(state) {
  const chosen = [...state.native.selectedOptions].filter((option) => option.value !== "");
  state.list.textContent = "";
  chosen.forEach((option, at) => {
    const tag = state.shape.content.firstElementChild.cloneNode(true);
    const label = option.textContent;
    tag.querySelector(".nd-tag-label").textContent = label;
    tag.dataset.ndValue = option.value;
    const cross = tag.querySelector(".nd-tag-remove");
    if (cross) {
      const named = fillMessage(state.words.remove, { label });
      cross.setAttribute("aria-label", named);
      cross.setAttribute("title", named);
      cross.value = option.value;
      // NOT A TAB STOP. Forty tags would be forty presses between the composer
      // and the next control; the list is one stop and the arrows walk it.
      cross.tabIndex = -1;
      cross.addEventListener("click", () => remove(state, option, at), { signal: state.signal });
    }
    const item = document.createElement("li");
    item.append(tag);
    state.list.append(item);
  });
  state.list.tabIndex = chosen.length > 0 ? 0 : -1;
  // AND THE COMPOSER STOPS BEING COMPULSORY ONCE THERE IS AN ANSWER.
  //
  // `required` sits on the composer, because the browser has to focus an
  // invalid control to anchor its bubble and the real `<select>` is hidden.
  // Every commit clears the composer and nothing took the attribute off, so a
  // reader who entered three tags met "please fill in this field" pointing at
  // an empty box they had deliberately emptied — the field was unsubmittable
  // exactly when it had been answered. The requirement is about the field, so
  // it follows the tags rather than the box.
  if (state.required) state.box.required = chosen.length === 0;
  state.root.toggleAttribute("data-nd-filled", chosen.length > 0);
  if (state.clear) state.clear.hidden = chosen.length === 0;
  // The standing sentence. `commit` and `remove` say something more specific
  // right after this and win, because `sayPicker` replaces rather than queues —
  // one element, one sentence, the most recent thing worth saying, which is the
  // shape `Search`'s note already has.
  state.root.dataset.ndCount = String(chosen.length);
  // NOTHING TO REPORT IS REPORTED BY AN ABSENCE. A field with no tags in it
  // saying *0 topics* is a page telling a reader about something that has not
  // happened — the same argument `Badge { show_zero }` makes, and the same one
  // `Search` makes by saying nothing until there is a query.
  sayPicker(
    state,
    chosen.length === 0
      ? ""
      : fillMessage(state.words.status, { count: chosen.length }, chosen.length),
  );
}

/** The one sentence, at most one per pause. */
function sayPicker(state, words) {
  if (!state.note) return;
  clearTimeout(state.said);
  if (!words) {
    state.note.textContent = "";
    state.said = 0;
    return;
  }
  state.said = setTimeout(() => {
    state.note.textContent = words;
  }, ANNOUNCE_AFTER);
}

/** Real events on the real control, so a consumer's listener runs. */
function announcePicker(state) {
  state.native.dispatchEvent(new Event("input", { bubbles: true }));
  state.native.dispatchEvent(new Event("change", { bubbles: true }));
}

function pickerRows(state) {
  return state.rows.filter((row) => !row.item.hidden);
}

function movePicker(state, delta) {
  const rows = pickerRows(state);
  if (rows.length === 0) return;
  const here = rows.findIndex((row) => row === state.rows[state.active]);
  const next = here < 0 ? (delta > 0 ? 0 : rows.length - 1) : (here + delta + rows.length) % rows.length;
  setPickerActive(state, state.rows.indexOf(rows[next]));
}

function setPickerActive(state, index) {
  for (const { item } of state.rows) item.removeAttribute("aria-selected");
  state.active = index;
  const row = state.rows[index];
  if (!row) {
    state.box.removeAttribute("aria-activedescendant");
    return;
  }
  row.item.setAttribute("aria-selected", "true");
  state.box.setAttribute("aria-activedescendant", row.item.id);
  row.item.scrollIntoView({ block: "nearest" });
}

function openPicker(state) {
  if (!state.panel.hidden) return;
  if (pickerRows(state).length === 0 && !state.empty.textContent) return;
  claimOpenPanel(state.close);
  state.panel.hidden = false;
  state.box.setAttribute("aria-expanded", "true");
  keepAnchored(state.panel, state.root.querySelector(".nd-tag-picker-field"), {
    width: "--nd-tag-picker-width",
  }, state.close);
}

function closePicker(state) {
  if (state.panel.hidden) return;
  releaseAnchor(state.panel);
  releaseOpenPanel(state.close);
  state.panel.hidden = true;
  state.box.setAttribute("aria-expanded", "false");
  state.box.removeAttribute("aria-activedescendant");
  state.active = -1;
}

function onPickerKey(state, event) {
  if (event.isComposing) return;

  // A separator is a keystroke, never an encoding — which is the whole reason
  // this is a `<select multiple>` and not one string with commas in it.
  if (state.separators.includes(event.key)) {
    event.preventDefault();
    commit(state, state.box.value);
    return;
  }

  switch (event.key) {
    case "Enter": {
      const row = state.rows[state.active];
      if (row) {
        event.preventDefault();
        commit(state, row.option ? row.option.value : state.box.value);
        return;
      }
      // With something typed, Enter commits it rather than submitting: a
      // reader whose last keystroke was Enter did not mean to send the page.
      if (state.box.value.trim() !== "") {
        event.preventDefault();
        commit(state, state.box.value);
      }
      return;
    }
    case "Backspace": {
      if (state.box.value !== "") return;
      const chosen = [...state.native.selectedOptions].filter((option) => option.value !== "");
      const last = chosen.at(-1);
      if (last) {
        event.preventDefault();
        remove(state, last, chosen.length - 1);
      }
      return;
    }
    case "ArrowDown":
      event.preventDefault();
      openPicker(state);
      movePicker(state, 1);
      return;
    case "ArrowUp":
      event.preventDefault();
      openPicker(state);
      movePicker(state, -1);
      return;
    case "Escape":
      if (state.panel.hidden) return;
      event.preventDefault();
      closePicker(state);
      return;
    default:
  }
}

/**
 * A pasted list is many values, and it is committed as one batch.
 *
 * Five hundred separate commits would be five hundred rebuilds, five hundred
 * repaints and a thousand dispatched events — every consumer's `onChange`
 * running five hundred times. So the store is filled first and the page is
 * drawn once.
 */
function onPickerPaste(state, event) {
  if (!state.paste) return;
  const text = event.clipboardData?.getData("text") ?? "";
  // SPLIT WITHOUT A REGEX. Building a character class out of the separators
  // means escaping them, and `\,` inside one is an invalid escape under the
  // `u` flag — so the `RegExp` constructor threw, the listener died, and a
  // paste of five addresses added none of them with nothing in the console
  // that named the separator as the cause.
  const cuts = new Set([...state.separators, "\n", "\r"]);
  if (![...text].some((ch) => cuts.has(ch))) return;
  event.preventDefault();
  const quiet = { quiet: true };
  const pieces = [];
  let piece = "";
  for (const ch of text) {
    if (cuts.has(ch)) {
      pieces.push(piece);
      piece = "";
    } else {
      piece += ch;
    }
  }
  pieces.push(piece);
  let added = 0;
  for (const one of pieces) {
    if (commit(state, one, quiet)) added += 1;
  }
  if (added > 0) {
    announcePicker(state);
    sayPicker(state, fillMessage(state.words.status, { count: added }, added));
  }
}

// --- The tree ----------------------------------------------------------------

// A HIERARCHY, AND EVERY ROLE IN IT WRITTEN HERE.
//
// The markup ships a nested `<ul>` of links and claims nothing: no
// `role="tree"`, no `aria-expanded`, no `tabindex`. With no script that page is
// complete and correct, and nothing on it promises a keyboard contract nothing
// is answering — `Segmented`'s rule, and `Table`'s reason for refusing
// `role="grid"`.
//
// WHY THIS IS NOT `stops()` AND `roving()`.
//
// Those two are one implementation for `role="toolbar"` and `role="radiogroup"`
// and their comment block is a proof about exactly two roles. A tree breaks
// both halves of it: the walk is over VISIBLE nodes, which skips a collapsed
// subtree and crosses levels, and Left and Right are structural rather than
// directional — collapse-or-parent and expand-or-first-child. About forty per
// cent of the keyboard is the flat model and the rest is not, so what is shared
// is the one sentence they have in common, `placeRoving` below, and the rest is
// written beside it rather than folded into it.

const TREE = "[data-nd-tree]";
const trees = new WeakMap();

/** One element carries `tabindex=0` and the rest `-1`. The shared sentence. */
function placeRoving(list, active) {
  if (list.length === 0) return;
  const at = Math.max(0, Math.min(active, list.length - 1));
  list.forEach((el, index) => {
    el.tabIndex = index === at ? 0 : -1;
  });
}

function enhanceTrees(root = document) {
  for (const root_ of root.querySelectorAll(TREE)) {
    if (trees.has(root_)) continue;
    const state = buildTree(root_);
    if (state) trees.set(root_, state);
  }
}

/**
 * Takes the behaviour back off every enhanced tree under `root`.
 *
 * A BEHAVIOUR THAT CANNOT BE RELEASED IS A DEFECT FOR EVERY CONSUMER WITH A
 * CLIENT-SIDE ROUTER. A tree binds only to its own elements, so what this
 * mostly buys is the right to enhance the same markup again: the guard is the
 * `trees` map, and an entry nothing removes is a tree that can never be rebuilt.
 */
function releaseTrees(root = document) {
  for (const element of root.querySelectorAll(TREE)) {
    const state = trees.get(element);
    if (!state) continue;
    state.binding.abort();
    trees.delete(element);
  }
}

function buildTree(root_) {
  const binding = new AbortController();
  const signal = binding.signal;
  const state = {
    // CARRIED ON THE STATE, because every helper below already takes it. A
    // parallel argument threaded through a dozen call sites is a dozen places
    // for one of them to be forgotten.
    binding,
    signal,
    root: root_,
    name: root_.dataset.ndTree,
    selection: root_.dataset.ndSelection ?? "none",
    form: root_.dataset.ndForm ?? "",
    words: {
      expand: root_.dataset.ndExpandLabel ?? "",
      collapse: root_.dataset.ndCollapseLabel ?? "",
      empty: root_.dataset.ndEmptyLabel ?? "",
    },
    anchor: null,
    typed: "",
    typedAt: 0,
  };

  root_.setAttribute("role", "tree");
  if (state.selection === "multiple") root_.setAttribute("aria-multiselectable", "true");
  // IT READS PRINTABLE KEYS, AND SAYS SO. Without this `isTyping` does not
  // recognise a focused tree node as somewhere letters belong, and `/` opens
  // the search palette out from under a reader who was typing a node's name to
  // jump to it — the hotkey listener is registered first, so it would win.
  root_.dataset.ndTypeahead = "";

  let index = 0;
  for (const node of root_.querySelectorAll(".nd-tree-item")) {
    node.id = node.id || `nd-tree-${state.name}-${index}`;
    index += 1;
    node.setAttribute("role", "treeitem");

    const group = node.querySelector(":scope > .nd-tree-group");
    const kids = group?.querySelector(":scope > .nd-tree-item");
    const branch = Boolean(kids) || node.dataset.ndBranch === "true";
    node.dataset.ndLeaf = String(!branch);
    if (group) {
      group.setAttribute("role", "group");
      // A BRANCH THAT OPENS ONTO A GAP SAYS SO, AND IT USED TO SAY NOTHING.
      //
      // `empty_label` reached `state.words.empty` and stopped there: no
      // element, no rule, nothing on the page — under a contract telling a
      // caller to set it so exactly this row is explained. With the word the
      // group holds one row that carries it; without one the group is empty,
      // and an empty `role="group"` is a container announcing nothing.
      // A BRANCH THAT CLAIMED CHILDREN, NOT A LEAF. Every node renders a
      // group, so `!kids` alone is true of every leaf on the tree — and a leaf
      // is not a row opening onto a gap, it is a row with nothing under it by
      // design. `data-nd-branch` is the claim this explains.
      if (!kids) {
        if (branch && state.words.empty) {
          const note = document.createElement("li");
          note.className = "nd-tree-empty";
          note.dataset.ndTreeEmpty = "";
          // Not a `treeitem`: it is the group explaining itself, not a row a
          // reader can walk to, choose or open.
          note.setAttribute("role", "none");
          note.textContent = state.words.empty;
          group.append(note);
        } else {
          group.removeAttribute("role");
        }
      }
    }

    const toggle = node.querySelector(":scope > .nd-tree-row > [data-nd-tree-toggle]");
    if (toggle) {
      // A LEAF KEEPS ITS TOGGLE HIDDEN. `aria-expanded` on a node with nothing
      // under it announces a control that is not there — the same reason
      // `TableHeaderCell` writes `aria-sort` only on a sortable column.
      if (branch) {
        toggle.hidden = false;
        // Not a tab stop of its own: the tree is one stop and the arrows do
        // this. It is here for a pointer, and for nothing else.
        toggle.tabIndex = -1;
        toggle.addEventListener("click", (event) => {
          event.stopPropagation();
          setOpen(state, node, node.getAttribute("aria-expanded") !== "true");
          focusNode(state, node);
        }, { signal });
      }
      // The link inside the row is untouched except for this: the tree is one
      // tab stop, so everything inside it is reached with the arrows.
      const link = node.querySelector(":scope > .nd-tree-row a");
      if (link) link.tabIndex = -1;
    }

    if (branch) setOpen(state, node, node.dataset.ndExpanded === "true", { quiet: true });
    if (state.selection !== "none") {
      node.setAttribute("aria-selected", String(node.dataset.ndSelected === "true"));
      if (state.form) addBox(state, node);
    }
  }

  root_.addEventListener("keydown", (event) => onTreeKey(state, event), { signal });
  root_.addEventListener("pointerdown", (event) => {
    const node = event.target.closest?.(".nd-tree-item");
    if (!node || !root_.contains(node)) return;
    if (event.target.closest("[data-nd-tree-toggle]")) return;
    focusNode(state, node);
    if (state.selection !== "none") {
      chooseNode(state, node, { add: event.ctrlKey || event.metaKey, range: event.shiftKey });
    }
  }, { signal });

  root_.dataset.ndEnhanced = "true";
  placeRoving(visibleNodes(state), 0);
  return state;
}

/**
 * The hidden checkbox a selectable node submits through.
 *
 * `Table`'s ruling, applied: "selection is a real checkbox, whose state the
 * browser announces without being asked". Hidden rather than styled away — the
 * attribute takes it out of the tab order and out of the accessibility tree in
 * one move, and a hidden form control still submits, which is the whole
 * arrangement. It is the ONE stored fact; `aria-selected` is derived from it
 * wherever selection changes, so the two can never come apart.
 */
function addBox(state, node) {
  const box = document.createElement("input");
  box.type = "checkbox";
  box.hidden = true;
  box.name = state.form;
  box.value = node.dataset.ndValue || node.querySelector(".nd-tree-label")?.textContent || "";
  box.checked = node.dataset.ndSelected === "true";
  node.querySelector(":scope > .nd-tree-row").append(box);
}

/** Every node a reader can currently see, in the order they see them. */
function visibleNodes(state) {
  return [...state.root.querySelectorAll(".nd-tree-item")].filter((node) => {
    const parent = node.parentElement?.closest(".nd-tree-item");
    for (let up = parent; up; up = up.parentElement?.closest(".nd-tree-item")) {
      if (up.getAttribute("aria-expanded") === "false") return false;
    }
    return true;
  });
}

function setOpen(state, node, open, { quiet = false } = {}) {
  node.setAttribute("aria-expanded", String(open));
  node.dataset.ndExpanded = String(open);
  const group = node.querySelector(":scope > .nd-tree-group");
  if (group) group.hidden = !open;
  const toggle = node.querySelector(":scope > .nd-tree-row > [data-nd-tree-toggle]");
  if (toggle) {
    const words = open ? state.words.collapse : state.words.expand;
    const label = node.querySelector(".nd-tree-label")?.textContent ?? "";
    const named = fillMessage(words, { label });
    if (named) {
      toggle.setAttribute("aria-label", named);
      toggle.setAttribute("title", named);
    }
  }
  if (!quiet) placeRoving(visibleNodes(state), visibleNodes(state).indexOf(node));
}

function focusNode(state, node) {
  const nodes = visibleNodes(state);
  placeRoving(nodes, nodes.indexOf(node));
  node.focus();
}

/**
 * Chooses a node, and keeps the checkbox and `aria-selected` in one direction.
 *
 * The checkbox is the stored state and `aria-selected` is written from it, here
 * and nowhere else — which is the "two facts kept in step" trap `TableRow`
 * warns about, avoided by having only one fact.
 */
function chooseNode(state, node, { add = false, range = false } = {}) {
  if (node.getAttribute("aria-disabled") === "true") return;
  const nodes = visibleNodes(state);
  const single = state.selection === "single";

  let wanted;
  if (single || (!add && !range)) {
    wanted = new Set([node]);
  } else if (range && state.anchor) {
    const from = nodes.indexOf(state.anchor);
    const to = nodes.indexOf(node);
    wanted = new Set(nodes.slice(Math.min(from, to), Math.max(from, to) + 1));
    for (const other of nodes) {
      if (other.getAttribute("aria-selected") === "true") wanted.add(other);
    }
  } else {
    wanted = new Set(nodes.filter((n) => n.getAttribute("aria-selected") === "true"));
    if (wanted.has(node)) wanted.delete(node);
    else wanted.add(node);
  }

  if (!range) state.anchor = node;
  for (const other of state.root.querySelectorAll(".nd-tree-item")) {
    const on = wanted.has(other);
    const box = other.querySelector(':scope > .nd-tree-row > input[type="checkbox"]');
    if (box) box.checked = on;
    other.setAttribute("aria-selected", String(on));
    other.dataset.ndSelected = String(on);
  }
}

function onTreeKey(state, event) {
  if (event.isComposing) return;
  const node = event.target.closest?.(".nd-tree-item");
  if (!node || !state.root.contains(node)) return;
  const nodes = visibleNodes(state);
  const at = nodes.indexOf(node);
  const open = node.getAttribute("aria-expanded");

  const go = (to) => {
    const next = nodes[Math.max(0, Math.min(to, nodes.length - 1))];
    if (!next) return;
    focusNode(state, next);
    // Shift extends; Control moves without choosing, which is the whole reason
    // it exists — a reader reaching a distant node and adding it with Space.
    if (state.selection === "multiple" && event.shiftKey) chooseNode(state, next, { range: true });
    else if (state.selection === "single" && !event.ctrlKey && !event.metaKey) chooseNode(state, next);
  };

  switch (event.key) {
    case "ArrowDown":
      event.preventDefault();
      go(at + 1);
      return;
    case "ArrowUp":
      event.preventDefault();
      go(at - 1);
      return;
    case "ArrowRight":
      event.preventDefault();
      // Open it, or step into it. On a leaf, nothing — which is how a reader
      // learns the row has nothing under it.
      if (open === "false") setOpen(state, node, true);
      else if (open === "true") go(at + 1);
      return;
    case "ArrowLeft": {
      event.preventDefault();
      // Close it, or climb out. That is how a reader leaves without walking
      // back up through everything they opened.
      if (open === "true") {
        setOpen(state, node, false);
        return;
      }
      const parent = node.parentElement?.closest(".nd-tree-item");
      if (parent) focusNode(state, parent);
      return;
    }
    case "Home":
      event.preventDefault();
      go(0);
      return;
    case "End":
      event.preventDefault();
      go(nodes.length - 1);
      return;
    case "*": {
      event.preventDefault();
      // Every branch at THIS level, and no deeper: opening everything is a
      // different request nobody has made.
      const siblings = [...(node.parentElement?.children ?? [])].filter((el) =>
        el.classList.contains("nd-tree-item"),
      );
      for (const sibling of siblings) {
        if (sibling.dataset.ndLeaf === "false") setOpen(state, sibling, true, { quiet: true });
      }
      placeRoving(visibleNodes(state), visibleNodes(state).indexOf(node));
      return;
    }
    case "Enter": {
      const link = node.querySelector(":scope > .nd-tree-row a");
      if (link) {
        event.preventDefault();
        if (event.metaKey || event.ctrlKey) window.open(link.href, "_blank", "noopener");
        else link.click();
        return;
      }
      if (open !== null) {
        event.preventDefault();
        setOpen(state, node, open !== "true");
      }
      return;
    }
    case " ":
      // REFUSED BEFORE IT IS ANSWERED. `tree.toml` promises that on a tree
      // with no selection Space "does nothing rather than scrolling", and this
      // returned before `preventDefault` — so the browser's own default ran
      // and the page jumped a screen under a reader whose focus was on a row.
      // Doing nothing is a thing that has to be done.
      event.preventDefault();
      if (state.selection === "none") return;
      chooseNode(state, node, { add: true });
      return;
    case "a":
    case "A":
      // Scoped to a tree that has multiple selection; anywhere else this stays
      // the browser's own *select all*.
      if (state.selection !== "multiple" || !(event.ctrlKey || event.metaKey)) break;
      event.preventDefault();
      for (const other of nodes) chooseNode(state, other, { add: true });
      return;
    default:
  }

  // Type-to-jump, on the same window and the same accumulation `Select` uses.
  // `isTyping` knows about this because the row carries `data-nd-typeahead`,
  // which is what stops `/` opening the search palette out from under it.
  if (event.key.length !== 1 || event.metaKey || event.ctrlKey || event.altKey) return;
  const now = performance.now();
  state.typed = now - state.typedAt > TYPE_AHEAD ? event.key : state.typed + event.key;
  state.typedAt = now;
  const term = state.typed.toLowerCase();
  const from = nodes.indexOf(node);
  const found = [...nodes.slice(from + 1), ...nodes.slice(0, from + 1)].find((other) =>
    (other.querySelector(".nd-tree-label")?.textContent ?? "").toLowerCase().startsWith(term),
  );
  if (found) {
    event.preventDefault();
    focusNode(state, found);
  }
}

// --- The language menu -----------------------------------------------------

// The redirect itself is the inline script in <head>: it has to run before the
// first paint, because a page corrected afterwards is a page the reader already
// saw in the wrong language. What is left for here is remembering the choice
// and marking the one in force.
//
// Nothing below turns a stored value into a URL. Every entry in the menu is a
// server-rendered `<a href>`, so the browser navigates and this listener only
// writes to storage — which is the same rule the theme settings follow, and the
// reason a corrupt `localStorage` cannot send a reader anywhere.
document.addEventListener("click", (event) => {
  const option = event.target.closest?.("[data-nd-locale]");
  if (!option) return;
  const value = option.dataset.ndLocale;
  if (value !== AUTOMATIC_LOCALE && !offeredLocales.includes(value)) return;
  try {
    localStorage.setItem(LOCALE_KEY, value);
  } catch {
    // Private browsing: the link still navigates, the choice is not kept.
  }
});

/** Marks the language in force, which is a stored answer and not the URL's. */
function refreshLocale() {
  let stored = AUTOMATIC_LOCALE;
  try {
    const value = localStorage.getItem(LOCALE_KEY);
    if (value === AUTOMATIC_LOCALE || offeredLocales.includes(value)) stored = value;
  } catch {
    // As above.
  }
  for (const option of document.querySelectorAll("[data-nd-locale]")) {
    if (option.dataset.ndLocale === stored) {
      option.setAttribute("aria-current", "true");
    } else {
      option.removeAttribute("aria-current");
    }
  }
}

refreshLocale();

// The colors page shows every palette at once, so it is the one page that
// legitimately links all thirty-nine stylesheets — about 650 KB gzipped, which
// is why no other page does.
//
// Each cell is a `data-palette` scope, so `dist/scope.css` re-resolves the whole
// semantic contract inside it and every chip shows THAT palette's colors. But a
// scope can only resolve what has been loaded: without these links the grid
// renders thirty-nine cells that all paint the reader's own palette, which is
// exactly what it did until this was written. The markup was right, the CSS was
// right, and nothing connected them.
if (document.querySelector("[data-nd-all-palettes]")) {
  for (const palette of paletteValues.values) ensureStylesheet(palette);
  // Every sheet is loaded here anyway, so the swatches outside the settings
  // panel — the accent axis on this page — can be painted immediately rather
  // than waiting for a panel the reader may never open.
  paintSwatches();
}

// --- The drawer ------------------------------------------------------------

/**
 * Opens the drawer, or closes it — and does nothing at all otherwise.
 *
 * THE GUARD IS NOT AN OPTIMISATION. The exchange below is a SWAP: it puts the
 * name the control is not showing into `aria-label` and files the one it was
 * showing under `data-nd-label-alt`. That is only correct once per real change
 * of state, and this function used to be called with the state it was already
 * in — every click on a link in the navigation calls it with `false`, whether
 * or not the drawer was ever open.
 *
 * It was invisible while a link took the whole page with it: the swap happened
 * and the document was gone a moment later. `navigate.js` keeps the document,
 * so the first click on any sidebar row renamed the closed hamburger to *Close
 * the menu* and marked it `aria-pressed="false"` — a control announcing the
 * opposite of what it does, on a page that had not moved. The byte-for-byte
 * comparison in `navigation.spec.js` is what found it.
 */
function setDrawer(open) {
  if (open === (root.getAttribute("data-nd-nav") === "open")) return;
  if (open) {
    root.setAttribute("data-nd-nav", "open");
  } else {
    root.removeAttribute("data-nd-nav");
  }
  const button = document.querySelector(MENU_BUTTON);
  if (!button) return;
  button.setAttribute("aria-expanded", String(open));
  // A button whose label never changes tells a screen-reader user the menu is
  // closed while they are standing in it. Both names come from the component's
  // `label`/`label_alt` props — `data-nd-open` and `data-nd-close` were two
  // attributes invented here, on markup written by hand, for want of a
  // component that could say "this control swaps its own name".
  //
  // `aria-pressed` is what the swapped GLYPH follows, in CSS, so the icon and
  // the name cannot disagree about which state they are drawing.
  button.setAttribute("aria-pressed", String(open));
  // A straight exchange, so the two names cannot get out of step however many
  // times the drawer is opened.
  const alt = button.dataset.ndLabelAlt;
  if (!alt) return;
  button.dataset.ndLabelAlt = button.getAttribute("aria-label");
  button.setAttribute("aria-label", alt);
  button.setAttribute("title", alt);
}

document.querySelector(MENU_BUTTON)?.addEventListener("click", () => {
  setDrawer(root.getAttribute("data-nd-nav") !== "open");
});

document
  .querySelector("[data-nd-scrim]")
  ?.addEventListener("click", () => setDrawer(false));

// Escape closes whatever is open. Not a focus trap — just the one key a reader
// expects to back out of something.
document.addEventListener("keydown", (event) => {
  if (event.key !== "Escape") return;

  // A component that handled Escape itself has already said so. `Search`
  // does — it clears on the first press and closes on the second — and the
  // branch that used to live here reached into a field this file no longer
  // owns.
  if (event.defaultPrevented) return;

  setDrawer(false);
  for (const panel of document.querySelectorAll(`${MENU}[open]`)) {
    panel.removeAttribute("open");
  }
});

// --- One roving-focus model, for both roles ---------------------------------

// Twelve groups and forty-two buttons meant forty-two tab stops. A radiogroup
// is ONE stop: Tab enters it, arrows move within it, Tab leaves. That is what
// every native radio group does and what a reader expects from `role="radio"`.
// A `role="toolbar"` promises exactly the same thing one level up, and there is
// one implementation of it here rather than two that disagree at the boundary
// where a `Segmented` sits inside a `Toolbar`.
//
// THE STOP LIST IS FLAT, AND THAT IS THE WHOLE TRICK.
//
// A toolbar containing a radio group has two plausible readings — the group is
// one stop, or each radio is — and the ARIA practices settle it: the radios
// join the toolbar's list, and moving onto one CHOOSES it, because that is what
// a radio group does anyway. So the toolbar needs no special case for the group
// and the group needs none for the toolbar. `.click()` below is what makes
// selection follow focus, and it is a no-op on an ordinary button because
// nothing is listening for a click that did not come from a pointer.

/**
 * Every control inside `container` that the arrows should walk.
 *
 * A TOOLBAR'S LIST IS FLAT AND INCLUDES A NESTED GROUP'S RADIOS. That is the
 * whole reason there is one implementation here rather than two: the ARIA
 * practices settle the ambiguity by having the radios join the toolbar's list,
 * so the toolbar needs no special case for the group and the group needs none
 * for the toolbar. A nested TOOLBAR would own its own stops, and is the one
 * thing excluded.
 *
 * A radiogroup's list is its own radios, and only its own.
 */
function stops(container) {
  const role = container.getAttribute("role");
  const toolbar = role === "toolbar";
  // A THIRD ROLE, THE SAME WALK. A tab list is one stop with arrows inside it,
  // exactly as a radio group is, and the only place the two differ is whether
  // moving also chooses — which is one line in the handler below rather than a
  // second implementation of everything above it.
  const selector = toolbar
    ? '[role="radio"], button, a[href], summary, [tabindex]'
    : role === "tablist"
      ? '[role="tab"]'
      : '[role="radio"]';
  return [...container.querySelectorAll(selector)].filter((el) => {
    if (el.hasAttribute("disabled")) return false;
    if (el.getAttribute("aria-hidden") === "true") return false;
    // A DISCLOSURE INSIDE A BAR IS ONE STOP: ITS SUMMARY.
    //
    // The example frame's toolbar opens a theme menu with thirty controls in
    // it. They are a region the reader tabs into, not more of the bar — a
    // toolbar whose arrows walked into a panel would be a bar with no end. The
    // summary is the stop; what it opens is not.
    const panel = el.closest("details");
    if (panel && container.contains(panel) && el.tagName !== "SUMMARY") return false;
    if (toolbar) {
      if (el.closest('[role="toolbar"]') !== container) return false;
      // AND A ROLE THAT WALKS ITS OWN CONTENT KEEPS IT. The selector above
      // collects `a[href]`, so every link of a list inside a bar joined the
      // bar's stop list — which nothing on this site did yet, and a `Tree` in a
      // toolbar would have done on the first try. A nested toolbar already got
      // this test; a nested listbox, tree or menu had none.
      const owner = el.closest(
        '[role="tree"], [role="listbox"], [role="menu"], [role="grid"], [role="tablist"]',
      );
      return !(owner && container.contains(owner));
    }
    return el.closest('[role="radiogroup"], [role="tablist"]') === container;
  });
}

/** Puts the one tab stop on the active control and `-1` on the rest. */
function roving(container) {
  const list = stops(container);
  if (list.length === 0) return;
  // The chosen one if there is one, the one that already had focus otherwise,
  // and the first as a last resort — so leaving a bar and coming back lands
  // where the reader left it.
  // A radio says `aria-checked` and a tab says `aria-selected`. Same question.
  const checked = list.findIndex(
    (el) =>
      el.getAttribute("aria-checked") === "true" || el.getAttribute("aria-selected") === "true",
  );
  const held = list.findIndex((el) => el.tabIndex === 0);
  const active = checked !== -1 ? checked : held !== -1 ? held : 0;
  list.forEach((el, index) => {
    el.tabIndex = index === active ? 0 : -1;
  });
}

function refreshRoving() {
  // THE GROUP'S OWN STOP IS THE MARKUP'S, AND IT ENDS HERE.
  //
  // `Segmented` renders `tabindex="0"` on the group, because with no script a
  // group whose items all say `-1` is a radiogroup no keyboard can reach — and
  // an item cannot know whether any of its siblings is chosen. Once this pass
  // has run a real radio holds the stop, so a group that kept its own would be
  // a second one. Inside a toolbar it would be worse: `stops()` collects
  // `[tabindex]`, so the group itself would join the bar's walk.
  for (const group of document.querySelectorAll('[role="radiogroup"][tabindex]')) {
    group.removeAttribute("tabindex");
  }
  // Toolbars first, then the radiogroups that are NOT inside one: a group
  // within a toolbar has already had its radios placed by the toolbar's pass,
  // and running the group's own pass afterwards would put a second `0` on the
  // bar — two tab stops where the role promises one.
  for (const bar of document.querySelectorAll('[role="toolbar"]')) roving(bar);
  for (const group of document.querySelectorAll('[role="radiogroup"], [role="tablist"]')) {
    if (!group.closest('[role="toolbar"]')) roving(group);
  }
}

document.addEventListener("keydown", (event) => {
  const nearest = event.target?.closest?.('[role="radiogroup"], [role="toolbar"], [role="tablist"]');
  if (!nearest) return;
  // A group inside a toolbar does not own its arrows — the bar does, and its
  // list already contains these radios. Handing them to the group would stop
  // the arrows at the group's edge, halfway along the bar.
  const container = nearest.closest('[role="toolbar"]') ?? nearest;

  const vertical = container.getAttribute("aria-orientation") === "vertical";
  // The axis is honoured rather than ignored: a horizontal bar that answered
  // Up and Down would swallow the keys the reader uses to scroll past it.
  const step = {
    ArrowRight: vertical ? 0 : 1,
    ArrowLeft: vertical ? 0 : -1,
    ArrowDown: vertical ? 1 : 0,
    ArrowUp: vertical ? -1 : 0,
  }[event.key];

  const list = stops(container);
  // The event may land on the glyph inside a control rather than the control,
  // so the stop is whichever one CONTAINS the target.
  const at = list.findIndex((el) => el === event.target || el.contains(event.target));
  if (at === -1) return;

  let next = null;
  if (step) next = list[(at + step + list.length) % list.length];
  else if (event.key === "Home") next = list[0];
  else if (event.key === "End") next = list[list.length - 1];
  if (!next) return;

  event.preventDefault();
  next.focus();
  // Selection follows focus in a radio group, which is the contract the role
  // carries. On a plain toolbar button this reaches no handler.
  if (next.getAttribute("role") === "radio") next.click();
  // AND A TAB IS THE ONE PLACE IT MAY NOT. `manual` is what a panel that costs
  // something to show asks for: the arrows move, and the reader says when.
  else if (
    next.getAttribute("role") === "tab" &&
    container.getAttribute("data-nd-activation") !== "manual"
  ) {
    next.click();
  }
  roving(container);
});

// Following a link inside the drawer closes it, or the destination arrives
// behind a menu the reader has to dismiss.
//
// BY ID, NOT BY WHATEVER HAPPENS TO BE INSIDE IT. This was
// `querySelector("[data-nd-search]")?.closest(".nd-nav")` — the drawer found
// through the search box that used to sit in it. The moment that attribute
// moved, `?.` swallowed the miss and the drawer silently stopped closing on a
// link, with nothing anywhere to say so.
document.getElementById("nd-nav")?.addEventListener("click", (event) => {
  if (event.target.closest("a")) setDrawer(false);
});

/** The CLDR plural categories, which is what a variant may be named after. */
const PLURAL = /^(zero|one|two|few|many|other)$/;

/**
 * Picks the variant of a message that agrees with a number.
 *
 * A MESSAGE WITH A NUMBER IN IT IS SEVERAL MESSAGES. `{count} results` printed
 * `1 results` in English and `1 resultados` in Portuguese, which is the exact
 * failure the `status` prop's own contract names — "word order and agreement
 * are not the same in every language" — committed by the message the contract
 * was written about.
 *
 * The form is `one:{count} result|other:{count} results`: still ONE key, one
 * translation, one thing to translate, split at a delimiter the translator
 * writes and controls. That is the same argument `Message`'s inline marks
 * make, and it is not `NOCTUA.md §9.9`'s forbidden assembly — nothing here
 * concatenates two lookups.
 *
 * Which forms a language has is not something this file may decide: Portuguese
 * has two and Polish has three, and a hand-written `count === 1` is English
 * grammar applied to every language on earth. `Intl.PluralRules` is the
 * platform's answer and a built-in rather than a dependency, like `normalize`
 * in the matcher. A message with no `|` is returned untouched, so every
 * existing message keeps working and only the ones that need this pay for it.
 */
function pluralize(template, count) {
  if (!template.includes("|")) return template;
  const forms = new Map();
  for (const part of template.split("|")) {
    const at = part.indexOf(":");
    const name = at < 0 ? "" : part.slice(0, at).trim();
    if (at < 0 || !PLURAL.test(name)) forms.set("other", part);
    else forms.set(name, part.slice(at + 1));
  }
  let category = "other";
  try {
    category = new Intl.PluralRules(document.documentElement.lang || "en").select(count);
  } catch {
    // An unknown tag — the pseudo-locale is one — falls back rather than
    // throwing inside a keystroke handler.
  }
  return forms.get(category) ?? forms.get("other") ?? template;
}

/** Substitutes `{name}` placeholders. The only sanctioned way to build a message. */
function fillMessage(template, args, count = null) {
  if (!template) return "";
  const chosen = count === null ? template : pluralize(template, count);
  return chosen.replace(/\{(\w+)\}/g, (whole, name) =>
    Object.hasOwn(args, name) ? String(args[name]) : whole,
  );
}

// --- Themed scopes ---------------------------------------------------------

// A scope's controls change the SAME element the scope stamped, never a wrapper
// or a child: the color system's forced-mode rule is compound and
// same-element, and splitting the palette from the mode makes it match nothing.
//
// Each scope remembers what it explicitly ASKED for, so the chain can be
// re-resolved from the top whenever anything above it changes. A scope that
// only asked for a mode has to pick up its parent's new palette; one that
// stored its resolved answer would keep the old one forever.
/**
 * What a frame seals: the frame, and the stage inside it.
 *
 * BOTH, because a frame is chrome around content — and inside another frame's
 * stage the whole of it, bar and border included, IS content. "A control that
 * changes appearance along with the thing it controls takes away the reader's
 * anchor" was written for a frame sitting on the real page, and that is still
 * where it holds: the page's own header keeps the page's theme whatever a frame
 * on it is showing.
 */

// EVERY STAGE IS REGISTERED, THEMED OR NOT, AND DOCUMENT ORDER IS WHY.
//
// `inheritedAt` reads a request off an ancestor, so an inner scope resolved
// before its outer one has been visited would inherit the page and stamp the
// page's values absolutely — which no later pass undoes. `querySelectorAll`
// answers in document order, so registering every stage up front makes the
// resolver always visit an outer one first. It used to self-heal only because
// every mutation path happened to resolve twice.
//
// The frame element itself is deliberately absent: `themedBy` routes every
// control to `ownStage`, so a frame never carries a request and visiting it
// could only ever stamp nothing.
//
// Visiting a stage costs a `scopeAttributes` call that returns null for one
// that has stated nothing, which stamps and removes nothing.
//
// AND IT IS A STRONG ARRAY THAT ONLY GROWS. `themeScope` and the frame both
// push into it, and nothing ever took anything out, so a stage rendered into
// the page and then taken away again stayed here for ever with the whole
// subtree hanging off it. `resolveScopes` walks the array on every theme
// change, so each leaked stage is also work done for nobody. `forgetScopes`
// below is the eviction.
const scopes = [];

/**
 * Drops every registered scope that has left the document.
 *
 * ASKED OF THE DOCUMENT, NOT OF THE ROOT BEING RELEASED. A stage still on the
 * page is still a scope, whoever released the behaviours inside it, and nothing
 * in `enhanceAll` re-registers one: the frame and `themeScope` do, at moments
 * of their own. Evicting a live stage would silently stop it resolving.
 *
 * A stage React unmounted IS disconnected, which is exactly the case this is
 * for and the only one it claims.
 */
function forgetScopes() {
  for (let at = scopes.length - 1; at >= 0; at -= 1) {
    if (!scopes[at].isConnected) scopes.splice(at, 1);
  }
}
const requests = new WeakMap();
/** What each scope was built with, so a reset has something to return to. */
const initial = new WeakMap();

/**
 * The scopable settings a scope can be BUILT with, read off its own markup.
 *
 * Generated, not listed: `accent` and `saturation` have no attribute of their
 * own — the palette is their product — so what can be read back is exactly the
 * scopable settings that carry one. This was a hand-written array, which is a
 * copy of the spec that stops being one the day a setting is added.
 */
const readableSettings = scopableSettings.filter((name) => model[name].attribute != null);

/**
 * Registers every scope and every stage under `root`, and reads what it asks.
 *
 * THIS USED TO BE A `const` AND A LOOP AT MODULE LEVEL, which was correct while
 * the page was rendered exactly once. `navigate.js` renders `<main>` again, and
 * a stage that arrived in the second render was in nobody's list: it resolved
 * against nothing, so a frame the reader had navigated to could not be themed
 * and its own controls reported the page.
 *
 * IN DOCUMENT ORDER, AND THAT IS THE WHOLE REASON THIS IS A LIST AT ALL.
 * `inheritedAt` reads a request off an ancestor, so an inner scope resolved
 * before its outer one has been visited would inherit the page and stamp the
 * page's values absolutely — which no later pass undoes. `querySelectorAll`
 * answers in document order; appending its answer keeps the invariant, because
 * everything already registered sits outside the subtree being adopted (the old
 * one's entries went in `forgetScopes`, which runs first).
 *
 * ALREADY-REGISTERED ELEMENTS ARE SKIPPED RATHER THAN RE-READ. A second pass
 * over a live scope would overwrite `requests` with what is STAMPED on it,
 * which is the resolver's output rather than the scope's question — so a stage
 * the reader had themed would silently adopt its own resolved answer as an
 * absolute request and stop following the page.
 */
function registerScopes(root = document) {
  for (const element of root.querySelectorAll(`[${scopeAttribute}], [data-nd-frame-stage]`)) {
    if (scopes.includes(element)) continue;
    const request = {};
    for (const name of readableSettings) {
      const value = element.getAttribute(model[name].attribute);
      if (value != null) request[name] = value;
    }
    // A scope with no `data-theme` is following whatever it inherits, which is
    // exactly what an absent value means everywhere else in this system.
    scopes.push(element);
    requests.set(element, request);
    initial.set(element, { ...request });
  }
}

registerScopes();

function documentTheme() {
  const theme = {};
  // Generated, not listed. This was the second hand-written copy of the
  // scopable set: a setting added to `spec/theme.toml` was missing from the
  // document's theme, so every scope inherited `undefined` for it and resolved
  // against nothing.
  for (const name of scopableSettings) {
    theme[name] = read(name);
  }
  return theme;
}

/**
 * WHAT A SCOPE ASKED FOR, AND ONLY THAT.
 *
 * A setting a frame STATES owns its whole subtree — a nested frame, its bar,
 * its border and its own menu panel. A setting nobody stated comes from
 * OUTSIDE, which on an ordinary page is the page and inside another frame is
 * that frame. The request is therefore sparse, and `scopeAttributes` stamps a
 * scope exactly when what it resolves to differs from what reaches it.
 *
 * THIS USED TO DEFAULT A NESTED FRAME'S REQUEST TO THE PAGE'S WHOLE THEME, and
 * that is the defect this rewrite exists to remove. It was written to stop a
 * frame inside a themed stage from showing its parent's answer — but showing
 * its parent's answer is precisely correct: inside the outer stage the inner
 * frame is CONTENT, and content follows the page it is on. What the seal did
 * instead was let the real page reach past a reader's own choice, so setting
 * the outer frame to dark left the inner one light.
 *
 * ONE RESIDUE SURVIVES, and it is not optional. A value spelled as the ABSENCE
 * of its attribute (`omit_when`) means two things at once: "the machine
 * decides" and "whatever is above me". A reader picking `system` inside a dark
 * frame has stated something, and stating it as absence would hand them the
 * frame's dark instead of the machine's light — the same defect, one menu row
 * over. So a STATED value that would be spelled as absence is spelled out,
 * and only when there is something above it that would otherwise be inherited.
 *
 * Driven by `model[name].omitWhen` over the settings the request actually
 * carries, never by the name `mode`: what needs resolving is the property.
 */
function requestAt(element, inherited) {
  const request = requests.get(element) ?? {};
  let spelled = null;
  for (const [name, value] of Object.entries(request)) {
    const omit = model[name]?.omitWhen;
    if (omit == null || value !== omit) continue;
    // Nothing above states it either, so absence already means the machine.
    if (inherited && inherited[name] === omit) continue;
    spelled ??= { ...request };
    spelled[name] = concrete(name, value);
  }
  return spelled ?? request;
}

/**
 * A value the DOM can actually state, given one the environment answers.
 *
 * `omit_when` says a value is stamped as the ABSENCE of its attribute —
 * `system` is not a mode, it is the machine's mode. Absence is also how a
 * subtree says "whatever is above me", and those two meanings are the same
 * markup: a sealed stage that stamped nothing for `system` went on inheriting
 * the enclosing frame's dark, which is the leak this seal exists to close.
 *
 * So a sealed scope asks the environment once and states the answer. Driven by
 * `model[name].omitWhen` rather than by the name `mode`, because what needs
 * resolving is the property, not the setting.
 */
function concrete(name, value) {
  if (model[name]?.omitWhen == null || value !== model[name].omitWhen) return value;
  // The one environment question this system asks. A future setting that
  // delegates to the machine answers its own here.
  if (name === "mode") return matchMedia("(prefers-color-scheme: dark)").matches ? "dark" : "light";
  return value;
}

/**
 * What the theme is at an element, walking every scope above it.
 *
 * BY REGISTRATION, NOT BY THE STAMPED ATTRIBUTE. This found its parent with
 * `closest('[data-noctua-scope]')` — an attribute the resolver itself writes —
 * so what a scope inherited depended on which ancestors happened to be stamped
 * in the current pass. With sparse requests that is a real hazard: theme an
 * inner frame before the outer one and it would inherit the page. Asking
 * `requests` instead makes this a pure function of what has been asked for,
 * with no dependency on the order anything is visited in.
 *
 * Recursive rather than one level up: a scope three deep that only asked for a
 * mode has to inherit the palette from wherever above it was last set, and
 * stopping at the first parent would read that parent's *request* rather than
 * what the parent resolved to.
 */
function inheritedAt(element) {
  for (let up = element.parentElement; up; up = up.parentElement) {
    if (!requests.has(up)) continue;
    const inherited = inheritedAt(up);
    return resolveScope(requestAt(up, inherited), inherited);
  }
  return documentTheme();
}

/**
 * Re-resolves every scope against the theme that actually reaches it.
 *
 * THIS IS THE ONLY PLACE A SCOPE ATTRIBUTE IS WRITTEN OR REMOVED.
 *
 * The resolver says which attributes a scope must carry, and `null` means it
 * must carry none — so `null` is exactly when they have to come OFF. It used to
 * `continue` there, leaving whatever the scope was last stamped with, and the
 * frame's appearance toggle removed two of six by hand on the way out. Four
 * were left behind, and `dist/scope.css` matches `[data-palette]:not(:root)` —
 * so the semantic contract went on resolving at that element, now against the
 * system preference instead of the page, and the subtree could not be
 * un-themed at all.
 *
 * Setting the ones the resolver returned and removing every other name it
 * could have returned makes both halves one loop over one generated list.
 */
function resolveScopes() {
  for (const element of scopes) {
    const inherited = inheritedAt(element);
    const attributes = scopeAttributes(requestAt(element, inherited), inherited) ?? {};
    for (const name of scopeAttributeNames) {
      // Written only when it changes. Every write invalidates style for a
      // subtree that `dist/scope.css` has given 1 767 custom properties, and
      // `refresh()` resolves every scope on every click.
      const value = name in attributes ? attributes[name] : null;
      if (value === null) element.removeAttribute(name);
      else if (element.getAttribute(name) !== value) element.setAttribute(name, value);
    }
    // THE PALETTE THAT WAS STAMPED, not the one that was asked for. A sparse
    // request names no palette although one is always stamped — mode and
    // palette travel together, because the colour system's forced-mode
    // selector is compound and same-element. Ensuring the value actually
    // written is correct by construction rather than by argument.
    const palette = attributes[model.palette.attribute];
    if (palette) ensureStylesheet(palette);
  }
}

/**
 * Applies one setting to one scope, and to nothing above or below it.
 *
 * `accent` and `saturation` are the case worth explaining: neither is a scope
 * attribute of its own — the PALETTE is — so choosing one means moving along
 * that axis while holding the other half of the palette already in force here.
 * Every value is validated against the generated lists before it is used.
 */
function themeScope(element, name, value) {
  // Registered on the way in. An element the resolver has never seen is one it
  // will not visit, so a request set on it would sit there doing nothing — and
  // the frame's stage is exactly that until something themes it for the first
  // time.
  if (!scopes.includes(element)) scopes.push(element);
  const request = requests.get(element) ?? {};
  if (name === "accent" || name === "saturation") {
    // BOTH AXES, AND `saturation` USED TO BE MISSING. `apply` has handled the
    // two together since it was written; this branch handled only the accent,
    // so a reader choosing `vivid` in a frame's menu stored a `saturation` that
    // no attribute is stamped from and no rule reads — the row moved, the chosen
    // state moved, and the frame did not change at all. A control offered where
    // it does nothing is a control that lies, and this one lied silently
    // because the row beside it worked.
    //
    // THE PALETTE IN FORCE HERE, not the page's. Read from the page while every
    // request was absolute, choosing `teal` in a frame that inherits
    // `rose-muted` gave `teal-<the page's saturation>` — two axes moved for one
    // click.
    //
    // The split is unambiguous: no accent contains a hyphen, so `<accent>` is
    // everything before the first one.
    const current = request.palette ?? settingAt(element, "palette");
    const cut = current.indexOf("-");
    const next =
      name === "accent"
        ? `${value}-${current.slice(cut + 1)}`
        : `${current.slice(0, cut)}-${value}`;
    if (!paletteValues.values.includes(next)) return;
    // The axis itself is recorded too, so `settingAt(element, "saturation")`
    // answers for the scope rather than falling through to the page — which is
    // what the frame's own menu reads to mark its rows.
    if (!model[name].values.includes(value)) return;
    request[name] = value;
    request.palette = next;
  } else {
    if (!model[name]?.values.includes(value)) return;
    request[name] = value;
  }
  requests.set(element, request);
  resolveScopes();
}

// The `scope-reset` handler that used to be here is gone with the control it
// answered: no markup emits `data-nd-action="scope-reset"` any more, and
// `pages.rs` records why — the frame's own Reset covers both. It also reached
// into nested frames with `region.contains`, which is the containment mistake
// `own` exists to prevent.

// --- The React path ---------------------------------------------------------
//
// It moved to `implementation.js`. What was here mounted ONE example, from a
// hand-written mirror of it, and called `mount(slot, request, text)` with the
// theme scraped off the rendered DOM and five strings scraped out by CSS
// selector. Every example is data now, so there is nothing to scrape and
// nothing to mirror.

// --- The example frame -----------------------------------------------------

// A FRAME'S PARTS ARE ITS OWN, AND A NESTED FRAME IS NOT PART OF ITS PARENT.
//
// Every handler here used to be a `frame.querySelector*`, which was right while
// the site could only ever have one frame per page — and the frame's own
// documentation page is the counter-example, since it demonstrates a frame by
// putting one inside another. The outer frame then owned ten width radios, two
// stages, two readouts and two resets. Pressing the outer control marked the
// INNER control's matching stop while resizing only the outer stage, so the
// inner one sat there claiming a width it was not.
//
// The nearest frame decides. It is the same test `exampleScope` already made
// for the React mount, applied everywhere it was missing.

/** The matches inside `frame` that belong to `frame` rather than to one in it. */
function own(frame, selector) {
  return [...frame.querySelectorAll(selector)].filter(
    (el) => el.closest("[data-nd-frame]") === frame,
  );
}

/** A frame's own stage. */
function ownStage(frame) {
  return own(frame, "[data-nd-frame-stage]")[0];
}

/**
 * Puts one frame's stage under the glass test, or takes it back out.
 *
 * # Why the material is applied by attribute rather than from a list
 *
 * `glass` is a `bool` prop, and a bool prop always writes its attribute —
 * `data-nd-glass="false"` when it is off. So every control that DECLARES the
 * prop is already selectable, and "frost everything in here that can be
 * frosted" is one query with no list of component names in it. A component
 * that gains the prop next month is picked up with no edit here, which is the
 * same rule the frame's theme seal follows.
 *
 * `own()` throughout: a frame nested inside this one has a glass control of
 * its own three inches away, and reaching into it would put that control out
 * of step with what it is showing.
 */
/** How much blank stays visible above the first example when the rig starts. */
const GLASS_LEAD = 32;

/**
 * The one glass value the rig cannot work with, spelled once.
 *
 * Nothing else here writes a setting's value down — the model publishes them —
 * but "the material is absent" has no name in the model, only a value that
 * means it. Asserted against the generated list so a rename in the spec fails
 * loudly here instead of quietly turning the guard off.
 */
const GLASS_OFF = "off";
if (!glassValues.values.includes(GLASS_OFF)) {
  throw new Error(`the glass setting no longer has a \`${GLASS_OFF}\` value`);
}

function setGlassTest(frame, on) {
  const stage = ownStage(frame);
  if (!stage) return;
  stage.toggleAttribute("data-nd-glass-test", on);
  // On the FRAME too, so the bar's own menu can say why it is refusing `off`.
  // The stage carries the layout; the frame carries the fact.
  frame.toggleAttribute("data-nd-glass-test", on);

  // THE RIG OWNS THE MATERIAL IT TESTS.
  //
  // A control that says "look at the glass" over a stage whose glass is `off`
  // is a control that does nothing, and the reader can turn it off underneath
  // it at any moment. So starting the test turns the material on if it is off,
  // and `off` is refused — reachable and marked, never removed, because
  // `SegmentedItem.disabled` exists so a reader "can find out it exists and is
  // refused instead of meeting a gap where an option used to be".
  //
  // What it was is remembered and put back, the same way every other thing the
  // rig touches is: a reader who had chosen `off` chose it.
  if (on) {
    if (frame.dataset.ndGlassWasSet === undefined) {
      frame.dataset.ndGlassWasSet = requests.get(stage)?.glass ?? "";
    }
    if (settingAt(stage, "glass") === GLASS_OFF) {
      themeScope(stage, "glass", model.glass.default);
    }
  } else if (frame.dataset.ndGlassWasSet !== undefined) {
    const was = frame.dataset.ndGlassWasSet;
    delete frame.dataset.ndGlassWasSet;
    const request = requests.get(stage) ?? {};
    if (was === "") delete request.glass;
    else request.glass = was;
    requests.set(stage, request);
    resolveScopes();
  }
  for (const off of own(frame, '[data-nd-group="glass"] [data-nd-value="off"]')) {
    off.setAttribute("aria-disabled", String(on));
  }
  for (const element of own(frame, "[data-nd-glass]")) {
    // Not the frame's own chrome: the bar is what you are pressing, and a
    // control that changes appearance along with the thing it controls takes
    // away the reader's anchor.
    if (!stage.contains(element)) continue;
    if (on) {
      // WHAT IT WAS, NOT `false`. Some examples are AUTHORED with `glass:
      // true` — that is the prop's own documentation — and switching the test
      // off used to un-frost them, quietly editing the example a reader came
      // to look at into a different one.
      if (element.dataset.ndGlassWas === undefined) {
        element.dataset.ndGlassWas = element.getAttribute("data-nd-glass");
      }
      element.setAttribute("data-nd-glass", "true");
    } else if (element.dataset.ndGlassWas !== undefined) {
      element.setAttribute("data-nd-glass", element.dataset.ndGlassWas);
      delete element.dataset.ndGlassWas;
    }
  }
  // A NESTED FRAME IS A COMPONENT IN THIS STAGE, SO IT IS FROSTED LIKE ONE.
  //
  // Its frame and its bar, not the controls inside it: those belong to the
  // inner frame, which has a control of its own for them, and frosting them
  // inside an already-frosted pane is glass sampling glass. `.nd-frame` carries
  // no `glass` prop — it is the site's own apparatus — but the attribute is the
  // contract the one recipe reads, so stamping it is all this takes.
  for (const nested of stage.querySelectorAll("[data-nd-frame]")) {
    // ITS GLASS LAYER, NEVER THE FRAME AND NEVER THE BAR. A `backdrop-filter`
    // is a containing block for every `position: fixed` descendant, and the
    // frame's own theme menu is one: stamping the frame moved that panel
    // inside a box that clips, and it opened where nobody could see it.
    for (const part of own(nested, ".nd-frame-glass")) {
      if (on) {
        if (part.dataset.ndGlassWas === undefined) {
          part.dataset.ndGlassWas = part.getAttribute("data-nd-glass") ?? "";
        }
        part.setAttribute("data-nd-glass", "true");
      } else if (part.dataset.ndGlassWas !== undefined) {
        const was = part.dataset.ndGlassWas;
        if (was === "") part.removeAttribute("data-nd-glass");
        else part.setAttribute("data-nd-glass", was);
        delete part.dataset.ndGlassWas;
      }
    }
  }

  // THE RIG LANDS ON THE CONTENT. The blank above is a whole stage, which is
  // what lets an example be dragged clear of every shape — and would show the
  // reader a blank field if the scroller stayed where it was. One step of it
  // stays visible above the first example, so the blank reads as blank rather
  // than as the top of the frame.
  const scroll = own(frame, ".nd-glass-scroll")[0];
  if (scroll) {
    // The first real child's own offset, which is the blank spacer above it
    // plus the gap. Read from the layout rather than from the stylesheet: the
    // blank is a `::before` now, and a script that recomputed it from a token
    // would be a second copy of the same length.
    const first = scroll.querySelector(":scope > *");
    scroll.scrollTop = on && first ? Math.max(0, first.offsetTop - GLASS_LEAD) : 0;
  }

  refresh();
  const button = own(frame, '[data-nd-action="frame-glass"]')[0];
  if (button && (button.getAttribute("aria-pressed") === "true") !== on) {
    swapLabel(button, on);
  }
}

/** Puts the frame's stage at one width, and names it on the control. */
function setWidth(frame, value) {
  const stage = ownStage(frame);
  if (!stage) return;
  stage.dataset.ndWidth = value;
  // THE TRIGGER SAYS WHICH WIDTH IS IN FORCE, and no item carries a tick.
  //
  // The width used to be a `Segmented`, where the chosen stop was marked with
  // `aria-checked`. It is a `Menu` now, because five stops do not fit in a rail
  // — and a menu whose items carry a state is a listbox wearing a menu's
  // clothes, which `spec/components/menu.toml` refuses in so many words. So the
  // state lives where it always was, on the stage, and the button that opens
  // the menu reads it back.
  const chosen = own(frame, `[data-nd-action="frame-width"][data-nd-value="${value}"]`)[0];
  const trigger = own(frame, ".nd-frame-width .nd-button-label")[0];
  if (trigger && chosen) {
    trigger.textContent = chosen.querySelector(".nd-menu-item-label")?.textContent ?? value;
  }
}

/**
 * Flips the frame's appearance, and ONLY the frame's.
 *
 * The stage is given a scope of its own, so `dist/scope.css` re-resolves the
 * whole semantic contract at that element and every example under it obeys —
 * while the page around it does not move at all. That is the entire mechanism
 * `ThemeScope` is built on, used here at runtime rather than at render time,
 * for one reason: a scope that changes nothing wraps nothing, so there is
 * nothing to server-render until the reader asks for it. With the script
 * blocked there is no toggle and no scope, and the frame shows the page's
 * appearance — which is correct, and keeps the page whole without JavaScript.
 *
 * Every attribute a scope needs goes on ONE element, because the colour
 * system's forced-mode selector is compound and same-element. `scopeAttributes`
 * is the generated resolver, so no mode string is composed here.
 */
function toggleFrameMode(frame, button) {
  const stage = ownStage(frame);
  if (!stage) return;
  const on = button.getAttribute("aria-pressed") === "true";

  // IT SETS A REQUEST. `resolveScopes` decides the DOM.
  //
  // This used to take the scope off by hand — two `removeAttribute` calls
  // against the six the resolver stamps — and the four it missed left the
  // stage stuck in one mode. An empty request is the way to say "this scope
  // asks for nothing", and clearing it is the resolver's job because the
  // resolver is the half that knows what stamping means.
  if (on) {
    requests.set(stage, {});
  } else {
    // THE OPPOSITE OF WHAT THIS STAGE IS SHOWING, not of what the PAGE is.
    //
    // It read `<html>`'s mode, which is the same thing only while no frame is
    // inside another. On the frame's own documentation page it is not: an
    // inner frame sitting in an inverted outer one asked for the opposite of
    // the page and got the mode it already had, so the resolver — correctly —
    // called it a no-op and stamped nothing. The inner frame could not invert
    // itself at all, which is the one thing the page exists to show.
    //
    // `settingAt` answers where the element SITS, walking every scope above it.
    // The stage's own request is empty on this branch, so what comes back is
    // what it inherits.
    //
    // `system` is a preference rather than an appearance, so it is resolved
    // against the machine before being inverted — flipping `system` to `light`
    // on a dark machine would have moved nothing.
    const showing = concrete("mode", settingAt(stage, "mode"));
    const wanted = showing === "dark" ? "light" : "dark";
    if (!model.mode.values.includes(wanted)) return;
    // Registered once, and never taken out again: a scope whose request is
    // empty resolves to nothing and is cleared, which is the same end state
    // and one fewer thing to keep in step.
    if (!scopes.includes(stage)) scopes.push(stage);
    requests.set(stage, { mode: wanted });
  }

  swapLabel(button, !on);
  resolveScopes();
  refresh();
}

/** Presses a toggle and gives it the other of its two names. */
function swapLabel(button, pressed) {
  button.setAttribute("aria-pressed", String(pressed));
  const alt = button.dataset.ndLabelAlt;
  if (!alt) return;
  button.dataset.ndLabelAlt = button.getAttribute("aria-label");
  button.setAttribute("aria-label", alt);
  button.setAttribute("title", alt);
}

/**
 * Writes the stage's real width into the frame's caption.
 *
 * The control PROMISES 320px; this is the page measuring itself and saying so,
 * which is a different and better claim — and at `full` it is the only way to
 * know what "full" happens to be on this screen. A length read back out of the
 * DOM is fine; a colour is not, and that distinction is the whole of the
 * `color_read_from_dom` gate.
 */
document.addEventListener("click", (event) => {
  const width = event.target.closest?.('[data-nd-action="frame-width"][data-nd-value]');
  if (width) {
    setWidth(width.closest("[data-nd-frame]"), width.dataset.ndValue);
    return;
  }

  const mode = event.target.closest?.('[data-nd-action="frame-mode"]');
  if (mode) {
    toggleFrameMode(mode.closest("[data-nd-frame]"), mode);
    return;
  }

  const glass = event.target.closest?.('[data-nd-action="frame-glass"]');
  if (glass) {
    setGlassTest(glass.closest("[data-nd-frame]"), glass.getAttribute("aria-pressed") !== "true");
    return;
  }

  const reset = event.target.closest?.('[data-nd-action="frame-reset"]');
  if (!reset) return;
  const frame = reset.closest("[data-nd-frame]");
  setWidth(frame, "full");
  setGlassTest(frame, false);

  // The appearance goes back to the page's, if it was ever taken off it.
  const toggle = own(frame, '[data-nd-action="frame-mode"]')[0];
  if (toggle?.getAttribute("aria-pressed") === "true") toggleFrameMode(frame, toggle);

  // A SCOPE INSIDE THE FRAME GOES BACK TO THE THEME IT WAS BUILT WITH.
  //
  // This said `requests.set(element, {})`, which is not that: an empty request
  // means "inherit the page", so resetting the ThemeScope demonstration threw
  // away the `azure-vivid / light / full / round` it was authored with and
  // adopted whatever the reader's own theme happened to be. The scope reset
  // twenty lines up had it right all along.
  // A nested frame is left alone: it has a Reset of its own, three inches
  // away, and reaching into it would put its controls out of step with what it
  // is showing.
  //
  // BY REGISTRATION, NOT BY THE STAMPED ATTRIBUTE. Selecting
  // `[data-noctua-scope]` skipped a stage whose request happens to resolve to
  // no attributes at all — theme a frame to exactly what it already inherits
  // and its own Reset could not reach it, so the stale request sat there and
  // the frame refused to follow the next thing that changed above it.
  for (const element of scopes) {
    if (element.closest("[data-nd-frame]") !== frame) continue;
    requests.set(element, { ...(initial.get(element) ?? {}) });
  }
  // THE IMPLEMENTATION GOES BACK TOO, and `implementation.js` does it — it
  // owns the mount, the caption and the frame's own answer, and doing half of
  // it here would be the second copy that stops agreeing.
  resolveScopes();
  refresh();
});

// --- Start -----------------------------------------------------------------

// The bootstrap already stamped everything before the first paint. This only
// marks the controls and links the palette the reader chose, so nothing that
// happens here can move the page.
ensureStylesheet(read("palette"));
refresh();
refreshRoving();
refreshChoices();
// --- The date input ---------------------------------------------------------

// The component renders a question and three labelled boxes that post three
// values; this adds everything the contract says is the script's to keep:
// the reorder to the reader's locale, the stepping, the paste that fills all
// three, the validation sentences, and the suggestions panel with its quick
// picks and its month grid. With this never running, none of that was
// promised — the markup claims no popup, no combobox, no keyboard beyond a
// text field's own.

const DATE_FIELD = "[data-nd-date-input]";

/** Segment letters to their names, the order attributes' vocabulary. */
const DATE_PARTS = { d: "day", m: "month", y: "year" };

/**
 * The chain a WORD is written in: the page's own language first, then the UA's.
 *
 * Month names, weekday initials and the bound inside `min_label` are all words
 * in a sentence the page is already written in, so they follow the page. A
 * CONVENTION is a different question and has a different answer below.
 */
function dateWords() {
  return [document.documentElement.lang, ...(navigator.languages || [])].filter(Boolean);
}

/**
 * The chain a CONVENTION is read from: the reader's own, and never the page's.
 *
 * THIS FUNCTION IS THE DEFECT. The one above was used for both questions, so
 * the row's order was asked of `document.documentElement.lang` — the language
 * this page happens to be written in, which is `en` on the English build. A
 * bare `en` has no region, so CLDR answers with the United States and every
 * reader of the English page was re-laid to month-day-year. A reader whose
 * browser says `en-GB` was moved OFF the day-month-year the server had already
 * served them, by a detection that never asked what their browser said.
 *
 * An empty chain is returned as an empty array on purpose: `Intl` reads that
 * as *use the default locale*, which is the reader's own by definition. There
 * is no table of locales to formats here and there must not be one — a table
 * is a copy of CLDR that stops being true the day CLDR changes.
 */
function readerLocales() {
  return [...(navigator.languages || []), navigator.language].filter(Boolean);
}

/** A segment's number, judged after Unicode normalisation so fullwidth
 * digits are digits. Null is "not a number", not zero. */
function dateNumber(raw) {
  const clean = raw.trim().normalize("NFKC");
  return /^[0-9]+$/.test(clean) ? Number.parseInt(clean, 10) : null;
}

/** ISO from three numbers, zero-padded — the wire format, never shown. */
function dateIso(year, month, day) {
  return `${String(year).padStart(4, "0")}-${String(month).padStart(2, "0")}-${String(day).padStart(2, "0")}`;
}

/** A bound, written for a reader by the locale's own formatter. */
function dateWorded(iso) {
  const [year, month, day] = iso.split("-").map(Number);
  return new Intl.DateTimeFormat(dateWords(), { dateStyle: "long", timeZone: "UTC" }).format(
    new Date(Date.UTC(year, month - 1, day)),
  );
}

/** Every enhanced date field's teardown, and the guard against a second pass. */
const dates = new WeakMap();

function enhanceDateInputs(root = document) {
  for (const field of root.querySelectorAll(DATE_FIELD)) {
    // THE GUARD THE OTHER FOUR HAD AND THIS ONE DID NOT.
    //
    // `enhanceDateInput` binds a keydown, a paste and two blurs per box, a
    // focusout on the field and a submit on the FORM, and it DRAWS a suggest
    // button into the row. None of that was guarded, so a second pass over the
    // same field bound every listener twice and appended a second button
    // beside the first. One press of the up arrow then stepped the day twice,
    // and the row grew a control per pass.
    //
    // It was latent only because this was called exactly once, at load. It is
    // a live defect the moment anything re-enhances a subtree, whether that is
    // a router or a framework remounting an example.
    if (dates.has(field)) continue;
    const binding = new AbortController();
    dates.set(field, binding);
    enhanceDateInput(field, binding.signal);
  }
}

/**
 * Takes the behaviour back off every date field under `root`.
 *
 * A BEHAVIOUR THAT CANNOT BE RELEASED IS A DEFECT FOR EVERY CONSUMER WITH A
 * CLIENT-SIDE ROUTER, and this field reaches outside itself twice. It binds
 * `submit` to the FORM, which is an ancestor and outlives the field, and it
 * DRAWS a suggest button and a panel into the row. The drawn parts are removed
 * here so a later pass draws one of each rather than a second of each.
 */
function releaseDateInputs(root = document) {
  for (const field of root.querySelectorAll(DATE_FIELD)) {
    const binding = dates.get(field);
    if (!binding) continue;
    binding.abort();
    dates.delete(field);
    field.querySelector(".nd-date-input-suggest-button")?.remove();
    const panel = field.querySelector(".nd-date-input-panel");
    if (panel) {
      releaseAnchor(panel);
      panel.remove();
    }
  }
}

function enhanceDateInput(root, signal) {
  const row = root.querySelector(".nd-date-input-row");
  const slot = root.closest(".nd-field")?.querySelector("[data-nd-field-error]");
  if (!row || !slot) return;
  const item = (part) => root.querySelector(`[data-nd-part="${part}"]`);
  const box = (part) => item(part)?.querySelector("input");
  const labelOf = (part) =>
    item(part)?.querySelector(".nd-date-input-label")?.textContent?.trim() ?? "";
  const boxes = { day: box("day"), month: box("month"), year: box("year") };
  if (!boxes.day || !boxes.month || !boxes.year) return;
  // A sentence the caller rendered is the caller's; the validator neither
  // clears nor overwrites it. `Select` states the rule.
  const saidByCaller = root.dataset.ndInvalid === "true";

  // THE ORDER, DETECTED ONCE AND MOVED IN THE DOCUMENT. Never CSS `order`:
  // the eyes, the Tab key and a screen reader must walk one row. Skipped if
  // something already has focus in here — the ground under a caret does not
  // move.
  //
  // THE ORDER COMES FROM THE PLATFORM, AND FROM THE READER. `formatToParts`
  // is asked to write one numeric date and the answer is read back part by
  // part, so what stands here is whatever CLDR says today for the locale the
  // reader's browser asked for — see `readerLocales`, which is where this was
  // wrong. The served `dmy` is the answer for a page with no script, not a
  // default this detection may fall back on: a fallback that cannot be told
  // from an answer is the defect, so `wanted` only moves on a part list this
  // recognises.
  if (root.dataset.ndOrder === "auto") {
    let wanted = "dmy";
    try {
      const parts = new Intl.DateTimeFormat(readerLocales(), {
        day: "numeric",
        month: "numeric",
        year: "numeric",
      }).formatToParts(new Date(Date.UTC(2001, 1, 3)));
      const seen = parts
        .filter((part) => part.type === "day" || part.type === "month" || part.type === "year")
        .map((part) => part.type[0])
        .join("");
      if (seen === "mdy" || seen === "ymd" || seen === "dmy") wanted = seen;
    } catch {
      wanted = "dmy";
    }
    if (wanted !== root.dataset.ndResolved && !root.contains(document.activeElement)) {
      for (const letter of wanted) row.append(item(DATE_PARTS[letter]));
      root.dataset.ndResolved = wanted;
    }
  }

  /** The three boxes in visual order, which is document order. */
  const inOrder = () =>
    [...row.querySelectorAll(".nd-date-input-segment")].filter(Boolean);

  // --- The sentences -------------------------------------------------------

  const say = (sentence, wrong) => {
    // THE WORD IN FRONT IS `Field`'S NOW. This built its own copy from
    // `data-nd-error-label` and replaced the whole paragraph to place it,
    // which is the same prefix drawn twice from one prop.
    saySentence(slot, sentence);
    root.dataset.ndInvalid = "true";
    for (const part of ["day", "month", "year"]) {
      if (wrong.includes(part)) boxes[part].setAttribute("aria-invalid", "true");
      else boxes[part].removeAttribute("aria-invalid");
    }
  };

  const settle = () => {
    if (saidByCaller) return;
    saySentence(slot, "");
    root.dataset.ndInvalid = "false";
    for (const part of ["day", "month", "year"]) boxes[part].removeAttribute("aria-invalid");
  };

  /** Judges the three boxes; speaks and returns the wrong parts, or settles
   * and returns null. The first failing sentence only — one per pause. */
  const judge = ({ askRequired = false } = {}) => {
    const words = root.dataset;
    const raw = {
      day: boxes.day.value.trim(),
      month: boxes.month.value.trim(),
      year: boxes.year.value.trim(),
    };
    const empty = ["day", "month", "year"].filter((part) => raw[part] === "");
    if (empty.length === 3) {
      if (askRequired && root.dataset.ndDateInput && boxes.day.required && words.ndRequiredLabel) {
        say(words.ndRequiredLabel, ["day", "month", "year"]);
        return ["day", "month", "year"];
      }
      settle();
      return null;
    }
    if (empty.length > 0) {
      if (words.ndMissingLabel) {
        say(words.ndMissingLabel.replace("{field}", labelOf(empty[0])), empty);
        return empty;
      }
      settle();
      return null;
    }
    const numbers = {
      day: dateNumber(raw.day),
      month: dateNumber(raw.month),
      year: dateNumber(raw.year),
    };
    const wordy = ["day", "month", "year"].filter((part) => numbers[part] === null);
    if (wordy.length > 0) {
      if (words.ndNumberLabel) {
        say(words.ndNumberLabel.replace("{field}", labelOf(wordy[0])), wordy);
        return wordy;
      }
      settle();
      return null;
    }
    if (raw.year.normalize("NFKC").length < 4) {
      if (words.ndYearDigitsLabel) {
        say(words.ndYearDigitsLabel, ["year"]);
        return ["year"];
      }
      settle();
      return null;
    }
    const at = new Date(Date.UTC(numbers.year, numbers.month - 1, numbers.day));
    const real =
      at.getUTCFullYear() === numbers.year &&
      at.getUTCMonth() === numbers.month - 1 &&
      at.getUTCDate() === numbers.day;
    if (!real) {
      const wrong = numbers.month < 1 || numbers.month > 12 ? ["month"] : ["day"];
      if (words.ndInvalidLabel) {
        say(words.ndInvalidLabel, wrong);
        return wrong;
      }
      settle();
      return null;
    }
    const iso = dateIso(numbers.year, numbers.month, numbers.day);
    if (words.ndMin && iso < words.ndMin && words.ndMinLabel) {
      const wrong = ["day", "month", "year"];
      say(words.ndMinLabel.replace("{min}", dateWorded(words.ndMin)), wrong);
      return wrong;
    }
    if (words.ndMax && iso > words.ndMax && words.ndMaxLabel) {
      const wrong = ["day", "month", "year"];
      say(words.ndMaxLabel.replace("{max}", dateWorded(words.ndMax)), wrong);
      return wrong;
    }
    settle();
    return null;
  };

  // ON SUBMIT, AND ON LEAVING A COMPLETE FIELD — never between two boxes of
  // an unfinished answer. Scolding a reader between day and month is the
  // anti-pattern the source pattern exists to document.
  root.closest("form")?.addEventListener("submit", (event) => {
    const wrong = judge({ askRequired: true });
    if (wrong) {
      event.preventDefault();
      inOrder()
        .find((one) => wrong.includes(one.closest("[data-nd-part]").dataset.ndPart))
        ?.focus();
    }
  }, { signal });
  root.addEventListener("focusout", (event) => {
    if (root.contains(event.relatedTarget)) return;
    const filled = [boxes.day, boxes.month, boxes.year].every((one) => one.value.trim() !== "");
    if (filled) judge();
  }, { signal });

  // --- The typing aids, on leaving a box only ------------------------------

  const padded = root.dataset.ndPad === "true";
  const expands = root.dataset.ndExpand === "true";
  for (const part of ["day", "month"]) {
    boxes[part].addEventListener("blur", () => {
      const value = boxes[part].value.trim().normalize("NFKC");
      if (padded && /^[0-9]$/.test(value)) boxes[part].value = value.padStart(2, "0");
    }, { signal });
  }
  boxes.year.addEventListener("blur", () => {
    const value = boxes.year.value.trim().normalize("NFKC");
    if (!expands || !/^[0-9]{2}$/.test(value)) return;
    const two = Number.parseInt(value, 10);
    const choices = [1900 + two, 2000 + two];
    const minYear = root.dataset.ndMin ? Number.parseInt(root.dataset.ndMin, 10) : null;
    const maxYear = root.dataset.ndMax ? Number.parseInt(root.dataset.ndMax, 10) : null;
    const allowed = choices.filter(
      (year) => (minYear === null || year >= minYear) && (maxYear === null || year <= maxYear),
    );
    const now = new Date().getFullYear();
    const chosen =
      allowed.length === 1 ? allowed[0] : 2000 + two <= now + 10 ? 2000 + two : 1900 + two;
    boxes.year.value = String(chosen);
  }, { signal });

  // --- Stepping, and the paste that fills all three ------------------------

  const write = (part, number) => {
    const wide = part === "year" ? 4 : padded ? 2 : 0;
    boxes[part].value = wide ? String(number).padStart(wide, "0") : String(number);
    boxes[part].dispatchEvent(new Event("input", { bubbles: true }));
    boxes[part].dispatchEvent(new Event("change", { bubbles: true }));
  };

  const step = (part, sign) => {
    const bounds = { day: [1, 31], month: [1, 12] }[part];
    const current = dateNumber(boxes[part].value);
    if (bounds) {
      const [low, high] = bounds;
      const base = current ?? (sign > 0 ? low - 1 : low + 1);
      let next = base + sign;
      if (next > high) next = low;
      if (next < low) next = high;
      write(part, next);
    } else {
      const base = current ?? new Date().getFullYear();
      write(part, Math.max(0, base + sign));
    }
  };

  // A whole date arriving at once — a paste, a pick, a grid day — is echoed
  // canonically: two digits for day and month whatever the typing aids say,
  // because it was never typed.
  const fillAll = (year, month, day) => {
    boxes.year.value = String(year).padStart(4, "0");
    boxes.month.value = String(month).padStart(2, "0");
    boxes.day.value = String(day).padStart(2, "0");
    for (const part of ["day", "month", "year"]) {
      boxes[part].dispatchEvent(new Event("input", { bubbles: true }));
      boxes[part].dispatchEvent(new Event("change", { bubbles: true }));
    }
    judge();
  };

  for (const part of ["day", "month", "year"]) {
    boxes[part].addEventListener("keydown", (event) => {
      if (event.altKey && event.key === "ArrowDown") {
        event.preventDefault();
        openPanel?.();
        return;
      }
      if (event.altKey || event.ctrlKey || event.metaKey) return;
      if (event.key === "ArrowUp") {
        event.preventDefault();
        step(part, 1);
      } else if (event.key === "ArrowDown") {
        event.preventDefault();
        step(part, -1);
      }
    }, { signal });
    boxes[part].addEventListener("paste", (event) => {
      const text = (event.clipboardData?.getData("text") ?? "").trim();
      let year;
      let month;
      let day;
      let match = text.match(/^([0-9]{4})[-/. ]([0-9]{1,2})[-/. ]([0-9]{1,2})$/);
      if (match) {
        [, year, month, day] = match.map(Number);
      } else {
        match = text.match(/^([0-9]{1,2})[-/. ]([0-9]{1,2})[-/. ]([0-9]{4})$/);
        if (!match) return;
        const first = Number(match[1]);
        const second = Number(match[2]);
        year = Number(match[3]);
        if (root.dataset.ndResolved === "mdy") {
          month = first;
          day = second;
        } else {
          day = first;
          month = second;
        }
      }
      event.preventDefault();
      fillAll(year, month, day);
      inOrder().at(-1)?.focus();
    }, { signal });
  }

  // --- The suggestions panel -----------------------------------------------

  const source = root.querySelector("[data-nd-date-input-suggest]");
  const picks = source ? [...source.querySelectorAll("time[datetime]")] : [];
  const wantsCalendar = root.dataset.ndCalendar === "true";
  const suggestWord = root.dataset.ndSuggestLabel;
  let openPanel = null;
  if ((picks.length > 0 || wantsCalendar) && suggestWord) {
    const id = `nd-date-input-${root.dataset.ndDateInput}`;
    const button = document.createElement("button");
    button.type = "button";
    button.className = "nd-date-input-suggest-button";
    button.setAttribute("aria-haspopup", "listbox");
    button.setAttribute("aria-expanded", "false");
    button.setAttribute("aria-label", suggestWord);
    const glyph = root.querySelector("template[data-nd-date-input-glyph]");
    if (glyph) button.append(glyph.content.cloneNode(true));
    row.append(button);

    const panel = document.createElement("div");
    panel.className = "nd-panel nd-date-input-panel";
    panel.hidden = true;
    let opener = null;

    const close = () => {
      if (panel.hidden) return;
      const inside = panel.contains(document.activeElement);
      panel.hidden = true;
      button.setAttribute("aria-expanded", "false");
      releaseAnchor(panel);
      releaseOpenPanel(close);
      if (inside) (opener ?? button).focus();
    };

    const take = (iso) => {
      const [year, month, day] = iso.split("-").map(Number);
      fillAll(year, month, day);
      close();
    };

    // The quick picks, cloned from the inert children the caller served.
    let list = null;
    let active = -1;
    if (picks.length > 0) {
      list = document.createElement("ul");
      list.className = "nd-date-input-list";
      list.id = `${id}-listbox`;
      list.setAttribute("role", "listbox");
      list.setAttribute("aria-label", suggestWord);
      list.tabIndex = -1;
      picks.forEach((pick, index) => {
        const rowEl = document.createElement("li");
        rowEl.className = "nd-date-input-option";
        rowEl.id = `${id}-option-${index}`;
        rowEl.setAttribute("role", "option");
        rowEl.dataset.ndValue = pick.getAttribute("datetime");
        rowEl.textContent = pick.textContent;
        rowEl.addEventListener("click", () => take(rowEl.dataset.ndValue), { signal });
        list.append(rowEl);
      });
      button.setAttribute("aria-controls", list.id);
      const highlight = (index) => {
        active = (index + picks.length) % picks.length;
        for (const [at, rowEl] of [...list.children].entries()) {
          rowEl.toggleAttribute("data-nd-active", at === active);
        }
        list.setAttribute("aria-activedescendant", `${id}-option-${active}`);
      };
      list.addEventListener("keydown", (event) => {
        if (event.key === "ArrowDown") {
          event.preventDefault();
          highlight(active + 1);
        } else if (event.key === "ArrowUp") {
          event.preventDefault();
          highlight(active - 1);
        } else if (event.key === "Home") {
          event.preventDefault();
          highlight(0);
        } else if (event.key === "End") {
          event.preventDefault();
          highlight(picks.length - 1);
        } else if (event.key === "Enter") {
          event.preventDefault();
          take(list.children[active]?.dataset.ndValue);
        } else if (event.key === "Escape") {
          event.preventDefault();
          close();
        }
      }, { signal });
      list.reset = () => highlight(0);
      panel.append(list);
    }

    // The month grid, from the locale's own formatter — the one vocabulary
    // that does not arrive as a prop, because month names are the locale's.
    let grid = null;
    let cursor = null;
    if (wantsCalendar) {
      grid = document.createElement("div");
      const head = document.createElement("div");
      head.className = "nd-date-input-grid-head";
      const back = document.createElement("button");
      back.type = "button";
      back.className = "nd-date-input-turn";
      back.textContent = "‹";
      back.setAttribute("aria-label", root.dataset.ndPreviousLabel ?? "");
      const monthName = document.createElement("span");
      monthName.className = "nd-date-input-month";
      // Not `-month`: the month BOX already owns that id.
      monthName.id = `${id}-month-name`;
      const ahead = document.createElement("button");
      ahead.type = "button";
      ahead.className = "nd-date-input-turn";
      ahead.textContent = "›";
      ahead.setAttribute("aria-label", root.dataset.ndNextLabel ?? "");
      head.append(back, monthName, ahead);
      const days = document.createElement("div");
      days.className = "nd-date-input-grid";
      days.setAttribute("role", "grid");
      days.setAttribute("aria-labelledby", monthName.id);
      grid.append(head, days);
      panel.append(grid);

      let firstDay = 1;
      try {
        // A CONVENTION, so the reader's own — see `readerLocales`. The names
        // in the grid are words and stay the page's.
        const locale = new Intl.Locale(readerLocales()[0] ?? "en");
        const info = locale.getWeekInfo ? locale.getWeekInfo() : locale.weekInfo;
        if (info && info.firstDay) firstDay = info.firstDay % 7;
      } catch {
        firstDay = 1;
      }

      const within = (iso) =>
        (!root.dataset.ndMin || iso >= root.dataset.ndMin) &&
        (!root.dataset.ndMax || iso <= root.dataset.ndMax);

      const build = (focusDay) => {
        const year = cursor.getUTCFullYear();
        const month = cursor.getUTCMonth();
        monthName.textContent = new Intl.DateTimeFormat(dateWords(), {
          month: "long",
          year: "numeric",
          timeZone: "UTC",
        }).format(cursor);
        days.replaceChildren();
        const header = document.createElement("div");
        header.setAttribute("role", "row");
        header.style.display = "contents";
        const naming = new Intl.DateTimeFormat(dateWords(), {
          weekday: "narrow",
          timeZone: "UTC",
        });
        for (let at = 0; at < 7; at += 1) {
          const cell = document.createElement("span");
          cell.className = "nd-date-input-weekday";
          cell.setAttribute("role", "columnheader");
          // 2023-01-01 was a Sunday; walk from the week's first day.
          cell.textContent = naming.format(new Date(Date.UTC(2023, 0, 1 + firstDay + at)));
          header.append(cell);
        }
        days.append(header);
        const startsOn = new Date(Date.UTC(year, month, 1)).getUTCDay();
        const lead = (startsOn - firstDay + 7) % 7;
        const today = new Date();
        const todayIso = dateIso(today.getFullYear(), today.getMonth() + 1, today.getDate());
        let rowEl = null;
        for (let at = 0; at < 42; at += 1) {
          if (at % 7 === 0) {
            rowEl = document.createElement("div");
            rowEl.setAttribute("role", "row");
            rowEl.style.display = "contents";
            days.append(rowEl);
          }
          const date = new Date(Date.UTC(year, month, at - lead + 1));
          const iso = dateIso(date.getUTCFullYear(), date.getUTCMonth() + 1, date.getUTCDate());
          const day = document.createElement("button");
          day.type = "button";
          day.className = "nd-date-input-day";
          day.setAttribute("role", "gridcell");
          day.dataset.ndValue = iso;
          day.textContent = String(date.getUTCDate());
          if (date.getUTCMonth() !== month) day.dataset.ndOutside = "true";
          if (iso === todayIso) day.setAttribute("aria-current", "date");
          if (!within(iso)) day.setAttribute("aria-disabled", "true");
          day.tabIndex = iso === focusDay ? 0 : -1;
          day.addEventListener("click", () => {
            if (day.getAttribute("aria-disabled") !== "true") take(iso);
          }, { signal });
          rowEl.append(day);
        }
        return days.querySelector('[tabindex="0"]');
      };

      const shift = (byDays, byMonths, byYears) => {
        const focused = days.querySelector('[tabindex="0"]');
        const [year, month, day] = (focused?.dataset.ndValue ?? dateIso(
          cursor.getUTCFullYear(),
          cursor.getUTCMonth() + 1,
          1,
        ))
          .split("-")
          .map(Number);
        const next = new Date(Date.UTC(year + byYears, month - 1 + byMonths, day + byDays));
        cursor = new Date(Date.UTC(next.getUTCFullYear(), next.getUTCMonth(), 1));
        const target = build(
          dateIso(next.getUTCFullYear(), next.getUTCMonth() + 1, next.getUTCDate()),
        );
        target?.focus();
      };

      days.addEventListener("keydown", (event) => {
        const moves = {
          ArrowLeft: [-1, 0, 0],
          ArrowRight: [1, 0, 0],
          ArrowUp: [-7, 0, 0],
          ArrowDown: [7, 0, 0],
          PageUp: [0, event.shiftKey ? 0 : -1, event.shiftKey ? -1 : 0],
          PageDown: [0, event.shiftKey ? 0 : 1, event.shiftKey ? 1 : 0],
        };
        if (event.key in moves) {
          event.preventDefault();
          shift(...moves[event.key]);
          return;
        }
        if (event.key === "Home" || event.key === "End") {
          event.preventDefault();
          const cells = [...days.querySelectorAll(".nd-date-input-day")];
          const at = cells.findIndex((cell) => cell.tabIndex === 0);
          if (at < 0) return;
          const start = at - (at % 7);
          const target = cells[event.key === "Home" ? start : start + 6];
          for (const cell of cells) cell.tabIndex = -1;
          target.tabIndex = 0;
          target.focus();
          return;
        }
        if (event.key === "Enter" || event.key === " ") {
          event.preventDefault();
          const focused = days.querySelector('[tabindex="0"]');
          if (focused && focused.getAttribute("aria-disabled") !== "true") {
            take(focused.dataset.ndValue);
          }
          return;
        }
        if (event.key === "Escape") {
          event.preventDefault();
          close();
        }
      }, { signal });

      grid.reset = () => {
        const year = dateNumber(boxes.year.value);
        const month = dateNumber(boxes.month.value);
        const day = dateNumber(boxes.day.value);
        const today = new Date();
        const seed =
          year && month
            ? new Date(Date.UTC(year, month - 1, day ?? 1))
            : new Date(Date.UTC(today.getFullYear(), today.getMonth(), today.getDate()));
        cursor = new Date(Date.UTC(seed.getUTCFullYear(), seed.getUTCMonth(), 1));
        return build(
          dateIso(seed.getUTCFullYear(), seed.getUTCMonth() + 1, seed.getUTCDate()),
        );
      };
    }

    root.append(panel);
    // The registry of open floating panels is module-level and holds this
    // closure, so it has to be told when the field goes.
    signal.addEventListener("abort", () => releaseOpenPanel(close));

    openPanel = () => {
      if (!panel.hidden) return;
      opener = document.activeElement;
      claimOpenPanel(close);
      panel.hidden = false;
      button.setAttribute("aria-expanded", "true");
      keepAnchored(panel, row, {}, close);
      if (list) {
        list.reset();
        list.focus();
      } else if (grid) {
        grid.reset()?.focus();
      } else if (grid === null && list === null) {
        button.focus();
      }
      if (list && grid) grid.reset();
    };

    button.addEventListener("click", () => {
      if (panel.hidden) openPanel();
      else close();
    }, { signal });
    panel.addEventListener("focusout", (event) => {
      if (panel.contains(event.relatedTarget) || event.relatedTarget === button) return;
      // Focus flickers to null while the grid rebuilds under it — a month
      // turned is the focused button replaced. Judge where focus LANDS, one
      // frame later, not where it left from.
      requestAnimationFrame(() => {
        if (!panel.contains(document.activeElement) && document.activeElement !== button) {
          close();
        }
      });
    }, { signal });
  }
}

/**
 * Opens whatever the address bar is pointing INTO.
 *
 * A page's sections fold now, so a jump link — from the index at the top, from
 * the site search, from a link somebody sent — can land on a heading whose
 * content is shut. Following a link and arriving at a closed box is arriving
 * nowhere, and the reader has no way to know they are one press away from the
 * thing they asked for.
 *
 * Chrome opens a `<details>` for find-in-page and for a fragment on its own;
 * Firefox and Safari do not, and this site is not one browser's.
 */
function revealTarget() {
  const id = decodeURIComponent(location.hash.slice(1));
  if (!id) return;
  const node = document.getElementById(id);
  if (!node) return;
  for (let up = node; up; up = up.parentElement) {
    if (up.tagName === "DETAILS") up.open = true;
  }
  // Re-run the jump: the browser scrolled before anything was opened, so the
  // target has moved by however tall the folds above it turned out to be.
  node.scrollIntoView();
}

// --- Tabs -------------------------------------------------------------------

// WHAT THE MARKUP CLAIMS AND WHAT THIS ADDS.
//
// `tabs.toml` argues it in full. With no script every panel is on the page and
// each tab is a link down to the one it names, so nothing claims a role that
// nothing can honour — `Tree`'s rule, applied to a second component. This is
// what earns the roles: it can hide a panel and show another one.
//
// The arrow walk is NOT here. `stops()` and `roving()` above serve a toolbar, a
// radio group and now a tab list, because the walk is the same walk and a third
// copy of Home, End and an orientation is a third place to get them wrong.

/** Every enhanced strip's teardown, so a subtree can be released. */
const tabStrips = new WeakMap();

/**
 * Takes the roles and the walk back off every strip under `root`.
 *
 * A BEHAVIOUR THAT CANNOT BE RELEASED IS A DEFECT FOR EVERY CONSUMER WITH A
 * CLIENT-SIDE ROUTER. The roles go back with the listeners, because a strip
 * still claiming `role="tablist"` with nothing answering its arrow keys is the
 * exact defect this file refuses everywhere else: a promise the markup makes
 * and nothing keeps.
 */
function releaseTabs(root = document) {
  for (const strip of root.querySelectorAll("[data-nd-tabs]")) {
    if (strip.dataset.ndEnhanced !== "tabs") continue;
    tabStrips.get(strip)?.abort();
    tabStrips.delete(strip);
    strip.removeAttribute("role");
    for (const tab of strip.querySelectorAll(".nd-tab")) {
      tab.removeAttribute("role");
      document.getElementById(tab.getAttribute("aria-controls"))?.removeAttribute("role");
    }
    delete strip.dataset.ndEnhanced;
  }
}

function enhanceTabs(root = document) {
  for (const strip of root.querySelectorAll("[data-nd-tabs]")) {
    if (strip.dataset.ndEnhanced) continue;
    const tabs = [...strip.querySelectorAll(".nd-tab")];
    if (tabs.length === 0) continue;
    strip.dataset.ndEnhanced = "tabs";
    const binding = new AbortController();
    tabStrips.set(strip, binding);
    const signal = binding.signal;

    // THE PANELS ARE SIBLINGS OF THE STRIP, NEVER CHILDREN: a tab list may hold
    // tabs and nothing else. Each is found by the id its own tab already points
    // at, so nothing here needs to know where a caller put them.
    const panels = tabs.map((tab) => document.getElementById(tab.getAttribute("aria-controls")));

    const show = (at) => {
      tabs.forEach((tab, index) => {
        const on = index === at;
        tab.setAttribute("aria-selected", String(on));
        tab.dataset.ndSelected = String(on);
        tab.tabIndex = on ? 0 : -1;
        const panel = panels[index];
        if (!panel) return;
        panel.hidden = !on;
        // A panel whose content has no stop of its own is one a keyboard could
        // not reach at all, so the panel itself is the stop.
        panel.tabIndex = 0;
      });
    };

    strip.setAttribute("role", "tablist");
    tabs.forEach((tab, index) => {
      tab.setAttribute("role", "tab");
      panels[index]?.setAttribute("role", "tabpanel");
      const choose = (event) => {
        // The `href` is what a page with no script uses. With one, jumping is
        // wrong: the panel arrives exactly where it already was.
        event.preventDefault();
        if (tab.getAttribute("aria-disabled") === "true") return;
        show(index);
        roving(strip);
      };
      tab.addEventListener("click", choose, { signal });
      tab.addEventListener("keydown", (event) => {
        if (event.key !== " " && event.key !== "Enter") return;
        choose(event);
      }, { signal });
    });

    const chosen = tabs.findIndex((tab) => tab.dataset.ndSelected === "true");
    show(chosen === -1 ? 0 : chosen);
  }
}

addEventListener("hashchange", revealTarget);

// --- The one list ------------------------------------------------------------

/**
 * Gives every behaviour in the system to everything under `root`.
 *
 * THERE IS ONE LIST OF ENHANCERS AND THIS IS IT. It used to be a bare block of
 * twenty-one calls at the foot of this file, which was correct while the page
 * was enhanced exactly once. Anything that renders part of the page a second
 * time needs the same twenty-one calls over a subtree, and a second block
 * spelling them out is a copy that stops agreeing with the first the day a
 * behaviour is added to one of them.
 *
 * `enhanceModels` IS IN IT, and it is the one that had to be argued. It is this
 * site's own furniture with no component behind it, so it can match nothing
 * inside a rendered example and costs one selector query that finds nothing.
 * That is cheaper than the alternative, which is a second call site beside this
 * one that somebody has to remember to keep in step. The rule this file keeps
 * everywhere is that a list is written once.
 *
 * THE ORDER IS NOT ALPHABETICAL AND IT IS NOT ARBITRARY. The controls that own
 * a floating panel come before the surfaces that may contain one, and
 * `enhanceMessageBars` has to run after the three components that carry
 * `data-nd-dismissible` for the reason its own file records.
 */
export function enhanceAll(root = document) {
  enhanceSelects(root);
  enhanceSearches(root);
  enhanceTagPickers(root);
  enhanceTrees(root);
  collapseBlocks(root);
  enhanceDateInputs(root);
  enhancePickers(root);
  enhanceInputs(root);
  enhanceCarousels(root);
  enhanceErrorSummaries(root);
  enhanceTables(root);
  enhanceInfos(root);
  enhanceMenus(root);
  enhanceMessageBars(root);
  enhanceDialogs(root);
  enhancePopovers(root);
  enhanceTooltips(root);
  enhanceToasts(root);
  enhanceToastReplays(root);
  enhanceTabs(root);
  enhanceLineages(root);
  enhanceModels(root);
  enhanceIconGallery(root);
}

// --- The icon gallery -------------------------------------------------------

/**
 * Narrows the gallery at `/icons/` to the drawings whose name matches.
 *
 * THIS SITE'S FURNITURE, NOT A COMPONENT. One page in the system shows every
 * icon there is, and one page is not a component — what is a component is the
 * field doing the asking, which is an `Input` like any other.
 *
 * IT HIDES, IT DOES NOT REMOVE. The rule `Table`'s filter already keeps, for
 * the same two reasons: the count in the prose above stays true, and a reader
 * whose script never ran gets the whole set rather than an empty box. Server
 * rendering the grid is what makes the second one free.
 *
 * The match is a plain substring of the icon's own name, folded for case. Not a
 * fuzzy match and not a search over tags: a name is what the gallery shows
 * beside each drawing, so it is the only thing a reader can be typing.
 */
function enhanceIconGallery(root = document) {
  for (const gallery of root.querySelectorAll("[data-nd-icon-gallery]")) {
    if (galleries.has(gallery)) continue;
    const field = gallery.querySelector("input");
    const empty = gallery.querySelector(".nd-icon-empty");
    const cells = [...gallery.querySelectorAll(".nd-icon-cell")];
    if (!field || !cells.length) continue;
    galleries.add(gallery);

    const narrow = () => {
      const query = field.value.trim().toLowerCase();
      let shown = 0;
      for (const cell of cells) {
        const name = cell.getAttribute("data-nd-icon-name") || "";
        const matches = !query || name.includes(query);
        // `hidden` rather than a class: it takes the cell out of the
        // accessibility tree as well as off the screen, which a reader walking
        // the list needs and `display: none` in a stylesheet also gives — but
        // only one of the two survives a stylesheet that failed to load.
        cell.hidden = !matches;
        if (matches) shown += 1;
      }
      if (empty) empty.hidden = shown > 0;
    };

    field.addEventListener("input", narrow);
    // The clear control writes the value and fires `input`, so it arrives here
    // through the same path rather than through a second listener.
    narrow();
  }
}

/** The galleries already wired, so a re-enhanced subtree is not wired twice. */
const galleries = new WeakSet();

// --- Running a timed message again ------------------------------------------
//
// THIS IS APPARATUS, AND IT SHIPS NOWHERE. `/toast/` shows a message that takes
// itself away after six seconds, and for every reader who arrives more than six
// seconds later that example was an empty box. The countdown beside it is the
// COMPONENT's, because a message with a deadline owes its reader the deadline
// wherever it is used; raising the same message a second time is the job of the
// page that raised it, so it lives here with the viewport rig and the
// implementation switch. `AGENTS.md` draws that line and
// `spec/components/toast.toml` writes down which half went which way.
//
// It borrows the component's own two verbs rather than reaching inside it: a
// release and an enhance is what re-arms the wait, re-reveals the controls and
// puts the live region back through `announce`, which is exactly what a message
// arriving a second time should do.

/** Every replay control, so a subtree can be released. */
const replays = new WeakMap();

function enhanceToastReplays(root = document) {
  for (const control of root.querySelectorAll('[data-nd-action="toast-replay"]')) {
    if (control.dataset.ndEnhanced) continue;
    control.dataset.ndEnhanced = "toast-replay";
    const binding = new AbortController();
    replays.set(control, binding);
    control.addEventListener("click", () => {
      // The stage this control is in, never the page: two of these on one page
      // would otherwise each raise the other's message.
      const stage = control.closest(".nd-example-toasts") ?? control.parentElement;
      if (!stage) return;
      releaseToasts(stage);
      for (const toast of stage.querySelectorAll("[data-nd-toast]")) {
        toast.removeAttribute("data-nd-dismissed");
        toast.removeAttribute("data-nd-paused");
      }
      enhanceToasts(stage);
    }, { signal: binding.signal });
  }
}

function releaseToastReplays(root = document) {
  for (const control of root.querySelectorAll('[data-nd-action="toast-replay"]')) {
    if (control.dataset.ndEnhanced !== "toast-replay") continue;
    replays.get(control)?.abort();
    replays.delete(control);
    delete control.dataset.ndEnhanced;
  }
}

/**
 * Takes every one of them back off again.
 *
 * A BEHAVIOUR THAT CAN BE APPLIED AND NOT TAKEN BACK OFF IS A DEFECT, and it is
 * a defect for every consumer with a client-side router rather than for one
 * framework. The enhancers register listeners on the DOCUMENT, observe elements
 * for resize and intersection, arm timers, and hold open panels in module-level
 * registries. None of that stops when the markup leaves the page, so a route
 * visited ten times carries ten copies of it, each pinning a detached subtree
 * that can never be collected. One of the observers FETCHES ROWS.
 *
 * IN THE REVERSE ORDER, for the reason a teardown always is: a surface is
 * released before the controls inside it, so nothing is torn down underneath
 * something still holding a reference to it.
 *
 * `releaseAnchorsIn` runs LAST and is the net under the rest. Each release
 * above evicts its own panel, and this catches a panel none of them owns,
 * including one whose control has already left the document.
 */
export function releaseAll(root = document) {
  releaseModels(root);
  releaseLineages(root);
  releaseTabs(root);
  releaseToastReplays(root);
  releaseToasts(root);
  releaseTooltips(root);
  releasePopovers(root);
  releaseDialogs(root);
  releaseMessageBars(root);
  releaseMenus(root);
  releaseInfos(root);
  releaseTables(root);
  releaseErrorSummaries(root);
  releaseCarousels(root);
  releaseInputs(root);
  releasePickers(root);
  releaseDateInputs(root);
  // `collapseBlocks` has no release and needs none. It binds nothing and holds
  // nothing: it reads the declared cap and writes one attribute on the block,
  // which goes when the block does. The gap is named here rather than filled
  // with an empty function, so the two lists still read against each other.
  releaseTrees(root);
  releaseTagPickers(root);
  releaseSearches(root);
  releaseSelects(root);
  releaseAnchorsIn(root);
  // And the theme registry, which is this file's own strong list rather than an
  // enhancer's. It is asked about the document rather than about `root`, so it
  // is written outside the reversed list above.
  forgetScopes();
}

enhanceAll();
/**
 * The example rail's axis, kept true at both widths.
 *
 * The strip is a rail down the frame's trailing edge on a wide screen and a row
 * under the stage on a narrow one, and `aria-orientation` decides which arrow
 * keys walk it. A value baked into the markup would be wrong at one of the two
 * widths, and a toolbar announcing an axis a reader cannot walk is exactly what
 * this system refuses everywhere else.
 *
 * The markup ships `horizontal`, which is the ARIA default and is true below
 * the breakpoint — so a reader with no script gets the narrow layout and the
 * attribute that matches it.
 *
 * THE BREAKPOINT IS READ, NOT WRITTEN. It comes off the token layer, so this
 * cannot drift from the `@media` rule in the stylesheet that stands the rail
 * up. Writing `30rem` here would be a second copy of one number.
 *
 * THE QUERY IS REGISTERED ONCE AND THE PASS IS RE-RUNNABLE, which is the split
 * a second render needs: a rail that arrived with `navigate.js`'s new `<main>`
 * ships the markup's `horizontal` and would announce a horizontal walk on a
 * wide screen for ever, because the media query it answers to had already
 * fired. Re-registering the listener instead would add one per navigation.
 */
const railWide = (() => {
  const step = getComputedStyle(document.documentElement)
    .getPropertyValue("--noctua-breakpoint-sm")
    .trim();
  return step ? matchMedia(`(min-width: ${step})`) : null;
})();

function applyRailAxis(root = document) {
  if (!railWide) return;
  for (const rail of root.querySelectorAll(".nd-frame-bar")) {
    rail.setAttribute("aria-orientation", railWide.matches ? "vertical" : "horizontal");
  }
}

railWide?.addEventListener("change", () => applyRailAxis());
applyRailAxis();
revealTarget();

// --- The implementation switch -------------------------------------------
//
// In its own module, because it runs only for a reader who asks for it and it
// owns four things this file should not: a stored preference, a lazy import, an
// observer and a failure mode. See `implementation.js`.
//
// The behaviours are passed in rather than imported there. They are 208 KB this
// page has already loaded, and importing them from the React bundle would make
// esbuild inline a second copy of every one of them.
import { install as installExamples, observe as observeFrames, forget as forgetFrames }
  from "./implementation.js";

installExamples({ enhance: enhanceAll, release: releaseAll });

// --- Rendering the page a second time --------------------------------------

/**
 * Everything this file does to markup that has just arrived.
 *
 * IN THE ORDER THE PAGE ITSELF IS BUILT, and that is not a preference. The
 * sequence at the foot of a first load is: scopes registered, `refresh`,
 * `refreshRoving`, `refreshChoices`, `enhanceAll`, the rail's axis,
 * `revealTarget`, then the example frames observed. A second ordering here
 * would be a second thing that stops agreeing with the first, and each pair in
 * it has a reason — `refresh` resolves the scopes BEFORE anything reads one,
 * and `enhanceAll` runs after the controls have been marked so a roving walk is
 * placed over the answer rather than over the markup's guess.
 *
 * `forgetScopes` FIRST, and it has to be called with the old subtree already
 * detached: it evicts by `isConnected`, so calling it while the old `<main>` is
 * still in the document evicts nothing and every stage on the page a reader has
 * left stays in the list for the rest of the visit.
 *
 * `ensureStylesheet` is deliberately absent. The palette in force was linked on
 * the first load and a navigation cannot change it; the one page that needs the
 * other thirty-eight asks for them itself.
 */
export function adopt(root) {
  forgetScopes();
  registerScopes(root);
  resolveScopes();
  refresh();
  refreshRoving();
  refreshChoices();
  enhanceAll(root);
  applyRailAxis(root);
  revealTarget();
  observeFrames(root);
}

/**
 * Everything this file undoes before markup leaves.
 *
 * Called while the subtree is still in the document, because that is the only
 * moment a behaviour can be taken off the elements holding it — which is the
 * rule `releaseAll` states and `undraw` in `implementation.js` obeys for React.
 */
export function abandon(root) {
  forgetFrames(root);
  releaseAll(root);
}

// --- The router ------------------------------------------------------------
//
// Its own module for the same reason the implementation switch is: it owns a
// history entry, a fetch, a parser and a failure mode, and none of those is
// this file's business. What it needs from here is the two functions above and
// nothing else — it never reaches into this file's registries.
import { install as installNavigation } from "./navigate.js";

installNavigation({ adopt, abandon });
