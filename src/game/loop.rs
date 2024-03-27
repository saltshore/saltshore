use crate::game::error::GameError;
use crate::game::state::GameState;

/// The game loop.
///
/// This struct defines the game loop for Saltshore.
#[derive(Clone, Copy, Debug, Default)]
pub struct GameLoop {
  /// The game state.
  state: GameState,
}

impl GameLoop {
  /// Create a new game loop.
  pub fn new() -> Self {
    Self {
      state: GameState::default(),
    }
  }

  /// The actual game loop.
  pub fn run(&self) -> Result<(), GameError> {
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
  fn setup(&self) -> Result<(), GameError> {
    Ok(())
  }

  /// Handle player commands or AI decisions.
  fn process_input(&self) -> Result<(), GameError> {
    Ok(())
  }
  /// Update game state, NPC behaviors, environment changes, etc.
  fn update(&self) -> Result<(), GameError> {
    Ok(())
  }
  /// Send updates to players or render the game state in some form.
  fn process_output(&self) -> Result<(), GameError> {
    Ok(())
  }
  /// Perform any necessary cleanup before the game loop exits.
  fn teardown(&self) -> Result<(), GameError> {
    Ok(())
  }
}
