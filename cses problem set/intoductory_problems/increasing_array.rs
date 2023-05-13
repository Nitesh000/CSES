use std::io;

fn main() {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    let len: usize = input.trim().parse().expect("Invalid input");
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read the line.");
    let values: Vec<i64> = input
        .trim()
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();
    let mut count: i64 = 0;
    let mut high: i64 = 0;
    for x in 1..len {
        let now = values.get(x).unwrap().clone();
        let prev = values.get(x - 1).unwrap().clone();

        if prev > high {
            high = prev;
        }

        if now < high {
            count = count + (high - now);
        }
    }
    println!("{}", count);
}
