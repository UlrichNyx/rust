// Author: UlrichNyx
// Date: 2024-10-22
//
// ### mammals::Dog
// This module defines the `Dog` struct and implements the `Animal` trait for it,
// providing its unique sound and description.

use crate::animals::Animal;

/// ### Dog Struct
/// Represents a domestic dog. 
/// This struct is currently empty because no additional fields are required for this example.
pub struct Dog;

/// ### Animal Trait Implementation for Dog
/// Implements the `Animal` trait for the `Dog` struct. 
/// This allows `Dog` to provide its own implementations of the methods defined in the `Animal` trait.
impl Animal for Dog {

    /// Provides the sound made by a Dog.
    ///
    /// ### Returns
    /// A string slice that represents the sound a dog makes.
    fn sound(&self) -> &str {
        "Woof!"
    }

    /// Provides a brief description of the Dog.
    ///
    /// ### Returns
    /// A string slice describing this animal.
    fn description(&self) -> &str {
        "This is a dog. Dogs bark."
    }
}
