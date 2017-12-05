extern crate itertools;

use std::io::Read;
use itertools::sorted;
use std::iter::FromIterator;

fn main() {
    // part1();
    part2();
}

fn part1() {
    let lines = parse();
    let mut solution = 0;

    for line in lines {
        if !contains_duplicates(line) {
            solution += 1;
        }
    }
    println!("Part 1: {}", solution);
}

fn part2() {
    let lines = parse();
    let mut solution = 0;

    for line in lines {
        if !contains_anagram_duplicates(line.clone()) {
            solution += 1;
        }
    }
    println!("Part 2: {}", solution);
}

fn parse() -> Vec<String> {
    let data = read();
    data.split('\n').map(|e| String::from(e)).collect()
}

fn contains_duplicates(line: String) -> bool {
    let mut one: Vec<&str> = line.split(' ').collect();
    let mut two: Vec<&str> = line.split(' ').collect();
    one.sort();
    two.sort();
    two.dedup();
    // println!("deduped: {:?}; orig: {:?}", splitted, orig);
    one.len() == two.len() && one.len() > 1
}

fn contains_anagram_duplicates(line: String) -> bool {
    // Split line
    let words: Vec<String> = line.split(' ').map(|e|String::from(e)).collect();

    // Split each word in line
    let mut words_sorted: Vec<String> = words.clone().into_iter().map(|w| String::from_iter(sorted(w.chars()))).collect();
    words_sorted.sort();

    for i in 0..(words_sorted.len() - 1) {
        if words_sorted[i] == words_sorted[i + 1] {
            return false
        }
    }
    true
}

fn read() -> String {
    let mut stdin = std::io::stdin();
    let mut buffer = String::new();
    stdin.read_to_string(&mut buffer).expect("Something in stdin went wrong");
    buffer
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(contains_duplicates(String::from("aa bb aa")), false);
        assert_eq!(contains_duplicates(String::from("aa bb cc dd aaa")), true);
        assert_eq!(contains_duplicates(String::from("aa bb cc dd ee")), true);
    }

    #[test]
    fn test_contains_anagram_dups() {
        assert_eq!(contains_anagram_duplicates(String::from("abcde fghij")), false);
        assert_eq!(contains_anagram_duplicates(String::from("abcde xyz ecdab")), true);
        assert_eq!(contains_anagram_duplicates(String::from("a ab abc abd abf abj")), false);
        assert_eq!(contains_anagram_duplicates(String::from("iiii oiii ooii oooi oooo")), false);
        assert_eq!(contains_anagram_duplicates(String::from("oiii ioii iioi iiio")), true);
    }
}
