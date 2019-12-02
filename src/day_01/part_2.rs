//! Solutions for Advent of Code 2019 Day 01 Part 2

use crate::day_01::part_1::{
    calculate_fuel_requirement,
    Fuel,
    Mass,
    SHIP_MODULES_MASSES,
};

/// Calculate required fuel for the mass of the ship and the additional fuel
/// required to also lift the initial ship mass fuel.
pub fn calculate_compensated_ship_fuel_requirement() -> Fuel {
    SHIP_MODULES_MASSES
        .iter()
        .map(|module_mass| calculate_compensated_fuel_requirement(*module_mass))
        .sum()
}

/// Calculate required fuel for mass while also compensating for the fuel
/// required to lift the fuel.
pub fn calculate_compensated_fuel_requirement(mass: Mass) -> Fuel {
    let base_mass = calculate_fuel_requirement(mass);

    std::iter::successors(Some(base_mass), |&x| Some(calculate_fuel_requirement(x)))
        .take_while(|&x| x != 0)
        .sum()
}

#[cfg(test)]
mod tests {
    extern crate test;
    use test::Bencher;

    #[test]
    fn calculate_compensated_fuel_requirement() {
        assert_eq!(super::calculate_compensated_fuel_requirement(12), 2);
        assert_eq!(super::calculate_compensated_fuel_requirement(14), 2);
        assert_eq!(super::calculate_compensated_fuel_requirement(1969), 966);
        assert_eq!(
            super::calculate_compensated_fuel_requirement(100_756),
            50346
        );
    }

    #[bench]
    fn bench_calculate_compensated_fuel_requirement_zero(b: &mut Bencher) {
        let n = test::black_box(0);
        b.iter(|| super::calculate_compensated_fuel_requirement(n));
    }

    #[bench]
    fn bench_calculate_compensated_fuel_requirement_small(b: &mut Bencher) {
        let n = test::black_box(12);
        b.iter(|| super::calculate_compensated_fuel_requirement(n));
    }

    #[bench]
    fn bench_calculate_compensated_fuel_requirement_big(b: &mut Bencher) {
        let n = test::black_box(999_999);
        b.iter(|| super::calculate_compensated_fuel_requirement(n));
    }

    #[bench]
    fn bench_calculate_compensated_fuel_requirement_huge(b: &mut Bencher) {
        let n = test::black_box(999_999_999);
        b.iter(|| super::calculate_compensated_fuel_requirement(n));
    }

    #[bench]
    fn bench_calculate_compensated_fuel_requirement_max(b: &mut Bencher) {
        let n = test::black_box(std::usize::MAX);
        b.iter(|| super::calculate_compensated_fuel_requirement(n));
    }
}
