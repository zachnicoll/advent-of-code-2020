use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use regex::Regex;

#[derive(Debug)]
struct Passport {
    byr: String,
    iyr: String,
    eyr: String,
    hgt: String,
    hcl: String,
    ecl: String,
    pid: String,
    cid: String
}

impl Passport {
    pub fn new() -> Passport {
        Passport {
            byr: String::new(),
            iyr: String::new(),
            eyr: String::new(),
            hgt: String::new(),
            hcl: String::new(),
            ecl: String::new(),
            pid: String::new(),
            cid: String::new()
        }
    }

    fn validate_byr(&self) -> bool {
        let re = Regex::new(r"^[0-9]{4}$").unwrap();
        if re.is_match(&self.byr) {
            let num = *&self.byr.parse::<i16>().unwrap();
            num <= 2002 && 
            num >= 1920
        } else {
            false
        }
    }

    fn validate_iyr(&self) -> bool {
        let re = Regex::new(r"^[0-9]{4}$").unwrap();
        if re.is_match(&self.iyr) {
            let num = *&self.iyr.parse::<i16>().unwrap();
            num <= 2020 && 
            num >= 2010   
        } else {
            false
        }
    }

    fn validate_eyr(&self) -> bool {
        let re = Regex::new(r"^[0-9]{4}$").unwrap();
        if re.is_match(&self.eyr) {
            let num = *&self.eyr.parse::<i16>().unwrap();
            num <= 2030 && 
            num >= 2020
        } else {
            false
        }

    }

    fn validate_hgt(&self) -> bool {
        let re = Regex::new(r"([0-9]+)(cm|in)").unwrap();
        if re.is_match(&self.hgt) {
            let unit = &re.captures(&self.hgt).unwrap()[2];
            let val = *&re.captures(&self.hgt).unwrap()[1].parse::<i16>().unwrap();
            (
                unit == "cm" &&
                val <= 193 &&
                val >= 150
            ) 
            ||
            (
                unit == "in" &&
                val <= 76 &&
                val >= 59
            )
        } else {
            false
        }
    }

    fn validate_hcl(&self) -> bool {
        Regex::new(r"#[0-9a-f]{6}").unwrap().is_match(&self.hcl)
    }

    fn validate_ecl(&self) -> bool {
        Regex::new(r"(amb|blu|brn|gry|grn|hzl|oth)").unwrap().is_match(&self.ecl) 
    }

    fn validate_pid(&self) -> bool {
        Regex::new(r"[0-9]{9}").unwrap().is_match(&self.pid)
    }

    pub fn validate(&self) -> bool {
        self.validate_byr() &&
        self.validate_iyr() &&
        self.validate_eyr() &&
        self.validate_hgt() &&
        self.validate_hcl() &&
        self.validate_ecl() &&
        self.validate_pid()
    }
}

fn parse_line(l: &String) -> Passport{
    let re = Regex::new(r"([a-z]+):([#0-9a-z]+)").unwrap();
    let mut p: Passport = Passport::new();
    
    for cap in re.captures_iter(l) {
        let value = String::from(&cap[2]);

        match &cap[1] {
            "byr" => p.byr = value,
            "iyr" => p.iyr = value,
            "eyr" => p.eyr = value,
            "hgt" => p.hgt = value,
            "hcl" => p.hcl = value,
            "ecl" => p.ecl = value,
            "pid" => p.pid = value,
            "cid" => p.cid = value,
            &_    => println!("Invalid key found")
        }
    }

    println!("{:?}, {}", &p, &p.validate());

    p
}

fn main() {
    let mut raw_passport = String::new();
    let mut valid = 0;
    let mut total = 0;

    if let Ok(lines) = read_lines("src/input") {
        for line in lines {
            if let Ok(l) = line {
                if !l.is_empty() {
                    raw_passport = format!("{} {}", raw_passport, l);
                }
                else {
                    total += 1;
                    if parse_line(&raw_passport).validate() { valid += 1; }
                    raw_passport = String::new();
                }
            }
        }
    }

    println!("Number of valid passports: {} / {}", valid, total);
}

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}