use crate::schemas::{BinOpKind, Primitive, UnaryOpKind};
use std::fmt;

#[derive(Debug)]
pub enum CompilerError {
    SyntaxError {
        message: String,
        col: u32,
        pos: u32,
    },
    TypeDeclarationError {
        expected: Primitive,
        found: Primitive,
        col: u32,
        pos: u32,
    },
    TypeBinOpError {
        op: BinOpKind,
        left: Primitive,
        right: Primitive,
        col: u32,
        pos: u32,
    },
    TypeUnaryOpError {
        op: UnaryOpKind,
        operand: Primitive,
        col: u32,
        pos: u32,
    },
    NameError {
        name: String,
        col: u32,
        pos: u32,
    },
}

impl fmt::Display for CompilerError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CompilerError::SyntaxError { message, col, pos } => {
                write!(
                    f,
                    "SyntaxError (line {}, position {}): {}",
                    col, pos, message
                )
            }
            CompilerError::TypeDeclarationError {
                expected,
                found,
                col,
                pos,
            } => {
                write!(
                    f,
                    "TypeError (line {}, position {}): Expected '{:?}', found '{:?}'.",
                    col, pos, expected, found
                )
            }
            CompilerError::TypeBinOpError {
                op,
                left,
                right,
                col,
                pos,
            } => {
                write!(
                    f,
                    "TypeError (line {}, position {}): Cannot apply binary operation '{:?}' to '{:?}' and '{:?}'.",
                    col, pos, op, left, right
                )
            }
            CompilerError::TypeUnaryOpError {
                op,
                operand,
                col,
                pos,
            } => {
                write!(
                    f,
                    "TypeError (line {}, position {}): Cannot apply unary operation '{:?}' to '{:?}''.",
                    col, pos, op, operand
                )
            }
            CompilerError::NameError {
                name,
                col,
                pos,
            } => {
                write!(
                    f,
                    "NameError (line {}, position {}): Cannot find identifier '{}'.",
                    col, pos, name
                )
            },
        }
    }
}
