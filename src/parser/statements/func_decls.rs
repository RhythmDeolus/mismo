use super::Statement;
#[derive(Debug)]
pub struct FunctionDeclaration {
    pub name: String,
    pub parameters_list: Vec<Box<dyn Statement>>,
    pub body: Box<dyn Statement>,
}
impl Statement for FunctionDeclaration {
    fn desugar(&self) -> Box<dyn Statement> {
        let body = self.body.desugar();
        Box::new(FunctionDeclaration{
            name: self.name.clone(),
            parameters_list: self.parameters_list.iter().map(|x| x.desugar()).collect(),
            body
        })
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

        for (i, x) in self.parameters_list.into_iter().enumerate() {
        }

        codegen.fun_stack.push(func);
        let bb = codegen.context.append_basic_block(func, "entry");
        codegen.builder.position_at_end(bb);
        self.body.generate_code(codegen);
        codegen.fun_stack.pop();
        codegen.builder.position_at_end(prev_bb);
        codegen.decrease_scope();
    }
}
