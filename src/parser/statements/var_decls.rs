use std::borrow::BorrowMut;

use super::Statement;
use super::super::expressions::Expression;
#[derive(Debug)]
pub struct VarDeclaration {
    pub identifier: String,
    pub expression: Option<Box<dyn Expression>>,
}

impl Statement for VarDeclaration {
    fn generate_code(&self, codegen: &mut crate::codegen::CodeGen) {
        if let Some(e) = &self.expression {
            codegen.allocate_variable(&self.identifier);
            let val = e.codegen_expression(codegen);
            let ptr = codegen.get_variable(&self.identifier).unwrap();
            codegen.builder.build_store(ptr, val.into_float_value());
        } else {
            codegen.allocate_variable(&self.identifier);
        }
    }
}
