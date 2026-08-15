// The camera, the keyboard walk and the switches — everything a `Lineage`
// gains when a script is running.
//
// WHAT IT IS NOT. It does not build the scene: the scene is server-rendered,
// three-dimensional and scrollable before this file is fetched, and every
// figure, every connection and every pick works with it blocked. This adds a
// camera to a picture that was already there.
//
// IT IS FETCHED ON DEMAND. A page that is only read costs nothing; the module
// arrives the first time a reader points at a scene, presses a key in one or
// asks for the camera. three.js arrives with it, and not before.
//
// WHY `CSS3DRenderer` IS GIVEN ONE OBJECT. It is built to place many objects
// that move independently, and it does that by re-parenting each element into
// its own camera element. This scene does not move independently — the layout
// engine already baked every place into a transform — so it is one rigid
// object, and giving the renderer exactly one of them buys the whole camera
// model (a real field of view, a real projection, the matrix CSS3D expects)
// while leaving every figure where the server put it. Re-parenting eighty
// elements out of the scroll container would take the scrolling, the clipping
// and the reading order with them.
//
// EVERY WORD IT SAYS IS A PROP. There is no string in this file that a reader
// can see, for the reason every other behaviour file in this system has none:
// a sentence here is a sentence in one language.

/** How far one wheel notch moves the camera. */
const WHEEL = 0.0015;
/**
 * The one bound the zoom keeps: above zero.
 *
 * The camera used to be caged between 0.15× and 4× of the fit, and a reader
 * flying into a deep scene hit the bars. Travel is free in every direction
 * and every sense now — the zoom is multiplicative, so it approaches zero
 * and infinity without ever arriving. The floor is a thousandth because
 * `apply` writes the zoom to three decimals: anything smaller serialises as
 * `0`, and `scale3d(0)` does not draw a scene very far away, it erases it —
 * at a thousandth the whole drawing is already smaller than two pixels.
 * Double-tap, `0` or "fit the whole scene" remain the way home from
 * anywhere.
 */
const NEAREST_TO_NOTHING = 0.001;
/**
 * The magnification at which zooming in becomes FLYING in.
 *
 * `distance = lens / zoom` approaches the front plane for ever without
 * arriving — so the deeper planes, whose scale converges to `lens / depth`,
 * stopped growing and a reader pressing toward Fay and Dan hit an invisible
 * wall. Past this magnification each further notch moves the CAMERA forward
 * through the scene instead, without limit; what passes behind it is hidden
 * rather than left to CSS, which does not cull and would draw it as a
 * mirrored smear across everything. Zooming back out retraces the flight to
 * where it began, then hands back to magnification.
 */
const FLIGHT = 4;
/** How close, in fractions of the down-pitch, a thing may come to the camera
 * before it is behind it for drawing purposes. */
const BRUSH = 0.2;
/** How far a key press turns the camera, in degrees. */
const TURN = 6;
/** How far a dragged pixel turns it, in degrees. Mouse and finger alike. */
const SWEEP = 0.25;
/** The longest a touch can last and still be a tap, in milliseconds. */
const TAP = 300;
/**
 * How far a pressed pointer may wander, in pixels, and still be a click.
 *
 * A click is never perfectly still — a mouse settles by a pixel or two as the
 * button goes down — and without this a click in the air beside a figure was
 * a one-pixel orbit: the camera engaged, and engaging is a mode change a
 * reader can see. The camera moves only once the pointer has left this
 * circle; links, buttons and checkboxes inside the scene were never at stake,
 * because a press on them is theirs before any of this runs.
 */
const STILL = 4;
/** What a scene is, once this has looked at it. */
const scenes = new WeakMap();

/**
 * Attaches the camera to every scene that has not got one.
 *
 * Idempotent, like every other `enhance*` in this system: it is called again
 * after anything that may have added a scene, and a scene it has already seen
 * is skipped rather than doubled.
 */
export function enhanceLineages(root = document) {
  for (const field of root.querySelectorAll("[data-nd-lineage] [data-nd-lineage-field]")) {
    if (scenes.has(field)) continue;
    const scene = build(field, new AbortController());
    if (scene) scenes.set(field, scene);
  }
}

/**
 * Takes the camera back off every scene under `root`.
 *
 * A BEHAVIOUR THAT CANNOT BE RELEASED IS A DEFECT FOR EVERY CONSUMER WITH A
 * CLIENT-SIDE ROUTER. Two of the things `listen` registers reach past the
 * scene: a `resize` on the WINDOW, and a `MutationObserver` on the scene's own
 * root. Both outlive a scene taken out of the document, and the resize handler
 * then projects a three.js camera into a field that is not on the screen.
 */
export function releaseLineages(root = document) {
  for (const field of root.querySelectorAll("[data-nd-lineage] [data-nd-lineage-field]")) {
    const scene = scenes.get(field);
    if (!scene) continue;
    scene.binding.abort();
    scenes.delete(field);
  }
}

function build(field, binding) {
  const root = field.closest("[data-nd-lineage]");
  const space = field.querySelector("[data-nd-lineage-space]");
  if (!root || !space) return null;

  const scene = {
    root,
    field,
    space,
    /** The camera's own state, and the field's, kept apart on purpose. */
    words: {
      camera: root.dataset.ndCameraLabel ?? "",
      zoomIn: root.dataset.ndZoomInLabel ?? "",
      zoomOut: root.dataset.ndZoomOutLabel ?? "",
      fit: root.dataset.ndFitLabel ?? "",
      matches: root.dataset.ndMatchesLabel ?? "",
    },
    // The resting camera, in the same three numbers the stylesheet uses, so
    // there is one camera rather than two that have to agree.
    yaw: degrees(root, "--nd-yaw"),
    pitch: degrees(root, "--nd-pitch"),
    zoom: 1,
    panX: 0,
    panY: 0,
    // Where the camera has FLOWN to, in scene pixels — see `zoom`. The
    // magnification approaches the front plane for ever without arriving,
    // so past `FLIGHT` the wheel travels instead, and this is the distance
    // covered.
    fly: { x: 0, y: 0, z: 0 },
    rest: null,
    three: null,
    // THE STYLESHEET'S PERSPECTIVE, READ ONCE AND NEVER WRITTEN BACK. The
    // camera is built to match the picture that is already on the screen
    // rather than the other way round: a focal length of its own would change
    // how deep the scene looks the instant a pointer crossed it, which is a
    // scene that reacts to being looked at. It is a length in CSS pixels and
    // the fitted cell decides it, so it is re-read on every frame.
    lens: 0,
    pieces: [...space.querySelectorAll(".nd-lineage-piece")],
    at: 0,
    // CARRIED ON THE SCENE, because every function below already takes it. A
    // parallel argument threaded through five call sites is five places for
    // one of them to be forgotten.
    binding,
    signal: binding.signal,
  };
  scene.rest = { yaw: scene.yaw, pitch: scene.pitch, zoom: 1, panX: 0, panY: 0 };

  walkable(scene);
  controls(scene);
  listen(scene);
  announce(scene);
  root.dataset.ndCamera = "css";
  return scene;
}

/**
 * The two pitches in CSS pixels — the units a pan and a reveal are counted in.
 *
 * Two, because the across-pitch may be squeezed to fit and the down-pitch
 * never is. Readable at all only because the stylesheet registers both as
 * lengths; unregistered they would answer with their own source text.
 */
function cellsOf(scene) {
  const style = getComputedStyle(scene.field);
  const x = Number.parseFloat(style.getPropertyValue("--nd-cellx"));
  const y = Number.parseFloat(style.getPropertyValue("--nd-celly"));
  return {
    x: Number.isFinite(x) && x > 0 ? x : 1,
    y: Number.isFinite(y) && y > 0 ? y : 1,
  };
}

/** A camera angle the stylesheet already decided, in degrees. */
function degrees(root, name) {
  const value = getComputedStyle(root).getPropertyValue(name).trim();
  return Number.parseFloat(value) || 0;
}

// --- The keyboard walk -------------------------------------------------------

/**
 * ONE TAB STOP FOR THE WHOLE SCENE, and the arrows walk it.
 *
 * Before this runs, every connection a figure has is a real link and a real tab
 * stop — which is correct with nothing running and is forty presses to cross a
 * family. The script takes them out of the tab order and answers the arrows
 * instead: the roving model `Toolbar` and `Tree` already use.
 */
function walkable(scene) {
  for (const piece of scene.pieces) {
    piece.tabIndex = -1;
    for (const tie of piece.querySelectorAll(".nd-lineage-tie")) tie.tabIndex = -1;
  }
}

/** The pieces this one is joined to, in the direction asked for. */
function neighbours(scene, piece, upward) {
  const out = [];
  for (const tie of piece.querySelectorAll(".nd-lineage-tie")) {
    if ((tie.dataset.ndUp === "true") !== upward) continue;
    // The tie's href IS the piece's id — both are `nd-{name}-{id}`, written by
    // one emitter — so there is nothing to parse here and nothing to escape.
    const other = document.getElementById(tie.hash.slice(1));
    if (other && other.classList.contains("nd-lineage-piece")) out.push(other);
  }
  return out;
}

/** A row-mate: the next figure along, by where it actually is. */
function along(scene, piece, step) {
  const y = Number.parseFloat(piece.style.getPropertyValue("--nd-y")) || 0;
  const x = Number.parseFloat(piece.style.getPropertyValue("--nd-x")) || 0;
  const row = scene.pieces
    .filter((other) => (Number.parseFloat(other.style.getPropertyValue("--nd-y")) || 0) === y)
    .sort(
      (a, b) =>
        (Number.parseFloat(a.style.getPropertyValue("--nd-x")) || 0) -
        (Number.parseFloat(b.style.getPropertyValue("--nd-x")) || 0),
    );
  const at = row.findIndex(
    (other) => other === piece || (Number.parseFloat(other.style.getPropertyValue("--nd-x")) || 0) === x,
  );
  return row[at + step] ?? null;
}

/**
 * The keyboard walk's step, and the camera follows it.
 *
 * `preventScroll` is not a nicety here. The field does not scroll — it clips —
 * so the browser's own "reveal the focused thing" would walk up and scroll the
 * PAGE instead, and a reader pressing an arrow inside a scene would watch the
 * document move under them. Panning is how this scene reveals anything, so
 * that is what it does: only when the figure is actually outside the window,
 * and only by as much as it takes to bring it in.
 */
function focus(scene, piece) {
  if (!piece) return;
  scene.at = scene.pieces.indexOf(piece);
  piece.focus({ preventScroll: true });
  reveal(scene, piece);
}

/** Pan until a figure is inside the window, and no further. */
function reveal(scene, piece) {
  const box = scene.field.getBoundingClientRect();
  const it = piece.getBoundingClientRect();
  // A cell in pixels, which is what the pan is counted in.
  const cell = cellsOf(scene);
  const edge = cell.y / 2;
  const dx = Math.min(0, box.right - edge - it.right) + Math.max(0, box.left + edge - it.left);
  const dy = Math.min(0, box.bottom - edge - it.bottom) + Math.max(0, box.top + edge - it.top);
  if (dx !== 0 || dy !== 0) pan(scene, dx / cell.x, dy / cell.y);
}

// --- The controls ------------------------------------------------------------

/**
 * Three buttons, and their names come from the contract.
 *
 * The slot they sit in is reserved by the stylesheet before this runs, so
 * nothing on the page moves when the module arrives — the lesson the palette
 * trigger cost this site a whole stage.
 */
function controls(scene) {
  if (!scene.words.camera) return;
  const bar = document.createElement("div");
  bar.className = "nd-lineage-camera";
  bar.setAttribute("role", "group");
  bar.setAttribute("aria-label", scene.words.camera);
  for (const [word, act] of [
    [scene.words.zoomOut, () => zoom(scene, 1 / 1.3)],
    [scene.words.zoomIn, () => zoom(scene, 1.3)],
    [scene.words.fit, () => fit(scene)],
  ]) {
    if (!word) continue;
    const button = document.createElement("button");
    button.type = "button";
    button.className = "nd-lineage-camera-key";
    // ITS NAME IS ITS TEXT. A script in this system may not draw — a drawing is
    // declared in `spec/icons.toml` and rendered by `Icon` — so a control this
    // file creates says what it is in words. `§9.10` prefers icons in the
    // header; a camera control that had to invent one would be a control with
    // an empty box in it.
    button.textContent = word;
    button.addEventListener("click", act, { signal: scene.signal });
    bar.append(button);
  }
  scene.root.append(bar);
}

// --- The camera --------------------------------------------------------------

/**
 * Swaps the platform's scrolling for the camera, without moving a pixel.
 *
 * Until the first camera movement the field is an ordinary scroll container
 * and the scene is an ordinary centred box — which is what a reader with no
 * script has, and why mounting alone changes nothing they can see. The first
 * orbit, zoom or pan flips the field to `clip` and the scene to the camera's
 * own transform; whatever the reader had scrolled to is folded into the pan,
 * so the picture under their pointer does not move when it happens.
 */
function engage(scene) {
  if (scene.engaged) return;
  scene.engaged = true;
  const field = scene.field;
  const cell = cellsOf(scene);
  const acrossOverflow = Math.max(0, field.scrollWidth - field.clientWidth);
  const downOverflow = Math.max(0, field.scrollHeight - field.clientHeight);
  scene.panX += (acrossOverflow / 2 - field.scrollLeft) / cell.x;
  scene.panY += (downOverflow / 2 - field.scrollTop) / cell.y;
  scene.root.dataset.ndCamera = "three";
}

function apply(scene) {
  const { root } = scene;
  // The walls of every figure are only drawn once there is an angle to see
  // them from — see the stylesheet, and the compositor that stopped answering.
  if (scene.three) engage(scene);
  root.toggleAttribute("data-nd-turned", scene.yaw !== 0 || scene.pitch !== 0);
  root.style.setProperty("--nd-yaw", `${round(scene.yaw)}deg`);
  root.style.setProperty("--nd-pitch", `${round(scene.pitch)}deg`);
  root.style.setProperty("--nd-zoom", `${round(scene.zoom)}`);
  root.style.setProperty("--nd-pan-x", `${round(scene.panX)}`);
  root.style.setProperty("--nd-pan-y", `${round(scene.panY)}`);
  if (scene.three) project(scene);
}

function round(value) {
  return Math.round(value * 1000) / 1000;
}

function orbit(scene, dyaw, dpitch) {
  scene.yaw += dyaw;
  // Bounded, because past a right angle a reader is looking at the scene edge
  // on and cannot get back without knowing which way they turned.
  scene.pitch = Math.max(-80, Math.min(80, scene.pitch + dpitch));
  apply(scene);
}

function zoom(scene, factor) {
  const flying = scene.three && scene.engaged;
  if (flying && factor > 1 && scene.zoom * factor > FLIGHT) {
    // Forward, through the scene. The step is a fifth of the camera's own
    // distance per 1.0 of factor — a steady stride once the magnification
    // has stopped at `FLIGHT`.
    scene.zoom = Math.min(scene.zoom * factor, FLIGHT);
    travel(scene, (factor - 1) * (scene.lens / scene.zoom));
    apply(scene);
    return;
  }
  if (flying && factor < 1 && length(scene.fly) > 0) {
    // Backward first: the flight is retraced to where it began, and only
    // then does the picture start to shrink.
    const step = (1 / factor - 1) * (scene.lens / scene.zoom);
    if (length(scene.fly) <= step) scene.fly = { x: 0, y: 0, z: 0 };
    else travel(scene, -step);
    apply(scene);
    return;
  }
  scene.zoom = Math.max(NEAREST_TO_NOTHING, scene.zoom * factor);
  apply(scene);
}

function length(fly) {
  return Math.hypot(fly.x, fly.y, fly.z);
}

/** Moves the flight target along the camera's own view direction. */
function travel(scene, step) {
  const yaw = (-scene.yaw * Math.PI) / 180;
  const pitch = (-scene.pitch * Math.PI) / 180;
  // The camera stands at `target + distance * offset`, so forward is the
  // negative of the offset the projection uses.
  scene.fly.x -= step * Math.sin(yaw) * Math.cos(pitch);
  scene.fly.y -= step * Math.sin(pitch);
  scene.fly.z -= step * Math.cos(yaw) * Math.cos(pitch);
}

function pan(scene, dx, dy) {
  scene.panX += dx;
  scene.panY += dy;
  apply(scene);
}

/**
 * The whole scene on the screen, which is the way out of anywhere.
 *
 * The stylesheet already fits the scene to its window with no script at all, so
 * at rest this is a reset: it puts the camera back where it started. Pressed
 * again — with the camera already at rest — it fits whatever is actually in the
 * box, which is the answer when a reader has turned the scene and the corner
 * they turned toward themselves no longer clears the edge.
 */
function fit(scene) {
  const was = { yaw: scene.yaw, pitch: scene.pitch, zoom: scene.zoom };
  Object.assign(scene, scene.rest);
  scene.fly = { x: 0, y: 0, z: 0 };
  const box = scene.field.getBoundingClientRect();
  const wide = scene.space.offsetWidth;
  const tall = scene.space.offsetHeight;
  const room = Math.min(box.width / Math.max(1, wide), box.height / Math.max(1, tall));
  const same = was.yaw === scene.yaw && was.pitch === scene.pitch && was.zoom === scene.zoom;
  scene.zoom = same || room >= 1 ? Math.min(1, room) : 1;
  apply(scene);
}

function motionless() {
  return window.matchMedia("(prefers-reduced-motion: reduce)").matches;
}

// --- three.js ----------------------------------------------------------------

/**
 * Hands the camera to three.js, once.
 *
 * # Why the renderer's own element is not in the page
 *
 * `CSS3DRenderer` builds a DOM of its own and sizes it with `setSize`. Dropped
 * into the field — which is a grid container that centres what is in it and
 * scrolls what does not fit — that sized element became the thing the field
 * measured itself against, and the frame around the example grew and shrank as
 * a reader moved the pointer over it. So what is used here is the part of
 * `CSS3DRenderer` that matters: **the camera matrix, composed exactly as it
 * composes one**, written onto the scene that is already there.
 *
 * # And the camera replaces the scrollbars rather than fighting them
 *
 * With no script the field is an ordinary scroll container and that is how a
 * reader reaches the far side of a family. Once the camera exists, panning is
 * how they do it — so the scene stops contributing to the scroll size, which is
 * also what makes the handover cost no layout at all.
 */
async function mount(scene) {
  if (scene.three || scene.mounting) return;
  scene.mounting = true;
  try {
    const three = await import("./vendor/three-css3d.js");
    const camera = new three.PerspectiveCamera(45, 1, 1, 100000);
    scene.three = { three, camera };
  } catch {
    // A blocked import, an offline reader, a consumer who copied `dist/` and
    // left the vendored directory behind. The scene keeps the camera the
    // stylesheet gives it, which is the one already on the screen.
    scene.root.dataset.ndCamera = "css";
  } finally {
    scene.mounting = false;
  }
}

/**
 * One frame.
 *
 * The composition is `CSS3DRenderer`'s: a perspective equal to the camera's
 * own focal length in pixels, `translateZ` by the same, the camera's inverse
 * world matrix with its second row negated — CSS counts `y` downward and three
 * counts it up — and the object's own `translate(-50%, -50%)`.
 */
function project(scene) {
  const { three, camera } = scene.three;
  const field = scene.field.getBoundingClientRect();
  const height = Math.max(1, field.height);
  // THE FIELD OF VIEW COMES FROM THE PERSPECTIVE, not the perspective from the
  // field of view. `perspective: 600px` on a box says the eye is 600 pixels in
  // front of it, and a camera standing at the same distance sees the same
  // picture only if its frustum is the same shape — which fixes the angle at
  // `2 * atan(h / 2d)`. Guess it instead and the handover is a jump in depth.
  scene.lens = Math.max(1, Number.parseFloat(getComputedStyle(scene.field).perspective) || 0);
  camera.fov = three.MathUtils.radToDeg(2 * Math.atan(height / (2 * scene.lens)));

  // THE SCENE'S CENTRE IS THE WORLD ORIGIN, because the chain below ends in
  // the object's own `translate(-50%, -50%)` — so the camera looks at zero, and
  // panning moves what it looks at rather than moving the scene.
  //
  // IN CELLS, which is what the stylesheet counts a pan in. The two halves have
  // to agree on the unit or the same drag moves the picture one cell before
  // three.js arrives and the width of a whole family afterwards.
  //
  // DIVIDED BY THE ZOOM, because a pan is a promise in SCREEN pixels — the
  // scene follows the pointer one for one — and the camera magnifies scene
  // units by the zoom on their way to the screen. The stylesheet keeps the
  // same promise the other way round, by putting its pan translate outside
  // its scale.
  const cell = cellsOf(scene);
  const wane = Math.max(NEAREST_TO_NOTHING, scene.zoom);
  const target = new three.Vector3(
    (-scene.panX * cell.x) / wane + scene.fly.x,
    (scene.panY * cell.y) / wane + scene.fly.y,
    scene.fly.z,
  );
  // THE HANDOVER IS INVISIBLE, and this line is why: standing the camera at
  // exactly the distance the stylesheet's perspective puts the eye at makes the
  // projected scale one, which is the scale already on the screen. A reader who
  // moves a pointer over a scene should not see it resize — that is `zoom`.
  const lens = scene.lens;
  const distance = lens / wane;
  // BOTH ANGLES ARE NEGATED, and neither is a guess.
  //
  // `--nd-yaw` and `--nd-pitch` are the angles of a CSS `rotateY`/`rotateX` on
  // the SCENE. Turning a scene one way is standing to the other side of it, so
  // a camera reproducing them orbits by the opposite of each — and CSS counts
  // `y` downward, which is the second flip and cancels nothing here because the
  // camera is placed in three's frame and the scene is turned in CSS's.
  //
  // Checked rather than reasoned into place: the composed matrix was compared
  // with the stylesheet's own at three unrelated pairs of angles, and this is
  // the one combination of the four that reproduces it to three decimals.
  const yaw = three.MathUtils.degToRad(-scene.yaw);
  const pitch = three.MathUtils.degToRad(-scene.pitch);
  camera.aspect = Math.max(1, field.width) / height;
  camera.position.set(
    target.x + distance * Math.sin(yaw) * Math.cos(pitch),
    target.y + distance * Math.sin(pitch),
    target.z + distance * Math.cos(yaw) * Math.cos(pitch),
  );
  camera.up.set(0, 1, 0);
  camera.lookAt(target);
  camera.updateMatrixWorld();
  camera.updateProjectionMatrix();

  // THE TWO FLIPS, AND THE CENTRING BETWEEN THEM CARRIES A PLUS.
  //
  // `CSS3DRenderer` negates `y` twice — once in the camera matrix, because CSS
  // counts it downward and three counts it up, and once again in every object's
  // own matrix — and the second one is not optional: compose only the first and
  // the scene renders upside down.
  //
  // The sign of the centring translate falls out of the same algebra rather
  // than out of taste. What has to be drawn is `S·M·S·T`: flip the element's
  // coordinates into three's frame, view them, flip back. Moving the flip past
  // the centring translate turns `S·T(-w/2, -h/2)` into `T(-w/2, +h/2)·S`, and
  // that plus is the whole difference between a scene that stays where it was
  // when the camera arrives and one that drops its own height down the field —
  // which is what a reader saw as the example jumping under the pointer.
  scene.space.style.transform =
    `translateZ(${round(lens)}px)${cameraMatrix(camera.matrixWorldInverse)}` +
    "translate(-50%, 50%)matrix3d(1,0,0,0,0,-1,0,0,0,0,1,0,0,0,0,1)";

  occlude(scene, camera, cell);
}

/**
 * Hides whatever the flight has put behind the camera.
 *
 * CSS draws no frustum: an element behind the eye is not skipped, it is
 * smeared across the screen mirrored and enormous. So each piece, run and
 * label is measured against the camera every frame — its scene position
 * against the view direction — and hidden once it comes within `BRUSH` of
 * the camera's plane. Nothing is hidden until the reader has actually flown
 * (the camera outside the scene can put nothing behind itself), so a scene
 * that was never entered is never touched.
 */
function occlude(scene, camera, cell) {
  if (scene.solids === undefined) {
    scene.solids = [
      ...scene.root.querySelectorAll(
        ".nd-lineage-piece, .nd-lineage-space > .nd-lineage-figure, .nd-lineage-run, .nd-lineage-tag",
      ),
    ]
      .map((el) => {
        // COMPUTED, not inline: a figure standing at a named point carries
        // `--nd-x: var(--nd-at-ada-x)`, and only the computed value has done
        // the substitution.
        const style = getComputedStyle(el);
        const x = Number.parseFloat(style.getPropertyValue("--nd-x"));
        const y = Number.parseFloat(style.getPropertyValue("--nd-y"));
        const z = Number.parseFloat(style.getPropertyValue("--nd-z")) || 0;
        if (!Number.isFinite(x) || !Number.isFinite(y)) return null;
        const len = Number.parseFloat(style.getPropertyValue("--nd-len")) || 0;
        return { el, x, y, z, len, axis: el.dataset.ndAxis ?? "", shown: true };
      })
      .filter(Boolean);
    scene.reach = {
      w: Number.parseFloat(getComputedStyle(scene.root).getPropertyValue("--nd-w")) || 0,
      h: Number.parseFloat(getComputedStyle(scene.root).getPropertyValue("--nd-h")) || 0,
    };
  }
  const flown = length(scene.fly) > 0;
  const eye = camera.position;
  const forward = new scene.three.three.Vector3();
  camera.getWorldDirection(forward);
  const near = cell.y * BRUSH;
  const world = (s, along) => {
    let { x, y, z } = s;
    if (along && s.axis === "x") x += s.len;
    if (along && s.axis === "y") y += s.len;
    if (along && s.axis === "z") z += s.len;
    return {
      x: cell.x * (x - scene.reach.w / 2),
      y: cell.y * (scene.reach.h / 2 - y),
      z: -z * cell.y,
    };
  };
  const depth = (p) =>
    (p.x - eye.x) * forward.x + (p.y - eye.y) * forward.y + (p.z - eye.z) * forward.z;
  for (const solid of scene.solids) {
    const behind =
      flown &&
      (depth(world(solid, false)) < near ||
        (solid.len > 0 && depth(world(solid, true)) < near));
    if (behind === solid.shown) {
      solid.shown = !behind;
      solid.el.style.visibility = behind ? "hidden" : "";
    }
  }
}

/** `CSS3DRenderer`'s camera matrix: the inverse world matrix, `y` flipped. */
function cameraMatrix(matrix) {
  const e = matrix.elements;
  const n = [
    e[0], -e[1], e[2], e[3],
    e[4], -e[5], e[6], e[7],
    e[8], -e[9], e[10], e[11],
    e[12], -e[13], e[14], e[15],
  ].map((value) => round(value));
  return `matrix3d(${n.join(",")})`;
}

// --- What a reader does ------------------------------------------------------

function listen(scene) {
  const { field } = scene;
  let dragging = null;
  const touches = new Map();
  let pair = null;
  let tapped = 0;

  const wake = () => mount(scene);
  /** Whether this event belongs to something a reader can operate instead. */
  const theirs = (event) => event.target.closest("a, button, label, input, summary");

  // A DRAG IS NOT A MENU, AND IT IS NOT A SELECTION.
  //
  // Dragging a scene with a mouse selected the text in every card it passed
  // over and finished by opening the browser's context menu — a reader turning
  // a family tree got a highlighted family tree and a list of things to do with
  // an image. Both are the platform being helpful about a gesture that is not
  // the platform's, so both are refused HERE, on the field, and nowhere else on
  // the page: a link inside a figure keeps its own menu.
  field.addEventListener("contextmenu", (event) => {
    if (theirs(event)) return;
    event.preventDefault();
  }, { signal: scene.signal });

  field.addEventListener("pointerdown", (event) => {
    // ANY press is an act: the module mounts even when the press belongs to a
    // link or a checkbox inside the scene, so the camera is ready by the time
    // the reader tries to turn it.
    wake();
    // A drag that started on something a reader can operate is that thing's.
    if (theirs(event)) return;
    if (event.pointerType === "touch") {
      touches.set(event.pointerId, point(event));
      if (touches.size === 2) pair = { ...between(touches), cell: cellsOf(scene) };
      capture(field, event.pointerId);
      wake();
      return;
    }
    // `preventDefault` is what stops the selection starting; the attribute is
    // what stops one that was already there from being dragged along.
    event.preventDefault();
    dragging = {
      x: event.clientX,
      y: event.clientY,
      // Where the press was, and whether it has left the `STILL` circle: a
      // press that never does is a click, and a click moves nothing.
      sx: event.clientX,
      sy: event.clientY,
      live: false,
      // THE LEFT BUTTON PANS, which the reader asked for in as many words:
      // moving the picture is the gesture a reader wants first and most, so
      // it belongs on the button that needs no knowledge. Turning is on the
      // right and middle buttons and on shift — the same drag, one intent
      // away.
      pan: event.button === 0 && !event.shiftKey,
      // Measured once per drag: a `getComputedStyle` on every pointer move is
      // a forced style recalculation on every pointer move.
      cell: cellsOf(scene),
    };
    scene.root.dataset.ndDragging = "";
    capture(field, event.pointerId);
    wake();
  }, { signal: scene.signal });

  field.addEventListener("pointermove", (event) => {
    if (scene.root.dataset.ndParallax === "true" && !motionless()) {
      const box = field.getBoundingClientRect();
      scene.root.style.setProperty("--nd-pointer-x", round((event.clientX - box.left) / box.width - 0.5));
      scene.root.style.setProperty("--nd-pointer-y", round((event.clientY - box.top) / box.height - 0.5));
    }
    if (touches.has(event.pointerId)) {
      const was = touches.get(event.pointerId);
      const now = point(event);
      now.moved = was.moved || Math.hypot(now.x - was.x, now.y - was.y) > STILL;
      now.at = was.at;
      touches.set(event.pointerId, now);
      // ONE FINGER TURNS THE SCENE, which is the gesture a reader tries first
      // and the one that was missing: a touch screen could pinch and nothing
      // else, so a scene on a phone had a zoom and no camera.
      // `was.moved`, so the frame that leaves the `STILL` circle does not fold
      // the wander into the first turn — the same rule the mouse drag applies.
      if (touches.size === 1) {
        if (was.moved) orbit(scene, (now.x - was.x) * SWEEP, -(now.y - was.y) * SWEEP);
        return;
      }
      // TWO FINGERS DO THE OTHER THREE AT ONCE, because that is how they are
      // measured: apart is the zoom, the middle is the pan, the angle is the
      // turn. Doing them together also means a reader never has to make one of
      // them cleanly — a sloppy pinch still zooms, and it also drifts and
      // turns by however much it drifted and turned.
      if (touches.size === 2 && pair) {
        const it = between(touches);
        if (pair.spread > 0) zoom(scene, it.spread / pair.spread);
        pan(scene, (it.x - pair.x) / pair.cell.x, (it.y - pair.y) / pair.cell.y);
        orbit(scene, turn(it.angle - pair.angle), 0);
        pair = { ...it, cell: pair.cell };
      }
      return;
    }
    if (!dragging) return;
    if (!dragging.live) {
      if (Math.hypot(event.clientX - dragging.sx, event.clientY - dragging.sy) <= STILL) return;
      // The drag starts HERE, not at the press: folding the wander into the
      // first frame would jump the camera by the circle just crossed.
      dragging.live = true;
      dragging.x = event.clientX;
      dragging.y = event.clientY;
      return;
    }
    const dx = event.clientX - dragging.x;
    const dy = event.clientY - dragging.y;
    dragging.x = event.clientX;
    dragging.y = event.clientY;
    // THE SCENE FOLLOWS THE POINTER, one pixel for one pixel, which is the only
    // pan a reader can aim. A fixed divisor moved it a different distance at
    // every scale — and after the fit there are many more scales than three.
    if (dragging.pan) pan(scene, dx / dragging.cell.x, dy / dragging.cell.y);
    else orbit(scene, dx * SWEEP, -dy * SWEEP);
  }, { signal: scene.signal });

  const release = (event) => {
    if (touches.has(event.pointerId)) {
      const was = touches.get(event.pointerId);
      touches.delete(event.pointerId);
      if (touches.size < 2) pair = null;
      // A DOUBLE TAP FITS, which is the way out of a camera on a screen with no
      // keyboard and no `0` on it. A tap is short and still; two of them close
      // together are a request.
      if (!was.moved && event.timeStamp - was.at < TAP) {
        if (event.timeStamp - tapped < TAP * 2) {
          tapped = 0;
          fit(scene);
        } else {
          tapped = event.timeStamp;
        }
      }
      return;
    }
    dragging = null;
    delete scene.root.dataset.ndDragging;
  };
  field.addEventListener("pointerup", release, { signal: scene.signal });
  field.addEventListener("pointercancel", release, { signal: scene.signal });
  field.addEventListener("pointerleave", () => {
    scene.root.style.setProperty("--nd-pointer-x", "0");
    scene.root.style.setProperty("--nd-pointer-y", "0");
  }, { signal: scene.signal });

  // THE WHEEL ZOOMS AN ENGAGED SCENE, AND ONLY AN ENGAGED ONE. Before the
  // reader has taken the camera, a wheel over a scene still scrolls the page —
  // taking every wheel event would mean a reader who scrolled onto a scene
  // cannot scroll off it, over every scene on the page. Once they have
  // engaged it, the scene is theirs and the wheel is the zoom, which is what
  // every drawing tool does with it. `ctrl` (and a trackpad pinch, which
  // arrives as exactly that) zooms at any time.
  field.addEventListener(
    "wheel",
    (event) => {
      if (!scene.engaged && !event.ctrlKey && !event.metaKey) return;
      event.preventDefault();
      zoom(scene, 1 - event.deltaY * WHEEL);
      wake();
    },
    { passive: false, signal: scene.signal },
  );

  // A STATE NOTHING WRITES IS A STATE THAT DOES NOTHING. `:checked` is what
  // draws a picked connection heavier, with no script and with one — but a
  // pseudo-class is not an attribute, so a consumer styling or querying
  // `data-nd-picked` would find nothing. The script writes it, and the contract
  // says that is what it is.
  field.addEventListener("change", (event) => {
    const box = event.target.closest(".nd-lineage-pick");
    if (!box) return;
    const owner = box.closest(".nd-lineage-bond, .nd-lineage-piece");
    if (owner) owner.toggleAttribute("data-nd-picked", box.checked);
  }, { signal: scene.signal });

  // ON AN ACT, NEVER ON A GLANCE. Mounting on `pointerenter` meant hovering
  // an example changed it, which was reported twice. A press, a key or focus
  // is a reader engaging; passing a pointer over the page is not.
  field.addEventListener("keydown", (event) => onKey(scene, event, wake), { signal: scene.signal });
  field.addEventListener("focusin", wake, { once: true, signal: scene.signal });

  for (const switcher of scene.root.querySelectorAll("[data-nd-swaps]")) {
    switcher.addEventListener("click", () => swap(scene, switcher.dataset.ndSwaps ?? ""), { signal: scene.signal });
  }

  // A CONSUMER SETS THE FILTER; THIS ANSWERS IT. There is no filter control
  // here because a scene does not own the box somebody types in — it owns what
  // the answer looks like.
  const watch = new MutationObserver(() => announce(scene));
  watch.observe(scene.root, {
    attributes: true,
    attributeFilter: ["data-nd-filter", "data-nd-sets", "data-nd-occasion"],
  });
  scene.signal.addEventListener("abort", () => watch.disconnect());

  // ON THE WINDOW, so it outlives the scene unless something takes it off.
  window.addEventListener("resize", () => scene.three && project(scene), {
    passive: true,
    signal: scene.signal,
  });
}

/**
 * What two fingers are doing, as one reading.
 *
 * Their distance apart is the zoom, the point between them is the pan, and the
 * angle of the line joining them is the turn — three gestures out of one pair
 * of numbers, which is why they are measured together rather than one at a
 * time. `atan2` is fine here and deliberately absent from the layout engine:
 * this runs in one language, and the engine has to agree with Rust to the last
 * digit.
 */
function between(touches) {
  const [a, b] = [...touches.values()];
  return {
    spread: Math.hypot(a.x - b.x, a.y - b.y),
    angle: (Math.atan2(b.y - a.y, b.x - a.x) * 180) / Math.PI,
    x: (a.x + b.x) / 2,
    y: (a.y + b.y) / 2,
  };
}

/**
 * Takes the pointer, and does not care if it cannot.
 *
 * `setPointerCapture` throws when the id is not an active pointer — a touch
 * that ended between the event and the handler, or a synthetic event in a test.
 * It is an optimisation here rather than a requirement: without it a drag that
 * leaves the field stops, which is a worse camera and not a broken one. Letting
 * it throw took the whole handler with it, `wake()` included, so the camera
 * never mounted at all.
 */
function capture(field, id) {
  try {
    field.setPointerCapture(id);
  } catch {
    // A pointer that is already gone. There is nothing to hold.
  }
}

/** A pointer, reduced to what a gesture needs to remember about it. */
function point(event) {
  return { x: event.clientX, y: event.clientY, at: event.timeStamp, moved: false };
}

/**
 * A twist, in degrees, and only past the wobble.
 *
 * Two fingers on a screen are never exactly still, so the angle between them
 * changes by a degree or two on every frame of a pinch that was meant to be a
 * pinch. Without a threshold every zoom is also a slow turn.
 */
function turn(delta) {
  const wrapped = ((delta + 540) % 360) - 180;
  return Math.abs(wrapped) < 3 ? 0 : wrapped;
}

function onKey(scene, event, wake) {
  if (event.isComposing) return;
  const here = scene.pieces[scene.at] ?? scene.pieces[0];
  const step = (list) => {
    if (list.length === 0) return false;
    focus(scene, list[0]);
    return true;
  };
  let handled = true;
  switch (event.key) {
    case "ArrowUp":
      handled = event.shiftKey ? (orbit(scene, 0, TURN), true) : step(neighbours(scene, here, true));
      break;
    case "ArrowDown":
      handled = event.shiftKey ? (orbit(scene, 0, -TURN), true) : step(neighbours(scene, here, false));
      break;
    case "ArrowLeft":
      handled = event.shiftKey ? (orbit(scene, -TURN, 0), true) : !!focusAlong(scene, here, -1);
      break;
    case "ArrowRight":
      handled = event.shiftKey ? (orbit(scene, TURN, 0), true) : !!focusAlong(scene, here, 1);
      break;
    case "+":
    case "=":
      zoom(scene, 1.3);
      break;
    case "-":
      zoom(scene, 1 / 1.3);
      break;
    case "0":
      fit(scene);
      break;
    case "Escape":
      clear(scene);
      break;
    case "Enter":
    case " ": {
      const box = here?.querySelector(".nd-lineage-pick");
      const link = here?.querySelector("a.nd-lineage-card");
      if (link && event.key === "Enter") link.click();
      else if (box) box.checked = !box.checked;
      else handled = false;
      break;
    }
    default:
      handled = false;
  }
  if (handled) {
    event.preventDefault();
    wake();
  }
}

function focusAlong(scene, here, step) {
  const next = along(scene, here, step);
  focus(scene, next);
  return next;
}

/** Everything a reader singled out, and the filter, back to nothing. */
function clear(scene) {
  for (const box of scene.space.querySelectorAll(".nd-lineage-pick")) {
    box.checked = false;
    box.closest(".nd-lineage-bond, .nd-lineage-piece")?.removeAttribute("data-nd-picked");
  }
  scene.root.removeAttribute("data-nd-filter");
  fit(scene);
  announce(scene);
}

/** Shows one set of a group and hides the others. */
function swap(scene, wanted) {
  const at = wanted.indexOf(":");
  if (at < 0) return;
  const group = wanted.slice(0, at);
  const sets = (scene.root.dataset.ndSets ?? "")
    .split(/\s+/u)
    .filter((entry) => entry);
  // PRESSED AGAIN, IT PUTS THE BRANCH AWAY. A switch that only ever switches
  // ON is a switch a reader cannot undo — they brought a branch in from
  // somewhere else and the only way back was to reload the page. Toggling is
  // also what makes it a switch rather than a button that fires once.
  const on = sets.includes(wanted);
  const rest = sets.filter((entry) => !entry.startsWith(`${group}:`));
  if (!on) rest.push(wanted);
  scene.root.dataset.ndSets = rest.join(" ");
  for (const thing of scene.root.querySelectorAll("[data-nd-set]")) {
    const set = thing.dataset.ndSet ?? "";
    if (!set.startsWith(`${group}:`)) continue;
    thing.hidden = on || set !== wanted;
  }
  scene.root.style.setProperty(
    `--nd-set-${group}-${wanted.slice(at + 1)}`,
    on ? "none" : "grid",
  );
  // And the control says which way it is, for a reader who cannot see the
  // branch appear and for one who can.
  for (const switcher of scene.root.querySelectorAll(`[data-nd-swaps="${wanted}"]`)) {
    switcher.setAttribute("aria-pressed", on ? "false" : "true");
  }
}

/** Says how many figures answer the filter, in the consumer's own words. */
function announce(scene) {
  const status = scene.root.querySelector("[data-nd-lineage-status]");
  if (!status) return;
  const filter = scene.root.dataset.ndFilter ?? "";
  if (!filter || !scene.words.matches) {
    status.textContent = "";
    return;
  }
  const count = scene.space.querySelectorAll('[data-nd-match="yes"][data-nd-piece="figure"]').length;
  status.textContent = plural(scene.words.matches, count);
}

/**
 * `one:… |other:…`, chosen by the locale's own rules.
 *
 * The same shape `Search` uses for the same reason: a language that agrees a
 * verb with a number cannot be served by a sentence assembled around a numeral.
 */
function plural(template, count) {
  const variants = new Map(
    template.split("|").map((part) => {
      const at = part.indexOf(":");
      return at < 0 ? ["other", part] : [part.slice(0, at), part.slice(at + 1)];
    }),
  );
  const rule = new Intl.PluralRules(document.documentElement.lang || "en").select(count);
  const chosen = variants.get(rule) ?? variants.get("other") ?? "";
  return chosen.replace("{count}", String(count));
}
