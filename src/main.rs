use dotenvy::{dotenv_iter, Error};
use std::process;

fn main() {
    let items = match dotenv_iter() {
        Ok(items) => items,
        Err(error) => {
            eprintln!("Problem opening the .env: '{:?}'.", error.to_string());
            process::exit(1);
        }
    };

    let items: Vec<Result<(String, String), Error>> = items.collect();

    let items = items.dedup();

    println!("{:?}", items);
}
