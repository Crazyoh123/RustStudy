//Implement a function to check if two strings are anagrams (contain the same characters but in a different order).

fn main() {
    let str1 = String::from("check");
    let str2 = String::from("hcekc");

    let sorted_str1: String = sort_string(&str1);
    let sorted_str2: String = sort_string(&str2);

    if sorted_str1 == sorted_str2 {
        println!("It is an anagram.");
    } else {
        println!("It is not an anagram.");
    }
}

fn sort_string(s: &String) -> String {
    let mut chars: Vec<char> = s.chars().collect();
    chars.sort();
    chars.into_iter().collect()
}
