use std::io::stdin;

pub fn console_input() -> String {
    let mut inp = String::new();
    let stdin = stdin();
    stdin.read_line(&mut inp).unwrap();
    inp
}