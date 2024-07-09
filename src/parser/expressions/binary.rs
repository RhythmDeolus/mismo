use inkwell::values::AnyValue;

use super::Expression;
#[derive(Debug, Clone, Copy)]
pub enum BinaryOpType{
    Assign,
    PlusEqual,
    MinusEqual,
    MulEqual,
    DivEqual,
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
    Mod,
}

#[derive(Debug)]
pub struct BinaryOp {
    pub left: Box<dyn Expression>,
    pub right: Box<dyn Expression>,
    pub op_type: BinaryOpType,
}
impl BinaryOp {
    pub fn map_assign_op_to(op_type: BinaryOpType) -> BinaryOpType {
        match op_type {
            BinaryOpType::PlusEqual => BinaryOpType::Add,
            BinaryOpType::MinusEqual => BinaryOpType::Sub,
            BinaryOpType::MulEqual => BinaryOpType::Mul,
            BinaryOpType::DivEqual => BinaryOpType::Div,
            _ => op_type
        }
    }
}
impl Expression for BinaryOp {
    fn is_assignable(&self) -> bool {
        match self.op_type {
            BinaryOpType::Index => self.left.is_assignable(),
            BinaryOpType::Dot => self.left.is_assignable() && self.right.is_assignable(),
            _ => false,
        }
    }

    fn my_clone(&self) -> Box<dyn Expression> { 
        Box::new(BinaryOp {
                    left: self.left.my_clone(),
                    right: self.right.my_clone(),
                    op_type: self.op_type
                })
    }

    fn codegen_expression<'a>(
        &self,
        codegen: &'a crate::codegen::CodeGen,
    ) -> inkwell::values::AnyValueEnum<'a> {
        println!("{:?}", self.op_type);
        if let BinaryOpType::Assign = self.op_type {
            let lhs = self.left.get_pointer(codegen);
            let rhs = self.right.codegen_expression(codegen);
            let rhs = rhs.into_float_value();
            let _ = codegen.builder.build_store(lhs, rhs);
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

    fn desugar(&self) -> Box<dyn Expression> {
        let left = self.left.desugar();
        let right = self.right.desugar();
        match self.op_type {
            BinaryOpType::PlusEqual
            | BinaryOpType::MinusEqual
            | BinaryOpType::MulEqual
            | BinaryOpType::DivEqual => {
                Box::new(BinaryOp  {
                    left: left.my_clone(),
                    op_type: BinaryOpType::Assign,
                    right: Box::new(BinaryOp {
                        left,
                        right,
                        op_type: BinaryOp::map_assign_op_to(self.op_type)
                    })
                })
            }
            _ => Box::new(BinaryOp {
                left,
                right,
                op_type: self.op_type
            })
        }
    }
}
