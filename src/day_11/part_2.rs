use super::roboter::{
    Color,
    Roboter,
};

pub fn run() {
    let mut robot = Roboter::default().with_default_color(Color::White);
    robot.run()
}
