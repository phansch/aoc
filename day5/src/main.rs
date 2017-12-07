use std::io::Read;

fn main() {
    part1()
}

fn part1() {
    let instructions = parse();
    let count = calculate_steps(instructions);
    println!("Solution: {}", count);
}

fn calculate_steps(mut instructions: Vec<i32>) -> i32 {
    let mut step_count: i32 = 0;
    let mut index: i32 = 0;

    // https://doc.rust-lang.org/std/vec/struct.Vec.html#method.get_mut
    while let Some(i) = instructions.get_mut(index as usize) {
        index += *i;
        // *i += 1; // part 1

        // Part 2
        if *i >= 3 {
            *i -= 1;
        } else {
            *i += 1;
        }
        step_count += 1;

    }

    step_count
}

fn parse() -> Vec<i32> {
    let string = read();
    string.split('\n').filter(|c| c.len() > 0).map(|c| c.parse::<i32>().unwrap()).collect()

}

fn read() -> String {
    let mut stdin = std::io::stdin();
    let mut buffer = String::new();
    stdin.read_to_string(&mut buffer).expect("uh oh");
    buffer
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(calculate_steps(vec![0, 3, 0, 1, -3]), 10);
    }
}
