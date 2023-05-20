use std::io::{self, Stdin, Stdout, Write};

use crate::lexer::Lexer;

static PROMPT: &'static str = ">> ";

pub fn start(stdin: &mut Stdin, stdout: &mut Stdout) {
    let mut input_buffer = String::new();

    loop {
        print!("{PROMPT}");
        io::stdout().flush().unwrap();

        input_buffer.clear();
        stdin
            .read_line(&mut input_buffer)
            .expect("Could not read from stdin");

        let lexer = Lexer::new(input_buffer.clone());

        for token in lexer {
            writeln!(stdout, "{token:?}").expect("Could not write to stdout");
        }
    }
}
