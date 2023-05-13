use std::io;

#[allow(unused_assignments)]
fn calculate(y: u64, x: u64) -> u64 {
    let mut res = 0u64;
    if y <= x {
        if x % 2 == 0 {
            res = ((x - 1) * (x - 1)) + y;
        } else {
            res = (x * x) - (y - 1);
        }
    } else {
        if y % 2 == 0 {
            res = (y * y) - (x - 1);
        } else {
            res = ((y - 1) * (y - 1)) + x;
        }
    }
    return res;
}

fn main() {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Unable to take the inputs");
    let length: u64 = input.trim().parse().expect("Please enter a correct input");

    let mut coordinates: Vec<(u64, u64)> = Vec::new();

    for _ in 0..length {
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Unable to take the inputs");
        let mut inputs = input.split_whitespace();
        let y: u64 = inputs
            .next()
            .unwrap()
            .parse()
            .expect("Please enter a correct input");
        let x: u64 = inputs
            .next()
            .unwrap()
            .parse()
            .expect("Please enter a correct input");

        coordinates.push((y, x));
    }

    for (y, x) in coordinates {
        println!("{}", calculate(y, x));
    }
}
