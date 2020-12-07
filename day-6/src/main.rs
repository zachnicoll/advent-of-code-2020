use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

#[derive(Debug)]
struct CharOccurence (char, i32);

fn parse_line(l: String, chars: &mut Vec<CharOccurence>) {
    for c in l.chars() {
        match chars.binary_search_by_key(&c, |CharOccurence (a,_)| *a) {
            Ok(i) => chars[i].1 += 1,
            Err(i) => chars.insert(i, CharOccurence (c, 1))
        }
    }
}

fn sum_occurences(chars: &Vec<CharOccurence>, size: i32) -> i32 {
    let mut sum = 0;

    for c in chars {
        if c.1 >= size { sum += 1 };
    }

    sum
}

fn main() {
    let mut counts: Vec<i32> = Vec::new();

    if let Ok(lines) = read_lines("src/input") {
        let mut chars: Vec<CharOccurence> = Vec::new();
        let mut group_size = 0;

        for line in lines {
            if let Ok(l) = line {
                if l.is_empty() {
                    counts.push(sum_occurences(&chars, group_size));
                    group_size = 0;
                    chars = Vec::new();
                } else {
                    group_size += 1;
                    parse_line(l, &mut chars);
                }
            }
        }

        // For last line
        counts.push(sum_occurences(&chars, group_size));
    }
    

    println!("Sum of counts is: {}", counts.iter().sum::<i32>())
}

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}