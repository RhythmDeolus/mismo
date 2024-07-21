
use inkwell::values::AnyValue;

use super::{AnyExpressionEnum, Expression};
#[derive(Debug)]
pub struct UnaryOp {
    pub operand: Box<AnyExpressionEnum>,
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
    fn into_any_expression_enum(self) -> AnyExpressionEnum {
        AnyExpressionEnum::Unary(self)
    }
    fn desugar(self) -> AnyExpressionEnum {
        let operand = Box::new(self.operand.desugar());
        UnaryOp {
            operand,
            op_type: self.op_type
        }.into_any_expression_enum()
    }
    fn my_clone(&self) -> AnyExpressionEnum {
        UnaryOp {
            op_type: self.op_type,
            operand: self.operand.my_clone().boxed()
        }.into_any_expression_enum()
    }
}

#[derive(Debug, Clone, Copy)]
pub enum UnaryOpType {
    Minus,
    Not,
}

