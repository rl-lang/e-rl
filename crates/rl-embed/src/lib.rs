#![no_std]
//! Host-facing facade for the no_std RL scripting engine.
//!
//! This crate is the intended entry point for embedding RL into a
//! bare-metal application. It wraps the lexer/parser/resolver/compiler/VM
//! pipeline behind a small, stable API ([`compile`], [`compile_with_source`],
//! [`run`]) and re-exports the types a host most commonly needs.
//!
//! # Host responsibilities
//!
//! Because every crate in this workspace is `#![no_std]`, the embedding
//! binary must supply a global allocator (`#[global_allocator]`) and a
//! panic handler. `print`/`println` from `std::io` write to a capture
//! buffer that the host drains via [`take_output`]; nothing writes to an
//! OS console by itself.
//!
//! The PRNG backing `std::random` defaults to a fixed seed. Hosts that want
//! per-boot randomization should seed it from their own entropy source via
//! [`seed_vm`].

extern crate alloc;

use alloc::string::{String, ToString};

pub use rl_ast::statements::{HandleKind, Statement};
pub use rl_lexer::tokenizer::Tokenizer;
pub use rl_parser::parser_logic::Parser;
pub use rl_resolver::Resolver;
pub use rl_utils::source::SourceFile;
pub use rl_vm::bytecode;
pub use rl_vm::chunk::Chunk;
pub use rl_vm::compiler::Compiler;
pub use rl_vm::vm_logic::Vm;
pub use rl_vm::{VmError, VmValue};

/// Runs the full compile pipeline (lex -> parse -> resolve -> compile) on
/// `source`, returning the chunk together with the source (for attaching to
/// a [`Vm`]). Errors carry the source file, so their `file:line:col`
/// location is available even when compilation fails.
fn compile_source(source: SourceFile) -> Result<(Chunk, SourceFile), VmError> {
    let tokens = Tokenizer::lex(source.clone()).map_err(|e| e.with_source_file(&source))?;
    let (ast, statements) = Parser::parse(tokens, source.clone()).map_err(|e| e.with_source_file(&source))?;

    let mut resolver = Resolver::new();
    let statements = resolver.resolve_program(ast, statements);

    let chunk = Compiler::new(&resolver.ast_arena)
        .compile(&statements)
        .map_err(|e| e.with_source_file(&source))?;

    Ok((chunk, source))
}

/// Compiles `code` (an RL program) into a standalone [Chunk] that can
/// be run repeatedly and serialized to `.rlc` bytecode.
///
/// `name` is only used for error messages / stack traces. Source-file
/// imports are not supported in the embedded build, so programs must be
/// self-contained.
pub fn compile(code: impl Into<String>, name: &str) -> Result<Chunk, String> {
    compile_with_source(code.into(), name).map(|(chunk, _)| chunk)
}

/// Compiles `code` and returns the chunk together with its source, so a
/// caller can attach it to a [`Vm`] for rich runtime errors. Compile errors
/// are returned as their message string.
pub fn compile_with_source(code: String, name: &str) -> Result<(Chunk, SourceFile), String> {
    let source = SourceFile::new(name, code);
    compile_source(source).map_err(|e| e.message().to_string())
}

/// Parses and compiles `code`, then runs it to completion on a fresh [`Vm`],
/// returning the value of the last expression. `print`/`println` output is
/// collected into the returned buffer.
///
/// `seed` optionally seeds the `std::random` PRNG; `None` uses the fixed
/// default seed.
pub fn run(
    code: impl Into<String>,
    name: &str,
    seed: Option<u64>,
) -> Result<(VmValue, String), VmError> {
    let (chunk, source) = compile_source(SourceFile::new(name, code.into()))?;

    let mut vm = attach_output_buffer(Vm::new().with_source_file(source));
    if let Some(seed) = seed {
        vm = seed_vm(vm, seed);
    }

    let result = vm.run_and_return(&chunk)?;
    let output = take_output(&mut vm).unwrap_or_default();

    Ok((result, output))
}

/// Builder form of `Vm::output_buffer`: all subsequent `print`/`println`
/// calls append here instead of writing to the host console.
pub fn attach_output_buffer(mut vm: Vm) -> Vm {
    vm.output_buffer = Some(String::new());
    vm
}

/// Builder form of `Vm::output_buffer` on a `&mut Vm`.
pub fn set_output_buffer(vm: &mut Vm) {
    vm.output_buffer = Some(String::new());
}

/// Retrieves and clears the captured `print`/`println` output. Returns
/// `None` if no buffer was attached.
pub fn take_output(vm: &mut Vm) -> Option<String> {
    vm.output_buffer.take()
}

/// Concatenates the captured output *without* clearing it.
pub fn peek_output(vm: &Vm) -> Option<&str> {
    vm.output_buffer.as_deref()
}

/// Seeds the PRNG backing `std::random` with host-supplied entropy. See
/// [`Vm::with_seed`].
pub fn seed_vm(vm: Vm, seed: u64) -> Vm {
    vm.with_seed(seed)
}