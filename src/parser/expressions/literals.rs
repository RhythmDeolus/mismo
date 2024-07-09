use inkwell::values::AnyValue;

use super::Expression;
use super::expr_list::ExpressionList;
#[derive(Debug, Clone)]
pub struct StringLiteral {
    pub val: String,
}
impl Expression for StringLiteral {
    fn desugar(&self) -> Box<dyn Expression> {
        Box::new(self.clone())
    }
    fn my_clone(&self) -> Box<dyn Expression> {
        Box::new(StringLiteral {
                    val: self.val.clone()
                })
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
    fn desugar(&self) -> Box<dyn Expression> {
        Box::new(self.clone())
    }
    fn my_clone(&self) -> Box<dyn Expression> {
        Box::new(NumberLiteral {
                    val: self.val.clone()
                })
    }
}

#[derive(Debug)]
pub struct ArrayLiteral {
    pub expressions: ExpressionList,
}
impl Expression for ArrayLiteral {
    fn desugar(&self) -> Box<dyn Expression> {
        Box::new(ArrayLiteral {
            expressions: self.expressions.desugar()
        })
    }
    fn my_clone(&self) -> Box<dyn Expression> {
        Box::new(ArrayLiteral {
                    expressions: self.expressions.my_clone()
                })
    }
}

#[derive(Debug, Clone)]
pub struct False;
impl Expression for False {
    fn codegen_expression<'ctx>(&self, codegen: &'ctx crate::codegen::CodeGen) -> inkwell::values::AnyValueEnum<'ctx>{
        codegen.context.bool_type().const_zero().as_any_value_enum()
    }
    fn desugar(&self) -> Box<dyn Expression> {
        Box::new(self.clone())
    }
    fn my_clone(&self) -> Box<dyn Expression> {
        Box::new(False{})
    }
}

#[derive(Debug, Clone)]
pub struct True;
impl Expression for True {
    fn codegen_expression<'ctx>(&self, codegen: &'ctx crate::codegen::CodeGen) -> inkwell::values::AnyValueEnum<'ctx>{
        codegen.context.bool_type().const_all_ones().as_any_value_enum()
    }
    fn desugar(&self) -> Box<dyn Expression> {
        Box::new(self.clone())
    }
    fn my_clone(&self) -> Box<dyn Expression> {
        Box::new(True{})
    }
}

#[derive(Debug, Clone)]
pub struct NoneVal;
impl Expression for NoneVal {
    fn desugar(&self) -> Box<dyn Expression> {
        Box::new(self.clone())
    }
    fn my_clone(&self) -> Box<dyn Expression> {
        Box::new(NoneVal{})
    }
}
