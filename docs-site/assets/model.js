// A stage for any binary glTF: the file and its parameters are the inputs.
//
// WHAT THIS IS. `[data-nd-model="<path>"]` on an element makes that element a
// viewer for the model at that path. Everything else is an optional
// `data-nd-model-*` parameter with a default chosen to carry MOST models
// well — this module is the seed of a future upload feature, where readers
// hand it arbitrary `.glb` files, so nothing in it may assume anything about
// what the model depicts. It sizes to whatever box its element is given,
// measures the model rather than trusting it, and plays whatever animation
// the file brought.
//
// | attribute             | default | meaning                                     |
// |-----------------------|---------|---------------------------------------------|
// | `data-nd-model`       | —       | the file, resolved against the site base    |
// | `data-nd-model-fit`   | `1.15`  | element-box fraction the performance fills; may pass 1 |
// | `data-nd-model-size`  | `1`     | presentation multiplier on the fitted size  |
// | `data-nd-model-clip`  | `0`     | animation index or name; `none` plays nothing |
// | `data-nd-model-gaze`  | `14`    | pointer-follow sweep, degrees; `0` disables |
// | `data-nd-model-fov`   | `30`    | camera field of view, degrees               |
// | `data-nd-model-ratio` | `4`     | rendered pixels per CSS pixel, at most      |
//
// LARGER THAN ITS BOX, AND NEVER CUT. The element's box is a WINDOW, not a
// cage: the canvas overdraws it on every side (`--nd-model-grow`, stamped
// for the stylesheet) by exactly the factor that lets the performance fill
// `fit` of the box — while the model's WORLD scale stays at the
// sphere-in-frustum maximum, so no orbit, gaze or pose can reach the
// canvas's real edge. `size` multiplies that presentation: `2` shows the
// same model at twice the fitted default, with proportionally more
// overdraw and the same guarantee. Clip the element only if you accept
// cut models. And
// inside a scene the canvas re-renders at the scene's own magnification, up
// to `ratio`, so closing the camera in sharpens the picture instead of
// enlarging its pixels.
//
// WITH NO SCRIPT THERE IS STILL A PICTURE. Whatever the element already
// contains marked `[data-nd-model-flat]` stays until the first frame is
// drawn, and stays forever when the module is blocked, WebGL refused, or the
// file missing.
//
// STANDING IN A `Lineage` SCENE, THE STAGE IS A BILLBOARD. The element
// counter-rotates the scene's camera (see the stylesheet) and this module
// turns the MODEL by the same angles, so orbiting the scene shows the
// model's actual side rather than a flattening plane. Outside a scene both
// angles are zero and nothing changes.

/** How much of the distance to the gaze target is covered per frame. */
const EASE = 0.12;

/** The stages this module has already built. */
const built = new WeakSet();

/**
 * Gives every `[data-nd-model]` element on the page its model.
 *
 * Idempotent, and lazy twice over: nothing is fetched until a stage is on
 * screen, and nothing is drawn while none is.
 */
export function enhanceModels(root = document) {
  const stages = [...root.querySelectorAll("[data-nd-model]")].filter(
    (stage) => !built.has(stage),
  );
  if (stages.length === 0) return;
  for (const stage of stages) built.add(stage);

  const watch = new IntersectionObserver((entries) => {
    for (const entry of entries) {
      if (!entry.isIntersecting) continue;
      watch.unobserve(entry.target);
      mount(entry.target).catch(() => {
        // A blocked module, no WebGL, a missing file. The flat content that
        // is already in the element stays, which is why it is there.
      });
    }
  });
  for (const stage of stages) watch.observe(stage);
}

/** A numeric parameter, with its default. */
function setting(stage, name, fallback) {
  const value = Number.parseFloat(stage.getAttribute(`data-nd-model-${name}`));
  return Number.isFinite(value) ? value : fallback;
}

async function mount(stage) {
  const base = document.documentElement.getAttribute("data-noctua-base") || "";
  const three = await import(`${base}assets/vendor/three-webgl.js`);

  // THE FILE FIRST, BEFORE ANY CANVAS EXISTS: a missing model throws here,
  // the caller swallows it, and the flat content keeps the stage.
  const loader = new three.GLTFLoader();
  const brought = await loader.loadAsync(`${base}${stage.dataset.ndModel}`);

  const fit = setting(stage, "fit", 1.15);
  const magnify = setting(stage, "size", 1);
  const gaze = (setting(stage, "gaze", 14) * Math.PI) / 180;
  const fov = setting(stage, "fov", 30);
  const ratio = setting(stage, "ratio", 4);

  const canvas = document.createElement("canvas");
  canvas.className = "nd-model";
  // It says nothing: the element around it carries whatever name the
  // consumer gave, and a canvas that also announced one would say it twice.
  canvas.setAttribute("aria-hidden", "true");

  const renderer = new three.WebGLRenderer({ canvas, alpha: true, antialias: true });
  renderer.setPixelRatio(Math.min(ratio, window.devicePixelRatio || 1));

  const scene = new three.Scene();
  const camera = new three.PerspectiveCamera(fov, 1, 0.1, 100);
  camera.position.set(0, 0, 7.4);

  // EVERY TEXTURE THE MODEL BROUGHT, at the hardware's best sampling —
  // whatever maps its materials happen to carry.
  const body = brought.scene;
  const sharp = renderer.capabilities.getMaxAnisotropy();
  body.traverse((node) => {
    if (!node.isMesh || !node.material) return;
    for (const material of Array.isArray(node.material) ? node.material : [node.material]) {
      for (const key of ["map", "normalMap", "roughnessMap", "metalnessMap", "aoMap", "emissiveMap", "specularColorMap", "specularIntensityMap"]) {
        if (material[key]) material[key].anisotropy = sharp;
      }
    }
  });

  // WHICH CLIP, BY INDEX OR BY NAME. `none` — or a file with no animations —
  // means the model stands still and is measured where it stands.
  const asked = stage.getAttribute("data-nd-model-clip") ?? "0";
  let clip = null;
  if (asked !== "none" && brought.animations.length > 0) {
    const index = Number.parseInt(asked, 10);
    clip = Number.isInteger(index)
      ? (brought.animations[index] ?? brought.animations[0])
      : (brought.animations.find((each) => each.name === asked) ?? brought.animations[0]);
  }
  const mixer = clip ? new three.AnimationMixer(body) : null;

  // THE BOUNDS ARE THE WHOLE PERFORMANCE, BY VERTEX, WITH THE SKIN APPLIED.
  // `expandByObject` reads geometry boxes, and a skinned mesh's geometry is
  // its bind pose — the bones deform it on the GPU, so a rest box loses
  // whatever the animation reaches for, and a walk cycle leaves its own box
  // entirely. `getVertexPosition` is the one reading that includes the
  // skinning; the union over sampled poses is an envelope nothing escapes.
  const box = new three.Box3();
  const swell = () => {
    body.updateMatrixWorld(true);
    const at = new three.Vector3();
    body.traverse((node) => {
      if (node.isSkinnedMesh) {
        const position = node.geometry.getAttribute("position");
        for (let i = 0; i < position.count; i += 1) {
          node.getVertexPosition(i, at);
          at.applyMatrix4(node.matrixWorld);
          box.expandByPoint(at);
        }
      } else if (node.isMesh) {
        box.expandByObject(node);
      }
    });
  };
  if (mixer) {
    mixer.clipAction(clip).play();
    // BY TIME, NOT BY COUNT: a fixed count of samples over a LONG clip can
    // step right over a fast wingbeat's extreme — thirty samples per second
    // of clip follows the motion at the motion's own scale, bounded so a
    // ten-minute clip cannot stall the mount.
    const steps = Math.min(240, Math.max(48, Math.ceil(clip.duration * 30)));
    for (let posed = 0; posed <= steps; posed += 1) {
      mixer.setTime((clip.duration * posed) / steps);
      swell();
    }
    mixer.setTime(0);
  } else {
    box.setFromObject(body);
  }
  const middle = new three.Vector3();
  box.getCenter(middle);
  const span = new three.Vector3();
  box.getSize(span);
  body.position.sub(middle);
  const held = new three.Group();
  held.add(body);
  scene.add(held);

  // A model's textures carry their own light; the lamps only lift them to
  // the page's brightness, with no colour of their own.
  scene.add(new three.AmbientLight(0xffffff, 1.6)); // allow-literal: white light, not a palette colour
  const key = new three.DirectionalLight(0xffffff, 1.9); // allow-literal: likewise
  key.position.set(2.5, 3, 4);
  scene.add(key);

  stage.append(canvas);
  const still = window.matchMedia("(prefers-reduced-motion: reduce)");

  // THE GUARANTEE AND THE SIZE ARE TWO SEPARATE DIALS, because the model
  // TURNS. The scene's orbit and the gaze rotate it to any orientation, and
  // a box fitted at rest is exceeded the moment a long axis swings toward
  // the camera — a wing seen at an angle comes nearer, grows in projection,
  // and leaves the frame, which a reader photographed twice. So the WORLD
  // scale holds the exact sphere-in-frustum maximum for the pose envelope's
  // bounding sphere — a frustum plane at half-angle `tan α = view·margin/d`
  // clears a centred sphere of scaled radius `s·r` precisely when
  // `s·r ≤ d·sin α` — which no orientation can break, perspective included.
  // Fitting the sphere alone then made the model SMALL, which the same
  // reader reported next: a sphere is larger than any single pose. The size
  // therefore comes from the CANVAS — it overdraws the element by exactly
  // the factor that shows the unrotated envelope at `fit` of the element's
  // own box, so the picture is as large as the axis fit ever was while its
  // turned extremes land on transparent overdraw, never on an edge.
  const radius = Math.hypot(span.x, span.y, span.z) / 2 || 1;
  const size = () => {
    // Layout boxes, not client rects: a scene's zoom is a transform, and a
    // transformed rect would scale the buffer by the zoom a second time —
    // the resolution dial below already follows the zoom once.
    const wide = Math.max(1, stage.offsetWidth);
    const tall = Math.max(1, stage.offsetHeight);
    const aspect = wide / tall;
    const d = camera.position.z;
    const half = Math.tan((camera.fov * Math.PI) / 360) * d;
    const room = (view) => {
      const spread = view * 0.97;
      return (spread * d) / (radius * Math.hypot(d, spread));
    };
    const scale = Math.min(room(half), room(half * aspect));
    held.scale.setScalar(scale);
    const want =
      magnify *
      Math.min((2 * half * aspect * fit) / (span.x || 1), (2 * half * fit) / (span.y || 1));
    // Overdraw is bounded: a needle-shaped envelope could ask for a wall of
    // canvas, and four times the element already covers every pose at the
    // resting presentation — a magnified one gets proportionally more room.
    const grow = Math.min(4 * Math.max(1, magnify), Math.max(1, want / scale));
    stage.style.setProperty("--nd-model-grow", String(grow));
    camera.aspect = aspect;
    camera.updateProjectionMatrix();
    renderer.setSize(wide * grow, tall * grow, false);
  };
  size();

  // A GUEST TELLS ITS HOST HOW MUCH ROOM IT NEEDS. In a scene, a canvas
  // taller than the rows pokes past the field's edge and paints over the
  // page — so the module measures its own poke and asks the field to
  // breathe by exactly that much (see the stylesheet). Measured from
  // rendered boxes — the resting tilt is part of the resting pose and must
  // be in the measure, and it ARRIVES when the scene's runtime does, which
  // can be after this mount: a single measure took 6px of air and the tilt
  // then raised the true poke past 160. So the draw loop keeps measuring,
  // gently, until the reader takes the scene — a measure taken mid-orbit
  // or under zoom would ask for swung, magnified air, and the padding never
  // gives any back. Self-limiting: once the field has the air, the poke
  // measures zero.
  const room = stage.closest("[data-nd-lineage-field]");
  const whole = stage.closest(".nd-lineage");
  let calm = true;
  if (room && whole) {
    for (const gesture of ["pointerdown", "wheel", "keydown"]) {
      whole.addEventListener(
        gesture,
        () => {
          calm = false;
        },
        { capture: true, once: true, passive: true },
      );
    }
  }
  const breathe = () => {
    if (!room || !whole || !calm) return;
    const zoomed = Number.parseFloat(getComputedStyle(room).getPropertyValue("--nd-zoom")) || 1;
    if (zoomed !== 1) return;
    // Against the FIELD — the very box the engaged camera clips to — and
    // stamped there, where the air moves the grid origin and the space's
    // height together (see the stylesheet), so the window holds the whole
    // canvas in every mode. The measure is the RESIDUAL poke on top of
    // whatever air the field already wears — the tilt eases in, so the
    // poke arrives over many rounds and each round's remainder is ADDED,
    // never written over what an earlier round already asked for.
    const edge = room.getBoundingClientRect();
    const brim = canvas.getBoundingClientRect();
    const over = Math.max(0, Math.ceil(edge.top - brim.top));
    const under = Math.max(0, Math.ceil(brim.bottom - edge.bottom));
    const wornOver = Number.parseFloat(room.style.getPropertyValue("--nd-model-air-over")) || 0;
    const wornUnder = Number.parseFloat(room.style.getPropertyValue("--nd-model-air-under")) || 0;
    if (over > 0) room.style.setProperty("--nd-model-air-over", `${wornOver + over}px`);
    if (under > 0) room.style.setProperty("--nd-model-air-under", `${wornUnder + under}px`);
  };
  breathe();
  new ResizeObserver(() => {
    size();
    breathe();
  }).observe(stage);

  // Where the pointer is, as a fraction of the window either side of the stage.
  let wantX = 0;
  let wantY = 0;
  window.addEventListener(
    "pointermove",
    (event) => {
      if (still.matches || gaze === 0) return;
      const box_ = stage.getBoundingClientRect();
      const cx = box_.left + box_.width / 2;
      const cy = box_.top + box_.height / 2;
      wantX = Math.max(-1, Math.min(1, (event.clientX - cx) / (window.innerWidth / 2)));
      wantY = Math.max(-1, Math.min(1, (event.clientY - cy) / (window.innerHeight / 2)));
    },
    { passive: true },
  );

  // The enclosing scene's camera, read fresh every frame — the billboard's
  // other half. No scene, no angles, no movement.
  const sceneRoot = stage.closest(".nd-lineage");
  const bearing = () => {
    if (!sceneRoot) return { yaw: 0, pitch: 0, zoom: 1 };
    const style = getComputedStyle(sceneRoot);
    return {
      yaw: ((Number.parseFloat(style.getPropertyValue("--nd-yaw")) || 0) * Math.PI) / 180,
      pitch: ((Number.parseFloat(style.getPropertyValue("--nd-pitch")) || 0) * Math.PI) / 180,
      zoom: Number.parseFloat(style.getPropertyValue("--nd-zoom")) || 1,
    };
  };

  let frames = 0;
  let was = 0;
  let gazeX = 0;
  let gazeY = 0;
  const draw = (now) => {
    frames += 1;
    // The clip advances by real elapsed time, so it runs at its authored
    // speed on every display — clamped, so the first frame after a
    // background tab returns does not leap.
    if (mixer && !still.matches) {
      mixer.update(Math.min(0.1, (now - was) / 1000 || 0));
    }
    was = now;
    const idle = still.matches || gaze === 0 ? 0 : Math.sin(frames / 190) * gaze * 0.25;
    const targetX = still.matches ? 0 : wantX * gaze + idle;
    const targetY = still.matches ? 0 : wantY * gaze * 0.6;
    gazeX += (targetX - gazeX) * EASE;
    // THE STAMP IS THE GAZE, not the bearing: a scene's resting tilt is not
    // the model looking anywhere.
    stage.dataset.ndModelLook =
      (gazeX > 0.05 ? "right" : gazeX < -0.05 ? "left" : "ahead") +
      " " +
      (gazeY > 0.05 ? "down" : gazeY < -0.05 ? "up" : "level");
    gazeY += (targetY - gazeY) * EASE;
    const view = bearing();
    held.rotation.y = view.yaw + gazeX;
    held.rotation.x = view.pitch + gazeY;
    // THE SCENE'S MAGNIFICATION IS THE CANVAS'S RESOLUTION. The camera's
    // zoom scales the canvas up as a picture, and a picture enlarged past
    // its pixels is pixels — so the canvas re-renders at the magnification
    // it is shown at, up to `ratio`. Checked at intervals with a wide dead
    // band: a resize reallocates the buffer, which is nothing to do per
    // wheel notch.
    if (frames % 15 === 0) {
      const wish = Math.min(
        ratio,
        Math.max(1, (window.devicePixelRatio || 1) * Math.max(1, view.zoom)),
      );
      const held_ = renderer.getPixelRatio();
      if (wish > held_ * 1.3 || wish < held_ / 1.6) {
        renderer.setPixelRatio(wish);
        size();
      }
    }
    // Until the reader takes the scene, keep the field's air honest — the
    // scene's own runtime may apply the resting tilt at any time after this
    // module mounted, and the tilt changes how far the canvas reaches. The
    // step is quick because each pad shifts the projection and the measure
    // converges over a few rounds, not one.
    if (calm && frames % 10 === 0) breathe();
    renderer.render(scene, camera);
    window.requestAnimationFrame(draw);
  };
  window.requestAnimationFrame(draw);

  // The flat content goes only now: removing it when the module loaded would
  // leave an empty box for however long the first frame took.
  for (const flat of stage.querySelectorAll("[data-nd-model-flat]")) flat.hidden = true;
}
