mod lexer;
mod parser;
mod semantic;
mod schemas;
mod errors;

use crate::{errors::CompilerError, lexer::*, parser::Parser, semantic::SemanticAnalyser};

fn main() {
    if let Err(err) = compile() {
        eprintln!("{}", err);
        std::process::exit(1);
    }
}

fn compile() -> Result<(), CompilerError> {
    /*let code = "
        int a = (1 * (2 + 3));
        float b = -a / 5;
        print(b + 3);
        int z = 10;
        float c = a + 0.4;
        bool b = 2 == 2;
        bool u = true == 2;
        \0
    ";*/
    let code = "
        bool a = !true;
        bool b = (-2 > +3) == a;
        \0
    ";

    let mut lexer = Lexer::new(code);
    lexer.tokenize()?;
    let tokens = lexer.get_tokens();

    let mut parser = Parser::new(tokens.to_vec());
    parser.parse()?;
    let ast = parser.get_tree();

    let mut analyser = SemanticAnalyser::new(ast.to_vec());
    analyser.check()?;

    println!("{:#?}", parser.get_tree());
    println!("{:#?}", analyser.get_symbol_table());

    Ok(())
}
