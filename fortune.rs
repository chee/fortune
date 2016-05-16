extern crate rand;

use std::io::prelude::*;
use std::fs::File;
use rand::distributions::{IndependentSample, Range};

fn main() {
    let mut file = File::open("fortunes").unwrap();
    let mut string = String::new();
    file.read_to_string(&mut string);

    let total = string.split("\n%\n").count();

    let range = Range::new(0, total);

    let choice = range.ind_sample(&mut rand::thread_rng());// * total;

    let fortune = string.split("\n%\n").nth(choice).unwrap();
    println!("{}", fortune);
}
