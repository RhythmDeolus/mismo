use inkwell::values::AnyValue;

use super::Expression;
use super::expr_list::ExpressionList;
#[derive(Debug)]
pub struct StringLiteral {
    pub val: String,
}
impl Expression for StringLiteral {}

#[derive(Debug)]
pub struct NumberLiteral {
    pub val: String,
}
impl Expression for NumberLiteral {
    fn codegen_expression<'ctx>(&self, codegen: &'ctx crate::codegen::CodeGen) -> inkwell::values::AnyValueEnum<'ctx>{
        let f = self.val.split(' ').filter_map(|s| s.parse::<f32>().ok()).collect::<Vec<_>>();
        let f = *f.first().unwrap() as f64;
        println!("parsed to value: {}", f);

        codegen.context.f64_type().const_float(f).as_any_value_enum()
    }
}

#[derive(Debug)]
pub struct ArrayLiteral {
    pub expressions: ExpressionList,
}
impl Expression for ArrayLiteral {}

#[derive(Debug)]
pub struct False;
impl Expression for False {
    fn codegen_expression<'ctx>(&self, codegen: &'ctx crate::codegen::CodeGen) -> inkwell::values::AnyValueEnum<'ctx>{
        codegen.context.bool_type().const_zero().as_any_value_enum()
    }
}

#[derive(Debug)]
pub struct True;
impl Expression for True {
    fn codegen_expression<'ctx>(&self, codegen: &'ctx crate::codegen::CodeGen) -> inkwell::values::AnyValueEnum<'ctx>{
        codegen.context.bool_type().const_all_ones().as_any_value_enum()
    }
}

#[derive(Debug)]
pub struct NoneVal;
impl Expression for NoneVal {}
