use crate::parser::statements::exprs::ExpresssionStatement;
use crate::codegen::expressions::GenerateExpr;

use super::Generate;

impl Generate for ExpresssionStatement {
    fn generate_code(& self, codegen : &crate::codegen::CodeGen) {
        self.expression.codegen_expression(codegen);
    }
}
