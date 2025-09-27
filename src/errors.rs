use crate::schemas::{BinOpKind, Primitive, Span, UnaryOpKind};
use std::fmt;

#[derive(Debug)]
pub enum CompilerError {
    SyntaxError {
        message: String,
        span: Span,
    },
    TypeDeclarationError {
        expected: Primitive,
        found: Primitive,
        span: Span,
    },
    TypeBinOpError {
        op: BinOpKind,
        left: Primitive,
        right: Primitive,
        span: Span,
    },
    TypeUnaryOpError {
        op: UnaryOpKind,
        operand: Primitive,
        span: Span,
    },
    NameError {
        name: String,
        span: Span,
    },
}

impl fmt::Display for CompilerError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CompilerError::SyntaxError { message, span } => {
                write!(
                    f,
                    "SyntaxError (line {}, position {}): {}",
                    span.line, span.col, message
                )
            }
            CompilerError::TypeDeclarationError {
                expected,
                found,
                span,
            } => {
                write!(
                    f,
                    "TypeError (line {}, position {}): Expected '{:?}', found '{:?}'.",
                    span.line, span.col, expected, found
                )
            }
            CompilerError::TypeBinOpError {
                op,
                left,
                right,
                span,
            } => {
                write!(
                    f,
                    "TypeError (line {}, position {}): Cannot apply binary operation '{:?}' to '{:?}' and '{:?}'.",
                    span.line, span.col, op, left, right
                )
            }
            CompilerError::TypeUnaryOpError {
                op,
                operand,
                span,
            } => {
                write!(
                    f,
                    "TypeError (line {}, position {}): Cannot apply unary operation '{:?}' to '{:?}''.",
                    span.line, span.col, op, operand
                )
            }
            CompilerError::NameError {
                name,
                span
            } => {
                write!(
                    f,
                    "NameError (line {}, position {}): Cannot find identifier '{}'.",
                    span.line, span.col, name
                )
            },
        }
    }
}
