// Author: UlrichNyx
// Date: 2024-10-22
//
// ### animals::Animal Trait
// This module defines the `Animal` trait, which provides a contract for animal-related structs to define their behavior.
// It also declares submodules for different categories of animals like `mammals` and `birds`.

/// ### Animal Trait
/// The `Animal` trait defines two methods that any struct implementing this trait must provide:
/// 1. `sound`: Returns the sound the animal makes.
/// 2. `description`: Returns a brief description of the animal.
///
/// This trait serves as a common interface for different types of animals, allowing them to implement 
/// custom behavior for `sound` and `description`.
pub trait Animal {

    /// ### sound
    /// This method should return a string slice representing the sound an animal makes.
    ///
    /// ### Returns
    /// A string slice of the animal's sound (e.g., "Woof!" for a dog).
    fn sound(&self) -> &str;

    /// ### description
    /// This method should return a brief description of the animal.
    ///
    /// ### Returns
    /// A string slice containing the animal's description.
    fn description(&self) -> &str;
}

// ### Module Declarations
// Declare the submodules within the `animals` module. 
// The `mammals` module contains structs related to mammals like `Dog`, 
// while the `birds` module contains bird-related structs like `Redneck`.
pub mod mammals;
pub mod birds;
