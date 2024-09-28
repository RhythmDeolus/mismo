use super::{AnyStatementEnum, Statement};
#[derive(Debug)]
pub struct Block {
    pub statements: Vec<Box<AnyStatementEnum>>,
}
impl Statement for Block {
    fn desugar(self) -> AnyStatementEnum {
        let mut new_stmts = vec![];
        for stmt in self.statements {
            new_stmts.push(stmt.desugar().boxed());
        }
        Block {
            statements: new_stmts
        }.into_any_statement_enum()
    }
    fn into_any_statement_enum(self) -> AnyStatementEnum {
        AnyStatementEnum::Block(self)
    }
}
