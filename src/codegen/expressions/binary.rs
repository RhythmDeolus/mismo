use crate::parser::expressions::binary::{BinaryOp, BinaryOpType};

use super::GenerateExpr;

use inkwell::values::AnyValue;

impl GenerateExpr for BinaryOp {
    fn codegen_expression<'a>(
        &self,
        codegen: &'a crate::codegen::CodeGen,
    ) -> inkwell::values::AnyValueEnum<'a> {
        if let BinaryOpType::Assign = self.op_type {
            let lhs = self.left.get_pointer(codegen);
            let rhs = self.right.codegen_expression(codegen);
            let rhs = rhs.into_float_value();
            match lhs {
                crate::codegen::VariableReference::Local(lhs) => {
                    let _ = codegen.builder.build_store(lhs, rhs);
                }
                crate::codegen::VariableReference::Global(lhs) => {
                    let ptr = lhs.as_pointer_value();
                    let _ = codegen.builder.build_store(ptr, rhs);
                }
            }
            return rhs.as_any_value_enum();
        }
        let lhs = self.left.codegen_expression(codegen);
        let rhs = self.right.codegen_expression(codegen);
        match self.op_type {
            BinaryOpType::Add => {
                let lhs = lhs.into_float_value();
                let rhs = rhs.into_float_value();
                let o = codegen.builder.build_float_add(lhs, rhs, "");
                o.unwrap().as_any_value_enum()
            }
            BinaryOpType::Sub => {
                let lhs = lhs.into_float_value();
                let rhs = rhs.into_float_value();
                let o = codegen.builder.build_float_sub(lhs, rhs, "");
                o.unwrap().as_any_value_enum()
            }
            BinaryOpType::Mul => {
                let lhs = lhs.into_float_value();
                let rhs = rhs.into_float_value();
                codegen
                    .builder
                    .build_float_mul(lhs, rhs, "")
                    .unwrap()
                    .as_any_value_enum()
            }
            BinaryOpType::Div => {
                let lhs = lhs.into_float_value();
                let rhs = rhs.into_float_value();
                codegen
                    .builder
                    .build_float_div(lhs, rhs, "")
                    .unwrap()
                    .as_any_value_enum()
            }
            BinaryOpType::Less => {
                let lhs = lhs.into_float_value();
                let rhs = rhs.into_float_value();
                codegen
                    .builder
                    .build_float_compare(inkwell::FloatPredicate::ULT, lhs, rhs, "")
                    .unwrap()
                    .as_any_value_enum()
            }
            BinaryOpType::Greater => {
                let lhs = lhs.into_float_value();
                let rhs = rhs.into_float_value();
                codegen
                    .builder
                    .build_float_compare(inkwell::FloatPredicate::UGT, lhs, rhs, "")
                    .unwrap()
                    .as_any_value_enum()
            }
            BinaryOpType::LessEqual => {
                let lhs = lhs.into_float_value();
                let rhs = rhs.into_float_value();
                codegen
                    .builder
                    .build_float_compare(inkwell::FloatPredicate::ULE, lhs, rhs, "")
                    .unwrap()
                    .as_any_value_enum()
            }
            BinaryOpType::GreatEqual => {
                let lhs = lhs.into_float_value();
                let rhs = rhs.into_float_value();
                codegen
                    .builder
                    .build_float_compare(inkwell::FloatPredicate::UGE, lhs, rhs, "")
                    .unwrap()
                    .as_any_value_enum()
            }
            BinaryOpType::NotEqual => {
                let lhs = lhs.into_float_value();
                let rhs = rhs.into_float_value();
                codegen
                    .builder
                    .build_float_compare(inkwell::FloatPredicate::UNE, lhs, rhs, "")
                    .unwrap()
                    .as_any_value_enum()
            }
            BinaryOpType::EqualEqual => {
                let lhs = lhs.into_float_value();
                let rhs = rhs.into_float_value();
                codegen
                    .builder
                    .build_float_compare(inkwell::FloatPredicate::UEQ, lhs, rhs, "")
                    .unwrap()
                    .as_any_value_enum()
            }
            BinaryOpType::Or => {
                let lhs = lhs.into_int_value();
                let rhs = rhs.into_int_value();
                codegen
                    .builder
                    .build_or(lhs, rhs, "")
                    .unwrap()
                    .as_any_value_enum()
            }
            BinaryOpType::And => {
                let lhs = lhs.into_int_value();
                let rhs = rhs.into_int_value();
                codegen
                    .builder
                    .build_and(lhs, rhs, "")
                    .unwrap()
                    .as_any_value_enum()
            }
            BinaryOpType::Mod => {
                let lhs = lhs.into_float_value();
                let rhs = rhs.into_float_value();
                codegen.builder.build_float_rem(lhs, rhs, "")
                    .unwrap()
                    .as_any_value_enum()
            }
            _ => todo!()
        }
    }
}
