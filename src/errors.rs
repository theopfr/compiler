use crate::schemas::{BinOpKind, Primitive, UnaryOpKind};
use std::fmt;

#[derive(Debug)]
pub enum CompilerError {
    SyntaxError {
        message: String,
        line: usize,
        col: usize,
    },
    TypeDeclarationError {
        expected: Primitive,
        found: Primitive,
        line: usize,
        col: usize,
    },
    TypeBinOpError {
        op: BinOpKind,
        left: Primitive,
        right: Primitive,
        line: usize,
        col: usize,
    },
    TypeUnaryOpError {
        op: UnaryOpKind,
        operand: Primitive,
        line: usize,
        col: usize,
    },
    NameError {
        name: String,
        line: usize,
        col: usize,
    },
}

impl fmt::Display for CompilerError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CompilerError::SyntaxError { message, line, col } => {
                write!(
                    f,
                    "SyntaxError (line {}, position {}): {}",
                    line, col, message
                )
            }
            CompilerError::TypeDeclarationError {
                expected,
                found,
                line,
                col,
            } => {
                write!(
                    f,
                    "TypeError (line {}, position {}): Expected '{:?}', found '{:?}'.",
                    line, col, expected, found
                )
            }
            CompilerError::TypeBinOpError {
                op,
                left,
                right,
                line,
                col,
            } => {
                write!(
                    f,
                    "TypeError (line {}, position {}): Cannot apply binary operation '{:?}' to '{:?}' and '{:?}'.",
                    line, col, op, left, right
                )
            }
            CompilerError::TypeUnaryOpError {
                op,
                operand,
                line,
                col,
            } => {
                write!(
                    f,
                    "TypeError (line {}, position {}): Cannot apply unary operation '{:?}' to '{:?}''.",
                    line, col, op, operand
                )
            }
            CompilerError::NameError {
                name,
                line,
                col,
            } => {
                write!(
                    f,
                    "NameError (line {}, position {}): Cannot find identifier '{}'.",
                    line, col, name
                )
            },
        }
    }
}
