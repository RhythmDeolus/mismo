use std::fmt::Debug;

use self::{binary::BinaryOp, calls::{Call, InbuiltCall}, expr_list::ExpressionList, identifer::Identifier, literals::{ArrayLiteral, False, NoneVal, NumberLiteral, StringLiteral, True}, unary::UnaryOp};

pub mod binary;
pub mod unary;
pub mod identifer;
pub mod calls;
pub mod literals;
pub mod expr_list;

#[derive(Debug)]
pub enum AnyExpressionEnum {
    Binary(BinaryOp),
    Unary(UnaryOp),
    Identifier(Identifier),
    InbuiltCall(InbuiltCall),
    Call(Call),
    StringLiteral(StringLiteral),
    NumberLiteral(NumberLiteral),
    ArrayLiteral(ArrayLiteral),
    ExpressionList(ExpressionList),
    True(True),
    False(False),
    NoneVal(NoneVal),
}

impl AnyExpressionEnum {
    pub fn is_assignable(&self) -> bool {
        matches!(self, AnyExpressionEnum::Identifier(_))
    }
    pub fn boxed(self) -> Box<Self> {
        Box::new(self)
    }
    pub fn desugar(self) -> Self {
        match self {
        AnyExpressionEnum::Binary(x) => x.desugar(),
        AnyExpressionEnum::Unary(x) => x.desugar(),
        AnyExpressionEnum::Identifier(x) => x.desugar(),
        AnyExpressionEnum::InbuiltCall(x) => x.desugar(),
        AnyExpressionEnum::Call(x) => x.desugar(),
        AnyExpressionEnum::StringLiteral(x) => x.desugar(),
        AnyExpressionEnum::NumberLiteral(x) => x.desugar(),
        AnyExpressionEnum::ArrayLiteral(x) => x.desugar(),
        AnyExpressionEnum::ExpressionList(_) => unreachable!(),
        AnyExpressionEnum::True(x) => x.desugar(),
        AnyExpressionEnum::False(x) => x.desugar(),
        AnyExpressionEnum::NoneVal(x) => x.desugar(),
        }
    }
    fn my_clone(&self) -> AnyExpressionEnum {
        match self {
            AnyExpressionEnum::Binary(x) => x.my_clone(),
            AnyExpressionEnum::Unary(x) => x.my_clone(),
            AnyExpressionEnum::Identifier(x) => x.my_clone(),
            AnyExpressionEnum::InbuiltCall(x) => x.my_clone(),
            AnyExpressionEnum::Call(x) => x.my_clone(),
            AnyExpressionEnum::StringLiteral(x) => x.my_clone(),
            AnyExpressionEnum::NumberLiteral(x) => x.my_clone(),
            AnyExpressionEnum::ArrayLiteral(x) => x.my_clone(),
            AnyExpressionEnum::ExpressionList(_) => unreachable!(),
            AnyExpressionEnum::True(x) => x.my_clone(),
            AnyExpressionEnum::False(x) => x.my_clone(),
            AnyExpressionEnum::NoneVal(x) => x.my_clone(),
        }
    }
}


pub trait Expression: Debug {
    fn is_assignable(&self) -> bool {
        false
    }
    fn my_clone(&self) -> AnyExpressionEnum;
    fn desugar(self) -> AnyExpressionEnum;
    fn into_any_expression_enum(self) -> AnyExpressionEnum;
}

