//! Import statement parser (`get`).
//!
//! Handles the stdlib import forms in RL:
//!
//! ```text
//! // stdlib function
//! get std::math::sin
//!
//! // named imports from a stdlib module
//! get sin, cos from std::math
//! ```
//!
//! The embedded build has no filesystem, so file imports (`get mymodule`,
//! `get add, sub from mymodule::utils`) are rejected at parse time with a
//! clear error. The first token after `get` and whether `::` or `from`
//! follows determines which [`StatementKind`] variant is produced:
//!
//! | syntax | kind |
//! |---|---|
//! | `get std::ns::fn` | [`StatementKind::Import`] |
//! | `get fn, fn from std::ns` | [`StatementKind::Import`] |

use alloc::vec::Vec;
use alloc::string::ToString;

use crate::parser_logic::Parser;
use rl_ast::statements::{Statement, StatementKind};
use rl_lexer::tokentypes::TokenType;
use rl_utils::{errors::Error, span::Span};

impl Parser {
    /// Parses a `get` import statement.
    ///
    /// Called after `get` has been consumed. Dispatches on the tokens that
    /// follow the first identifier:
    ///
    /// - **`get std::ns::fn`** - multi-segment stdlib path. The last segment is
    ///   treated as the function name and the rest as the namespace path ->
    ///   [`StatementKind::Import`].
    ///
    /// - **`get name, name from std::ns`** - named stdlib imports ->
    ///   [`StatementKind::Import`]`{ names, path }`.
    ///
    /// # Errors
    /// Returns an error if an identifier is missing after `get`, `::`, `,`, or
    /// `from`, if `from` itself is absent in the named-import form, or if the
    /// import references a source file (unsupported on the embedded build).
    pub fn parse_import(&mut self, start: Span) -> Result<Statement, Error> {
        let first = match self.peek() {
            TokenType::Identifier(name) => name,
            _ => return Err(self.err("expected identifier after 'get'", self.peek_span())),
        };
        self.advance();

        // multi-segment path: get std::math::sin
        if self.match_type(&[TokenType::ColonColon]) {
            let mut segments = vec![first];
            loop {
                match self.peek() {
                    TokenType::Identifier(seg) => {
                        self.advance();
                        segments.push(seg);
                    }
                    _ => return Err(self.err("expected identifier after '::'", self.peek_span())),
                }
                if !self.match_type(&[TokenType::ColonColon]) {
                    break;
                }
            }
            let span = start.join(self.previous_span());
            let is_std = segments[0] == "std";
            return if is_std {
                // last segment is the function name; everything before it is the path
                let name = segments
                    .pop()
                    .ok_or_else(|| self.err("expected function name after '::'", start))?;
                Ok(Statement::new(
                    StatementKind::Import {
                        names: vec![name],
                        path: segments,
                    },
                    span,
                ))
            } else {
                Err(self.err(
                    "file imports are not supported in the embedded build",
                    span,
                ))
            };
        }

        // single-segment file import: get mymodule (unsupported on the embedded build)
        if !matches!(self.peek(), TokenType::Comma | TokenType::From) {
            let span = start.join(self.previous_span());
            return Err(self.err(
                "file imports are not supported in the embedded build",
                span,
            ));
        }

        // named imports: get add, sub from …
        let mut names = vec![first];
        if self.match_type(&[TokenType::Comma]) {
            loop {
                self.match_type(&[TokenType::Newline]);
                match self.peek() {
                    TokenType::Identifier(name) => {
                        self.advance();
                        names.push(name);
                    }
                    _ => return Err(self.err("expected identifier after ','", self.peek_span())),
                }
                if !self.match_type(&[TokenType::Comma]) {
                    break;
                }
            }
        }

        if !self.match_type(&[TokenType::From]) {
            return Err(self.err("expected 'from' after names", self.peek_span()));
        }

        let mut path = Vec::new();
        loop {
            match self.peek() {
                TokenType::Identifier(segment) => {
                    self.advance();
                    path.push(segment);
                }
                TokenType::Set => {
                    self.advance();
                    path.push("set".to_string());
                }
                TokenType::Map => {
                    self.advance();
                    path.push("map".to_string());
                }
                _ => return Err(self.err("expected path after 'from'", self.peek_span())),
            }
            if !self.match_type(&[TokenType::ColonColon]) {
                break;
            }
        }

        let span = start.join(self.previous_span());
        let is_std = path.first().map(|s| s == "std").unwrap_or(false);

        if is_std {
            Ok(Statement::new(StatementKind::Import { names, path }, span))
        } else {
            Err(self.err(
                "file imports are not supported in the embedded build",
                span,
            ))
        }
    }
}