//Create a program to find the first non-repeated character in a string.
use std::collections::HashMap;

fn first_non_repeated_char(input_string: &str) -> Option<char> {
    let mut char_counts: HashMap<char, usize> = HashMap::new();

    // Count the occurrences of each character in the input string
    for ch in input_string.chars() {
        *char_counts.entry(ch).or_insert(0) += 1;
    }

    // Find the first non-repeated character
    for ch in input_string.chars() {
        if char_counts[&ch] == 1 {
            return Some(ch);
        }
    }

    None // If no non-repeated character is found, return None
}

fn main() {
    let input_string = "abacddbec";
    match first_non_repeated_char(input_string) {
        Some(ch) => println!("First non-repeated character: {}", ch),
        None => println!("No non-repeated character found."),
    }
}
