/// The game state.
///
/// This struct defines the game state for Saltshore.
#[derive(Clone, Copy, Debug, Default)]
pub struct GameState {
  /// Whether the game is finished.
  pub quit_flag: bool,
}

impl GameState {
  /// Get the quit flag.
  pub fn quit_flag(&self) -> bool {
    self.quit_flag
  }
  /// Set the quit flag.
  pub fn set_quit_flag(&mut self, flag: bool) {
    self.quit_flag = flag;
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
