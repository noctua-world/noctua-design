// One icon in, one icon out — measured on a real bundler.
//
// The icon set is every Lucide drawing, and the promise attached to that is
// that an application drawing one of them pays for one of them. `cargo xtask
// check` makes that measurement on a linked release binary, which is the half
// the Rust build can make on its own. This is the other half, and it is the one
// that matters to a JavaScript consumer: what an ESM bundler actually keeps.
//
// It has to be a bundler. The arrangement it verifies — one module per icon,
// one export each, no aggregate anywhere — is exactly the arrangement that
// LOOKS fine under every other kind of check and fails under a bundler, because
// what defeats tree-shaking is a dynamic property read and a dynamic property
// read is valid JavaScript. `ICONS[icon]` is what the React `Icon` used to do.
//
// esbuild is already a devDependency of this workspace, so this adds nothing.
// It runs in the browser suite because the Rust half of this repository must
// build with no Node installed (`NOCTUA.md §9.11`), which rules out asking
// `cargo xtask check` to do it.

import { test, expect } from "@playwright/test";
import { execFileSync } from "node:child_process";
import { mkdtempSync, mkdirSync, writeFileSync, readFileSync, rmSync, symlinkSync } from "node:fs";
import { tmpdir } from "node:os";
import { join, dirname, resolve } from "node:path";
import { fileURLToPath } from "node:url";

const root = resolve(dirname(fileURLToPath(import.meta.url)), "../..");
const esbuild = join(root, "node_modules/.bin/esbuild");

// THE GEOMETRY, NOT THE LITERAL THAT SPELLS IT. Read out of the manifest,
// which is JSON, so what comes back is the string a drawing actually is. The
// escaped form a module is written in is the wrong needle: esbuild re-quotes a
// string containing `"` with `'` to avoid the escapes, so a search for
// `<path d=\"…\"/>` finds nothing in a bundle that contains the drawing.
const MANIFEST = new Map(
  JSON.parse(readFileSync(join(root, "dist/icons.json"), "utf8")).icons.map((icon) => [
    icon.name,
    icon.body,
  ]),
);

/** The geometry of an icon, as a string a bundle either holds or does not. */
function drawing(name) {
  const body = MANIFEST.get(name);
  expect(body, `${name} is not in the manifest`).toBeTruthy();
  return body;
}

/** Bundles `source` as production ESM and returns the output. */
function bundle(source) {
  const directory = mkdtempSync(join(tmpdir(), "noctua-bundle-"));
  try {
    const entry = join(directory, "entry.js");
    writeFileSync(entry, source);
    // BY THE PUBLISHED NAMES, exactly as a consumer writes them. The fixture
    // gets the two packages installed under those names, which is what an
    // install produces — an entry that reached into `../../packages/react/src`
    // would be measuring an import nobody can write.
    const scope = join(directory, "node_modules/@noctua-world");
    mkdirSync(scope, { recursive: true });
    symlinkSync(join(root, "packages/react"), join(scope, "design-react"));
    symlinkSync(join(root, "packages/tokens"), join(scope, "design-tokens"));
    execFileSync(
      esbuild,
      [
        entry,
        "--bundle",
        "--minify",
        "--format=esm",
        "--tree-shaking=true",
        "--platform=browser",
        // React is the consumer's, not ours: the package declares it a peer.
        "--external:react",
        `--outfile=${join(directory, "out.js")}`,
      ],
      { cwd: root, stdio: "pipe" },
    );
    return readFileSync(join(directory, "out.js"), "utf8");
  } finally {
    rmSync(directory, { recursive: true, force: true });
  }
}

test("a bundle that draws one icon contains one icon", () => {
  const out = bundle(`
    import { Icon } from "@noctua-world/design-react/components/icon.js";
    import { chevronDown } from "@noctua-world/design-tokens/icons/chevron-down.js";
    export const app = () => Icon({ icon: chevronDown });
  `);

  // The positive half first. A bundle that contains none of the four would pass
  // the three absences below while proving nothing at all.
  expect(out, "the icon that was imported is not in the bundle").toContain(
    drawing("chevron-down"),
  );
  for (const name of ["dna", "vault", "flask-conical", "menu"]) {
    expect(out, `${name} was never imported and is in the bundle`).not.toContain(drawing(name));
  }
});

test("a component brings the drawings it draws, and no others", () => {
  // `Button` renders `chevron-down` when it opens a menu, so its module imports
  // that one drawing. What it must not bring is the rest of the set.
  const out = bundle(`
    import { Button } from "@noctua-world/design-react/components/button.js";
    export const app = () => Button({ label: "Open", menu: true });
  `);

  expect(out, "Button draws a chevron and does not carry one").toContain(drawing("chevron-down"));
  for (const name of ["star", "pipette", "vault"]) {
    expect(out, `Button carries ${name}`).not.toContain(drawing(name));
  }
});

// A CEILING, NOT A TREND LINE. This is the size of an application that draws
// one icon, and the number the whole arrangement exists to keep small. It is
// deliberately loose — the point is to fail loudly if the set ever arrives
// whole again, which would be two orders of magnitude over it, not to argue
// about a kilobyte.
test("one icon costs kilobytes and not the whole set", () => {
  const out = bundle(`
    import { Icon } from "@noctua-world/design-react/components/icon.js";
    import { chevronDown } from "@noctua-world/design-tokens/icons/chevron-down.js";
    export const app = () => Icon({ icon: chevronDown });
  `);

  const everything = readFileSync(join(root, "dist/icons.json"), "utf8").length;
  expect(out.length, "one icon should not cost what every icon costs").toBeLessThan(
    everything / 100,
  );
  expect(out.length, `one icon and the Icon component came to ${out.length} bytes`).toBeLessThan(
    4096,
  );
});
