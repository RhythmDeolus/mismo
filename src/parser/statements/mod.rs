use std::fmt::Debug;

use self::{
    blocks::Block, exprs::ExpresssionStatement, fors::ForStatement, func_decls::FunctionDeclaration, ifs::IfStatement, returns::ReturnStatement, var_decls::VarDeclaration, whiles::WhileStatement
};

pub mod blocks;
pub mod exprs;
pub mod fors;
pub mod func_decls;
pub mod ifs;
pub mod returns;
pub mod var_decls;
pub mod whiles;

#[derive(Debug)]
pub enum AnyStatementEnum {
    Block(Block),
    For(ForStatement),
    FunctionDeclaration(FunctionDeclaration),
    If(IfStatement),
    Return(ReturnStatement),
    VarDeclaration(VarDeclaration),
    While(WhileStatement),
    Expression(ExpresssionStatement),
}

impl AnyStatementEnum {
    pub fn desugar(self) -> AnyStatementEnum {
        match  self {
            AnyStatementEnum::Block(x) => x.desugar(),
            AnyStatementEnum::For(x) => x.desugar(),
            AnyStatementEnum::FunctionDeclaration(x) => x.desugar(),
            AnyStatementEnum::If(x) => x.desugar(),
            AnyStatementEnum::Return(x) => x.desugar(),
            AnyStatementEnum::VarDeclaration(x) => x.desugar(),
            AnyStatementEnum::While(x) => x.desugar(),
            AnyStatementEnum::Expression(x) => x.desugar(),
        }
    }
    pub fn boxed(self) -> Box<Self> {
        Box::new(self)
    }


    pub fn as_var_declaration(&self) -> Option<&VarDeclaration> {
        match self {
            AnyStatementEnum::VarDeclaration(x) => Some(x),
            _ => None
        }
    }
    pub fn as_block(&self) -> Option<&Block> {
        match self {
            AnyStatementEnum::Block(x) => Some(x),
            _ => None
        }
    }
}


pub trait Statement: Debug {

    fn desugar(self) -> AnyStatementEnum;
    fn into_any_statement_enum(self) -> AnyStatementEnum;
}
