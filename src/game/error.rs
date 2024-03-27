use crate::input::error::InputError;
use crate::output::error::OutputError;
use thiserror::Error as ThisError;

/// Any error that can occur when running the game.
#[derive(ThisError, Debug)]
pub enum GameError {
  /// An error that can occur when processing input.
  #[error("An error occurred while processing input: {0}")]
  InputError(#[from] InputError),
  /// An error that can occur when processing output.
  #[error("An error occurred while processing output: {0}")]
  OutputError(#[from] OutputError),
}
