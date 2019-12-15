use crate::day_09::computer::{
    Computer,
    Reader,
};

pub const BOOST_SOFTWARE: &[isize] = &[
    1102, 34_463_338, 34_463_338, 63, 1007, 63, 34_463_338, 63, 1005, 63, 53, 1101, 3, 0, 1000,
    109, 988, 209, 12, 9, 1000, 209, 6, 209, 3, 203, 0, 1008, 1000, 1, 63, 1005, 63, 65, 1008,
    1000, 2, 63, 1005, 63, 904, 1008, 1000, 0, 63, 1005, 63, 58, 4, 25, 104, 0, 99, 4, 0, 104, 0,
    99, 4, 17, 104, 0, 99, 0, 0, 1101, 0, 33, 1003, 1101, 0, 23, 1002, 1102, 1, 557, 1022, 1102, 1,
    24, 1010, 1102, 1, 22, 1014, 1101, 470, 0, 1027, 1102, 38, 1, 1001, 1102, 1, 21, 1012, 1102, 1,
    1, 1021, 1101, 0, 26, 1018, 1101, 0, 827, 1024, 1101, 690, 0, 1029, 1101, 0, 473, 1026, 1102,
    1, 27, 1015, 1101, 695, 0, 1028, 1101, 822, 0, 1025, 1102, 1, 35, 1019, 1102, 1, 30, 1000,
    1101, 0, 39, 1013, 1101, 25, 0, 1016, 1101, 28, 0, 1006, 1102, 1, 36, 1004, 1101, 34, 0, 1011,
    1101, 31, 0, 1017, 1101, 0, 0, 1020, 1101, 29, 0, 1009, 1102, 1, 554, 1023, 1102, 32, 1, 1007,
    1101, 37, 0, 1008, 1101, 20, 0, 1005, 109, 5, 2101, 0, 0, 63, 1008, 63, 20, 63, 1005, 63, 203,
    4, 187, 1106, 0, 207, 1001, 64, 1, 64, 1002, 64, 2, 64, 109, -4, 2107, 21, 4, 63, 1005, 63,
    227, 1001, 64, 1, 64, 1105, 1, 229, 4, 213, 1002, 64, 2, 64, 109, 4, 2108, 37, 3, 63, 1005, 63,
    251, 4, 235, 1001, 64, 1, 64, 1106, 0, 251, 1002, 64, 2, 64, 109, 12, 21101, 40, 0, -5, 1008,
    1012, 38, 63, 1005, 63, 275, 1001, 64, 1, 64, 1105, 1, 277, 4, 257, 1002, 64, 2, 64, 109, -14,
    21108, 41, 41, 10, 1005, 1013, 299, 4, 283, 1001, 64, 1, 64, 1105, 1, 299, 1002, 64, 2, 64,
    109, 5, 1202, -4, 1, 63, 1008, 63, 36, 63, 1005, 63, 321, 4, 305, 1106, 0, 325, 1001, 64, 1,
    64, 1002, 64, 2, 64, 109, -3, 2108, 38, -1, 63, 1005, 63, 345, 1001, 64, 1, 64, 1106, 0, 347,
    4, 331, 1002, 64, 2, 64, 109, -8, 1201, 4, 0, 63, 1008, 63, 40, 63, 1005, 63, 367, 1105, 1,
    373, 4, 353, 1001, 64, 1, 64, 1002, 64, 2, 64, 109, 20, 1205, 4, 391, 4, 379, 1001, 64, 1, 64,
    1106, 0, 391, 1002, 64, 2, 64, 109, 5, 1205, -2, 407, 1001, 64, 1, 64, 1106, 0, 409, 4, 397,
    1002, 64, 2, 64, 109, -15, 2102, 1, -3, 63, 1008, 63, 36, 63, 1005, 63, 431, 4, 415, 1106, 0,
    435, 1001, 64, 1, 64, 1002, 64, 2, 64, 109, -6, 1202, 6, 1, 63, 1008, 63, 31, 63, 1005, 63,
    459, 1001, 64, 1, 64, 1105, 1, 461, 4, 441, 1002, 64, 2, 64, 109, 28, 2106, 0, -2, 1105, 1,
    479, 4, 467, 1001, 64, 1, 64, 1002, 64, 2, 64, 109, -14, 21107, 42, 41, -4, 1005, 1011, 499,
    1001, 64, 1, 64, 1106, 0, 501, 4, 485, 1002, 64, 2, 64, 109, 8, 1206, -3, 515, 4, 507, 1105, 1,
    519, 1001, 64, 1, 64, 1002, 64, 2, 64, 109, -29, 2101, 0, 6, 63, 1008, 63, 33, 63, 1005, 63,
    539, 1105, 1, 545, 4, 525, 1001, 64, 1, 64, 1002, 64, 2, 64, 109, 30, 2105, 1, -1, 1106, 0,
    563, 4, 551, 1001, 64, 1, 64, 1002, 64, 2, 64, 109, 5, 1206, -8, 579, 1001, 64, 1, 64, 1106, 0,
    581, 4, 569, 1002, 64, 2, 64, 109, -31, 1201, 3, 0, 63, 1008, 63, 38, 63, 1005, 63, 607, 4,
    587, 1001, 64, 1, 64, 1106, 0, 607, 1002, 64, 2, 64, 109, 11, 21101, 43, 0, 4, 1008, 1013, 43,
    63, 1005, 63, 633, 4, 613, 1001, 64, 1, 64, 1106, 0, 633, 1002, 64, 2, 64, 109, -10, 2107, 22,
    3, 63, 1005, 63, 651, 4, 639, 1106, 0, 655, 1001, 64, 1, 64, 1002, 64, 2, 64, 109, 26, 21102,
    44, 1, -8, 1008, 1017, 44, 63, 1005, 63, 681, 4, 661, 1001, 64, 1, 64, 1105, 1, 681, 1002, 64,
    2, 64, 109, -3, 2106, 0, 6, 4, 687, 1105, 1, 699, 1001, 64, 1, 64, 1002, 64, 2, 64, 109, -3,
    21108, 45, 43, 0, 1005, 1019, 715, 1105, 1, 721, 4, 705, 1001, 64, 1, 64, 1002, 64, 2, 64, 109,
    -25, 1207, 9, 32, 63, 1005, 63, 737, 1105, 1, 743, 4, 727, 1001, 64, 1, 64, 1002, 64, 2, 64,
    109, 18, 21107, 46, 47, 3, 1005, 1015, 761, 4, 749, 1106, 0, 765, 1001, 64, 1, 64, 1002, 64, 2,
    64, 109, -3, 2102, 1, -3, 63, 1008, 63, 31, 63, 1005, 63, 789, 1001, 64, 1, 64, 1105, 1, 791,
    4, 771, 1002, 64, 2, 64, 109, -5, 1208, -4, 30, 63, 1005, 63, 813, 4, 797, 1001, 64, 1, 64,
    1105, 1, 813, 1002, 64, 2, 64, 109, 28, 2105, 1, -8, 4, 819, 1106, 0, 831, 1001, 64, 1, 64,
    1002, 64, 2, 64, 109, -30, 1207, 0, 24, 63, 1005, 63, 853, 4, 837, 1001, 64, 1, 64, 1106, 0,
    853, 1002, 64, 2, 64, 109, 16, 21102, 47, 1, -7, 1008, 1011, 45, 63, 1005, 63, 873, 1105, 1,
    879, 4, 859, 1001, 64, 1, 64, 1002, 64, 2, 64, 109, -21, 1208, 5, 26, 63, 1005, 63, 899, 1001,
    64, 1, 64, 1105, 1, 901, 4, 885, 4, 64, 99, 21102, 27, 1, 1, 21102, 915, 1, 0, 1106, 0, 922,
    21201, 1, 69417, 1, 204, 1, 99, 109, 3, 1207, -2, 3, 63, 1005, 63, 964, 21201, -2, -1, 1,
    21101, 0, 942, 0, 1106, 0, 922, 21201, 1, 0, -1, 21201, -2, -3, 1, 21101, 0, 957, 0, 1105, 1,
    922, 22201, 1, -1, -2, 1105, 1, 968, 22102, 1, -2, -2, 109, -3, 2106, 0, 0,
];

pub const INPUT_VALUES: &[isize] = &[1];

pub fn run() {
    let mut computer = Computer::default()
        .with_memory(BOOST_SOFTWARE.to_vec())
        .with_input(Reader::Tester(Box::new(INPUT_VALUES.to_vec().into_iter())));

    computer.run().unwrap();
    let output = computer.output.values();

    dbg!(output);
}
