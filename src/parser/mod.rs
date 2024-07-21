use std::fmt::Display;
use std::fmt::Debug;

use crate::tokenizer::token::{Token, TokenType};

pub mod statements;
pub mod expressions;

use statements::{
    Statement,
    blocks::Block,
    exprs::ExpresssionStatement,
    fors::ForStatement,
    func_decls::FunctionDeclaration,
    ifs::IfStatement,
    returns::ReturnStatement,
    var_decls::VarDeclaration,
    whiles::WhileStatement
};

use expressions::{
    Expression,
    binary::{
        BinaryOpType,
        BinaryOp
    },
    unary::{
        UnaryOpType,
        UnaryOp
    },
    identifer::Identifier,
    calls::{
        InbuiltCallTypes,
        InbuiltCall,
        Call,
    },
    literals::{
        True,
        False,
        NoneVal,
        ArrayLiteral,
        NumberLiteral,
        StringLiteral
    },
    expr_list::ExpressionList
};

use self::expressions::AnyExpressionEnum;
use self::statements::AnyStatementEnum;

// type Possible<T> = Result<Box<T>, CompilerError>;
type PossibleStatement = Result<AnyStatementEnum, CompilerError>;
type PossibleExpression = Result<AnyExpressionEnum, CompilerError>;

#[derive(Debug, Clone)]
pub struct CompilerError {
    pub message: String,
    pub line_no: Option<usize>
}


impl Display for CompilerError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}




pub enum NodeTypes {
    String,
    Number,
    Boolean,
    NoneType,
    VarDeclaration,
    BinaryOp,
    UnaryOp,
    Grouping,
    InBuiltCall,
    IfStatement,
    Block,
    Identifier,
    WhileLoop,
    ForLoop,
    ExprList,
    VarExprList,
    FunDeclare,
    FunCall,
    ReturnStmt,
    ArrayLiteral,
    Array,
}

#[derive(Debug)]
pub enum ParserStatus {
    Ok,
    Failure
}

#[derive(Debug)]
pub struct Parser {
    pub tokens: Vec<Token>,
    pub curr_token: usize,
    pub fun_scope: usize,
    pub errors: Vec<CompilerError>,
    pub status: ParserStatus
}

impl Parser {
    pub fn create(tokens: Vec<Token>) -> Self {
        Parser {
            tokens,
            curr_token: 0,
            fun_scope: 0,
            errors: vec![],
            status: ParserStatus::Ok
        }
    }

    fn is_eot(&self) -> bool {
        self.tokens.len() <= self.curr_token
    }

    fn advance(&mut self) {
        if !self.is_eot() {
            self.curr_token += 1;
        }
    }

    fn match_token(&mut self, tt: TokenType) -> bool {
        match self.peek() {
            None => false,
            Some(x) => {
                if x.t_type == tt {
                    self.advance();
                    return true;
                }
                false
            }
        }
    }

    fn peek(&self) -> Option<&Token> {
        if self.is_eot() {
            return None;
        }
        Some(&self.tokens[self.curr_token])
    }

    fn consume(&mut self, tt: TokenType, s: &str) -> Result<(), CompilerError> {
        if self.inverse_match_token(tt)? {
            self.error(s)?;
        }
        Ok(())
    }

    fn previous(&self) -> Option<&Token> {
        if self.curr_token == 0 || self.tokens.is_empty() {
            return None;
        }
        Some(&self.tokens[self.curr_token - 1])
    }

    fn var_declaration(&mut self) -> PossibleStatement {
        self.consume(TokenType::Identifer, "Expected an identifer")?;
        let i = self.previous_literal();
        let mut e = None;

        if self.match_token(TokenType::Equal) {
            e = Some(Box::new(self.or()?));
        }


        Ok(VarDeclaration {
            identifier: i,
            expression: e,
        }.into_any_statement_enum())
    }

    fn expression(&mut self) -> PossibleExpression {
        self.assign()
    }

    fn assign(&mut self) -> PossibleExpression {
        let mut e = self.or()?;
        while self.match_token(TokenType::Equal)
        || self.match_token(TokenType::PlusEqual) 
        || self.match_token(TokenType::MinusEqual)
        || self.match_token(TokenType::MulEqual)
        || self.match_token(TokenType::SlashEqual)
        {
            let ptt = self.previous_token_type();
            if e.is_assignable() {
                e = BinaryOp {
                    left: Box::new(e),
                    right: Box::new(self.or()?),
                    op_type: Parser::map_to_boptype(ptt).unwrap(),
                }.into_any_expression_enum();
            } else {
                self.error("Can't assign.")?
            }
        }
        Ok(e)
    }

    // TODO
    // fn assign_list(&mut self) -> Box<dyn Expression> {
    //     let mut e = self.expression();
    //     if self.match_token(TokenType::Comma) {
    //         let mut el = self.expression_list();
    //         el.expressions.insert(0, e);
    //         while(self.match_token(TokenType::Equal)) {
    //             let t = self.previous_token_type();
    //             let right = self.expression_list();
    //             if el.is_assignable() {
    //                 el = Box::new(BinaryOp {
    //                                     left: el,
    //                                     right: right,
    //                                     op_type: t,
    //                                 })
    //             }
    //         }
    //     }
    //     e
    // }
    //
    fn or(&mut self) -> PossibleExpression {
        let mut e = self.and()?;
        while self.match_token(TokenType::Or) {
            let ptt = self.previous_token_type();
            let a = self.and();
            e = BinaryOp {
                left: Box::new(e),
                right: Box::new(a?),
                op_type: Parser::map_to_boptype(ptt).unwrap(),
            }.into_any_expression_enum();
        }
        Ok(e)
    }

    fn and(&mut self) -> PossibleExpression {
        let mut e = self.equality()?;
        while self.match_token(TokenType::And) {
            let ptt = self.previous_token_type();
            let a = self.equality();
            e = BinaryOp {
                left: Box::new(e),
                right: Box::new(a?),
                op_type: Parser::map_to_boptype(ptt).unwrap(),
            }.into_any_expression_enum();
        }
        Ok(e)
    }

    fn _match_equality_token(&mut self) -> bool {
        let p = self.peek();
        match p {
            None => false,
            Some(x) => match x.t_type {
                TokenType::EqualEqual
                | TokenType::BangEqual
                | TokenType::LessEqual
                | TokenType::Less
                | TokenType::Greater
                | TokenType::GreatEqual => {
                    self.advance();
                    true
                }
                _ => false,
            },
        }
    }

    fn map_to_boptype(tt: TokenType) -> Option<BinaryOpType> {
        let x = match tt {
            TokenType::Plus => BinaryOpType::Add,
            TokenType::Minus => BinaryOpType::Sub,
            TokenType::Slash => BinaryOpType::Div,
            TokenType::Mul => BinaryOpType::Mul,
            TokenType::Or => BinaryOpType::Or,
            TokenType::And => BinaryOpType::And,
            TokenType::Greater => BinaryOpType::Greater,
            TokenType::Less => BinaryOpType::Less,
            TokenType::GreatEqual => BinaryOpType::GreatEqual,
            TokenType::LessEqual => BinaryOpType::LessEqual,
            TokenType::EqualEqual => BinaryOpType::EqualEqual,
            TokenType::BangEqual => BinaryOpType::NotEqual,
            TokenType::OpenSquare => BinaryOpType::Index,
            TokenType::Dot => BinaryOpType::Dot,
            TokenType::Equal => BinaryOpType::Assign,
            TokenType::PlusEqual => BinaryOpType::PlusEqual,
            TokenType::MinusEqual => BinaryOpType::MinusEqual,
            TokenType::MulEqual => BinaryOpType::MulEqual,
            TokenType::SlashEqual => BinaryOpType::DivEqual,
            TokenType::Mod => BinaryOpType::Mod,
            _ => return None,
        };
        Some(x)
    }

    fn equality(&mut self) -> PossibleExpression {
        let mut e = self.term()?;
        while self._match_equality_token() {
            let ptt = self.previous_token_type();
            let a = self.term();
            e = BinaryOp {
                left: Box::new(e),
                right: Box::new(a?),
                op_type: Parser::map_to_boptype(ptt).unwrap(),
            }.into_any_expression_enum();
        }
        Ok(e)
    }

    // + or -
    fn term(&mut self) -> PossibleExpression {
        let mut e = self.factor()?;
        while self.match_token(TokenType::Plus) || self.match_token(TokenType::Minus) {
            let ptt = self.previous_token_type();
            let a = self.factor();
            e = BinaryOp {
                left: Box::new(e),
                right: Box::new(a?),
                op_type: Parser::map_to_boptype(ptt).unwrap(),
            }.into_any_expression_enum()
        }

        Ok(e)
    }

    fn factor(&mut self) -> PossibleExpression {
        let mut e = self.unary()?;
        while self.match_token(TokenType::Mul) 
        || self.match_token(TokenType::Slash) 
        || self.match_token(TokenType::Mod)
        {
            let ptt = self.previous_token_type();
            let a = self.unary();
            e = BinaryOp {
                left: Box::new(e),
                right: Box::new(a?),
                op_type: Parser::map_to_boptype(ptt).unwrap(),
            }.into_any_expression_enum();
        }

        Ok(e)
    }

    fn map_to_uoptype(tt: TokenType) -> Option<UnaryOpType> {
        let x = match tt {
            TokenType::Bang => UnaryOpType::Not,
            TokenType::Minus => UnaryOpType::Minus,
            _ => return None,
        };
        Some(x)
    }

    fn unary(&mut self) -> PossibleExpression {
        if self.match_token(TokenType::Minus) || self.match_token(TokenType::Bang) {
            let ptt = self.previous_token_type();
            Ok(UnaryOp {
                operand: Box::new(self.unary()?),
                op_type: Parser::map_to_uoptype(ptt).unwrap(),
            }.into_any_expression_enum())
        } else {
            self.index()
        }

    }

    fn previous_token_type(&self) -> TokenType {
        self.previous().unwrap().t_type
    }

    fn index(&mut self) -> PossibleExpression {
        let mut e = self.dot()?;
        while self.match_token(TokenType::OpenSquare) {
            let ptt = self.previous_token_type();
            let a = self.term();
            self.consume(TokenType::CloseSquare, "Expected a ']'")?;
            e = BinaryOp {
                left: Box::new(e),
                right: Box::new(a?),
                op_type: Parser::map_to_boptype(ptt).unwrap(),
            }.into_any_expression_enum();
        }
        Ok(e)
    }

    fn dot(&mut self) -> PossibleExpression {
        let mut e = self.primary()?;
        while self.match_token(TokenType::Dot) {
            let ptt = self.previous().unwrap().t_type;
            let a = self.primary();
            e = BinaryOp {
                left: Box::new(e),
                right: Box::new(a?),
                op_type: Parser::map_to_boptype(ptt).unwrap(),
            }.into_any_expression_enum()
        }

        Ok(e)
    }

    fn previous_literal(&self) -> String {
        self.previous().unwrap().literal.clone()
    }

    fn inverse_match_token(&mut self, tt: TokenType) -> Result<bool, CompilerError> {
        match self.peek() {
            None => self.error("Unexpected End of tokens"),
            Some(x) => {
                if x.t_type == tt {
                    self.advance();
                    return Ok(false);
                }
                Ok(true)
            }
        }
    }

    fn primary(&mut self) -> PossibleExpression {
        let v = 
        if self.match_token(TokenType::False) {
            AnyExpressionEnum::False(False {})
        } else if self.match_token(TokenType::True) {
            AnyExpressionEnum::True(True {})
        } else if self.match_token(TokenType::None) {
            AnyExpressionEnum::NoneVal(NoneVal {})
        } else if self.match_token(TokenType::String) {
            AnyExpressionEnum::StringLiteral(StringLiteral {
                val: self.previous_literal(),
            })
        } else if self.match_token(TokenType::Number) {
            AnyExpressionEnum::NumberLiteral(NumberLiteral {
                val: self.previous_literal(),
            })
        } else if self.match_token(TokenType::Identifer) {
            let identifier = self.previous_literal();
            if self.match_token(TokenType::OpenParen) {
                let mut el = ExpressionList {
                    expressions: vec![],
                };
                if self.inverse_match_token(TokenType::CloseParen)? {
                    el = self.expression_list()?;
                    self.consume(TokenType::CloseParen, "Expected a ')'")?;
                }
                AnyExpressionEnum::Call(Call {
                    left: identifier.clone(),
                    arguments: el,
                })
            } else {
                AnyExpressionEnum::Identifier(Identifier { name: identifier })
            }
        } else if self.match_token(TokenType::OpenParen) {
            let e1 = self.expression()?;
            self.consume(TokenType::CloseParen, "Expected a ')'")?;
            e1
        } else if self.match_token(TokenType::OpenSquare) {
            let el = self.expression_list()?;
            self.consume(TokenType::CloseSquare, "Expected a ']")?;
            AnyExpressionEnum::ArrayLiteral(ArrayLiteral { expressions: el })
        } else {
            return self.error("Expected an expression");
        };
        Ok(v)
    }

    fn error<T>(&self, s: &str) -> Result<T, CompilerError>{
        let p = self.peek().unwrap_or_else(|| self.previous().unwrap());
        Err(CompilerError {
            message: format!("{}, but got {}: {} at line no: {}, col no: {}", s, p.t_type, p.literal, p.line_no, p.col_no),
            line_no: Some(p.line_no)
        })
    }

    fn expression_list(&mut self) -> Result<ExpressionList, CompilerError> {
        let mut el = ExpressionList {
            expressions: vec![],
        };
        let mut e = self.expression()?;
        el.expressions.push(Box::new(e));
        while self.match_token(TokenType::Comma) {
            e = self.expression()?;
            el.expressions.push(Box::new(e));
        }

        Ok(el)
    }

    fn _map_token_to_inbuiltcall(tt: TokenType) -> Option<InbuiltCallTypes> {
        let x = match tt {
            TokenType::Print => InbuiltCallTypes::Print,
            _ => return None,
        };
        Some(x)
    }

    fn print_statement(&mut self) -> PossibleStatement {
        let op = Parser::_map_token_to_inbuiltcall(self.previous_token_type());
        let el = self.expression_list()?;
        Ok(ExpresssionStatement{
            expression: Box::new(InbuiltCall {
                            c_type: op.unwrap(),
                            arguments: el,
                        }.into_any_expression_enum())
        }.into_any_statement_enum())
    }

    fn if_statement(&mut self) -> PossibleStatement {
        self.consume(TokenType::OpenParen, "Expected a '('")?;
        let e = self.expression()?;
        self.consume(TokenType::CloseParen, "Expected a ')'")?;
        let block = Box::new(self.statement()?);
        let mut else_block = None;
        if self.match_token(TokenType::Else) {
            else_block = Some(Box::new(self.statement()?));
        }
        Ok(IfStatement {
            expression: Box::new(e),
            block,
            else_block,
        }.into_any_statement_enum())
    }

    fn block(&mut self) -> Result<Block, CompilerError> {
        let mut statements = vec![];
        while self.inverse_match_token(TokenType::CloseCurly)? {
            statements.push(Box::new(self.statement()?));
        }
        Ok(Block { statements })
    }

    fn while_statement(&mut self) -> PossibleStatement {
        self.consume(TokenType::OpenParen, "Expected a '('")?;
        let e = self.expression()?;
        self.consume(TokenType::CloseParen, "Expected a ')'")?;
        Ok(WhileStatement {
            expression: Box::new(e),
            statement: Box::new(self.statement()?),
        }.into_any_statement_enum())
    }

    fn for_statement(&mut self) -> PossibleStatement {
        self.consume(TokenType::OpenParen, "Expected a '('")?;
        let mut inital: Option<Box<AnyStatementEnum>> = None;
        if self.inverse_match_token(TokenType::Semicolon)? {
            self.consume(TokenType::Var, "Expected a variable declaration")?;
            inital = Some(Box::new(self.var_declaration()?));
            self.consume(TokenType::Semicolon, "Expected a ';'")?;
        }
        let mut check = None;
        if self.inverse_match_token(TokenType::Semicolon)? {
            check = Some(Box::new(self.expression()?));
            self.consume(TokenType::Semicolon, "Expected a ';'")?;
        }
        let mut change = None;
        if self.inverse_match_token(TokenType::CloseParen)? {
            change = Some(self.expression_list()?);
            self.consume(TokenType::CloseParen, "Expected a ')'")?;
        }
        let statement = Box::new(self.statement()?);

        Ok(ForStatement {
            inital,
            check,
            change,
            statement,
        }.into_any_statement_enum())
    }

    fn function_declaration(&mut self) -> PossibleStatement {
        self.fun_scope += 1;
        self.consume(TokenType::Identifer, "Expected a function name")?;
        let name = self.previous_literal();
        self.consume(TokenType::OpenParen, "Expected a '('")?;
        let mut pl: Vec<Box<AnyStatementEnum>> = vec![];
        if self.inverse_match_token(TokenType::CloseParen)? {
            loop {
                self.consume(TokenType::Var, "Expected a parameter")?;
                let p = self.var_declaration()?;
                pl.push(Box::new(p));
                if self.match_token(TokenType::CloseParen) {
                    break;
                }
                if self.is_eot() {
                    self.error("Unexpected End of File")?;
                }
                self.consume(TokenType::Comma, "Expected a ','")?;
            }
        }
        self.consume(TokenType::OpenCurly, "Expected a '{'")?;
        let block = self.block()?;
        Ok(FunctionDeclaration {
            name,
            parameters_list: pl,
            body: block.into_any_statement_enum().boxed(),
        }.into_any_statement_enum())
    }

    fn return_statement(&mut self) -> PossibleStatement {
        if self.fun_scope == 0 {
            self.error("Can't write Return Statement in Global Scope")?;
        }
        let expression = Box::new(self.expression()?);
        Ok(ReturnStatement { expression }.into_any_statement_enum())
    }
    fn statement(&mut self) -> PossibleStatement {
        let v:Box<AnyStatementEnum> = if self.match_token(TokenType::Var) {
            let v = self.var_declaration()?;
            self.consume(
                TokenType::Semicolon,
                "Expected a semicolon after Variable Declaration",
            )?;
            Box::new(v)
        } else if self.match_token(TokenType::Print) {
            let v = self.print_statement()?;
            self.consume(
                TokenType::Semicolon,
                "Expected a semicolon after Print Declaration",
            )?;
            Box::new(v)
        } else if self.match_token(TokenType::If) {
            let v = self.if_statement()?;
            Box::new(v)
        } else if self.match_token(TokenType::OpenCurly) {
            let v = self.block()?;
            Box::new(v.into_any_statement_enum())
        } else if self.match_token(TokenType::While) {
            let v = self.while_statement()?;
            Box::new(v)
        } else if self.match_token(TokenType::For) {
            let v = self.for_statement()?;
            Box::new(v)
        } else if self.match_token(TokenType::Return) {
            let v = self.return_statement()?;
            self.consume(
                TokenType::Semicolon,
                "Expected a semicolon after Return Statement",
            )?;
            Box::new(v)
        } else if self.match_token(TokenType::Function) {
            let v = self.function_declaration()?;
            Box::new(v)
        } else {
            let v = self.expression()?;
            self.consume(
                TokenType::Semicolon,
                "Expected a semicolon after an expression",
            )?;
            Box::new(ExpresssionStatement { expression: Box::new(v) }.into_any_statement_enum())
        };
        Ok(*v)
    }

    fn synchronize(&mut self) {
        self.advance();
        while !self.is_eot() {
            if self.previous_token_type() == TokenType::Semicolon {
                return;
            }
            if  self.match_token(TokenType::Function)
            || self.match_token(TokenType::Var)
            || self.match_token(TokenType::If)
            || self.match_token(TokenType::While)
            || self.match_token(TokenType::For)
            || self.match_token(TokenType::Return)
            {
                return;
            }
            self.advance();
        }
    }
    
    fn reset_errors(&mut self) {
        self.errors.clear();
        self.status = ParserStatus::Ok;
    }

    fn set_error(&mut self, x: CompilerError) {
        self.errors.push(x);
        self.status = ParserStatus::Failure;
    }

    pub fn parse_statements(&mut self) -> Vec<AnyStatementEnum> {
        self.reset_errors();
        let mut statements = vec![];
        while !self.tokens.is_empty() && self.tokens.len() > self.curr_token {
            let statement = self.statement();
            match statement {
                Ok(x) => statements.push(x),
                Err(x) => {
                    self.set_error(x);
                    self.synchronize();
                }
            }
        }
        statements
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use crate::tokenizer::Tokenizer;

    use super::*;
    fn check_for(stmt: AnyStatementEnum, s: &str) {
        let contents = s.chars().collect();
        let mut t = Tokenizer::create(&contents);
        let mut tokens = vec![];
        let map: HashMap<_, _> = HashMap::from([
            ("if", TokenType::If),
            ("else", TokenType::Else),
            ("var", TokenType::Var),
            ("return", TokenType::Return),
            ("func", TokenType::Function),
            ("while", TokenType::While),
            ("for", TokenType::For),
            ("print", TokenType::Print),
            ("true", TokenType::True),
            ("false", TokenType::False),
            ("none", TokenType::None),
            ("and", TokenType::And),
            ("or", TokenType::Or),
        ]);
        while let Some(x) = t.next_token(&map) {
            tokens.push(x);
        }
        println!("tokens: {:?}", tokens);

        let mut parser = Parser::create(tokens);
        let statements = parser.parse_statements();
        assert_eq!(statements.len(), 1);
        assert_eq!(format!("{:?}",stmt), format!("{:?}", statements[0]));
    }
    #[test]
    fn test_blocks()  {
        let s = "{}";
        let o = Block {
            statements: vec![]
        }.into_any_statement_enum();
        check_for(o, s);
    }

    #[test]
    fn test_binary_exprs() {
        let operator_n_types = [
        ("=", BinaryOpType::Assign),
        ("+=", BinaryOpType::PlusEqual),
        ("-=", BinaryOpType::MinusEqual),
        ("*=", BinaryOpType::MulEqual),
        ("/=", BinaryOpType::DivEqual),
        ("+", BinaryOpType::Add),
        ("-", BinaryOpType::Sub),
        ("/", BinaryOpType::Div),
        ("*", BinaryOpType::Mul),
        ("or", BinaryOpType::Or),
        ("and", BinaryOpType::And),
        (">", BinaryOpType::Greater),
        ("<", BinaryOpType::Less),
        (">=", BinaryOpType::GreatEqual),
        ("<=", BinaryOpType::LessEqual),
        ("==", BinaryOpType::EqualEqual),
        ("!=", BinaryOpType::NotEqual),
        ];
        for (x, y) in operator_n_types {
            let bo = BinaryOp {
                left: Box::new(Identifier {
                                    name: "a".to_string()
                                }.into_any_expression_enum()),
                right: Box::new(Identifier {
                                    name: "b".to_string()
                                }.into_any_expression_enum()),
                op_type: y
            };

            let stmt = wrap_expression(bo.into_any_expression_enum());

            let st = format!("a {} b;" , x);
            check_for(stmt, &st);
        }
        let operator_n_types = [
        ("a[b];", BinaryOpType::Index),
        ("a.b;", BinaryOpType::Dot),
        ];
        for (x, y) in operator_n_types {
            let bo = BinaryOp {
                left: Box::new(Identifier {
                                    name: "a".to_string()
                                }.into_any_expression_enum()),
                right: Box::new(Identifier {
                                    name: "b".to_string()
                                }.into_any_expression_enum()),
                op_type: y
            };

            let stmt = wrap_expression(bo.into_any_expression_enum());

            check_for(stmt, x);
        }

    }

    fn wrap_expression(expr: AnyExpressionEnum) -> AnyStatementEnum {
        ExpresssionStatement {
            expression: Box::new(expr)
        }.into_any_statement_enum()
    }

    #[test]
    fn test_unary_exprs() {
        let operator_n_types = [
        ("!", UnaryOpType::Not),
        ("-", UnaryOpType::Minus),
        ];
        for (x, y) in operator_n_types {
            let uo = UnaryOp {
                operand: Box::new(Identifier {
                                    name: "a".to_string()
                                }.into_any_expression_enum()),
                op_type: y
            };

            let stmt = wrap_expression(uo.into_any_expression_enum());

            let st = format!("{}a;" , x);
            check_for(stmt, &st);
        }
    }

    #[test]
    fn test_identifer_exprs() {
        let s = "variable_name;";
        let ident = Identifier {
            name: "variable_name".to_string()
        };
        let stmt = wrap_expression(ident.into_any_expression_enum());
        check_for(stmt, s);
    }


    #[test]
    fn test_calls_exprs() {
        let s = "test_call();";
        let ident = Call {
            left: "test_call".to_string(),
            arguments: ExpressionList {
                expressions: vec![]
            }
        };
        let stmt = wrap_expression(ident.into_any_expression_enum());
        check_for(stmt, s);
    }

    #[test]
    fn test_literal_exprs() {
        let s_to_stmt: Vec<(_, AnyExpressionEnum)> = vec![
        ("\"hello world\";", StringLiteral { val: "hello world".to_string() }.into_any_expression_enum()),
        ("1.0;", NumberLiteral { val: "1.0".to_string() }.into_any_expression_enum()),
        ("[1, 2];", ArrayLiteral {
                expressions: ExpressionList {
                    expressions: vec![
                    NumberLiteral{val:"1".to_string(),}.into_any_expression_enum().boxed(),
                    NumberLiteral{val:"2".to_string(),}.into_any_expression_enum().boxed()
                    ]
                }
            }.into_any_expression_enum()),
        ("false;", False {}.into_any_expression_enum()),
        ("true;", True {}.into_any_expression_enum()),
        ("none;", NoneVal {}.into_any_expression_enum()),
        ];

        for (x, y) in s_to_stmt {
            let y2 = wrap_expression(y);
            check_for(y2, x);
        }
    }

    #[test]
    fn test_func_decls() {
        //TODO
    }
    
    #[test]
    fn test_if_stmts() {
        //TODO
    }
    
    #[test]
    fn test_return_stmts() {
        //TODO
    }
    
    #[test]
    fn test_var_decls() {
        //TODO
    }
    
    #[test]
    fn test_while_stmts() {
        //TODO
    }
}


