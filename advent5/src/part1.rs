use std::fs;

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();
    let seeds: Vec<usize> = input
        .lines()
        .next()
        .unwrap()
        .split(':')
        .nth(1)
        .unwrap()
        .trim()
        .split(' ')
        .map(|number| number.parse().unwrap())
        .collect();

    let lines: Vec<&str> = input.lines().into_iter().collect();
    let tails = &lines[2..];

    let vecs: Vec<Vec<Vec<usize>>> =
        tails
            .iter()
            .filter(|s| !s.is_empty())
            .fold(vec![], |mut v, &s| {
                if s.contains(':') {
                    v.push(vec![]);
                } else {
                    v.last_mut()
                        .unwrap()
                        .push(s.split(' ').map(|n| n.parse().unwrap()).collect());
                }
                v
            });

    let destinations: usize = seeds
        .iter()
        .map(|seed| {
            vecs.iter().fold(*seed, |acc, vec| {
                let op_vec = vec.iter().find(|v| acc >= v[1] && acc < v[1] + v[2]);
                match op_vec {
                    None => acc,
                    Some(v) => v[0] + (acc - v[1]),
                }
            })
        })
        .min()
        .unwrap();

    println!("{:?}", destinations);
}
