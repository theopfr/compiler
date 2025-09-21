mod lexer;
mod parser;
mod schemas;

use crate::{lexer::*, parser::Ast};

fn main() {
    //let code = "let a = ((3 + 2.5) * 4) / .05;\nlet b = a * 2;print(b);\0";
    let code = "
        int a = 1 * 2 + -3;\n
        int b = -a + -a / 5;\0
    ";

    let mut lexer = Lexer::new(code);
    let tokens = lexer.tokenize();

    let parser = Ast::new(tokens.to_vec());
    let tree = parser.parse();

    println!("{:#?}", tree);
}
