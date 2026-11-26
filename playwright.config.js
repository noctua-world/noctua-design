// The browser suite.
//
// It runs against the BUILT site — `cargo xtask docs` output served as static
// files — rather than a dev server, because what ships is static files and a
// dev server can differ from them in exactly the ways that matter here
// (stylesheet order, render blocking, whether the bootstrap ran).

import { defineConfig, devices } from "@playwright/test";

// ONE PART OF THE SUITE, when NOCTUA_SHARD names one, as `2/4`. CI's `browser`
// job runs the suite in four parts, because a runner's two workers would take
// longer over the whole of it than the rest of CI together; unset, the whole
// suite runs. Parts are cut between test groups, so a serial walk that tallies
// its pages after the last one is never split across two runners.
//
// FAIL CLOSED. A value that is not a part — a typo, `0/4`, `5/4` — stops the
// run instead of running everything or nothing: either would be a green job
// that did not run what its name says.
function shard(value) {
  if (value === undefined || value === "") return null;
  const match = /^([1-9][0-9]*)\/([1-9][0-9]*)$/.exec(value);
  if (!match || Number(match[1]) > Number(match[2])) {
    throw new Error(`NOCTUA_SHARD is "${value}", which is not a part such as 2/4`);
  }
  return { current: Number(match[1]), total: Number(match[2]) };
}

export default defineConfig({
  // Both suites: the browser tests, and the conformance cases that drive both
  // implementations from `spec/components/<name>.cases.toml`.
  testDir: "tests",
  fullyParallel: true,
  shard: shard(process.env.NOCTUA_SHARD),
  forbidOnly: !!process.env.CI,
  retries: 0,
  reporter: process.env.CI ? "github" : [["list"]],
  use: {
    baseURL: "http://127.0.0.1:8731",
    // Colors are read from screenshots, never from getComputedStyle, so the
    // rendering has to be deterministic across machines.
    deviceScaleFactor: 1,
    colorScheme: "light",
    // AND SO DOES THE READER'S LOCALE, now that anything reads it. A date
    // row's order and a time's clock are the reader's conventions, taken from
    // `navigator.languages` — which is the machine's unless a context pins
    // it, so the same suite would assert month-first here and day-first on a
    // machine set to en-GB. A test that wants another locale asks for its own
    // context, which is what `date-input.spec.js` does for all three.
    locale: "en-US",
  },
  projects: [
    {
      name: "chromium",
      use: {
        ...devices["Desktop Chrome"],
        // The system's Chrome, not Playwright's download.
        //
        // On NixOS the downloaded binary cannot find its shared libraries and
        // exits 127 before Playwright can say why — "Target page, context or
        // browser has been closed" is what that looks like from the outside.
        // `executablePath` rather than `channel: "chrome"`: the channel lookup
        // hardcodes /opt/google/chrome, which is not where a Nix system puts
        // it. NOCTUA_CHROME lets any machine point at its own.
        launchOptions: {
          // `executablePath` belongs inside launchOptions; at the top level of
          // `use` it is silently ignored, and Playwright falls back to its own
          // download without saying so.
          executablePath:
            process.env.NOCTUA_CHROME ??
            "/run/current-system/sw/bin/google-chrome-stable",
          // The sandbox needs kernel user namespaces, which a container often
          // does not grant. Chrome then exits before Playwright has a channel
          // to report on, which surfaces only as "browser has been closed".
          args: ["--no-sandbox", "--disable-dev-shm-usage"],
        },
      },
    },
  ],
  webServer: [
    {
      command: "python3 -m http.server 8731 --directory docs-site/dist",
      url: "http://127.0.0.1:8731/en/intro/",
      reuseExistingServer: true,
      timeout: 30_000,
    },
    // The repository root, for `examples/consumer-web/`. Over HTTP rather than
    // `file://` because that page imports the behaviour modules the way a
    // consumer does, and a module script from `file://` has a null origin,
    // which Chrome refuses — so the one thing the example exists to prove
    // could not be proved.
    {
      command: "python3 -m http.server 8732 --directory .",
      url: "http://127.0.0.1:8732/examples/consumer-web/index.html",
      reuseExistingServer: true,
      timeout: 30_000,
    },
  ],
});
