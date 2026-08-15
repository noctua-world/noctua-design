// Everything the seven input controls do once a script is running.
//
// IT SHIPS, for `ColorPicker`'s reason: a component whose consumer cannot use
// it is a component that lies. One module rather than six, because what these
// controls have in common is most of what they do — a sentence in a live
// region, a value clamped into a range, a control revealed only once something
// can answer for it. `segmentWalk` is the sharpest example: a time and a code
// are the same three gestures over a row of boxes, so there is one of it.
//
// WHAT NONE OF THEM DOES:
//
// 1. Builds markup. Every element touched here was rendered by its component,
//    and the one repeated thing — a file row — is cloned from a `<template>`
//    the component shipped. `NOCTUA.md §9.5` puts drawings in one place.
// 2. Claims a role the markup has not earned. `role="spinbutton"` is written
//    here rather than in the contract's markup, because with no script there
//    are no steppers and nothing to spin.
// 3. Rewrites what a reader typed. Out of range, not a number, not a time —
//    each is refused with the caller's own sentence, and the value stays.

import { fill, plural, say } from "./messages.js";

/** How long after the last keystroke a counter speaks. */
const SETTLE = 500;

/** Every attribute read the same way, from the control's root. */
function reader(root) {
  return (name) => root.getAttribute(`data-nd-${name}`) || null;
}

/** A number, or `null` — never `NaN` leaking into arithmetic. */
function number(text) {
  if (text === null || text === undefined || String(text).trim() === "") return null;
  const value = Number(String(text).replace(",", "."));
  return Number.isFinite(value) ? value : null;
}

/** Keeps a number inside its range, where there is one. */
function clamp(value, low, high) {
  let out = value;
  if (low !== null && out < low) out = low;
  if (high !== null && out > high) out = high;
  return out;
}

// --- Input ---------------------------------------------------------------

function enhanceInput(root) {
  const at = reader(root);
  const box = root.querySelector("[data-nd-input-box]");
  if (!box) return;
  const clear = root.querySelector("[data-nd-input-clear]");
  const reveal = root.querySelector("[data-nd-input-reveal]");
  const counter = root.parentElement?.querySelector("[data-nd-input-counter]");

  if (clear) {
    // REVEALED ONLY NOW, because until this line there was nothing to answer
    // for it. Shown and hidden with the content rather than always: a control
    // that is there when the box is empty is a control that does nothing.
    const show = () => {
      clear.hidden = box.value.length === 0 || box.disabled || box.readOnly;
    };
    show();
    box.addEventListener("input", show);
    clear.addEventListener("click", () => {
      box.value = "";
      box.dispatchEvent(new Event("input", { bubbles: true }));
      // Focus goes back to the box, because emptying a field is something a
      // reader does in order to type in it.
      box.focus();
      show();
    });
    box.addEventListener("keydown", (event) => {
      // Nothing at all when there is nothing to clear, rather than swallowing
      // a key another control may want.
      if (event.key !== "Escape" || box.value.length === 0) return;
      event.preventDefault();
      clear.click();
    });
  }

  if (reveal) {
    reveal.hidden = false;
    const conceal = at("conceal-label");
    const shown = at("reveal-label");
    reveal.addEventListener("click", () => {
      const showing = box.type === "text";
      box.type = showing ? "password" : "text";
      root.toggleAttribute("data-nd-revealed", !showing);
      reveal.setAttribute("aria-pressed", String(!showing));
      // A TOGGLE THAT KEEPS ONE NAME announces the wrong state from the first
      // press onward, and no script may invent the other name in the reader's
      // language.
      const name = showing ? shown : conceal;
      if (name) {
        reveal.setAttribute("aria-label", name);
        reveal.setAttribute("title", name);
      }
      box.focus();
    });
  }

  if (counter) countFor(box, counter, at("counter-label"));
}

/**
 * The count of what is left, spoken when the typing pauses.
 *
 * Not on every keystroke: a live region that speaks per character is one a
 * reader turns off, and the number is only worth hearing once they have
 * stopped to look at it.
 */
function countFor(box, region, sentence) {
  const max = number(box.getAttribute("maxlength"));
  let waiting = null;
  const draw = () => {
    region.textContent = fill(plural(sentence, box.value.length), {
      count: box.value.length,
      max: max ?? "",
    });
  };
  // Drawn immediately so the number is right before anybody types, and
  // announced only after a pause.
  region.setAttribute("aria-live", "off");
  draw();
  box.addEventListener("input", () => {
    region.setAttribute("aria-live", "off");
    draw();
    clearTimeout(waiting);
    waiting = setTimeout(() => region.setAttribute("aria-live", "polite"), SETTLE);
  });
}

// --- Textarea ----------------------------------------------------------------

function enhanceTextarea(root) {
  const counter = root.parentElement?.querySelector("[data-nd-textarea-counter]");
  if (counter) countFor(root, counter, root.getAttribute("data-nd-counter-label"));
  if (root.getAttribute("data-nd-autosize") !== "true") return;

  const maxRows = number(root.getAttribute("data-nd-max-rows"));
  const grow = () => {
    // Measured from zero, because a box already at its old height reports that
    // height as its content's. The cap is applied to the STYLE rather than to
    // the measurement, so the box scrolls at the ceiling instead of stopping
    // short of its own text.
    root.style.height = "auto";
    const line = Number.parseFloat(getComputedStyle(root).lineHeight) || 0;
    const ceiling = maxRows && line ? maxRows * line : Infinity;
    const wanted = root.scrollHeight;
    root.style.height = `${Math.min(wanted, ceiling)}px`;
    root.style.overflowY = wanted > ceiling ? "auto" : "hidden";
  };
  grow();
  root.addEventListener("input", grow);
  // A box that was hidden when the page loaded measures as zero, so it is
  // measured again the first time it is shown.
  new ResizeObserver(grow).observe(root);
}

// --- NumberInput -------------------------------------------------------------

function enhanceNumberInput(root) {
  const at = reader(root);
  const box = root.querySelector("[data-nd-number-input-box]");
  if (!box) return;
  // SOMEWHERE FOR A SENTENCE TO GO, ON EVERY NUMBER FIELD. `min_label`,
  // `max_label` and `invalid_label` are this control's own words, so `Field`
  // draws the slot here whether or not the field is required. It used to
  // arrive with `required`, and a bounded field that was not required marked
  // itself wrong and said nothing to anyone.
  const status = root.closest(".nd-field")?.querySelector("[data-nd-field-error]");
  const min = number(at("min"));
  const max = number(at("max"));
  const step = number(at("step")) ?? 1;
  const bigStep = number(at("big-step")) ?? step * 10;
  const grouped = at("group") === "true";
  const prefix = at("prefix");
  const suffix = at("suffix");
  // The decimals `step` implies: a step of 0.01 and a display of 3.1 disagree
  // about what the next press will do.
  const places = (String(step).split(".")[1] ?? "").length;
  // How many decimals are drawn at the moment. `step` sets the fewest, and a
  // value carrying more keeps them: 7.5 in a field that steps by one is 7.5,
  // never 8.
  let digits = places;
  // The number this script last drew, and the text it drew for it. A grouped
  // value cannot be read back out of the box — `1.499,90` is a number in
  // Portuguese and `NaN` to `Number` — so what was written is remembered
  // rather than parsed, and it is trusted only while the box still shows it.
  let held = null;
  let drawnText = null;
  const locale = document.documentElement.lang || undefined;
  // The caller's own error. This script marks and unmarks what a reader typed,
  // and it never takes away a mark that arrived as a prop.
  const given = box.getAttribute("aria-invalid") === "true";

  // IT SAYS IT IS A SPINBUTTON ONLY NOW. With no script there is nothing to
  // spin, and a role claimed without its contract is a promise the markup does
  // not keep.
  box.setAttribute("role", "spinbutton");
  if (min !== null) box.setAttribute("aria-valuemin", String(min));
  if (max !== null) box.setAttribute("aria-valuemax", String(max));

  const current = () => number(box.value.replace(/[^\d.,\-]/g, ""));

  /** What the box holds: what a reader typed, or what this script drew. */
  const reading = () => (held !== null && box.value === drawnText ? held : current());

  /** The decimals a written value carries, and never fewer than `step` asks. */
  const carried = (text) => {
    const tail = /[.,](\d+)\s*$/.exec(text);
    return Math.max(places, tail ? tail[1].length : 0);
  };

  // WHAT IT WROTE, IT CAN TAKE BACK, and nothing else. `ErrorSummary` marks
  // its own sentences the same way, so a sentence the caller wrote survives
  // both of them.
  const announce = (sentence) => {
    if (!status) return;
    if (sentence) {
      status.textContent = sentence;
      status.setAttribute("data-nd-said", "");
    } else if (status.hasAttribute("data-nd-said")) {
      status.textContent = "";
      status.removeAttribute("data-nd-said");
    }
  };

  const mark = (wrong) => box.setAttribute("aria-invalid", wrong || given ? "true" : "false");

  /** What the box shows for a value: grouped while nobody is typing into it. */
  const drawn = (value) =>
    grouped && document.activeElement !== box
      ? new Intl.NumberFormat(locale, {
          minimumFractionDigits: digits,
          maximumFractionDigits: digits,
        }).format(value)
      : value.toFixed(digits);

  // A SPINBUTTON CARRIES ITS VALUE AT REST. `aria-valuenow` was written inside
  // the move, so every number field on a page was a spinbutton with no current
  // value until something touched it. `aria-valuetext` says what a reader can
  // see — the separators, the currency, the unit — where that is more than the
  // digits.
  const show = (value) => {
    const text = drawn(value);
    if (text !== box.value) box.value = text;
    held = value;
    drawnText = text;
    const plain = value.toFixed(digits);
    box.setAttribute("aria-valuenow", plain);
    const spoken = [prefix, text, suffix].filter(Boolean).join(" ");
    if (spoken === plain) box.removeAttribute("aria-valuetext");
    else box.setAttribute("aria-valuetext", spoken);
  };

  /** Draws what the box already holds, at load and after an accepted blur. */
  const settle = () => {
    const value = reading();
    if (value === null) return;
    digits = carried(box.value);
    show(value);
  };

  /** Moves the value, and takes back whatever this script said about it. */
  const settled = (value) => {
    digits = places;
    show(clamp(value, min, max));
    mark(false);
    announce(null);
    box.dispatchEvent(new Event("input", { bubbles: true }));
  };

  const move = (by) => settled((reading() ?? min ?? 0) + by);

  for (const stepper of root.querySelectorAll("[data-nd-number-input-step]")) {
    stepper.addEventListener("click", () => {
      // READONLY REFUSES THEM TOO, which is what the contract says: a value a
      // press can still change is a value that is still editable.
      if (box.disabled || box.readOnly) return;
      move(stepper.dataset.ndNumberInputStep === "up" ? step : -step);
      box.focus();
    });
  }

  // SHOWN ONLY NOW. They ship `hidden`, like `Input`'s clear control, because
  // until this line neither of them could move anything.
  const steppers = root.querySelector("[data-nd-number-input-steppers]");
  if (steppers) steppers.hidden = false;

  box.addEventListener("keydown", (event) => {
    // The same rule as the steppers, from the keyboard.
    if (box.readOnly) return;
    const by = {
      ArrowUp: step,
      ArrowDown: -step,
      PageUp: bigStep,
      PageDown: -bigStep,
    }[event.key];
    if (by !== undefined) {
      event.preventDefault();
      move(by);
      return;
    }
    if (event.key === "Home" && min !== null) {
      event.preventDefault();
      settled(min);
    }
    if (event.key === "End" && max !== null) {
      event.preventDefault();
      settled(max);
    }
  });

  // THE WHEEL IS REFUSED. A reader scrolling the page with the pointer resting
  // here would otherwise change an answer they had already given, silently,
  // and find out at submission. `passive: false` because refusing needs the
  // default prevented and a passive listener may not.
  box.addEventListener("wheel", (event) => event.preventDefault(), { passive: false });

  // Grouping is drawn on blur and taken away on focus, so a separator never
  // appears under the caret as a character the reader has to delete. THAT IS
  // ALL FOCUS DOES: it used to rewrite the value at `step`'s decimals, so
  // clicking into a box holding 7.5 turned it into 8 before a key was pressed.
  box.addEventListener("focus", () => {
    if (!grouped) return;
    const value = reading();
    if (value !== null) show(value);
  });
  box.addEventListener("blur", () => {
    if (box.value.trim() === "") {
      mark(false);
      announce(null);
      return;
    }
    const value = reading();
    if (value === null) {
      mark(true);
      announce(fill(at("invalid-label"), { value: box.value }));
      return;
    }
    // REFUSED AND EXPLAINED, NEVER REWRITTEN — the contract's own words. The
    // number stays as typed so the reader can see what was wrong with it.
    if (min !== null && value < min) {
      mark(true);
      announce(fill(at("min-label"), { min }));
      return;
    }
    if (max !== null && value > max) {
      mark(true);
      announce(fill(at("max-label"), { max }));
      return;
    }
    mark(false);
    announce(null);
    settle();
  });

  // NEVER SUBMITTED, which is what `group` promises. The separators are a
  // drawing, so the entry the form posts is rewritten to the plain number
  // rather than to what the box shows.
  if (grouped && box.form && box.name) {
    box.form.addEventListener("formdata", (event) => {
      // Only a number this control can vouch for. Anything else is posted
      // exactly as the reader typed it.
      const value = reading();
      if (value !== null) event.formData.set(box.name, value.toFixed(digits));
    });
  }

  // AT REST, NOT ONLY ONCE TOUCHED. `group` draws its separators from here,
  // and the spinbutton takes the value it was rendered with.
  settle();
}

// --- Slider ------------------------------------------------------------------

function enhanceSlider(root) {
  const at = reader(root);
  const thumbs = [...root.querySelectorAll("[data-nd-slider-thumb]")];
  if (thumbs.length === 0) return;
  const rails = root.querySelector("[data-nd-slider-rails]");
  const out = root.querySelector("[data-nd-slider-output]");
  const shape = at("output-label");
  const range = at("range") === "true";

  const text = (value) => (shape ? fill(shape, { value }) : String(value));

  const paint = () => {
    const [start, end] = thumbs;
    // NEITHER MAY CROSS THE OTHER. Clamped on input rather than prevented, so
    // a reader dragging past the other thumb stops at it instead of finding
    // the drag refused.
    if (range && end) {
      if (Number(start.value) > Number(end.value)) start.value = end.value;
      if (Number(end.value) < Number(start.value)) end.value = start.value;
    }
    const low = Number(thumbs[0].min || 0);
    const high = Number(thumbs[0].max || 100);
    const span = high - low || 1;
    const fillTo = range && end ? Number(end.value) : Number(start.value);
    rails?.style.setProperty("--nd-slider-fill", `${((fillTo - low) / span) * 100}%`);
    // THE UNIT IS ANNOUNCED, NOT ONLY DRAWN. `37` is a number; *37 pixels* is
    // an answer, and the two must not say different things about one value.
    for (const thumb of thumbs) thumb.setAttribute("aria-valuetext", text(thumb.value));
    if (out) {
      out.textContent =
        range && end ? `${text(start.value)} – ${text(end.value)}` : text(start.value);
    }
  };

  for (const thumb of thumbs) thumb.addEventListener("input", paint);
  paint();

  const marks = at("marks");
  const holder = root.querySelector("[data-nd-slider-marks]");
  if (!marks || !holder) return;
  // The words, read position by position against the values — and `aria-hidden`
  // on the holder already, because the scale is announced by the range itself.
  const words = (at("mark-labels") ?? "").split("|");
  holder.replaceChildren();
  marks
    .split(/\s+/)
    .filter(Boolean)
    .forEach((value, index) => {
      const mark = document.createElement("span");
      mark.className = "nd-slider-mark";
      mark.textContent = words[index]?.trim() || value;
      holder.append(mark);
    });
}

// --- FileInput ---------------------------------------------------------------

/** A size a person can read, in the units a file manager uses. */
function readableSize(bytes, locale) {
  const units = ["B", "kB", "MB", "GB"];
  let value = bytes;
  let unit = 0;
  while (value >= 1024 && unit < units.length - 1) {
    value /= 1024;
    unit += 1;
  }
  const shown = new Intl.NumberFormat(locale, { maximumFractionDigits: unit === 0 ? 0 : 1 });
  return `${shown.format(value)} ${units[unit]}`;
}

function enhanceFileInput(root) {
  const at = reader(root);
  const native = root.querySelector("[data-nd-file-input-native]");
  const area = root.querySelector("[data-nd-file-input-area]");
  const list = root.querySelector("[data-nd-file-input-list]");
  const status = root.querySelector("[data-nd-file-input-status]");
  const template = root.closest(".nd-field")?.querySelector("[data-nd-file-template]");
  if (!native || !list || !template) return;

  const locale = document.documentElement.lang || undefined;
  const max = number(at("max"));
  const maxSize = number(at("max-size"));
  const accept = (native.getAttribute("accept") ?? "")
    .split(",")
    .map((one) => one.trim().toLowerCase())
    .filter(Boolean);

  /** The files chosen so far. A `DataTransfer` because that is the only thing
   *  a `FileList` can be built from, and the input's `files` is the store. */
  const keep = new DataTransfer();

  const allowed = (file) => {
    if (accept.length === 0) return true;
    const name = file.name.toLowerCase();
    const type = (file.type || "").toLowerCase();
    return accept.some((one) =>
      one.startsWith(".")
        ? name.endsWith(one)
        : one.endsWith("/*")
          ? type.startsWith(one.slice(0, -1))
          : type === one,
    );
  };

  const refuse = (sentence) => {
    root.setAttribute("data-nd-invalid", "true");
    say(status, sentence);
  };

  const draw = () => {
    list.replaceChildren();
    if (keep.files.length === 0) {
      const empty = at("empty-label");
      if (empty) {
        const row = document.createElement("li");
        row.className = "nd-file-input-empty";
        row.dataset.ndFileInputEmpty = "";
        row.textContent = empty;
        list.append(row);
      }
    }
    [...keep.files].forEach((file, index) => {
      const row = template.content.firstElementChild.cloneNode(true);
      row.dataset.ndIndex = String(index);
      row.querySelector(".nd-file-input-name").textContent = file.name;
      row.querySelector(".nd-file-input-size").textContent = readableSize(file.size, locale);
      const remove = row.querySelector('[data-nd-action="file-input-remove"]');
      if (remove) {
        // THE FILE, NOT THE WORD. Six rows of *Remove* is six controls a
        // screen-reader user cannot tell apart.
        const name = fill(at("remove-label"), { name: file.name });
        if (name) {
          remove.setAttribute("aria-label", name);
          remove.setAttribute("title", name);
        }
      }
      list.append(row);
    });
    native.files = keep.files;
    say(status, plural(at("status"), keep.files.length), { quiet: true });
  };

  const add = (files) => {
    let taken = 0;
    for (const file of files) {
      if (!native.multiple) keep.items.clear();
      if (max !== null && keep.files.length >= max) {
        refuse(fill(at("max-label"), { max }));
        break;
      }
      if (!allowed(file)) {
        refuse(fill(at("accept-label"), { name: file.name }));
        continue;
      }
      if (maxSize !== null && file.size > maxSize) {
        refuse(fill(at("max-size-label"), { name: file.name, size: readableSize(maxSize, locale) }));
        continue;
      }
      keep.items.add(file);
      taken += 1;
    }
    draw();
    if (taken > 0) {
      root.setAttribute("data-nd-invalid", "false");
      say(status, fill(plural(at("added-label"), taken), { count: taken }));
    }
  };

  native.addEventListener("change", () => {
    const chosen = [...native.files];
    // The input's own list is replaced by ours, so a second choice adds to the
    // first rather than replacing it — which is what `multiple` means.
    native.value = "";
    add(chosen);
  });

  root.addEventListener("click", (event) => {
    const remove = event.target.closest?.('[data-nd-action="file-input-remove"]');
    if (!remove) return;
    const row = remove.closest("[data-nd-index]");
    if (!row) return;
    const index = Number(row.dataset.ndIndex);
    const gone = keep.files[index]?.name;
    keep.items.remove(index);
    draw();
    say(status, fill(at("removed-label"), { count: 1, name: gone ?? "" }));
    // THE CONTROL THAT DID THIS HAS LEFT THE PAGE. Focus goes to the next
    // file's control, or to the area, rather than falling to `<body>`.
    const rows = list.querySelectorAll("[data-nd-index]");
    const next = rows[Math.min(index, rows.length - 1)];
    (next?.querySelector('[data-nd-action="file-input-remove"]') ?? native).focus();
  });

  // Delete on a focused row, which is the gesture a reader tries first.
  root.addEventListener("keydown", (event) => {
    if (event.key !== "Delete" && event.key !== "Backspace") return;
    const remove = event.target.closest?.('[data-nd-action="file-input-remove"]');
    if (!remove) return;
    event.preventDefault();
    remove.click();
  });

  if (at("drop") === "true" && area) {
    for (const kind of ["dragenter", "dragover"]) {
      area.addEventListener(kind, (event) => {
        event.preventDefault();
        root.setAttribute("data-nd-dragging", "");
      });
    }
    for (const kind of ["dragleave", "dragend"]) {
      // CLEARED ON LEAVING AS WELL AS ON DROPPING: an area that stays lit is
      // an area that looks stuck.
      area.addEventListener(kind, () => root.removeAttribute("data-nd-dragging"));
    }
    area.addEventListener("drop", (event) => {
      event.preventDefault();
      root.removeAttribute("data-nd-dragging");
      add(event.dataTransfer?.files ?? []);
    });
  }

  draw();
}

// --- The walk a row of boxes shares ------------------------------------------

/**
 * Auto-advance, Backspace back, and a paste that splits across the boxes.
 *
 * ONE IMPLEMENTATION, TWO CONTROLS. `TimeInput` needs these three gestures for
 * its hour and minute boxes and `OtpField` needs exactly the same three for the
 * characters of a code. A second copy would be a second place to fix the day
 * one of them is wrong, which is what `NOCTUA.md §9.2` is about.
 *
 * What each control keeps for itself is what is NOT shared: the arrow-key step
 * and the padding are a time's, and replacing a full box is a code's.
 *
 * @param {HTMLInputElement[]} boxes in document order
 * @param {object} [options]
 * @param {number} [options.size] characters one box holds
 * @param {RegExp} [options.strip] what to drop as it is typed
 * @param {number} [options.minPaste] the fewest characters that count as the
 *   whole value rather than as one box's worth
 * @param {(box: HTMLInputElement, text: string) => void} [options.write]
 * @param {() => void} [options.changed] after anything this moved
 */
export function segmentWalk(boxes, options = {}) {
  const {
    size = 2,
    strip = /\D/g,
    minPaste = 3,
    write = (box, text) => {
      box.value = text;
    },
    changed = () => {},
  } = options;

  const at = (box) => boxes.indexOf(box);

  for (const box of boxes) {
    box.addEventListener("keydown", (event) => {
      // Backspace in an empty box goes back to the previous one, which is the
      // gesture every reader tries. In a box with something in it the browser
      // is already right.
      if (event.key !== "Backspace" || box.value !== "") return;
      const index = at(box);
      if (index <= 0) return;
      event.preventDefault();
      const previous = boxes[index - 1];
      previous.focus();
      previous.setSelectionRange(previous.value.length, previous.value.length);
    });

    box.addEventListener("input", () => {
      box.value = box.value.replace(strip, "").slice(0, size);
      if (box.value.length === size) {
        const next = boxes[at(box) + 1];
        next?.focus();
        next?.select();
      }
      changed();
    });

    box.addEventListener("paste", (event) => {
      const text = (event.clipboardData?.getData("text") ?? "").replace(strip, "");
      if (text.length < minPaste) return;
      event.preventDefault();
      // FROM THE BOX IT LANDED IN, not from the first: a reader correcting the
      // tail of a value pastes into the tail.
      const from = Math.max(0, at(box));
      boxes.slice(from).forEach((one, offset) => {
        const piece = text.slice(offset * size, offset * size + size);
        if (piece) write(one, piece);
      });
      const last = boxes[Math.min(boxes.length - 1, from + Math.ceil(text.length / size) - 1)];
      last?.focus();
      changed();
    });
  }
}

// --- TimeInput ---------------------------------------------------------------

/** Whether this locale writes its times on a twelve-hour clock. */
function twelveHour(locale) {
  try {
    return new Intl.DateTimeFormat(locale, { hour: "numeric" }).resolvedOptions().hour12 === true;
  } catch {
    return false;
  }
}

function enhanceTimeInput(root) {
  const at = reader(root);
  const segments = [...root.querySelectorAll("[data-nd-time-input-segment]")];
  if (segments.length === 0) return;
  const status = root.querySelector("[data-nd-time-input-status]");
  const row = root.querySelector("[data-nd-time-input-row]");
  const meridiem = root.querySelector("[data-nd-time-input-meridiem]");
  const pad = at("pad") !== "false";
  const step = number(at("step")) ?? 1;
  const locale = document.documentElement.lang || undefined;
  const clock = at("clock");
  const twelve = clock === "12" || (clock === "auto" && twelveHour(locale));

  const bounds = { hour: [0, 23], minute: [0, 59], second: [0, 59] };
  const of = (part) => segments.find((one) => one.dataset.ndTimeInputSegment === part);

  // THE VALUE ARRIVES AS TWENTY-FOUR HOURS AND LEAVES AS TWENTY-FOUR HOURS,
  // whatever the reader was shown — a server should not have to know what
  // clock somebody's phone keeps.
  const given = (at("value") ?? "").split(":");
  segments.forEach((segment, index) => {
    if (given[index] !== undefined && given[index] !== "") {
      segment.value = pad ? String(given[index]).padStart(2, "0") : given[index];
    }
  });

  let afternoon = Number(given[0] ?? -1) >= 12;

  if (twelve && meridiem) {
    meridiem.hidden = false;
    // A choice between two named things, which is what `Segmented` is for —
    // built here rather than shipped because a control that appears for some
    // readers and not others cannot be server-rendered honestly.
    const group = document.createElement("div");
    group.className = "nd-segmented nd-time-input-meridiem-group";
    group.setAttribute("role", "radiogroup");
    group.dataset.ndSize = "small";
    const named = root.querySelector(".nd-visually-hidden[id$='-meridiem-label']");
    if (named) group.setAttribute("aria-labelledby", named.id);
    for (const [value, word] of [
      ["am", at("am-label")],
      ["pm", at("pm-label")],
    ]) {
      const button = document.createElement("button");
      button.type = "button";
      button.className = "nd-segmented-item";
      button.setAttribute("role", "radio");
      button.dataset.ndValue = value;
      button.setAttribute("aria-checked", String((value === "pm") === afternoon));
      button.tabIndex = (value === "pm") === afternoon ? 0 : -1;
      button.textContent = word ?? value;
      button.addEventListener("click", () => {
        afternoon = value === "pm";
        for (const other of group.querySelectorAll("[data-nd-value]")) {
          const on = other.dataset.ndValue === value;
          other.setAttribute("aria-checked", String(on));
          other.tabIndex = on ? 0 : -1;
        }
        commit();
      });
      group.append(button);
    }
    meridiem.append(group);

    // The hour box is read on twelve now, so what it shows is re-laid — the
    // boxes MOVE rather than being restyled, which is why `data-nd-resolved`
    // is what a test asks about.
    const hour = of("hour");
    if (hour && hour.value !== "") {
      const raw = Number(hour.value);
      const shown = raw % 12 === 0 ? 12 : raw % 12;
      hour.value = pad ? String(shown).padStart(2, "0") : String(shown);
    }
    bounds.hour = [1, 12];
    if (row && meridiem) row.append(meridiem);
  }
  root.setAttribute("data-nd-resolved", "true");

  /** What the boxes currently say, on a twenty-four hour clock, or `null`. */
  function readAll() {
    const parts = {};
    for (const part of ["hour", "minute", "second"]) {
      const segment = of(part);
      if (!segment) continue;
      if (segment.value.trim() === "") return { missing: part };
      const value = number(segment.value);
      const [low, high] = bounds[part];
      if (value === null || value < low || value > high) return { invalid: segment.value };
      parts[part] = value;
    }
    if (twelve) {
      const raw = parts.hour % 12;
      parts.hour = afternoon ? raw + 12 : raw;
    }
    return parts;
  }

  /** Writes what the boxes mean back onto the fieldset, and says what is wrong. */
  function commit() {
    const read = readAll();
    if (read.missing !== undefined) {
      // Half a time is the commonest way this goes wrong and the one the
      // platform has nothing to say about.
      const empty = segments.every((one) => one.value.trim() === "");
      say(status, empty ? at("required-label") : fill(at("missing-label"), { part: read.missing }));
      root.setAttribute("data-nd-invalid", String(!empty));
      return;
    }
    if (read.invalid !== undefined) {
      say(status, fill(at("invalid-label"), { value: read.invalid }));
      root.setAttribute("data-nd-invalid", "true");
      return;
    }
    const spelled = ["hour", "minute", "second"]
      .filter((part) => read[part] !== undefined)
      .map((part) => String(read[part]).padStart(2, "0"))
      .join(":");
    const low = at("min");
    const high = at("max");
    // A WINDOW MAY CROSS MIDNIGHT, which is why this is not one comparison.
    const inside =
      low && high && low > high
        ? spelled >= low || spelled <= high
        : (!low || spelled >= low) && (!high || spelled <= high);
    if (!inside) {
      say(status, spelled < (low ?? "") ? fill(at("min-label"), { min: low }) : fill(at("max-label"), { max: high }));
      root.setAttribute("data-nd-invalid", "true");
      return;
    }
    root.setAttribute("data-nd-invalid", "false");
    root.setAttribute("data-nd-value", spelled);
    say(status, "");
  }

  // THE THREE GESTURES A ROW OF BOXES SHARES, from `segmentWalk`: two digits
  // fill a box and the caret moves on, Backspace in an empty box goes back, and
  // a pasted time splits itself across the row. `OtpField` calls the same
  // function, which is the point of it being one.
  segmentWalk(segments, {
    size: 2,
    minPaste: 3,
    write: (one, piece) => {
      one.value = pad ? piece.padStart(2, "0") : piece;
    },
    changed: commit,
  });

  for (const segment of segments) {
    const part = segment.dataset.ndTimeInputSegment;
    const [low, high] = bounds[part];

    segment.addEventListener("keydown", (event) => {
      const by = { ArrowUp: 1, ArrowDown: -1 }[event.key];
      if (by === undefined) return;
      event.preventDefault();
      const move = part === "minute" ? step * by : by;
      const from = number(segment.value) ?? low - (by > 0 ? 1 : 0);
      // IT WRAPS, because the hour after 23 is 00 and a reader stepping past
      // midnight meant it.
      const span = high - low + 1;
      const next = (((from + move - low) % span) + span) % span + low;
      segment.value = pad ? String(next).padStart(2, "0") : String(next);
      commit();
    });

    segment.addEventListener("blur", () => {
      // Padded on leaving and never while the caret is in it, where a leading
      // zero is a character the reader has to delete.
      if (pad && segment.value.length === 1) segment.value = segment.value.padStart(2, "0");
      commit();
    });
  }

  const now = root.querySelector('[data-nd-action="time-input-now"]');
  now?.addEventListener("click", () => {
    const clockNow = new Date();
    const values = { hour: clockNow.getHours(), minute: clockNow.getMinutes(), second: clockNow.getSeconds() };
    afternoon = values.hour >= 12;
    for (const part of ["hour", "minute", "second"]) {
      const segment = of(part);
      if (!segment) continue;
      let value = values[part];
      if (twelve && part === "hour") value = value % 12 === 0 ? 12 : value % 12;
      segment.value = pad ? String(value).padStart(2, "0") : String(value);
    }
    for (const button of root.querySelectorAll(".nd-time-input-meridiem-group [data-nd-value]")) {
      const on = (button.dataset.ndValue === "pm") === afternoon;
      button.setAttribute("aria-checked", String(on));
      button.tabIndex = on ? 0 : -1;
    }
    commit();
  });

  commit();
}

// --- OtpField ----------------------------------------------------------------

/**
 * One short code, in a row of boxes that fill and move on.
 *
 * Almost all of it is `segmentWalk`. What is here is the three things a code
 * needs that a time does not: a box that already holds a character is replaced
 * rather than appended to, the arrows and Home and End walk the row, and the
 * refusal is said when the field is LEFT rather than between two boxes.
 */
function enhanceOtpField(root) {
  const at = reader(root);
  const boxes = [...root.querySelectorAll("[data-nd-otp-field-segment]")];
  if (boxes.length === 0) return;
  const status = root.querySelector("[data-nd-otp-field-status]");
  // The same two sets the markup already asked the platform for. Anything else
  // is dropped as it is typed rather than refused after the fact.
  const strip = at("kind") === "alphanumeric" ? /[^0-9a-z]/gi : /[^0-9]/g;

  const read = () => boxes.map((one) => one.value).join("");

  /** What is wrong, said on leaving and never mid-answer. */
  function judge() {
    const code = read();
    // AN UNFINISHED ANSWER IS NOT A WRONG ONE. An empty row is the browser's
    // own `required` to report, and a full one is right by construction.
    const wrong = code.length > 0 && code.length < boxes.length;
    say(status, wrong ? at("invalid-label") : "");
    root.setAttribute("data-nd-invalid", String(wrong));
  }

  segmentWalk(boxes, {
    size: 1,
    strip,
    // Two characters is already more than one box holds, so anything longer
    // than a single keystroke is the code rather than one box's worth.
    minPaste: 2,
    changed: () => root.setAttribute("data-nd-value", read()),
  });

  for (const box of boxes) {
    // Typing over a box that is already full. Without this the character is
    // dropped by `maxlength` and a reader correcting one digit has to delete
    // it first.
    box.addEventListener("beforeinput", (event) => {
      if (event.inputType !== "insertText") return;
      if (box.value.length === 1 && box.selectionStart === box.selectionEnd) box.select();
    });
    box.addEventListener("focus", () => box.select());
    box.addEventListener("keydown", (event) => {
      const step = { ArrowLeft: -1, ArrowRight: 1 }[event.key];
      const index = boxes.indexOf(box);
      let next = null;
      if (step !== undefined) next = boxes[index + step];
      else if (event.key === "Home") next = boxes[0];
      else if (event.key === "End") next = boxes[boxes.length - 1];
      if (!next || next === box) return;
      event.preventDefault();
      next.focus();
      next.select();
    });
  }

  // LEAVING THE FIELD, not leaving a box. A `blur` fires between two boxes of
  // an answer nobody has finished, and refusing there is refusing the reader
  // mid-word.
  root.addEventListener("focusout", (event) => {
    if (root.contains(event.relatedTarget)) return;
    judge();
  });
  root.closest("form")?.addEventListener("submit", judge);
}

// --- Wiring ------------------------------------------------------------------

/** Every control this module owns, by the hook its own `name` prop writes. */
const CONTROLS = [
  ["[data-nd-input]", enhanceInput],
  ["[data-nd-textarea]", enhanceTextarea],
  ["[data-nd-number-input]", enhanceNumberInput],
  ["[data-nd-slider]", enhanceSlider],
  ["[data-nd-file-input]", enhanceFileInput],
  ["[data-nd-time-input]", enhanceTimeInput],
  ["[data-nd-otp-field]", enhanceOtpField],
];

/**
 * Gives every input control under `root` its behaviour.
 *
 * Idempotent: one already enhanced is skipped, so a page that adds a control
 * later can call this again with nothing to undo.
 */
export function enhanceInputs(root = document) {
  for (const [selector, enhance] of CONTROLS) {
    for (const element of root.querySelectorAll(selector)) {
      if (element.dataset.ndEnhanced) continue;
      element.dataset.ndEnhanced = "input";
      enhance(element);
    }
  }
}
