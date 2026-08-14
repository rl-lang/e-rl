# rl-std-core

> Runtime-agnostic core for the embedded RL standard library: the `Runtime` abstraction, native-function descriptors, value conversions, and stdlib signatures

Part of the **RL embedded scripting engine** - the `#![no_std]`-only hard fork of the rl-lang toolchain built around the bytecode VM.

## Overview

Holds what the shared stdlib (`rl-std`) and the runtime (`rl-vm`) need in common, without referencing the runtime's value type, so it can sit below it in the dependency graph:

- The `Runtime` trait the runtime implements, plus the shared `HandleStore` for opaque resource handles
- The thin-`fn`-pointer native descriptor (`NativeHandle` / `Arity`)
- Value <-> Rust type conversions (`ValueType` / `FromValueR` / `IntoValueR`)
- The signature types (`StdFn` / `ModuleNames`)
- `Xoshiro256`, the shared PRNG

## Modules

| Module | Contents |
|---|---|
| `runtime` | `Runtime` trait, `HandleStore`, and `R::Cx` runtime context |
| `handle` | `NativeHandle`, `Arity`, `NativeThunk` - thin-pointer native descriptors |
| `convert` | `ValueType`, `FromValueR`, `IntoValueR` value <-> Rust type conversions |
| `module` | Module builders that aggregate per-function handles and signatures |
| `rng` | `Xoshiro256` - the shared PRNG |
| `signatures` | `StdFn`, `ModuleNames` - signature types |

## Dependencies

Depends only on `rl-ast` and `rl-utils`.

## Usage

```toml
[dependencies]
rl-std-core = { workspace = true }
```

```rust
use rl_std_core::Runtime;

fn process<R: Runtime>(_cx: &mut R::Cx, value: R::Value) -> R::Value {
    value
}
```

## License

Licensed under either of Apache License, Version 2.0 or MIT license at your option - see [LICENSE.md](../../LICENSE.md) for the full text and why both are offered.
