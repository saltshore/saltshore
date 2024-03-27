//! Sork, an implementation of Zork using Saltshore.
//!
//! ░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░
//! ░░▒▓███████▓▒░░▒▓██████▓▒░░▒▓███████▓▒░░▒▓█▓▒░░▒▓█▓▒░░░
//! ░▒▓█▓▒░░░░░░░░▒▓█▓▒░░▒▓█▓▒░▒▓█▓▒░░▒▓█▓▒░▒▓█▓▒░░▒▓█▓▒░░░
//! ░▒▓█▓▒░░░░░░░░▒▓█▓▒░░▒▓█▓▒░▒▓█▓▒░░▒▓█▓▒░▒▓█▓▒░░▒▓█▓▒░░░
//! ░░▒▓██████▓▒░░▒▓█▓▒░░▒▓█▓▒░▒▓███████▓▒░░▒▓███████▓▒░░░░
//! ░░░░░░░░▒▓█▓▒░▒▓█▓▒░░▒▓█▓▒░▒▓█▓▒░░▒▓█▓▒░▒▓█▓▒░░▒▓█▓▒░░░
//! ░░░░░░░░▒▓█▓▒░▒▓█▓▒░░▒▓█▓▒░▒▓█▓▒░░▒▓█▓▒░▒▓█▓▒░░▒▓█▓▒░░░
//! ░▒▓███████▓▒░░░▒▓██████▓▒░░▒▓█▓▒░░▒▓█▓▒░▒▓█▓▒░░▒▓█▓▒░░░
//! ░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░
//!
//! Sork is as close as I can get to a direct port of Zork to Saltshore. It's
//! a text adventure game where you can explore a world, solve puzzles, and
//! interact with objects.
//!
//! This seemed like a reasonable place to start identifying user stories and
//! requirements for Saltshore, since Zork was a hugely formative game for me.
//!
//! Basically, my process is as follows:
//!
//! - Implement a feature from Zork, in the simplest way possible.
//! - Identify the requirements for that feature.
//! - Implement the requirements in Saltshore.
//! - Refactor the Sork code to use the Saltshore implementation.
//!

fn main() {
  println!(
    r#"
  SORK I: The Great Underground Empire
  Apologies and thanks to Tim Anderson, Marc Blank, Bruce Daniels, Dave Lebling,
  and to everyone else who created the original ZORK and other Infocom games.
  ZORK is a registered trademark of Infocom, Inc.
  Revision 1 / Serial number 12345
    "#
  );
  // As a player, I need some semblance of a game loop so that I can play the
  // game. See #6.
  // As a player, I need some game state so that I can interact with the game.
  // See #7.
  // As a player, I need to be able to enter commands so that I can interact
  // with the game. See #8.
  // As a player, I need to be able to see the game state so that I can
  // understand the game. See #9.
  // As an engineer, I need to be able to log so I can debug. See #10.
}
