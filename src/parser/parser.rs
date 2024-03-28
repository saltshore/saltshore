use crate::command::commands::*;
use crate::command::prelude::Command;
use crate::command::prelude::CommandError;
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
      ["debug:succeed"] => Ok(Command::Succeed(SucceedCommand)),
      ["debug:throw_error:ic"] => Ok(Command::ThrowError(ThrowErrorCommand {
        error: CommandError::InCharacter("Test error.".to_string()),
      })),
      ["debug:throw_error:oc"] => Ok(Command::ThrowError(ThrowErrorCommand {
        error: CommandError::OutOfCharacter("Test error.".to_string()),
      })),
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
  fn test_parse_succeed() {
    let parser = Parser::new();
    let command = parser.parse("debug:succeed");
    assert!(command.is_ok());
    assert_eq!(command.unwrap(), Command::Succeed(SucceedCommand));
  }

  #[test]
  fn test_parse_throw_error_ic() {
    let parser = Parser::new();
    let command = parser.parse("debug:throw_error:ic");
    assert!(command.is_ok());
    assert_eq!(
      command.unwrap(),
      Command::ThrowError(ThrowErrorCommand {
        error: CommandError::InCharacter("Test error.".to_string()),
      })
    );
  }

  #[test]
  fn test_parse_throw_error_oc() {
    let parser = Parser::new();
    let command = parser.parse("debug:throw_error:oc");
    assert!(command.is_ok());
    assert_eq!(
      command.unwrap(),
      Command::ThrowError(ThrowErrorCommand {
        error: CommandError::OutOfCharacter("Test error.".to_string()),
      })
    );
  }

  #[test]
  fn test_parse_invalid() {
    let parser = Parser::new();
    let command = parser.parse("invalid");
    assert!(command.is_err());
  }
}
