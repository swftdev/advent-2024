use std::io::{BufRead, BufReader};
use std::fs::File;
use std::path::Path;
use std::io;

use std::collections::HashMap;

pub fn day1() -> io::Result<()> {
    // Input
    let file_path = Path::new("data").join("1.txt");
    let file = File::open(&file_path)?;
    let reader = BufReader::new(file);

    let part1_result = part1(reader);
    println!("{}", part1_result.unwrap());

    let file_path = Path::new("data").join("1.txt");
    let file = File::open(&file_path)?;
    let reader = BufReader::new(file);

    let part2_result = part2(reader);
    println!("{}", part2_result.unwrap());
    // Part 1
    Ok(())
}


fn part1(reader: BufReader<File>) -> io::Result<i32> {
    let mut first_values: Vec<i32> = Vec::new();
    let mut second_values: Vec<i32> = Vec::new();

    first_values.reserve(1000);
    second_values.reserve(1000);

    for line in reader.lines() {
        let result = line?;
        let parts: Vec<&str> = result.split_whitespace().collect();
        if let [first, second] = parts.as_slice() {
            let first_int = first.parse::<i32>().unwrap();
            let second_int = second.parse::<i32>().unwrap();
            first_values.push(first_int);
            second_values.push(second_int);
        }
    }

    first_values.sort();
    second_values.sort();

    let mut total = 0;
    for (first, second) in first_values.iter().zip(second_values.iter()) {
        let distance =  first - second;
        total += distance.abs()
    }

    Ok(total)
}

fn part2(reader: BufReader<File>) -> io::Result<i32>{
    let mut first_values: Vec<i32> = Vec::new();
    let mut second_values: Vec<i32> = Vec::new();

    first_values.reserve(1000);
    second_values.reserve(1000);

    for line in reader.lines() {
        let result = line?;
        let parts: Vec<&str> = result.split_whitespace().collect();
        if let [first, second] = parts.as_slice() {
            let first_int = first.parse::<i32>().unwrap();
            let second_int = second.parse::<i32>().unwrap();
            first_values.push(first_int);
            second_values.push(second_int);
        }
    }

    let mut map: HashMap<i32, i32> = HashMap::new();
    for value in second_values {
        *map.entry(value).or_insert(0) += 1;
    }

    let score: i32 = first_values.iter()
        .map(|&num| {
            let count = map.get(&num).unwrap_or(&0);
            num * count
        })
        .sum();

    Ok(score)
}


