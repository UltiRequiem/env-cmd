use dotenvy::dotenv_iter;
use std::process;

fn main() {
    let items = match dotenv_iter() {
        Ok(items) => items,
        Err(error) => {
            eprintln!("Problem opening the .env: '{:?}'.", error.to_string());
            process::exit(1);
        }
    };

    for item in items {
        let (key, val) = item.unwrap();

        println!("{}={}", key, val);
    }
}
