use crate::parser::expressions::calls::{Call, InbuiltCall, InbuiltCallTypes};

use super::GenerateExpr;
use inkwell::values::AnyValue;

impl GenerateExpr for InbuiltCall {
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
}


impl GenerateExpr for Call {
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
}
