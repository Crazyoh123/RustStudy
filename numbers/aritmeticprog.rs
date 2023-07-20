use std::io;

fn nthterm(A: i32, B: i32, n: i32) {
    let S: i32 = A + (n - 1) * B;
    println!("{}", S);
}

fn main() {
    let mut str1 = String::new();
    io::stdin().read_line(&mut str1).expect("Failed to read input");
    let num: i32 = str1.trim().parse().expect("Invalid number");

    let mut str2 = String::new();
    io::stdin().read_line(&mut str2).expect("Failed to read input");
    let num2: i32 = str2.trim().parse().expect("Invalid number");

    let mut str3 = String::new();
    io::stdin().read_line(&mut str3).expect("Failed to read input");
    let num3: i32 = str3.trim().parse().expect("Invalid number");

    nthterm(num, num2, num3);
}
