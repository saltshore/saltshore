use thiserror::Error as ThisError;

/// Any error that can occur when running the game.
#[derive(ThisError, Clone, Copy, Debug)]
pub enum GameError {}
