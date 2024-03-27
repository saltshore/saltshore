/// The game state.
///
/// This struct defines the game state for Saltshore.
#[derive(Clone, Copy, Debug, Default)]
pub struct GameState {
  /// Whether the game is finished.
  pub quit_flag: bool,
}
