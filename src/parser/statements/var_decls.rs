use crate::parser::expressions::AnyExpressionEnum;

use super::{AnyStatementEnum, Statement};
#[derive(Debug)]
pub struct VarDeclaration {
    pub identifier: String,
    pub expression: Option<Box<AnyExpressionEnum>>,
}

impl Statement for VarDeclaration {
    fn desugar(self) -> AnyStatementEnum {
        VarDeclaration {
            identifier: self.identifier.clone(),
            expression: self.expression.map(|x| x.desugar().boxed())
        }.into_any_statement_enum()
    }
    fn into_any_statement_enum(self) -> super::AnyStatementEnum {
        super::AnyStatementEnum::VarDeclaration(self)
    }
}
