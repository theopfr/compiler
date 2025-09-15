use core::panic;

#[derive(Debug)]
pub enum TokenType {
    Let,
    Ident(String),
    Assign,
    Number(String),
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
    token_type: TokenType,
    pos: usize,
}

pub enum OpKind {
    Add,
    Sub,
    Mult,
    Div,
}

pub enum Expr {
    Number(i64),
    Ident(String),
    BinOp {
        op: OpKind,
        left: Box<Expr>,
        right: Box<Expr>,
    },
}

pub enum Statement {
    Let { name: String, expr: Expr },
    Print { expr: Expr },
}

struct Lexer {
    chars: Vec<char>,
    cur_pos: usize,
    tokens: Vec<Token>,
}

impl Lexer {
    fn new(program: &str) -> Self {
        Lexer {
            chars: program.chars().collect(),
            cur_pos: 0,
            tokens: vec![],
        }
    }

    fn handle_alphanumeric(&mut self) {
        let start = self.cur_pos;
        while let Some(&ch) = self.chars.get(self.cur_pos) {
            if ch.is_alphanumeric() || ch == '_' {
                self.cur_pos += 1;
                continue;
            }
            break;
        }

        let token: String = self.chars[start..self.cur_pos].iter().collect();
        match token.as_str() {
            "let" => self.tokens.push(Token {
                token_type: TokenType::Let,
                pos: start,
            }),
            "print" => self.tokens.push(Token {
                token_type: TokenType::Print,
                pos: start,
            }),
            _ => self.tokens.push(Token {
                token_type: TokenType::Ident(token),
                pos: start,
            }),
        }
    }

    fn handle_numeric(&mut self) {
        let start = self.cur_pos;
        while let Some(&ch) = self.chars.get(self.cur_pos) {
            if ch.is_numeric() || ch == '.' {
                self.cur_pos += 1;
                continue;
            }
            break;
        }

        let token: String = self.chars[start..self.cur_pos].iter().collect();
        println!("{}", token);
        self.tokens.push(Token {
            token_type: TokenType::Number(token),
            pos: start,
        });
    }

    fn tokenize(&mut self) -> &Vec<Token> {
        loop {
            let cur_char = self.chars[self.cur_pos];

            match cur_char {
                c if c.is_whitespace() => (),
                c if c.is_alphabetic() => {
                    self.handle_alphanumeric();
                    continue;
                }
                c if c.is_numeric() || cur_char == '.' => {
                    self.handle_numeric();
                    continue;
                }
                '=' => self.tokens.push(Token {
                    token_type: TokenType::Assign,
                    pos: self.cur_pos,
                }),

                '+' => self.tokens.push(Token {
                    token_type: TokenType::Add,
                    pos: self.cur_pos,
                }),

                '-' => self.tokens.push(Token {
                    token_type: TokenType::Sub,
                    pos: self.cur_pos,
                }),

                '*' => self.tokens.push(Token {
                    token_type: TokenType::Mult,
                    pos: self.cur_pos,
                }),

                '/' => self.tokens.push(Token {
                    token_type: TokenType::Div,
                    pos: self.cur_pos,
                }),

                '(' => self.tokens.push(Token {
                    token_type: TokenType::LParen,
                    pos: self.cur_pos,
                }),

                ')' => self.tokens.push(Token {
                    token_type: TokenType::RParen,
                    pos: self.cur_pos,
                }),

                ';' => self.tokens.push(Token {
                    token_type: TokenType::EOS,
                    pos: self.cur_pos,
                }),
                '\0' => {
                    self.tokens.push(Token {
                        token_type: TokenType::EOF,
                        pos: self.cur_pos,
                    });
                    break;
                }
                _ => {
                    panic!(
                        "Unexpcted character '{}' at position {}.",
                        cur_char, self.cur_pos
                    );
                }
            }

            self.cur_pos += 1;
        }

        return &self.tokens;
    }
}

fn main() {
    let code = "let a = 3 + 2 * 4;\0";
    let mut lexer = Lexer::new(code);
    let tokens = lexer.tokenize();

    println!("{:?}", tokens);
}
