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
// A Rust program defining a `Rectangle` struct with width and height fields. The program includes 
// an implementation block for the struct, providing a method to calculate the area of the rectangle. 
// The main function demonstrates the usage of the struct by creating a rectangle instance, 
// printing its debug information, and displaying its area.

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    /// ### area
    ///
    /// Calculates the area of the rectangle.
    ///
    /// ### Returns
    ///
    /// * `u32` - The area of the rectangle as a 32-bit unsigned integer.
    fn area(&self) -> u32 {
        self.width * self.height
    }
}

/// ### Main function
///
/// Creates a `Rectangle` instance, prints its details, and calculates and displays its area.
fn main() {
    let rect: Rectangle = Rectangle { width: 30, height: 50 };
    println!("{:?}", &rect);
    println!("{}", rect.area());
}
