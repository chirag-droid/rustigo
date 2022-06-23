mod errors;
mod lexer;
mod token;

use errors::RuntimeError;
use lexer::tokenize;
use std::io::Write;

fn main() {
  println!("Welcome To Rustigo!!");
  loop {
    print!(">>> ");
    std::io::stdout().flush().expect("Couldn't flush to output");

    let mut input = String::new();

    std::io::stdin()
      .read_line(&mut input)
      .ok()
      .expect("Failed to read line");

    match tokenize(&input) {
      Ok(tokens) => println!("{:?}", tokens),
      Err(RuntimeError::Why(why)) => eprintln!("Error: {}", why),
    }
  }
}
