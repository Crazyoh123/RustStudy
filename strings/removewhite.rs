//Write a function to remove all whitespace characters from a string.



fn main(){
    let str1 = String::from("remove all whitespace characters");
    let mut str2 = String::new();
    for i in 0..str1.len(){
        if str1.chars().nth(i) == Some(' '){
            continue;
        }
        else{
            if let Some(ch) = str1.chars().nth(i) {
                str2.push(ch);
            }
        }
    }
    print!("{}",str2);
}