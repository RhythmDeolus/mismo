use inkwell::{
    values::{AnyValue, AnyValueEnum},
    IntPredicate,
};

use super::Expression;
#[derive(Debug)]
pub enum BinaryOpType {
    Assign,
    Add,
    Sub,
    Div,
    Mul,
    Or,
    And,
    Greater,
    Less,
    GreatEqual,
    LessEqual,
    EqualEqual,
    NotEqual,
    Index,
    Dot,
}

#[derive(Debug)]
pub struct BinaryOp {
    pub left: Box<dyn Expression>,
    pub right: Box<dyn Expression>,
    pub op_type: BinaryOpType,
}
impl Expression for BinaryOp {
    fn is_assignable(&self) -> bool {
        match self.op_type {
            BinaryOpType::Index => self.left.is_assignable(),
            BinaryOpType::Dot => self.left.is_assignable() && self.right.is_assignable(),
            _ => false,
        }
    }

    fn codegen_expression<'a>(
        &self,
        codegen: &'a crate::codegen::CodeGen,
    ) -> inkwell::values::AnyValueEnum<'a> {
        println!("{:?}", self.op_type);
        match self.op_type {
            BinaryOpType::Assign => {
                let lhs = self.left.get_pointer(codegen);
                let rhs = self.right.codegen_expression(codegen);
                let rhs = rhs.into_float_value();
                let _ = codegen.builder.build_store(lhs, rhs);
                return rhs.as_any_value_enum();
            }
            _ => {}
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
            _ => codegen.context.f64_type().const_zero().as_any_value_enum(),
        }
    }
}
