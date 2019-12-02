#![deny(missing_docs)]
//! Solutions for Advent of Code 2019

pub mod day_00;
pub mod day_01;

fn main() {
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info"))
        .format_timestamp_nanos()
        .init();

    dbg!(day_00::calculate_ship_fuel_requirement());
    dbg!(day_01::calculate_compensated_ship_fuel_requirement());
}
