/// An error type for output handling.
pub mod error;
/// A mock output writer for testing.
pub mod mock;
/// A type for writing output.
pub mod writer;

/// The output prelude.
pub mod prelude {
  pub use crate::output::error::OutputError;
  pub use crate::output::mock::OutputMock;
  pub use crate::output::writer::FileWriter;
  pub use crate::output::writer::MockWriter;
  pub use crate::output::writer::OutputWriter;
  pub use crate::output::writer::StderrWriter;
  pub use crate::output::writer::StdoutWriter;
}
