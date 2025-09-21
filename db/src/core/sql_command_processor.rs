use crate::compiler::code_generator::Statement;
use crate::compiler::compiler::{CompileError, CompileResult, Compiler};

pub enum CommandAction {
    Continue,
    Execute(Statement),
}

pub struct SqlCommandProcessor {
    compiler: Compiler,
}

impl SqlCommandProcessor {
    pub fn new() -> Self {
        Self {
            compiler: Compiler::new(),
        }
    }

    pub fn process_input(&self, input: &str) -> CommandAction {
        match self.compiler.compile(input) {
            Ok(CompileResult::Statement(statement)) => CommandAction::Execute(statement),
            Err(CompileError::EmptyInput) => CommandAction::Continue,
            Err(CompileError::MetaCommandError) => {
                println!("Unrecognized command '{}'.\n", input.trim());
                CommandAction::Continue
            }
            Err(CompileError::PrepareError) => {
                println!("Syntax error. Could not parse statement.\n");
                CommandAction::Continue
            }
        }
    }
}
