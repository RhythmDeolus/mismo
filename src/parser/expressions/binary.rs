use super::{AnyExpressionEnum, Expression};
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
    pub left: Box<AnyExpressionEnum>,
    pub right: Box<AnyExpressionEnum>,
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

    fn into_any_expression_enum(self) -> AnyExpressionEnum {
        AnyExpressionEnum::Binary(self)
    }

    fn my_clone(&self) -> AnyExpressionEnum {
        BinaryOp {
            left: self.left.my_clone().boxed(),
            right: self.right.my_clone().boxed(),
            op_type: self.op_type
        }.into_any_expression_enum()
    }


    fn desugar(self) -> AnyExpressionEnum {
        let left = Box::new(self.left.desugar());
        let right = Box::new(self.right.desugar());
        match self.op_type {
            BinaryOpType::PlusEqual
            | BinaryOpType::MinusEqual
            | BinaryOpType::MulEqual
            | BinaryOpType::DivEqual => {
                BinaryOp  {
                    left: Box::new(left.my_clone()),
                    op_type: BinaryOpType::Assign,
                    right: Box::new(BinaryOp {
                                            left,
                                            right,
                                            op_type: BinaryOp::map_assign_op_to(self.op_type)
                                        }.into_any_expression_enum())
                }.into_any_expression_enum()
            }
            _ => BinaryOp {
                left,
                right,
                op_type: self.op_type
            }.into_any_expression_enum()
        }
    }
}
