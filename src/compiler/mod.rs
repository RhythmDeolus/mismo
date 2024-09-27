use std::{collections::HashMap, time::SystemTime};

use inkwell::{
     context::Context, passes, targets::{CodeModel, Target, TargetMachine}
};

use crate::{
    codegen::CodeGen, parser::{statements::AnyStatementEnum, CompilerError, Parser, ParserStatus}, tokenizer::{token::TokenType, Tokenizer}
};

#[allow(dead_code)] //TODO
pub struct Compiler {
    pub keywords_to_tokentype: HashMap<&'static str, TokenType>,
    pub tokentype_to_keyword: HashMap<TokenType, &'static str>,
    pub show_info: bool,
    pub show_time: bool,
}

fn unescape_string(s: &str) -> String {
    s.replace("\\n", "\n")
     .replace("\\t", "\t")
     .replace("\\\"", "\"")
     .replace("\\\\", "\\")
}


impl<'ctx> Compiler {
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
            map2.insert(*v, k.to_owned());
        }



        Compiler {
            keywords_to_tokentype: map,
            tokentype_to_keyword: map2,
            show_info: false,
            show_time: false
        }
    }

    pub fn set_show_info(&mut self, val: bool) {
        self.show_info = val;
    }
    
    pub fn set_show_time(&mut self, val: bool) {
        self.show_time = val;
    }

    pub fn get_context() -> Context {
        CodeGen::get_context()
    }
    pub fn get_codegen(context: &Context) -> CodeGen {
        let codegen = CodeGen::create(context).unwrap();
        codegen.initialize();
        codegen
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

    fn print_statements(statements: &[AnyStatementEnum]) {
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

    fn get_time_now() -> u128 {
        SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap().as_millis()
    }

    pub fn run(&self, codegen: &CodeGen<'ctx>, contents:  Vec<char>) {
        let time_compile_1 = Compiler::get_time_now();
        let t1 = Compiler::get_time_now();
        // tokenization
        let mut tokenizer = Tokenizer::create(&contents);
        let mut tokens = vec![];

        while let Some(x) = tokenizer.next_token(&self.keywords_to_tokentype) {
            if self.show_info {
                println!("{:?}", x);
            }
            tokens.push(x);
        }
        let t2 = Compiler::get_time_now();
        if self.show_time {
            println!("Tokenization time: {}ms", t2 - t1);
        }

        let t1 = Compiler::get_time_now();
        // parsing
        let mut parser = Parser::create(tokens);

        let statements = parser.parse_statements();
        let t2 = Compiler::get_time_now();
        if self.show_time {
            println!("Parsing time: {}ms", t2 - t1);
        }

        //printing info
        if self.show_info {
            match parser.status {
                ParserStatus::Ok => {
                    Compiler::print_statements(&statements);
                }
                ParserStatus::Failure => {
                    Compiler::print_errors(&contents, parser.errors);
                }
            }
        }

        let t1 = Compiler::get_time_now();
        // de-sugaring
        let mut desugared_statements = vec![];

        for stmt in statements {
            desugared_statements.push(stmt.desugar())
        }
        let t2 = Compiler::get_time_now();
        if self.show_time {
            println!("Desugaring time: {}ms", t2 - t1);
        }

        //printing info
        if self.show_info {
            Compiler::print_statements(&desugared_statements);
        }


        let t1 = Compiler::get_time_now();
        // code generation
        codegen.hoist_statements(&desugared_statements);

        for stmt in &desugared_statements {
            codegen.codegen(stmt);
        }

        codegen.builder.position_at_end(codegen.main.get_last_basic_block().unwrap());

        let _ = codegen.builder.build_return(None);
        for x in codegen.main.get_basic_block_iter() {
            if  x.get_instructions().count() == 0 {
                codegen.builder.position_at_end(x);
                let _  = codegen.builder.build_unconditional_branch(x.get_next_basic_block().unwrap());
            }

            // for instr in x.get_instructions() {
            //     let res = instr.get_previous_instruction();
            //     if let Some(prev) = res {
            //         if instr.get_type() == prev.get_type() &&
            //             instr.is_terminator() {
            //             instr.remove_from_basic_block();
            //         }
            //     }
            // }
        }
        for func in codegen.module.get_functions() {
            let can_work = func.verify(true);
            if self.show_info && !can_work {
                println!("For: {}\n", func.get_name().to_str().unwrap());
                println!("{}", unescape_string(&func.to_string()));
                panic!("Can't Compile!");
            }
        }

        // optimization passes
        let tt = TargetMachine::get_default_triple();
        let t = Target::from_triple(&tt).unwrap();
        let tm = t.create_target_machine(
            &tt,
            TargetMachine::get_host_cpu_name().to_str().unwrap(),
            TargetMachine::get_host_cpu_features().to_str().unwrap(),
            inkwell::OptimizationLevel::None,
            inkwell::targets::RelocMode::Default,
            CodeModel::Default,
        ).unwrap();

        let result = codegen.module.run_passes("mem2reg,loop-unroll", &tm, passes::PassBuilderOptions::create());
        if self.show_info { 
            println!("{:?}", result);

            println!("{}", codegen.module.to_string());
            println!("{}", codegen.main);
        }
        let t2 = Compiler::get_time_now();
        if self.show_time {
            println!("Code generation time: {}ms", t2 - t1);
        }
        let time_compile_2 = Compiler::get_time_now();
        if self.show_time {
            println!("Compile time: {}ms", time_compile_2 - time_compile_1);
        }


        let t1 = Compiler::get_time_now();
        // jit compilation & execution
        unsafe {
            let _ = codegen.execution_engine.add_module(&codegen.module);
            codegen.execution_engine.run_function(codegen.main, &[]);
            let _ = codegen.execution_engine.remove_module(&codegen.module);
        };
        let t2 = Compiler::get_time_now();
        if self.show_time {
            println!("Execution time: {}ms", t2 - t1);
        }
        
        // resetting main function
        for x in codegen.main.get_basic_block_iter() {
            let _ = x.remove_from_function();
        }

        let bb = codegen.context.append_basic_block(codegen.main, "entry");
        codegen.builder.position_at_end(bb);

    }
}
