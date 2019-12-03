//! Solutions for Advent of Code 2019 Day 02 Part 1
use crate::day_02::computer::Computer;

pub const INPUT: &[usize] = &[
    1, 0, 0, 3, 1, 1, 2, 3, 1, 3, 4, 3, 1, 5, 0, 3, 2, 13, 1, 19, 1, 5, 19, 23, 2, 10, 23, 27, 1,
    27, 5, 31, 2, 9, 31, 35, 1, 35, 5, 39, 2, 6, 39, 43, 1, 43, 5, 47, 2, 47, 10, 51, 2, 51, 6, 55,
    1, 5, 55, 59, 2, 10, 59, 63, 1, 63, 6, 67, 2, 67, 6, 71, 1, 71, 5, 75, 1, 13, 75, 79, 1, 6, 79,
    83, 2, 83, 13, 87, 1, 87, 6, 91, 1, 10, 91, 95, 1, 95, 9, 99, 2, 99, 13, 103, 1, 103, 6, 107,
    2, 107, 6, 111, 1, 111, 2, 115, 1, 115, 13, 0, 99, 2, 0, 14, 0,
];

pub fn restore_gravity_assist_program() -> usize {
    let mut input = INPUT.to_vec();

    input[1] = 12;
    input[2] = 2;

    let computer = Computer::from(input);
    let values = computer.run().unwrap();

    values[0]
}
