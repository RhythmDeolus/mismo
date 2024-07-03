use super::Statement;
use super::super::expressions::Expression;
#[derive(Debug)]
pub struct ReturnStatement {
    pub expression: Box<dyn Expression>,
}
impl Statement for ReturnStatement {}
