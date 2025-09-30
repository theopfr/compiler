use crate::{
    errors::CompilerError,
    schemas::{Ast, BinOpKind, Expr, Identifier, Primitive, Span, Stmt, UnaryOpKind},
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
        span: &Span,
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
            (
                BinOpKind::And | BinOpKind::Or | BinOpKind::Not | BinOpKind::Eq | BinOpKind::Ne,
                Primitive::Bool,
                Primitive::Bool,
            ) => Ok(Primitive::Bool),

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
                    _ => Err(CompilerError::TypeDeclarationError {
                        expected: left_type.clone(),
                        found: right_type.clone(),
                        span: span.clone(),
                    }),
                }
            }
            _ => Err(CompilerError::TypeBinOpError {
                op: op.clone(),
                left: left_type.clone(),
                right: right_type.clone(),
                span: span.clone(),
            }),
        }
    }

    fn infer_unaryop_type(
        op: &UnaryOpKind,
        operand_type: &Primitive,
        span: &Span,
    ) -> Result<Primitive, CompilerError> {
        match (op, operand_type) {
            // Unary negation (-) only valid on int or float
            (UnaryOpKind::Neg, Primitive::Int | Primitive::Float) => Ok(operand_type.clone()),

            // Logical not (!) only valid on bool
            (UnaryOpKind::Not, Primitive::Bool) => Ok(Primitive::Bool),

            _ => Err(CompilerError::TypeUnaryOpError {
                op: op.clone(),
                operand: operand_type.clone(),
                span: span.clone(),
            }),
        }
    }

    fn check_expr(
        expr: &Expr,
        symbol_table: &HashMap<String, Identifier>,
    ) -> Result<Primitive, CompilerError> {
        match expr {
            Expr::Literal { primitive, .. } => {
                return Ok(primitive.clone());
            }
            Expr::Identifier { name, span } => match symbol_table.get(name) {
                Some(identifier) => return Ok(identifier.primitive.clone()),
                None => Err(CompilerError::NameError {
                    name: name.to_string(),
                    span: span.clone(),
                }),
            },
            Expr::BinOp {
                op,
                left,
                right,
                span,
            } => {
                let left_type = Self::check_expr(left, symbol_table)?;
                let right_type = Self::check_expr(right, symbol_table)?;

                match Self::infer_binop_type(&op, &left_type, &right_type, &span) {
                    Ok(infered_type) => Ok(infered_type),
                    Err(err) => Err(err),
                }
            }
            Expr::UnaryOp { op, expr, span } => {
                let expr = Self::check_expr(expr, symbol_table)?;
                match Self::infer_unaryop_type(&op, &expr, &span) {
                    Ok(infered_type) => Ok(infered_type),
                    Err(err) => Err(err),
                }
            }
        }
    }

    fn check_stmt(
        stmt: &Stmt,
        symbol_table: &mut HashMap<String, Identifier>,
    ) -> Result<(), CompilerError> {
        match stmt {
            Stmt::Declare {
                dtype,
                name,
                expr,
                span,
                mutable,
            } => {
                symbol_table.insert(
                    name.to_string(),
                    Identifier {
                        primitive: dtype.clone(),
                        span: span.clone(),
                        mutable: *mutable,
                    },
                );
                let expr_type = Self::check_expr(expr, symbol_table)?;
                match Self::infer_binop_type(&BinOpKind::Assign, dtype, &expr_type, span) {
                    Ok(_) => Ok(()),
                    Err(err) => return Err(err),
                }
            }
            Stmt::MutAssign { name, expr, span } => {
                let symbol = match symbol_table.get(name) {
                    Some(identifier) => identifier,
                    None => return Err(CompilerError::NameError {
                        name: name.to_string(),
                        span: span.clone(),
                    }),
                };

                if !symbol.mutable {
                    return Err(CompilerError::MutabilityError {
                        name: name.to_string(),
                        span: span.clone(),
                    })
                }

                let expr_type = Self::check_expr(expr, symbol_table)?;
                match Self::infer_binop_type(&BinOpKind::Assign, &symbol.primitive, &expr_type, span) {
                    Ok(_) => Ok(()),
                    Err(err) => return Err(err),
                }
            },
            Stmt::Print { expr, span: _ } => {
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{lexer::Lexer, parser::Parser};

    fn check(input: &str) -> Result<(), CompilerError> {
        let mut lexer = Lexer::new(&(input.to_owned() + "\0"));
        lexer.tokenize()?;

        let mut parser = Parser::new(lexer.get_tokens().to_vec());
        parser.parse()?;
        let mut parser = SemanticAnalyser::new(parser.get_tree().to_vec());
        parser.check()?;

        Ok(())
    }

    #[test]
    fn test_correct_program_analysis() {
        check(
            "
            int a = (1 * (2 + 3));
            float b = -a / 5;
            print(b + 3);
            float c = 0.00001;
            float d = a + c;
            float e = a;
            bool b1 = 2 == 2;
            bool b2 = !(true && (2 > 0.5)) || b1 != (e <= 200);
            print(true && b2);
            \0
        ",
        )
        .unwrap();
    }

    #[test]
    fn test_assigning_float_to_int_var() {
        check("int a = 0.5;\0").unwrap();
    }

    #[test]
    fn test_assigning_int_to_float_var() {
        check("float a = 200;\0").unwrap();
    }

    #[test]
    fn test_arithm_binop_between_int_and_float() {
        check("int a = 0.5 * -200;\0").unwrap();
        check("float a = 0.5 * -200;\0").unwrap();
    }

    #[test]
    fn test_assigning_bool_to_int_and_float_var() {
        let result = check("int a = 200 == 200;\0");
        assert!(matches!(
            result,
            Err(CompilerError::TypeDeclarationError { .. })
        ));

        let result = check("float b = !false;\0");
        assert!(matches!(
            result,
            Err(CompilerError::TypeDeclarationError { .. })
        ));
    }

    #[test]
    fn test_assigning_int_and_float_to_bool_var() {
        let result = check("bool b = 200 - 200;\0");
        assert!(matches!(
            result,
            Err(CompilerError::TypeDeclarationError { .. })
        ));

        let result = check("bool b = 0.02;\0");
        assert!(matches!(
            result,
            Err(CompilerError::TypeDeclarationError { .. })
        ));
    }

    #[test]
    fn test_numeric_comparison_binop() {
        check(
            "
            bool a = 0.5 > -200;
            bool b = !(10 == 11);
            bool c = 10 <= 11;
            bool d = 10 != 11;
            \0
        ",
        )
        .unwrap();
    }

    #[test]
    fn test_boolean_comparison_binop() {
        check(
            "
            bool a = true == true;
            bool b = true != false;
            \0
        ",
        )
        .unwrap();
    }

    #[test]
    fn test_mutable_reassign() {
        check(
            "
            mut int a = 1;
            int b = 2;
            a = (5 * b) / 3;
            \0
        ",
        )
        .unwrap();
    }

    #[test]
    fn test_immutable_reassign_() {
        let result = check(
            "
            int a = 1;
            a = 2;
            \0
        ",
        );
        assert!(matches!(result, Err(CompilerError::MutabilityError { .. })));
    }

    #[test]
    fn test_boolean_binop_between_bool_and_int() {
        let result = check("int a = 1 && true;\0");
        assert!(matches!(result, Err(CompilerError::TypeBinOpError { .. })));

        let result = check("bool b = 1 != true;\0");
        assert!(matches!(result, Err(CompilerError::TypeBinOpError { .. })));

        let result = check("int a = false || 4;\0");
        assert!(matches!(result, Err(CompilerError::TypeBinOpError { .. })));
    }

    #[test]
    fn test_cmp_binop_between_bool_and_int() {
        let result = check("int a = 1 > true;\0");
        assert!(matches!(result, Err(CompilerError::TypeBinOpError { .. })));

        let result = check("bool b = 1 != (true <= false);\0");
        assert!(matches!(result, Err(CompilerError::TypeBinOpError { .. })));
    }

    #[test]
    fn test_arithm_unaryop() {
        check("int a = -2 * +-+-+(-+-4.0);\0").unwrap();

        let result = check("bool a = -false;\0");
        assert!(matches!(
            result,
            Err(CompilerError::TypeUnaryOpError { .. })
        ));
    }

    #[test]
    fn test_boolean_unaryop() {
        check("bool b = !true && !!(!!false);\0").unwrap();

        let result = check("int a = 200;int b = !a;\0");
        assert!(matches!(
            result,
            Err(CompilerError::TypeUnaryOpError { .. })
        ));
    }
}
