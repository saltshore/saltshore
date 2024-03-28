use super::mock::OutputMock;
use crate::output::error::OutputError;
use std::fs::File;
use std::io::{self, BufWriter, Stderr, Stdout, Write};

/// A generic output writer.
#[derive(Debug)]
pub struct OutputWriter<W> {
  writer: W,
}

impl<W: Write> OutputWriter<W> {
  /// Create a new OutputWriter.
  pub fn new(writer: W) -> Self {
    Self { writer }
  }

  /// Send output to the writer.
  pub fn writeln(&mut self, output: &str) -> Result<(), OutputError> {
    writeln!(self.writer, "{}", output)?;
    Ok(())
  }

  /// Send output to the writer.
  pub fn write(&mut self, output: &str) -> Result<(), OutputError> {
    write!(self.writer, "{}", output)?;
    Ok(())
  }

  /// Flush the writer.
  pub fn flush(&mut self) -> Result<(), OutputError> {
    self.writer.flush()?;
    Ok(())
  }

  /// Prompt the player for input.
  pub fn prompt(&mut self) -> Result<(), OutputError> {
    self.write("> ")?;
    self.flush()?;
    Ok(())
  }
}

/// A type alias for a writer to standard output.
pub type StdoutWriter = OutputWriter<Stdout>;

/// Implement the Default trait for StdoutWriter.
impl Default for StdoutWriter {
  fn default() -> Self {
    Self::new(io::stdout())
  }
}

/// A type alias for a writer to standard error.
pub type StderrWriter = OutputWriter<Stderr>;

/// Implement the Default trait for StderrWriter.
impl Default for StderrWriter {
  fn default() -> Self {
    Self::new(io::stderr())
  }
}

/// A type alias for a writer to a file.
pub type FileWriter = OutputWriter<BufWriter<File>>;

/// A type alias for a mock writer.
pub type MockWriter = OutputWriter<OutputMock>;

/// Implement the Default trait for MockWriter.
impl Default for MockWriter {
  fn default() -> Self {
    Self::new(OutputMock::default())
  }
}

/// Implement the MockWriter.
impl MockWriter {
  /// Get the output from the mock writer.
  pub fn output(&self) -> Vec<String> {
    self.writer.output.clone()
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use crate::test_utils;
  use std::io::BufWriter;
  use tempfile::NamedTempFile;

  #[test]
  fn test_write() {
    test_utils::init();
    let file = NamedTempFile::new().unwrap();
    let mut sink = FileWriter::new(BufWriter::new(file.reopen().unwrap()));
    sink.writeln("test").unwrap();
    sink.writeln("test2").unwrap();
    sink.flush().unwrap();
    let contents = std::fs::read_to_string(file.path()).unwrap();
    assert_eq!(contents, "test\ntest2\n");
  }

  #[test]
  fn test_write_stdout() {
    test_utils::init();
    let mut sink = StdoutWriter::new(std::io::stdout());
    // sink.writeln("test").unwrap();
    sink.flush().unwrap();
  }

  #[test]
  fn test_write_stderr() {
    test_utils::init();
    let mut sink = StderrWriter::new(std::io::stderr());
    // sink.writeln("test").unwrap();
    sink.flush().unwrap();
  }

  #[test]
  fn test_write_file() {
    test_utils::init();
    let file = NamedTempFile::new().unwrap();
    let mut sink = FileWriter::new(BufWriter::new(file.reopen().unwrap()));
    sink.writeln("test").unwrap();
    sink.writeln("test2").unwrap();
    sink.flush().unwrap();
    let contents = std::fs::read_to_string(file.path()).unwrap();
    assert_eq!(contents, "test\ntest2\n");
  }
}
