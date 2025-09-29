use crate::{errors::CompilerError, schemas::*};
use std::f32::INFINITY;

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
            span: Span { line: 0, col: 0 },
        })
    }

    fn consume_next(&mut self) -> Token {
        self.tokens.pop().unwrap_or(Token {
            kind: TokenKind::EOF,
            span: Span { line: 0, col: 0 },
        })
    }

    fn parse_expression(&mut self, min_binding_pow: f32) -> Result<Expr, CompilerError> {
        let cur_token = self.consume_next();
        let mut lhs = match cur_token.kind {
            TokenKind::Literal(literal) => Expr::Literal {
                literal: literal.clone(),
                span: cur_token.span,
            },
            TokenKind::Identifier(name) => Expr::Identifier {
                name: name.clone(),
                span: cur_token.span,
            },

            // Handles unary '-' sign.
            TokenKind::BinOp(BinOpKind::Sub) => Expr::UnaryOp {
                op: UnaryOpKind::Neg,
                expr: Box::new(self.parse_expression(INFINITY)?),
                span: cur_token.span,
            },

            // Handle unary '-' sign.
            TokenKind::BinOp(BinOpKind::Add) => self.parse_expression(INFINITY)?,

            // Handle unary '!' (boolean negation).
            TokenKind::BinOp(BinOpKind::Not) => Expr::UnaryOp {
                op: UnaryOpKind::Not,
                expr: Box::new(self.parse_expression(INFINITY)?),
                span: cur_token.span,
            },

            // Handle expression in parentheses.
            TokenKind::LParen => {
                let expr = self.parse_expression(0.0)?;

                let next_token = self.peek_next();
                if !matches!(next_token.kind, TokenKind::RParen) {
                    return Err(CompilerError::SyntaxError {
                        message: "Expected closing ')'.".to_string(),
                        span: next_token.span,
                    });
                }
                self.consume_next();
                expr
            }
            t => {
                return Err(CompilerError::SyntaxError {
                    message: format!("Unexpected token {:?}.", t),
                    span: cur_token.span,
                });
            }
        };

        loop {
            let next_op_token = self.peek_next();

            match &next_op_token.kind {
                TokenKind::BinOp(op) => {
                    let (lbp, rbp) = Self::airthmetic_binding_power(&op, &next_op_token.span)?;
                    if lbp < min_binding_pow {
                        break;
                    }

                    let op_clone = op.clone();
                    let _ = self.consume_next();

                    lhs = Expr::BinOp {
                        op: op_clone,
                        left: Box::new(lhs),
                        right: Box::new(self.parse_expression(rbp.clone())?),
                        span: next_op_token.span,
                    };
                }
                TokenKind::RParen => break,
                TokenKind::EOS => break,
                TokenKind::EOF => break,
                t => {
                    return Err(CompilerError::SyntaxError {
                        message: format!("Unexpected token {:?}.", t),
                        span: next_op_token.span,
                    });
                }
            };
        }

        Ok(lhs)
    }

    fn airthmetic_binding_power(binop_kind: &BinOpKind, span: &Span) -> Result<(f32, f32), CompilerError> {
        match binop_kind {
            BinOpKind::Mult | BinOpKind::Div => Ok((6.1, 6.2)),
            BinOpKind::Add | BinOpKind::Sub => Ok((5.1, 5.2)),
            BinOpKind::Gt | BinOpKind::Lt | BinOpKind::Ge | BinOpKind::Le => Ok((4.1, 4.2)),
            BinOpKind::Eq | BinOpKind::Ne => Ok((3.1, 3.2)),
            BinOpKind::And => Ok((2.1, 2.2)),
            BinOpKind::Or => Ok((1.1, 1.2)),
            t => Err(CompilerError::SyntaxError {
                message: format!("Unexpected token {:?}.", t),
                span: span.clone(),
            }),
        }
    }

    fn parse_statement(&mut self) -> Result<Stmt, CompilerError> {
        let cur_token = self.consume_next().clone();
        match cur_token.kind {
            TokenKind::Declare(ref primitive) => {
                let next_token = self.peek_next().clone();

                // Check for identifier (ie. variable name)
                let identifer_name = match next_token.kind {
                    TokenKind::Identifier(name) => name,
                    t => {
                        return Err(CompilerError::SyntaxError {
                            message: format!("Unexpected token {:?}.", t),
                            span: next_token.span,
                        });
                    }
                };
                self.consume_next();

                let next_token = self.peek_next();

                // Check for assign token (ie. '=')
                if !matches!(next_token.kind, TokenKind::BinOp(BinOpKind::Assign)) {
                    return Err(CompilerError::SyntaxError {
                        message: "Expected '=' after declaration.".to_string(),
                        span: next_token.span,
                    });
                }
                self.consume_next();

                Ok(Stmt::Declare {
                    dtype: primitive.clone(),
                    name: identifer_name.clone(),
                    expr: self.parse_expression(0.0)?,
                    span: Span {
                        line: cur_token.span.line,
                        col: cur_token.span.col,
                    },
                })
            }
            TokenKind::Print => {
                // Check for opening parenthese.
                let next_token = self.peek_next();
                if !matches!(next_token.kind, TokenKind::LParen) {
                    return Err(CompilerError::SyntaxError {
                        message: "Expected opening '(' after 'print' keyword.".to_string(),
                        span: next_token.span,
                    });
                }
                self.consume_next();

                // Processes expression inside print().
                let expr = self.parse_expression(0.0)?;

                // Check for closing parenthese.
                let next_token = self.peek_next();
                if !matches!(next_token.kind, TokenKind::RParen) {
                    return Err(CompilerError::SyntaxError {
                        message: "Expected closing ')'.".to_string(),
                        span: next_token.span,
                    });
                }
                self.consume_next();

                Ok(Stmt::Print {
                    expr,
                    span: Span {
                        line: cur_token.span.line,
                        col: cur_token.span.col,
                    },
                })
            }
            k => Err(CompilerError::SyntaxError {
                message: format!("Unexpected token of kind {:?}.", k),
                span: cur_token.span,
            }),
        }
    }

    pub fn parse(&mut self) -> Result<(), CompilerError> {
        while !matches!(self.peek_next().kind, TokenKind::EOF) {
            let stmt = self.parse_statement()?;

            let next_token = self.peek_next();
            match next_token.kind {
                TokenKind::EOS => {
                    self.consume_next();
                    self.tree.push(stmt);
                }
                TokenKind::RParen => {
                    return Err(CompilerError::SyntaxError {
                        message: "Unmatched ')'.".to_string(),
                        span: next_token.span,
                    });
                }
                _ => {
                    return Err(CompilerError::SyntaxError {
                        message: "Expected ';' at end of expression.".to_string(),
                        span: next_token.span,
                    });
                }
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
        lexer.tokenize()?;

        let mut parser = Parser::new(lexer.get_tokens().to_vec());
        parser.parse()?;

        Ok(parser.get_tree().to_vec())
    }

    fn ignore_spans_expr(expr: Expr) -> Expr {
        match expr {
            Expr::Literal { literal, .. } => Expr::Literal {
                literal,
                span: Span::default(),
            },
            Expr::Identifier { name, .. } => Expr::Identifier {
                name,
                span: Span::default(),
            },
            Expr::UnaryOp { op, expr, span: _ } => Expr::UnaryOp {
                op,
                expr: Box::new(ignore_spans_expr(*expr)),
                span: Span::default(),
            },
            Expr::BinOp {
                op,
                left,
                right,
                span: _,
            } => Expr::BinOp {
                op,
                left: Box::new(ignore_spans_expr(*left)),
                right: Box::new(ignore_spans_expr(*right)),
                span: Span::default(),
            },
        }
    }

    fn ignore_spans_stmt(stmt: Stmt) -> Stmt {
        match stmt {
            Stmt::Declare {
                dtype,
                name,
                expr,
                span: _,
            } => Stmt::Declare {
                dtype,
                name,
                expr: ignore_spans_expr(expr),
                span: Span::default(),
            },
            Stmt::Print { expr, span: _ } => Stmt::Print {
                expr: ignore_spans_expr(expr),
                span: Span::default(),
            },
        }
    }

    fn ignore_spans_ast(ast: Ast) -> Ast {
        ast.into_iter().map(ignore_spans_stmt).collect()
    }

    #[test]
    fn test_simple_statement() {
        let ast = parse("int a = 1 + 2;").unwrap();
        assert_eq!(
            ignore_spans_ast(ast),
            [Stmt::Declare {
                dtype: Primitive::Int,
                name: "a".to_string(),
                expr: Expr::BinOp {
                    op: BinOpKind::Add,
                    left: Box::new(Expr::Literal {
                        literal: Literal {
                            value: "1".to_string(),
                            primitive: Primitive::Int
                        },
                        span: Span::default()
                    }),
                    right: Box::new(Expr::Literal {
                        literal: Literal {
                            value: "2".to_string(),
                            primitive: Primitive::Int
                        },
                        span: Span::default()
                    }),
                    span: Span::default()
                },
                span: Span::default()
            }]
        );
    }

    #[test]
    fn test_left_side_precedence() {
        let ast = parse("float a = 1 * 2 + 3.5;").unwrap();
        assert_eq!(
            ignore_spans_ast(ast),
            [Stmt::Declare {
                dtype: Primitive::Float,
                name: "a".to_string(),
                expr: Expr::BinOp {
                    op: BinOpKind::Add,
                    left: Box::new(Expr::BinOp {
                        op: BinOpKind::Mult,
                        left: Box::new(Expr::Literal {
                            literal: Literal {
                                value: "1".to_string(),
                                primitive: Primitive::Int
                            },
                            span: Span::default()
                        }),
                        right: Box::new(Expr::Literal {
                            literal: Literal {
                                value: "2".to_string(),
                                primitive: Primitive::Int
                            },
                            span: Span::default()
                        }),
                        span: Span::default()
                    }),
                    right: Box::new(Expr::Literal {
                        literal: Literal {
                            value: "3.5".to_string(),
                            primitive: Primitive::Float
                        },
                        span: Span::default()
                    }),
                    span: Span::default()
                },
                span: Span::default()
            }]
        );
    }

    #[test]
    fn test_right_side_precedence() {
        let ast = parse("float a = 0.3333 - 2 / 3;").unwrap();
        assert_eq!(
            ignore_spans_ast(ast),
            [Stmt::Declare {
                dtype: Primitive::Float,
                name: "a".to_string(),
                expr: Expr::BinOp {
                    op: BinOpKind::Sub,
                    left: Box::new(Expr::Literal {
                        literal: Literal {
                            value: "0.3333".to_string(),
                            primitive: Primitive::Float
                        },
                        span: Span::default()
                    }),
                    right: Box::new(Expr::BinOp {
                        op: BinOpKind::Div,
                        left: Box::new(Expr::Literal {
                            literal: Literal {
                                value: "2".to_string(),
                                primitive: Primitive::Int
                            },
                            span: Span::default()
                        }),
                        right: Box::new(Expr::Literal {
                            literal: Literal {
                                value: "3".to_string(),
                                primitive: Primitive::Int
                            },
                            span: Span::default()
                        }),
                        span: Span::default()
                    }),
                    span: Span::default()
                },
                span: Span::default()
            }]
        );
    }

    #[test]
    fn test_unary_sign_operator() {
        let ast = parse("int res = -b * +3;").unwrap();
        assert_eq!(
            ignore_spans_ast(ast),
            [Stmt::Declare {
                dtype: Primitive::Int,
                name: "res".to_string(),
                expr: Expr::BinOp {
                    op: BinOpKind::Mult,
                    left: Box::new(Expr::UnaryOp {
                        op: UnaryOpKind::Neg,
                        expr: Box::new(Expr::Identifier {
                            name: "b".to_string(),
                            span: Span::default()
                        }),
                        span: Span::default()
                    }),
                    right: Box::new(Expr::Literal {
                        literal: Literal {
                            value: "3".to_string(),
                            primitive: Primitive::Int
                        },
                        span: Span::default()
                    }),
                    span: Span::default()
                },
                span: Span::default()
            }]
        );
    }

    #[test]
    fn test_simple_parentheses() {
        let ast = parse("int c = (1 + 2) * 3;").unwrap();
        assert_eq!(
            ignore_spans_ast(ast),
            [Stmt::Declare {
                dtype: Primitive::Int,
                name: "c".to_string(),
                expr: Expr::BinOp {
                    op: BinOpKind::Mult,
                    left: Box::new(Expr::BinOp {
                        op: BinOpKind::Add,
                        left: Box::new(Expr::Literal {
                            literal: Literal {
                                value: "1".to_string(),
                                primitive: Primitive::Int
                            },
                            span: Span::default()
                        }),
                        right: Box::new(Expr::Literal {
                            literal: Literal {
                                value: "2".to_string(),
                                primitive: Primitive::Int
                            },
                            span: Span::default()
                        }),
                        span: Span::default()
                    }),
                    right: Box::new(Expr::Literal {
                        literal: Literal {
                            value: "3".to_string(),
                            primitive: Primitive::Int
                        },
                        span: Span::default()
                    }),
                    span: Span::default()
                },
                span: Span::default()
            }]
        );
    }

    #[test]
    fn test_nested_parentheses() {
        let ast = parse("float c = ((1 + a) * b) / (a - b);").unwrap();
        assert_eq!(
            ignore_spans_ast(ast),
            [Stmt::Declare {
                dtype: Primitive::Float,
                name: "c".to_string(),
                expr: Expr::BinOp {
                    op: BinOpKind::Div,
                    left: Box::new(Expr::BinOp {
                        op: BinOpKind::Mult,
                        left: Box::new(Expr::BinOp {
                            op: BinOpKind::Add,
                            left: Box::new(Expr::Literal {
                                literal: Literal {
                                    value: "1".to_string(),
                                    primitive: Primitive::Int
                                },
                                span: Span::default()
                            }),
                            right: Box::new(Expr::Identifier {
                                name: "a".to_string(),
                                span: Span::default()
                            }),
                            span: Span::default()
                        }),
                        right: Box::new(Expr::Identifier {
                            name: "b".to_string(),
                            span: Span::default()
                        }),
                        span: Span::default()
                    }),
                    right: Box::new(Expr::BinOp {
                        op: BinOpKind::Sub,
                        left: Box::new(Expr::Identifier {
                            name: "a".to_string(),
                            span: Span::default()
                        }),
                        right: Box::new(Expr::Identifier {
                            name: "b".to_string(),
                            span: Span::default()
                        }),
                        span: Span::default()
                    }),
                    span: Span::default()
                },
                span: Span::default()
            }]
        );
    }

    #[test]
    fn test_print_statement() {
        let ast = parse("print(1 * b);").unwrap();
        assert_eq!(
            ignore_spans_ast(ast),
            [Stmt::Print {
                expr: Expr::BinOp {
                    op: BinOpKind::Mult,
                    left: Box::new(Expr::Literal {
                        literal: Literal {
                            value: "1".to_string(),
                            primitive: Primitive::Int
                        },
                        span: Span::default()
                    }),
                    right: Box::new(Expr::Identifier {
                        name: "b".to_string(),
                        span: Span::default()
                    }),
                    span: Span::default()
                },
                span: Span::default()
            }]
        );
    }

    #[test]
    fn test_print_statement_with_parentheses() {
        let ast = parse("print((1 - b) * c);").unwrap();
        assert_eq!(
            ignore_spans_ast(ast),
            [Stmt::Print {
                expr: Expr::BinOp {
                    op: BinOpKind::Mult,
                    left: Box::new(Expr::BinOp {
                        op: BinOpKind::Sub,
                        left: Box::new(Expr::Literal {
                            literal: Literal {
                                value: "1".to_string(),
                                primitive: Primitive::Int
                            },
                            span: Span::default()
                        }),
                        right: Box::new(Expr::Identifier {
                            name: "b".to_string(),
                            span: Span::default()
                        }),
                        span: Span::default()
                    }),
                    right: Box::new(Expr::Identifier {
                        name: "c".to_string(),
                        span: Span::default()
                    }),
                    span: Span::default()
                },
                span: Span::default()
            }]
        );
    }

    #[test]
    fn test_boolean_statement() {
        let ast = parse("bool a = true || (b >= 4);").unwrap();
        assert_eq!(
            ignore_spans_ast(ast),
            [Stmt::Declare {
                dtype: Primitive::Bool,
                name: "a".to_string(),
                expr: Expr::BinOp {
                    op: BinOpKind::Or,
                    left: Box::new(Expr::Literal {
                        literal: Literal {
                            value: "true".to_string(),
                            primitive: Primitive::Bool
                        },
                        span: Span::default()
                    }),
                    right: Box::new(Expr::BinOp {
                        op: BinOpKind::Ge,
                        left: Box::new(Expr::Identifier {
                            name: "b".to_string(),
                            span: Span::default()
                        }),
                        right: Box::new(Expr::Literal {
                            literal: Literal {
                                value: "4".to_string(),
                                primitive: Primitive::Int
                            },
                            span: Span::default()
                        }),
                        span: Span::default()
                    }),
                    span: Span::default()
                },
                span: Span::default()
            }]
        );
    }

    #[test]
    fn test_logical_not_unary_operation() {
        let ast = parse("bool a = !(true && !b);").unwrap();
        assert_eq!(
            ignore_spans_ast(ast),
            [Stmt::Declare {
                dtype: Primitive::Bool,
                name: "a".to_string(),
                expr: Expr::UnaryOp {
                    op: UnaryOpKind::Not,
                    expr: Box::new(Expr::BinOp {
                        op: BinOpKind::And,
                        left: Box::new(Expr::Literal {
                            literal: Literal {
                                value: "true".to_string(),
                                primitive: Primitive::Bool
                            },
                            span: Span::default()
                        }),
                        right: Box::new(Expr::UnaryOp {
                            op: UnaryOpKind::Not,
                            expr: Box::new(Expr::Identifier {
                                name: "b".to_string(),
                                span: Span::default()
                            }),
                            span: Span::default()
                        }),
                        span: Span::default()
                    }),
                    span: Span::default()
                },
                span: Span::default()
            }]
        );
    }

    #[test]
    fn test_boolean_precedence() {
        let ast = parse("bool a = true || b >= 4 && c == d != e;").unwrap();
        assert_eq!(
            ignore_spans_ast(ast),
            [Stmt::Declare {
                dtype: Primitive::Bool,
                name: "a".to_string(),
                expr: Expr::BinOp {
                    op: BinOpKind::Or,
                    left: Box::new(Expr::Literal {
                        literal: Literal {
                            value: "true".to_string(),
                            primitive: Primitive::Bool
                        },
                        span: Span::default()
                    }),
                    right: Box::new(Expr::BinOp {
                        op: BinOpKind::And,
                        left: Box::new(Expr::BinOp {
                            op: BinOpKind::Ge,
                            left: Box::new(Expr::Identifier {
                                name: "b".to_string(),
                                span: Span::default()
                            }),
                            right: Box::new(Expr::Literal {
                                literal: Literal {
                                    value: "4".to_string(),
                                    primitive: Primitive::Int
                                },
                                span: Span::default()
                            }),
                            span: Span::default()
                        }),
                        right: Box::new(Expr::BinOp {
                            op: BinOpKind::Ne,
                            left: Box::new(Expr::BinOp {
                                op: BinOpKind::Eq,
                                left: Box::new(Expr::Identifier {
                                    name: "c".to_string(),
                                    span: Span::default()
                                }),
                                right: Box::new(Expr::Identifier {
                                    name: "d".to_string(),
                                    span: Span::default()
                                }),
                                span: Span::default()
                            }),
                            right: Box::new(Expr::Identifier {
                                name: "e".to_string(),
                                span: Span::default()
                            }),
                            span: Span::default()
                        }),
                        span: Span::default()
                    }),
                    span: Span::default()
                },
                span: Span::default()
            }]
        );
    }

    #[test]
    fn test_bool_expr_without_whitespaces() {
        let ast = parse("bool a=true||b>=4&&c==d!=e;").unwrap();
        assert_eq!(
            ignore_spans_ast(ast),
            [Stmt::Declare {
                dtype: Primitive::Bool,
                name: "a".to_string(),
                expr: Expr::BinOp {
                    op: BinOpKind::Or,
                    left: Box::new(Expr::Literal {
                        literal: Literal {
                            value: "true".to_string(),
                            primitive: Primitive::Bool
                        },
                        span: Span::default()
                    }),
                    right: Box::new(Expr::BinOp {
                        op: BinOpKind::And,
                        left: Box::new(Expr::BinOp {
                            op: BinOpKind::Ge,
                            left: Box::new(Expr::Identifier {
                                name: "b".to_string(),
                                span: Span::default()
                            }),
                            right: Box::new(Expr::Literal {
                                literal: Literal {
                                    value: "4".to_string(),
                                    primitive: Primitive::Int
                                },
                                span: Span::default()
                            }),
                            span: Span::default()
                        }),
                        right: Box::new(Expr::BinOp {
                            op: BinOpKind::Ne,
                            left: Box::new(Expr::BinOp {
                                op: BinOpKind::Eq,
                                left: Box::new(Expr::Identifier {
                                    name: "c".to_string(),
                                    span: Span::default()
                                }),
                                right: Box::new(Expr::Identifier {
                                    name: "d".to_string(),
                                    span: Span::default()
                                }),
                                span: Span::default()
                            }),
                            right: Box::new(Expr::Identifier {
                                name: "e".to_string(),
                                span: Span::default()
                            }),
                            span: Span::default()
                        }),
                        span: Span::default()
                    }),
                    span: Span::default()
                },
                span: Span::default()
            }]
        );
    }

    #[test]
    fn test_arithm_expr_without_whitespaces() {
        let ast = parse("float c=((1+a)*b)/(a-b);").unwrap();
        assert_eq!(
            ignore_spans_ast(ast),
            [Stmt::Declare {
                dtype: Primitive::Float,
                name: "c".to_string(),
                expr: Expr::BinOp {
                    op: BinOpKind::Div,
                    left: Box::new(Expr::BinOp {
                        op: BinOpKind::Mult,
                        left: Box::new(Expr::BinOp {
                            op: BinOpKind::Add,
                            left: Box::new(Expr::Literal {
                                literal: Literal {
                                    value: "1".to_string(),
                                    primitive: Primitive::Int
                                },
                                span: Span::default()
                            }),
                            right: Box::new(Expr::Identifier {
                                name: "a".to_string(),
                                span: Span::default()
                            }),
                            span: Span::default()
                        }),
                        right: Box::new(Expr::Identifier {
                            name: "b".to_string(),
                            span: Span::default()
                        }),
                        span: Span::default()
                    }),
                    right: Box::new(Expr::BinOp {
                        op: BinOpKind::Sub,
                        left: Box::new(Expr::Identifier {
                            name: "a".to_string(),
                            span: Span::default()
                        }),
                        right: Box::new(Expr::Identifier {
                            name: "b".to_string(),
                            span: Span::default()
                        }),
                        span: Span::default()
                    }),
                    span: Span::default()
                },
                span: Span::default()
            }]
        );
    }

    #[test]
    fn test_missing_eos_semicolon() {
        let result = parse("int a = 0 print(a);");
        assert!(matches!(
            result,
            Err(CompilerError::SyntaxError { span, .. }) if span.line == 1 && span.col == 11
        ));
    }

    #[test]
    fn test_missing_closing_parenthese() {
        let result = parse("int a = ((5 + 4) / 4;");
        assert!(matches!(
            result,
            Err(CompilerError::SyntaxError { span, .. }) if span.line == 1 && span.col == 21
        ));
    }

    #[test]
    fn test_unmatched_closing_parenthese() {
        let result = parse("int a = 3;\nfloat b = a + 4) / 4;");
        assert!(matches!(
            result,
            Err(CompilerError::SyntaxError { span, .. }) if span.line == 2 && span.col == 16
        ));
    }

    #[test]
    fn test_missing_assign_operator_after_declaration() {
        let result = parse("int a = 3;\nfloat b a = 2;");
        assert!(matches!(
            result,
            Err(CompilerError::SyntaxError { span, .. }) if span.line == 2 && span.col == 9
        ));
    }

    #[test]
    fn test_missing_parentheses_after_print() {
        let result = parse("print a + 2;");
        assert!(matches!(
            result,
            Err(CompilerError::SyntaxError { span, .. }) if span.line == 1 && span.col == 7
        ));
    }

    #[test]
    fn test_unknown_statement_start_token() {
        let result = parse("let a = 2;"); // keyword 'let' doesn't exist
        assert!(matches!(
            result,
            Err(CompilerError::SyntaxError { span, .. }) if span.line == 1 && span.col == 1
        ));
    }

    #[test]
    fn test_missing_identifier_initialisation() {
        let result = parse("int = 2;"); // keyword 'let' doesn't exist
        assert!(matches!(
            result,
            Err(CompilerError::SyntaxError { span, .. }) if span.line == 1 && span.col == 5
        ));
    }

    #[test]
    fn test_wrong_greater_than_token() {
        let result = parse("int a = 5;\nbool b = (a => 6);"); // typo, should be '=>' but is "assign + greater-than"
        assert!(matches!(
            result,
            Err(CompilerError::SyntaxError { span, .. }) if span.line == 2 && span.col == 13
        ));
    }
}
