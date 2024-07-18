use std::io::{self, Stdin, Write};

use crate::lexer::Lexer;
use crate::parser::Parser;

static PROMPT: &str = ">> ";

pub fn start(stdin: &mut Stdin) {
    let mut input_buffer = String::new();

    loop {
        print!("{PROMPT}");
        io::stdout().flush().unwrap();

        input_buffer.clear();
        stdin
            .read_line(&mut input_buffer)
            .expect("Could not read from stdin");

        let line = input_buffer.clone();
        let lexer = Lexer::new(line);
        let mut parser = Parser::new(lexer);
        let program = parser.parse_program();

        if !parser.errors.is_empty() {
            print_parser_errors(parser.errors);
            continue;
        }

        println!("{program}");
    }
}

pub fn print_parser_errors(errors: Vec<String>) {
    for error in errors {
        println!("\t{error}");
    }
}
