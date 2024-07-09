use mismo::compiler;

use std::fs;

#[test]
fn test_cases() {
    let paths = fs::read_dir("./tests/cases").unwrap();
    let paths: Vec<String> = paths.map(|x| x.unwrap().path().to_str().unwrap().to_string()).collect();
    let (mut input_paths, mut output_paths): (Vec<_>, Vec<_>) = paths.into_iter().partition(|x| x.ends_with(".mi"));
    input_paths.sort();
    output_paths.sort();
    assert_eq!(input_paths.len(), output_paths.len());
    let n = input_paths.len();
    for i in 0..n {
        // if i > 3 {
        //     break;
        // }
        println!("Testing for: {} and {}", input_paths[i], output_paths[i]);
        let _ = fs::write("./tests/temp.out.txt", "");
        let input_text = fs::read_to_string(input_paths[i].clone()).unwrap();
        let output_text = fs::read_to_string(output_paths[i].clone()).unwrap();
        let comp = compiler::Compiler::create();
        comp.run(input_text.chars().collect());
        let content = fs::read_to_string("./tests/temp.out.txt").unwrap();
        println!("?: {} = {}", content, output_text);
        assert_eq!(content, output_text);
    }
}
