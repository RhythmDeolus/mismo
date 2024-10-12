
use super::{AnyStatementEnum, Statement};

#[derive(Debug)]
pub struct FunctionDeclaration {
    pub name: String,
    pub parameters_list: Vec<Box<AnyStatementEnum>>,
    pub body: Box<AnyStatementEnum>,
}
impl Statement for FunctionDeclaration {
    fn desugar(self) -> AnyStatementEnum {
        let body = self.body.desugar().boxed();
        FunctionDeclaration{
            name: self.name.clone(),
            parameters_list: self.parameters_list.into_iter().map(|x| x.desugar().boxed()).collect(),
            body
        }.into_any_statement_enum()
    }
    fn into_any_statement_enum(self) -> AnyStatementEnum {
        AnyStatementEnum::FunctionDeclaration(self)
    }
}
