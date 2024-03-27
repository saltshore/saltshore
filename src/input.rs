/// An error type for input handling.
pub mod error;
/// A type for reading input.
pub mod reader;

/// The input prelude.
pub mod prelude {
  pub use crate::input::error::InputError;
  pub use crate::input::reader::FileReader;
  pub use crate::input::reader::InputReader;
  pub use crate::input::reader::StdinReader;
}
