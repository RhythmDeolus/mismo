use std::fmt::Debug;

use crate::parser::statements::AnyStatementEnum;

pub mod blocks;
pub mod exprs;
pub mod fors;
pub mod func_decls;
pub mod ifs;
pub mod returns;
pub mod var_decls;
pub mod whiles;

impl Generate for AnyStatementEnum {
    fn generate_code(& self, codegen : &crate::codegen::CodeGen) {
        match self {
            AnyStatementEnum::Block(x) => x.generate_code(codegen),
            AnyStatementEnum::For(x) => x.generate_code(codegen),
            AnyStatementEnum::FunctionDeclaration(x) => x.generate_code(codegen),
            AnyStatementEnum::If(x) => x.generate_code(codegen),
            AnyStatementEnum::Return(x) => x.generate_code(codegen),
            AnyStatementEnum::VarDeclaration(x) => x.generate_code(codegen),
            AnyStatementEnum::While(x) => x.generate_code(codegen),
            AnyStatementEnum::Expression(x) => x.generate_code(codegen),
        };
    }
}

pub trait Generate: Debug {
    fn generate_code(& self, _ : &crate::codegen::CodeGen) {
        println!("generating code..");
    }
}
