use super::Statement;
use super::super::expressions::Expression;
#[derive(Debug)]
pub struct ReturnStatement {
    pub expression: Box<dyn Expression>,
}
impl Statement for ReturnStatement {
    fn desugar(&self) -> Box<dyn Statement> {
        Box::new(ReturnStatement {
            expression: self.expression.desugar()
        })
    }
}
