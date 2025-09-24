use crate::schemas::{BinOpKind, Primitive, UnaryOpKind};
use std::fmt;

#[derive(Debug)]
pub enum CompilerError {
    Syntax {
        message: String,
        col: u32,
        pos: u32,
    },
    TypeDeclaration {
        expected: Primitive,
        found: Primitive,
        col: u32,
        pos: u32,
    },
    TypeBinOp {
        op: BinOpKind,
        left: Primitive,
        right: Primitive,
        col: u32,
        pos: u32,
    },
    TypeUnaryOp {
        op: UnaryOpKind,
        operand: Primitive,
        col: u32,
        pos: u32,
    },
    Name {
        name: String,
        col: u32,
        pos: u32,
    },
}

impl fmt::Display for CompilerError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CompilerError::Syntax { message, col, pos } => {
                write!(
                    f,
                    "SyntaxError (line {}, position {}): {}",
                    col, pos, message
                )
            }
            CompilerError::TypeDeclaration {
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
            CompilerError::TypeBinOp {
                op,
                left,
                right,
                col,
                pos,
            } => {
                write!(
                    f,
                    "TypeError (line {}, position {}): Cannot binary apply {:?} to '{:?}' and '{:?}'.",
                    col, pos, op, left, right
                )
            }
            CompilerError::TypeUnaryOp {
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
            CompilerError::Name {
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
