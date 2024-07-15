use core::panic;

use inkwell::execution_engine;
use inkwell::values::AnyValue;

use crate::codegen;

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
    fn as_any_expression_enum(self) -> AnyExpressionEnum {
        AnyExpressionEnum::InbuiltCall(self)
    }
    fn codegen_expression<'a>(&self, codegen: &'a crate::codegen::CodeGen) -> inkwell::values::AnyValueEnum<'a> {
        match self.c_type {
            InbuiltCallTypes::Print => {
                let mut args = vec![];
                for a in self.arguments.expressions.iter() {
                    args.push(a.codegen_expression(codegen).into_float_value());
                }
                for a in args {
                    let f = codegen.module.get_function("print").unwrap();
                    let _ = codegen.builder.build_call(f, &[a.into()], "");
                }
            }
            InbuiltCallTypes::PrintTime => {
                let f = codegen.module.get_function("print_time").unwrap();
                let f = f.as_any_value_enum().into_function_value();
                let _ = codegen.builder.build_call(f, &[], "");
            }
        }
        codegen.context.f64_type().const_zero().as_any_value_enum()
    }
    fn desugar(self) -> AnyExpressionEnum {
        InbuiltCall{
            arguments: self.arguments.desugar(),
            c_type: self.c_type
        }.as_any_expression_enum()
    }
    fn my_clone(&self) -> AnyExpressionEnum {
        InbuiltCall {
            c_type: self.c_type,
            arguments: self.arguments.my_clone()
        }.as_any_expression_enum()
    }
}
#[derive(Debug)]
pub struct Call {
    pub left: String,
    pub arguments: ExpressionList,
}
impl Expression for Call {
    fn as_any_expression_enum(self) -> AnyExpressionEnum {
        AnyExpressionEnum::Call(self)
    }
    fn codegen_expression<'ctx>(
        &self,
        codegen: &'ctx  crate::codegen::CodeGen,
    ) -> inkwell::values::AnyValueEnum<'ctx>{
        let inbuilt_f = codegen.module.get_function(&self.left);
        if let Some(val) = inbuilt_f {
            let mut args = vec![];
            for a in self.arguments.expressions.iter() {
                let v = a.codegen_expression(codegen);
                let v = v.into_float_value();
                args.push(v.into());
            }
            let retv = codegen.builder.build_call(val, &args, "retv");
            let retv = retv.unwrap().as_any_value_enum();
            return retv.as_any_value_enum();
        }         //TODO
        codegen.context.f64_type().const_zero().as_any_value_enum()
    }
    fn desugar(self) -> AnyExpressionEnum{
        Call {
            left: self.left.clone(),
            arguments: self.arguments.desugar()
        }.as_any_expression_enum()
    }
    fn my_clone(&self) -> AnyExpressionEnum {
        Call {
            left: self.left.clone(),
            arguments: self.arguments.my_clone()
        }.as_any_expression_enum()
    }
}
