use crate::game::error::GameError;
use crate::game::state::GameState;
use crate::input::prelude::InputReader;
use crate::input::prelude::StdinReader;
use crate::output::prelude::OutputWriter;
use crate::output::prelude::StdoutWriter;
use crate::parser::prelude::Parser;
use std::fs::File;
use std::io::{BufRead, BufReader, BufWriter, StdinLock, Stdout, Write};

/// The game loop.
///
/// This struct defines the game loop for Saltshore.
#[derive(Debug)]
pub struct GameLoop<R, W>
where
  R: BufRead,
  W: Write,
{
  /// The game state.
  state: GameState,
  /// The input reader.
  input: InputReader<R>,
  /// The output writer.
  output: OutputWriter<W>,
  /// The parser.
  parser: Parser,
}

impl<R, W> GameLoop<R, W>
where
  R: BufRead,
  W: Write,
{
  /// The actual game loop.
  pub fn run(&mut self) -> Result<(), GameError> {
    // Initialize game world, load assets, etc.
    self.setup()?;
    // The inner core of the game loop.
    // Until we have been told to quit...
    while !self.is_finished() {
      // Send updates to the player or render the game state in some form.
      self.process_output()?;
      // Handle player commands or AI decisions.
      self.process_input()?;
      // Update game state, NPC behaviors, environment changes, etc.
      self.update()?;
    }
    // Perform any necessary cleanup before the game loop exits.
    self.teardown()?;
    Ok(())
  }

  /// Determine if the game loop should exit.
  fn is_finished(&self) -> bool {
    self.state.quit_flag()
  }

  /// Initialize game world, load assets, etc.
  fn setup(&mut self) -> Result<(), GameError> {
    Ok(())
  }

  /// Handle player commands.
  fn process_input(&mut self) -> Result<(), GameError> {
    // We will read the player's input and attempt to parse it as a command; if
    // the command is valid, we will execute it. If the command is invalid, we
    // will inform the player and prompt them to try again.

    // We check to see if we still have any commands in the queue; if we do, we
    // will execute them and return early.
    if let Some(command) = self.state.dequeue_command() {
      command.execute(&mut self.state);
      return Ok(());
    }

    // If we have no input to process, we will prompt the player for input.
    loop {
      // If we have unparsed input, we will dequeue and attempt to parse it.
      while let Some(input) = self.state.dequeue_input() {
        match self.parser.parse(&input) {
          Ok(command) => {
            command.execute(&mut self.state);
            return Ok(());
          },
          Err(_) => {
            // If the input could not be parsed as a command, we will inform the
            // player and prompt them to try again; we will also empty the input
            // queue to reduce the likelihood of the player going down the wrong
            // path because of a simple typo.
            self
              .output
              .writeln(&format!("I'm sorry, I don't understand \"{}\".", input))?;
            self.state.clear_input_queue();
          },
        }
      }
      // If we have no input to process, we will prompt the player for input.
      self.output.prompt()?;
      // When we have input, we will read it and enqueue it for processing.
      match self.input.read_inputs()? {
        Some(inputs) => {
          for input in inputs {
            self.state.enqueue_input(input);
          }
        },
        None => {
          // This means we received an EOF.
          self.state.set_quit_flag(true);
          return Ok(());
        },
      }
    }
  }

  /// Update game state, NPC behaviors, environment changes, etc.
  fn update(&mut self) -> Result<(), GameError> {
    Ok(())
  }

  /// Send updates to players or render the game state in some form.
  fn process_output(&mut self) -> Result<(), GameError> {
    self
      .output
      .writeln("You are standing in an open field west of a white house, with a boarded front door.")?;
    Ok(())
  }

  /// Perform any necessary cleanup before the game loop exits.
  fn teardown(&mut self) -> Result<(), GameError> {
    Ok(())
  }
}

impl GameLoop<StdinLock<'static>, Stdout> {
  /// Create a new game loop with standard input and output.
  pub fn new_with_stdio() -> Self {
    GameLoop {
      state: GameState::default(),
      input: StdinReader::default(),
      output: StdoutWriter::default(),
      parser: Parser,
    }
  }
}

impl GameLoop<BufReader<File>, BufWriter<File>> {
  /// Create a new game loop with files for input and output.
  pub fn new_with_files(input: &str, output: &str) -> Self {
    let input_file = File::open(input).unwrap();
    let output_file = File::create(output).unwrap();
    GameLoop {
      state: GameState::default(),
      input: InputReader::new(BufReader::new(input_file)),
      output: OutputWriter::new(BufWriter::new(output_file)),
      parser: Parser,
    }
  }
}

impl Default for GameLoop<StdinLock<'static>, Stdout> {
  fn default() -> Self {
    GameLoop {
      state: GameState::default(),
      input: StdinReader::default(),
      output: StdoutWriter::default(),
      parser: Parser,
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use crate::command::prelude::{Command, QuitCommand};
  use crate::input::prelude::MockReader;
  use crate::output::prelude::MockWriter;

  #[test]
  fn test_run() {
    let mut game_loop = GameLoop::new_with_stdio();
    let quit_command = Command::Quit(QuitCommand);
    quit_command.execute(&mut game_loop.state);
    assert!(game_loop.run().is_ok());
  }

  #[test]
  fn test_is_finished() {
    let mut game_loop = GameLoop::new_with_stdio();
    assert_eq!(game_loop.is_finished(), false);
    let quit_command = Command::Quit(QuitCommand);
    quit_command.execute(&mut game_loop.state);
    assert!(game_loop.is_finished());
  }

  #[test]
  fn test_setup() {
    let mut game_loop = GameLoop::new_with_stdio();
    assert!(game_loop.setup().is_ok());
  }

  #[test]
  fn test_process_input() {
    let mut game_loop = GameLoop {
      state: GameState::default(),
      input: MockReader::default(),
      output: MockWriter::default(),
      parser: Parser,
    };
    assert!(game_loop.process_input().is_ok());
  }

  #[test]
  fn test_update() {
    let mut game_loop = GameLoop::new_with_stdio();
    assert!(game_loop.update().is_ok());
  }

  #[test]
  fn test_process_output() {
    let mut game_loop = GameLoop::new_with_stdio();
    assert!(game_loop.process_output().is_ok());
  }

  #[test]
  fn test_teardown() {
    let mut game_loop = GameLoop::new_with_stdio();
    assert!(game_loop.teardown().is_ok());
  }
}
