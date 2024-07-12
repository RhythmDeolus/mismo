use crate::parser::expressions::AnyExpressionEnum;

use super::super::expressions::Expression;
use super::{AnyStatementEnum, Statement};
#[derive(Debug)]
pub struct IfStatement {
    pub expression: Box<AnyExpressionEnum>,
    pub block: Box<AnyStatementEnum>,
    pub else_block: Option<Box<AnyStatementEnum>>,
}

impl Statement for IfStatement {
    fn generate_code(&self, codegen: &mut crate::codegen::CodeGen) {
        let fun = codegen.builder.get_insert_block()
                        .unwrap()
                        .get_parent()
                        .unwrap();
        let then_block = codegen
            .context
            .append_basic_block(fun,  "if_then");
        let else_block = codegen
            .context
            .append_basic_block(fun, "else");
        let return_block = codegen
            .context
            .append_basic_block(fun, "if_return");

        let i = self.expression.codegen_expression(codegen);
        let i = i.into_int_value();
        println!("parsed as {}", i);
        codegen
            .builder
            .build_conditional_branch(i, then_block, else_block)
            .unwrap();
        codegen.increase_scope();
        codegen.builder.position_at_end(then_block);
        self.block.generate_code(codegen);
        codegen.decrease_scope();
        let _ = codegen.builder.build_unconditional_branch(return_block);
        codegen.builder.position_at_end(else_block);
        if self.else_block.is_some() {
            codegen.increase_scope();
            self.else_block.as_ref().unwrap().generate_code(codegen);
            codegen.decrease_scope();
        }
        let _ = codegen.builder.build_unconditional_branch(return_block);
        codegen.builder.position_at_end(return_block);
    }

    fn desugar(self) -> AnyStatementEnum {
        let expression = self.expression.desugar().boxed();
        let block = self.block.desugar().boxed();
        let else_block = self.else_block.map(|x| x.desugar().boxed());
        IfStatement {
            expression,
            block,
            else_block
        }.as_any_statement_enum()
    }
    fn as_any_statement_enum(self) -> AnyStatementEnum {
        AnyStatementEnum::If(self)
    }
}
