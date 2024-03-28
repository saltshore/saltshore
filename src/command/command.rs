use crate::game::prelude::GameState;

use super::commands::*;

/// Actual commands that can be executed.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Command {
  /// Quit command.
  Quit(QuitCommand),
}

impl Command {
  /// Execute the command.
  pub fn execute(&self, game_state: &mut GameState) {
    match self {
      Command::Quit(command) => command.execute(game_state),
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use crate::game::prelude::GameState;
  use crate::test_utils;

  #[test]
  fn test_execute() {
    test_utils::init();
    let mut game_state = GameState::default();
    let command = Command::Quit(QuitCommand);
    assert_eq!(game_state.quit_flag(), false);
    command.execute(&mut game_state);
    assert_eq!(game_state.quit_flag(), true);
  }
}
