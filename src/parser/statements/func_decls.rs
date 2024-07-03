use super::blocks::Block;
use super::Statement;
#[derive(Debug)]
pub struct FunctionDeclaration {
    pub name: String,
    pub parameters_list: Vec<Box<dyn Statement>>,
    pub body: Block,
}
impl Statement for FunctionDeclaration {}
