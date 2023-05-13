use std::io;

fn main() {
    let mut n = String::new();
    io::stdin()
        .read_line(&mut n)
        .expect("Not able to take input");
    let input: i32 = n.trim().parse().expect("Please enter a valid integet.");

    if input == 1 {
        println!("{}", 1);
        return;
    }
    if input == 2 || input == 3 {
        println!("NO SOLUTION");
        return;
    }
    if input % 2 == 0 {
        for i in (2..=input).step_by(2) {
            print!("{} ", i);
        }
        for i in (1..=input).step_by(2) {
            print!("{} ", i);
        }
    } else {
        for i in (2..=input - 1).rev().step_by(2) {
            print!("{} ", i);
        }
        for i in (1..=input).rev().step_by(2) {
            print!("{} ", i);
        }
    }
}
