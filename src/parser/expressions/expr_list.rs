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
    pub fn desugar(&self) -> ExpressionList {
        let expressions: Vec<Box<dyn Expression>> = self.expressions.iter().map(|x| x.desugar()).collect();
        ExpressionList {
            expressions
        }
    }
    pub fn my_clone(&self) -> ExpressionList {
        let mut expressions: Vec<Box<dyn Expression>> = vec![];
        for x in self.expressions.iter() {
            expressions.push(x.my_clone());
        }
        ExpressionList {
            expressions
        }
    }
}
