use crate::command::prelude::CommandError;
use crate::game::prelude::GameState;

/// Quit command.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct QuitCommand;

impl QuitCommand {
  /// Execute the command.
  pub fn execute(&self, game_state: &mut GameState) -> Result<(), CommandError> {
    game_state.set_quit_flag(true);
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
    let command = QuitCommand;
    assert_eq!(game_state.quit_flag(), false);
    command.execute(&mut game_state)?;
    assert_eq!(game_state.quit_flag(), true);
    Ok(())
  }
}
