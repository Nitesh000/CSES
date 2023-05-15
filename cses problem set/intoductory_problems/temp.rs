struct Solution {}

impl Solution {
    pub fn roman_to_int(s: String) -> i32 {
        let mutated = s
            .replace("IV", "IIII")
            .replace("IX", "VIIII")
            .replace("XL", "XXXX")
            .replace("XC", "LXXXX")
            .replace("CD", "CCCC")
            .replace("CM", "DCCCC");

        mutated
            .chars()
            .map(|c| match c {
                'I' => 1,
                'V' => 5,
                'X' => 10,
                'L' => 50,
                'C' => 100,
                'D' => 500,
                'M' => 1000,
                _ => 0,
            })
            .sum()
    }
}

fn main() {
    let s = "III".to_string();
    let result = Solution::roman_to_int(s);
    println!("{}", result);
}
