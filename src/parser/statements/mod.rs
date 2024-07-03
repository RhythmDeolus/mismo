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
    fn generate_code(&self, codegen: &mut CodeGen) {
        println!("generating code..");
    }
}
