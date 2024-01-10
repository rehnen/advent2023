use regex::Regex;
use std::{collections::HashMap, fs, thread::sleep, time::Duration};

fn main() {
    let input = fs::read_to_string("data.txt").unwrap();
    let lines: Vec<&str> = input.lines().collect();
    let reg = Regex::new(r"(\d)+").unwrap();
    let mut map: HashMap<usize, usize> = lines.iter().enumerate().map(|(i, _)| (i, 1)).collect();
    for (i, line) in lines.iter().enumerate() {
        let mut splitted = line.split(':').nth(1).unwrap().split('|');
        let winning_numbers: Vec<usize> = reg
            .find_iter(splitted.next().unwrap())
            .map(|nr| nr.as_str().parse().unwrap())
            .collect();
        let nr_of_copies: usize = reg
            .find_iter(splitted.next().unwrap())
            .map(|nr| nr.as_str().parse().unwrap())
            .filter(|nr| winning_numbers.contains(nr))
            .count();
        let how_many = *map.get(&i).unwrap();
        if nr_of_copies > 0 {
            let copied_indexes = i + 1..i + nr_of_copies + 1;
            for coppy in copied_indexes {
                map.insert(coppy, map.get(&coppy).unwrap() + how_many);
            }
        }
    }
    println!("{:?}", map.values().sum::<usize>());
}
