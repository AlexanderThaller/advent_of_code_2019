use crate::day_00::{
    calculate_fuel_requirement,
    Fuel,
    Mass,
    SHIP_MODULES_MASSES,
};

pub(super) fn calculate_ship_fuel_requirement() -> Fuel {
    SHIP_MODULES_MASSES
        .iter()
        .map(|module_mass| advanced_calculate_fuel_requirement(*module_mass))
        .sum()
}

pub(super) fn advanced_calculate_fuel_requirement(mass: Mass) -> Fuel {
    let mass_fuel = calculate_fuel_requirement(mass);
    let fuel_fuel = match calculate_fuel_requirement(mass_fuel) {
        0 => 0,
        additional => additional + advanced_calculate_fuel_requirement(additional),
    };

    mass_fuel + fuel_fuel
}

#[cfg(test)]
mod tests {
    #[test]
    fn calculate_fuel_requirement() {
        assert_eq!(super::advanced_calculate_fuel_requirement(12), 2);
        assert_eq!(super::advanced_calculate_fuel_requirement(14), 2);
        assert_eq!(super::advanced_calculate_fuel_requirement(1969), 966);
        assert_eq!(super::advanced_calculate_fuel_requirement(100_756), 50346);
    }
}
