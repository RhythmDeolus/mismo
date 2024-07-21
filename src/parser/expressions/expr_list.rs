use super::AnyExpressionEnum;
#[derive(Debug)]
pub struct ExpressionList{
    pub expressions: Vec<Box<AnyExpressionEnum>>,
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
    pub fn desugar(self) -> ExpressionList {
        let expressions: Vec<Box<AnyExpressionEnum>> = self.expressions.into_iter().map(|x| Box::new(x.desugar())).collect();
        ExpressionList {
            expressions
        }
    }
    pub fn as_any_expression(self) -> AnyExpressionEnum {
        AnyExpressionEnum::ExpressionList(self)
    }
    pub fn my_clone(&self) -> ExpressionList {
        let mut expressions: Vec<Box<AnyExpressionEnum>> = vec![];
        for x in self.expressions.iter() {
            expressions.push(x.my_clone().boxed());
        }
        ExpressionList {
            expressions
        }
    }
}
