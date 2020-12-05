use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
const MAX_POS: usize = 30;

fn parse_line(l: &String) -> Vec<i8> {
    let mut v: Vec<i8> = Vec::new();

    for c in l.chars() {
        if c == '.' {
            v.push(0);
        } else {
            v.push(1);
        }
    }

    v
}

fn main() {
    let slope_arr = [[1,1], [3,1], [5,1], [7,1], [1,2]];
    let mut trees_product: i64 = 1;

    for slope in slope_arr.iter() {
        let mut x_pos = 0;
        let mut trees = 0;

        if let Ok(lines) = read_lines("src/input") {
            for (i, line) in lines.enumerate() {
                if let Ok(l) = line {
                    if i % slope[1] == 0 {  
                        if parse_line(&l)[x_pos] == 1 {
                            trees += 1;
                        }
        
                        if x_pos + slope[0] <= MAX_POS {
                            x_pos += slope[0];
                        } else {
                            x_pos = x_pos + slope[0] - MAX_POS - 1;
                        }
                    }
                }
            }

            trees_product *= trees;
        }
    }

    println!("Trees product is: {}", trees_product);
}

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}