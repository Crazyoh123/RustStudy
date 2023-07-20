use std::io;

fn fibonacci(n: i32) -> i32 {
    if n == 0 {
        return 0;
    } else if n == 1 {
        return 1;
    } else {
        return fibonacci(n - 1) + fibonacci(n - 2);
    }
}

fn main() {
    let mut number = String::new();
    io::stdin().read_line(&mut number).expect("Failed to read input");
    let a: i32 = number.trim().parse().expect("Invalid input");

    for i in 0..a {
        print!("{} ", fibonacci(i));
    }
}
