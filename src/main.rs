#![crate_name = "rustigo"]

mod errors;
mod grammar;
pub mod lexer;
mod token;

use std::io::Write;

use crate::lexer::LalrpopLexer;

pub fn main() {
  println!("Welcome To Rustigo!!");

  // A simple repl loop
  // TODO: improve this later using libraries like clap
  loop {
    print!(">>> ");
    // flush the standard output, as some times it doesn't update
    // because we are using simple print and not println
    std::io::stdout().flush().expect("Couldn't flush to output");

    // mutable user input string that needs to be executed
    let mut input = String::new();

    // read the user input and store in input
    std::io::stdin()
      .read_line(&mut input)
      .ok()
      .expect("Failed to read line");

    let lexer = LalrpopLexer::new(&input);

    let output = grammar::TermParser::new().parse(lexer);

    match output {
      // if the tokenize is successful print them
      Ok(term) => println!("{:?}", term),

      // if there is a user error, report to the user
      // TODO: Improve error reporting
      Err(_) => eprintln!("Unexpected token"),
    }
  }
}
