use ::std::error::Error;

use inkwell::builder::Builder;
use inkwell::context::Context;
use inkwell::execution_engine::ExecutionEngine;
use inkwell::module::Module;
use inkwell::values::FunctionValue;
use inkwell::OptimizationLevel;

use crate::parser::statements::Statement;

mod mystd;

pub struct CodeGen<'ctx> {
    pub context: &'ctx Context,
    pub module: Module<'ctx>,
    pub builder: Builder<'ctx>,
    pub execution_engine: ExecutionEngine<'ctx>,
    pub main: FunctionValue<'ctx>,
    pub scoped_variables: Vec<(u16, String, inkwell::values::PointerValue<'ctx>)>,
    pub fun_stack: Vec<FunctionValue<'ctx>>,
    pub curr_scope: u16,
}

impl<'ctx> CodeGen<'ctx> {
    pub fn get_context() -> Context {
        Context::create()
    }
    pub fn create(context: &'ctx Context) -> Result<Self, Box<dyn Error>> {
        let module = context.create_module("main");
        let builder = context.create_builder();
        let execution_engine = module.create_jit_execution_engine(OptimizationLevel::Aggressive)?;
        let fnt = context.void_type().fn_type(&[], false);
        let main = module.add_function("main", fnt, None);
        let bb = context.append_basic_block(main, "entry");
        let fun_stack = vec![main];
        builder.position_at_end(bb);
        Ok(CodeGen {
            context,
            module,
            builder,
            execution_engine,
            main,
            fun_stack,
            scoped_variables: vec![],
            curr_scope: 0,
        })
    }
    pub fn initialize(&self) {
        let void_type = self.context.void_type();
        let fnt = void_type.fn_type(&[], false);
        let extf = self.module.add_function("print_time", fnt, None);
        self.execution_engine
            .add_global_mapping(&extf, mystd::print_time as usize);
        let ft = self.context.f64_type();
        let fnt2 = void_type.fn_type(&[ft.into()], false);
        let extf2 = self.module.add_function("print", fnt2, None);
        self.execution_engine
            .add_global_mapping(&extf2, mystd::print as usize);
    }

    pub fn get_curr_func(&self) -> FunctionValue {
        *self.fun_stack.last().unwrap()
    }

    pub fn get_variable(&self, name: &str) -> Option<inkwell::values::PointerValue> {
        for (_, x, y) in self.scoped_variables.iter().rev() {
            if x == name {
                return Some(*y);
            }
        }
        None
    }

    pub fn increase_scope(&mut self) {
        self.curr_scope += 1;
    }

    pub fn allocate_variable(&mut self, name: &str) {
        let curr_block = self.builder.get_insert_block().unwrap();
        let entry_block = self
            .builder
            .get_insert_block()
            .unwrap()
            .get_parent()
            .unwrap()
            .get_first_basic_block()
            .unwrap();
        let start = &entry_block.get_first_instruction();
        match start {
            Some(ip) => {
                self.builder.position_at(entry_block, ip);
            }
            None => {
                self.builder.position_at_end(entry_block);
            }
        }
        let ptr = self
            .builder
            .build_alloca(self.context.f64_type(), name)
            .unwrap();
        // TODO setting by default to None
        let _ = self
            .builder
            .build_store(ptr, self.context.f64_type().const_zero());
        self.scoped_variables
            .push((self.curr_scope, name.to_string(), ptr));
        self.builder.position_at_end(curr_block);
    }

    pub fn decrease_scope(&mut self) {
        self.curr_scope -= 1;
        let curr_i = self.curr_scope;
        let mut i = self.scoped_variables.len();
        for (x, _, _) in self.scoped_variables.iter().rev() {
            if *x <= curr_i {
                break;
            }
            i -= 1;
        }
        while self.scoped_variables.len() > i {
            let v = self.scoped_variables.len();
            self.scoped_variables.remove(v - 1);
        }
    }

    pub fn codegen(&mut self, stmt: &dyn Statement) {
        stmt.generate_code(self);
    }
}
