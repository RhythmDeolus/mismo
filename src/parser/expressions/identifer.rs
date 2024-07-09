use inkwell::values::AnyValue;

use super::Expression;
#[derive(Debug, Clone)]
pub struct Identifier {
    pub name: String,
}
impl Expression for Identifier {
    fn is_assignable(&self) -> bool {
        true
    }
    fn codegen_expression<'a>(&self, codegen: &'a crate::codegen::CodeGen) -> inkwell::values::AnyValueEnum<'a>{
        let ptr =codegen.get_variable(&self.name);
        let ptr = ptr.unwrap();
        let f = codegen.builder.build_load(ptr, "").unwrap();
        f.as_any_value_enum()
    }
    fn get_pointer<'a>(&self, codegen: &'a crate::codegen::CodeGen) ->inkwell::values::PointerValue<'a> {
        let ptr = codegen.get_variable(&self.name);
        ptr.unwrap()
    }
    fn desugar(&self) -> Box<dyn Expression> {
        Box::new(self.clone())
    }
    fn my_clone(&self) -> Box<dyn Expression> {
        Box::new(Identifier {
                    name: self.name.clone()
                })
    }
}

