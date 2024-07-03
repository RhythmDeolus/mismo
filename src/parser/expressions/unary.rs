
use inkwell::values::AnyValue;

use super::Expression;
#[derive(Debug)]
pub struct UnaryOp {
    pub operand: Box<dyn Expression>,
    pub op_type: UnaryOpType,
}
impl Expression for UnaryOp {
    fn codegen_expression<'ctx>(&self, codegen: &'ctx crate::codegen::CodeGen) -> inkwell::values::AnyValueEnum<'ctx> {
        let v = self.operand.codegen_expression(codegen);
        match self.op_type {
            UnaryOpType::Not => {
                let v = v.into_int_value();
                codegen.builder
                    .build_not(v, "")
                    .unwrap()
                    .as_any_value_enum()
            }
            UnaryOpType::Minus => {
                let v = v.into_float_value();
                codegen.builder
                    .build_float_neg(v, "")
                    .unwrap()
                    .as_any_value_enum()
            }
        }
    }
}

#[derive(Debug)]
pub enum UnaryOpType {
    Minus,
    Not,
}

