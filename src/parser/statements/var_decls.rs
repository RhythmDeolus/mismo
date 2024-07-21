use crate::parser::expressions::AnyExpressionEnum;

use super::{AnyStatementEnum, Statement};
#[derive(Debug)]
pub struct VarDeclaration {
    pub identifier: String,
    pub expression: Option<Box<AnyExpressionEnum>>,
}

impl Statement for VarDeclaration {
    fn generate_code(& self, codegen : &crate::codegen::CodeGen) {
        if let Some(e) = &self.expression {
            let val = e.codegen_expression(codegen);
            codegen.allocate_variable(&self.identifier);
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
        }.into_any_statement_enum()
    }
    fn into_any_statement_enum(self) -> super::AnyStatementEnum {
        super::AnyStatementEnum::VarDeclaration(self)
    }
}
