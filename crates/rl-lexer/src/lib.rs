//! The lexer - converts raw source text into a flat [`Vec<Token>`].
//!
//! # Pipeline position
//! ```text
//! source -> Lexer -> [Token] -> Parser -> [Statement] -> Compiler -> VM
//! ```
//!
//! # Module layout
//! - [`tokenizer`] - the [`Tokenizer`] struct and its main scanning loop
//! - [`tokentypes`] - the [`Token`] and [`TokenType`] definitions
//! - `scanner` - top-level scan driver called by the pipeline
//! - `types` - sub-scanners for each literal kind (string, char, number, identifier)
//! - `utils` - shared cursor helpers used across the sub-scanners
#![no_std]

#[macro_use]
extern crate alloc;

mod scanner;
pub mod tokenizer;
pub mod tokentypes;
mod types;
mod utils;