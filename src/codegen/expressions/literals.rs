use crate::parser::expressions::literals::{ArrayLiteral, False, NoneVal, NumberLiteral, StringLiteral, True};

use super::GenerateExpr;
use inkwell::values::AnyValue;

impl GenerateExpr for StringLiteral {
}

impl GenerateExpr for NumberLiteral {
    fn codegen_expression<'ctx>(&self, codegen: &'ctx crate::codegen::CodeGen) -> inkwell::values::AnyValueEnum<'ctx>{
        let f = self.val.split(' ').filter_map(|s| s.parse::<f32>().ok()).collect::<Vec<_>>();
        let f = *f.first().unwrap() as f64;

        codegen.context.f64_type().const_float(f).as_any_value_enum()
    }
}

impl GenerateExpr for ArrayLiteral {
}

impl GenerateExpr for False {
    fn codegen_expression<'ctx>(&self, codegen: &'ctx crate::codegen::CodeGen) -> inkwell::values::AnyValueEnum<'ctx>{
        codegen.context.bool_type().const_zero().as_any_value_enum()
    }
}

impl GenerateExpr for True {
    fn codegen_expression<'ctx>(&self, codegen: &'ctx crate::codegen::CodeGen) -> inkwell::values::AnyValueEnum<'ctx>{
        codegen.context.bool_type().const_all_ones().as_any_value_enum()
    }
}

impl GenerateExpr for NoneVal {
}
