use crate::command::commands::*;
use crate::command::prelude::Command;
use crate::parser::error::ParserError;

/// Parser implementation.
#[derive(Clone, Copy, Debug, Default)]
pub struct Parser;

impl Parser {
  /// Create a new parser.
  pub fn new() -> Self {
    Self {}
  }

  /// Parse a string into a command.
  pub fn parse(&self, input: &str) -> Result<Command, ParserError> {
    let input = input.trim();
    let parts: Vec<&str> = input.split_whitespace().collect();
    match parts.as_slice() {
      //["go", direction] => Ok(Command::Go(direction.to_string())),
      //["look"] => Ok(Command::Look),
      ["quit"] => Ok(Command::Quit(QuitCommand)),
      _ => Err(ParserError::InvalidInput(input.to_string())),
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use crate::command::prelude::{Command, QuitCommand};

  #[test]
  fn test_parse() {
    let parser = Parser::new();
    let command = parser.parse("quit");
    assert!(command.is_ok());
    assert_eq!(command.unwrap(), Command::Quit(QuitCommand));
  }

  #[test]
  fn test_parse_invalid() {
    let parser = Parser::new();
    let command = parser.parse("invalid");
    assert!(command.is_err());
  }
}
