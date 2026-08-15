# three.js, vendored

`three-css3d.js` is a tree-shaken bundle of **three.js r180** containing the
classes `CSS3DRenderer` needs and nothing else: `PerspectiveCamera`, `Scene`,
`Object3D`, `Vector3`, `Matrix4`, `Quaternion`, `MathUtils`, plus
`CSS3DObject` and `CSS3DRenderer` themselves.

| | |
|---|---|
| Project | three.js |
| Version | r180 (`three@0.180.0`) |
| Home | https://threejs.org |
| Source | https://github.com/mrdoob/three.js |
| Licence | MIT — `LICENSE-three-MIT.txt` beside this file |
| Size | 235 KB, 53 KB gzipped |

`three-webgl.js` is a second tree-shaken bundle of the same release, containing
the renderer, the geometries, the materials and the lights that the owl on the
`Lineage` page is modelled from. **It is not part of `dist/`**: a consumer of
`Lineage` never fetches it, because it is not part of `Lineage` — it is an
example of what a consumer may put INSIDE a figure, and it lives with the
documentation site that shows it.

| | |
|---|---|
| File | `three-webgl.js` |
| Size | 571 KB, 146 KB gzipped |
| Fetched | by `docs-site/assets/owl.js`, on the first frame the owl is on screen |

## Why it is vendored rather than installed

`cargo xtask check` runs in CI **with no Node installed at all**, and the Rust
half of this repository must build and test without a JavaScript toolchain
(`NOCTUA.md §9.11`). A dependency resolved at build time would break both. So
the bundle is committed, exactly as the fonts, the colour tokens and the Lucide
drawings are, and `cargo xtask build` copies it into `dist/`.

It is also **fetched on demand**, never on page load: `dist/lineage.js` imports
it the first time a reader touches a scene. A page that is only read costs
nothing.

## Reproducing this file

```sh
V=0.180.0
mkdir -p /tmp/three/node_modules/three && cd /tmp/three
curl -s -o node_modules/three/index.js       https://unpkg.com/three@$V/build/three.module.js
curl -s -o node_modules/three/three.core.js  https://unpkg.com/three@$V/build/three.core.js
curl -s -o CSS3DRenderer.js                  https://unpkg.com/three@$V/examples/jsm/renderers/CSS3DRenderer.js
curl -s -o LICENSE-three-MIT.txt             https://unpkg.com/three@$V/LICENSE
printf '{"name":"three","version":"%s","type":"module","main":"index.js"}' $V > node_modules/three/package.json
cat > entry.js <<'ENTRY'
export { PerspectiveCamera, Scene, Object3D, Vector3, Matrix4, Quaternion, MathUtils } from "three";
export { CSS3DObject, CSS3DRenderer } from "./CSS3DRenderer.js";
ENTRY
esbuild entry.js --bundle --format=esm --minify --tree-shaking=true --legal-comments=none --outfile=three-css3d.js

curl -s -o GLTFLoader.js                              https://unpkg.com/three@$V/examples/jsm/loaders/GLTFLoader.js
mkdir -p utils
curl -s -o utils/BufferGeometryUtils.js               https://unpkg.com/three@$V/examples/jsm/utils/BufferGeometryUtils.js
sed -i "s#from '../utils/BufferGeometryUtils.js'#from './utils/BufferGeometryUtils.js'#" GLTFLoader.js
cat > webgl.js <<'ENTRY'
export {
  WebGLRenderer, Scene, PerspectiveCamera, Group, Mesh, Color, MathUtils,
  SphereGeometry, ConeGeometry, CylinderGeometry, CircleGeometry,
  MeshStandardMaterial, DirectionalLight, AmbientLight, Vector2, Vector3,
  Box3, AnimationMixer,
} from "three";
export { GLTFLoader } from "./GLTFLoader.js";
ENTRY
esbuild webgl.js --bundle --format=esm --minify --tree-shaking=true --legal-comments=none --outfile=three-webgl.js
```

Then copy `three-css3d.js` and `LICENSE-three-MIT.txt` here and update the
version above. `esbuild` is already a dev dependency of this repository.
