use crate::input::error::InputError;
use std::fs::File;
use std::io::{self, BufRead, BufReader, StdinLock};

/// An input source that reads from a generic reader.
#[derive(Debug)]
pub struct InputReader<R> {
  reader: R,
}

impl<R: BufRead> InputReader<R> {
  /// Create a new InputReader.
  pub fn new(reader: R) -> Self {
    Self { reader }
  }

  /// Fetch input from the reader.
  pub fn read(&mut self) -> Result<String, InputError> {
    let mut input = String::new();
    self.reader.read_line(&mut input)?;
    Ok(input.trim().to_string())
  }
}

/// A type alias for a reader from standard input.
pub type StdinReader = InputReader<StdinLock<'static>>;

/// Implement the Default trait for StdinReader.
impl Default for StdinReader {
  fn default() -> Self {
    Self::new(io::stdin().lock())
  }
}

/// A type alias for a reader from a file.
pub type FileReader = InputReader<BufReader<File>>;

#[cfg(test)]
mod tests {
  use super::*;
  use crate::test_utils;
  use std::io::BufReader;
  use std::io::Write;
  use tempfile::NamedTempFile;

  #[test]
  fn test_read() {
    test_utils::init();
    let mut file = NamedTempFile::new().unwrap();
    writeln!(file, "test").unwrap();
    file.flush().unwrap();

    let file = file.reopen().unwrap();
    let file_source = FileReader::new(BufReader::new(file));
    let mut input_source = file_source;
    let input = input_source.read().unwrap();
    assert_eq!(input, "test");
  }
}
