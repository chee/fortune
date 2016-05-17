extern crate rand;

use std::io::Read;
use std::fs::File;
use rand::Rng;

fn main() {
    let mut file = File::open("fortunes").unwrap();
    let mut string = String::new();
    file.read_to_string(&mut string)
        .expect("Failed to read file");

    let total = string.split("\n%\n").count();

    let choice = rand::thread_rng().gen_range(0, total);

    let fortune = string.split("\n%\n").nth(choice).unwrap();
    println!("{}", fortune);
}
