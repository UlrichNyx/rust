// ### Author
// 
// * UlrichNyx
//
// ### Date
//
// * 2024-10-10
//
// ### Description
//
// A Rust program that defines an enum `Transportation` to represent different modes of transportation.
// The program includes a method `transportation_time` to return the estimated travel time for each mode.
// The main function demonstrates calling this method for each variant of the enum.

/// Enum representing various modes of transportation.
enum Transportation {
    Car,
    Bike,
    Bus,
    Walking,
}

impl Transportation {
    /// ### transportation_time
    ///
    /// Returns the estimated transportation time in minutes based on the mode of transportation.
    ///
    /// ### Returns
    ///
    /// * `u32` - The estimated transportation time as a 32-bit unsigned integer.
    fn transportation_time(&self) -> u32 {
        match self {
            Transportation::Bike => 25,
            Transportation::Car => 20,
            Transportation::Bus => 22,
            Transportation::Walking => 40,
        }
    }
}

/// ### Main function
///
/// Demonstrates the usage of the `Transportation` enum by printing the estimated transportation time
/// for each mode.
fn main() {
    println!("{}", Transportation::Car.transportation_time());
    println!("{}", Transportation::Bike.transportation_time());
    println!("{}", Transportation::Bus.transportation_time());
    println!("{}", Transportation::Walking.transportation_time());
}
