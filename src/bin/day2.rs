use std::fs::File;
use std::io::{BufRead, BufReader};

#[allow(dead_code)]
fn first() -> Result<(), String> {
    let file = File::open("./src/inputs/day02.txt");
    let reader = BufReader::new(file.expect("File could not be loaded"));
    let mut lines = reader.lines();

    let mut valid_games: Vec<i32> = vec![];

    'line_loop: while let Some(Ok(line)) = lines.next() {
        let mut split_line = line.split(":");
        let raw_game_id = split_line.next().expect("Failed to parse header");
        // Can't use Regex because no internet :D
        // sooo... format is `Game \d+: ...`
        // remove `Game` and parse to int

        let game_id = i32::from_str_radix(&raw_game_id[5..raw_game_id.len()], 10)
            .expect("Failed to parse game id as int");

        let body = split_line.next().expect("Failed to parse body");

        let formatted_body = body.replace(" ", "");
        let parts = formatted_body.split(";");

        for part in parts {
            let items = part.split(",");
            for item in items {
                let mut peekable_chars = item.chars().peekable();
                let mut quantity = String::new();

                while peekable_chars
                    .peek()
                    .expect("Numeric characters should not be at the end")
                    .is_numeric()
                {
                    quantity.push(
                        peekable_chars
                            .next()
                            .expect("This char has already been peeked"),
                    )
                }
                let quantity = i32::from_str_radix(quantity.as_str(), 10)
                    .expect("Failed to parse quantity to int");

                let color = peekable_chars.collect::<String>();

                match color.as_str() {
                    "red" => {
                        if quantity > 12 {
                            continue 'line_loop;
                        }
                    }
                    "green" => {
                        if quantity > 13 {
                            continue 'line_loop;
                        }
                    }
                    "blue" => {
                        if quantity > 14 {
                            continue 'line_loop;
                        }
                    }
                    _ => {}
                }
            }
        }

        valid_games.push(game_id);
    }

    println!("Final result: {}", valid_games.iter().sum::<i32>());
    Ok(())
}

#[allow(dead_code)]
fn second() -> Result<(), String> {
    let file = File::open("./src/inputs/day02.txt");
    let reader = BufReader::new(file.expect("File could not be loaded"));
    let mut lines = reader.lines();

    let mut powers: Vec<i32> = vec![];

    while let Some(Ok(line)) = lines.next() {
        let mut max_red = 0;
        let mut max_green = 0;
        let mut max_blue = 0;

        let body = line
            .split(":")
            .skip(1)
            .next()
            .expect("Failed to parse body");

        let formatted_body = body.replace(" ", "");
        let parts = formatted_body.split(";");

        for part in parts {
            let items = part.split(",");
            for item in items {
                let mut chars = item.chars().peekable();
                let mut quantity = String::new();

                while chars
                    .peek()
                    .expect("Numeric characters should not be at the end")
                    .is_numeric()
                {
                    quantity.push(chars.next().expect("This char has already been peeked"))
                }
                let quantity = i32::from_str_radix(quantity.as_str(), 10)
                    .expect("Failed to parse quantity to int");

                let color = chars.collect::<String>();
                match color.as_str() {
                    "red" => {
                        if quantity > max_red {
                            max_red = quantity;
                        }
                    }
                    "green" => {
                        if quantity > max_green {
                            max_green = quantity;
                        }
                    }
                    "blue" => {
                        if quantity > max_blue {
                            max_blue = quantity;
                        }
                    }
                    _ => {}
                }
            }
        }

        powers.push(max_red * max_blue * max_green);
    }

    println!("Final result: {}", powers.iter().sum::<i32>());
    Ok(())
}

fn main() -> Result<(), String> {
    second()
}
