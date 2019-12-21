use super::arcade::{
    Arcade,
    Tile,
};

pub fn run() {
    let mut arcade = Arcade::default();
    let score = arcade.run();

    let blocks = arcade
        .tiles
        .values()
        .filter(|tile| **tile == Tile::Block)
        .count();

    dbg!((blocks, score));
}
