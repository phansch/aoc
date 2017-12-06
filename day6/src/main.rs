use std::io::{self, Read};

fn main() {
    part1();
}

fn part1() {
    let input = parse();
    let (arr, _last_state) = redistribute(input.to_vec());
    let solution = arr.clone().len() + 1;
    println!("1: {}", solution);
}

fn part2() {
    // let input = parse();
    // let (arr, last_state) = redistribute(input);
    // let first_index: usize = arr.iter().position(|e| e == &last_state).unwrap();
    // let last_index: usize = arr.len() - 1;
    // let solution = last_index - first_index;
    // println!("{}", solution);
}

fn parse() -> Vec<i32> {
    let data = read();
    let splitted: Vec<String> = data.split('\t').map(|e| String::from(e.trim())).collect();
    println!("{:?}", splitted);
    data.split('\t').map(|e| e.trim().parse::<i32>().unwrap()).collect()
}

fn redistribute_once(mut input: Vec<i32>) -> Vec<i32> {
    let mut value_to_redistribute: i32 = input.iter().max().unwrap().clone();
    let largest_index = input.iter().position(|&e| e == value_to_redistribute).unwrap();

    // The index to be modified next
    let mut index = (largest_index + 1) % input.len();

    input[largest_index] = 0;

    loop {
        input[index] += 1;
        value_to_redistribute -= 1;

        index = (index + 1) % input.len();
        if value_to_redistribute == 0 {
            break;
        }
    }

    input
}

fn redistribute(input: Vec<i32>) -> (Vec<Vec<i32>>, Vec<i32>) {
    let mut states: Vec<Vec<i32>> = vec![];

    loop {
        let last_state: Vec<i32> = if states.len() > 0 {
            states.last().unwrap().to_vec()
        } else {
            input.to_vec()
        };
        let redistributed = redistribute_once(last_state.to_vec());

        if states.contains(&redistributed) {
            return (states, last_state)
        }
        states.push(redistributed);
    }
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
    fn test_redistribute_one() {
        assert_eq!(redistribute_once(vec![0, 2, 7, 0]), vec![2, 4, 1, 2]);
        assert_eq!(redistribute_once(vec![2, 4, 1, 2]), vec![3, 1, 2, 3]);
        assert_eq!(redistribute_once(vec![3, 1, 2, 3]), vec![0, 2, 3, 4]);
        assert_eq!(redistribute_once(vec![0, 2, 3, 4]), vec![1, 3, 4, 1]);
        assert_eq!(redistribute_once(vec![1, 3, 4, 1]), vec![2, 4, 1, 2]);
    }


    #[test]
    fn test_solution_1() {
        assert_eq!(redistribute_to_loop(vec![0, 2, 7, 0]), 5);
    }
}
