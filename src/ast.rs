use crate::schemas::*;


pub struct Ast {
    tokens: Vec<Token>,
    tree: Vec<Stmt>
}

impl Ast {
    pub fn new(tokens: Vec<Token>) -> Self {
        Ast {
            tokens: tokens,
            tree: vec![]
        }
    }

    fn parse_statement(tokens: Vec<Token>) -> Stmt {
        todo!()
    }

    pub fn parse(self) {
        todo!()
    }
}