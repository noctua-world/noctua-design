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
// 2. It does not read a colour out of the DOM. `getComputedStyle(…).color`
//    returns whatever the engine felt like serialising, in whatever space, and
//    a test that compares two of those strings passes on a colour it never
//    checked. Paint into a 1×1 canvas and read the pixel instead.
// 3. It does not trap focus or set `inert`. The drawer is not modal — widening
//    the window past `lg` undoes the whole open state in CSS, with no listener
//    to miss and no state to unwind.

import {
  modeValues,
  accentValues,
  saturationValues,
  paletteValues,
  fontValues,
  glassValues,
  radiusValues,
  blackoutValues,
  patternValues,
  glowValues,
  grainValues,
  resolveScope,
  scopeAttributes,
  stylesheetHref,
  scopeAttribute,
} from "./theme.js";

const root = document.documentElement;
const base = root.getAttribute("data-noctua-base") || "";

/** Every setting, keyed by the name the markup uses. Generated, not written. */
const model = {
  mode: modeValues,
  accent: accentValues,
  saturation: saturationValues,
  palette: paletteValues,
  font: fontValues,
  glass: glassValues,
  radius: radiusValues,
  blackout: blackoutValues,
  pattern: patternValues,
  glow: glowValues,
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
  // A `Choice` the theme model knows nothing about is not this handler's
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
  if (name === "palette") ensureStylesheet(value);
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

/** Marks the buttons that match what is currently in force. */
function refresh() {
  for (const group of document.querySelectorAll("[data-nd-setting]")) {
    const name = group.dataset.ndSetting;
    if (!model[name]) continue;
    const current = read(name);
    for (const button of group.querySelectorAll("[data-nd-value]")) {
      button.setAttribute(
        "aria-checked",
        String(button.dataset.ndValue === current),
      );
    }
  }
  resolveScopes();
  refreshRoving();
}

document.addEventListener("click", (event) => {
  const button = event.target.closest?.("[data-nd-value]");
  if (button) {
    const group = button.closest("[data-nd-setting]");
    if (group) {
      apply(group.dataset.ndSetting, button.dataset.ndValue);
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

  if (event.target.closest?.("[data-nd-reset]")) {
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

  // Close the panel on any click outside it. `<details>` has no such behaviour
  // of its own, and a disclosure that stays open behind the page you just
  // clicked into reads as stuck.
  const panel = document.querySelector("[data-nd-settings][open]");
  if (panel && !panel.contains(event.target)) panel.removeAttribute("open");
});

// The accent swatches each need their own palette's stylesheet to show their
// own colour. Fetched when the panel is first opened, and never on a page view
// that does not open it.
document.querySelector("[data-nd-settings]")?.addEventListener("toggle", (event) => {
  if (!event.target.open) return;
  const saturation = read("saturation");
  for (const swatch of document.querySelectorAll("[data-nd-swatch]")) {
    const palette = `${swatch.dataset.ndSwatch}-${saturation}`;
    if (!paletteValues.values.includes(palette)) continue;
    swatch.setAttribute("data-palette", palette);
    ensureStylesheet(palette);
  }
});

// The colours page shows every palette at once, so it is the one page that
// legitimately links all thirty-nine stylesheets — about 650 KB gzipped, which
// is why no other page does.
//
// Each cell is a `data-palette` scope, so `dist/scope.css` re-resolves the whole
// semantic contract inside it and every chip shows THAT palette's colours. But a
// scope can only resolve what has been loaded: without these links the grid
// renders thirty-nine cells that all paint the reader's own palette, which is
// exactly what it did until this was written. The markup was right, the CSS was
// right, and nothing connected them.
if (document.querySelector("[data-nd-all-palettes]")) {
  for (const palette of paletteValues.values) ensureStylesheet(palette);
}

// --- The drawer ------------------------------------------------------------

function setDrawer(open) {
  if (open) {
    root.setAttribute("data-nd-nav", "open");
  } else {
    root.removeAttribute("data-nd-nav");
  }
  const button = document.querySelector("[data-nd-menu]");
  if (button) {
    button.setAttribute("aria-expanded", String(open));
    // A button whose label never changes tells a screen-reader user the menu is
    // closed while they are standing in it.
    const label = open ? button.dataset.ndClose : button.dataset.ndOpen;
    if (label) {
      button.setAttribute("aria-label", label);
      button.setAttribute("title", label);
    }
  }
}

document.querySelector("[data-nd-menu]")?.addEventListener("click", () => {
  setDrawer(root.getAttribute("data-nd-nav") !== "open");
});

document
  .querySelector("[data-nd-scrim]")
  ?.addEventListener("click", () => setDrawer(false));

// Escape closes whatever is open. Not a focus trap — just the one key a reader
// expects to back out of something.
document.addEventListener("keydown", (event) => {
  if (event.key !== "Escape") return;

  // Escape in a search field clears the field. Closing the navigation the
  // reader is searching, because they used the key every browser has trained
  // them to use, is the opposite of helpful.
  if (event.target?.matches?.("[data-nd-search]")) {
    if (event.target.value !== "") {
      event.target.value = "";
      event.target.dispatchEvent(new Event("input"));
      event.stopPropagation();
      return;
    }
  }

  setDrawer(false);
  document.querySelector("[data-nd-settings][open]")?.removeAttribute("open");
});

// --- Keyboard model for the radiogroups -------------------------------------

// Twelve groups and forty-two buttons meant forty-two tab stops. A radiogroup
// is ONE stop: Tab enters it, arrows move within it, Tab leaves. That is what
// every native radio group does and what a reader expects from `role="radio"`.
function roving(group) {
  const options = [...group.querySelectorAll('[role="radio"]')];
  if (options.length === 0) return;
  const checked = options.findIndex((o) => o.getAttribute("aria-checked") === "true");
  const active = checked === -1 ? 0 : checked;
  options.forEach((option, index) => {
    option.tabIndex = index === active ? 0 : -1;
  });
}

function refreshRoving() {
  for (const group of document.querySelectorAll('[role="radiogroup"]')) roving(group);
}

document.addEventListener("keydown", (event) => {
  const option = event.target?.closest?.('[role="radio"]');
  if (!option) return;
  const group = option.closest('[role="radiogroup"]');
  if (!group) return;

  const options = [...group.querySelectorAll('[role="radio"]')];
  const index = options.indexOf(option);
  const step = { ArrowRight: 1, ArrowDown: 1, ArrowLeft: -1, ArrowUp: -1 }[event.key];

  let next = null;
  if (step !== undefined) next = options[(index + step + options.length) % options.length];
  else if (event.key === "Home") next = options[0];
  else if (event.key === "End") next = options[options.length - 1];
  if (!next) return;

  event.preventDefault();
  next.focus();
  next.click();
});

// Following a link inside the drawer closes it, or the destination arrives
// behind a menu the reader has to dismiss.
document.querySelector("[data-nd-search]")?.closest(".nd-nav")
  ?.addEventListener("click", (event) => {
    if (event.target.closest("a")) setDrawer(false);
  });

// --- Search ----------------------------------------------------------------

// In-house and index-based: each link carries its own collated title in
// `data-nd-index`, computed at build time by the same function that sorted the
// list, so the search matches what the reader sees rather than the slug.
const search = document.querySelector("[data-nd-search]");
const found = document.querySelector("[data-nd-found]");

search?.addEventListener("input", () => {
  const needle = search.value.trim().toLowerCase();
  let pages = 0;
  let names = 0;

  for (const link of document.querySelectorAll("[data-nd-index]")) {
    const hit = link.dataset.ndIndex.toLowerCase().includes(needle);
    const isPage = link.classList.contains("nd-nav-link");

    if (isPage) {
      // A page row hides its <li>; an empty query shows every page, because the
      // sidebar with nothing typed in it is the navigation.
      link.parentElement.hidden = !(needle === "" || hit);
      if (needle === "" || hit) pages += 1;
    } else {
      // A name row is hidden until something is typed. Showing thirty-nine
      // palettes and every token under an empty search box would bury the
      // navigation the sidebar exists for.
      link.hidden = needle === "" || !hit;
      if (needle !== "" && hit) names += 1;
    }
  }

  if (found) found.hidden = names === 0;
  const empty = document.querySelector("[data-nd-search-empty]");
  if (empty) empty.hidden = pages + names > 0;

  const status = document.querySelector("[data-nd-search-status]");
  if (status) {
    // One whole sentence with a hole in it, filled from the catalogue — never
    // a count concatenated to a word. Word order and pluralisation differ
    // between languages, and neither half is a sentence a translator can move.
    status.textContent =
      needle === "" ? "" : fillMessage(status.dataset.ndTemplate, { count: pages + names });
  }
});

/** Substitutes `{name}` placeholders. The only sanctioned way to build a message. */
function fillMessage(template, args) {
  if (!template) return "";
  return template.replace(/\{(\w+)\}/g, (whole, name) =>
    Object.hasOwn(args, name) ? String(args[name]) : whole,
  );
}

// --- Themed scopes ---------------------------------------------------------

// A scope's controls change the SAME element the scope stamped, never a wrapper
// or a child: the colour system's forced-mode rule is compound and
// same-element, and splitting the palette from the mode makes it match nothing.
//
// Each scope remembers what it explicitly ASKED for, so the chain can be
// re-resolved from the top whenever anything above it changes. A scope that
// only asked for a mode has to pick up its parent's new palette; one that
// stored its resolved answer would keep the old one forever.
const scopes = [...document.querySelectorAll(`[${scopeAttribute}]`)];
const requests = new WeakMap();
/** What each scope was built with, so a reset has something to return to. */
const initial = new WeakMap();

for (const element of scopes) {
  const request = {};
  for (const name of ["mode", "palette", "font", "glass", "radius"]) {
    const value = element.getAttribute(model[name].attribute);
    if (value != null) request[name] = value;
  }
  // A scope with no `data-theme` is following whatever it inherits, which is
  // exactly what an absent value means everywhere else in this system.
  requests.set(element, request);
  initial.set(element, { ...request });
}

function documentTheme() {
  const theme = {};
  for (const name of ["mode", "accent", "saturation", "palette", "font", "glass", "radius"]) {
    theme[name] = read(name);
  }
  return theme;
}

/** What the theme is at an element, walking every scope above it. */
function inheritedAt(element) {
  const parent = element.parentElement?.closest(`[${scopeAttribute}]`);
  // Recursive rather than one level up: a scope three deep that only asked for
  // a mode has to inherit the palette from wherever above it was last set, and
  // stopping at the first parent would read that parent's *request* rather than
  // what the parent resolved to.
  if (!parent) return documentTheme();
  return resolveScope(requests.get(parent) ?? {}, inheritedAt(parent));
}

/** Re-resolves every scope against the theme that actually reaches it. */
function resolveScopes() {
  for (const element of scopes) {
    const inherited = inheritedAt(element);
    const request = requests.get(element) ?? {};
    const attributes = scopeAttributes(request, inherited);
    if (attributes === null) continue;
    for (const [name, value] of Object.entries(attributes)) {
      element.setAttribute(name, value);
    }
    // An attribute the resolver dropped has to come off the element too, or a
    // scope that stops forcing a mode keeps the last one it forced.
    if (!(model.mode.attribute in attributes)) {
      element.removeAttribute(model.mode.attribute);
    }
    if (request.palette) ensureStylesheet(request.palette);
  }
}

document.addEventListener("click", (event) => {
  if (event.target.closest?.("[data-nd-scope-reset]")) {
    // Without this the only way back from a cycled demo was a reload.
    for (const element of scopes) requests.set(element, initial.get(element) ?? {});
    resolveScopes();
    return;
  }

  const button = event.target.closest?.("[data-nd-scope-cycle]");
  if (!button) return;
  const element = button.closest(`[${scopeAttribute}]`);
  if (!element) return;

  const name = button.dataset.ndScopeCycle;
  const request = requests.get(element) ?? {};

  if (name === "accent") {
    // The accent is not a scope attribute of its own — the palette is. So
    // cycling it means moving along the accent axis while holding the
    // saturation the current palette already has.
    const current = request.palette ?? read("palette");
    const parts = current.split("-");
    const saturation = parts[parts.length - 1];
    const accents = accentValues.values;
    const index = accents.indexOf(parts.slice(0, -1).join("-"));
    const next = `${accents[(index + 1) % accents.length]}-${saturation}`;
    if (paletteValues.values.includes(next)) request.palette = next;
  } else {
    const values = model[name].values;
    const current = request[name] ?? read(name);
    request[name] = values[(values.indexOf(current) + 1) % values.length];
  }

  requests.set(element, request);
  resolveScopes();
});

// --- The React path, fetched on demand -------------------------------------

// The bundle is 190 KB. It is not requested on a page view, and it IS requested
// the first time a reader switches an example to React — which is the whole
// point of a lazy path: the cost lands on the person who asked for it.
//
// `import()` rather than a <script> tag, so the browser deduplicates the fetch,
// the module gets its own scope, and a failure is a rejected promise this file
// can report rather than a silent no-op.
let reactModule = null;

async function loadReact() {
  if (reactModule === null) {
    reactModule = import("./example-react.js");
  }
  return reactModule;
}

/** Reads a scope element's attributes back into the request that built it. */
function requestFrom(element) {
  const request = {};
  for (const name of ["mode", "palette", "font", "glass", "radius"]) {
    const value = element.getAttribute(model[name].attribute);
    if (value != null) request[name] = value;
  }
  return request;
}

async function showReact(frame, button) {
  const stage = frame.querySelector("[data-nd-frame-stage]");
  const original = stage.querySelector(`[${scopeAttribute}]`);
  if (!original) return;

  const label = button.textContent;
  button.textContent = frame.dataset.ndLoading ?? label;
  let module;
  try {
    module = await loadReact();
  } catch (error) {
    // A failed lazy import must not leave a dead button. Put the label back and
    // let the reader stay on the implementation that is already working.
    button.textContent = label;
    console.error("[noctua] the React example failed to load", error);
    return;
  }
  button.textContent = label;

  const request = requestFrom(original);
  const text = {
    heading: original.querySelector(".nd-sample-heading")?.textContent ?? "",
    body: original.querySelector(".nd-prose")?.firstChild?.textContent ?? "",
    link: original.querySelector(".nd-link")?.textContent ?? "",
    action: original.querySelector(".nd-button")?.textContent ?? "",
    quiet: original.querySelector(".nd-button--quiet")?.textContent ?? "",
  };

  let slot = frame.querySelector("[data-nd-react-slot]");
  if (!slot) {
    slot = document.createElement("div");
    slot.dataset.ndReactSlot = "";
    stage.appendChild(slot);
  }
  original.hidden = true;
  slot.hidden = false;
  module.mount(slot, request, text);
}

function showDioxus(frame) {
  const stage = frame.querySelector("[data-nd-frame-stage]");
  const original = stage.querySelector(`[${scopeAttribute}]`);
  const slot = frame.querySelector("[data-nd-react-slot]");
  if (original) original.hidden = false;
  if (slot) {
    slot.hidden = true;
    // Unmounted rather than merely hidden: a live React root behind a hidden
    // div keeps its state and its effects, and the next switch would show a
    // component that had been running the whole time.
    reactModule?.then((module) => module.unmount(slot)).catch(() => {});
  }
}

document.addEventListener("click", (event) => {
  const button = event.target.closest?.('[data-nd-setting="frame-impl"] [data-nd-value]');
  if (!button) return;
  const frame = button.closest("[data-nd-frame]");
  const language = button.dataset.ndValue;

  for (const other of frame.querySelectorAll('[data-nd-setting="frame-impl"] [data-nd-value]')) {
    other.setAttribute("aria-checked", String(other === button));
  }
  for (const source of frame.querySelectorAll("[data-nd-impl-source]")) {
    source.hidden = source.dataset.ndImplSource !== language;
  }

  if (language === "React") {
    void showReact(frame, button);
  } else {
    showDioxus(frame);
  }
});

// --- The example frame -----------------------------------------------------

document.addEventListener("click", (event) => {
  const width = event.target.closest?.('[data-nd-setting="frame-width"] [data-nd-value]');
  if (width) {
    const frame = width.closest("[data-nd-frame]");
    frame.querySelector("[data-nd-frame-stage]").dataset.ndWidth =
      width.dataset.ndValue;
    for (const button of frame.querySelectorAll('[data-nd-setting="frame-width"] [data-nd-value]')) {
      button.setAttribute("aria-checked", String(button === width));
    }
    return;
  }

  const reset = event.target.closest?.("[data-nd-frame-reset]");
  if (reset) {
    const frame = reset.closest("[data-nd-frame]");
    frame.querySelector("[data-nd-frame-stage]").dataset.ndWidth = "full";
    for (const button of frame.querySelectorAll('[data-nd-setting="frame-width"] [data-nd-value]')) {
      button.setAttribute(
        "aria-checked",
        String(button.dataset.ndValue === "full"),
      );
    }
    // A scope inside the frame goes back to the theme it was built with, and
    // the React example goes away with it.
    for (const element of frame.querySelectorAll(`[${scopeAttribute}]`)) {
      requests.set(element, {});
    }
    showDioxus(frame);
    for (const impl of frame.querySelectorAll('[data-nd-setting="frame-impl"] [data-nd-value]')) {
      impl.setAttribute("aria-checked", String(impl.dataset.ndValue === "Dioxus"));
    }
    resolveScopes();
  }
});

// --- Start -----------------------------------------------------------------

// The bootstrap already stamped everything before the first paint. This only
// marks the controls and links the palette the reader chose, so nothing that
// happens here can move the page.
ensureStylesheet(read("palette"));
refresh();
refreshRoving();
