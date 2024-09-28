use crate::parser::expressions::AnyExpressionEnum;

use super::{AnyStatementEnum, Statement};
#[derive(Debug)]
pub struct IfStatement {
    pub expression: Box<AnyExpressionEnum>,
    pub block: Box<AnyStatementEnum>,
    pub else_block: Option<Box<AnyStatementEnum>>,
}

impl Statement for IfStatement {
    fn desugar(self) -> AnyStatementEnum {
        let expression = self.expression.desugar().boxed();
        let block = self.block.desugar().boxed();
        let else_block = self.else_block.map(|x| x.desugar().boxed());
        IfStatement {
            expression,
            block,
            else_block,
        }
        .into_any_statement_enum()
    }
    fn into_any_statement_enum(self) -> AnyStatementEnum {
        AnyStatementEnum::If(self)
    }
}
