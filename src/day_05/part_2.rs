use crate::day_05::computer::{
    Computer,
    Reader,
    Writer,
};

pub const INPUT_VALUES: &[isize] = &[5];

pub fn run() {
    let mut computer = Computer {
        memory: crate::day_05::part_1::INPUT_MEMORY.to_vec(),
        input: Reader::Tester(Box::new(INPUT_VALUES.to_vec().into_iter())),
        output: Writer::Tester { values: Vec::new() },
    };

    computer.run().unwrap();
    let output = computer.output;

    dbg!(&output);
}
