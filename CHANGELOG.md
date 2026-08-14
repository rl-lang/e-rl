# Changelog

All notable changes to the RL embedded scripting engine are documented here. The format is based on
[Keep a Changelog](https://keepachangelog.com/), and the project follows Semantic Versioning (see
[VERSIONING.md](VERSIONING.md)).

## [Unreleased]

### Changed

- Rebranded as a standalone project. This workspace is now the **RL embedded scripting engine**: a
  hard fork of the rl-lang toolchain rebuilt as a `#![no_std]`-only language runtime.
- Every crate in the workspace is `#![no_std]` and single-threaded (`alloc::rc`).
- Removed the CLI, TUI REPL, type checker, language server, tree-walking interpreter, and the
  OS-facing stdlib modules (`c`, `audio`, `fs`, `gui`, `http`, `net`, `path`, `process`, `term`,
  `time`).
- The stdlib (`rl-std`) is now runtime-agnostic and pure-computation only: `array`, `bitwise`,
  `collections`, `debug`, `io` (print/println into a host capture buffer), `math` (transcendentals
  via `libm`), `random`, `result`, `str`, and `types`.
- New `rl-embed` crate is the intended embedding entry point (`compile`, `run`, output-buffer and
  PRNG-seed helpers).
- New `rl-tests` crate hosts the host-side integration test harness.
- `.rlc` bytecode is now deflate-compressed via `miniz_oxide` instead of zstd. Compiled chunks from
  the old toolchain are not loadable.
- `print`/`println` write to a capture buffer only - nothing writes to an OS console by itself.
- The `std::random` PRNG defaults to a fixed seed; hosts seed it from their own entropy source via
  `Vm::with_seed`.
- Errors render as plain `file:line:col` text instead of ariadne-styled diagnostics.
- File imports (`get x from path/to/file.rl`) are rejected at parse time.

## [1.2.0] - 2026-08-06

Last release before the hard fork. Upstream rl-lang 1.x history.
