use logos::Logos;

use crate::{errors::RuntimeError, token::Token};

pub fn tokenize(text: &str) -> Result<Vec<Token>, RuntimeError> {
  let mut tokens: Vec<Token> = vec![];
  let mut lex = Token::lexer(text);

  while let Some(token) = lex.next() {
    match token {
      Token::TokenError => return Err(RuntimeError::Why(String::from("Unexpected Token"))),
      t => tokens.push(t),
    }
  }

  Ok(tokens)
}
