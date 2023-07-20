use std::io;

fn main() {
    let mut array1: Vec<i32> = vec![1, 2, 3, 4, 5];
    let pos: usize = 3;

    for _ in 0..pos {
        let last_element = array1.pop().unwrap();
        array1.insert(0, last_element);
    }

    println!("{:?}", array1);
}
