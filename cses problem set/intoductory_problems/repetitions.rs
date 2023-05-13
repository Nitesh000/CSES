use std::io;

fn main() {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Please enter a valid string");
    let input = input.trim();

    let mut ans = 1;
    let mut temp: char = '\0';
    let mut temp_len = 0;
    for c in input.chars() {
        if c == temp {
            temp_len += 1;
            ans = temp_len.max(ans);
        } else {
            temp = c;
            temp_len = 1;
        }
    }
    println!("{}", ans);
}
