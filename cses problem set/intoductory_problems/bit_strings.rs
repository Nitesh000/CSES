use std::io;

fn main() {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Unable to take inputs");
    let input = input.trim().parse().expect("Please enter a valid integet");
    let mut ans: u64 = 1;

    for _ in 0..input {
        ans = (ans * 2) % 1000000007;
    }
    println!("{}", ans);
}
