# rl-vm

> Bytecode virtual machine for the embedded RL language

Part of the **RL embedded scripting engine** - the `#![no_std]`-only hard fork of the rl-lang toolchain built around the bytecode VM. This crate is the execution engine: it compiles the resolved AST to bytecode and runs it on a stack-based VM.

## Pipeline

```text
Vec<Statement> -> Compiler::compile() -> Chunk -> Vm::run() / Vm::run_and_return()
```

## Modules

| Module | Contents |
|---|---|
| `compiler` | `Compiler` - compiles a resolved AST into a `Chunk` of bytecode |
| `bytecode` | `serialize_chunk` / `deserialize_chunk` - persisting compiled chunks (deflate-compressed) |
| `chunk` | `Chunk` and `OpCode` - the bytecode representation |
| `vm_logic` | `Vm` - the stack-based bytecode interpreter, plus `VmError` |
| `native` | `Module` and `NativeFn` - native function binding for the VM |
| `values` | `VmValue` and `VmNativeFn` - the VM's runtime value representation |
| `stdlib` | Standard library modules exposed to VM-compiled code |

## Dependencies

Builds on `rl-ast`, `rl-lexer`, `rl-resolver`, and `miniz_oxide` (for deflate-compressed bytecode serialization).

## Usage

```toml
[dependencies]
rl-vm = { workspace = true }
```

```rust
use rl_vm::{Compiler, Vm};

let chunk = Compiler::new(&ast).compile(&statements)?;
let result = Vm::new().run_and_return(&chunk)?;
```

## License

Licensed under either of Apache License, Version 2.0 or MIT license at your option - see [LICENSE.md](../../LICENSE.md) for the full text and why both are offered.
