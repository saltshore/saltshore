use thiserror::Error as ThisError;

/// An error that can occur when executing a command.
///
/// This error type is used to represent errors that can occur when executing a
/// command.
#[derive(ThisError, Clone, Copy, Debug)]
pub enum CommandError {}
