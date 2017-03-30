extern crate phf;

use std::io::{self, BufReader, BufRead};

include!(concat!(env!("OUT_DIR"), "/flip_char.rs"));

fn main() {
    let stdin = io::stdin();
    let stdin = stdin.lock();
    let stdin = BufReader::new(stdin);

    for line in stdin.lines() {
        let line = line.unwrap();

        for c in line.chars().rev() {
            print!("{}", match FLIPPED.get(&c) {
                Some(e) => *e,
                None    => c
            });
        }
        print!("\n");
    }
}
