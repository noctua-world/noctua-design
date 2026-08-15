// Everything a `Carousel` does once a script is running.
//
// IT SHIPS, for `ColorPicker`'s reason: a component whose consumer cannot use
// it is a component that lies.
//
// WHAT IT NEVER DOES, AND THESE ARE THE WHOLE DESIGN:
//
// 1. It never sets a transform, a `left`, or a slide index. The track is a
//    scroll container and this calls `scrollTo` on it — so dragging, flicking,
//    momentum, the trackpad, the scrollbar, Home/End and a reader's own
//    reduced-motion setting all keep working, because none of them was ours to
//    break. Every hand-built slideshow this avoids gets one of those six
//    subtly wrong.
// 2. It never moves focus. Not the controls, not the dots, not autoplay. A
//    carousel that focuses the slide it just brought in tears a reader out of
//    wherever they were; the announcement is how they are told instead.
// 3. It never hides a slide from a keyboard. `inert` on what is off screen is
//    how a reader comes to Tab into a link they cannot see and then lose it
//    mid-scroll. The browser scrolls the focused thing into view, which is
//    better than anything this could do about it.
//
// AND THE ONE THING IT REFUSES TO DO: rotate without a named pause control.
// WCAG 2.2.2 wants a way to stop anything that moves for more than five
// seconds. Making that a condition of starting is the only version of the rule
// that cannot be forgotten.

import { fill, say } from "./messages.js";

/** How long after the last scroll the position is treated as settled. */
const SETTLE = 120;

/** Every attribute read the same way, from the root. */
function reader(root) {
  return (name) => root.getAttribute(`data-nd-${name}`) || null;
}

/** Whether the reader has asked for less movement. */
function stillness() {
  return matchMedia("(prefers-reduced-motion: reduce)").matches;
}

/** The scroll behaviour a reader's own setting allows. */
function behavior() {
  return stillness() ? "auto" : "smooth";
}

/** Every enhanced carousel's teardown, so a subtree can be released. */
const carousels = new WeakMap();

export function enhanceCarousels(root = document) {
  for (const element of root.querySelectorAll("[data-nd-carousel]")) {
    if (element.dataset.ndEnhanced) continue;
    element.dataset.ndEnhanced = "carousel";
    const binding = new AbortController();
    carousels.set(element, binding);
    enhance(element, binding.signal);
  }
}

/**
 * Takes the behaviour back off every carousel under `root`.
 *
 * A BEHAVIOUR THAT CANNOT BE RELEASED IS A DEFECT FOR EVERY CONSUMER WITH A
 * CLIENT-SIDE ROUTER, and this file has three reasons of its own. It listens
 * for `visibilitychange` on the DOCUMENT, it observes the track for resizes,
 * and autoplay is a `setTimeout` that arms the next one. None of the three
 * stops when the element leaves the page, so a rotation nobody can see keeps
 * scrolling a detached track for as long as the tab is open.
 */
export function releaseCarousels(root = document) {
  for (const element of root.querySelectorAll("[data-nd-carousel]")) {
    if (element.dataset.ndEnhanced !== "carousel") continue;
    carousels.get(element)?.abort();
    carousels.delete(element);
    delete element.dataset.ndEnhanced;
  }
}

function enhance(root, signal) {
  const at = reader(root);
  const track = root.querySelector("[data-nd-carousel-track]");
  if (!track) return;
  const slides = [...track.querySelectorAll("[data-nd-carousel-slide]")];
  if (slides.length === 0) return;

  const status = root.querySelector("[data-nd-carousel-status]");
  const dots = root.querySelector("[data-nd-carousel-dots]");
  const counter = root.querySelector("[data-nd-carousel-counter]");
  const rotation = root.querySelector("[data-nd-carousel-rotation]");
  const steps = [...root.querySelectorAll("[data-nd-carousel-step]")];
  const template = root.querySelector("[data-nd-dot-template]");
  const vertical = root.getAttribute("data-nd-orientation") === "vertical";
  const wraps = at("wrap") === "true";
  const count = slides.length;
  // Whether a slide is where the track comes to rest, which decides what the
  // two controls do. See `step`.
  const snaps = at("snap") !== "none";
  /** Whether the track's inline axis runs the other way. */
  const rightToLeft = () => getComputedStyle(track).direction === "rtl";

  // `--nd-carousel-per-view` IS NOT WRITTEN HERE ANY MORE. It used to be, and
  // that made a width depend on a script: a `per_view: "3"` carousel showed
  // one slide with JavaScript blocked, under a contract promising the slides
  // "scroll and snap before any script runs". The component renders the
  // property on the track, where a prop belongs, and the stylesheet divides by
  // it exactly as before.

  // EVERY SLIDE IS NAMED BY ITS POSITION, from the slides that are actually
  // here — a count a caller wrote goes wrong the first time one is added. A
  // `label` of their own is joined to it rather than replacing it: losing the
  // position to gain a description is a bad trade.
  slides.forEach((slide, index) => {
    const own = slide.getAttribute("data-nd-slide-label");
    const position = fill(at("announce-label") ?? "", { index: index + 1, count });
    const name = [position, own].filter(Boolean).join(" — ");
    if (name) slide.setAttribute("aria-label", name);
  });

  // THE TRACK IS A TAB STOP ONLY WHEN NOTHING IN IT IS. A scroll container
  // with no focusable content is a region a keyboard cannot scroll; one that
  // is focusable *and* full of links is an extra press before every slide.
  const reachable = track.querySelector(
    "a[href], button, input, select, textarea, [tabindex]:not([tabindex='-1'])",
  );
  if (!reachable) track.tabIndex = 0;

  /** Which slide is nearest the track's leading edge. */
  function current() {
    const box = track.getBoundingClientRect();
    const edge = vertical ? box.top : box.left;
    let best = 0;
    let closest = Infinity;
    slides.forEach((slide, index) => {
      const slideBox = slide.getBoundingClientRect();
      const away = Math.abs((vertical ? slideBox.top : slideBox.left) - edge);
      if (away < closest) {
        closest = away;
        best = index;
      }
    });
    return best;
  }

  /** Whether the track is against an end, which a slide index cannot know. */
  function ends() {
    const size = vertical ? "scrollHeight" : "scrollWidth";
    const seen = vertical ? "clientHeight" : "clientWidth";
    const from = vertical ? track.scrollTop : Math.abs(track.scrollLeft);
    // A PARTLY VISIBLE LAST SLIDE IS STILL THE END, and one pixel of rounding
    // is not the middle — which is why this is a measurement and not a
    // comparison of indices.
    return { start: from <= 1, end: from + track[seen] >= track[size] - 1 };
  }

  function goTo(index) {
    const wanted = slides[Math.max(0, Math.min(index, count - 1))];
    if (!wanted) return;
    // `scrollIntoView` on the slide rather than a computed offset: it is the
    // browser's own arithmetic, it honours the snap alignment already in the
    // stylesheet, and it is right in a right-to-left locale with no second
    // rule.
    wanted.scrollIntoView({
      behavior: behavior(),
      block: vertical ? "start" : "nearest",
      inline: vertical ? "nearest" : "start",
    });
  }

  function step(by) {
    // `snap: none` IS A PLAIN SCROLLER, AND THE TWO CONTROLS SAY SO. The
    // contract has always read "turns the two controls into a scroll by one
    // screen rather than to a slide", and this file never read the attribute
    // at all — so a carousel a caller had deliberately unsnapped still jumped
    // slide by slide, which is the one behaviour the value exists to refuse.
    //
    // Wrapping is the contract's business and not the snapping's, so a
    // scroller that wraps still goes round at either end.
    if (!snaps) {
      const edge = ends();
      if (wraps && ((by < 0 && edge.start) || (by > 0 && edge.end))) {
        goTo(by < 0 ? count - 1 : 0);
        return;
      }
      // ONE SCREEN OF THE TRACK, measured rather than assumed, and reversed in
      // a right-to-left locale: `scrollBy` takes a physical offset, so *next*
      // is a negative one wherever the inline axis runs the other way.
      const screen = vertical ? track.clientHeight : track.clientWidth;
      const away = screen * by * (!vertical && rightToLeft() ? -1 : 1);
      track.scrollBy({
        top: vertical ? away : 0,
        left: vertical ? 0 : away,
        behavior: behavior(),
      });
      return;
    }
    const from = current();
    let next = from + by;
    if (next < 0) next = wraps ? count - 1 : 0;
    if (next > count - 1) next = wraps ? 0 : count - 1;
    goTo(next);
  }

  /** Repaints everything that says where the track is. */
  function paint() {
    const index = current();
    const edge = ends();
    root.toggleAttribute("data-nd-at", edge.start || edge.end);
    if (edge.start || edge.end) {
      root.setAttribute("data-nd-at", edge.start && edge.end ? "start end" : edge.start ? "start" : "end");
    }
    for (const control of steps) {
      const back = control.dataset.ndCarouselStep === "previous";
      // Disabled at the ends only when it does not wrap — a control that
      // wraps is never at an end.
      control.disabled = !wraps && (back ? edge.start : edge.end);
    }
    if (counter) counter.textContent = fill(at("counter-label"), { index: index + 1, count });
    if (dots) {
      [...dots.children].forEach((dot, position) => {
        dot.setAttribute("aria-current", String(position === index));
      });
    }
    return index;
  }

  // The dots, one per slide the runtime found, cloned from the component's own
  // template — a script that drew one would be a second place that knows what
  // a dot looks like.
  if (dots && template) {
    dots.replaceChildren();
    slides.forEach((_, index) => {
      const dot = template.content.firstElementChild.cloneNode(true);
      const name = fill(at("dot-label"), { index: index + 1, count });
      if (name) {
        dot.setAttribute("aria-label", name);
        dot.setAttribute("title", name);
      }
      dot.addEventListener("click", () => {
        stop();
        goTo(index);
      }, { signal });
      dots.append(dot);
    });
    dots.hidden = false;
  }

  for (const control of steps) {
    control.hidden = false;
    control.addEventListener("click", () => {
      stop();
      step(control.dataset.ndCarouselStep === "previous" ? -1 : 1);
    }, { signal });
  }
  if (counter) counter.hidden = false;

  let announced = -1;
  let settling = null;
  track.addEventListener(
    "scroll",
    () => {
      clearTimeout(settling);
      // ONE SENTENCE PER PAUSE, NOT ONE PER PIXEL. A live region that speaks
      // through a flick is a live region a reader turns off.
      settling = setTimeout(() => {
        const index = paint();
        if (index === announced) return;
        announced = index;
        if (!rotating) {
          say(status, fill(at("announce-label"), { index: index + 1, count }));
        }
      }, SETTLE);
    },
    { passive: true, signal },
  );

  // --- Autoplay, and the four rules it carries -------------------------------

  const every = Number.parseInt(at("autoplay") ?? "", 10);
  const pauseLabel = at("pause-label");
  const playLabel = at("play-label");
  // REFUSED WITHOUT A NAMED CONTROL. WCAG 2.2.2 wants a way to stop anything
  // that moves for more than five seconds; making that a condition of starting
  // is the only version of the rule that cannot be forgotten. And never under
  // reduced motion, which is a reader saying so in as many words.
  const allowed =
    Number.isFinite(every) && every > 0 && Boolean(pauseLabel) && Boolean(rotation) && !stillness();
  let rotating = false;
  let timer = null;
  let stopped = !allowed;
  // AND POLITENESS COMES BACK WHEN ROTATION IS REFUSED. The markup ships
  // `aria-live="off"` on the track of a carousel that declares autoplay,
  // because something speaking every few seconds unasked makes the rest of the
  // page unusable. Only `start` and `hold` ever moved it back, and under
  // `prefers-reduced-motion` neither runs — so the track stayed silent for
  // ever, for exactly the reader who is driving it by hand.
  if (!allowed) track.setAttribute("aria-live", "polite");

  function tick() {
    if (!rotating) return;
    step(1);
    timer = setTimeout(tick, every);
  }

  function start() {
    if (stopped || rotating) return;
    rotating = true;
    root.removeAttribute("data-nd-paused");
    rotation?.setAttribute("aria-pressed", "false");
    if (pauseLabel) {
      rotation?.setAttribute("aria-label", pauseLabel);
      rotation?.setAttribute("title", pauseLabel);
    }
    // The region goes quiet while it rotates: something that speaks every four
    // seconds unasked makes the rest of the page unusable.
    track.setAttribute("aria-live", "off");
    clearTimeout(timer);
    timer = setTimeout(tick, every);
  }

  /** Pauses while something is true — a pointer, focus, a hidden tab. */
  function hold() {
    rotating = false;
    clearTimeout(timer);
    track.setAttribute("aria-live", "polite");
  }

  /** STOPS FOR GOOD. Anything that resumes under a reader who has taken over
   *  is something they have to fight, so a manual interaction ends it. */
  function stop() {
    if (stopped) return;
    stopped = true;
    hold();
    root.setAttribute("data-nd-paused", "");
    rotation?.setAttribute("aria-pressed", "true");
    if (playLabel) {
      rotation?.setAttribute("aria-label", playLabel);
      rotation?.setAttribute("title", playLabel);
    }
  }

  if (allowed) {
    rotation.hidden = false;
    rotation.addEventListener("click", () => {
      if (stopped) {
        stopped = false;
        start();
      } else {
        stop();
      }
    }, { signal });
    // Paused while a pointer is over it, while focus is inside it, and while
    // the tab is away — three ways a reader is doing something else.
    root.addEventListener("pointerenter", hold, { signal });
    root.addEventListener("pointerleave", () => start(), { signal });
    root.addEventListener("focusin", hold, { signal });
    root.addEventListener("focusout", (event) => {
      if (!root.contains(event.relatedTarget)) start();
    }, { signal });
    // ON THE DOCUMENT, so it outlives the element unless something takes it
    // off. That is the listener `releaseCarousels` exists for.
    document.addEventListener(
      "visibilitychange",
      () => (document.hidden ? hold() : start()),
      { signal },
    );
    // And it stops the moment a reader scrolls it themselves.
    track.addEventListener("pointerdown", stop, { passive: true, signal });
    track.addEventListener("wheel", stop, { passive: true, signal });
    track.addEventListener("keydown", stop, { signal });
    start();
  } else if (rotation) {
    // Asked for and refused: the control would be a button that toggles a
    // rotation which never happens.
    rotation.remove();
  }

  paint();
  // A track that was hidden when the page loaded measures as nothing, so the
  // ends are measured again the first time it has a size.
  const watch = new ResizeObserver(() => paint());
  watch.observe(track);
  // THE TEARDOWN IS WRITTEN BESIDE WHAT IT UNDOES, on the same signal the
  // listeners take. A second registry of things to disconnect is a second list
  // that stops agreeing with the first.
  signal.addEventListener("abort", () => {
    watch.disconnect();
    clearTimeout(timer);
    clearTimeout(settling);
  });
}
