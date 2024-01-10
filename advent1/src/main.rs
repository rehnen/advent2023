use fancy_regex::Regex;
use std::fs;

/*
*
*
*
*    let total: usize = file_content
       .lines()
       .map(|line| {
           let first_number: usize = line
               .chars()
               .into_iter()
               .find(|c| c.is_numeric())
               .unwrap()
               .to_string()
               .parse()
               .unwrap();

           let last_number: usize = line
               .chars()
               .rev()
               .find(|c| c.is_numeric())
               .unwrap()
               .to_string()
               .parse()
               .unwrap();
           let s: String = format!("{}{}", first_number, last_number);
           let num: usize = s.parse().unwrap();
           println!("{}", num);
           num
       })
       .sum();
*/
fn main() {
    let file_content = fs::read_to_string("./src/data.txt").unwrap();
    let total: usize = file_content
        .lines()
        .map(|line| {
            //    Regex::new(r"(?=(one|two|three|four|five|six|seven|eight|nine|[1-9]))").unwrap();
            let first_number: usize = line
                .replace("one", "o1ne")
                .replace("two", "t2wo")
                .replace("three", "thr3ee")
                .replace("four", "fo4ur")
                .replace("five", "fi5ve")
                .replace("six", "si6x")
                .replace("seven", "se7ven")
                .replace("eight", "eig8ht")
                .replace("nine", "ni9ne")
                .chars()
                .into_iter()
                .find(|c| c.is_numeric())
                .unwrap()
                .to_string()
                .parse()
                .unwrap();

            let last_number: usize = line
                .replace("one", "o1ne")
                .replace("two", "t2wo")
                .replace("three", "thr3ee")
                .replace("four", "fo4ur")
                .replace("five", "fi5ve")
                .replace("six", "si6x")
                .replace("seven", "se7ven")
                .replace("eight", "eig8ht")
                .replace("nine", "ni9ne")
                .chars()
                .rev()
                .find(|c| c.is_numeric())
                .unwrap()
                .to_string()
                .parse()
                .unwrap();
            let s: String = format!("{}{}", first_number, last_number);
            let num: usize = s.parse().unwrap();
            println!("{}", num);
            num
        })
        .sum();

    println!("{}", total);
}
