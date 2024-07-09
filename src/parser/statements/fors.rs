use crate::parser::expressions::literals::True;

use super::blocks::Block;
use super::exprs::ExpresssionStatement;
use super::whiles::WhileStatement;
use super::Statement;
use super::super::expressions::{
    Expression,
    expr_list::ExpressionList
};
#[derive(Debug)]
pub struct ForStatement {
    pub inital: Option<Box<dyn Statement>>,
    pub check: Option<Box<dyn Expression>>,
    pub change: Option<ExpressionList>,
    pub statement: Box<dyn Statement>,
}
impl Statement for ForStatement {
    fn desugar(&self) -> Box<dyn Statement> {
        let initial = self.inital.as_ref().map(|x| x.desugar());
        let check = self.check.as_ref().map(|x| x.desugar());
        let change = self.change.as_ref().map(|x| x.desugar());
        let statement = self.statement.desugar();
        let mut block_statements = vec![];
        if let Some(x) = initial {
            block_statements.push(x);
        }
        let mut inner_block_statements: Vec<Box<dyn Statement>> = vec![];
        inner_block_statements.push(
            Box::new(Block {
                statements: vec![statement]
            })
        );
        if let Some(x) = change {
            for t in x.expressions {
                inner_block_statements.push(Box::new(
                    ExpresssionStatement {
                        expression: t
                    }
                ));
            }
        }
        block_statements.push(
            Box::new(
                WhileStatement {
                    expression: check.unwrap_or(Box::new(True)),
                    statement: Box::new(Block {
                        statements: inner_block_statements,
                    })
                }
            )
        );
        Box::new(Block {
            statements: block_statements
        })
    }
}
