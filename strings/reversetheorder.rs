//Write a program to reverse the order of words in a sentence.

fn main() {
    let  str1 = String::from("reverse the words in a sentence");
    let mut my_string_vector: Vec<String> = Vec::new();
    let mut current_word = String::new();
    let len = str1.len();

    for i in 0..len {
        let current_char = str1.chars().nth(i).unwrap(); 

        if current_char == ' ' {
            if !current_word.is_empty() {
                my_string_vector.push(current_word.clone());
                current_word.clear();
            }
        } else {
            current_word.push(current_char);
        }
    }

    if !current_word.is_empty() {
        my_string_vector.push(current_word);
    }

    my_string_vector.reverse();

    let reversed_sentence = my_string_vector.join(" ");
    println!("{}", reversed_sentence);
}
