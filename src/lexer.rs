use std::str::Chars;

use crate::{errors::CompilerError, schemas::*};

pub struct Lexer {
    chars: Vec<char>,
    cur_pos: usize,
    tokens: Vec<Token>,
}

impl Lexer {
    pub fn new(program: &str) -> Self {
        Lexer {
            chars: program.chars().rev().collect(),
            cur_pos: 0,
            tokens: vec![],
        }
    }

    fn peek_next(&self) -> char {
        self.chars.last().cloned().unwrap_or('\0')
    }

    fn consume_next(&mut self) -> char {
        self.chars.pop().unwrap_or('\0')
    }

    fn handle_alphanumeric(&mut self) {
        let start = self.cur_pos;
        let mut token = String::new();
        loop {
            let next_char = self.peek_next();
            if next_char.is_alphanumeric() || next_char == '_' {
                token.push(self.consume_next());
                continue;
            }
            break;
        }

        match token.as_str() {
            "int" => self.tokens.push(Token {
                kind: TokenKind::Declare(Primitive::Int),
                pos: start,
            }),
            "float" => self.tokens.push(Token {
                kind: TokenKind::Declare(Primitive::Float),
                pos: start,
            }),
            "bool" => self.tokens.push(Token {
                kind: TokenKind::Declare(Primitive::Bool),
                pos: start,
            }),
            "print" => self.tokens.push(Token {
                kind: TokenKind::Print,
                pos: start,
            }),
            "true" => self.tokens.push(Token {
                kind: TokenKind::Literal(Literal {
                    value: "true".to_string(),
                    primitive: Primitive::Bool,
                }),
                pos: start,
            }),
            "false" => self.tokens.push(Token {
                kind: TokenKind::Literal(Literal {
                    value: "false".to_string(),
                    primitive: Primitive::Bool,
                }),
                pos: start,
            }),
            _ => self.tokens.push(Token {
                kind: TokenKind::Identifier(token),
                pos: start,
            }),
        }
    }

    fn handle_numeric(&mut self) {
        let start = self.cur_pos;
        let mut token = String::new();
        loop {
            let next_char = self.peek_next();
            if next_char.is_numeric() || next_char == '.' {
                token.push(self.consume_next());
                continue;
            }
            break;
        }

        self.tokens.push(Token {
            kind: TokenKind::Literal(if token.contains('.') {
                Literal {
                    value: token,
                    primitive: Primitive::Float,
                }
            } else {
                Literal {
                    value: token,
                    primitive: Primitive::Int,
                }
            }),
            pos: start,
        });
    }

    fn handle_boolean(&mut self) {
        let token = self.consume_next();
        match token {
            '=' => {
                match self.peek_next() {
                    '=' => self.tokens.push(Token {
                        kind: TokenKind::BinOp(BinOpKind::Eq),
                        pos: self.cur_pos,
                    }),
                    _ => self.tokens.push(Token {
                        kind: TokenKind::BinOp(BinOpKind::Assign),
                        pos: self.cur_pos,
                    }),
                }
                self.consume_next();
            }
            '<' => {
                match self.peek_next() {
                    '=' => self.tokens.push(Token {
                        kind: TokenKind::BinOp(BinOpKind::Le),
                        pos: self.cur_pos,
                    }),
                    _ => self.tokens.push(Token {
                        kind: TokenKind::BinOp(BinOpKind::Lt),
                        pos: self.cur_pos,
                    }),
                }
                self.consume_next();
            }
            '>' => {
                match self.peek_next() {
                    '=' => self.tokens.push(Token {
                        kind: TokenKind::BinOp(BinOpKind::Ge),
                        pos: self.cur_pos,
                    }),
                    _ => self.tokens.push(Token {
                        kind: TokenKind::BinOp(BinOpKind::Gt),
                        pos: self.cur_pos,
                    }),
                }
                self.consume_next();
            }
            '&' => match self.peek_next() {
                '&' => {
                    self.tokens.push(Token {
                        kind: TokenKind::BinOp(BinOpKind::And),
                        pos: self.cur_pos,
                    });
                    self.consume_next();
                }
                _ => panic!("Unexpected single character '&', did you mean '&&'?"),
            },
            '|' => match self.peek_next() {
                '|' => {
                    self.tokens.push(Token {
                        kind: TokenKind::BinOp(BinOpKind::Or),
                        pos: self.cur_pos,
                    });
                    self.consume_next();
                }
                _ => panic!("Unexpected single character '|', did you mean '||'?"),
            },
            '!' => {
                match self.peek_next() {
                    '=' => {
                        self.tokens.push(Token {
                            kind: TokenKind::BinOp(BinOpKind::Ne),
                            pos: self.cur_pos,
                        });
                        self.consume_next();
                    }
                    _ => self.tokens.push(Token {
                        kind: TokenKind::BinOp(BinOpKind::Not),
                        pos: self.cur_pos,
                    }),
                };
            }
            t => panic!("Ahh {}", t),
        }
    }

    pub fn tokenize(&mut self) -> Result<(), CompilerError> {
        loop {
            let cur_char: char = self.peek_next();

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
                '<' | '>' | '=' | '&' | '!' | '|' => {
                    self.handle_boolean();
                    continue;
                }
                '+' => self.tokens.push(Token {
                    kind: TokenKind::BinOp(BinOpKind::Add),
                    pos: self.cur_pos,
                }),
                '-' => self.tokens.push(Token {
                    kind: TokenKind::BinOp(BinOpKind::Sub),
                    pos: self.cur_pos,
                }),
                '*' => self.tokens.push(Token {
                    kind: TokenKind::BinOp(BinOpKind::Mult),
                    pos: self.cur_pos,
                }),
                '/' => self.tokens.push(Token {
                    kind: TokenKind::BinOp(BinOpKind::Div),
                    pos: self.cur_pos,
                }),
                '(' => self.tokens.push(Token {
                    kind: TokenKind::LParen,
                    pos: self.cur_pos,
                }),
                ')' => self.tokens.push(Token {
                    kind: TokenKind::RParen,
                    pos: self.cur_pos,
                }),
                ';' => self.tokens.push(Token {
                    kind: TokenKind::EOS,
                    pos: self.cur_pos,
                }),
                '\0' => {
                    self.tokens.push(Token {
                        kind: TokenKind::EOF,
                        pos: self.cur_pos,
                    });
                    self.consume_next();
                    break;
                }
                _ => {
                    return Err(CompilerError::Syntax {
                        message: format!("Unexpected character '{}'.", cur_char),
                        col: 0,
                        pos: 0,
                    });
                }
            }
            self.consume_next();
        }

        Ok(())
    }

    pub fn get_tokens(&self) -> &Vec<Token> {
        &self.tokens
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn tokenize(input: &str) -> Result<Vec<TokenKind>, CompilerError> {
        let mut lexer = Lexer::new(&(input.to_owned() + "\0"));
        lexer.tokenize()?;
        Ok(lexer.get_tokens().iter().map(|t| t.kind.clone()).collect())
    }

    #[test]
    fn test_int_declaration() {
        let tokens = tokenize("int a = 42;").unwrap();
        assert_eq!(
            tokens,
            vec![
                TokenKind::Declare(Primitive::Int),
                TokenKind::Identifier("a".into()),
                TokenKind::BinOp(BinOpKind::Assign),
                TokenKind::Literal(Literal {
                    value: "42".to_string(),
                    primitive: Primitive::Int
                }),
                TokenKind::EOS,
                TokenKind::EOF,
            ]
        );
    }

    #[test]
    fn test_float_declaration() {
        let tokens = tokenize("float pi = 3.14;").unwrap();
        assert_eq!(
            tokens,
            vec![
                TokenKind::Declare(Primitive::Float),
                TokenKind::Identifier("pi".into()),
                TokenKind::BinOp(BinOpKind::Assign),
                TokenKind::Literal(Literal {
                    value: "3.14".to_string(),
                    primitive: Primitive::Float
                }),
                TokenKind::EOS,
                TokenKind::EOF,
            ]
        );
    }

    #[test]
    fn test_print_statement() {
        let tokens = tokenize("print(x);").unwrap();
        assert_eq!(
            tokens,
            vec![
                TokenKind::Print,
                TokenKind::LParen,
                TokenKind::Identifier("x".into()),
                TokenKind::RParen,
                TokenKind::EOS,
                TokenKind::EOF,
            ]
        );
    }

    #[test]
    fn test_simple_expression() {
        let tokens = tokenize("int a = 3 + 5;").unwrap();
        assert_eq!(
            tokens,
            vec![
                TokenKind::Declare(Primitive::Int),
                TokenKind::Identifier("a".into()),
                TokenKind::BinOp(BinOpKind::Assign),
                TokenKind::Literal(Literal {
                    value: "3".to_string(),
                    primitive: Primitive::Int
                }),
                TokenKind::BinOp(BinOpKind::Add),
                TokenKind::Literal(Literal {
                    value: "5".to_string(),
                    primitive: Primitive::Int
                }),
                TokenKind::EOS,
                TokenKind::EOF,
            ]
        );
    }

    #[test]
    fn test_expression_with_negation() {
        let tokens = tokenize("int a = 3 * (-5);").unwrap();
        assert_eq!(
            tokens,
            vec![
                TokenKind::Declare(Primitive::Int),
                TokenKind::Identifier("a".into()),
                TokenKind::BinOp(BinOpKind::Assign),
                TokenKind::Literal(Literal {
                    value: "3".to_string(),
                    primitive: Primitive::Int
                }),
                TokenKind::BinOp(BinOpKind::Mult),
                TokenKind::LParen,
                TokenKind::BinOp(BinOpKind::Sub),
                TokenKind::Literal(Literal {
                    value: "5".to_string(),
                    primitive: Primitive::Int
                }),
                TokenKind::RParen,
                TokenKind::EOS,
                TokenKind::EOF,
            ]
        );
    }

    #[test]
    fn test_parentheses_and_mult_div() {
        let tokens = tokenize("(2 * 4) / .5;").unwrap();
        assert_eq!(
            tokens,
            vec![
                TokenKind::LParen,
                TokenKind::Literal(Literal {
                    value: "2".to_string(),
                    primitive: Primitive::Int
                }),
                TokenKind::BinOp(BinOpKind::Mult),
                TokenKind::Literal(Literal {
                    value: "4".to_string(),
                    primitive: Primitive::Int
                }),
                TokenKind::RParen,
                TokenKind::BinOp(BinOpKind::Div),
                TokenKind::Literal(Literal {
                    value: ".5".to_string(),
                    primitive: Primitive::Float
                }),
                TokenKind::EOS,
                TokenKind::EOF,
            ]
        );
    }

    #[test]
    fn test_identifier_with_underscore() {
        let tokens = tokenize("int my_var = 1;").unwrap();
        assert_eq!(
            tokens,
            vec![
                TokenKind::Declare(Primitive::Int),
                TokenKind::Identifier("my_var".into()),
                TokenKind::BinOp(BinOpKind::Assign),
                TokenKind::Literal(Literal {
                    value: "1".to_string(),
                    primitive: Primitive::Int
                }),
                TokenKind::EOS,
                TokenKind::EOF,
            ]
        );
    }

    #[test]
    fn test_boolean_expression() {
        let tokens = tokenize("bool b = true && false || true != false && true == false;").unwrap();
        assert_eq!(
            tokens,
            vec![
                TokenKind::Declare(Primitive::Bool),
                TokenKind::Identifier("b".into()),
                TokenKind::BinOp(BinOpKind::Assign),
                TokenKind::Literal(Literal {
                    value: "true".to_string(),
                    primitive: Primitive::Bool
                }),
                TokenKind::BinOp(BinOpKind::And),
                TokenKind::Literal(Literal {
                    value: "false".to_string(),
                    primitive: Primitive::Bool
                }),
                TokenKind::BinOp(BinOpKind::Or),
                TokenKind::Literal(Literal {
                    value: "true".to_string(),
                    primitive: Primitive::Bool
                }),
                TokenKind::BinOp(BinOpKind::Ne),
                TokenKind::Literal(Literal {
                    value: "false".to_string(),
                    primitive: Primitive::Bool
                }),
                TokenKind::BinOp(BinOpKind::And),
                TokenKind::Literal(Literal {
                    value: "true".to_string(),
                    primitive: Primitive::Bool
                }),
                TokenKind::BinOp(BinOpKind::Eq),
                TokenKind::Literal(Literal {
                    value: "false".to_string(),
                    primitive: Primitive::Bool
                }),
                TokenKind::EOS,
                TokenKind::EOF,
            ]
        );
    }

    #[test]
    fn test_number_comparison() {
        let tokens = tokenize("int a = 1 > 1 && 2 < 2 && 3 >= 3 && 4 <= 4;").unwrap();
        assert_eq!(
            tokens,
            vec![
                TokenKind::Declare(Primitive::Int),
                TokenKind::Identifier("a".into()),
                TokenKind::BinOp(BinOpKind::Assign),
                TokenKind::Literal(Literal {
                    value: "1".to_string(),
                    primitive: Primitive::Int
                }),
                TokenKind::BinOp(BinOpKind::Gt),
                TokenKind::Literal(Literal {
                    value: "1".to_string(),
                    primitive: Primitive::Int
                }),
                TokenKind::BinOp(BinOpKind::And),
                TokenKind::Literal(Literal {
                    value: "2".to_string(),
                    primitive: Primitive::Int
                }),
                TokenKind::BinOp(BinOpKind::Lt),
                TokenKind::Literal(Literal {
                    value: "2".to_string(),
                    primitive: Primitive::Int
                }),
                TokenKind::BinOp(BinOpKind::And),
                TokenKind::Literal(Literal {
                    value: "3".to_string(),
                    primitive: Primitive::Int
                }),
                TokenKind::BinOp(BinOpKind::Ge),
                TokenKind::Literal(Literal {
                    value: "3".to_string(),
                    primitive: Primitive::Int
                }),
                TokenKind::BinOp(BinOpKind::And),
                TokenKind::Literal(Literal {
                    value: "4".to_string(),
                    primitive: Primitive::Int
                }),
                TokenKind::BinOp(BinOpKind::Le),
                TokenKind::Literal(Literal {
                    value: "4".to_string(),
                    primitive: Primitive::Int
                }),
                TokenKind::EOS,
                TokenKind::EOF,
            ]
        );
    }

    #[test]
    fn test_invalid_character() {
        let result = tokenize("int a = 5 $ 2;");
        assert!(matches!(result, Err(CompilerError::Syntax { .. })));
    }
}
