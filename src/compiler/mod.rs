use std::{borrow::Borrow, collections::HashMap};

use crate::{
    codegen::CodeGen, parser::{statements::Statement, CompilerError, Parser, ParserStatus}, tokenizer::{token::TokenType, Tokenizer}
};

pub struct Compiler {
    keywords_to_tokentype: HashMap<&'static str, TokenType>,
    tokentype_to_keyword: HashMap<TokenType, &'static str>,
}

impl Compiler {
    pub fn create() -> Self {
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
        let mut map2 = HashMap::new();
        for (k, v) in map.iter() {
            map2.insert(v.clone(), k.to_owned());
        }

        Compiler {
            keywords_to_tokentype: map,
            tokentype_to_keyword: map2,
        }
    }

    fn print_nth_line(contents: &[char], n: usize) {
        let mut i = 0;
        let mut matched_index = None;
        for (index, c) in contents.iter().enumerate() {
            if i == n - 1 {
                matched_index = Some(index);
                break;
            }
            if *c == '\n' {
                i += 1;
            }
        }

        if let Some(x) = matched_index {
            for val in contents.iter().skip(x) {
                if *val == '\n' {
                    break;
                }
                print!("{}", val);
            }
            println!()
        }
    }

    fn print_statements(statements: &Vec<Box<dyn Statement>>) {
        println!("\n========= STATEMENTS =========================\n");
        for (i, stmt) in statements.iter().enumerate() {
            println!("{}: {:#?}\n", i, stmt);
        }
    }

    fn print_errors(contents: &[char], errors: Vec<CompilerError>) {
        println!("\n========= ERRORS =========================\n");
        for err in errors.iter() {
            print!("Error at line no: {}\n--> ", err.line_no.unwrap());
            Compiler::print_nth_line(contents, err.line_no.unwrap());
            println!("{}\n", err);
        }
    }


    pub fn run(&self, contents: Vec<char>) {
        let mut tokenizer = Tokenizer::create(&contents);
        let mut tokens = vec![];

        while let Some(x) = tokenizer.next_token(&self.keywords_to_tokentype) {
            println!("{:?}", x);
            tokens.push(x);
        }

        let mut parser = Parser::create(tokens);

        let statements = parser.parse_statements();
        match parser.status {
            ParserStatus::Ok => {
                Compiler::print_statements(&statements);
            }
            ParserStatus::Failure => {
                Compiler::print_errors(&contents, parser.errors);
            }
        }
        let context = CodeGen::get_context();
        let mut codegen = CodeGen::create(&context).unwrap();
        codegen.initialize();
        for stmt in statements {
            codegen.codegen(stmt.borrow());
        }
        codegen.builder.position_at_end(codegen.main.get_last_basic_block().unwrap());
        let _ = codegen.builder.build_return(None);
        codegen.main.verify(true);

        println!("{}", codegen.module.to_string());

        unsafe {
            codegen.execution_engine.run_function_as_main(codegen.main, &[])
        };

    }
}
