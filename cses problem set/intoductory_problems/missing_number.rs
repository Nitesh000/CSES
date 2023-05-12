use std::io;

fn main() {
    let mut range = String::new();
    io::stdin()
        .read_line(&mut range)
        .expect("Pleae enter a valid input");
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Pleae enter a valid input");
    let range: u64 = range.trim().parse().expect("unable to parse");
    let input: Vec<u64> = input
        .split_whitespace()
        .map(|x| x.parse().expect("falied to parse."))
        .collect();

    let result_should = range * (range + 1) / 2;
    let result_now: u64 = input.iter().sum();
    println!("{}", result_should - result_now);
}
