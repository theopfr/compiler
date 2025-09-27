use crate::{errors::CompilerError, schemas::*};

pub struct Lexer {
    chars: Vec<char>,
    cur_line: usize,
    cur_col: usize,
    tokens: Vec<Token>,
}

impl Lexer {
    pub fn new(program: &str) -> Self {
        Lexer {
            chars: program.chars().rev().collect(),
            cur_line: 1,
            cur_col: 1,
            tokens: vec![],
        }
    }

    fn peek_next(&self) -> char {
        self.chars.last().cloned().unwrap_or('\0')
    }

    fn consume_next(&mut self) -> char {
        let cur_char = self.chars.pop().unwrap_or('\0');
        if cur_char == '\n' {
            self.cur_line += 1;
            self.cur_col = 1;
        } else {
            self.cur_col += 1;
        }
        cur_char
    }

    fn handle_alphanumeric(&mut self) {
        let (start_line, start_col) = (self.cur_line, self.cur_col);

        let mut token: String = String::new();
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
                span: Span { line: start_line, col: start_col },
            }),
            "float" => self.tokens.push(Token {
                kind: TokenKind::Declare(Primitive::Float),
                span: Span { line: start_line, col: start_col },
            }),
            "bool" => self.tokens.push(Token {
                kind: TokenKind::Declare(Primitive::Bool),
                span: Span { line: start_line, col: start_col },
            }),
            "print" => self.tokens.push(Token {
                kind: TokenKind::Print,
                span: Span { line: start_line, col: start_col },
            }),
            "true" => self.tokens.push(Token {
                kind: TokenKind::Literal(Literal {
                    value: "true".to_string(),
                    primitive: Primitive::Bool,
                }),
                span: Span { line: start_line, col: start_col },
            }),
            "false" => self.tokens.push(Token {
                kind: TokenKind::Literal(Literal {
                    value: "false".to_string(),
                    primitive: Primitive::Bool,
                }),
                span: Span { line: start_line, col: start_col },
            }),
            _ => self.tokens.push(Token {
                kind: TokenKind::Identifier(token),
                span: Span { line: start_line, col: start_col },
            }),
        }
    }

    fn handle_numeric(&mut self) {
        let (start_line, start_col) = (self.cur_line, self.cur_col);

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
            span: Span { line: start_line, col: start_col },
        });
    }

    fn handle_boolean(&mut self) -> Result<(), CompilerError> {
        let (start_line, start_col) = (self.cur_line, self.cur_col);

        let token = self.consume_next();
        match token {
            '=' => match self.peek_next() {
                '=' => {
                    self.tokens.push(Token {
                        kind: TokenKind::BinOp(BinOpKind::Eq),
                        span: Span { line: start_line, col: start_col },
                    });
                    self.consume_next();
                }
                _ => self.tokens.push(Token {
                    kind: TokenKind::BinOp(BinOpKind::Assign),
                    span: Span { line: start_line, col: start_col },
                }),
            },
            '<' => match self.peek_next() {
                '=' => {
                    self.tokens.push(Token {
                        kind: TokenKind::BinOp(BinOpKind::Le),
                        span: Span { line: start_line, col: start_col },
                    });
                    self.consume_next();
                }
                _ => self.tokens.push(Token {
                    kind: TokenKind::BinOp(BinOpKind::Lt),
                    span: Span { line: start_line, col: start_col },
                }),
            },
            '>' => match self.peek_next() {
                '=' => {
                    self.tokens.push(Token {
                        kind: TokenKind::BinOp(BinOpKind::Ge),
                        span: Span { line: start_line, col: start_col },
                    });
                    self.consume_next();
                }
                _ => self.tokens.push(Token {
                    kind: TokenKind::BinOp(BinOpKind::Gt),
                    span: Span { line: start_line, col: start_col },
                }),
            },
            '&' => match self.peek_next() {
                '&' => {
                    self.tokens.push(Token {
                        kind: TokenKind::BinOp(BinOpKind::And),
                        span: Span { line: start_line, col: start_col },
                    });
                    self.consume_next();
                }
                _ => {
                    return Err(CompilerError::SyntaxError {
                        message: "Unexpected single character '&', did you mean '&&'?".to_string(),
                        span: Span { line: start_line, col: start_col },
                    });
                }
            },
            '|' => match self.peek_next() {
                '|' => {
                    self.tokens.push(Token {
                        kind: TokenKind::BinOp(BinOpKind::Or),
                        span: Span { line: start_line, col: start_col },
                    });
                    self.consume_next();
                }
                _ => {
                    return Err(CompilerError::SyntaxError {
                        message: "Unexpected single character '|', did you mean '||'?".to_string(),
                        span: Span { line: start_line, col: start_col },
                    });
                }
            },
            '!' => {
                match self.peek_next() {
                    '=' => {
                        self.tokens.push(Token {
                            kind: TokenKind::BinOp(BinOpKind::Ne),
                            span: Span { line: start_line, col: start_col },
                        });
                        self.consume_next();
                    }
                    _ => self.tokens.push(Token {
                        kind: TokenKind::BinOp(BinOpKind::Not),
                        span: Span { line: start_line, col: start_col },
                    }),
                };
            }
            t => {
                return Err(CompilerError::SyntaxError {
                    message: format!("Unexpected character '{}'.", t),
                    span: Span { line: start_line, col: start_col },
                });
            }
        }

        Ok(())
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
                    match self.handle_boolean() {
                        Ok(_) => continue,
                        Err(err) => return Err(err),
                    }
                }
                '+' => self.tokens.push(Token {
                    kind: TokenKind::BinOp(BinOpKind::Add),
                    span: Span { line: self.cur_line, col: self.cur_col },
                }),
                '-' => self.tokens.push(Token {
                    kind: TokenKind::BinOp(BinOpKind::Sub),
                    span: Span { line: self.cur_line, col: self.cur_col },
                }),
                '*' => self.tokens.push(Token {
                    kind: TokenKind::BinOp(BinOpKind::Mult),
                    span: Span { line: self.cur_line, col: self.cur_col },
                }),
                '/' => self.tokens.push(Token {
                    kind: TokenKind::BinOp(BinOpKind::Div),
                    span: Span { line: self.cur_line, col: self.cur_col },
                }),
                '(' => self.tokens.push(Token {
                    kind: TokenKind::LParen,
                    span: Span { line: self.cur_line, col: self.cur_col },
                }),
                ')' => self.tokens.push(Token {
                    kind: TokenKind::RParen,
                    span: Span { line: self.cur_line, col: self.cur_col },
                }),
                ';' => self.tokens.push(Token {
                    kind: TokenKind::EOS,
                    span: Span { line: self.cur_line, col: self.cur_col },
                }),
                '\0' => {
                    self.tokens.push(Token {
                        kind: TokenKind::EOF,
                        span: Span { line: self.cur_line, col: self.cur_col },

                    });
                    self.consume_next();
                    break;
                }
                _ => {
                    return Err(CompilerError::SyntaxError {
                        message: format!("Unexpected character '{}'.", cur_char),
                        span: Span { line: self.cur_line, col: self.cur_col },
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

    fn get_token_spans(input: &str) -> Result<Vec<Span>, CompilerError> {
        let mut lexer = Lexer::new(&(input.to_owned() + "\0"));
        lexer.tokenize()?;
        Ok(lexer.get_tokens().iter().map(|t| t.span.clone()).collect())
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
        assert!(matches!(result, Err(CompilerError::SyntaxError { .. })));
    }

    #[test]
    fn test_span_positions() {
        let spans = get_token_spans("int a = 5 - 0.2;\nbool b = !(a >= 17);").unwrap();
        assert_eq!(
            spans,
            vec![
                Span { line: 1, col: 1 },    // int
                Span { line: 1, col: 5 },    // a
                Span { line: 1, col: 7 },    // =
                Span { line: 1, col: 9 },    // 5
                Span { line: 1, col: 11 },   // -
                Span { line: 1, col: 13 },   // 0.2
                Span { line: 1, col: 16 },   // ;
                Span { line: 2, col: 1 },    // bool
                Span { line: 2, col: 6 },    // b
                Span { line: 2, col: 8 },    // =
                Span { line: 2, col: 10 },   // !
                Span { line: 2, col: 11 },   // (
                Span { line: 2, col: 12 },   // a
                Span { line: 2, col: 14 },   // >=
                Span { line: 2, col: 17 },   // 17
                Span { line: 2, col: 19 },   // )
                Span { line: 2, col: 20 },   // ;
                Span { line: 2, col: 21 },   // EOF
            ]
        );
    }

    #[test]
    fn test_span_positions_no_whitespaces() {
        let spans = get_token_spans("int a=5-0.2;\nbool b=!(a>=17);").unwrap();
        assert_eq!(
            spans,
            vec![
                Span { line: 1, col: 1 },   // int
                Span { line: 1, col: 5 },   // a
                Span { line: 1, col: 6 },   // =
                Span { line: 1, col: 7 },   // 5
                Span { line: 1, col: 8 },   // -
                Span { line: 1, col: 9 },   // 0.2
                Span { line: 1, col: 12 },  // ;
                Span { line: 2, col: 1 },   // bool
                Span { line: 2, col: 6 },   // b
                Span { line: 2, col: 7 },   // =
                Span { line: 2, col: 8 },   // !
                Span { line: 2, col: 9 },   // (
                Span { line: 2, col: 10 },  // a
                Span { line: 2, col: 11 },  // >=
                Span { line: 2, col: 13 },  // 17
                Span { line: 2, col: 15 },  // )
                Span { line: 2, col: 16 },  // ;
                Span { line: 2, col: 17 },  // EOF
            ]
        );
    }

    #[test]
    fn test_span_multi_line() {
        let spans = get_token_spans("int a = -5 +\n 7;\n\nbool \nb = false;").unwrap();
        assert_eq!(
            spans,
            vec![
                Span { line: 1, col: 1 },   // int
                Span { line: 1, col: 5 },   // a
                Span { line: 1, col: 7 },   // =
                Span { line: 1, col: 9 },   // -
                Span { line: 1, col: 10 },  // 5
                Span { line: 1, col: 12 },  // +
                Span { line: 2, col: 2 },   // 7
                Span { line: 2, col: 3 },   // ;
                Span { line: 4, col: 1 },   // bool
                Span { line: 5, col: 1 },   // b
                Span { line: 5, col: 3 },   // =
                Span { line: 5, col: 5 },   // false
                Span { line: 5, col: 10 },  // ;
                Span { line: 5, col: 11 },  // EOF
            ]
        );
    }
}
