
use super::expr_list::ExpressionList;
use super::{AnyExpressionEnum, Expression};
#[derive(Debug, Clone, Copy)]
pub enum InbuiltCallTypes {
    Print,
    PrintTime,
}

#[derive(Debug)]
pub struct InbuiltCall {
    pub c_type: InbuiltCallTypes,
    pub arguments: ExpressionList,
}
impl Expression for InbuiltCall {
    fn into_any_expression_enum(self) -> AnyExpressionEnum {
        AnyExpressionEnum::InbuiltCall(self)
    }
    fn desugar(self) -> AnyExpressionEnum {
        InbuiltCall{
            arguments: self.arguments.desugar(),
            c_type: self.c_type
        }.into_any_expression_enum()
    }
    fn my_clone(&self) -> AnyExpressionEnum {
        InbuiltCall {
            c_type: self.c_type,
            arguments: self.arguments.my_clone()
        }.into_any_expression_enum()
    }
}
#[derive(Debug)]
pub struct Call {
    pub left: String,
    pub arguments: ExpressionList,
}
impl Expression for Call {
    fn into_any_expression_enum(self) -> AnyExpressionEnum {
        AnyExpressionEnum::Call(self)
    }
    fn desugar(self) -> AnyExpressionEnum{
        Call {
            left: self.left.clone(),
            arguments: self.arguments.desugar()
        }.into_any_expression_enum()
    }
    fn my_clone(&self) -> AnyExpressionEnum {
        Call {
            left: self.left.clone(),
            arguments: self.arguments.my_clone()
        }.into_any_expression_enum()
    }
}
