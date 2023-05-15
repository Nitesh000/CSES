use std::io;

fn main() {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Unable to take input");
    let n: u64 = input.trim().parse().expect("Please enter a valid number");
    let mut count: u64 = 0;
    let mut i: u64 = 5;
    while i <= n {
        count += n / i;
        i *= 5;
    }

    println!("{}", count);
}
