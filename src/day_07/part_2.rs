use crate::day_07::{
    amplifier::AmplifiersLoop,
    part_1::AMPLIFIER_SOFTWARE,
};
use itertools::Itertools;

pub fn run() {
    let phase_settings = (0..5).permutations(5);

    let max = phase_settings
        .map(|setting| setting.into_iter().map(|x| x + 5).collect())
        .map(|setting| AmplifiersLoop::new(setting, AMPLIFIER_SOFTWARE.to_vec()).run())
        .max();

    dbg!(max);
}
