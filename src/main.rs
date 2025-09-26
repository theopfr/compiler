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
    let code = 
"bool a = (1 * (2 + 3)) + 3;
float b = -a / 5;
print(b + 3);
float c = 0.00001;
float d = a + c;
float e = a;
bool b1 = 2 == 2;
bool b2 = !(true && (2 > 0.5)) || (d != e) && (10 <= 200);
print(true && b2);\0";

    /*let code = 
"bool b = true && false;
int a = 2\0"
    ;*/

    let mut lexer = Lexer::new(code);
    lexer.tokenize()?;
    let tokens = lexer.get_tokens();

    let mut parser = Parser::new(tokens.to_vec());
    parser.parse()?;
    let ast = parser.get_tree();

    let mut analyser = SemanticAnalyser::new(ast.to_vec());
    analyser.check()?;

    println!("{:#?}", parser.get_tree());
    //println!("{:#?}", analyser.get_symbol_table());

    Ok(())
}
