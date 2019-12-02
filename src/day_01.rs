//! Solutions for Advent of Code 2019 Day 01 Part 1

use crate::day_00::{
    calculate_fuel_requirement,
    Fuel,
    Mass,
    SHIP_MODULES_MASSES,
};
use log::debug;

/// Calculate required fuel for the mass of the ship and the additional fuel
/// required to also lift the initial ship mass fuel.
pub fn calculate_compensated_ship_fuel_requirement() -> Fuel {
    SHIP_MODULES_MASSES
        .iter()
        .map(|module_mass| calculate_compensated_fuel_requirement(*module_mass))
        .sum()
}

/// Calculate required fuel for mass while also compensating for the fuel
/// required to lift the added fuel.
pub fn calculate_compensated_fuel_requirement(mass: Mass) -> Fuel {
    let mass_fuel = calculate_fuel_requirement(mass);
    debug!("mass fuel is {}", mass_fuel);

    let fuel_fuel = match calculate_fuel_requirement(mass_fuel) {
        0 => 0,
        additional => {
            debug!(
                "calculating additional fuel required additional is {}",
                additional
            );
            additional + calculate_compensated_fuel_requirement(additional)
        }
    };

    mass_fuel + fuel_fuel
}

#[cfg(test)]
mod tests {
    #[test]
    fn calculate_fuel_requirement() {
        assert_eq!(super::calculate_compensated_fuel_requirement(12), 2);
        assert_eq!(super::calculate_compensated_fuel_requirement(14), 2);
        assert_eq!(super::calculate_compensated_fuel_requirement(1969), 966);
        assert_eq!(
            super::calculate_compensated_fuel_requirement(100_756),
            50346
        );
    }
}
