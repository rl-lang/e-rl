use alloc::string::String;
use alloc::string::ToString;
use crate::values::VmValue;
use rl_utils::source::SourceFile;

/// Lexes, parses, resolves, compiles, and runs `code` on a fresh `Vm`,
/// returning the value of the last expression. Used by `std::rl::eval` and
/// `std::rl::eval_isolated` (the VM cannot splice into the running call
/// stack, so both run isolated).
pub fn compile_and_run(code: String, name: &str) -> Result<VmValue, String> {
    let source = SourceFile::new(name, code);

    let tokens =
        rl_lexer::tokenizer::Tokenizer::lex(source.clone()).map_err(|e| e.message().to_string())?;
    let (ast, statements) = rl_parser::parser_logic::Parser::parse(tokens, source.clone())
        .map_err(|e| e.message().to_string())?;

    let mut resolver = rl_resolver::Resolver::new();
    let statements = resolver.resolve_program(ast, statements);

    let chunk = crate::compiler::Compiler::new(&resolver.ast_arena)
        .compile(&statements)
        .map_err(|e| e.message().to_string())?;

    let result = crate::vm_logic::Vm::new()
        .with_source_file(source)
        .run_and_return(&chunk)
        .map_err(|e| e.message().to_string())?;

    Ok(result)
}
