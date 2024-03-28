use thiserror::Error as ThisError;

/// An error that can occur when executing a command.
///
/// This error type is used to represent errors that can occur when executing a
/// command, whether they be unexpected (the result of a bug), in-character
/// (the result of a valid command that did not complete successfully), or out-
/// of-character (the result of an invalid command or a command that cannot be
/// executed in the current context).
#[derive(ThisError, Clone, Debug, PartialEq)]
pub enum CommandError {
  /// An unexpected error occurred; this is normally a bug.
  #[error("An unexpected error occurred: {0}")]
  Unexpected(String),
  /// An in-character error occurred (e.g. "The key twists and turns but fails
  /// to unlock the door.").
  #[error("{0}")]
  InCharacter(String),
  /// An out-of-character error occurred (e.g. "You can't do that here.").
  #[error("{0}")]
  OutOfCharacter(String),
}
