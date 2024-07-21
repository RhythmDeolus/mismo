use crate::parser::expressions::literals::True;
use crate::parser::expressions::AnyExpressionEnum;

use super::super::expressions::{expr_list::ExpressionList, Expression};
use super::blocks::Block;
use super::exprs::ExpresssionStatement;
use super::whiles::WhileStatement;
use super::{AnyStatementEnum, Statement};
#[derive(Debug)]
pub struct ForStatement {
    pub inital: Option<Box<AnyStatementEnum>>,
    pub check: Option<Box<AnyExpressionEnum>>,
    pub change: Option<ExpressionList>,
    pub statement: Box<AnyStatementEnum>,
}
impl Statement for ForStatement {
    fn into_any_statement_enum(self) -> AnyStatementEnum {
        AnyStatementEnum::For(self)
    }
    fn desugar(self) -> AnyStatementEnum {
        let initial = self.inital.map(|x| x.desugar());
        let check = self.check.map(|x| x.desugar());
        let change = self.change.map(|x| x.desugar());
        let statement = self.statement.desugar();
        let mut block_statements = vec![];
        if let Some(x) = initial {
            block_statements.push(x.boxed());
        }
        let mut inner_block_statements = vec![];
        inner_block_statements.push(Block {
                    statements: vec![statement.boxed()],
                }.into_any_statement_enum().boxed());
        if let Some(x) = change {
            for t in x.expressions {
                inner_block_statements.push(ExpresssionStatement { expression: t }.into_any_statement_enum().boxed());
            }
        }
        block_statements.push(WhileStatement {
            expression: check.unwrap_or(True.into_any_expression_enum()).boxed(),
            statement: Block {
                statements: inner_block_statements,
            }.into_any_statement_enum().boxed(),
        }.into_any_statement_enum().boxed());
        Block {
            statements: block_statements,
        }.into_any_statement_enum()
    }
}
