use inkwell::types::AnyType;

use super::super::expressions::Expression;
use super::Statement;
#[derive(Debug)]
pub struct IfStatement {
    pub expression: Box<dyn Expression>,
    pub block: Box<dyn Statement>,
    pub else_block: Option<Box<dyn Statement>>,
}

impl Statement for IfStatement {
    fn generate_code(&self, codegen: &mut crate::codegen::CodeGen) {
        let fun = codegen.builder.get_insert_block()
                        .unwrap()
                        .get_parent()
                        .unwrap();
        let then_block = codegen
            .context
            .append_basic_block(fun,  "");
        let else_block = codegen
            .context
            .append_basic_block(fun, "");
        let return_block = codegen
            .context
            .append_basic_block(fun, "");

        let i = self.expression.codegen_expression(codegen);
        let i = i.into_int_value();
        println!("parsed as {}", i);
        codegen
            .builder
            .build_conditional_branch(i, then_block, else_block)
            .unwrap();
        codegen.builder.position_at_end(then_block);
        self.block.generate_code(codegen);
        codegen.builder.build_unconditional_branch(return_block);
        codegen.builder.position_at_end(else_block);
        if self.else_block.is_some() {
            self.else_block.as_ref().unwrap().generate_code(codegen);
        }
        codegen.builder.build_unconditional_branch(return_block);
    }
}
