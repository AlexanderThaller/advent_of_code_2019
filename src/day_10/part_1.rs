use crate::day_10::asteroids::{
    angles::Angles,
    canvas,
    ray::{
        Ray,
        RAY_MIN_STEP,
    },
    Field,
    Object,
    Position,
};

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
