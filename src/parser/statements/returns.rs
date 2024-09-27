use crate::parser::expressions::AnyExpressionEnum;

use super::{AnyStatementEnum, Statement};
#[derive(Debug)]
pub struct ReturnStatement {
    pub expression: Box<AnyExpressionEnum>,
}
impl Statement for ReturnStatement {
    fn desugar(self) -> AnyStatementEnum {
        ReturnStatement {
            expression: self.expression.desugar().boxed()
        }.into_any_statement_enum()
    }
    fn into_any_statement_enum(self) -> super::AnyStatementEnum {
        super::AnyStatementEnum::Return(self)
    }
    fn generate_code(& self, codegen : &crate::codegen::CodeGen) {
        codegen.print_module();
        let prev_block = codegen.builder.get_insert_block().unwrap();
        let return_point = codegen.context.append_basic_block(prev_block.get_parent().unwrap(), ".return_point");
        let _ = codegen.builder.build_unconditional_branch(return_point);
        codegen.builder.position_at_end(return_point);
        let e = self.expression.codegen_expression(codegen).into_float_value();
        let retprt = match codegen.get_variable(".return").unwrap() {
            crate::codegen::VariableReference::Local(x) => x,
            _ => unreachable!()
        };
        let _ = codegen.builder.build_store(retprt, e);
        codegen.return_points.lock().unwrap().push(return_point);
        let then_block = codegen.context.append_basic_block(return_point.get_parent().unwrap(), "return_then");
        codegen.builder.position_at_end(then_block);
        codegen.print_module();
    }
}
