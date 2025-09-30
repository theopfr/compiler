#[derive(Debug, PartialEq, Clone)]

pub enum Primitive {
    Int,
    Float,
    Bool,
}

#[derive(Debug)]
pub struct Identifier {
    pub primitive: Primitive,
    pub span: Span,
    pub mutable: bool,
}

#[derive(PartialEq, Clone, Debug)]
pub struct Span {
    pub line: usize,
    pub col: usize,
}

impl Default for Span {
    fn default() -> Self {
        Span { line: 0, col: 0 }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct Literal {
    pub value: String,
    pub primitive: Primitive,
}

// lexer schemas
#[derive(Debug, PartialEq, Clone)]
pub enum TokenKind {
    Declare(Primitive),
    Identifier(String),
    Literal(Literal),
    BinOp(BinOpKind),
    Mut,
    LParen,
    RParen,
    Print,
    EOS,
    EOF,
}

#[derive(Clone, Debug)]
pub struct Token {
    pub kind: TokenKind,
    pub span: Span,
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
    Not,
}

#[derive(PartialEq, Clone, Debug)]
pub enum UnaryOpKind {
    Neg,
    Not,
}

#[derive(PartialEq, Clone, Debug)]
pub enum Expr {
    Literal {
        value: String,
        primitive: Primitive,
        span: Span,
    },
    Identifier {
        name: String,
        span: Span,
    },
    BinOp {
        op: BinOpKind,
        left: Box<Expr>,
        right: Box<Expr>,
        span: Span,
    },
    UnaryOp {
        op: UnaryOpKind,
        expr: Box<Expr>,
        span: Span,
    },
}

#[derive(PartialEq, Clone, Debug)]
pub enum Stmt {
    Declare {
        dtype: Primitive,
        mutable: bool,
        name: String,
        expr: Expr,
        span: Span,
    },
    MutAssign {
        name: String,
        expr: Expr,
        span: Span,
    },
    Print {
        expr: Expr,
        span: Span,
    },
}

pub type Ast = Vec<Stmt>;
