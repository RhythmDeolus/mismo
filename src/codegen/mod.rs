use ::std::error::Error;
use std::sync::Mutex;

use inkwell::basic_block::BasicBlock;
use inkwell::builder::Builder;
use inkwell::context::Context;
use inkwell::execution_engine::ExecutionEngine;
use inkwell::module::Module;
use inkwell::values::{FunctionValue, GlobalValue, PointerValue};
use inkwell::OptimizationLevel;

use crate::parser::statements::blocks::Block;
use crate::parser::statements::func_decls::FunctionDeclaration;
use crate::parser::statements::AnyStatementEnum;

pub mod mystd;

pub struct CodeGen<'ctx> {
    pub context: &'ctx Context,
    pub module: Module<'ctx>,
    pub builder: Builder<'ctx>,
    pub execution_engine: ExecutionEngine<'ctx>,
    pub main: FunctionValue<'ctx>,
    pub scoped_variables: Mutex<Vec<(u16, String, inkwell::values::PointerValue<'ctx>)>>,
    pub return_points: Mutex<Vec<BasicBlock<'ctx>>>,
    pub fun_stack: Mutex<Vec<FunctionValue<'ctx>>>,
    pub curr_scope: Mutex<u16>,
    pub show_info: bool,
}

pub enum VariableReference<'a> {
    Local(PointerValue<'a>),
    Global(GlobalValue<'a>),
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
        let fun_stack = Mutex::new(vec![main]);
        builder.position_at_end(bb);
        Ok(CodeGen {
            context,
            module,
            builder,
            execution_engine,
            main,
            fun_stack,
            return_points: Mutex::new(vec![]),
            scoped_variables: Mutex::new(vec![]),
            curr_scope: Mutex::new(0),
            show_info: false,
        })
    }

    pub fn print_module(&self) {
        if self.show_info {
            println!("Module: {}", self.module.to_string());
        }
    }
    pub fn initialize(&self) {
        let void_type = self.context.void_type();
        let fnt = void_type.fn_type(&[], false);
        let extf = self.module.add_function("print_time", fnt, None);
        self.execution_engine
            .add_global_mapping(&extf, mystd::print_time as usize);
        let ft = self.context.f64_type();
        let fnt2 = ft.fn_type(&[], false);
        let extf3 = self.module.add_function("get_time", fnt2, None);
        self.execution_engine
            .add_global_mapping(&extf3, mystd::get_time as usize);
        let fnt2 = void_type.fn_type(&[ft.into()], false);
        let extf2 = self.module.add_function("print", fnt2, None);
        self.execution_engine
            .add_global_mapping(&extf2, mystd::print as usize);
    }

    pub fn get_curr_func(&self) -> FunctionValue {
        *self.fun_stack.lock().unwrap().last().unwrap()
    }
    pub fn push_func_stack(&self, func: FunctionValue<'ctx>) {
        self.fun_stack.lock().unwrap().push(func);
    }

    pub fn pop_func_stack(&self) -> Option<FunctionValue<'ctx>> {
        self.fun_stack.lock().unwrap().pop()
    }

    pub fn get_variable(&self, name: &str) -> Option<VariableReference> {
        for (_, x, y) in self.scoped_variables.lock().unwrap().iter().rev() {
            if x == name {
                return Some(VariableReference::Local(*y));
            }
        }
        Some(VariableReference::Global(self.module.get_global(name)?))
    }

    pub fn increase_scope(&self) {
        let mut curr_scope = self.curr_scope.lock().unwrap();
        *curr_scope += 1;
    }

    pub fn allocate_variable(&self, name: &str) {
        let curr_scope = self.curr_scope.lock().unwrap();
        if *curr_scope == 0 {
            let gval = self.module.add_global(self.context.f64_type(), None, name);
            gval.set_initializer(&self.context.f64_type().const_zero());
            return;
        }
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
            .lock()
            .unwrap()
            .push((*curr_scope, name.to_string(), ptr));
        self.builder.position_at_end(curr_block);
    }

    pub fn hoist_statements_boxed(&self, stmts: &Vec<Box<AnyStatementEnum>>) {
        for stmt in stmts {
            if let AnyStatementEnum::FunctionDeclaration(x) = stmt.as_ref() {
                self.define_func(x);
            }
        }
    }

    pub fn hoist_statements(&self, stmts: &Vec<AnyStatementEnum>) {
        for stmt in stmts {
            if let AnyStatementEnum::FunctionDeclaration(x) = stmt {
                self.define_func(x);
            }
        }
    }

    pub fn define_func(&self, x: &FunctionDeclaration) {
        let ft = self.context.f64_type();
        let mut params = vec![];
        for _ in &x.parameters_list {
            params.push(ft.into());
        }
        let fnt = ft.fn_type(&params, false);
        let _ = self.module.add_function(&x.name, fnt, None);
    }

    pub fn decrease_scope(&self) {
        let mut curr_scope = self.curr_scope.lock().unwrap();
        *curr_scope -= 1;
        let curr_i = *curr_scope;
        let mut scoped_variables = self.scoped_variables.lock().unwrap();
        let mut i = scoped_variables.len();
        for (x, _, _) in scoped_variables.iter().rev() {
            if *x <= curr_i {
                break;
            }
            i -= 1;
        }
        while scoped_variables.len() > i {
            let v = scoped_variables.len();
            scoped_variables.remove(v - 1);
        }
    }

    pub fn codegen(&self, stmt: &AnyStatementEnum) {
        stmt.generate_code(self);
    }
}
