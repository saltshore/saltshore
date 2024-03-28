/// Actual commands that can be executed.
#[allow(clippy::module_inception)]
pub mod command;
/// Actual commands that can be executed.
pub mod commands;
/// An error that can occur when executing a command.
pub mod error;

/// The prelude for the command module.
pub mod prelude {
  pub use crate::command::command::Command;
  pub use crate::command::commands::*;
  pub use crate::command::error::CommandError;
}
