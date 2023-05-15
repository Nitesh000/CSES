use std::io;

fn main() {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Unable to take input");

    let range: u64 = input.trim().parse().expect("Please enter a valid input");
    let mut s1: Vec<u64> = Vec::new();
    let mut s2: Vec<u64> = Vec::new();
    if (range * (range + 1) / 2) % 2 != 0 {
        println!("NO");
        return;
    } else {
        println!("YES");
        if range % 2 == 0 {
            for i in 0..range / 2 {
                if i % 2 == 0 {
                    s1.push(i + 1);
                    s1.push(range - i);
                } else {
                    s2.push(i + 1);
                    s2.push(range - i);
                }
            }
        } else {
            for i in 0..(range - 1) / 2 {
                if i % 2 == 0 {
                    s1.push(i + 1);
                    s1.push(range - i - 1);
                } else {
                    s2.push(i + 1);
                    s2.push(range - i - 1);
                }
            }

            if s1.len() > s2.len() {
                s2.push(range);
            } else {
                s1.push(range);
            }
        }
    }
    println!("{}", s1.len());
    for n in s1.iter() {
        print!("{} ", n);
    }
    println!();
    println!("{}", s2.len());
    for n in s2.iter() {
        print!("{} ", n);
    }
}
