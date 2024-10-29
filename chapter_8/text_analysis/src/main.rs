// Author: UlrichNyx
// Date: 2024-10-28
//
// ### Word Processing Application
// This application processes a block of text to:
// 1. Count the total number of words
// 2. Sort words lexicographically
// 3. Generate a frequency map for each word

use std::collections::HashMap;

fn main() {
    // Input text to be analyzed
    let text = "Hello, world! My name is Ulrich Nyx and I am very excited to be here. \
                The world is beautiful and my desire to learn about it is endless.";
    
    let (mut words, count) = word_count(text);
    println!("Here is the word count: {}", count);
    println!("Sorted words: {}", word_sorter(&mut words));
    println!("Word frequency: {:?}", word_frequency(&words));
}

/// ### word_count
/// Counts the total number of words in the input text.
/// 
/// ### Returns
/// - A vector containing each word as a slice
/// - The total count of words
fn word_count(text: &str) -> (Vec<&str>, usize) {
    let words: Vec<&str> = text.split_whitespace().collect();
    let count = words.len();
    (words, count)
}

/// ### word_sorter
/// Sorts a vector of word slices lexicographically and returns a comma-separated string.
///
/// ### Returns
/// A `String` of sorted words, separated by commas.
fn word_sorter(words: &mut Vec<&str>) -> String {
    words.sort();
    words.join(", ")
}

/// ### word_frequency
/// Generates a frequency map for each word in the given vector of word slices.
///
/// ### Returns
/// A `HashMap` where the key is a word and the value is its frequency.
fn word_frequency(words: &[&str]) -> HashMap<String, u32> {
    let mut map: HashMap<String, u32> = HashMap::new();
    for &word in words {
        *map.entry(word.to_string()).or_insert(0) += 1;
    }
    map
}
