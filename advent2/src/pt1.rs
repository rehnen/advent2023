use std::fs;

#[derive(Debug)]
struct Cubes {
    red: usize,
    green: usize,
    blue: usize,
}

fn main() {
    let input = fs::read_to_string("src/data.txt").unwrap();
    let nr_red_cubes = 12;
    let nr_green_cubes = 13;
    let nr_blue_cubes = 14;

    let sum: usize = input
        .lines()
        .map(|line| {
            let game = line.split(':').next().unwrap();
            let sets = line.split(':').nth(1).unwrap().split(';');
            let mut cubes = Cubes {
                red: 0,
                green: 0,
                blue: 0,
            };
            for set in sets {
                for ele in set.split(',') {
                    let mut number_and_color = ele.trim().split(' ');
                    let number: usize = number_and_color.next().unwrap().parse().unwrap();
                    let color = number_and_color.next().unwrap();

                    match color {
                        "red" => {
                            if number > nr_red_cubes {
                                return 0;
                            }
                        }
                        "green" => {
                            if number > nr_green_cubes {
                                return 0;
                            }
                        }
                        "blue" => {
                            if number > nr_blue_cubes {
                                return 0;
                            }
                        }
                        _ => panic!("parsing logic is way off"),
                    }
                }
            }

            println!("{} {:?}", game, cubes);
            return game.split(' ').nth(1).unwrap().parse().unwrap();
        })
        .sum();
    println!("{}", sum);
    println!("Hello, world!");
}
