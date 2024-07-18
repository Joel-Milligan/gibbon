use std::io;

use gibbon::repl;

fn main() {
    println!("This is Gibbon!");
    println!("Begin typing commands.");

    let mut stdin = io::stdin();

    repl::start(&mut stdin);
}
