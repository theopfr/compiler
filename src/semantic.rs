use crate::schemas::{Ast, BinOpKind, Expr, Identifier, Primitive, Stmt};
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
    ) -> Result<Primitive, String> {
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

            (BinOpKind::Add, Primitive::Str, Primitive::Str) => Ok(Primitive::Str),

            _ => Err(format!(
                "Cannot apply {:?} to {:?} and {:?}",
                op, left_type, right_type
            )),
        }
    }

    fn check_type_declaration(declared_type: &Primitive, expr_type: &Primitive) {
        if !matches!(declared_type, Primitive::Auto) && declared_type != expr_type {
            panic!(
                "Mismatched types. Expected '{:?}', found '{:?}'.",
                declared_type, expr_type
            );
        }
    }

    fn check_expr(expr: &Expr, symbol_table: &HashMap<String, Identifier>) -> Primitive {
        match expr {
            Expr::Literal(literal) => {
                return literal.primitive.clone();
            }
            Expr::Identifier(ident_name) => {
                match symbol_table.get(ident_name) {
                    Some(identifier) => return identifier.primitive.clone(),
                    None => panic!("Cannot find identifier '{}'.", ident_name),
                }
            }
            Expr::BinOp { op, left, right } => {
                let left_type = Self::check_expr(left, symbol_table);
                let right_type = Self::check_expr(right, symbol_table);

                match Self::infer_binop_type_ctx(&op, &left_type, &right_type) {
                    Ok(infered_type) => infered_type,
                    Err(err) => panic!("{}", err),
                }
            }
        }
    }

    fn check_stmt(stmt: &Stmt, symbol_table: &mut HashMap<String, Identifier>) {
        match stmt {
            Stmt::Declare { dtype, name, expr } => {
                symbol_table.insert(
                    name.to_string(),
                    Identifier {
                        primitive: dtype.clone(),
                    },
                );
                let expr_type = Self::check_expr(expr, symbol_table);
                Self::check_type_declaration(dtype, &expr_type);
            }
            Stmt::Print { expr } => {
                Self::check_expr(expr, symbol_table);
            }
            s => panic!("Unexpected statement {:?}", s),
        }
    }

    pub fn check(&mut self) {
        for stmt in &self.ast {
            Self::check_stmt(&stmt, &mut self.symbol_table);
        }
    }

    pub fn get_symbol_table(&self) -> &HashMap<String, Identifier> {
        return &self.symbol_table;
    }
}
