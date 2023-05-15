use std::io;

fn main() {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Unable to read any input");

    let n: u64 = input.trim().parse().expect("Unable to parse input");

    let mut result: Vec<&str> = Vec::new();

    for _ in 0..n {
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Unable to read any input");
        let mut pair = input
            .trim()
            .split_whitespace()
            .map(|x| x.parse::<u64>().expect("Please enter a valid input"));

        let a = pair.next().unwrap();
        let b = pair.next().unwrap();

        if a * 2 < b || b * 2 < a {
            result.push("NO");
        } else {
            if (a + b) % 3 == 0 {
                result.push("YES");
            } else {
                result.push("NO");
            }
        }
    }

    for i in result {
        println!("{}", i);
    }
}
