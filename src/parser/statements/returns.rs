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
}
