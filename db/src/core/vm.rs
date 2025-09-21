use crate::compiler::code_generator::{Statement, StatementType};

pub struct VirtualMachine;

impl VirtualMachine {
    pub fn new() -> Self {
        Self {}
    }

    pub fn execute_statement(&self, statement: &Statement) {
        match statement.type_ {
            StatementType::StatementInsert => {
                println!("This is where we would do an insert.");
            }
            StatementType::StatementSelect => {
                println!("This is where we would do a select.");
            }
        }
    }
}
