use core::panic;
use std::fmt::Debug;

use inkwell::values::AnyValue;

use crate::codegen::{CodeGen, VariableReference};

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
        match self {
            AnyExpressionEnum::Identifier(_) => {
                true
            }
            _ => false
        }
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
    pub fn codegen_expression<'a, 'b>(&self, codegen: &'a CodeGen<'b>) -> inkwell::values::AnyValueEnum<'a> {
        match self {
            AnyExpressionEnum::Binary(x) => x.codegen_expression(codegen),
            AnyExpressionEnum::Unary(x) => x.codegen_expression(codegen),
            AnyExpressionEnum::Identifier(x) => x.codegen_expression(codegen),
            AnyExpressionEnum::InbuiltCall(x) => x.codegen_expression(codegen),
            AnyExpressionEnum::Call(x) => x.codegen_expression(codegen),
            AnyExpressionEnum::StringLiteral(x) => x.codegen_expression(codegen),
            AnyExpressionEnum::NumberLiteral(x) => x.codegen_expression(codegen),
            AnyExpressionEnum::ArrayLiteral(x) => x.codegen_expression(codegen),
            AnyExpressionEnum::ExpressionList(_) => unreachable!(),
            AnyExpressionEnum::True(x) => x.codegen_expression(codegen),
            AnyExpressionEnum::False(x) => x.codegen_expression(codegen),
            AnyExpressionEnum::NoneVal(x) => x.codegen_expression(codegen),
        }
    }
    pub fn get_pointer<'a>(&self, codegen : &'a CodeGen) -> VariableReference<'a> {
        match self {
            AnyExpressionEnum::Binary(x) => x.get_pointer(codegen),
            AnyExpressionEnum::Unary(x) => x.get_pointer(codegen),
            AnyExpressionEnum::Identifier(x) => x.get_pointer(codegen),
            AnyExpressionEnum::InbuiltCall(x) => x.get_pointer(codegen),
            AnyExpressionEnum::Call(x) => x.get_pointer(codegen),
            AnyExpressionEnum::StringLiteral(x) => x.get_pointer(codegen),
            AnyExpressionEnum::NumberLiteral(x) => x.get_pointer(codegen),
            AnyExpressionEnum::ArrayLiteral(x) => x.get_pointer(codegen),
            AnyExpressionEnum::ExpressionList(_) => unreachable!(),
            AnyExpressionEnum::True(x) => x.get_pointer(codegen),
            AnyExpressionEnum::False(x) => x.get_pointer(codegen),
            AnyExpressionEnum::NoneVal(x) => x.get_pointer(codegen),
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
    fn codegen_expression<'a, 'b>(&self, codegen: &'a CodeGen<'b>) -> inkwell::values::AnyValueEnum<'a> {
        codegen.context.f64_type().const_zero().as_any_value_enum()
    }
    fn get_pointer<'a>(&self, _ : &'a CodeGen) -> VariableReference<'a> {
        panic!("Can't get pointer");
    }
    fn my_clone(&self) -> AnyExpressionEnum;
    fn desugar(self) -> AnyExpressionEnum;
    fn as_any_expression_enum(self) -> AnyExpressionEnum;
}

