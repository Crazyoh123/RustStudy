//Print a Right-Angled Triangle Pattern:
*
**
***
****
*****

use std::io;

fn main() {
    let n: i32 = 5;
    let sk: String = "*".to_string();

    for i in 0..n {
        for j in 0..i {
            print!("{}", sk);
        }
        println!();
    }
}



