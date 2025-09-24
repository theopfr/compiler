use crate::{errors::CompilerError, schemas::*};

pub struct Lexer {
    chars: Vec<char>,
    cur_pos: usize,
    tokens: Vec<Token>,
}

impl Lexer {
    pub fn new(program: &str) -> Self {
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
            "int" => self.tokens.push(Token {
                kind: TokenKind::Declare(Primitive::Int),
                pos: start,
            }),
            "float" => self.tokens.push(Token {
                kind: TokenKind::Declare(Primitive::Float),
                pos: start,
            }),
            "print" => self.tokens.push(Token {
                kind: TokenKind::Print,
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
        while let Some(&ch) = self.chars.get(self.cur_pos) {
            if ch.is_numeric() || ch == '.' {
                self.cur_pos += 1;
                continue;
            }
            break;
        }

        let numeric_token: String = self.chars[start..self.cur_pos].iter().collect();

        self.tokens.push(Token {
            kind: TokenKind::Literal(if numeric_token.contains('.') {
                Literal {
                    value: numeric_token,
                    primitive: Primitive::Float,
                }
            } else {
                Literal {
                    value: numeric_token,
                    primitive: Primitive::Int,
                }
            }),
            pos: start,
        });
    }

    pub fn tokenize(&mut self) -> Result<(), CompilerError> {
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
                    kind: TokenKind::Assign,
                    pos: self.cur_pos,
                }),
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

            self.cur_pos += 1;
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
                TokenKind::Assign,
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
                TokenKind::Assign,
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
                TokenKind::Assign,
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
                TokenKind::Assign,
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
                TokenKind::Assign,
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
    fn test_invalid_character() {
        let result = tokenize("int a = 5 $ 2;");
        assert!(matches!(result, Err(CompilerError::Syntax { .. })));
    }

    #[test]
    fn test_unterminated_string() {
        let result = tokenize("print(\"hello)");
        assert!(matches!(result, Err(CompilerError::Syntax { .. })));
    }
}
