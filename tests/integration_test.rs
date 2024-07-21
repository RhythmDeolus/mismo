use std::{env, fs, panic::catch_unwind, process::Command};

#[test]
fn test_cases() {
    let _ = log4rs::init_file("/logs/test/log4rs.yaml", Default::default());
    env::set_var("RUST_BACKTRACE", "1");
    let paths = fs::read_dir("tests/cases").unwrap();
    let paths: Vec<String> = paths.map(|x| x.unwrap().path().to_str().unwrap().to_string()).collect();
    let (mut input_paths, mut output_paths): (Vec<_>, Vec<_>) = paths.into_iter().partition(|x| x.ends_with(".mi"));
    input_paths.sort();
    output_paths.sort();
    assert_eq!(input_paths.len(), output_paths.len());
    let mut successes = vec![];
    let n = input_paths.len();
    for i in 0..n {
        eprintln!("Testing for: {} and {}", input_paths[i], output_paths[i]);
        let result = catch_unwind(|| {
            let test_output = Command::new("cargo")
                .arg("run")
                .arg(input_paths[i].clone())
                .output()
                .unwrap();
            let test_output = String::from_utf8(test_output.stdout).unwrap();
            let output_text = fs::read_to_string(output_paths[i].clone()).unwrap();
            // let codegen_static: &CodeGen<'static> = unsafe { std::mem::transmute(&codegen) };
            println!("?: {} = {}", test_output, output_text);
            assert_eq!(test_output, output_text);
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
    assert_eq!(i, successes.len());
}
