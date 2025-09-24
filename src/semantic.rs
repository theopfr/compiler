use std::{collections::HashMap};
use crate::schemas::{Ast, Expr, Identifier, Primitive, Stmt};

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

    fn check_type_ctx(type_ctx: &mut Primitive, cur_type: &Primitive) {
        if let Primitive::Auto = type_ctx {
            *type_ctx = cur_type.clone();
            return;
        }
        if type_ctx != cur_type {
            panic!(
                "Mismatched types. Expected '{:?}', found '{:?}'.",
                type_ctx, cur_type
            );
        }
    }

    fn check_expr(expr: &Expr, type_ctx: &mut Primitive, symbol_table: &HashMap<String, Identifier>) {
        match expr {
            Expr::Literal(literal) => {
                Self::check_type_ctx(type_ctx, &literal.primitive);
            }
            Expr::Identifier(ident_name) => {
                if let Some(identifier) = symbol_table.get(ident_name) {
                    Self::check_type_ctx(type_ctx, &identifier.primitive);
                    return;
                }
                panic!("Cannot find identifier '{}'.", ident_name);
            }
            Expr::BinOp { op: _, left, right } => {
                Self::check_expr(left, type_ctx, symbol_table);
                Self::check_expr(right, type_ctx, symbol_table);
            }
        }
    }

    fn check_stmt(stmt: &Stmt, symbol_table: &mut HashMap<String, Identifier>) {
        let mut type_ctx = Primitive::Auto;

        match stmt {
            Stmt::Declare { dtype, name, expr } => {
                type_ctx = dtype.clone();
                symbol_table.insert(name.to_string(), Identifier {
                    primitive: type_ctx.clone(),
                });
                Self::check_expr(expr, &mut type_ctx, symbol_table);
            }
            Stmt::Print { expr } => {
                Self::check_expr(expr, &mut type_ctx, symbol_table);
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
