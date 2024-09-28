use crate::parser::expressions::AnyExpressionEnum;

use super::{AnyStatementEnum, Statement};

#[derive(Debug)]
pub struct WhileStatement {
    pub expression: Box<AnyExpressionEnum>,
    pub statement: Box<AnyStatementEnum>,
}
impl Statement for WhileStatement {
    fn desugar(self) -> AnyStatementEnum {
        WhileStatement{
            expression: self.expression.desugar().boxed(),
            statement: self.statement.desugar().boxed()
        }.into_any_statement_enum()
    }
    fn into_any_statement_enum(self) -> AnyStatementEnum {
        AnyStatementEnum::While(self)
    }
}
