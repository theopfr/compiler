use std::f32::INFINITY;
use crate::{errors::CompilerError, schemas::*};

pub struct Parser {
    tokens: Vec<Token>,
    tree: Ast,
}

impl Parser {
    pub fn new(mut tokens: Vec<Token>) -> Self {
        tokens.reverse();

        Parser {
            tokens: tokens,
            tree: vec![],
        }
    }

    fn peek_next(&self) -> Token {
        self.tokens.last().cloned().unwrap_or(Token {
            kind: TokenKind::EOF,
            pos: self.tokens.len(),
        })
    }

    fn consume_next(&mut self) -> Token {
        self.tokens.pop().unwrap_or(Token {
            kind: TokenKind::EOF,
            pos: self.tokens.len(),
        })
    }

    fn parse_expression(&mut self, min_binding_pow: f32) -> Result<Expr, CompilerError> {
        let mut lhs = match &self.consume_next().kind {
            TokenKind::Literal(literal) => Expr::Literal(literal.clone()),
            TokenKind::Identifier(name) => Expr::Identifier(name.to_string()),

            // Handles unary '-' sign.
            TokenKind::BinOp(BinOpKind::Sub) => Expr::UnaryOp {
                op: UnaryOpKind::Neg,
                expr: Box::new(self.parse_expression(INFINITY)?),
            },

            // Handle unary '-' sign.
            TokenKind::BinOp(BinOpKind::Add) => self.parse_expression(INFINITY)?,

            // Handle unary '!' (boolean negation).
            TokenKind::BinOp(BinOpKind::Not) => Expr::UnaryOp {
                op: UnaryOpKind::Not,
                expr: Box::new(self.parse_expression(INFINITY)?),
            },

            // Handle expression in parentheses.
            TokenKind::LParen => {
                let expr = self.parse_expression(0.0)?;
                if !matches!(self.peek_next().kind, TokenKind::RParen) {
                    return Err(CompilerError::Syntax {
                        message: "Expected closing ')'.".to_string(),
                        col: 0,
                        pos: 0,
                    });
                }
                self.consume_next();
                expr
            }
            t => {
                return Err(CompilerError::Syntax {
                    message: format!("Unexpected token {:?}.", t),
                    col: 0,
                    pos: 0,
                });
            }
        };

        loop {
            let next_op = self.peek_next();

            match &next_op.kind {
                TokenKind::BinOp(op) => {
                    let (lbp, rbp) = Self::airthmetic_binding_power(&op);
                    if lbp < min_binding_pow {
                        break;
                    }

                    let op_clone = op.clone();
                    let _ = self.consume_next();

                    lhs = Expr::BinOp {
                        op: op_clone,
                        left: Box::new(lhs),
                        right: Box::new(self.parse_expression(rbp.clone())?),
                    };
                }
                TokenKind::EOS => break,
                TokenKind::EOF => break,
                TokenKind::RParen => break,
                t => {
                    return Err(CompilerError::Syntax {
                        message: format!("Unexpected token {:?}.", t),
                        col: 0,
                        pos: 0,
                    });
                }
            };
        }

        Ok(lhs)
    }

    fn airthmetic_binding_power(op: &BinOpKind) -> (f32, f32) {
        match op {
            BinOpKind::Mult | BinOpKind::Div => (6.1, 6.2),
            BinOpKind::Add | BinOpKind::Sub => (5.1, 5.2),
            BinOpKind::Gt | BinOpKind::Lt | BinOpKind::Ge | BinOpKind::Le => (4.1, 4.2),
            BinOpKind::Eq | BinOpKind::Ne => (3.1, 3.2),
            BinOpKind::And => (2.1, 2.2),
            BinOpKind::Or => (1.1, 1.2),
            BinOpKind::Not => panic!("Unary!"),
            BinOpKind::Assign => panic!("Assign!"),
        }
    }

    fn parse_statement(&mut self) -> Result<Stmt, CompilerError> {
        match self.consume_next().clone().kind {
            TokenKind::Declare(ref primitive) => {
                // Check for identifier (ie. variable name)
                let identifer_name = match self.peek_next().clone().kind {
                    TokenKind::Identifier(name) => name,
                    t => {
                        return Err(CompilerError::Syntax {
                            message: format!("Unexpected token {:?}.", t),
                            col: 0,
                            pos: 0,
                        });
                    }
                };
                self.consume_next();

                // Check for assign token (ie. '=')
                if !matches!(self.peek_next().kind, TokenKind::BinOp(BinOpKind::Assign)) {
                    return Err(CompilerError::Syntax {
                        message: "Expected '=' after declaration.".to_string(),
                        col: 0,
                        pos: 0,
                    });
                }
                self.consume_next();

                Ok(Stmt::Declare {
                    dtype: primitive.clone(),
                    name: identifer_name.clone(),
                    expr: self.parse_expression(0.0)?,
                })
            }
            TokenKind::Print => {
                if !matches!(self.peek_next().kind, TokenKind::LParen) {
                    return Err(CompilerError::Syntax {
                        message: "Expected opening '(' after 'print' keyword.".to_string(),
                        col: 0,
                        pos: 0,
                    });
                }
                self.consume_next();
                let expr = self.parse_expression(0.0)?;
                if !matches!(self.peek_next().kind, TokenKind::RParen) {
                    return Err(CompilerError::Syntax {
                        message: "Expected closing ')'.".to_string(),
                        col: 0,
                        pos: 0,
                    });
                }
                self.consume_next();

                Ok(Stmt::Print { expr })
            }
            k => Err(CompilerError::Syntax {
                message: format!("Unexpected token of kind {:?}.", k),
                col: 0,
                pos: 0,
            }),
        }
    }

    pub fn parse(&mut self) -> Result<(), CompilerError> {
        while !matches!(self.peek_next().kind, TokenKind::EOF) {
            let stmt = self.parse_statement()?;
            if !matches!(self.peek_next().kind, TokenKind::EOS) {
                return Err(CompilerError::Syntax {
                    message: "Expected ';' at end of expression.".to_string(),
                    col: 0,
                    pos: 0,
                });
            }

            self.tree.push(stmt);

            if matches!(self.peek_next().kind, TokenKind::EOS) {
                self.consume_next();
            }
        }

        Ok(())
    }

    pub fn get_tree(&self) -> &Ast {
        &self.tree
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::lexer::Lexer;

    fn parse(input: &str) -> Result<Ast, CompilerError> {
        let mut lexer = Lexer::new(&(input.to_owned() + "\0"));
        lexer.tokenize()?; // propagate error

        let mut parser = Parser::new(lexer.get_tokens().to_vec());
        parser.parse()?; // propagate error

        Ok(parser.get_tree().to_vec())
    }

    #[test]
    fn test_simple_statement() {
        let ast = parse("int a = 1 + 2;").unwrap();
        assert_eq!(
            ast,
            [Stmt::Declare {
                dtype: Primitive::Int,
                name: "a".to_string(),
                expr: Expr::BinOp {
                    op: BinOpKind::Add,
                    left: Box::new(Expr::Literal(Literal {
                        value: "1".to_string(),
                        primitive: Primitive::Int
                    })),
                    right: Box::new(Expr::Literal(Literal {
                        value: "2".to_string(),
                        primitive: Primitive::Int
                    }))
                }
            }]
        );
    }

    #[test]
    fn test_left_side_precedence() {
        let ast = parse("float a = 1 * 2 + 3.5;").unwrap();
        assert_eq!(
            ast,
            [Stmt::Declare {
                dtype: Primitive::Float,
                name: "a".to_string(),
                expr: Expr::BinOp {
                    op: BinOpKind::Add,
                    left: Box::new(Expr::BinOp {
                        op: BinOpKind::Mult,
                        left: Box::new(Expr::Literal(Literal {
                            value: "1".to_string(),
                            primitive: Primitive::Int
                        })),
                        right: Box::new(Expr::Literal(Literal {
                            value: "2".to_string(),
                            primitive: Primitive::Int
                        }))
                    }),
                    right: Box::new(Expr::Literal(Literal {
                        value: "3.5".to_string(),
                        primitive: Primitive::Float
                    }))
                }
            }]
        );
    }

    #[test]
    fn test_right_side_precedence() {
        let ast = parse("float a = 0.3333 - 2 / 3;").unwrap();
        assert_eq!(
            ast,
            [Stmt::Declare {
                dtype: Primitive::Float,
                name: "a".to_string(),
                expr: Expr::BinOp {
                    op: BinOpKind::Sub,
                    left: Box::new(Expr::Literal(Literal {
                        value: "0.3333".to_string(),
                        primitive: Primitive::Float
                    })),
                    right: Box::new(Expr::BinOp {
                        op: BinOpKind::Div,
                        left: Box::new(Expr::Literal(Literal {
                            value: "2".to_string(),
                            primitive: Primitive::Int
                        })),
                        right: Box::new(Expr::Literal(Literal {
                            value: "3".to_string(),
                            primitive: Primitive::Int
                        }))
                    }),
                }
            }]
        );
    }

    #[test]
    fn test_unary_sign_operator() {
        let ast = parse("int res = -b * +3;").unwrap();
        assert_eq!(
            ast,
            [Stmt::Declare {
                dtype: Primitive::Int,
                name: "res".to_string(),
                expr: Expr::BinOp {
                    op: BinOpKind::Mult,
                    left: Box::new(Expr::UnaryOp {
                        op: UnaryOpKind::Neg,
                        expr: Box::new(Expr::Identifier("b".to_string()))
                    }),
                    right: Box::new(Expr::Literal(Literal {
                        value: "3".to_string(),
                        primitive: Primitive::Int
                    })),
                }
            }]
        );
    }

    #[test]
    fn test_simple_parentheses() {
        let ast = parse("int c = (1 + 2) * 3;").unwrap();
        assert_eq!(
            ast,
            [Stmt::Declare {
                dtype: Primitive::Int,
                name: "c".to_string(),
                expr: Expr::BinOp {
                    op: BinOpKind::Mult,
                    left: Box::new(Expr::BinOp {
                        op: BinOpKind::Add,
                        left: Box::new(Expr::Literal(Literal {
                            value: "1".to_string(),
                            primitive: Primitive::Int
                        })),
                        right: Box::new(Expr::Literal(Literal {
                            value: "2".to_string(),
                            primitive: Primitive::Int
                        })),
                    }),
                    right: Box::new(Expr::Literal(Literal {
                        value: "3".to_string(),
                        primitive: Primitive::Int
                    })),
                }
            }]
        );
    }

    #[test]
    fn test_nested_parentheses() {
        let ast = parse("float c = ((1 + a) * b) / (a - b);").unwrap();
        assert_eq!(
            ast,
            [Stmt::Declare {
                dtype: Primitive::Float,
                name: "c".to_string(),
                expr: Expr::BinOp {
                    op: BinOpKind::Div,
                    left: Box::new(Expr::BinOp {
                        op: BinOpKind::Mult,
                        left: Box::new(Expr::BinOp {
                            op: BinOpKind::Add,
                            left: Box::new(Expr::Literal(Literal {
                                value: "1".to_string(),
                                primitive: Primitive::Int
                            })),
                            right: Box::new(Expr::Identifier("a".to_string())),
                        }),
                        right: Box::new(Expr::Identifier("b".to_string())),
                    }),
                    right: Box::new(Expr::BinOp {
                        op: BinOpKind::Sub,
                        left: Box::new(Expr::Identifier("a".to_string())),
                        right: Box::new(Expr::Identifier("b".to_string())),
                    }),
                }
            }]
        );
    }

    #[test]
    fn test_print_statement() {
        let ast = parse("print(1 * b);").unwrap();
        assert_eq!(
            ast,
            [Stmt::Print {
                expr: Expr::BinOp {
                    op: BinOpKind::Mult,
                    left: Box::new(Expr::Literal(Literal {
                        value: "1".to_string(),
                        primitive: Primitive::Int
                    })),
                    right: Box::new(Expr::Identifier("b".to_string()))
                }
            }]
        );
    }

    #[test]
    fn test_print_statement_with_parentheses() {
        let ast = parse("print((1 - b) * c);").unwrap();
        assert_eq!(
            ast,
            [Stmt::Print {
                expr: Expr::BinOp {
                    op: BinOpKind::Mult,
                    left: Box::new(Expr::BinOp {
                        op: BinOpKind::Sub,
                        left: Box::new(Expr::Literal(Literal {
                            value: "1".to_string(),
                            primitive: Primitive::Int
                        })),
                        right: Box::new(Expr::Identifier("b".to_string()))
                    }),
                    right: Box::new(Expr::Identifier("c".to_string()))
                }
            }]
        );
    }

    #[test]
    fn test_boolean_statement() {
        let ast = parse("bool a = true || (b >= 4);").unwrap();
        assert_eq!(
            ast,
            [Stmt::Declare {
                dtype: Primitive::Bool,
                name: "a".to_string(),
                expr: Expr::BinOp {
                    op: BinOpKind::Or,
                    left: Box::new(Expr::Literal(Literal {
                        value: "true".to_string(),
                        primitive: Primitive::Bool
                    })),
                    right: Box::new(Expr::BinOp {
                        op: BinOpKind::Ge,
                        left: Box::new(Expr::Identifier("b".to_string())),
                        right: Box::new(Expr::Literal(Literal {
                            value: "4".to_string(),
                            primitive: Primitive::Int
                        })),
                    })
                }
            }]
        );
    }

    #[test]
    fn test_logical_not_unary_operation() {
        let ast = parse("bool a = !(true && !b);").unwrap();
        assert_eq!(
            ast,
            [Stmt::Declare {
                dtype: Primitive::Bool,
                name: "a".to_string(),
                expr: Expr::UnaryOp {
                    op: UnaryOpKind::Not,
                    expr: Box::new(Expr::BinOp {
                        op: BinOpKind::And,
                        left: Box::new(Expr::Literal(Literal {
                            value: "true".to_string(),
                            primitive: Primitive::Bool
                        })),
                        right: Box::new(Expr::UnaryOp {
                            op: UnaryOpKind::Not,
                            expr: Box::new(Expr::Identifier("b".to_string()))
                        })
                    })
                }
            }]
        );
    }

    #[test]
    fn test_boolean_precedence() {
        let ast = parse("bool a = true || b >= 4 && c == d != e;").unwrap();
        assert_eq!(
            ast,
            [Stmt::Declare {
                dtype: Primitive::Bool,
                name: "a".to_string(),
                expr: Expr::BinOp {
                    op: BinOpKind::Or,
                    left: Box::new(Expr::Literal(Literal {
                        value: "true".to_string(),
                        primitive: Primitive::Bool
                    })),
                    right: Box::new(Expr::BinOp {
                        op: BinOpKind::And,
                        left: Box::new(Expr::BinOp {
                            op: BinOpKind::Ge,
                            left: Box::new(Expr::Identifier("b".to_string())),
                            right: Box::new(Expr::Literal(Literal {
                                value: "4".to_string(),
                                primitive: Primitive::Int
                            })),
                        }),
                        right: Box::new(Expr::BinOp {
                            op: BinOpKind::Ne,
                            left: Box::new(Expr::BinOp {
                                op: BinOpKind::Eq,
                                left: Box::new(Expr::Identifier("c".to_string())),
                                right: Box::new(Expr::Identifier("d".to_string())),
                            }),
                            right: Box::new(Expr::Identifier("e".to_string())),
                        })
                    })
                }
            }]
        );
    }

    #[test]
    fn test_missing_eos_semicolon() {
        let result = parse("int a = 0 print(a);");
        assert!(matches!(result, Err(CompilerError::Syntax { .. })));
    }
}
