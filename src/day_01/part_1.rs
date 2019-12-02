//! Solutions for Advent of Code 2019 Day 01 Part 1

/// Represents mass of things.
pub type Mass = usize;
/// Represents fuel. One fuel has one mass.
pub type Fuel = usize;

/// List of the masses of all modules in the ship.
///
/// 140005
/// 95473
/// 139497
/// 62962
/// 61114
/// 66330
/// 54137
/// 77360
/// 108752
/// 142999
/// 92160
/// 65690
/// 139896
/// 135072
/// 141864
/// 145599
/// 140998
/// 134694
/// 126576
/// 141438
/// 112238
/// 77339
/// 116736
/// 64294
/// 77811
/// 83634
/// 102059
/// 146691
/// 104534
/// 61196
/// 105119
/// 125791
/// 124352
/// 125501
/// 68498
/// 96795
/// 82878
/// 126702
/// 74334
/// 126798
/// 131179
/// 109231
/// 101065
/// 115470
/// 54542
/// 148706
/// 101296
/// 63312
/// 85799
/// 98328
/// 105926
/// 101047
/// 85470
/// 78531
/// 52510
/// 98761
/// 123019
/// 79495
/// 74902
/// 103869
/// 57090
/// 138222
/// 121620
/// 109994
/// 64769
/// 148785
/// 132349
/// 80485
/// 95575
/// 66123
/// 56283
/// 101019
/// 142671
/// 147116
/// 148490
/// 114580
/// 107192
/// 115741
/// 107455
/// 62769
/// 139998
/// 146798
/// 90032
/// 72028
/// 144485
/// 91251
/// 51054
/// 148665
/// 113542
/// 148607
/// 141060
/// 88025
/// 109776
/// 62421
/// 64482
/// 130387
/// 120481
/// 135012
/// 55101
/// 67926
pub const SHIP_MODULES_MASSES: &[usize] = &[
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

/// Calculates the ship fuel requirements
pub fn calculate_ship_fuel_requirement() -> Fuel {
    SHIP_MODULES_MASSES
        .iter()
        .map(|module_mass| calculate_fuel_requirement(*module_mass))
        .sum()
}

/// Calculate fuel requirement for a given mass based on a predefined formula.
/// Takes the mass of the object that the fuel should be calculated for and
/// returns the fuel required to lift that mass of the ground.
/// ```
/// calculate_fuel_requirement(12);
/// ```
pub fn calculate_fuel_requirement(mass: Mass) -> Fuel {
    (mass / 3).saturating_sub(2)
}

#[cfg(test)]
mod tests {
    extern crate test;

    use test::Bencher;

    #[test]
    fn calculate_fuel_requirement() {
        assert_eq!(super::calculate_fuel_requirement(12), 2);
        assert_eq!(super::calculate_fuel_requirement(14), 2);
        assert_eq!(super::calculate_fuel_requirement(1969), 654);
        assert_eq!(super::calculate_fuel_requirement(100_756), 33583);
    }

    #[bench]
    fn bench_calculate_fuel_requirement_zero(b: &mut Bencher) {
        let n = test::black_box(0);
        b.iter(|| super::calculate_fuel_requirement(n));
    }

    #[bench]
    fn bench_calculate_fuel_requirement_small(b: &mut Bencher) {
        let n = test::black_box(12);
        b.iter(|| super::calculate_fuel_requirement(n));
    }

    #[bench]
    fn bench_calculate_fuel_requirement_big(b: &mut Bencher) {
        let n = test::black_box(999_999);
        b.iter(|| super::calculate_fuel_requirement(n));
    }

    #[bench]
    fn bench_calculate_fuel_requirement_huge(b: &mut Bencher) {
        let n = test::black_box(999_999_999);
        b.iter(|| super::calculate_fuel_requirement(n));
    }

    #[bench]
    fn bench_calculate_fuel_requirement_max(b: &mut Bencher) {
        let n = test::black_box(std::usize::MAX);
        b.iter(|| super::calculate_fuel_requirement(n));
    }
}
