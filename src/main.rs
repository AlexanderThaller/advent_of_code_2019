#![feature(test)]
//#![deny(missing_docs)]
//! Solutions for Advent of Code 2019

pub mod day_01;
pub mod day_02;
pub mod day_03;
pub mod day_04;
pub mod day_05;

fn main() {
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("warn"))
        .format_timestamp_nanos()
        .init();

    // dbg!(day_01::part_1::calculate_ship_fuel_requirement());
    // dbg!(day_01::part_2::calculate_compensated_ship_fuel_requirement());
    //
    // dbg!(day_02::part_1::restore_gravity_assist_program());
    // dbg!(day_02::part_2::what().unwrap());
    //
    // dbg!(day_03::closest_intersection());
    // dbg!(day_03::closest_intersection_draw());
    //
    // dbg!(day_04::part_1::count_passwords());
    // dbg!(day_04::part_2::count_passwords());

    day_05::part_1::run();
}
