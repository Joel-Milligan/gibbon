use std::io::{self, Stdin, Stdout, Write};

use crate::{lexer::Lexer, token::Kind};

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

        let mut lexer = Lexer::new(input_buffer.clone());
        let mut token = lexer.next_token();

        while token.kind != Kind::Eof {
            writeln!(stdout, "{token:?}").expect("Could not write to stdout");
            token = lexer.next_token();
        }
    }
}
