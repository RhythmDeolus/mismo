use crate::parser::expressions::AnyExpressionEnum;

use super::{AnyStatementEnum, Statement};
use super::super::expressions::Expression;
#[derive(Debug)]
pub struct VarDeclaration {
    pub identifier: String,
    pub expression: Option<Box<AnyExpressionEnum>>,
}

impl Statement for VarDeclaration {
    fn generate_code<'a>(& self, codegen : &'a crate::codegen::CodeGen<'a>) {
        if let Some(e) = &self.expression {
            codegen.allocate_variable(&self.identifier);
            let val = e.codegen_expression(codegen);
            let ptr = codegen.get_variable(&self.identifier).unwrap();
            // TODO: Error handling
            match ptr {
                crate::codegen::VariableReference::Local(lhs) => {
                    let _ = codegen.builder.build_store(lhs, val.into_float_value());
                }
                crate::codegen::VariableReference::Global(lhs) => {
                    lhs.set_initializer(&val.into_float_value());
                }
            }
        } else {
            codegen.allocate_variable(&self.identifier);
        }
    }
    fn desugar(self) -> AnyStatementEnum {
        VarDeclaration {
            identifier: self.identifier.clone(),
            expression: self.expression.map(|x| x.desugar().boxed())
        }.as_any_statement_enum()
    }
    fn as_any_statement_enum(self) -> super::AnyStatementEnum {
        super::AnyStatementEnum::VarDeclaration(self)
    }
}
