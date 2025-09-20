enum MetaCommandResult {
    MetaCommandSuccess,
    MetaCommandUnrecognizedCommand,
}

enum PrepareResult {
    PrepareSuccess,
    PrepareUnrecognizedStatement,
}

enum StatementType {
    StatementInsert,
    StatementSelect,
}

struct Statement {
    type_: StatementType,
}

fn print_prompt() {
    print!("db > ");
    std::io::Write::flush(&mut std::io::stdout()).expect("Failed to flush stdout");
}

fn prepare_statement(input: &str, statement: &mut Statement) -> PrepareResult {
    if input.trim().to_lowercase().starts_with("insert") {
        statement.type_ = StatementType::StatementInsert;
        PrepareResult::PrepareSuccess
    } else if input.trim().to_lowercase() == "select" {
        statement.type_ = StatementType::StatementSelect;
        PrepareResult::PrepareSuccess
    } else {
        PrepareResult::PrepareUnrecognizedStatement
    }
}

fn execute_statement(statement: &Statement) {
    match statement.type_ {
        StatementType::StatementInsert => {
            println!("This is where we would do an insert.\n");
        }
        StatementType::StatementSelect => {
            println!("This is where we would do a select.\n");
        }
    }
}

fn do_meta_command(input: &str) -> MetaCommandResult {
    if input.trim() == ".exit" {
        std::process::exit(0);
    } else {
        MetaCommandResult::MetaCommandUnrecognizedCommand
    }
}

fn main() {
    let mut input = String::new();

    loop {
        print_prompt();
        std::io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        if input.starts_with('.') {
            match do_meta_command(&input) {
                MetaCommandResult::MetaCommandSuccess => {
                    input.clear();
                    continue;
                }
                MetaCommandResult::MetaCommandUnrecognizedCommand => {
                    println!("Unrecognized command '{}'.\n", input.trim());
                    input.clear();
                    continue;
                }
            }
        }

        let mut statement = Statement {
            type_: StatementType::StatementSelect,
        };

        match prepare_statement(&input, &mut statement) {
            PrepareResult::PrepareSuccess => {
                execute_statement(&statement);
                input.clear();
                continue;
            }
            PrepareResult::PrepareUnrecognizedStatement => {
                println!("Unrecognized keyword at start of '{}'.\n", input.trim());
                input.clear();
                continue;
            }
        }
    }
}
