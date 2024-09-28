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
}
