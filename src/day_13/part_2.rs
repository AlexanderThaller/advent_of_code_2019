use super::arcade::Arcade;

pub fn run() {
    let mut arcade = Arcade::default().with_quarters(2);
    let score = arcade.run();

    dbg!(score);
}
