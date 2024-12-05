use std::fs;

use regex::Regex;

fn part_01(data: String) -> i32 {
    let re = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();

    let mut total = 0;
    for cap in re.captures_iter(&data) {
        let a = cap[1].parse::<i32>().unwrap();
        let b = cap[2].parse::<i32>().unwrap();
        let tmp = a * b;
        total += tmp;
    }
    total
}

fn part_02(data: String) -> i64 {
    let mul_re = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();
    let do_re = Regex::new(r"do\(\)").unwrap();
    let dont_re = Regex::new(r"don't\(\)").unwrap();

    let mut total = 0;
    let mut enabled = true;

    let mut pos = 0;
    while pos < data.len() {
        if let Some(m) = do_re.find_at(&data, pos) {
            if m.start() == pos {
                enabled = true;
                pos = m.end();
                continue;
            }
        }

        if let Some(m) = dont_re.find_at(&data, pos) {
            if m.start() == pos {
                enabled = false;
                pos = m.end();
                continue;
            }
        }

        if let Some(m) = mul_re.find_at(&data, pos) {
            if m.start() == pos {
                if enabled {
                    if let Some(cap) = mul_re.captures(&data[pos..m.end()]) {
                        let a = cap[1].parse::<i64>().unwrap();
                        let b = cap[2].parse::<i64>().unwrap();

                        total += a * b;
                    }
                }
                pos = m.end();
            } else {
                pos += 1;
            }
        } else {
            pos += 1
        }
    }

    total
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let data = fs::read_to_string("./input/day03.txt")?;

    println!("Total found: {}", part_01(data.clone()));
    println!("Total found pt2: {}", part_02(data));
    Ok(())
}
