use std::io;

fn main() {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Unable to take any input");
    let length: u64 = input.trim().parse().expect("Please enter a vlaid input");

    for c in 1..=length {
        let tot_square = c.pow(2);
        let mut possiblities = tot_square * (tot_square - 1) / 2;

        if c > 2 {
            possiblities -= 4 * (c - 1) * (c - 2);
        }
        println!("{}", possiblities);
    }
}
