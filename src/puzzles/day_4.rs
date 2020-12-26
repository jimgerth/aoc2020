use std::fs;

use regex::Regex;

pub fn part_1() {
    let input = fs::read_to_string("src/inputs/day_4.txt").unwrap();
    let mut passports = input.split("\n\n");

    let mut valids = 0;

    while let Some(passport) = passports.next() {
        let fields: Vec<&str> = passport.split_whitespace().collect();

        if fields.len() == 8 {
            valids += 1;
        } else if fields.len() == 7 {
            valids += 1;
            for field in fields.iter() {
                if field.starts_with("cid") {
                    valids -= 1;
                }
            }
        }
    }

    println!("The answer to part 1 is {}", valids);
}

pub fn part_2() {
    let input = fs::read_to_string("src/inputs/day_4.txt").unwrap();
    let mut passports = input.split("\n\n");

    let mut valids = 0;

    while let Some(passport) = passports.next() {
        let fields: Vec<&str> = passport.split_whitespace().collect();

        if check_fields(&fields) {
            valids += 1;
        }
    }

    println!("The answer to part 2 is {}", valids);
}

fn check_fields(fields: &Vec<&str>) -> bool {
    let mut byr = false;
    let mut iyr = false;
    let mut eyr = false;
    let mut hgt = false;
    let mut hcl = false;
    let mut ecl = false;
    let mut pid = false;

    for field in fields.iter() {
        match &field[..3] {
            "byr" => match field[4..].parse::<i32>() {
                Ok(value) => byr = value >= 1920 && value <= 2002,
                _ => {}
            },
            "iyr" => match field[4..].parse::<i32>() {
                Ok(value) => iyr = value >= 2010 && value <= 2020,
                _ => {}
            },
            "eyr" => match field[4..].parse::<i32>() {
                Ok(value) => eyr = value >= 2020 && value <= 2030,
                _ => {}
            },
            "hgt" => match &field[field.len() - 2..] {
                "cm" => match field[4..field.len() - 2].parse::<i32>() {
                    Ok(value) => hgt = value >= 150 && value <= 193,
                    _ => {}
                },
                "in" => match field[4..field.len() - 2].parse::<i32>() {
                    Ok(value) => hgt = value >= 59 && value <= 76,
                    _ => {}
                },
                _ => {}
            },
            "hcl" => {
                hcl = Regex::new(r"^#[0-9 a-f]{6}$")
                    .unwrap()
                    .is_match(&field[4..])
            }
            "ecl" => match &field[4..] {
                "amb" => ecl = true,
                "blu" => ecl = true,
                "brn" => ecl = true,
                "gry" => ecl = true,
                "grn" => ecl = true,
                "hzl" => ecl = true,
                "oth" => ecl = true,
                _ => {}
            },
            "pid" => pid = Regex::new(r"^[0-9]{9}$").unwrap().is_match(&field[4..]),
            _ => {}
        }
    }

    byr && iyr && eyr && hgt && hcl && ecl && pid
}