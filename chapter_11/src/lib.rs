//! ### Rectangle Comparisons and Simple Addition
//! Author: UlrichNyx  
//! Date: 2024-11-19  
//! 
//! This module demonstrates basic Rust functionality, including:
//! - Implementing a `Rectangle` struct with methods.
//! - Writing unit tests for functionality.
//! - Performing simple addition using a standalone function.

/// ### Add Function
/// Adds two unsigned integers.
///
/// ### Parameters:
/// - `left`: The first number.
/// - `right`: The second number.
///
/// ### Returns:
/// - The sum of `left` and `right`.
pub fn add(left: usize, right: usize) -> usize {
    left + right
}

/// ### Rectangle Struct
/// Represents a rectangle with a width and height.
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    /// Checks if the current rectangle can completely contain another rectangle.
    ///
    /// ### Parameters:
    /// - `other`: A reference to another `Rectangle`.
    ///
    /// ### Returns:
    /// - `true` if the current rectangle can contain the other rectangle.
    /// - `false` otherwise.
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Tests if a larger rectangle can hold a smaller rectangle.
    #[test]
    fn larger_can_hold_smaller() {
        let smaller = Rectangle {
            width: 3,
            height: 6,
        };
        let larger = Rectangle {
            width: 6,
            height: 12,
        };

        assert!(larger.can_hold(&smaller));
    }

    /// Tests if a smaller rectangle cannot hold a larger rectangle.
    #[test]
    fn smaller_cant_hold_larger() {
        let smaller = Rectangle {
            width: 3,
            height: 6,
        };
        let larger = Rectangle {
            width: 6,
            height: 12,
        };

        assert!(!smaller.can_hold(&larger));
    }
}
