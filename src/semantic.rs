use crate::schemas::{Ast, Expr, Identifier, Primitive, Stmt};



pub struct SemanticAnalyser {
    ast: Ast,
    identifiers: Vec<Identifier>
}


impl SemanticAnalyser {
    pub fn new(ast: Ast) -> Self {
        SemanticAnalyser {
            ast: ast,
            identifiers: vec![],
        }
    }

    fn check_stmt(stmt: Stmt) {
        todo!()
    }

    fn check_expr(&self, expr: &Expr, ctx_type: &Primitive, identifiers: &Vec<Identifier>) {
        todo!()
    }

    pub fn check(&mut self) {
        for stmt in &self.ast {
            let mut cur_ctx_type = Primitive::Int;

            match stmt {
                Stmt::Declare { dtype, name, expr } => {
                    cur_ctx_type = dtype.clone();
                    self.identifiers.push(Identifier { name: name.to_string(), primitive: cur_ctx_type.clone()});
                    self.check_expr(expr, &cur_ctx_type, &self.identifiers);
                },
                Stmt::Print { expr } => todo!(),
                s => panic!("Unexpected statement {:?}", s),
            }
        }
    }
}