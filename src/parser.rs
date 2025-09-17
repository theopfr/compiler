use crate::schemas::*;

pub struct Ast {
    tokens: Vec<Token>,
    cur_pos: usize,
    tree: Vec<Stmt>,
}

impl Ast {
    pub fn new(tokens: Vec<Token>) -> Self {
        Ast {
            tokens: tokens,
            cur_pos: 0,
            tree: vec![],
        }
    }

    fn parse_expression(&self, tokens: &[Token], pos: usize) -> Expr {
        let cur_token = &tokens[pos];
        match &cur_token.kind {
            TokenKind::Identifier(_) => todo!(),
            TokenKind::Literal(literal) => todo!(),
            TokenKind::Add => todo!(),
            TokenKind::Sub => todo!(),
            TokenKind::Mult => todo!(),
            TokenKind::Div => todo!(),
            TokenKind::LParen => todo!(),
            TokenKind::RParen => todo!(),
            _ => panic!("Unexpected token {:?}.", cur_token),
        }
    }

    fn parse_until_eos(&mut self) -> Expr {
        let start = self.cur_pos;

        while !matches!(self.tokens[self.cur_pos].kind, TokenKind::EOS | TokenKind::EOF) {
            self.cur_pos += 1;
        }

        let expr_tokens = &self.tokens[start..self.cur_pos];

        // consume the semicolon
        if matches!(self.tokens[self.cur_pos].kind, TokenKind::EOS) {
            self.cur_pos += 1;
        }

        self.parse_expression(expr_tokens, self.cur_pos.clone())
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
                        expr: self.parse_until_eos(),
                    };
                }
                else {
                    panic!("Expected identifier.");
                }
            }
            TokenKind::Identifier(_) => todo!(),
            TokenKind::Literal(literal) => todo!(),
            TokenKind::Assign => todo!(),
            TokenKind::Add => todo!(),
            TokenKind::Sub => todo!(),
            TokenKind::Mult => todo!(),
            TokenKind::Div => todo!(),
            TokenKind::LParen => todo!(),
            TokenKind::RParen => todo!(),
            TokenKind::Print => todo!(),
            TokenKind::EOS => todo!(),
            TokenKind::EOF => return Stmt::EOF,
        }
    }

    pub fn parse(mut self) {
        loop {
            let statement = self.parse_statement();

            if statement == Stmt::EOF {
                return;
            }

            self.tree.push(statement.clone());
        }
    }
}
