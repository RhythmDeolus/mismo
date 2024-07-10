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
    fn desugar(&self) -> Box<dyn Statement> {
        Box::new(VarDeclaration {
            identifier: self.identifier.clone(),
            expression: self.expression.as_ref().map(|x| x.desugar())
        })
    }
}
