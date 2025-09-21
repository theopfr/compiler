mod lexer;
mod parser;
mod schemas;

use crate::{lexer::*, parser::Ast};

fn main() {
    /*let code = "
        int a = (1 * (2 + 3)) / 3;\n
        int b = -a + a / 5;\n
        float c = (1 + 2) * 3;\0
    ";*/
    let code = "
        float c = (1 + 2) * 3;\0
    ";

    let mut lexer = Lexer::new(code);
    lexer.tokenize();
    let tokens = lexer.get_tokens();

    let mut parser = Ast::new(tokens.to_vec());
    parser.parse();

    println!("{:#?}", parser.get_tree());
}
