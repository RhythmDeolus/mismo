use super::Expression;
#[derive(Debug)]
pub struct ExpressionList {
    pub expressions: Vec<Box<dyn Expression>>,
}
impl ExpressionList {
    pub fn is_assignable(&self) -> bool {
        for exp in self.expressions.iter() {
            if !exp.is_assignable() {
                return false;
            }
        }
        true
    }
}
