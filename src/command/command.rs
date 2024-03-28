use crate::game::prelude::GameState;

use super::commands::*;

/// Actual commands that can be executed.
#[derive(Clone, Copy, Debug)]
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
