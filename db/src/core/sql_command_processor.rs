use crate::compiler::code_generator::{
    MetaCommandResult, PrepareResult, Statement, StatementType, do_meta_command, prepare_statement,
};

pub enum CommandAction {
    Continue,           // Nothing to execute (handled meta or printed an error)
    Execute(Statement), // Prepared statement ready to run
}

pub fn process_input(input: &str) -> CommandAction {
    if input.starts_with('.') {
        match do_meta_command(input) {
            MetaCommandResult::MetaCommandSuccess => CommandAction::Continue,
            MetaCommandResult::MetaCommandUnrecognizedCommand => {
                println!("Unrecognized command '{}'.\n", input.trim());
                CommandAction::Continue
            }
        }
    } else {
        let mut statement = Statement {
            type_: StatementType::StatementSelect,
        };
        match prepare_statement(input, &mut statement) {
            PrepareResult::PrepareSuccess => CommandAction::Execute(statement),
            PrepareResult::PrepareUnrecognizedStatement => {
                println!("Unrecognized keyword at start of '{}'.\n", input.trim());
                CommandAction::Continue
            }
            PrepareResult::PrepareSyntaxError => {
                println!("Syntax error. Could not parse statement.\n");
                CommandAction::Continue
            }
        }
    }
}
