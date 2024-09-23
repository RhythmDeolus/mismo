use std::ops::Deref;

use super::{AnyStatementEnum, Statement};

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
        }.into_any_statement_enum()
    }
    fn generate_code(& self, codegen : &crate::codegen::CodeGen) {
        codegen.increase_scope();
        let prev_bb = codegen.builder.get_insert_block().unwrap();
        codegen.hoist_statements_boxed(&self.body.as_block().unwrap().statements);
        let func = codegen.module.get_function(&self.name).unwrap();


        codegen.push_func_stack(func);
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
        //setting return points
        let prev_block = codegen.builder.get_insert_block().unwrap();
        let mut return_points = codegen.return_points.lock().unwrap();
        for x in &*return_points {
            codegen.builder.position_at_end(*x);
            let _ = codegen.builder.build_unconditional_branch(return_block);
        }
        // whats wrong with this?
        return_points.clear();
        codegen.builder.position_at_end(prev_block);
        codegen.pop_func_stack();
        codegen.builder.position_at_end(prev_bb);
        // clear return points without stuck in loop
        codegen.decrease_scope();
        codegen.print_module();
    }
    fn into_any_statement_enum(self) -> AnyStatementEnum {
        AnyStatementEnum::FunctionDeclaration(self)
    }
}
