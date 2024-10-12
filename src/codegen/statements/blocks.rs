use crate::parser::statements::blocks::Block;

use super::Generate;


impl Generate for Block {
    fn generate_code(& self, codegen : &crate::codegen::CodeGen) {
        codegen.increase_scope();
        // hoisting functions
        codegen.hoist_statements_boxed(&self.statements);

        for stmt in self.statements.iter() {
            stmt.generate_code(codegen)
        }
        codegen.decrease_scope();
    }
}
