fn main() {
    println!("{}",longest_word("This is the longest word possible!"));
}


fn longest_word(text: &str) -> &str
{
    let words: Vec<&str> = text.split(" ").collect();
    let mut maximum_length = 0;
    let mut index = 0;
    for (idx , word) in words.iter().enumerate() {
        if word.len() > maximum_length {
            maximum_length = word.len();
            index = idx;
        }
    }
    return words[index];
}