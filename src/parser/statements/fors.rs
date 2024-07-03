use super::Statement;
use super::super::expressions::{
    Expression,
    expr_list::ExpressionList
};
#[derive(Debug)]
pub struct ForStatement {
    pub inital: Option<Box<dyn Statement>>,
    pub check: Option<Box<dyn Expression>>,
    pub change: Option<ExpressionList>,
    pub statement: Box<dyn Statement>,
}
impl Statement for ForStatement {}
