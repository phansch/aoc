extern crate csv;

use std::io;

fn main() {
    let rows = rows();
    let mut divisibles: Vec<u32> = vec![];

    for mut row in rows {
        let divisible: u32 = find_divisible_in_row(row);
        println!("divisible: {}", divisible);
        divisibles.push(divisible);
    }
    let sum: u32 = divisibles.iter().sum();
    println!("Checksum is: {}", sum);
}

fn find_divisible_in_row(row: Vec<u32>) -> u32 {
    let mut divisors: Vec<u32> = vec![];

    for num in row.to_vec() {
        for num2 in row.to_vec() {
            if num != num2 && num % num2 == 0 {
                divisors.push(num);
                divisors.push(num2);
                break;
            }
        }
    }
    divisors[0] / divisors[1]
}

fn rows() -> Vec<Vec<u32>> {
    let mut rows: Vec<Vec<u32>> = vec![];
    let mut rdr = csv::ReaderBuilder::new()
        .has_headers(false)
        .delimiter(b'\t')
        .from_reader(io::stdin());

    for result in rdr.deserialize() {
        // let record: Vec<u32> = result?;
        // rows.push(record);
        let record: Vec<u32> = result.unwrap();
        rows.push(record.to_vec());
        println!("{:?}", &record);
    }
    rows
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(find_divisible_in_row(vec![5, 9, 2, 8]), 4);
        assert_eq!(find_divisible_in_row(vec![9, 4, 7, 3]), 3);
        assert_eq!(find_divisible_in_row(vec![3, 8, 6, 5]), 2);
    }
}
