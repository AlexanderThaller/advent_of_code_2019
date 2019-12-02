//! Solutions for Advent of Code 2019 Day 01

pub mod part_1;
pub mod part_2;

pub use crate::day_01::{
    part_1::{
        calculate_fuel_requirement,
        calculate_ship_fuel_requirement,
    },
    part_2::{
        calculate_compensated_fuel_requirement,
        calculate_compensated_ship_fuel_requirement,
    },
};
