//Implement a program to find the length of the longest word in a sentence.

fn main() {
    let mut count = 0;
    let mut sum = 0;
    let str1 = String::from("program to find the length");
    
    for i in 0..str1.len() {
        if str1.chars().nth(i) == Some(' ') {
            if count > sum {
                sum = count;
            }
            count = 0;
        } else {
            count += 1;
        }
    }

   
    if count > sum {
        sum = count;
    }

    print!("{}", sum);
}
