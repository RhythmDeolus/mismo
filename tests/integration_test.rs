use mismo::{codegen::CodeGen, compiler};

use std::{env, fs, panic::{catch_unwind, AssertUnwindSafe}};

#[test]
fn test_cases() {
    env::set_var("RUST_BACKTRACE", "1");
    let paths = fs::read_dir("./tests/cases").unwrap();
    let paths: Vec<String> = paths.map(|x| x.unwrap().path().to_str().unwrap().to_string()).collect();
    let (mut input_paths, mut output_paths): (Vec<_>, Vec<_>) = paths.into_iter().partition(|x| x.ends_with(".mi"));
    input_paths.sort();
    output_paths.sort();
    assert_eq!(input_paths.len(), output_paths.len());
    let mut successes = vec![];
    let n = input_paths.len();
    for i in 0..n {
        // if i > 5 {
        //     break;
        // }
        eprintln!("Testing for: {} and {}", input_paths[i], output_paths[i]);
        let result = catch_unwind(|| {
            let _ = fs::write("./tests/temp.out.txt", "");
            let input_text = fs::read_to_string(input_paths[i].clone()).unwrap();
            let output_text = fs::read_to_string(output_paths[i].clone()).unwrap();
            let comp = compiler::Compiler::create();
            let context = compiler::Compiler::get_context();
            let codegen = compiler::Compiler::get_codegen(&context);
            let codegen_static: &CodeGen<'static> = unsafe { std::mem::transmute(&codegen) };
            comp.run(codegen_static, input_text.chars().collect());
            let content = fs::read_to_string("./tests/temp.out.txt").unwrap();
            println!("?: {} = {}", content, output_text);
            assert_eq!(content, output_text);
        });
        successes.push((input_paths[i].clone(), result));
    }
    let mut i = 0;
    for (case , result) in successes.iter_mut() {
        match result {
            Ok(_) => {
                println!("{}: success", case);
                i += 1;
            }
            Err(x) => {
                println!("{}: error", case);
                println!("Error: {}", x.downcast_ref::<String>().unwrap());
            }
        }
    }
    println!("Total: {}/{}", i, successes.len());
}
