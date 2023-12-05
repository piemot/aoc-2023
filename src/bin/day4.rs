use std::collections::HashMap;
use std::fs::read_to_string;

#[allow(dead_code)]
fn first() -> Result<(), String> {
    let contents = read_to_string("./src/inputs/day04.txt").expect("File could not be opened");

    let mut matches = vec![];
    for line in contents.split('\n') {
        let line = line.split(':').skip(1).next().expect("Body should exist");
        let mut components = line.split('|');

        let mut matching = 0;

        let winning_numbers: Vec<i32> = components
            .next()
            .expect("Winning component should exist")
            .split(' ')
            .filter(|x| x != &"")
            .map(|x| i32::from_str_radix(x, 10).expect("Failed to parse int"))
            .collect();

        for held_number in components
            .next()
            .expect("Held component should exist")
            .split(' ')
            .filter(|x| x != &"")
            .map(|x| i32::from_str_radix(x, 10).expect("Failed to parse int"))
        {
            if winning_numbers.contains(&held_number) {
                matching += 1;
            }
        }
        matches.push(matching);
    }

    println!("Matches: {:?}", matches);

    println!(
        "Final result: {}",
        matches
            .iter()
            .filter(|m| m > &&0)
            .map(|m| 2_i32.pow(*m - 1))
            .sum::<i32>()
    );

    Ok(())
}

#[allow(dead_code)]
fn second() -> Result<(), String> {
    let contents = read_to_string("./src/inputs/day04.txt").expect("File could not be opened");

    let mut cards: HashMap<usize, Vec<usize>> = HashMap::new();

    for (id, line) in contents.split('\n').enumerate() {
        let line = line.split(':').skip(1).next().expect("Body should exist");
        let mut components = line.split('|');

        let winning: Vec<i32> = components
            .next()
            .expect("Winning component should exist")
            .split(' ')
            .filter(|x| x != &"")
            .map(|x| i32::from_str_radix(x, 10).expect("Failed to parse int"))
            .collect();

        let mut pointer_id = id + 1;
        let mut new: Vec<usize> = vec![];
        for held_number in components
            .next()
            .expect("Held component should exist")
            .split(' ')
            .filter(|x| x != &"")
            .map(|x| i32::from_str_radix(x, 10).expect("Failed to parse int"))
        {
            if winning.contains(&held_number) {
                pointer_id += 1;
                new.push(pointer_id);
            }
        }
        cards.insert(id + 1, new);
    }

    let mut card_count: HashMap<usize, usize> = HashMap::new();

    for id in 1..cards.len() + 1 {
        let card = cards.get(&id).unwrap();
        let found_qty = card_count.get(&id);

        let qty: usize = if found_qty.is_none() {
            card_count.insert(id, 1);
            1
        } else {
            *found_qty.unwrap()
        };

        for next_card in card {
            let exist = card_count.get(&next_card).unwrap_or(&1);
            let total = qty + exist;
            card_count.insert(*next_card, total);
        }
        println!("{:?}", card_count);
    }

    let total_cards: usize = card_count.iter().map(|i| i.1).sum();

    println!("Total: {}", total_cards);

    Ok(())
}

fn main() -> Result<(), String> {
    second()
}
