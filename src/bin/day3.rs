use std::collections::HashSet;
use std::fs::read_to_string;

fn is_special_char(ch: &char) -> bool {
    !ch.is_numeric() && ch != &'.'
}

fn create_maps(id: usize) -> Vec<usize> {
    if id < 1 {
        return vec![id, id + 1];
    } else {
        return vec![id - 1, id, id + 1];
    }
}

#[derive(Debug, PartialEq, Eq, Hash)]
struct Location {
    x: Vec<usize>,
    y: usize,
    value: i32,
}

#[allow(dead_code)]
fn first() -> Result<(), String> {
    let contents = read_to_string("./src/inputs/day03.txt").expect("File could not be opened");

    let mut number_addresses: Vec<Location> = vec![];
    let mut valid_addresses: HashSet<&Location> = HashSet::new();

    let mut lines = contents.split('\n').peekable();
    let line_length = lines
        .peek()
        .expect("Lines should have at least one item")
        .chars()
        .count();

    for (line, line_content) in lines.clone().enumerate() {
        let mut constructed_number = String::new();
        let mut constructing = false;

        for (index, char) in line_content.chars().enumerate() {
            if char.is_numeric() {
                constructing = true;
                constructed_number.push(char);
            } else if constructing {
                let mut x_pos = vec![];
                for i in (index - constructed_number.len())..index {
                    x_pos.push(i);
                }
                number_addresses.push(Location {
                    x: x_pos,
                    y: line,
                    // len: constructed_number.len() as i32,
                    value: i32::from_str_radix(&constructed_number, 10).expect("Could not parse"),
                });
                constructing = false;
                constructed_number.clear();
            }
        }
        if constructing {
            let mut x_pos = vec![];
            for i in (line_length - constructed_number.len())..line_length {
                x_pos.push(i);
            }
            number_addresses.push(Location {
                x: x_pos,
                y: line,
                value: i32::from_str_radix(&constructed_number, 10).expect("Could not parse"),
            });
        }
    }

    for (line, line_content) in lines.enumerate() {
        for (index, char) in line_content.chars().enumerate() {
            if is_special_char(&char) {
                for location in &number_addresses {
                    if valid_addresses.contains(&location) {
                        continue;
                    }

                    if create_maps(line).contains(&location.y) {
                        for i in create_maps(index) {
                            if location.x.contains(&i) {
                                valid_addresses.insert(location);
                                break;
                            }
                        }
                    }
                }
            }
        }
    }

    println!(
        "Final answer: {:?}",
        valid_addresses.iter().map(|a| a.value).sum::<i32>()
    );

    Ok(())
}

#[allow(dead_code)]
fn second() -> Result<(), String> {
    let contents = read_to_string("./src/inputs/day03.txt").expect("File could not be opened");

    let mut number_addresses: Vec<Location> = vec![];
    let mut gear_ratios: Vec<i32> = vec![];

    let mut lines = contents.split('\n').peekable();
    let line_length = lines
        .peek()
        .expect("Lines should have at least one item")
        .chars()
        .count();

    for (line, line_content) in lines.clone().enumerate() {
        let mut constructed_number = String::new();
        let mut constructing = false;

        for (index, char) in line_content.chars().enumerate() {
            if char.is_numeric() {
                constructing = true;
                constructed_number.push(char);
            } else if constructing {
                let mut x_pos = vec![];
                for i in (index - constructed_number.len())..index {
                    x_pos.push(i);
                }
                number_addresses.push(Location {
                    x: x_pos,
                    y: line,
                    value: i32::from_str_radix(&constructed_number, 10).expect("Could not parse"),
                });
                constructing = false;
                constructed_number.clear();
            }
        }
        if constructing {
            let mut x_pos = vec![];
            for i in (line_length - constructed_number.len())..line_length {
                x_pos.push(i);
            }
            number_addresses.push(Location {
                x: x_pos,
                y: line,
                value: i32::from_str_radix(&constructed_number, 10).expect("Could not parse"),
            });
        }
    }

    for (line, line_content) in lines.enumerate() {
        for (index, char) in line_content.chars().enumerate() {
            if is_special_char(&char) {
                let mut connecting_gear: Option<&Location> = None;
                for location in &number_addresses {
                    if create_maps(line).contains(&location.y) {
                        for i in create_maps(index) {
                            if location.x.contains(&i) {
                                if connecting_gear.is_some() {
                                    if connecting_gear.unwrap() != location {
                                        gear_ratios
                                            .push(connecting_gear.unwrap().value * location.value);
                                        break;
                                    }
                                } else {
                                    connecting_gear = Some(location);
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    println!("Final answer: {:?}", gear_ratios.iter().sum::<i32>());

    Ok(())
}

fn main() -> Result<(), String> {
    first()
}
