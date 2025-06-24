use std::io;

fn main() -> io::Result<()> {
    /*
    Features to Implement

    Word Count
        Split the input by whitespace and count the number of words.
        Practice borrowing a string slice.

    Character Count (excluding whitespace)
        Iterate over characters and filter out spaces.
        Pay attention to ownership vs borrowing of char iterators.

    Find the Longest Word
        Loop through the words and track the longest one.
        Use string slices (&str) to avoid taking ownership.

    Return the First Sentence
        Find the index of the first period (.) and return the substring before or including it.
        Use slicing, and test it on edge cases like no periods or multiple ones.
     */
    
    let mut buffer = String::new();
    let stdin = io::stdin();
    
    stdin.read_line(&mut buffer)?;
    
    let words_count = count_words(&buffer);
    let char_count = char_count(&buffer);
    let longest_word_length = longest_word(&buffer);
    let first_sentence = first_sentence(&buffer);
    
    println!("Stats:");
    println!("  words_count: {}", words_count);
    println!("  char_count: {}", char_count);
    println!("  longest_word: {}", longest_word_length);
    println!("  first_sentence: {}", first_sentence);
    Ok(())
}
fn count_words(buffer: &str) -> usize {
    let mut wc = 0;
    for c in buffer.chars() {
        if c == ' ' {
            wc += 1;
        }
    }
    wc += 1;
    wc
}

fn char_count(buffer: &str) -> usize {
    let mut count = 0;
    for c in buffer.chars() {
        if c.is_alphanumeric() {
            count += 1;
        }
    }
    count
}
fn longest_word(buffer: &str) -> &str {
    let mut longest: &str = "";

    for word in buffer.split_whitespace(){
        if word.len() > longest.len() {
            longest = word;
        }
    }
    
    longest
}

fn first_sentence(buffer: &str) -> String {
    let mut count = 0;
    let sentence_end = vec!['.', '!', '?'];
    
    for c in buffer.chars(){
        if sentence_end.contains(&c){
           break;
        }else{
            count += 1;
        }
    }
    if buffer.chars().count() == count{
        buffer.to_string()
    }else{
        buffer[..count+1].to_string()
    }
}