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
// A Rust program that finds and prints the longest word from a given text. It demonstrates 
// how to split a string, iterate over elements, and find the longest item based on its length.

/// ### Main function
///
/// This is the entry point of the application. It calls the `longest_word` function with a sample
/// text and prints the longest word to the console.
fn main() {
    println!("{}", longest_word("This is the longest word possible!"));
}

/// Finds the longest word in a given string.
///
/// ### Arguments
///
/// * `text` - A string slice representing the text from which the longest word will be found.
///
/// ### Returns
///
/// * `&str` - A string slice of the longest word found in the given text.
fn longest_word(text: &str) -> &str {
    let words: Vec<&str> = text.split(" ").collect();
    let mut maximum_length = 0;
    let mut index = 0;
    
    for (idx, word) in words.iter().enumerate() {
        if word.len() > maximum_length {
            maximum_length = word.len();
            index = idx;
        }
    }
    
    return words[index];
}
