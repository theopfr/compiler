#[derive(Debug, PartialEq, Clone)]


pub enum Primitive {
    Int,
    Float,
    Str,
}

pub struct Identifier {
    pub name: String,
    pub primitive: Primitive
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
    BinOp(BinOpKind),
    LParen,
    RParen,
    Print,
    EOS,
    EOF,
}

#[derive(Clone, Debug)]
pub struct Token {
    pub kind: TokenKind,
    pub pos: usize,
}


// ast schemas
#[derive(PartialEq, Clone, Debug)]
pub enum BinOpKind {
    Add,
    Sub,
    Mult,
    Div,
}

#[derive(PartialEq, Clone, Debug)]
pub enum Expr {
    Literal(Literal),
    Identifier(String),
    BinOp {
        op: BinOpKind,
        left: Box<Expr>,
        right: Box<Expr>,
    },
}

#[derive(PartialEq, Clone, Debug)]
pub enum Stmt {
    Declare { dtype: Primitive, name: String, expr: Expr },
    Print { expr: Expr },
    EOF,
    EOS
}


pub enum ContextType {
    Int,
    Float,
    Str,
    Auto
}

pub type Ast = Vec<Stmt>;