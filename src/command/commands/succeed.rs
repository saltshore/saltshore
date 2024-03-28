use crate::command::prelude::CommandError;
use crate::game::prelude::GameState;

/// Succeed command, which vacuously succeeds.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SucceedCommand;

impl SucceedCommand {
  /// Execute the command.
  pub fn execute(&self, _game_state: &mut GameState) -> Result<(), CommandError> {
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
    let command = SucceedCommand;
    command.execute(&mut game_state)?;
    Ok(())
  }
}
