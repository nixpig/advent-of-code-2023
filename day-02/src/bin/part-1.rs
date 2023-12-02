use std::fs;

const PATH: &str = "/home/nixpig/projects/advent-of-code-2023/day-02/src/example.txt";

#[derive(Debug)]
struct Game {
    id: String,
    blue: usize,
    red: usize,
    green: usize,
}

#[derive(Debug)]
struct Collection {
    colour: String,
    count: usize,
}

fn main() {
    let input = fs::read_to_string(PATH).unwrap();

    for line in input.lines() {
        let parts = line.split(": ").collect::<Vec<_>>();

        let id = parts[0].split(" ").collect::<Vec<_>>()[1];

        let reveals = parts[1].split("; ").map(|x| {
            let colour
        });

        println!("{:?}", reveals);

        let game = Game {
            id: String::from(id),
            blue: 0,
            red: 0,
            green: 0,
        };

        println!("{:?}", game);
    }

    println!("{:?}", input);
}
