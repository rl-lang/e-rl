<div align="center">
  <img src="assets/logo-circle.svg" width="200">
  <h1>RL</h1>
  <p>A statically-typed scripting language for bare-metal and embedded applications, written in Rust.</p>
</div>

[![Rust](https://img.shields.io/badge/Made%20with-Rust-000000?style=for-the-badge&logo=rust&logoColor=white)](https://www.rust-lang.org/)
[![License](https://img.shields.io/badge/License-MIT%20OR%20Apache--2.0-blue?style=for-the-badge)](LICENSE.md)
[![no_std](https://img.shields.io/badge/std-no__std-red?style=for-the-badge)](crates/rl-vm/src/lib.rs)

## What this is

RL is the **RL embedded scripting engine**: a hard fork of the rl-lang toolchain, rebuilt as a
`#![no_std]`-only language runtime that runs on bare metal and custom kernels.

> **Note:** this project is an independent fork. It is not affiliated with, endorsed by, or
> maintained by the rl-lang project or its maintainers, and the RL language maintained here is
> not the same project as upstream rl-lang. This repository has its own maintainer.

It keeps the RL language and its bytecode VM, and drops everything that assumes a hosted OS -
the CLI, TUI REPL, type checker, language server, tree-walking interpreter, and every OS-facing
stdlib module (filesystem, network, GUI, audio, C interop, ...).

The full pipeline compiles and executes entirely in a single-threaded, `alloc`-based environment:

```text
source -> Lexer -> Parser -> Resolver -> Compiler -> Chunk -> Vm
```

## What ships

| Component | What it provides |
|---|---|
| `rl-embed` | The host-facing entry point: `compile`, `run`, output-buffer and PRNG-seed helpers |
| `rl-vm` | Stack-based bytecode VM, `Chunk`/`OpCode`, `.rlc` bytecode serialization, VM stdlib |
| `rl-std` | Pure-computation stdlib modules: `array`, `bitwise`, `collections`, `debug`, `io` (print/println into a capture buffer), `math`, `random`, `result`, `str`, `types` |
| `rl-parser` / `rl-resolver` / `rl-ast` / `rl-lexer` | The compile pipeline |
| `rl-std-core` / `rl-std-macros` | Runtime-agnostic stdlib core and the `#[native_fn]` proc macro |
| `rl-tests` | Host-side integration test harness |

The VM and stdlib are `#![no_std]` and single-threaded (`alloc::rc`). Float math uses `libm`
(transcendentals) and `ryu` (display). Compiled `.rlc` chunks are deflate-compressed via
`miniz_oxide`.

## Quick look

```rl
get println from std::io

fn fib(int n) {
    if (n < 2) {
        return n
    }
    return fib(n - 1) + fib(n - 2)
}

dec int total = 0
dec int i = 0
while (i <= 10) {
    total += fib(i)
    i += 1
}
println("sum of fib(0..10) = ", total)   // sum of fib(0..10) = 143
```

RL syntax is newline-separated (no `;`), variables are declared with `dec`/`CONST`, and stdlib
functions must be imported first via `get name from std::module`.

## Embedding

Add `rl-embed` to your application:

```rust
use rl_embed::{run, VmValue};

let code = r#"
    get println from std::io
    println("hello, ", 2 + 3)
"#;

let (value, output) = run(code, "app.rl", None)?;
assert_eq!(output, "hello, 5\n");
```
The embedding binary must supply a global allocator and a panic handler; `print`/`println`
write to a capture buffer the host drains via `take_output`. Seed `std::random` from your own
entropy source via `seed_vm`.

## Development

```bash
cargo test --workspace        # full test suite
cargo clippy --workspace -- -D warnings   # lints
# bare-metal gate (thumbv7em-none-eabihf):
cargo check --workspace --exclude rl-tests --target thumbv7em-none-eabihf
```

## License

Licensed under either of [MIT](LICENSE-MIT.md) or [Apache 2.0](LICENSE-APACHE.md) at your option.
