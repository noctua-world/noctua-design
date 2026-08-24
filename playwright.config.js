// The browser suite.
//
// It runs against the BUILT site — `cargo xtask docs` output served as static
// files — rather than a dev server, because what ships is static files and a
// dev server can differ from them in exactly the ways that matter here
// (stylesheet order, render blocking, whether the bootstrap ran).

import { defineConfig, devices } from "@playwright/test";

export default defineConfig({
  // Both suites: the browser tests, and the conformance cases that drive both
  // implementations from `spec/components/<name>.cases.toml`.
  testDir: "tests",
  fullyParallel: true,
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
