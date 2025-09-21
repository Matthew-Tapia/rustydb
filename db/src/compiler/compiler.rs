use crate::compiler::code_generator::{
    CodeGenerator, MetaCommandResult, PrepareResult, Statement, StatementType,
};
use crate::compiler::tokenizer::Tokenizer;

pub enum CompileError {
    EmptyInput,
    MetaCommandError,
    PrepareError,
}

pub enum CompileResult {
    Statement(Statement),
}

pub struct Compiler {
    tokenizer: Tokenizer,
    code_generator: CodeGenerator,
}

impl Compiler {
    pub fn new() -> Self {
        Self {
            tokenizer: Tokenizer::new(),
            code_generator: CodeGenerator::new(),
        }
    }

    pub fn compile(&self, input: &str) -> Result<CompileResult, CompileError> {
        if input.is_empty() {
            return Err(CompileError::EmptyInput);
        }

        if input.starts_with('.') {
            let result = self.code_generator.do_meta_command(input);
            match result {
                MetaCommandResult::MetaCommandUnrecognizedCommand => {
                    return Err(CompileError::MetaCommandError);
                }
            }
        }

        let tokens = self.tokenizer.tokenize(input);

        if tokens.is_empty() {
            return Err(CompileError::EmptyInput);
        }

        let mut statement = Statement {
            type_: StatementType::StatementSelect,
        };

        match self
            .code_generator
            .prepare_statement(&tokens, &mut statement)
        {
            PrepareResult::PrepareSuccess => Ok(CompileResult::Statement(statement)),
            _error => Err(CompileError::PrepareError),
        }
    }
}
