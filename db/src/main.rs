mod core {
    pub mod interface;
    pub mod sql_command_processor;
    pub mod vm;
}

mod compiler {
    pub mod code_generator;
    pub mod tokenizer;
}

use crate::core::interface::get_user_input;
use crate::core::sql_command_processor::{CommandAction, process_input};
use crate::core::vm::execute_statement;

fn main() {
    loop {
        let input = get_user_input();

        match process_input(&input) {
            CommandAction::Execute(stmt) => {
                execute_statement(&stmt);
            }
            CommandAction::Continue => {
                continue;
            }
        }
    }
}
