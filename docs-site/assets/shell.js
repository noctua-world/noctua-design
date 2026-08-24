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
import { message, saySentence } from "./messages.js";
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
// The month grid, and the panel it folds into. A `dist/` file because a month
// with no script is nothing at all, unlike the three-box field beside it.
import { dateWords, enhanceDatePickers, readerLocales, releaseDatePickers } from "./date-picker.js";
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
// The five that used to live in THIS file, and the arrow walk they share.
// Moving them out is what made them real for a consumer: a `Select` that
// cannot open its list outside this site is a `Select` that lies, which is the
// rule `NOCTUA.md §9.4` states and `color-picker.js` was the first to follow.
import { enhanceSelects, releaseSelects } from "./select.js";
import { enhanceTagPickers, releaseTagPickers } from "./tag-picker.js";
import { enhanceTrees, releaseTrees } from "./tree.js";
import { enhanceTabs, releaseTabs } from "./tabs.js";
import { enhanceDateInputs, releaseDateInputs } from "./date-input.js";
import { enhanceSearches, releaseSearches } from "./search.js";
import { enhanceChoices, releaseChoices } from "./choice.js";
import { enhanceRoving } from "./roving.js";

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
  enhanceRoving();
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
// WHERE A CHOSEN VALUE GOES IS THIS SITE'S BUSINESS, AND ONLY THAT.
//
// `dist/choice.js` marks the choice — a switch flips, an item in a group
// becomes the chosen one, the tab stop follows — and says so with a `change`
// carrying the group's name and the value now in force. What is left here is
// the routing: a control inside a themed scope changes that scope, and one
// outside changes the page. A component that knew either would be a component
// with this site inside it.
document.addEventListener("change", (event) => {
  const { name, value } = event.detail ?? {};
  if (!name || !model[name]) return;
  route(event.target, name, value);
});

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

/**
 * Fetches the stylesheet each chip's palette needs, when a list opens.
 *
 * A `Swatch` takes its colour from the palette it declares, and a palette is a
 * stylesheet THIS SITE loads on demand — so a chip whose sheet has not arrived
 * paints the reader's own accent instead. Six identical chips is what that
 * looks like, and the theme menu's own accent row shipped thirteen of them once
 * for exactly this reason.
 *
 * It listens rather than being called: `select.js` ships, and a component that
 * knew how this site fetches a palette would be a component with a page inside
 * it. It says when the list opened; what that costs is the page's business.
 */
document.addEventListener("nd-select-open", (event) => {
  for (const option of event.target.querySelectorAll("[data-nd-swatch]")) {
    const palette = option.dataset.ndSwatch;
    if (palette && paletteValues.values.includes(palette)) ensureStylesheet(palette);
  }
});

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
enhanceRoving();
refreshChoices();
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
  enhanceChoices(root);
  enhanceSelects(root);
  enhanceSearches(root);
  enhanceTagPickers(root);
  enhanceTrees(root);
  collapseBlocks(root);
  enhanceDateInputs(root);
  enhanceDatePickers(root);
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
  releaseChoices(root);
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
  releaseDatePickers(root);
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
 * `enhanceRoving`, `refreshChoices`, `enhanceAll`, the rail's axis,
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
  enhanceRoving();
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
