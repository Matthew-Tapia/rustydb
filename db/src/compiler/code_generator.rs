pub enum PrepareResult {
    PrepareSuccess,
    PrepareUnrecognizedStatement,
    PrepareSyntaxError,
}

pub enum MetaCommandResult {
    MetaCommandUnrecognizedCommand,
}

pub enum StatementType {
    StatementInsert,
    StatementSelect,
}

pub struct Statement {
    pub type_: StatementType,
}

pub struct CodeGenerator;

impl CodeGenerator {
    pub fn new() -> Self {
        Self {}
    }

    pub fn prepare_statement(&self, tokens: &[String], statement: &mut Statement) -> PrepareResult {
        if tokens.is_empty() {
            return PrepareResult::PrepareUnrecognizedStatement;
        }

        match tokens[0].to_lowercase().as_str() {
            "insert" => {
                if tokens.len() < 4 {
                    return PrepareResult::PrepareSyntaxError;
                }

                statement.type_ = StatementType::StatementInsert;
                PrepareResult::PrepareSuccess
            }
            "select" => {
                statement.type_ = StatementType::StatementSelect;
                PrepareResult::PrepareSuccess
            }
            _ => PrepareResult::PrepareUnrecognizedStatement,
        }
    }

    pub fn do_meta_command(&self, input: &str) -> MetaCommandResult {
        match input.trim() {
            ".exit" => {
                std::process::exit(0);
            }
            _ => MetaCommandResult::MetaCommandUnrecognizedCommand,
        }
    }
}
