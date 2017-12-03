use std::io::{self, Read};

// curl https://adventofcode.com/2017/day/3/input > ~/code/rust/advent3/input

fn main() {
    let data = read();
}

fn read() -> String {
    let mut stdin = std::io::stdin();
    let mut buffer = String::new();
    stdin.read_to_string(&mut buffer);
    buffer
}

fn parse() {
    
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        // assert_eq!(find_divisible_in_row(vec![5, 9, 2, 8]), 4);
        // assert_eq!(find_divisible_in_row(vec![9, 4, 7, 3]), 3);
        // assert_eq!(find_divisible_in_row(vec![3, 8, 6, 5]), 2);
    }
}
