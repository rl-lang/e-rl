# Contributing to RL

Thanks for your interest in contributing!

## Getting started

```bash
git clone <your-fork-url> rl
cd rl
cargo build
```

## Before submitting a PR

```bash
cargo test --workspace                       # make sure all tests pass
cargo clippy --workspace -- -D warnings      # no lint warnings
cargo check --workspace --exclude rl-tests --target thumbv7em-none-eabihf   # bare-metal gate
```

- PRs must be up to date with the `dev` branch - always branch off `dev` and rebase before submitting
- Every crate in the workspace is `#![no_std]` and single-threaded (`alloc::rc`). New code must not
  pull in `std`, OS facilities, threads, or global state.

## What to work on

Open an issue on this repository for open bugs and feature requests.

## Versioning & releases

This project follows SemVer (`vMAJOR.MINOR.PATCH`, with `-alpha`/`-beta`/`-rc` pre-releases). See
[VERSIONING.md](VERSIONING.md) for the full breakdown of when to use each. PR descriptions that
change public behavior should note whether the change is a breaking (major), additive (minor), or
fix-only (patch) change so maintainers tag it correctly.

## Guidelines

- Keep PRs focused - one fix or feature per PR
- Add tests for new behavior where possible (integration tests live in `crates/rl-tests/tests/`)
- Follow the existing code style
- Update docs if you change language behavior or add stdlib functions

## Adding a stdlib function

A stdlib function is written once, generic over `rl_std_core::Runtime`, and lowered by the
`#[native_fn]` proc macro into a thin function-pointer handle plus its signature. Adding one
touches two places:

1. **Implementation** - add the function to the right module in `crates/rl-std/src/` (one file per
   module: `array.rs`, `bitwise.rs`, ...), annotated with `#[native_fn]`. The macro generates
   `mod::handles::<R>()` and `mod::signature()` builders automatically.
2. **Register the module** - if the module isn't registered yet, add it to the module tree in
   `crates/rl-vm/src/stdlib/mod.rs` (`root()`) with
   `Module::from_std("<name>", rl_std::<module>::handles::<VmRuntime>())`.

Only pure-computation modules belong in the embedded stdlib. Anything that needs a filesystem,
network, wall-clock time, stdin/stderr, or threads is a host concern and belongs in the embedding
application instead.

## AI usage

Using AI tools to help write a contribution is fine, but you're expected to understand, test, and
take responsibility for anything you submit. See [AI_POLICY.md](AI_POLICY.md) for what's and isn't
okay.

## Questions

Open an issue or reach out via GitHub.
