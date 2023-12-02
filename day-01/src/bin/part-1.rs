use std::fs;

const PATH: &str = "/home/nixpig/projects/advent-of-code-2023/day-01/input.txt";

fn main() {
    let input = fs::read_to_string(PATH).unwrap();

    let mut calibration_values: Vec<usize> = vec![];

    for line in input.lines() {
        let mut cal = String::new();

        for ch in line.chars() {
            if ch.is_ascii_digit() {
                cal.push_str(&ch.to_string());
                break;
            }
        }

        for ch in line.chars().rev() {
            if ch.is_ascii_digit() {
                cal.push_str(&ch.to_string());
                break;
            }
        }

        let cali = str::parse::<usize>(&cal);

        if let Ok(c) = cali {
            calibration_values.push(c);
        }
    }

    let total = calibration_values.into_iter().reduce(|a, c| a + c).unwrap();

    println!("{:?}", total);
}
