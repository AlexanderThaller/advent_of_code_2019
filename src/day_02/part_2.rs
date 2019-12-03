//! Solutions for Advent of Code 2019 Day 02 Part 2

use crate::day_02::{
    computer::Computer,
    part_1::INPUT,
};

pub fn what() -> Option<usize> {
    const LOOKING_FOR: usize = 19_690_720;

    for noun in { 0..=99 } {
        for verb in { 0..=99 } {
            let mut input = INPUT.to_vec();
            input[1] = noun;
            input[2] = verb;

            let computer = Computer::from(input);
            let values = computer.run().unwrap();

            if values[0] == LOOKING_FOR {
                return Some(100 * noun + verb);
            }
        }
    }

    None
}
