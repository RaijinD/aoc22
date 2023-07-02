mod day1;
use std::env;

use std::{
    fs::File,
    io::{prelude::*, BufReader},
};

use day1::*;


fn main() {
    let args: Vec<String> = env::args().collect();
    let input_path = &args[1];

    let input_file = File::open(input_path).expect("Input path must be given");

    let buffer_reader = BufReader::new(input_file)
        .lines()
        .map(|l| l.expect("Could not parse line"))
        .collect::<Vec<String>>();
    part1(&buffer_reader);
    part2(&buffer_reader);
}
