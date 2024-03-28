use crate::input::error::InputError;
use crate::input::mock::InputMock;
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
  pub fn read(&mut self) -> Result<Option<String>, InputError> {
    let mut input = String::new();
    let bytes_read = self.reader.read_line(&mut input)?;
    if bytes_read == 0 || input.is_empty() {
      // If no bytes were read, the reader has reached the end of the input.
      return Ok(None);
    }
    Ok(Some(input.trim().to_string()))
  }

  /// Read a list of potential commands from the reader.
  ///
  /// The player's input can be split by semi-colons to allow for multiple
  /// commands. Of course, we cannot treat the input as a command until we
  /// have parsed it.
  pub fn read_inputs(&mut self) -> Result<Option<Vec<String>>, InputError> {
    let mut input = String::new();
    let bytes_read = self.reader.read_line(&mut input)?;
    if bytes_read == 0 || input.is_empty() {
      // If no bytes were read, the reader has reached the end of the input.
      return Ok(None);
    }
    let lines = input.trim().split(';').map(|s| s.to_string()).collect();
    Ok(Some(lines))
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

/// A type alias for a reader from a mock source.
pub type MockReader = InputReader<InputMock>;

/// Implement the Default trait for MockReader.
impl Default for MockReader {
  fn default() -> Self {
    Self::new(InputMock::default())
  }
}

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
    let input = input_source.read().unwrap().unwrap();
    assert_eq!(input, "test");
  }

  #[test]
  fn test_read_mock_input() {
    test_utils::init();
    let input = InputMock::new(vec!["test".to_string()]);
    let mut input_source = MockReader::new(input);
    let input = input_source.read().unwrap().unwrap();
    assert_eq!(input, "test");
  }

  #[test]
  fn test_eof() {
    test_utils::init();
    let input = InputMock::new(vec!["test".to_string()]);
    let mut input_source = MockReader::new(input);
    let input = input_source.read().unwrap().unwrap();
    assert_eq!(input, "test");
    let input = input_source.read().unwrap();
    assert!(input.is_none());
  }
}
