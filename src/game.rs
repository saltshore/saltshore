/// An error type for the game.
pub mod error;
/// The game loop.
pub mod r#loop;
/// The game state.
pub mod state;

/// The game prelude.
pub mod prelude {
  pub use crate::game::error::GameError;
  pub use crate::game::r#loop::GameLoop;
  pub use crate::game::state::GameState;
}
