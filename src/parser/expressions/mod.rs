use std::fmt::Debug;

use inkwell::{builder::BuilderError, context::Context, llvm_sys::prelude::LLVMValueRef, types::{AnyTypeEnum, FloatMathType, VoidType}, values::{AnyValue, AnyValueEnum, FloatValue}};

use crate::codegen::{self, CodeGen};

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
    fn get_pointer<'a>(&self, codegen: &'a CodeGen) -> inkwell::values::PointerValue<'a> {
        panic!("Can't get pointer");
    }
}

