//! Statement resolution - transforms declarations and control flow into
//! slot-indexed variants and resolves stdlib import statements at compile time.
//!
//! Key behaviors:
//!
//! - Variable/constant/array declarations: resolve the initializer expression
//!   first, *then* declare the name so the initializer cannot reference itself
//! - Function declarations: declare the name in the outer scope first (enabling
//!   recursion), then push a new scope for parameters and resolve the body
//! - `ForEach`/`ForRange`: the loop variable is declared inside its own scope
//!   so it does not leak into the surrounding scope after the loop ends

use crate::Resolver;
use alloc::boxed::Box;
use alloc::vec::Vec;
use rl_ast::{
    nodes::ExpressionKind,
    statements::{Statement, StatementKind},
    Ast,
};

impl Resolver {
    pub fn resolve_program(&mut self, ast: Ast, statements: Vec<Statement>) -> Vec<Statement> {
        let statements = self.ast_arena.merge_statements(ast, statements);
        self.resolve_statements(statements)
    }

    pub fn resolve_statements(&mut self, statements: Vec<Statement>) -> Vec<Statement> {
        statements
            .into_iter()
            .map(|statement| self.resolve_statement(statement))
            .collect()
    }

    fn resolve_statement(&mut self, stmt: Statement) -> Statement {
        let span = stmt.span;
        let kind = match stmt.kind {
            StatementKind::VariableDeclaration {
                name,
                type_annotation,
                unit_annotation: _,
                value,
            } => {
                let value = self.resolve_expression(value);
                let slot = self.declare(name.clone());
                StatementKind::ResolvedVariableDeclaration {
                    name,
                    slot,
                    type_annotation,
                    value,
                }
            }
            StatementKind::ConstantDeclaration {
                name,
                type_annotation,
                unit_annotation: _,
                value,
            } => {
                let value = self.resolve_expression(value);
                let slot = self.declare(name.clone());
                StatementKind::ResolvedConstantDeclaration {
                    name,
                    slot,
                    type_annotation,
                    value,
                }
            }
            // new arrays variant
            StatementKind::Array {
                name,
                type_annotation,
                value,
            } => {
                let value = value
                    .into_iter()
                    .map(|e| self.resolve_expression(e))
                    .collect();
                let slot = self.declare(name.clone());
                let value = self
                    .ast_arena
                    .alloc_expr(ExpressionKind::ArrayLiteral(value), span);

                StatementKind::ResolvedArray {
                    name,
                    slot,
                    type_annotation,
                    value,
                }
            }
            StatementKind::ConstantArray {
                name,
                type_annotation,
                value,
            } => {
                let value = value
                    .into_iter()
                    .map(|e| self.resolve_expression(e))
                    .collect();
                let slot = self.declare(name.clone());
                let value = self
                    .ast_arena
                    .alloc_expr(ExpressionKind::ArrayLiteral(value), span);

                StatementKind::ResolvedConstantArray {
                    name,
                    slot,
                    type_annotation,
                    value,
                }
            }

            StatementKind::Set {
                name,
                type_annotation,
                items,
            } => {
                let value = items
                    .into_iter()
                    .map(|e| self.resolve_expression(e))
                    .collect();
                let slot = self.declare(name.clone());
                let value = self
                    .ast_arena
                    .alloc_expr(ExpressionKind::SetLiteral(value), span);

                StatementKind::ResolvedSet {
                    name,
                    slot,
                    type_annotation,
                    value,
                }
            }
            StatementKind::ConstantSet {
                name,
                type_annotation,
                items,
            } => {
                let value = items
                    .into_iter()
                    .map(|e| self.resolve_expression(e))
                    .collect();
                let slot = self.declare(name.clone());
                let value = self
                    .ast_arena
                    .alloc_expr(ExpressionKind::SetLiteral(value), span);

                StatementKind::ResolvedConstantSet {
                    name,
                    slot,
                    type_annotation,
                    value,
                }
            }

            StatementKind::Map {
                name,
                type_annotation,
                entries,
            } => {
                let entries = entries
                    .into_iter()
                    .map(|(k, v)| (self.resolve_expression(k), self.resolve_expression(v)))
                    .collect();
                let slot = self.declare(name.clone());
                let value = self
                    .ast_arena
                    .alloc_expr(ExpressionKind::MapLiteral(entries), span);

                StatementKind::ResolvedMap {
                    name,
                    slot,
                    type_annotation,
                    value,
                }
            }
            StatementKind::ConstantMap {
                name,
                type_annotation,
                entries,
            } => {
                let entries = entries
                    .into_iter()
                    .map(|(k, v)| (self.resolve_expression(k), self.resolve_expression(v)))
                    .collect();
                let slot = self.declare(name.clone());
                let value = self
                    .ast_arena
                    .alloc_expr(ExpressionKind::MapLiteral(entries), span);

                StatementKind::ResolvedConstantMap {
                    name,
                    slot,
                    type_annotation,
                    value,
                }
            }

            StatementKind::FunctionDeclaration {
                name,
                params,
                return_type,
                body,
                attribute,
            } => {
                let slot = self.declare(name.clone());
                self.push_scope();
                for p in &params {
                    self.declare(p.param_name.clone());
                }
                let body = self.resolve_statements(body);
                self.pop_scope();
                StatementKind::ResolvedFunctionDeclaration {
                    name,
                    slot,
                    params,
                    return_type,
                    body,
                    attribute,
                }
            }
            StatementKind::ImplBlock { record, methods } => {
                let methods = methods
                    .into_iter()
                    .map(|m| {
                        let m_span = m.span;
                        match m.kind {
                            StatementKind::FunctionDeclaration {
                                name,
                                params,
                                return_type,
                                body,
                                attribute,
                            } => {
                                self.push_scope();
                                for p in &params {
                                    self.declare(p.param_name.clone());
                                }
                                let body = self.resolve_statements(body);
                                self.pop_scope();
                                Statement::new(
                                    StatementKind::ResolvedFunctionDeclaration {
                                        name,
                                        // impl methods are dispatched by
                                        // `record::method` name, not by
                                        // lexical address - this slot is
                                        // never read.
                                        slot: usize::MAX,
                                        params,
                                        return_type,
                                        body,
                                        attribute,
                                    },
                                    m_span,
                                )
                            }
                            other => Statement::new(other, m_span),
                        }
                    })
                    .collect();
                StatementKind::ResolvedImplBlock { record, methods }
            }

            StatementKind::ForEach {
                variable,
                iterable,
                body,
            } => {
                let iterable = self.resolve_expression(iterable);
                self.push_scope();
                let slot = self.declare(variable.clone());
                let body = self.resolve_statements(body);
                self.pop_scope();
                StatementKind::ResolvedForEach {
                    variable,
                    slot,
                    iterable,
                    body,
                }
            }
            StatementKind::ForRange {
                variable,
                range,
                body,
            } => {
                let range = Box::new(self.resolve_statement(*range));
                self.push_scope();
                let slot = self.declare(variable.clone());
                let body = self.resolve_statements(body);
                self.pop_scope();
                StatementKind::ResolvedForRange {
                    variable,
                    slot,
                    range,
                    body,
                }
            }
            StatementKind::For {
                initializer,
                condition,
                increment,
                body,
            } => {
                let initializer = Box::new(self.resolve_statement(*initializer));
                let condition = self.resolve_expression(condition);
                let increment = self.resolve_expression(increment);
                let body = self.resolve_statements(body);
                StatementKind::ResolvedFor {
                    initializer,
                    condition,
                    increment,
                    body,
                }
            }
            StatementKind::While { condition, body } => {
                let condition = self.resolve_expression(condition);
                self.push_scope();
                let body = self.resolve_statements(body);
                self.pop_scope();
                StatementKind::While { condition, body }
            }
            StatementKind::Loop(body) => {
                self.push_scope();
                let body = self.resolve_statements(body);
                self.pop_scope();
                StatementKind::Loop(body)
            }
            StatementKind::Conditional {
                if_branch,
                else_branch,
            } => {
                let if_branch = Box::new(self.resolve_statement(*if_branch));
                let else_branch = else_branch.map(|e| Box::new(self.resolve_statement(*e)));
                StatementKind::Conditional {
                    if_branch,
                    else_branch,
                }
            }

            StatementKind::ConditionalBranch {
                condition, body, ..
            } => {
                let condition = condition.map(|e| self.resolve_expression(e));
                let needs_scope = body.iter().any(|s| {
                    matches!(
                        s.kind,
                        StatementKind::VariableDeclaration { .. }
                            | StatementKind::ConstantDeclaration { .. }
                            | StatementKind::Array { .. }
                            | StatementKind::ConstantArray { .. }
                            | StatementKind::Map { .. }
                            | StatementKind::ConstantMap { .. }
                            | StatementKind::FunctionDeclaration { .. }
                    )
                });
                if needs_scope {
                    self.push_scope();
                }
                let body = self.resolve_statements(body);
                if needs_scope {
                    self.pop_scope();
                }
                StatementKind::ConditionalBranch {
                    condition,
                    body,
                    needs_scope,
                }
            }

            StatementKind::Return(expr) => {
                StatementKind::Return(expr.map(|e| self.resolve_expression(e)))
            }
            StatementKind::Expression(expr) => {
                StatementKind::Expression(self.resolve_expression(expr))
            }

            StatementKind::DestructureDeclaration { bindings, value } => {
                let value = self.resolve_expression(value);
                let slots = bindings
                    .iter()
                    .map(|(_, name)| self.declare(name.clone()))
                    .collect();
                StatementKind::ResolvedDestructureDeclaration {
                    bindings,
                    slots,
                    value,
                }
            }

            StatementKind::Match { value, arms } => {
                let value = self.resolve_expression(value);
                let arms = arms
                    .into_iter()
                    .map(|(pattern, body)| {
                        self.push_scope();
                        let body = self.resolve_statements(body);
                        self.pop_scope();
                        (pattern, body)
                    })
                    .collect();
                StatementKind::Match { value, arms }
            }

            other => other,
        };
        Statement::new(kind, span)
    }
}
