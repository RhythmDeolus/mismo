use crate::parser::expressions::AnyExpressionEnum;

use super::{AnyStatementEnum, Statement};
use super::super::expressions::Expression;
#[derive(Debug)]
pub struct ReturnStatement {
    pub expression: Box<AnyExpressionEnum>,
}
impl Statement for ReturnStatement {
    fn desugar(self) -> AnyStatementEnum {
        ReturnStatement {
            expression: self.expression.desugar().boxed()
        }.as_any_statement_enum()
    }
    fn as_any_statement_enum(self) -> super::AnyStatementEnum {
        super::AnyStatementEnum::Return(self)
    }
    fn generate_code(&self, codegen : &mut crate::codegen::CodeGen) {
        let e = self.expression.codegen_expression(codegen).into_float_value();
        println!("generating return statement...");
        codegen.print_module();
        let retprt = match codegen.get_variable(".return").unwrap() {
            crate::codegen::VariableReference::Local(x) => x,
            _ => unreachable!()
        };
        let instr = codegen.builder.build_store(retprt, e);
        let instr =  instr.unwrap();
        let _ = codegen.builder.build_unconditional_branch(codegen.builder.get_insert_block().unwrap().get_parent().unwrap().get_last_basic_block().unwrap());
    }
}
