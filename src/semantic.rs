use crate::{
    errors::CompilerError,
    schemas::{Ast, BinOpKind, Expr, Identifier, Primitive, Stmt, UnaryOpKind},
};
use std::collections::HashMap;

pub struct SemanticAnalyser {
    ast: Ast,
    symbol_table: HashMap<String, Identifier>,
}

impl SemanticAnalyser {
    pub fn new(ast: Ast) -> Self {
        SemanticAnalyser {
            ast: ast,
            symbol_table: HashMap::new(),
        }
    }

    fn infer_binop_type(
        op: &BinOpKind,
        left_type: &Primitive,
        right_type: &Primitive,
    ) -> Result<Primitive, CompilerError> {
        match (op, left_type, right_type) {
            // Addition, subtraction and multiplication return int for int operands.
            (BinOpKind::Add | BinOpKind::Sub | BinOpKind::Mult, Primitive::Int, Primitive::Int) => {
                Ok(Primitive::Int)
            }

            // Division returns float for int operands.
            (BinOpKind::Div, Primitive::Int, Primitive::Int) => Ok(Primitive::Float),

            // Any airthmetic operation with one or more float operand returns float.
            (
                BinOpKind::Add | BinOpKind::Sub | BinOpKind::Mult | BinOpKind::Div,
                Primitive::Int | Primitive::Float,
                Primitive::Int | Primitive::Float,
            ) => Ok(Primitive::Float),

            // Boolean operation on bool operands return bool.
            (BinOpKind::And | BinOpKind::Or | BinOpKind::Not | BinOpKind::Eq | BinOpKind::Ne, Primitive::Bool, Primitive::Bool) => {
                Ok(Primitive::Bool)
            }

            // Comparison operations on int and float return bool.
            (
                BinOpKind::Gt
                | BinOpKind::Lt
                | BinOpKind::Ge
                | BinOpKind::Le
                | BinOpKind::Eq
                | BinOpKind::Ne,
                Primitive::Int | Primitive::Float,
                Primitive::Int | Primitive::Float,
            ) => Ok(Primitive::Bool),

            // Int and float can be assigned to each other, bool only to bool.
            (BinOpKind::Assign, left_type, right_type) => {
                if left_type == right_type {
                    return Ok(left_type.clone());
                }
                match (left_type, right_type) {
                    (Primitive::Int, Primitive::Int | Primitive::Float) => Ok(Primitive::Int),
                    (Primitive::Float, Primitive::Int | Primitive::Float) => Ok(Primitive::Float),
                    (Primitive::Bool, Primitive::Bool) => Ok(Primitive::Bool),
                    _ => Err(CompilerError::TypeDeclaration {
                        expected: left_type.clone(),
                        found: right_type.clone(),
                        col: 0,
                        pos: 0,
                    }),
                }
            }
            _ => Err(CompilerError::TypeBinOp {
                op: op.clone(),
                left: left_type.clone(),
                right: right_type.clone(),
                col: 0,
                pos: 0,
            }),
        }
    }

    fn infer_unaryop_type(
        op: &UnaryOpKind,
        operand_type: &Primitive,
    ) -> Result<Primitive, CompilerError> {
        match (op, operand_type) {
            // Unary negation (-) only valid on int or float
            (UnaryOpKind::Neg, Primitive::Int | Primitive::Float) => {
                Ok(operand_type.clone())
            }

            // Logical not (!) only valid on bool
            (UnaryOpKind::Not, Primitive::Bool) => Ok(Primitive::Bool),

            _ => Err(CompilerError::TypeUnaryOp {
                op: op.clone(),
                operand: operand_type.clone(),
                col: 0,
                pos: 0,
            }),
        }
    }

    fn check_expr(
        expr: &Expr,
        symbol_table: &HashMap<String, Identifier>,
    ) -> Result<Primitive, CompilerError> {
        match expr {
            Expr::Literal(literal) => {
                return Ok(literal.primitive.clone());
            }
            Expr::Identifier(ident_name) => match symbol_table.get(ident_name) {
                Some(identifier) => return Ok(identifier.primitive.clone()),
                None => Err(CompilerError::Name {
                    name: ident_name.to_string(),
                    col: 0,
                    pos: 0,
                }),
            },
            Expr::BinOp { op, left, right } => {
                let left_type = Self::check_expr(left, symbol_table)?;
                let right_type = Self::check_expr(right, symbol_table)?;

                match Self::infer_binop_type(&op, &left_type, &right_type) {
                    Ok(infered_type) => Ok(infered_type),
                    Err(err) => Err(err),
                }
            }
            Expr::UnaryOp { op, expr } => {
                let expr = Self::check_expr(expr, symbol_table)?;
                match Self::infer_unaryop_type(&op, &expr) {
                    Ok(infered_type) => Ok(infered_type),
                    Err(err) => Err(err),
                }
            },
        }
    }

    fn check_stmt(
        stmt: &Stmt,
        symbol_table: &mut HashMap<String, Identifier>,
    ) -> Result<(), CompilerError> {
        match stmt {
            Stmt::Declare { dtype, name, expr } => {
                symbol_table.insert(
                    name.to_string(),
                    Identifier {
                        primitive: dtype.clone(),
                    },
                );
                let expr_type = Self::check_expr(expr, symbol_table)?;
                match Self::infer_binop_type(&BinOpKind::Assign, dtype, &expr_type) {
                    Ok(_) => Ok(()),
                    Err(err) => return Err(err),
                }
            }
            Stmt::Print { expr } => {
                Self::check_expr(expr, symbol_table)?;
                Ok(())
            }
        }
    }

    pub fn check(&mut self) -> Result<(), CompilerError> {
        for stmt in &self.ast {
            match Self::check_stmt(&stmt, &mut self.symbol_table) {
                Ok(_) => (),
                Err(err) => return Err(err),
            }
        }
        Ok(())
    }

    pub fn get_symbol_table(&self) -> &HashMap<String, Identifier> {
        return &self.symbol_table;
    }
}
