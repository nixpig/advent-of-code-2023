use std::fs;

fn main() {
    let input = "/Users/jaward/projects/advent-of-code-2023/day-01/input.txt";

    let input = fs::read_to_string(input).unwrap();

    let total = input
        .lines()
        .map(|x| {
            let tens =
                str::parse::<usize>(&x.chars().find(|c| c.is_ascii_digit()).unwrap().to_string())
                    .unwrap();
            let units = str::parse::<usize>(
                &x.chars()
                    .rev()
                    .find(|c| c.is_ascii_digit())
                    .unwrap()
                    .to_string(),
            )
            .unwrap();

            tens * 10 + units
        })
        .reduce(|a, c| a + c)
        .unwrap();

    println!("{total}");
}
