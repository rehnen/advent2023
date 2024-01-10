use regex::Regex;
use std::fs;

fn main() {
    let input = fs::read_to_string("data.txt").unwrap();
    let games: Vec<usize> = input
        .lines()
        .map(|line| {
            let mut splitted = line.split(':').nth(1).unwrap().split('|');

            let reg = Regex::new(r"(\d)+").unwrap();
            let winning_numbers: Vec<usize> = reg
                .find_iter(splitted.next().unwrap())
                .map(|nr| nr.as_str().parse().unwrap())
                .collect();
            let my_numbers: Vec<usize> = reg
                .find_iter(splitted.next().unwrap())
                .map(|nr| nr.as_str().parse().unwrap())
                .filter(|nr| winning_numbers.contains(nr))
                .collect();

            if my_numbers.len() < 2 {
                return my_numbers.len();
            }
            return (1..my_numbers.len()).fold(1, |item, _| item * 2);
        })
        .collect();
    println!("{:?}", games.into_iter().sum::<usize>());

    println!("Hello, world!");
}
