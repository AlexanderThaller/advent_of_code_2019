#![feature(test)]
//#![deny(missing_docs)]
//! Solutions for Advent of Code 2019

pub mod day_01;
pub mod day_02;
pub mod day_03;
pub mod day_04;
pub mod day_05;
pub mod day_07;
pub mod day_08;
pub mod day_09;
pub mod day_10;
pub mod day_11;

fn main() {
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("warn"))
        .format_timestamp_nanos()
        .init();

    let args = std::env::args();
    let mut args = args.skip(1);

    let day = args.next().unwrap_or_default();

    match day.as_str() {
        "day_01" => {
            dbg!(day_01::part_1::calculate_ship_fuel_requirement());
            dbg!(day_01::part_2::calculate_compensated_ship_fuel_requirement());
        }

        "day_02" => {
            dbg!(day_02::part_1::restore_gravity_assist_program());
            dbg!(day_02::part_2::what().unwrap());
        }

        "day_03" => {
            dbg!(day_03::closest_intersection());
            dbg!(day_03::closest_intersection_draw());
        }

        "day_04" => {
            dbg!(day_04::part_1::count_passwords());
            dbg!(day_04::part_2::count_passwords());
        }

        "day_05" => {
            day_05::part_1::run();
            day_05::part_2::run();
        }

        "day_07" => {
            day_07::part_1::run();
            day_07::part_2::run();
        }

        "day_08" => {
            day_08::part_1::run();
            day_08::part_2::run();
        }

        "day_09" => {
            day_09::part_1::run();
            day_09::part_2::run();
        }

        "day_10" => {
            day_10::part_1::run();
            day_10::part_2::run();
        }

        _ => {
            day_11::part_1::run();
            day_11::part_2::run();
        }
    }
}
