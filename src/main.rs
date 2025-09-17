mod schemas;
mod lexer;
mod parser;

use crate::lexer::*;


fn main() {
    //let code = "let a = ((3 + 2.5) * 4) / .05;\nlet b = a * 2;print(b);\0";
    let code = "print(\"hello\");\0";

    let mut lexer = Lexer::new(code);
    let tokens = lexer.tokenize();

    println!("{:#?}", tokens);
}


