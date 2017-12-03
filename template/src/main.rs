use std::io::{self, Read};

fn main() {
    let data = read();
}

fn part1() {
    let solution = 0;
    println!("{}", solution);
}

fn part2() {
    let solution = 0;
    println!("{}", solution);
}

fn parse() {
    
}

fn read() -> String {
    let mut stdin = std::io::stdin();
    let mut buffer = String::new();
    stdin.read_to_string(&mut buffer);
    buffer
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
