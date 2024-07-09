use super::Statement;
use super::super::expressions::Expression;

#[derive(Debug)]
pub struct ExpresssionStatement {
    pub expression: Box<dyn Expression>,
}
impl Statement for ExpresssionStatement {
    fn generate_code(&self, codegen: &mut crate::codegen::CodeGen) {
        self.expression.codegen_expression(codegen);
    }
    fn desugar(&self) -> Box<dyn Statement> {
        let e = self.expression.desugar();
        Box::new(ExpresssionStatement {
            expression: e
        })
    }
}
