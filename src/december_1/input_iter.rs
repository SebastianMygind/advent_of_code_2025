use std::{
    fs::File,
    io::{BufReader, Lines},
};

use crate::december_1::dial::Direction;

pub struct InputIterator {
    file_reader: Lines<BufReader<File>>,
}

impl Iterator for InputIterator {
    type Item = (Direction, u32);

    fn next(&mut self) -> Option<Self::Item> {
        let line = self.file_reader.next()?.ok()?;

        return Some(Self::parse_str_to_tuple(&line));
    }
}

impl InputIterator {
    pub fn new(reader: Lines<BufReader<File>>) -> Self {
        Self {
            file_reader: reader,
        }
    }

    fn parse_str_to_tuple(tuple: &str) -> (Direction, u32) {
        let direction =
            Self::parse_direction(tuple).expect("Could not parse direction from {tuple}");

        let rotations_str = &tuple[1..];

        let rotations = rotations_str
            .parse::<u32>()
            .expect("Could not parse rotations from {rotations_str}");

        return (direction, rotations);
    }

    fn parse_direction(dir_str: &str) -> Option<Direction> {
        let mut chars = dir_str.chars();

        let dir_char = chars.next()?;

        match dir_char {
            'L' => Some(Direction::Left),
            'R' => Some(Direction::Right),
            _ => None,
        }
    }
}
