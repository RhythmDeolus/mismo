use std::io::{self, Write};

use mismo::codegen::CodeGen;

fn check_if_brackets_of_anytype_match(contents: Vec<char>) -> usize {
    let mut stack = vec![];
    for c in contents.iter() {
        if *c == '(' || *c == '[' || *c == '{' {
            stack.push(*c);
        } else if *c == ')' {
            if stack.is_empty() || stack.pop().unwrap() != '(' {
                return 0;
            }
        } else if *c == ']' {
            if stack.is_empty() || stack.pop().unwrap() != '[' {
                return 0;
            }
        } else if *c == '}' && ( stack.is_empty() || stack.pop().unwrap() != '{') {
                return 0;
        }
    }
    if stack.is_empty() {
        return 0;
    }
    stack.len()
}
pub fn run_repl() {
    let comp = mismo::compiler::Compiler::create();
    let context = mismo::compiler::Compiler::get_context();
    let codegen = mismo::compiler::Compiler::get_codegen(&context);
    loop {
        print!(">> "); 
        io::stdout().flush().unwrap();
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
        if input.trim() == "exit" {
            break;
        }
        let mut i = check_if_brackets_of_anytype_match(input.chars().collect());
        while i > 0 {
            let s = "\t";
            print!("... {}", s.repeat(i)); 
            io::stdout().flush().unwrap();
            let mut append = String::new();
            std::io::stdin().read_line(&mut append).unwrap();
            input.push_str(&append);
            i = check_if_brackets_of_anytype_match(input.chars().collect());
        }
        // let codegen_static: &CodeGen<'static> = unsafe { std::mem::transmute(&codegen) };
        comp.run(&codegen, input.chars().collect());
        io::stdout().flush().unwrap();
    }
}
