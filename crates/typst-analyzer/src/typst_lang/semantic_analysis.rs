//! # semantic analysis
//!
//! semantic analysis for typst lang, utilizing concepts like symbol tables,
//! scopes, and type checking. It processes an abstract syntax tree (AST) to
//! ensure semantic correctness (e.g., variables are defined before use, types
//! are consistent).

use thiserror::Error;

use super::span::Span;

#[derive(Error, Debug)]
pub enum SemanticError {
    #[error("Undefined variable {name}")]
    UndefinedVariable { name: String, span: Span },
    #[error("Expect element type: {expect_type}, but got {actual_type}")]
    ImConsistentArrayType {
        expect_type: String,
        actual_type: String,
        span: Span,
    },
}

impl SemanticError {
    pub fn span(&self) -> Span {
        match self {
            SemanticError::UndefinedVariable { span, .. } => span.clone(),
            SemanticError::ImConsistentArrayType { span, .. } => span.clone(),
        }
    }
}

pub type Result<T> = std::result::Result<T, SemanticError>;

pub enum IndentType {}
