use crate::compiler::tokenizer::tokenize;

pub enum PrepareResult {
    PrepareSuccess,
    PrepareUnrecognizedStatement,
    PrepareSyntaxError,
}

pub enum StatementType {
    StatementInsert,
    StatementSelect,
}

pub struct Statement {
    pub type_: StatementType,
}

pub fn prepare_statement(input: &str, statement: &mut Statement) -> PrepareResult {
    let tokens: Vec<String> = tokenize(input);

    if tokens[0].to_lowercase() == "insert" {
        if tokens.len() < 4 {
            return PrepareResult::PrepareSyntaxError;
        }

        statement.type_ = StatementType::StatementInsert;

        PrepareResult::PrepareSuccess
    } else if input.trim().to_lowercase() == "select" {
        statement.type_ = StatementType::StatementSelect;
        return PrepareResult::PrepareSuccess;
    } else {
        return PrepareResult::PrepareUnrecognizedStatement;
    }
}

pub enum MetaCommandResult {
    MetaCommandSuccess,
    MetaCommandUnrecognizedCommand,
}

pub fn do_meta_command(input: &str) -> MetaCommandResult {
    if input.trim() == ".exit" {
        std::process::exit(0);
    } else {
        return MetaCommandResult::MetaCommandUnrecognizedCommand;
    }
}
