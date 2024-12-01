use csv::Reader;
use std::{collections::HashMap, error::Error, fs::File};

fn diff(a: i32, b: i32) -> Result<i32, &'static str> {
    Ok(if a > b {
        a - b
    } else if b > a {
        b - a
    } else {
        0
    })
}

#[allow(dead_code)]
fn day_one() -> Result<i32, Box<dyn Error>> {
    let file = File::open("input.csv")?;

    let mut reader = Reader::from_reader(file);
    let mut left = Vec::new();
    let mut right = Vec::new();

    for result in reader.records() {
        let record = result?;

        let left_num = record[0].parse::<i32>()?;
        left.push(left_num);
        let right_num = record[1].parse::<i32>()?;

        right.push(right_num);
    }

    left.sort();
    right.sort();

    let total_diff: i32 = left
        .iter()
        .zip(right.iter())
        .map(|(l, r)| diff(*l, *r).unwrap_or(0))
        .sum();

    println!("Total diff: {}", total_diff);
    Ok(total_diff)
}

fn main() -> Result<(), Box<dyn Error>> {
    let file = File::open("input.csv")?;

    let mut reader = Reader::from_reader(file);
    let mut left = Vec::new();
    let mut right = Vec::new();

    for result in reader.records() {
        let record = result?;

        let left_num = record[0].parse::<i32>()?;
        left.push(left_num);
        let right_num = record[1].parse::<i32>()?;

        right.push(right_num);
    }

    left.sort();
    right.sort();

    let total_diff = day_one();
    println!("{}", total_diff?);

    let mut right_map: HashMap<i32, i32> = HashMap::new();
    for &num in right.iter() {
        *right_map.entry(num).or_insert(0) += 1;
    }

    let total_similarity: i32 = left
        .iter()
        .map(|num| num * right_map.get(num).unwrap_or(&0))
        .sum();

    println!("Total sim score: {}", total_similarity);

    Ok(())
}
