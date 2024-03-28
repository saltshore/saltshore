/// Error handling for the parser module.
pub mod error;
/// Actual parser implementation.
#[allow(clippy::module_inception)]
pub mod parser;

/// The prelude for the parser module.
pub mod prelude {
  pub use crate::parser::error::ParserError;
  pub use crate::parser::parser::Parser;
}
