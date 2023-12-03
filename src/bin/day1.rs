use std::fs::File;
use std::io::{BufRead, BufReader};

#[allow(dead_code)]
fn first() -> std::io::Result<()> {
    let file = File::open("./src/inputs/day01.txt")?;
    let reader = BufReader::new(file);
    let mut lines = reader.lines();

    let mut values: Vec<i32> = vec![];

    while let Some(Ok(line)) = lines.next() {
        // Store all numeric values in the line
        let mut items = vec![];

        for char in line.chars() {
            if char.is_numeric() {
                items.push(char);
            }
        }

        let mut res = String::new();
        // Get first and last stored values
        res.push(*items.get(0).expect("There must be a first attr"));
        res.push(*items.last().expect("There must be a last attr"));

        values
            .push(i32::from_str_radix(res.as_str(), 10).expect("Could not convert result to int"));
    }

    println!("Final result: {}", values.iter().sum::<i32>());
    Ok(())
}

#[allow(dead_code)]
fn second() -> std::io::Result<()> {
    let keys: Vec<&str> = vec![
        "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    ];

    let file = File::open("./src/inputs/day01.txt")?;
    let reader = BufReader::new(file);
    let mut lines = reader.lines();

    let mut values: Vec<i32> = vec![];

    while let Some(Ok(line)) = lines.next() {
        let slice: Vec<char> = line.chars().collect();

        // Store all numeric values in the line
        let mut items = vec![];

        for (ind, char) in slice.iter().enumerate() {
            if char.is_digit(10) {
                items.push(char.to_string());
            }

            for (num, key) in keys.iter().enumerate() {
                // Iterate over our keys ("one", "two", "three").
                // Select a range from ind..ind+key.len()
                // from `slice` for comparison
                let check: String = slice
                    .clone()
                    .into_iter()
                    .skip(ind)
                    .take(key.len())
                    .collect();

                if key == &&check {
                    // num refers to the item's index, so must be incremented to make it 1-indexed.
                    items.push((num + 1).to_string())
                }
            }
        }

        let mut res = String::new();
        // Get first and last stored values
        res.push_str(&items.get(0).expect("There must be a first attr"));
        res.push_str(&items.last().expect("There must be a last attr"));

        values
            .push(i32::from_str_radix(res.as_str(), 10).expect("Could not convert result to int"));
    }

    println!("Final result: {}", values.iter().sum::<i32>());
    Ok(())
}

fn main() -> std::io::Result<()> {
    first()
}
