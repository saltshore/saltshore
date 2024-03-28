use super::commands::*;
use crate::command::prelude::CommandError;
use crate::game::prelude::GameState;

/// Actual commands that can be executed.
#[derive(Clone, Debug, PartialEq)]
pub enum Command {
  /// Succeed silently.
  Succeed(SucceedCommand),
  /// Throw a CommandError.
  ThrowError(ThrowErrorCommand),
  /// Quit command.
  Quit(QuitCommand),
}

impl Command {
  /// Execute the command.
  pub fn execute(&self, game_state: &mut GameState) -> Result<(), CommandError> {
    match self {
      Command::Succeed(command) => command.execute(game_state)?,
      Command::ThrowError(command) => command.execute(game_state)?,
      Command::Quit(command) => command.execute(game_state)?,
    }
    Ok(())
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use crate::game::prelude::GameState;
  use crate::test_utils;

  #[test]
  fn test_execute() -> Result<(), CommandError> {
    test_utils::init();
    let mut game_state = GameState::default();
    let command = Command::Quit(QuitCommand);
    assert_eq!(game_state.quit_flag(), false);
    command.execute(&mut game_state)?;
    assert_eq!(game_state.quit_flag(), true);
    Ok(())
  }

  #[test]
  fn test_execute_succeed() -> Result<(), CommandError> {
    test_utils::init();
    let mut game_state = GameState::default();
    let command = Command::Succeed(SucceedCommand);
    command.execute(&mut game_state)?;
    Ok(())
  }

  #[test]
  fn test_execute_throw_error() -> Result<(), CommandError> {
    test_utils::init();
    let mut game_state = GameState::default();
    let command = Command::ThrowError(ThrowErrorCommand {
      error: CommandError::InCharacter("Test error.".to_string()),
    });
    let result = command.execute(&mut game_state);
    assert_eq!(result.is_err(), true);
    assert_eq!(result.unwrap_err().to_string(), "Test error.");
    Ok(())
  }
}
