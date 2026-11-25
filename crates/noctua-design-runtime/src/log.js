// Everything a `Log` does once a script is running.
//
// IT SHIPS, for the reason `menu.js` gives: a component whose consumer cannot
// use it is a component that lies. A log that does not follow its newest line
// is a region a reader has to scroll by hand every time a line arrives.
//
// WHAT IT DOES:
//
// 1. IT FOLLOWS THE END. With `follow`, a line added to the region is brought
//    into view, and the log opens at its newest line.
// 2. IT STOPS WHEN THE READER SCROLLS AWAY, and when a line arrives while a
//    selection is held inside the log, so the text being read or copied does
//    not move under the reader. Stopped, the log writes `data-nd-paused` and
//    shows the resume control.
// 3. IT FOLLOWS AGAIN AT THE END: a reader who scrolls back to the newest
//    line, or presses End there, or presses the resume control.
// 4. IT SAYS SO. `nd-log-follow`, with the log's `name` and whether it now
//    follows, each time that changes and never otherwise.
//
// WHAT IT NEVER DOES:
//
// 1. It never adds, removes or rewrites a line. The lines are the caller's,
//    and so is how many of them there are.
// 2. It never moves the focus, but for one case: the resume control is hidden
//    once pressed, so the keyboard on it goes to the region it scrolled.
// 3. It never animates the scroll. A line arriving is drawn where it is, at
//    once, under every motion setting.

/** How far from the end, in pixels, still counts as at it: a rounding error. */
const SLACK = 2;

/** Every enhanced log's teardown, so a subtree can be released. */
const logs = new WeakMap();

export function enhanceLogs(root = document) {
  for (const log of root.querySelectorAll("[data-nd-log]")) {
    if (log.dataset.ndEnhanced) continue;
    const lines = log.querySelector(":scope > .nd-log-lines");
    if (!lines) continue;
    log.dataset.ndEnhanced = "log";
    logs.set(log, enhance(log, lines));
  }
}

/**
 * Takes the behaviour back off every log under `root`.
 *
 * A BEHAVIOUR THAT CANNOT BE RELEASED IS A DEFECT FOR EVERY CONSUMER WITH A
 * CLIENT-SIDE ROUTER. The two observers go with the listeners, and so does what
 * this file wrote: the mark that following stopped, and the resume control,
 * hidden again as the markup ships it.
 */
export function releaseLogs(root = document) {
  for (const log of root.querySelectorAll("[data-nd-log]")) {
    if (log.dataset.ndEnhanced !== "log") continue;
    logs.get(log)?.();
    logs.delete(log);
    delete log.dataset.ndEnhanced;
  }
}

function enhance(log, lines) {
  const binding = new AbortController();
  const { signal } = binding;
  const resume = log.querySelector(":scope > .nd-log-resume");

  let paused = false;
  /** Where this file last put the region, so a scroll it made is known. */
  let written = -1;

  const follows = () => log.getAttribute("data-nd-follow") !== "false";
  const atEnd = () => lines.scrollHeight - lines.clientHeight - lines.scrollTop <= SLACK;

  /** Whether the reader holds a selection inside the lines. */
  function selecting() {
    const selection = document.getSelection();
    return Boolean(selection && !selection.isCollapsed && lines.contains(selection.anchorNode));
  }

  function toEnd() {
    lines.scrollTop = lines.scrollHeight;
    written = lines.scrollTop;
  }

  function say(following) {
    log.dispatchEvent(
      new CustomEvent("nd-log-follow", {
        bubbles: true,
        detail: { name: log.getAttribute("data-nd-log"), follow: following },
      }),
    );
  }

  function pause() {
    if (paused || !follows()) return;
    paused = true;
    log.setAttribute("data-nd-paused", "true");
    if (resume) resume.hidden = false;
    say(false);
  }

  function carryOn() {
    if (!paused) return;
    paused = false;
    log.removeAttribute("data-nd-paused");
    if (resume) resume.hidden = true;
    say(true);
  }

  lines.addEventListener("scroll", () => {
    if (!follows()) return;
    // A SCROLL THAT ENDS AT THE END FOLLOWS, whoever made it: the reader, End,
    // or this file.
    if (atEnd()) {
      carryOn();
      return;
    }
    // A SCROLL THIS FILE MADE, AND A LAYOUT OVERTOOK. The event is told a
    // frame after the scroll, and a face that loaded in between made the lines
    // taller: the region is where this file put it, and the end has moved on.
    if (!paused && lines.scrollTop === written) {
      toEnd();
      return;
    }
    // Anywhere else is where the reader went.
    pause();
  }, { signal, passive: true });

  resume?.addEventListener("click", () => {
    const keyboard = document.activeElement === resume;
    toEnd();
    carryOn();
    // The control is hidden now, and the keyboard on it would be on nothing.
    if (keyboard) lines.focus({ preventScroll: true });
  }, { signal });

  // A LINE ARRIVED WHILE THE READER HOLDS A SELECTION. The following stops
  // before the lines are drawn again, so what is being selected stays where
  // it is. Told before the observer below, which is told after layout.
  const added = new MutationObserver(() => {
    if (follows() && !paused && selecting()) pause();
  });
  added.observe(lines, { childList: true, subtree: true, characterData: true });

  // THE END MOVED, AND A LOG THAT FOLLOWS GOES WITH IT. The lines grow when
  // one is added, and when a face that loads late or a narrower window makes
  // every line taller with none added. The region changes size with its box.
  const resized = new ResizeObserver(() => {
    if (follows() && !paused) toEnd();
  });
  resized.observe(lines);
  const content = lines.querySelector(":scope > .nd-log-content");
  if (content) resized.observe(content);

  // `follow` turned off is the reader's place kept: nothing stopped, and
  // nothing to resume. Turned on, the log goes to its end.
  const turned = new MutationObserver(() => {
    if (!follows()) {
      paused = false;
      log.removeAttribute("data-nd-paused");
      if (resume) resume.hidden = true;
    } else if (!paused) {
      toEnd();
    }
  });
  turned.observe(log, { attributes: true, attributeFilter: ["data-nd-follow"] });

  if (follows()) toEnd();

  return () => {
    added.disconnect();
    resized.disconnect();
    turned.disconnect();
    binding.abort();
    log.removeAttribute("data-nd-paused");
    if (resume) resume.hidden = true;
  };
}
