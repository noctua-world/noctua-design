# Convenience aliases. Every one is a `cargo xtask` verb underneath — `just` is
# never the only path, so a fresh clone with nothing but rustup can still do
# everything.

# List the available commands.
default:
    @just --list

# Compile spec/ into tokens, icons, props types and dist/.
build:
    cargo xtask build

# The single gate: spec validity, source gates, drift, fmt, clippy, tests.
check:
    cargo xtask check

# Watch spec/ and the components, rebuild, and serve the docs site.
dev port="8080":
    cargo xtask dev --port {{port}}

# Build the documentation site for both locales.
docs:
    cargo xtask docs

# Prepare a release. Writes the version; committing is left to a human.
release version:
    cargo xtask release {{version}}
