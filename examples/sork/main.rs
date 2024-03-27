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
  println!("Hello, world!");
}
