# rl-std

> The single, runtime-agnostic standard library for the embedded RL engine, shared by the VM and the interpreter

Part of the **RL embedded scripting engine** - the `#![no_std]`-only hard fork of the rl-lang toolchain built around the bytecode VM.

## Overview

Each standard-library function is written once, generic over `rl_std_core::Runtime`, and annotated with `#[native_fn]` so it is available to the runtime as a thin function pointer, and to the compiler as a signature.

The embedded build is `#![no_std]` and single-threaded (`alloc::rc`). Only pure-computation modules ship; OS-facing modules (`c`, `audio`, `gui`, `http`, `net`, `path`, `process`, `terminal`, `time`, `fs`) are intentionally absent - there is no filesystem or network on bare metal, and wall-clock time / stdin / stderr are host concerns handled by the embedding application.

## Modules

| Module | Contents |
|---|---|
| `array` | `std::array` - array builders, slicing, and reductions |
| `bitwise` | `std::bitwise` - bit-level operations on integers |
| `collections` | `std::collections` - map and set operations |
| `debug` | `std::debug` - runtime debugging helpers |
| `io` | `std::io` - print / println into the host output buffer |
| `math` | `std::math` - math functions (float transcendentals via `libm`) |
| `random` | `std::random` - PRNG-backed generation |
| `result` | `std::result` - `result[T]` helpers |
| `string` | `std::str` - string manipulation |
| `types` | `std::types` - type introspection |

## Dependencies

Depends on `rl-std-core`, `rl-std-macros`, `rl-ast`, and `rl-utils`.

## Usage

```toml
[dependencies]
rl-std = { workspace = true }
```

## License

Licensed under either of Apache License, Version 2.0 or MIT license at your option - see [LICENSE.md](../../LICENSE.md) for the full text and why both are offered.
