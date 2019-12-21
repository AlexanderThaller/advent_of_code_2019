use super::field::Field;

pub const INPUT: &str = "<x=-1, y=7, z=3>
<x=12, y=2, z=-13>
<x=14, y=18, z=-8>
<x=17, y=4, z=-4>";

pub fn run() {
    let field: Field = INPUT.parse().unwrap();
    let energy = field.step_n(1000).calculate_energy();

    dbg!(energy);
}
