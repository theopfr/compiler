use core::panic;
use crate::schemas::*;

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
            "str" => self.tokens.push(Token {
                kind: TokenKind::Declare(Primitive::Str),
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
                Literal::Float(numeric_token)
            } else {
                Literal::Int(numeric_token)
            }),
            pos: start,
        });
    }

    fn handle_string(&mut self) {
        self.cur_pos += 1;

        let start = self.cur_pos;
        while let Some(&ch) = self.chars.get(self.cur_pos) {
            if ch == '"' {
                let string_token: String = self.chars[start..self.cur_pos].iter().collect();

                self.tokens.push(Token {
                    kind: TokenKind::Literal(Literal::Str(string_token)),
                    pos: start,
                });

                self.cur_pos += 1;
                return;
            }
            self.cur_pos += 1;
        }

        panic!("Unterminated string at position {start}.");
    }

    pub fn tokenize(&mut self) -> &Vec<Token> {
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
                '"' => {
                    self.handle_string();
                    continue;
                }
                '=' => self.tokens.push(Token {
                    kind: TokenKind::Assign,
                    pos: self.cur_pos,
                }),
                '+' => self.tokens.push(Token {
                    kind: TokenKind::Add,
                    pos: self.cur_pos,
                }),
                '-' => self.tokens.push(Token {
                    kind: TokenKind::Sub,
                    pos: self.cur_pos,
                }),
                '*' => self.tokens.push(Token {
                    kind: TokenKind::Mult,
                    pos: self.cur_pos,
                }),
                '/' => self.tokens.push(Token {
                    kind: TokenKind::Div,
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


#[cfg(test)]
mod tests {
    use super::*;

    fn lex(input: &str) -> Vec<TokenKind> {
        let mut lexer = Lexer::new(&(input.to_owned() + "\0"));
        lexer.tokenize().iter().map(|t| t.kind.clone()).collect()
    }

    #[test]
    fn test_int_declaration() {
        let tokens = lex("int a = 42;");
        assert_eq!(
            tokens,
            vec![
                TokenKind::Declare(Primitive::Int),
                TokenKind::Identifier("a".into()),
                TokenKind::Assign,
                TokenKind::Literal(Literal::Int("42".into())),
                TokenKind::EOS,
                TokenKind::EOF,
            ]
        );
    }

    #[test]
    fn test_float_declaration() {
        let tokens = lex("float pi = 3.14;");
        assert_eq!(
            tokens,
            vec![
                TokenKind::Declare(Primitive::Float),
                TokenKind::Identifier("pi".into()),
                TokenKind::Assign,
                TokenKind::Literal(Literal::Float("3.14".into())),
                TokenKind::EOS,
                TokenKind::EOF,
            ]
        );
    }

    #[test]
    fn test_print_statement() {
        let tokens = lex("print(x);");
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
        let tokens = lex("int a = 3 + 5;");
        assert_eq!(
            tokens,
            vec![
                TokenKind::Declare(Primitive::Int),
                TokenKind::Identifier("a".into()),
                TokenKind::Assign,
                TokenKind::Literal(Literal::Int("3".into())),
                TokenKind::Add,
                TokenKind::Literal(Literal::Int("5".into())),
                TokenKind::EOS,
                TokenKind::EOF,
            ]
        );
    }
    
    #[test]
    fn test_expression_with_negation() {
        let tokens = lex("int a = 3 * (-5);");
        assert_eq!(
            tokens,
            vec![
                TokenKind::Declare(Primitive::Int),
                TokenKind::Identifier("a".into()),
                TokenKind::Assign,
                TokenKind::Literal(Literal::Int("3".into())),
                TokenKind::Mult,
                TokenKind::LParen,
                TokenKind::Sub,
                TokenKind::Literal(Literal::Int("5".into())),
                TokenKind::RParen,
                TokenKind::EOS,
                TokenKind::EOF,
            ]
        );
    }

    #[test]
    fn test_parentheses_and_mult_div() {
        let tokens = lex("(2 * 4) / .5;");
        assert_eq!(
            tokens,
            vec![
                TokenKind::LParen,
                TokenKind::Literal(Literal::Int("2".into())),
                TokenKind::Mult,
                TokenKind::Literal(Literal::Int("4".into())),
                TokenKind::RParen,
                TokenKind::Div,
                TokenKind::Literal(Literal::Float(".5".into())),
                TokenKind::EOS,
                TokenKind::EOF,
            ]
        );
    }

    #[test]
    fn test_identifier_with_underscore() {
        let tokens = lex("int my_var = 1;");
        assert_eq!(
            tokens,
            vec![
                TokenKind::Declare(Primitive::Int),
                TokenKind::Identifier("my_var".into()),
                TokenKind::Assign,
                TokenKind::Literal(Literal::Int("1".into())),
                TokenKind::EOS,
                TokenKind::EOF,
            ]
        );
    }

    #[test]
    #[should_panic]
    fn test_invalid_character() {
        let _ = lex("int a = 5 $ 2;");
    }

    #[test]
    fn test_simple_string() {
        let tokens = lex("str a = \"hello\";");
        assert_eq!(
            tokens,
            vec![
                TokenKind::Declare(Primitive::Str),
                TokenKind::Identifier("a".into()),
                TokenKind::Assign,
                TokenKind::Literal(Literal::Str("hello".to_string())),
                TokenKind::EOS,
                TokenKind::EOF,
            ]
        );
    }

    #[test]
    fn test_string_with_expression() {
        let tokens = lex("\"4 + 3\"");
        assert_eq!(
            tokens,
            vec![
                TokenKind::Literal(Literal::Str("4 + 3".to_string())),
                TokenKind::EOF
            ]
        );
    }

    #[test]
    fn test_empty_string() {
        let tokens = lex("print(\"\")");
        assert_eq!(
            tokens,
            vec![
                TokenKind::Print,
                TokenKind::LParen,
                TokenKind::Literal(Literal::Str("".to_string())),
                TokenKind::RParen,
                TokenKind::EOF
            ]
        );
    }

    #[test]
    #[should_panic]
    fn test_unterminated_string() {
        let _ = lex("print(\"hello)");
    }
}
