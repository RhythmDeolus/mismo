use mismo::compiler::Compiler;
use repl::run_repl;
use std::env::{args, set_var};
use std::fs;
mod repl;
fn main() {
    let mut arg = args();
    set_var("RUST_BACKTRACE", "1");
    if arg.len() == 1 {
        run_repl();
        return;
    }
    let file_name = arg.nth(1).expect("No argument provided");
    let contents = fs::read_to_string(file_name);
    match contents {
        Err(e) => {
            eprintln!("Error: {}", e)
        }
        Ok(contents) => {
            let compiler = Compiler::create();
            let context = Compiler::get_context();
            let mut codegen = Compiler::get_codegen(&context);
            compiler.run(contents.chars().collect(), &mut codegen);
        }
    }
}
// use inkwell::builder::{self, Builder};
// use inkwell::context::{self, Context};
// use inkwell::execution_engine::{ExecutionEngine, JitFunction};
// use inkwell::llvm_sys::core::LLVMStructType;
// use inkwell::module::Module;
// use inkwell::types::{self, StructType};
// use inkwell::{attributes, OptimizationLevel};
//
// use std::error::Error;
// use std::time::SystemTime;
//
// /// Convenience type alias for the `sum` function.
// ///
// /// Calling this is innately `unsafe` because there's no guarantee it doesn't
// /// do `unsafe` operations internally.
// type SumFunc = unsafe extern "C" fn(u64, u64, u64) -> u64;
//
// extern fn print_time() {
//     let t =  SystemTime::now().duration_since(SystemTime::UNIX_EPOCH);
//     match t {
//         Ok(x) => {
//             println!("{:?}", x);
//         }
//         Err(x) => {
//             println!("{}", x);
//         }
//     }
// }
//
// struct CodeGen<'ctx> {
//     context: &'ctx Context,
//     module: Module<'ctx>,
//     builder: Builder<'ctx>,
//     execution_engine: ExecutionEngine<'ctx>,
// }
//
// impl<'ctx> CodeGen<'ctx> {
//     fn jit_compile_sum(&self) -> Option<JitFunction<SumFunc>> {
//         let i64_type = self.context.i64_type();
//         let fn_type = i64_type.fn_type(&[i64_type.into(), i64_type.into(), i64_type.into()], false);
//         let function = self.module.add_function("sum", fn_type, None);
//         let basic_block = self.context.append_basic_block(function, "entry");
//
//         self.builder.position_at_end(basic_block);
//
//         let x = function.get_nth_param(0)?.into_int_value();
//         let y = function.get_nth_param(1)?.into_int_value();
//         let z = function.get_nth_param(2)?.into_int_value();
//
//         let sum = self.builder.build_int_add(x, y, "sum").unwrap();
//         let sum = self.builder.build_int_add(sum, z, "sum").unwrap();
//
//         self.builder.build_return(Some(&sum)).unwrap();
//
//         unsafe { self.execution_engine.get_function("sum").ok() }
//     }
//     fn print_time(&self) {
//         let void_type = self.context.void_type();
//         let fnt = void_type.fn_type(&[], false);
//         let f = self.module.add_function("main", fnt, None);
//
//         let bb = self.context.append_basic_block(f, "entry");
//
//         self.builder.position_at_end(bb);
//         let extf = self.module.add_function("print_time", fnt, None);
//
//         let _ = self.builder.build_call(extf, &[], "retv").unwrap();
//
//
//         let _ = self.builder.build_return(None);
//         self.execution_engine.add_global_mapping(&extf, print_time as usize);
//
//         unsafe {
//             self.execution_engine.run_function(f, &[]);
//         }
//     }
// }
//
// fn main() -> Result<(), Box<dyn Error>> {
//     let context = Context::create();
//     let module = context.create_module("sum");
//     let execution_engine = module.create_jit_execution_engine(OptimizationLevel::None)?;
//     let codegen = CodeGen {
//         context: &context,
//         module,
//         builder: context.create_builder(),
//         execution_engine,
//     };
//
//     codegen.print_time();
//
//     // let sum = codegen.jit_compile_sum().ok_or("Unable to JIT compile `sum`")?;
//     //
//     // let x = 1u64;
//     // let y = 2u64;
//     // let z = 3u64;
//     //
//     // unsafe {
//     //     println!("{} + {} + {} = {}", x, y, z, sum.call(x, y, z));
//     //     assert_eq!(sum.call(x, y, z), x + y + z);
//     // }
//     //
//     Ok(())
// }
