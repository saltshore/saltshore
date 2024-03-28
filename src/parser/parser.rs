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
