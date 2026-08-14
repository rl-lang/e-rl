# rl-ast

> Abstract syntax tree types for the embedded RL language

Part of the **RL embedded scripting engine** - the `#![no_std]`-only hard fork of the rl-lang toolchain built around the bytecode VM.

## Pipeline position

```text
source -> Lexer -> [Token] -> Parser -> [Statement] -> Resolver -> VM Compiler
```

`rl-ast` defines the node types produced by the parser and consumed by every later stage (resolver, compiler, and VM).

## Modules

| Module | Contents |
|---|---|
| `nodes` | `Expression` and `ExpressionKind` - the expression AST |
| `statements` | `Statement`, `StatementKind`, `TypeAnnotation`, `Param` |
| `arena` | `Arena` / `Id` - arena allocation backing the AST nodes |

## Dependencies

Builds on `rl-lexer` (for token/span types shared with AST nodes) and `rl-utils`.

## Usage

```toml
[dependencies]
rl-ast = { workspace = true }
```

```rust
use rl_ast::nodes::{Expression, ExpressionKind};
use rl_ast::statements::{Statement, StatementKind};
```

## License

Licensed under either of Apache License, Version 2.0 or MIT license at your option - see [LICENSE.md](../../LICENSE.md) for the full text and why both are offered.
