use crate::schemas::*;

pub struct Ast {
    tokens: Vec<Token>,
    cur_pos: usize,
}

impl Ast {
    pub fn new(tokens: Vec<Token>) -> Self {
        Ast {
            tokens: tokens,
            cur_pos: 0,
        }
    }

    fn peek_next(&mut self) -> &Token {
        return &self.tokens[self.cur_pos];
    }

    fn consume_next(&mut self) -> &Token {
        let token = &self.tokens[self.cur_pos];
        self.cur_pos += 1;
        token
    }

    fn parse_expression(&mut self, min_binding_pow: f32) -> Expr {
        let mut lhs = match &self.consume_next().kind {
            TokenKind::Literal(literal) => Expr::Literal(literal.clone()),
            TokenKind::Identifier(name) => Expr::Identifier(name.to_string()),
            TokenKind::BinOp(BinOpKind::Sub) => Expr::BinOp {
                op: BinOpKind::Sub,
                left: Box::new(Expr::Literal(Literal::Int("0".to_string()))),
                right: Box::new(self.parse_expression(3.0)),
            },

            TokenKind::BinOp(BinOpKind::Add) => self.parse_expression(3.0),
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
                t => panic!("Unexpected token {:?}.", t),
            };
        }

        lhs
    }

    fn airthmetic_binding_power(op: &BinOpKind) -> (f32, f32) {
        match op {
            BinOpKind::Add | BinOpKind::Sub => (1.1, 1.2),
            BinOpKind::Mult | BinOpKind::Div => (2.1, 2.2),
            _ => panic!("Unknown operation: {:?}", op),
        }
    }

    fn parse_statement(&mut self) -> Stmt {
        let cur_token = &self.tokens[self.cur_pos];

        match &cur_token.kind {
            TokenKind::Declare(primitive) => {
                let data_type = primitive.clone();
                self.cur_pos += 1;

                if let TokenKind::Identifier(name) = &self.tokens[self.cur_pos].kind {
                    self.cur_pos += 1;

                    if !matches!(self.tokens[self.cur_pos].kind, TokenKind::Assign) {
                        panic!("Expected '=' after declaration.");
                    }
                    self.cur_pos += 1;

                    return Stmt::Declare {
                        dtype: data_type,
                        name: name.clone(),
                        expr: self.parse_expression(0.0),
                    };
                } else {
                    panic!("Expected identifier.");
                }
            }
            TokenKind::EOS => Stmt::EOS,
            TokenKind::EOF => Stmt::EOF,
            _ => todo!(),
        }
    }

    pub fn parse(mut self) -> Vec<Stmt> {
        let mut tree = Vec::new();

        while !matches!(self.peek_next().kind, TokenKind::EOF) {
            let stmt = self.parse_statement();
            tree.push(stmt);

            if matches!(self.peek_next().kind, TokenKind::EOS) {
                self.consume_next();
            }
        }

        tree
    }
}
