use crate::day_09::{
    computer::{
        Computer,
        Reader,
    },
    part_1::BOOST_SOFTWARE,
};

pub const INPUT_VALUES: &[isize] = &[2];

pub fn run() {
    let mut computer = Computer::default()
        .with_memory(BOOST_SOFTWARE.to_vec())
        .with_input(Reader::Tester(Box::new(INPUT_VALUES.to_vec().into_iter())));

    computer.run().unwrap();
    let output = computer.output.values();

    dbg!(output);
}
