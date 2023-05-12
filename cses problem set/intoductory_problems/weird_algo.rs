use std::io;

fn main() {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to take any input");
    let mut number: u64 = input.trim().parse().expect("Please enter a valid number");

    loop {
        print!("{} ", number);
        if number % 2 != 0 {
            if number == 1 {
                break;
            }
            number = (number * 3) + 1;
        } else {
            number /= 2;
        }
    }
}
