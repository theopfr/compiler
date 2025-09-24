#[derive(Debug, PartialEq, Clone)]


pub enum Primitive {
    Int,
    Float,
    Bool
}

#[derive(Debug)]
pub struct Identifier {
    pub primitive: Primitive
}


#[derive(Debug, PartialEq, Clone)]
pub struct Literal {
    pub value: String,
    pub primitive: Primitive
}

// lexer schemas
#[derive(Debug, PartialEq, Clone)]
pub enum TokenKind {
    Declare(Primitive),
    Identifier(String),
    Literal(Literal),
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
    Assign,
    Add,
    Sub,
    Mult,
    Div,
    Gt,
    Lt,
    Ge,
    Le,
    Eq,
    Ne,
    And,
    Or,
    Not
}

#[derive(PartialEq, Clone, Debug)]
pub enum UnaryOpKind {
    Neg,
    Not
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
    UnaryOp {
        op: UnaryOpKind,
        expr: Box<Expr>
    }
}

#[derive(PartialEq, Clone, Debug)]
pub enum Stmt {
    Declare { dtype: Primitive, name: String, expr: Expr },
    Print { expr: Expr },
}


pub type Ast = Vec<Stmt>;