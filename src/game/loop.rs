use crate::game::error::GameError;
use crate::game::state::GameState;
use crate::input::prelude::StdinReader;
use crate::output::prelude::StdoutWriter;

/// The game loop.
///
/// This struct defines the game loop for Saltshore.
#[derive(Debug, Default)]
pub struct GameLoop {
  /// The game state.
  state: GameState,
  /// The input reader.
  input: StdinReader,
  /// The output writer.
  output: StdoutWriter,
}

impl GameLoop {
  /// Create a new game loop.
  pub fn new() -> Self {
    Self {
      state: GameState::default(),
      input: StdinReader::default(),
      output: StdoutWriter::default(),
    }
  }

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
    self.state.quit_flag
  }

  /// Initialize game world, load assets, etc.
  fn setup(&mut self) -> Result<(), GameError> {
    Ok(())
  }

  /// Handle player commands or AI decisions.
  fn process_input(&mut self) -> Result<(), GameError> {
    let buffer = self.input.read()?;
    match buffer.trim() {
      "quit" => self.state.quit_flag = true,
      "exit" => self.state.quit_flag = true,
      _ => (),
    }
    Ok(())
  }
  /// Update game state, NPC behaviors, environment changes, etc.
  fn update(&mut self) -> Result<(), GameError> {
    Ok(())
  }
  /// Send updates to players or render the game state in some form.
  fn process_output(&mut self) -> Result<(), GameError> {
    self
      .output
      .writeln("You are standing in an open field west of a white house, with a boarded front door.".to_string())?;
    self.output.write("> ".to_string())?;
    self.output.flush()?;
    Ok(())
  }

  /// Perform any necessary cleanup before the game loop exits.
  fn teardown(&mut self) -> Result<(), GameError> {
    Ok(())
  }
}
