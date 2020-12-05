use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use regex::Regex;

#[derive(Debug)]
struct Password {
    range_start: usize,
    range_end: usize,
    character: char,
    string: String
}

fn validate_pass(pass: &Password) -> bool {
    let mut first = false;
    let mut second = false;

    for (i, c) in pass.string.chars().enumerate() {
        if i == pass.range_start - 1 && c == pass.character {
            first = true;
        }
        else if i == pass.range_end - 1 && c == pass.character {
            second = true;
        }
    }

    (first || second) && !(first && second)
}

fn parse_pass(pass: &String) -> Password {
    let re = Regex::new(r"(\d+)-(\d+) ([a-z]): ([a-z]+)").unwrap();
    let caps = re.captures(pass).unwrap();
    Password {
        range_start: caps.get(1).unwrap().as_str().parse::<usize>().unwrap(),
        range_end: caps.get(2).unwrap().as_str().parse::<usize>().unwrap(),
        character: caps.get(3).unwrap().as_str().parse::<char>().unwrap(),
        string: caps.get(4).unwrap().as_str().parse::<String>().unwrap(),
    }
}

fn main() {
    let mut valid = 0;

    // File hosts must exist in current path before this produces output
    if let Ok(lines) = read_lines("src/input") {
        // Consumes the iterator, returns an (Optional) String
        for line in lines {
            if let Ok(pass) = line {
                if validate_pass(&parse_pass(&pass)) {
                    valid += 1;
                }
            }
        }
    }

    println!("Number of valid passwords: {}", valid);
}

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}