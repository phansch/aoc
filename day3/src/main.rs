use std::f32;

fn main() {
    part1();
}

fn part1() {
    let position = calculate_position(361527.0);
    let solution = calculate_distance(position, (0.0, 0.0));
    println!("{}", solution);
}

fn part2() {
    let solution = 0;
    println!("{}", solution);
}

fn calculate_distance(p: (f32, f32), q: (f32, f32)) -> i32 {
    (p.0.abs() - q.0.abs() + p.1.abs() - q.1.abs()) as i32
}

fn calculate_position(number: f32) -> (f32, f32) {
    let radius = ((number.sqrt() - 1.0) / 2.0).ceil();

    let mut t = 2.0 * radius + 1.0;
    let mut m = (t as i32).pow(2) as f32;

    t -= 1.0;

    if number >= m - t {
        return (radius - (m - number), -radius)
    }

    m -= t;

    if number >= m - t {
        return (-radius, -radius + (m - number));
    }

    m -= t;

    if number >= m - t {
        return (-radius + (m - number), radius);
    }

    (radius, radius - (m - number - t))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_calculates_the_position_of_n() {
        assert_eq!(calculate_position(5.0), (-1.0, 1.0));
        assert_eq!(calculate_position(17.0), (-2.0, 2.0));
        assert_eq!(calculate_position(31.0), (3.0, 3.0));
        assert_eq!(calculate_position(361527.0), (301.0, 25.0));
    }

    #[test]
    fn it_calculates_the_distance_of_two_points() {
        assert_eq!(calculate_distance((2.0, 2.0), (0.0, 0.0)), 4);
        assert_eq!(calculate_distance((2.0, 0.0), (0.0, 0.0)), 2);
        assert_eq!(calculate_distance((2.0, -1.0), (0.0, 0.0)), 3);
    }
}
