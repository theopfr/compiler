#[derive(Debug, PartialEq, Clone)]


pub enum Primitive {
    Int,
    Float,
    Str,
}

#[derive(Debug, PartialEq, Clone)]
pub enum Literal {
    Int(String),
    Float(String),
    Str(String),
}

// lexer schemas
#[derive(Debug, PartialEq, Clone)]
pub enum TokenKind {
    Declare(Primitive),
    Identifier(String),
    Literal(Literal),
    Assign,
    Add,
    Sub,
    Mult,
    Div,
    LParen,
    RParen,
    Print,
    EOS,
    EOF,
}

#[derive(Debug)]
pub struct Token {
    pub kind: TokenKind,
    pub pos: usize,
}


// ast schemas
pub enum BinOpKind {
    Add,
    Sub,
    Mult,
    Div,
}


pub enum Expr {
    Literal(Literal),
    Identifier(String),
    BinOp {
        op: BinOpKind,
        left: Box<Expr>,
        right: Box<Expr>,
    },
}

pub enum Stmt {
    Declare { dtype: Primitive, name: String, expr: Expr },
    Print { expr: Expr },
}
