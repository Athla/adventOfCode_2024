use anyhow::Result;
use csv::Reader;
use std::{error::Error, fs::File};

fn diff(a: i32, b: i32) -> Result<i32, &'static str> {
    Ok(if a > b {
        a - b
    } else if b > a {
        b - a
    } else {
        0
    })
}

fn main() -> Result<(), Box<dyn Error>> {
    // Open file
    // two arrs
    // sort it
    //
    let file = File::open("input.csv")?;

    let mut reader = Reader::from_reader(file);
    let mut left = Vec::new();
    let mut right = Vec::new();

    for result in reader.records() {
        let record = result?;

        let left_num = record[0].parse::<i32>()?;
        left.push(left_num);
        let right_num = record[1].parse::<i32>()?;
        println!("{:?} -  {:?}", left_num, right_num);

        right.push(right_num);
    }

    left.sort();
    right.sort();

    let mut total_diff: i32 = 0;

    for (i, _) in left.iter().enumerate() {
        total_diff += diff(left[i], right[i]).unwrap();
    }

    println!("Total diff: {}", total_diff);
    Ok(())
}
