use logos::{Lexer, Logos};

use crate::{errors::RuntimeError, token::Token};

/// A custom type that is emitted by our lexer so that it works with our parser
///
/// *See:* [How to create custom lexer for lalrpop](https://lalrpop.github.io/lalrpop/lexer_tutorial/002_writing_custom_lexer.html)
pub type Spanned<Tok, Loc, Error> = Result<(Loc, Tok, Loc), Error>;

/// Custom lexer for lalrpop that wraps around Logos lexer
/// This is done so that it can be integrated with our parser
///
/// *See:* [How to create custom lexer for lalrpop](https://lalrpop.github.io/lalrpop/lexer_tutorial/002_writing_custom_lexer.html)
pub struct LalrpopLexer<'t> {
  /// internal logos tokenizer
  tokenizer: Lexer<'t, Token>,
}

impl<'t> LalrpopLexer<'t> {
  /// Creates a new LalrpopLexer instance
  ///
  /// # Argument
  ///
  /// - *text* - String that holds the text that needs to be tokenized
  ///
  /// # Examples
  ///
  /// ```
  /// use rustigo::lexer::LalrpopLexer;
  /// let lexer = LalrpopLexer::new("(2+3)");
  /// ```
  pub fn new(text: &'t str) -> Self {
    let tokenizer = Token::lexer(text);
    Self { tokenizer }
  }
}

impl<'input> Iterator for LalrpopLexer<'input> {
  type Item = Spanned<Token, usize, RuntimeError>;

  fn next(&mut self) -> Option<Self::Item> {
    if let Some(token) = self.tokenizer.next() {
      match token {
        Token::TokenError => return Some(Err(RuntimeError::Why(String::from("Unexpected token")))),
        t => {
          let span = self.tokenizer.span();

          return Some(Ok((span.start, t, span.end)));
        }
      }
    }

    None
  }
}

/// Function to tokenize string into tokens
///
/// # Arguments
///
/// * `text` - String that holds text that needs to be tokenised
///
/// # Examples
///
/// ```
/// use rustigo::lexer::tokenize;
/// let tokens = tokenize("(1 + 3)");
/// ```
pub fn tokenize(text: &str) -> Result<Vec<Token>, RuntimeError> {
  // A vector holding Token that will be returned later
  let mut tokens: Vec<Token> = vec![];

  // Create a lexer instance that produces tokens
  let mut lex = Token::lexer(text);

  // iterate over all the tokens
  while let Some(token) = lex.next() {
    match token {
      // return `RuntimeError` if token returns an error
      Token::TokenError => return Err(RuntimeError::Why(String::from("Unexpected Token"))),

      // otherwise push the token in `tokens`
      t => tokens.push(t),
    }
  }

  // return the vector of tokens
  Ok(tokens)
}
