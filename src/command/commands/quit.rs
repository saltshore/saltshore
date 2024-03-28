use crate::game::prelude::GameState;

/// Quit command.
#[derive(Clone, Copy, Debug)]
pub struct QuitCommand;

impl QuitCommand {
  /// Execute the command.
  pub fn execute(&self, game_state: &mut GameState) {
    game_state.set_quit_flag(true);
  }
}
