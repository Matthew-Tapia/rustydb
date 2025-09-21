mod backend;
mod compiler;
mod core;

use crate::core::interface::get_user_input;
use crate::core::sql_command_processor::{CommandAction, SqlCommandProcessor};
use crate::core::vm::VirtualMachine;

fn main() {
    let vm = VirtualMachine::new();
    let command_processor = SqlCommandProcessor::new();

    loop {
        let input = get_user_input();

        match command_processor.process_input(&input) {
            CommandAction::Execute(stmt) => {
                vm.execute_statement(&stmt);
            }
            CommandAction::Continue => {
                continue;
            }
        }
    }
}
