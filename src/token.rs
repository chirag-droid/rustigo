use logos::Logos;
use std::str::FromStr;

/// Stores all the info about the tokens and
/// has information on how to tokenise them
#[derive(Logos, Debug, Clone)]
pub enum Token {
  #[token("return")]
  KeywordReturn,

  #[token("while")]
  KeywordWhile,

  #[token("else")]
  KeywordElse,

  #[token("def")]
  KeywordDef,

  #[token("let")]
  KeywordLet,

  #[token("for")]
  KeywordFor,

  #[token("if")]
  KeywordIf,

  #[regex("[0-9]+", |l| i128::from_str(l.slice()), priority=2)]
  Number(i128),

  #[regex("[a-zA-Z0-9]+", |l| l.slice().to_string(), priority=1)]
  Identifier(String),

  #[token("&&")]
  And,

  #[token("||")]
  Or,

  #[token("<<")]
  LeftShift,

  #[token(">>")]
  RightShift,

  #[token("<=")]
  LessEqual,

  #[token("==")]
  IsEqual,

  #[token(">=")]
  GreaterEqual,

  #[token("!=")]
  NotEqual,

  #[token("+=")]
  PlusAssign,

  #[token("-=")]
  MinusAssign,

  #[token("*=")]
  StarAssign,

  #[token("/=")]
  SlashAssign,

  #[token("%=")]
  PercentAssign,

  #[token("+")]
  Plus,

  #[token("-")]
  Minus,

  #[token("*")]
  Star,

  #[token("/")]
  Slash,

  #[token("%")]
  Percent,

  #[token("<")]
  LessThan,

  #[token(">")]
  GreaterThan,

  #[token("!")]
  Not,

  #[token("&")]
  BitwiseAnd,

  #[token("|")]
  BitwiseOr,

  #[token("=")]
  SimpleAssign,

  #[token("(")]
  LeftParen,

  #[token(")")]
  RightParen,

  #[token("}")]
  LeftBrace,

  #[token("{")]
  RightBrace,

  #[token("[")]
  LeftBracket,

  #[token("]")]
  RightBracket,

  #[token(";")]
  SemiColon,

  #[token("'")]
  SemiQuote,

  #[token("\"")]
  Quote,

  #[token(":")]
  Colon,

  #[token("->")]
  Arrow,

  #[regex(r"[ \t\n\f]+", logos::skip)]
  #[error]
  TokenError,
}
