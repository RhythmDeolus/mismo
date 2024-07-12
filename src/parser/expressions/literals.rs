use inkwell::values::AnyValue;

use super::{AnyExpressionEnum, Expression};
use super::expr_list::ExpressionList;
#[derive(Debug, Clone)]
pub struct StringLiteral {
    pub val: String,
}
impl Expression for StringLiteral {
    fn desugar(self) -> AnyExpressionEnum {
        self.clone().as_any_expression_enum()
    }
    fn as_any_expression_enum(self) -> super::AnyExpressionEnum {
        super::AnyExpressionEnum::StringLiteral(self)
    }
    fn my_clone(&self) -> AnyExpressionEnum {
        StringLiteral {
            val: self.val.clone()
        }.as_any_expression_enum()
    }
}

#[derive(Debug, Clone)]
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
    fn as_any_expression_enum(self) -> super::AnyExpressionEnum {
        super::AnyExpressionEnum::NumberLiteral(self)
    }
    fn desugar(self) -> AnyExpressionEnum {
        self.clone().as_any_expression_enum()
    }
    fn my_clone(&self) -> AnyExpressionEnum {
        NumberLiteral {
            val: self.val.clone()
        }.as_any_expression_enum()
    }
}

#[derive(Debug)]
pub struct ArrayLiteral {
    pub expressions: ExpressionList,
}
impl Expression for ArrayLiteral {
    fn as_any_expression_enum(self) -> super::AnyExpressionEnum {
        super::AnyExpressionEnum::ArrayLiteral(self)
    }
    fn desugar(self) -> AnyExpressionEnum {
        ArrayLiteral {
            expressions: self.expressions.desugar()
        }.as_any_expression_enum()
    }
    fn my_clone(&self) -> AnyExpressionEnum {
        ArrayLiteral {
            expressions: self.expressions.my_clone()
        }.as_any_expression_enum()
    }
}

#[derive(Debug, Clone)]
pub struct False;
impl Expression for False {
    fn as_any_expression_enum(self) -> super::AnyExpressionEnum {
        super::AnyExpressionEnum::False(self)
    }
    fn codegen_expression<'ctx>(&self, codegen: &'ctx crate::codegen::CodeGen) -> inkwell::values::AnyValueEnum<'ctx>{
        codegen.context.bool_type().const_zero().as_any_value_enum()
    }
    fn desugar(self) -> AnyExpressionEnum {
        self.clone().as_any_expression_enum()
    }
    fn my_clone(&self) -> AnyExpressionEnum {
        False{}.as_any_expression_enum()
    }
}

#[derive(Debug, Clone)]
pub struct True;
impl Expression for True {
    fn as_any_expression_enum(self) -> super::AnyExpressionEnum {
        super::AnyExpressionEnum::True(self)
    }
    fn codegen_expression<'ctx>(&self, codegen: &'ctx crate::codegen::CodeGen) -> inkwell::values::AnyValueEnum<'ctx>{
        codegen.context.bool_type().const_all_ones().as_any_value_enum()
    }
    fn desugar(self) -> AnyExpressionEnum {
        self.clone().as_any_expression_enum()
    }
    fn my_clone(&self) -> AnyExpressionEnum {
        True{}.as_any_expression_enum()
    }
}

#[derive(Debug, Clone)]
pub struct NoneVal;
impl Expression for NoneVal {
    fn as_any_expression_enum(self) -> super::AnyExpressionEnum {
        super::AnyExpressionEnum::NoneVal(self)
    }
    fn desugar(self) -> AnyExpressionEnum {
        self.clone().as_any_expression_enum()
    }
    fn my_clone(&self) -> AnyExpressionEnum {
        NoneVal{}.as_any_expression_enum()
    }
}
