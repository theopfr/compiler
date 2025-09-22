use std::f32::INFINITY;

use crate::schemas::*;

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
        self.tokens
            .last()
            .cloned()
            .unwrap_or(Token { kind: TokenKind::EOF, pos: self.tokens.len() })
    }
    
    fn consume_next(&mut self) -> Token {
        self.tokens.pop().unwrap_or(Token { kind: TokenKind::EOF, pos: self.tokens.len() })
    }

    fn parse_expression(&mut self, min_binding_pow: f32) -> Expr {
        let mut lhs = match &self.consume_next().kind {
            TokenKind::Literal(literal) => Expr::Literal(literal.clone()),
            TokenKind::Identifier(name) => Expr::Identifier(name.to_string()),
            TokenKind::BinOp(BinOpKind::Sub) => Expr::BinOp {
                op: BinOpKind::Sub,
                left: Box::new(Expr::Literal(Literal::Int("0".to_string()))),
                right: Box::new(self.parse_expression(INFINITY)),
            },
            TokenKind::BinOp(BinOpKind::Add) => self.parse_expression(INFINITY),
            TokenKind::LParen => {
                let expr = self.parse_expression(0.0);
                if !matches!(self.peek_next().kind, TokenKind::RParen) {
                    panic!("Expected ')'.");
                }
                self.consume_next();
                expr
            }
            t => panic!("Unexpected token {:?}.", t),
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
                        right: Box::new(self.parse_expression(rbp.clone())),
                    };
                }
                TokenKind::EOS => break,
                TokenKind::EOF => break,
                TokenKind::RParen => break,
                t => panic!("Unexpected token {:?}.", t),
            };
        }

        lhs
    }

    fn airthmetic_binding_power(op: &BinOpKind) -> (f32, f32) {
        match op {
            BinOpKind::Add | BinOpKind::Sub => (1.1, 1.2),
            BinOpKind::Mult | BinOpKind::Div => (2.1, 2.2),
        }
    }

    fn parse_statement(&mut self) -> Stmt {
        match self.consume_next().clone().kind {
            TokenKind::Declare(ref primitive) => {
                // Check for identifier (ie. variable name)
                let identifer_name = match self.peek_next().clone().kind {
                    TokenKind::Identifier(name) => name,
                    t => panic!("Unexpected token {:?}.", t),
                };
                self.consume_next();

                // Check for assign token (ie. '=')
                if !matches!(self.peek_next().kind, TokenKind::Assign) {
                    panic!("Expected '=' after declaration.");
                }
                self.consume_next();

                Stmt::Declare {
                    dtype: primitive.clone(),
                    name: identifer_name.clone(),
                    expr: self.parse_expression(0.0),
                }
            }
            TokenKind::Print => {
                if !matches!(self.peek_next().kind, TokenKind::LParen) {
                    panic!("Expected '(' after 'print' keyword.");
                }
                self.consume_next();
                let expr = self.parse_expression(0.0);
                if !matches!(self.peek_next().kind, TokenKind::RParen) {
                    panic!("Expected ')'.");
                }
                self.consume_next();

                Stmt::Print { expr: expr }
            }
            k => panic!("Unexpected token kind {:?}.", k),
        }
    }

    pub fn parse(&mut self) {
        while !matches!(self.peek_next().kind, TokenKind::EOF) {
            let stmt = self.parse_statement();
            if !matches!(self.peek_next().kind, TokenKind::EOS) {
                panic!("Expected ';' at end of expression.");
            }

            self.tree.push(stmt);

            if matches!(self.peek_next().kind, TokenKind::EOS) {
                self.consume_next();
            }
        }
    }

    fn type_check_expr(&self, expr: &Expr, ctx_type: &Primitive, identifiers: &Vec<Identifier>) {
        match expr {
            Expr::Literal(literal) => {
                
            },
            Expr::Identifier(name) => {

            },
            Expr::BinOp { op: _, left, right } => {
                self.type_check_expr(left, &ctx_type, &identifiers);
                self.type_check_expr(right, &ctx_type, &identifiers);
            },
        }
    }

    pub fn type_check(&self) {
        let mut identifiers: Vec<Identifier> = vec![];

        for stmt in &self.tree {
            let mut cur_ctx_type = Primitive::Int;

            match stmt {
                Stmt::Declare { dtype, name, expr } => {
                    cur_ctx_type = dtype.clone();
                    identifiers.push(Identifier { name: name.to_string(), primitive: cur_ctx_type.clone()});
                    self.type_check_expr(expr, &cur_ctx_type, &identifiers);
                },
                Stmt::Print { expr } => todo!(),
                s => panic!("Unexpected statement {:?}", s),
            }
        }
    }

    pub fn get_tree(&self) -> &Ast {
        &self.tree
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::lexer::Lexer;

    fn parse(input: &str) -> Ast {
        let mut lexer = Lexer::new(&(input.to_owned() + "\0"));
        lexer.tokenize();

        let mut parser = Parser::new(lexer.get_tokens().to_vec());
        parser.parse();

        parser.get_tree().to_vec()
    }

    #[test]
    fn test_simple_statement() {
        let ast = parse("int a = 1 + 2;");
        assert_eq!(
            ast,
            [Stmt::Declare {
                dtype: Primitive::Int,
                name: "a".to_string(),
                expr: Expr::BinOp {
                    op: BinOpKind::Add,
                    left: Box::new(Expr::Literal(Literal::Int("1".to_string()))),
                    right: Box::new(Expr::Literal(Literal::Int("2".to_string())))
                }
            }]
        );
    }

    #[test]
    fn test_left_side_precedence() {
        let ast = parse("float a = 1 * 2 + 3.5;");
        assert_eq!(
            ast,
            [Stmt::Declare {
                dtype: Primitive::Float,
                name: "a".to_string(),
                expr: Expr::BinOp {
                    op: BinOpKind::Add,
                    left: Box::new(Expr::BinOp {
                        op: BinOpKind::Mult,
                        left: Box::new(Expr::Literal(Literal::Int("1".to_string()))),
                        right: Box::new(Expr::Literal(Literal::Int("2".to_string())))
                    }),
                    right: Box::new(Expr::Literal(Literal::Float("3.5".to_string())))
                }
            }]
        );
    }

    #[test]
    fn test_right_side_precedence() {
        let ast = parse("float a = 0.3333 - 2 / 3;");
        assert_eq!(
            ast,
            [Stmt::Declare {
                dtype: Primitive::Float,
                name: "a".to_string(),
                expr: Expr::BinOp {
                    op: BinOpKind::Sub,
                    left: Box::new(Expr::Literal(Literal::Float("0.3333".to_string()))),
                    right: Box::new(Expr::BinOp {
                        op: BinOpKind::Div,
                        left: Box::new(Expr::Literal(Literal::Int("2".to_string()))),
                        right: Box::new(Expr::Literal(Literal::Int("3".to_string())))
                    }),
                }
            }]
        );
    }

    #[test]
    fn test_unary_operator() {
        let ast = parse("int res = -b * +3;");
        assert_eq!(
            ast,
            [Stmt::Declare {
                dtype: Primitive::Int,
                name: "res".to_string(),
                expr: Expr::BinOp {
                    op: BinOpKind::Mult,
                    left: Box::new(Expr::BinOp {
                        op: BinOpKind::Sub,
                        left: Box::new(Expr::Literal(Literal::Int("0".to_string()))),
                        right: Box::new(Expr::Identifier("b".to_string()))
                    }),
                    right: Box::new(Expr::Literal(Literal::Int("3".to_string()))),
                }
            }]
        );
    }

    #[test]
    fn test_simple_parentheses() {
        let ast = parse("int c = (1 + 2) * 3;");
        assert_eq!(
            ast,
            [Stmt::Declare {
                dtype: Primitive::Int,
                name: "c".to_string(),
                expr: Expr::BinOp {
                    op: BinOpKind::Mult,
                    left: Box::new(Expr::BinOp {
                        op: BinOpKind::Add,
                        left: Box::new(Expr::Literal(Literal::Int("1".to_string()))),
                        right: Box::new(Expr::Literal(Literal::Int("2".to_string()))),
                    }),
                    right: Box::new(Expr::Literal(Literal::Int("3".to_string()))),
                }
            }]
        );
    }

    #[test]
    fn test_nested_parentheses() {
        let ast = parse("float c = ((1 + a) * b) / (a - b);");
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
                            left: Box::new(Expr::Literal(Literal::Int("1".to_string()))),
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
        let ast = parse("print(1 * b);");
        assert_eq!(
            ast,
            [Stmt::Print {
                expr: Expr::BinOp {
                    op: BinOpKind::Mult,
                    left: Box::new(Expr::Literal(Literal::Int("1".to_string()))),
                    right: Box::new(Expr::Identifier("b".to_string()))
                }
            }]
        );
    }

    #[test]
    fn test_print_statement_with_parentheses() {
        let ast = parse("print((1 - b) * c);");
        assert_eq!(
            ast,
            [Stmt::Print {
                expr: Expr::BinOp {
                    op: BinOpKind::Mult,
                    left: Box::new(Expr::BinOp {
                        op: BinOpKind::Sub,
                        left: Box::new(Expr::Literal(Literal::Int("1".to_string()))),
                        right: Box::new(Expr::Identifier("b".to_string()))
                    }),
                    right: Box::new(Expr::Identifier("c".to_string()))
                }
            }]
        );
    }

    #[test]
    #[should_panic]
    fn test_missing_eos_semicolon() {
        let _ = parse("int a = 0 print(a);");
    }
}
