use crate::parser::expressions::AnyExpressionEnum;

use super::{AnyStatementEnum, Statement};

#[derive(Debug)]
pub struct ExpresssionStatement {
    pub expression: Box<AnyExpressionEnum>,
}
impl Statement for ExpresssionStatement {
    fn desugar(self) -> AnyStatementEnum {
        let e = self.expression.desugar().boxed();
        ExpresssionStatement {
            expression: e
        }.into_any_statement_enum()
    }
    fn into_any_statement_enum(self) -> super::AnyStatementEnum {
        super::AnyStatementEnum::Expression(self)
    }
}
