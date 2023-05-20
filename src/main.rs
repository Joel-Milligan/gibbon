use std::io;

use gibbon::repl;

fn main() {
    println!("This is Gibbon!");
    println!("Begin typing commands.");

    let mut stdin = io::stdin();
    let mut stdout = io::stdout();

    repl::start(&mut stdin, &mut stdout);
}
