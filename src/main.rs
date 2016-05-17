extern crate rand;

use std::io::Read;
use std::fs::File;
use rand::Rng;

fn main() {
    let mut file = File::open("fortunes").unwrap();
    let mut string = String::new();
    file.read_to_string(&mut string)
        .expect("Failed to read fortunes file");

    let fortunes: Vec<&str> = string.split("\n%\n").collect();

    let total = fortunes.len();

    let choice = rand::thread_rng().gen_range(0, total);

    let fortune = fortunes[choice];
    println!("{}", fortune);
}
