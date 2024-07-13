use std::borrow::{Borrow, BorrowMut};

use inkwell::values::BasicValue;

use super::{var_decls::VarDeclaration, AnyStatementEnum, Statement};
#[derive(Debug)]
pub struct FunctionDeclaration {
    pub name: String,
    pub parameters_list: Vec<Box<AnyStatementEnum>>,
    pub body: Box<AnyStatementEnum>,
}
impl Statement for FunctionDeclaration {
    fn desugar(self) -> AnyStatementEnum {
        let body = self.body.desugar().boxed();
        FunctionDeclaration{
            name: self.name.clone(),
            parameters_list: self.parameters_list.into_iter().map(|x| x.desugar().boxed()).collect(),
            body
        }.as_any_statement_enum()
    }
    fn generate_code(&self, codegen : &mut crate::codegen::CodeGen) {
        codegen.increase_scope();
        let prev_bb = codegen.builder.get_insert_block().unwrap();
        let ft = codegen.context.f64_type();
        let mut params = vec![];
        for _ in &self.parameters_list {
            params.push(ft.into());
        }
        let fnt = ft.fn_type(&params, false);
        let func = codegen.module.add_function(&self.name, fnt, None);


        codegen.fun_stack.push(func);
        let bb = codegen.context.append_basic_block(func, "entry");
        codegen.builder.position_at_end(bb);
        for (i, x) in self.parameters_list.iter().enumerate() {
            let v = x.as_var_declaration().unwrap();
            codegen.allocate_variable(&v.identifier);
            let ptr = codegen.get_variable(&v.identifier).unwrap();
            match ptr {
                crate::codegen::VariableReference::Local(x) => {
                    let value = func.get_nth_param(i as u32).unwrap().into_float_value();
                    let _ = codegen.builder.build_store(x, value);
                }
                _ => unreachable!()
            }
        }
        codegen.allocate_variable(".return");
        self.body.generate_code(codegen);
        let return_block = codegen.context.append_basic_block(func, "return");
        let _ = codegen.builder.build_unconditional_branch(return_block);
        codegen.builder.position_at_end(return_block);
        let retptr = codegen.get_variable(".return").unwrap();
        let retptr = match retptr {
            crate::codegen::VariableReference::Local(x) => {
                x
            },
            _ => unreachable!()
        };
        let retv = codegen.builder.build_load(retptr, "ret_value").unwrap();
        let _ = codegen.builder.build_return(Some(&retv));
        codegen.fun_stack.pop();
        codegen.builder.position_at_end(prev_bb);
        codegen.decrease_scope();
    }
    fn as_any_statement_enum(self) -> AnyStatementEnum {
        AnyStatementEnum::FunctionDeclaration(self)
    }
}
