//! An experimental text adventure coded in the dumbest way possible.

// Linting.
#![deny(rustdoc::bare_urls)]
#![deny(rustdoc::broken_intra_doc_links)]
#![deny(rustdoc::invalid_codeblock_attributes)]
#![deny(rustdoc::invalid_rust_codeblocks)]
#![deny(rustdoc::missing_crate_level_docs)]
#![deny(rustdoc::unescaped_backticks)]
#![deny(ambiguous_glob_imports)]
#![deny(ambiguous_glob_reexports)]
#![deny(bare_trait_objects)]
#![deny(const_item_mutation)]
#![deny(explicit_outlives_requirements)]
#![deny(let_underscore_drop)]
#![deny(meta_variable_misuse)]
#![deny(missing_copy_implementations)]
#![deny(missing_debug_implementations)]
#![deny(missing_docs)]
#![deny(non_ascii_idents)]
#![deny(single_use_lifetimes)]
#![deny(trivial_casts)]
#![deny(trivial_numeric_casts)]
#![deny(unreachable_pub)]
#![deny(unsafe_code)]
#![warn(unused_crate_dependencies)]
#![deny(unused_extern_crates)]
#![deny(unused_import_braces)]
#![deny(unused_lifetimes)]
#![deny(unused_qualifications)]
#![deny(variant_size_differences)]

#[allow(unused_imports)]
#[macro_use]
extern crate anyhow;

/// The game loop and game state.
pub mod game;
/// Basic input handling.
pub mod input;
/// Basic output handling.
pub mod output;

/// Prelude for the library.
pub mod prelude {
  pub use crate::game::prelude::*;
  pub use crate::input::prelude::*;
  pub use crate::output::prelude::*;
}

#[cfg(test)]
pub mod test_utils {

  use std::env;

  #[allow(unused_imports)]
  use super::*;

  /// Call this function at the beginning of each test.
  pub fn init() {
    // Enable backtraces.
    env::set_var("RUST_BACKTRACE", "full");
    // Enable printing messages to the console.
    env::set_var("RUST_TEST_NOCAPTURE", "1");
  }
}
