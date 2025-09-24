use crate::{errors::{CompilerError}, schemas::{Ast, BinOpKind, Expr, Identifier, Primitive, Stmt}};
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

    fn infer_binop_type_ctx(
        op: &BinOpKind,
        left_type: &Primitive,
        right_type: &Primitive,
    ) -> Result<Primitive, CompilerError> {
        match (op, left_type, right_type) {
            (BinOpKind::Add | BinOpKind::Sub | BinOpKind::Mult, Primitive::Int, Primitive::Int) => {
                Ok(Primitive::Int)
            }

            (BinOpKind::Div, Primitive::Int, Primitive::Int) => Ok(Primitive::Float),

            (
                BinOpKind::Add | BinOpKind::Sub | BinOpKind::Mult | BinOpKind::Div,
                Primitive::Float,
                Primitive::Int,
            )
            | (
                BinOpKind::Add | BinOpKind::Sub | BinOpKind::Mult | BinOpKind::Div,
                Primitive::Int,
                Primitive::Float,
            )
            | (
                BinOpKind::Add | BinOpKind::Sub | BinOpKind::Mult | BinOpKind::Div,
                Primitive::Float,
                Primitive::Float,
            ) => Ok(Primitive::Float),

            _ => Err(CompilerError::TypeBinOp {
                    op: op.clone(),
                    left: left_type.clone(),
                    right: right_type.clone(),
                    col: 0,
                    pos: 0,
                })
        }
    }

    fn check_type_declaration(declared_type: &Primitive, expr_type: &Primitive) -> Result<(), CompilerError> {
        if declared_type != expr_type {
            return Err(CompilerError::TypeDeclaration {
                expected: declared_type.clone(),
                found: expr_type.clone(),
                col: 0,
                pos: 0,
            });
        }
        Ok(())
    }

    fn check_expr(expr: &Expr, symbol_table: &HashMap<String, Identifier>) -> Result<Primitive, CompilerError> {
        match expr {
            Expr::Literal(literal) => {
                return Ok(literal.primitive.clone());
            }
            Expr::Identifier(ident_name) => {
                match symbol_table.get(ident_name) {
                    Some(identifier) => return Ok(identifier.primitive.clone()),
                    None => Err(CompilerError::Name { name: ident_name.to_string(), col: 0, pos: 0 }),
                }
            }
            Expr::BinOp { op, left, right } => {
                let left_type = Self::check_expr(left, symbol_table)?;
                let right_type = Self::check_expr(right, symbol_table)?;

                match Self::infer_binop_type_ctx(&op, &left_type, &right_type) {
                    Ok(infered_type) => Ok(infered_type),
                    Err(err) => Err(err),
                }
            }
        }
    }

    fn check_stmt(stmt: &Stmt, symbol_table: &mut HashMap<String, Identifier>) -> Result<(), CompilerError> {
        match stmt {
            Stmt::Declare { dtype, name, expr } => {
                symbol_table.insert(
                    name.to_string(),
                    Identifier {
                        primitive: dtype.clone(),
                    },
                );
                let expr_type = Self::check_expr(expr, symbol_table)?;
                match Self::check_type_declaration(dtype, &expr_type) {
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
