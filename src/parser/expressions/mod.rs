use std::fmt::Debug;

use inkwell::values::AnyValue;

use crate::codegen::{CodeGen, VariableReference};

pub mod binary;
pub mod unary;
pub mod identifer;
pub mod calls;
pub mod literals;
pub mod expr_list;


pub trait Expression: Debug {
    fn is_assignable(&self) -> bool {
        false
    }
    fn codegen_expression<'a>(&self, codegen: &'a CodeGen) -> inkwell::values::AnyValueEnum<'a> {
        codegen.context.f64_type().const_zero().as_any_value_enum()
    }
    fn get_pointer<'a>(&self, _ : &'a CodeGen) -> VariableReference<'a> {
        panic!("Can't get pointer");
    }
    fn my_clone(&self) -> Box<dyn Expression>;
    fn desugar(&self) -> Box<dyn Expression>;
}

