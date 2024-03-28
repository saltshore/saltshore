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
