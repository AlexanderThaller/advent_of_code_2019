use super::roboter::Roboter;

pub fn run() {
    let mut robot = Roboter::default();
    robot.run();

    dbg!(&robot.canvas.len());
}
