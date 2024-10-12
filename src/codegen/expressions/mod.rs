use std::fmt::Debug;

use crate::parser::expressions::AnyExpressionEnum;

use super::{CodeGen, VariableReference};

pub mod binary;
pub mod unary;
pub mod identifer;
pub mod calls;
pub mod literals;
pub mod expr_list;

use inkwell::values::AnyValue;

impl GenerateExpr for AnyExpressionEnum {
    fn codegen_expression<'a>(&self, codegen: &'a CodeGen<'_>) -> inkwell::values::AnyValueEnum<'a> {
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
    fn get_pointer<'a>(&self, codegen : &'a CodeGen) -> VariableReference<'a> {
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
}

pub trait GenerateExpr: Debug { 
    fn codegen_expression<'a>(&self, codegen: &'a CodeGen<'_>) -> inkwell::values::AnyValueEnum<'a> {
        codegen.context.f64_type().const_zero().as_any_value_enum()
    }
    fn get_pointer<'a>(&self, _ : &'a CodeGen) -> VariableReference<'a> {
        panic!("Can't get pointer");
    }
}
