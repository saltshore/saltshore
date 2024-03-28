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

  /// Enqueue multiple inputs.
  pub fn enqueue_inputs(&mut self, inputs: Vec<String>) {
    inputs
      .iter()
      .for_each(|input| self.input_queue.push_back(input.clone()));
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

  /// Clear the command queue.
  pub fn clear_command_queue(&mut self) {
    self.command_queue.clear();
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use crate::command::prelude::Command;
  use crate::command::prelude::QuitCommand;
  use crate::test_utils;

  #[test]
  fn test_quit_flag() {
    test_utils::init();
    let mut game_state = GameState::default();
    assert_eq!(game_state.quit_flag(), false);
    game_state.set_quit_flag(true);
    assert_eq!(game_state.quit_flag(), true);
  }

  #[test]
  fn test_enqueue_inputs() {
    test_utils::init();
    let mut game_state = GameState::default();
    game_state.enqueue_inputs(vec!["test".to_string(), "test2".to_string()]);
    assert_eq!(game_state.input_queue.len(), 2);
    assert_eq!(game_state.input_queue[0], "test");
    assert_eq!(game_state.input_queue[1], "test2");
  }

  #[test]
  fn test_enqueue_input() {
    test_utils::init();
    let mut game_state = GameState::default();
    game_state.enqueue_input("test".to_string());
    assert_eq!(game_state.input_queue.len(), 1);
  }

  #[test]
  fn test_dequeue_input() {
    test_utils::init();
    let mut game_state = GameState::default();
    game_state.enqueue_input("test".to_string());
    let input = game_state.dequeue_input();
    assert_eq!(input, Some("test".to_string()));
    assert_eq!(game_state.input_queue.len(), 0);
  }

  #[test]
  fn test_clear_input_queue() {
    test_utils::init();
    let mut game_state = GameState::default();
    game_state.enqueue_input("test".to_string());
    game_state.clear_input_queue();
    assert_eq!(game_state.input_queue.len(), 0);
  }

  #[test]
  fn test_enqueue_command() {
    test_utils::init();
    let mut game_state = GameState::default();
    game_state.enqueue_command(Command::Quit(QuitCommand));
    assert_eq!(game_state.command_queue.len(), 1);
  }

  #[test]
  fn test_dequeue_command() {
    test_utils::init();
    let mut game_state = GameState::default();
    game_state.enqueue_command(Command::Quit(QuitCommand));
    let command = game_state.dequeue_command();
    assert_eq!(command, Some(Command::Quit(QuitCommand)));
    assert_eq!(game_state.command_queue.len(), 0);
  }

  #[test]
  fn test_new() {
    test_utils::init();
    let game_state = GameState::new();
    assert_eq!(game_state.quit_flag(), false);
    assert_eq!(game_state.input_queue.len(), 0);
    assert_eq!(game_state.command_queue.len(), 0);
  }
}
