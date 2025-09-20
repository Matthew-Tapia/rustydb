pub fn print_prompt() {
    print!("db > ");
    std::io::Write::flush(&mut std::io::stdout()).expect("Failed to flush stdout");
}

pub fn get_user_input() -> String {
    print_prompt();

    let mut input = String::new();
    std::io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    return input;
}
