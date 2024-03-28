use std::io::{Error, ErrorKind, Result, Write};
use std::str;

/// A mock output writer for testing.
#[derive(Debug, Default)]
pub struct OutputMock {
  /// The output buffer.
  pub output: Vec<String>,
}

impl Write for OutputMock {
  fn write(&mut self, buf: &[u8]) -> Result<usize> {
    let s = match str::from_utf8(buf) {
      Ok(v) => v,
      Err(e) => return Err(Error::new(ErrorKind::InvalidData, e)),
    };
    self.output.push(s.to_string());
    Ok(buf.len())
  }

  fn flush(&mut self) -> Result<()> {
    Ok(())
  }
}

impl OutputMock {
  /// Create a new OutputMock.
  pub fn new() -> Self {
    OutputMock { output: Vec::new() }
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_write() {
    let mut output = OutputMock::new();
    output.write(b"test").unwrap();
    assert_eq!(output.output, vec!["test"]);
  }

  #[test]
  fn test_write_multiple() {
    let mut output = OutputMock::new();
    output.write(b"test").unwrap();
    output.write(b"test2").unwrap();
    assert_eq!(output.output, vec!["test", "test2"]);
  }

  #[test]
  fn test_flush() {
    let mut output = OutputMock::new();
    output.flush().unwrap();
  }
}
