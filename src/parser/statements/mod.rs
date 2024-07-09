use std::fmt::Debug;

use crate::codegen::CodeGen;

pub mod blocks;
pub mod exprs;
pub mod fors;
pub mod func_decls;
pub mod ifs;
pub mod returns;
pub mod var_decls;
pub mod whiles;

pub trait Statement: Debug {
    fn generate_code(&self, _ : &mut CodeGen) {
        println!("generating code..");
    }

    fn desugar(&self) -> Box<dyn Statement>;
}
