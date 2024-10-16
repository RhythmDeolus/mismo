use crate::parser::statements::whiles::WhileStatement;
use crate::codegen::expressions::GenerateExpr;

use super::Generate;

impl Generate for WhileStatement {
    fn generate_code(& self, codegen : &crate::codegen::CodeGen) {
        let parent = codegen.builder.get_insert_block().unwrap();
        let fun = parent.get_parent().unwrap();
        let condition_block = codegen.context.append_basic_block(fun, "condition_block");
        let body_block = codegen.context.append_basic_block(fun, "body_block");
        let return_block = codegen.context.append_basic_block(fun, "return_block");
        // hack
        codegen.builder.build_unconditional_branch(condition_block).unwrap();
        //
        codegen.builder.position_at_end(condition_block);
        let i = self.expression.codegen_expression(codegen).into_int_value();
        codegen.builder.build_conditional_branch(i, body_block, return_block).unwrap();
        codegen.increase_scope();
        codegen.builder.position_at_end(body_block);
        self.statement.generate_code(codegen);
        codegen.builder.build_unconditional_branch(condition_block).unwrap();
        codegen.decrease_scope();
    }
}
