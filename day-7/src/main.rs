use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::error::Error;
use regex::Regex;

#[macro_use]
extern crate simple_error;

#[derive(Debug)]
struct Rule {
    colour: String,
    rules: Vec<(Rule, i32)>
}

impl Rule {
    fn generate(_rules: &String) -> Vec<(Rule, i32)> {
        let mut rules: Vec<(Rule, i32)> = Vec::new();

        for rule in _rules.split(", ") {
            let s = &rule.split(" bag").collect::<Vec<&str>>()[0];

            let num: i32 = String::from(s.chars().nth(0).unwrap()).parse::<i32>().unwrap();
            let colour = s.split(char::is_numeric).collect::<Vec<&str>>()[1].trim_start();

            rules.push(
                (
                    Rule {
                        colour: String::from(colour),
                        rules: Vec::new()
                    },
                    num
                )
            )
        }

        rules
    }
}

fn parse_line(l : &String) -> Result<Rule, Box<Error>> {
    let parent_bag_re = Regex::new(r"([a-z ]+) bags contain ([a-z0-9, ]+).").unwrap();
    let caps = parent_bag_re.captures(l);
    match caps {
        Some(_caps)  => {
            Ok(Rule {
                colour: String::from(&_caps[1]),
                rules:  {
                    if &_caps[2] == "no other bags" {
                        Vec::new()
                    } else {
                        Rule::generate(&String::from(&_caps[2]))
                    }
                }
            })
        },
        _           => bail!("Found no captures in line!")
    }
}

fn main() {
    if let Ok(lines) = read_lines("src/input") {
        for line in lines {
            if let Ok(l) = line {
                println!("{:?}", parse_line(&l).unwrap())
                //parse_line(&l).unwrap();
            }
        }
    }
}

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}