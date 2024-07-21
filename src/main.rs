use mismo::compiler::Compiler;
use repl::run_repl;
use std::env::set_var;
use std::fs;
use clap::Parser;

mod repl;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(help = "Path of the source file")]
    filename: Option<String>,

    #[arg(short, long, help = "Print debug info as well")]
    show_debug_info: bool,
}

fn main() {
    let _ = log4rs::init_file("logs/main/log4rs.yaml", Default::default());

    let args = Args::parse();
    set_var("RUST_BACKTRACE", "1");
    match args.filename {
        Some(filename) => {
            let contents = fs::read_to_string(filename);
            match contents {
                Err(e) => {
                    eprintln!("Error: {}", e)
                }
                Ok(contents) => {
                    let mut compiler = Compiler::create();
                    if args.show_debug_info {
                        compiler.set_show_info(true);
                    }
                    let context  = Compiler::get_context();
                    let codegen = Compiler::get_codegen(&context);

                    compiler.run(&codegen , contents.chars().collect());
                }
            }
        }
        None => {
            run_repl();
        }
    }

}
