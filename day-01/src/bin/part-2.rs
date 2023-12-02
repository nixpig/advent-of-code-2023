use std::fs;

const PATH: &str = "/home/nixpig/projects/advent-of-code-2023/day-01/input.txt";

fn main() {
    let str_nums = [
        "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    ];

    let input = fs::read_to_string(PATH).unwrap();

    let mut calibration_values: Vec<usize> = vec![];

    for line in input.lines() {
        let mut cal = String::new();

        let mut val1: usize = 0;
        let mut pos1: usize = 999;

        let mut val2: usize = 0;
        let mut pos2: usize = 999;

        for (i, n) in str_nums.iter().enumerate() {
            let position = line.find(n);

            if let Some(p) = position {
                if p < pos1 {
                    pos1 = p;
                    val1 = i + 1;
                }
            }
        }

        for (i, n) in (1..=9).enumerate() {
            let position = line.find(&n.to_string());
            if let Some(p) = position {
                if p < pos1 {
                    pos1 = p;
                    val1 = i + 1;
                }
            }
        }

        for (i, n) in str_nums.iter().enumerate() {
            let position = line
                .chars()
                .rev()
                .collect::<String>()
                .find(&n.chars().rev().collect::<String>());

            if let Some(p) = position {
                if p < pos2 {
                    pos2 = p;
                    val2 = i + 1;
                }
            }
        }

        for (i, n) in (1..=9).enumerate() {
            let position = line.chars().rev().collect::<String>().find(&n.to_string());

            if let Some(p) = position {
                if p < pos2 {
                    pos2 = p;
                    val2 = i + 1;
                }
            }
        }

        cal.push_str(&val1.to_string());
        cal.push_str(&val2.to_string());

        let cali = str::parse::<usize>(&cal);

        if let Ok(c) = cali {
            calibration_values.push(c);
        }
    }

    let total = calibration_values.into_iter().reduce(|a, c| a + c).unwrap();

    println!("{:?}", total);
}
