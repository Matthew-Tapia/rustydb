pub struct Tokenizer;

impl Tokenizer {
    pub fn new() -> Self {
        Tokenizer
    }

    pub fn tokenize(&self, input: &str) -> Vec<String> {
        input.split_whitespace().map(String::from).collect()
    }
}
