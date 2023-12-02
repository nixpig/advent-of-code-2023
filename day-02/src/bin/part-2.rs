use std::{env, error::Error, fs};

const PATH: &str = "/Users/jaward/projects/advent-of-code-2023/day-02/src/input.txt";

#[derive(Debug)]
struct Game {
    id: usize,
    blue: usize,
    red: usize,
    green: usize,
}

fn main() -> Result<(), Box<dyn Error>> {
    let input = fs::read_to_string(PATH).unwrap();

    let mut games: Vec<Game> = vec![];

    for line in input.lines() {
        let parts = line.split(": ").collect::<Vec<_>>();

        let id = parts[0].split(" ").collect::<Vec<_>>()[1];

        let mut game = Game {
            id: id.parse::<usize>().unwrap(),
            blue: 0,
            red: 0,
            green: 0,
        };

        parts[1].split("; ").for_each(|x| {
            for y in x.split(", ") {
                let z = y.split(" ").collect::<Vec<_>>();

                let colour = z[1];
                let count = z[0].parse::<usize>().unwrap();

                match colour {
                    "blue" => {
                        if count > game.blue {
                            game.blue = count;
                        }
                    }
                    "green" => {
                        if count > game.green {
                            game.green = count;
                        }
                    }

                    "red" => {
                        if count > game.red {
                            game.red = count;
                        }
                    }
                    _ => {}
                }
            }
        });

        games.push(game);
    }

    let total: usize = games
        .iter()
        // .filter(|x| x.red <= 12 && x.green <= 13 && x.blue <= 14)
        .map(|x| x.green * x.blue * x.red)
        .sum();

    println!("{:?}", total);

    Ok(())
}
