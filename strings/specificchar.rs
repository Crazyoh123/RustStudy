//Create a function to count the occurrences of a specific character in a string.

fn main(){
    let str1 = String::from("count the occurrences");
    let len = str1.len();
    let mut count = 0;

    for i in 0..len{
        if str1.chars().nth(i) == Some('o'){
            count = count + 1;
        }
    }
    print!("{}",count);
}