use std::env;

fn main() {
    let args: env::Args = env::args();
    if let Some(a) = args.skip(1).next() {
        if let Ok(b) = a.parse::<i32>() {
            println!("Ok, {}", b)
        }
        println!("{}", a)
    } else {
        println!("null");
    }
}
