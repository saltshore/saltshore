use crate::command::prelude::CommandError;
use crate::game::prelude::GameState;

/// Throw error command, which throws a CommandError.
#[derive(Clone, Debug, PartialEq)]
pub struct ThrowErrorCommand {
  /// The error.
  pub error: CommandError,
}

impl ThrowErrorCommand {
  /// Execute the command.
  pub fn execute(&self, _game_state: &mut GameState) -> Result<(), CommandError> {
    Err(self.error.clone())
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
    let command = ThrowErrorCommand {
      error: CommandError::InCharacter("Test error.".to_string()),
    };
    let result = command.execute(&mut game_state);
    assert_eq!(result.is_err(), true);
    assert_eq!(result.unwrap_err().to_string(), "Test error.");
    Ok(())
  }
}
