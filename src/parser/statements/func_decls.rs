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
        }.as_any_statement_enum()
    }
    fn generate_code(&self, codegen : &mut crate::codegen::CodeGen) {
        codegen.increase_scope();
        let prev_bb = codegen.builder.get_insert_block().unwrap();
        let ft = codegen.context.f64_type();
        let mut params = vec![];
        for _ in self.parameters_list.iter() {
            params.push(ft.into());
        }
        let fnt = ft.fn_type(&params, false);
        let func = codegen.module.add_function(&self.name, fnt, None);

        // for (i, x) in self.parameters_list.into_iter().enumerate() {
        // }
        //
        codegen.fun_stack.push(func);
        let bb = codegen.context.append_basic_block(func, "entry");
        codegen.builder.position_at_end(bb);
        self.body.generate_code(codegen);
        codegen.fun_stack.pop();
        codegen.builder.position_at_end(prev_bb);
        codegen.decrease_scope();
    }
    fn as_any_statement_enum(self) -> AnyStatementEnum {
        AnyStatementEnum::FunctionDeclaration(self)
    }
}
