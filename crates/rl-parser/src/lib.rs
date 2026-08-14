//! The parser - transforms a flat [`Vec<Token>`] produced by the lexer into a
//! [`Vec<Statement>`] that represents the abstract syntax tree (AST).
//!
//! # Pipeline position
//! ```text
//! source -> Lexer -> [Token] -> Parser -> [Statement] -> Compiler -> VM
//! ```
//!
//! # Module layout
//! - [`parser_logic`] - the [`Parser`] struct and its core cursor primitives
//! - `expressions` - precedence-climbing expression parser
//! - `statements` - one sub-module per statement kind
//! - `utils` - shared helpers (type annotation parsing)
#![no_std]

#[macro_use]
extern crate alloc;

mod expressions;
pub mod parser_logic;
mod statements;
mod utils;
