use crate::day_10::asteroids::Field;

pub fn run() {
    const EXAMPLE_01: &str = "#....
.....
.....
.....
....#";

    let field: Field = EXAMPLE_01.into();

    let (position, count) = field.find_best_monitoring_location();

    dbg!((position, count));
}
