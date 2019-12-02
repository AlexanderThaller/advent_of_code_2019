pub(super) type Mass = usize;
pub(super) type Fuel = usize;

pub(super) const SHIP_MODULES_MASSES: &[usize] = &[
    134_492, 88713, 84405, 148_193, 95951, 63545, 137_840, 65558, 124_836, 95431, 77622, 91864,
    108_677, 116_871, 119_496, 97172, 86115, 105_704, 68613, 77114, 114_013, 52766, 57048, 80814,
    73888, 58253, 135_934, 97409, 112_439, 98262, 116_047, 57456, 124_261, 83006, 101_495, 133_449,
    111_372, 56146, 87818, 92209, 149_259, 124_559, 141_838, 147_988, 65703, 125_566, 59650,
    139_564, 92430, 126_307, 120_406, 147_383, 84362, 51529, 146_366, 131_840, 53270, 71886,
    118_767, 104_311, 126_181, 76964, 129_430, 95489, 91098, 54133, 110_057, 107_276, 118_226,
    96104, 135_382, 85152, 61697, 143_417, 148_879, 126_846, 130_205, 111_170, 86687, 113_729,
    123_330, 56976, 148_470, 66028, 129_715, 75686, 74964, 148_258, 72669, 88809, 78173, 92699,
    124_806, 67217, 139_066, 136_002, 135_730, 145_708, 142_054, 135_772,
];

pub(super) fn calculate_ship_fuel_requirement() -> Fuel {
    SHIP_MODULES_MASSES
        .iter()
        .map(|module_mass| calculate_fuel_requirement(*module_mass))
        .sum()
}

pub(super) fn calculate_fuel_requirement(mass: Mass) -> Fuel {
    (mass / 3).saturating_sub(2)
}

#[cfg(test)]
mod tests {
    #[test]
    fn calculate_fuel_requirement() {
        assert_eq!(super::calculate_fuel_requirement(12), 2);
        assert_eq!(super::calculate_fuel_requirement(14), 2);
        assert_eq!(super::calculate_fuel_requirement(1969), 654);
        assert_eq!(super::calculate_fuel_requirement(100_756), 33583);
    }
}
