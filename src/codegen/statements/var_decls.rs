use crate::parser::statements::var_decls::VarDeclaration;
use crate::codegen::expressions::GenerateExpr;

use super::Generate;

impl Generate for VarDeclaration {
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
                    let _ = codegen.builder.build_store(lhs.as_pointer_value(), val.into_float_value());
                }
            }
        } else {
            codegen.allocate_variable(&self.identifier);
        }
    }
}
