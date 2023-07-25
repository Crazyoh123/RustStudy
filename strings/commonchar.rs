//Create a function to find the most common character in a two string.

fn main() {
    let str1 = String::from("string");
    let str2 = String::from("stri");
    let mut common_chars = Vec::new();
    for i in 0..str1.len(){
        for j in 0..str2.len(){
            if str1.chars().nth(i) == str2.chars().nth(j){
                common_chars.push(str1.chars().nth(i).unwrap());
            }
            else{
                continue;
            }
        }
    }
    for i in 0..common_chars.len(){
        print!("{}",common_chars[i]);
    }
}