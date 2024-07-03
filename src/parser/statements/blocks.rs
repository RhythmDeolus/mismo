use super::Statement;
#[derive(Debug)]
pub struct Block {
    pub statements: Vec<Box<dyn Statement>>,
}
impl Statement for Block {
    fn generate_code(&self, codegen: &mut crate::codegen::CodeGen) {
        for stmt in self.statements.iter() {
            stmt.generate_code(codegen)
        }
    }
}
