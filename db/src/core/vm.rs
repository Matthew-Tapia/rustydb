use crate::compiler::code_generator::{Statement, StatementType};

pub fn execute_statement(statement: &Statement) {
    match statement.type_ {
        StatementType::StatementInsert => {
            println!("This is where we would do an insert.\n");
        }
        StatementType::StatementSelect => {
            println!("This is where we would do a select.\n");
        }
    }
}
