// Author: UlrichNyx
// Date: 2024-10-22
//
// ### Module Imports
// The animals module is declared (pointing to animals/mod.rs), where all animal-related structs and traits are stored.
mod animals;  // Declare the animals module (this points to the animals/mod.rs file)

// Import the Redneck bird struct from the birds module within animals.
use animals::birds::Redneck;

// Import the Animal trait from mod.rs for shared behavior definitions across animals.
use crate::animals::Animal;  // Import the Animal trait from mod.rs

// Import the Dog struct from the mammals module within animals.
use crate::animals::mammals::Dog;  // Import Dog struct from mammals.rs

/// ### main function
/// This function demonstrates polymorphism using the `Animal` trait and prints the behavior of different animal structs.
///
/// * It creates instances of `Dog` and `Redneck` and calls their respective methods defined by the `Animal` trait.
/// * The output is displayed via `println!`.
fn main() {
    let dog = Dog;  // Create an instance of Dog
    let bird: Redneck = Redneck;  // Create an instance of Redneck bird
    
    // Display the sound made by the dog and the description of the bird using methods from the Animal trait.
    println!("The Dog says: {}", dog.sound());
    println!("Description: {} ", bird.description());
}
