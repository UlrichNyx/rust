// Author: UlrichNyx
// Date: 2024-10-22
//
// ### birds::Redneck
// This module defines the `Redneck` bird struct and implements the `Animal` trait for it, 
// providing its unique sound and description.

use crate::animals::Animal;

/// ### Redneck Struct
/// Represents a bird species, Redneck. 
/// This struct is currently empty because no additional fields are required for this example.
pub struct Redneck;

/// ### Animal Trait Implementation for Redneck
/// Implements the `Animal` trait for the `Redneck` struct. 
/// This allows `Redneck` to provide its own implementations of the methods defined in the `Animal` trait.
impl Animal for Redneck {
    
    /// Provides the sound made by a Redneck bird.
    /// 
    /// ### Returns
    /// A string slice that represents the sound this bird makes.
    fn sound(&self) -> &str {
        "Chirp chirp!"
    }

    /// Provides a brief description of the Redneck bird.
    ///
    /// ### Returns
    /// A string slice describing this bird.
    fn description(&self) -> &str {
        "This is a red bird!"
    }
}
