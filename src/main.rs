mod lexer;
mod parser;
mod semantic;
mod schemas;

use crate::{lexer::*, parser::Parser, semantic::SemanticAnalyser};

fn main() {
    /*let code = "
        int a = (1 * (2 + 3)) / 3;\n
        int b = -a + a / 5;\n
        float c = (1 + 2) * 3;\0
    ";*/
    let code = "
        int b = -a + a / 5;\0
    ";

    let mut lexer = Lexer::new(code);
    lexer.tokenize();
    let tokens = lexer.get_tokens();

    let mut parser = Parser::new(tokens.to_vec());
    parser.parse();
    let ast = parser.get_tree();

    // let mut analyser = SemanticAnalyser::new(ast.to_vec());
    // analyser.check();

    println!("{:#?}", parser.get_tree());
}
