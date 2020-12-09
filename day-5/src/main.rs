use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

#[derive(Debug)]
struct TwoPoints {
    upper: f32,
    lower: f32,
    chosen: f32
}

fn parse_line(l: &String) -> f32 {
    let mut row: TwoPoints = TwoPoints {
        upper: 127.0,
        lower: 0.0,
        chosen: 0.0
    };
    let mut col: TwoPoints = TwoPoints {
        upper: 7.0,
        lower: 0.0,
        chosen: 0.0
    };

    for (i, c) in l.chars().enumerate() {
        if i == 6 || i == 9 {
            match c {
                'F' => row.chosen = row.lower, // Lower Half Row
                'B' => row.chosen = row.upper, // Upper Half Row
                'R' => col.chosen = col.upper, // Upper Half Col
                'L' => col.chosen = col.lower, // Lower Half Col
                _   => println!("Unrecognised char!")
            }
        } else {
            match c {
                'F' => row.upper = row.upper - ((row.upper - row.lower)/2.0).ceil(), // Lower Half Row
                'B' => row.lower = row.lower + ((row.upper - row.lower)/2.0).ceil(), // Upper Half Row
                'R' => col.lower = col.lower + ((col.upper - col.lower)/2.0).ceil(), // Upper Half Col
                'L' => col.upper = col.upper - ((col.upper - col.lower)/2.0).ceil(), // Lower Half Col
                _   => println!("Unrecognised char!")
            }
        }
    }

    row.chosen * 8.0 + col.chosen
}

fn main() {
    let mut ids: Vec<i32> = Vec::new();

    if let Ok(lines) = read_lines("src/input") {
        for line in lines {
            if let Ok(l) = line {
                ids.push(parse_line(&l) as i32);
            }
        }
    }

    ids.sort();

    for i in 0..ids.len()-1 {
        if ids[i+1] - ids[i] > 1 {
            println!("Missing ID is {}", ids[i] + 1);
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