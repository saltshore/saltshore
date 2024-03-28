use crate::command::prelude::Command;
use std::collections::VecDeque;

/// The game state.
///
/// This struct defines the game state for Saltshore.
#[derive(Clone, Debug, Default)]
pub struct GameState {
  /// Whether the game is finished.
  pub quit_flag: bool,
  /// The input queue.
  pub input_queue: VecDeque<String>,
  /// The command queue.
  pub command_queue: VecDeque<Command>,
}

impl GameState {
  /// Create a new game state.
  pub fn new() -> Self {
    Self {
      quit_flag: false,
      input_queue: VecDeque::new(),
      command_queue: VecDeque::new(),
    }
  }

  /// Get the quit flag.
  pub fn quit_flag(&self) -> bool {
    self.quit_flag
  }

  /// Set the quit flag.
  pub fn set_quit_flag(&mut self, flag: bool) {
    self.quit_flag = flag;
  }

  /// Enqueue an input.
  pub fn enqueue_input(&mut self, input: String) {
    self.input_queue.push_back(input);
  }

  /// Dequeue an input.
  pub fn dequeue_input(&mut self) -> Option<String> {
    self.input_queue.pop_front()
  }

  /// Clear the input queue.
  pub fn clear_input_queue(&mut self) {
    self.input_queue.clear();
  }

  /// Enqueue a command.
  pub fn enqueue_command(&mut self, command: Command) {
    self.command_queue.push_back(command);
  }

  /// Dequeue a command.
  pub fn dequeue_command(&mut self) -> Option<Command> {
    self.command_queue.pop_front()
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use crate::test_utils;

  #[test]
  fn test_quit_flag() {
    test_utils::init();
    let mut game_state = GameState::default();
    assert_eq!(game_state.quit_flag(), false);
    game_state.set_quit_flag(true);
    assert_eq!(game_state.quit_flag(), true);
  }
}
