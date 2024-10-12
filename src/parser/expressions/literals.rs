use super::{AnyExpressionEnum, Expression};
use super::expr_list::ExpressionList;
#[derive(Debug, Clone)]
pub struct StringLiteral {
    pub val: String,
}
impl Expression for StringLiteral {
    fn desugar(self) -> AnyExpressionEnum {
        self.clone().into_any_expression_enum()
    }
    fn into_any_expression_enum(self) -> super::AnyExpressionEnum {
        super::AnyExpressionEnum::StringLiteral(self)
    }
    fn my_clone(&self) -> AnyExpressionEnum {
        StringLiteral {
            val: self.val.clone()
        }.into_any_expression_enum()
    }
}

#[derive(Debug, Clone)]
pub struct NumberLiteral {
    pub val: String,
}
impl Expression for NumberLiteral {
    fn into_any_expression_enum(self) -> super::AnyExpressionEnum {
        super::AnyExpressionEnum::NumberLiteral(self)
    }
    fn desugar(self) -> AnyExpressionEnum {
        self.clone().into_any_expression_enum()
    }
    fn my_clone(&self) -> AnyExpressionEnum {
        NumberLiteral {
            val: self.val.clone()
        }.into_any_expression_enum()
    }
}

#[derive(Debug)]
pub struct ArrayLiteral {
    pub expressions: ExpressionList,
}
impl Expression for ArrayLiteral {
    fn into_any_expression_enum(self) -> super::AnyExpressionEnum {
        super::AnyExpressionEnum::ArrayLiteral(self)
    }
    fn desugar(self) -> AnyExpressionEnum {
        ArrayLiteral {
            expressions: self.expressions.desugar()
        }.into_any_expression_enum()
    }
    fn my_clone(&self) -> AnyExpressionEnum {
        ArrayLiteral {
            expressions: self.expressions.my_clone()
        }.into_any_expression_enum()
    }
}

#[derive(Debug, Clone)]
pub struct False;
impl Expression for False {
    fn into_any_expression_enum(self) -> super::AnyExpressionEnum {
        super::AnyExpressionEnum::False(self)
    }
    fn desugar(self) -> AnyExpressionEnum {
        self.clone().into_any_expression_enum()
    }
    fn my_clone(&self) -> AnyExpressionEnum {
        False{}.into_any_expression_enum()
    }
}

#[derive(Debug, Clone)]
pub struct True;
impl Expression for True {
    fn into_any_expression_enum(self) -> super::AnyExpressionEnum {
        super::AnyExpressionEnum::True(self)
    }
    fn desugar(self) -> AnyExpressionEnum {
        self.clone().into_any_expression_enum()
    }
    fn my_clone(&self) -> AnyExpressionEnum {
        True{}.into_any_expression_enum()
    }
}

#[derive(Debug, Clone)]
pub struct NoneVal;
impl Expression for NoneVal {
    fn into_any_expression_enum(self) -> super::AnyExpressionEnum {
        super::AnyExpressionEnum::NoneVal(self)
    }
    fn desugar(self) -> AnyExpressionEnum {
        self.clone().into_any_expression_enum()
    }
    fn my_clone(&self) -> AnyExpressionEnum {
        NoneVal{}.into_any_expression_enum()
    }
}
