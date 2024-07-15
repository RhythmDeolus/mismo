use super::{AnyStatementEnum, Statement};
#[derive(Debug)]
pub struct Block {
    pub statements: Vec<Box<AnyStatementEnum>>,
}
impl Statement for Block {
    fn generate_code<'a>(& self, codegen : &'a crate::codegen::CodeGen<'a>) {
        codegen.increase_scope();
        for stmt in self.statements.iter() {
            stmt.generate_code(codegen)
        }
        codegen.decrease_scope();
    }
    fn desugar(self) -> AnyStatementEnum {
        let mut new_stmts = vec![];
        for stmt in self.statements {
            new_stmts.push(stmt.desugar().boxed());
        }
        Block {
            statements: new_stmts
        }.as_any_statement_enum()
    }
    fn as_any_statement_enum(self) -> AnyStatementEnum {
        AnyStatementEnum::Block(self)
    }
}
