use std::io::{BufRead, Read, Result};

/// A mock input source for testing.
///
/// This should be used in place of a real input source when testing code that
/// reads input.
///
/// It allows us to provide a series of lines that will be read by the input
/// source as if they were discrete lines of input from the player, allowing us
/// to simulate a player's input over a game session.
#[derive(Debug, Default)]
pub struct InputMock {
  lines: Vec<String>,
  current_line: usize,
}

impl InputMock {
  /// Create a new InputMock.
  pub fn new(mut lines: Vec<String>) -> InputMock {
    for line in lines.iter_mut() {
      line.push('\n');
    }
    InputMock { lines, current_line: 0 }
  }

  /// Adds a line of input to the end of the mock input.
  pub fn add_line(&mut self, line: String) {
    self.lines.push(format!("{}\n", line.trim()));
  }
}

impl Read for InputMock {
  /// Reads the next line of input into the buffer.
  fn read(&mut self, buf: &mut [u8]) -> Result<usize> {
    // If we've reached the end of the input, return 0 to indicate EOF.
    if self.current_line >= self.lines.len() {
      return Ok(0);
    }
    // Otherwise, copy the next line of input into the buffer, up to the length
    // of the buffer or the length of the line, whichever is smaller.
    let current_line_bytes = self.lines[self.current_line].as_bytes();
    let mut bytes_written = 0;
    for (i, byte) in current_line_bytes.iter().enumerate() {
      if i >= buf.len() {
        break;
      }
      buf[i] = *byte;
      bytes_written += 1;
    }
    // Simulate moving to the next line after reading the current one.
    self.current_line += 1;
    Ok(bytes_written)
  }
}

impl BufRead for InputMock {
  /// Fills the internal buffer with more input data.
  fn fill_buf(&mut self) -> Result<&[u8]> {
    // If we've reached the end of the input, return an empty slice to indicate
    // EOF.
    if self.current_line >= self.lines.len() {
      return Ok(&[]);
    }
    // Otherwise, return the next line of input.
    Ok(self.lines[self.current_line].as_bytes())
  }

  /// Consumes the next `amt` bytes from the input buffer.
  fn consume(&mut self, amt: usize) {
    if self.current_line < self.lines.len() {
      let line = &self.lines[self.current_line];
      let len = line.len();
      if amt >= len {
        self.current_line += 1;
      }
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  /// Read a single line of input.
  #[test]
  fn test_read() {
    let mut input = InputMock::new(vec!["test".to_string()]);
    let mut buf = [0; 5];
    assert_eq!(input.read(&mut buf).unwrap(), 5);
    assert_eq!(&buf, b"test\n");
  }

  /// Read a single line of input with a buffer.
  #[test]
  fn test_fill_buf() {
    let mut input = InputMock::new(vec!["test".to_string()]);
    let buf = input.fill_buf().unwrap();
    assert_eq!(buf, b"test\n");
  }

  /// Confirm that we correctly detect EOF.
  #[test]
  fn test_eof() {
    let mut input = InputMock::new(vec!["test".to_string()]);
    let mut buf = [0; 4];
    input.read(&mut buf).unwrap();
    assert_eq!(input.read(&mut buf).unwrap(), 0);
  }

  #[test]
  fn test_multiple() {
    let mut input = InputMock::new(vec!["test".to_string(), "test2".to_string()]);
    let mut buf = [0; 4];
    let mut buf2 = [0; 5];
    assert_eq!(input.read(&mut buf).unwrap(), 4);
    assert_eq!(&buf, b"test");
    assert_eq!(input.read(&mut buf2).unwrap(), 5);
    assert_eq!(&buf2, b"test2");
  }

  #[test]
  fn test_string() {
    let mut input = InputMock::new(vec!["test".to_string()]);
    let mut s = String::new();
    input.read_to_string(&mut s).unwrap();
    assert_eq!(s, "test\n");
  }

  #[test]
  fn test_string_multiple() {
    let mut input = InputMock::new(vec!["test".to_string(), "test2".to_string()]);
    let mut s = String::new();
    input.read_line(&mut s).unwrap();
    assert_eq!(s, "test\n");
    s.clear();
    input.read_line(&mut s).unwrap();
    assert_eq!(s, "test2\n");
  }
}
