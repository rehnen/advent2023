use regex::Regex;
use std::fs;

#[derive(Debug)]
struct Number {
    start: i64,
    end: i64,
    value: i64,
    line_number: i64,
}

impl Number {
    fn intesects(&self, line_position: i64, line_number: i64) -> bool {
        if line_number < self.line_number - 1 || line_number > self.line_number + 1 {
            return false;
        }
        if line_position >= self.start - 1 && line_position <= self.end {
            return true;
        }
        false
    }
}

#[derive(Debug, Clone)]
struct Character {
    position: i64,
    line_number: i64,
    value: char,
}

fn main() {
    println!("Hello, world!");
    let reg = Regex::new(r"(\d)+").unwrap();
    let data = fs::read_to_string("src/data.txt").unwrap();
    let lines: Vec<&str> = data.lines().collect();
    let mut chars: Vec<Character> = vec![];
    let mut numbers: Vec<Number> = vec![];

    for (i, line) in lines.iter().enumerate() {
        for ele in reg.find_iter(line) {
            numbers.push(Number {
                start: ele.start() as i64,
                end: ele.end() as i64,
                value: ele.as_str().parse::<i64>().unwrap(),
                line_number: i as i64,
            });
        }
    }
    for (i, line) in lines.iter().enumerate() {
        for (j, character) in line.chars().enumerate() {
            if !character.is_ascii_alphanumeric() && !character.eq(&'.') {
                chars.push(Character {
                    position: j as i64,
                    line_number: i as i64,
                    value: character,
                });
            }
        }

        println!("{}", line);
    }

    let sum: i64 = numbers
        .into_iter()
        .filter(|num| {
            println!("{:?}", num);
            chars
                .clone()
                .into_iter()
                .find(|ch| num.intesects(ch.position, ch.line_number))
                .is_some()
        })
        .map(|num| num.value)
        .sum();

    println!("{}", sum)
}
