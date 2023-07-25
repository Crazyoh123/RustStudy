//Implement a program to convert a string to uppercase or lowercase.

fn main() {
    let str1 = String::from("uppercase or LOWERCASE");

    for i in 0..str1.len() {
        let ch = str1.chars().nth(i).expect("REASON");

        if ch.is_ascii_uppercase() {
            let upper_ch = ch.to_lowercase();
            print!("{}", upper_ch);
        } else {
            let lower_ch = ch.to_uppercase();
            print!("{}", lower_ch);
        }
    }
}
