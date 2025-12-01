use std::{
    fs::File,
    io::{self, BufRead},
};

use crate::december_1::input_iter::InputIterator;

pub fn read_input() -> InputIterator {
    let file = File::open("./input/december_1.txt").expect("File must exist!");

    let lines = io::BufReader::new(file).lines();

    return InputIterator::new(lines);
}
