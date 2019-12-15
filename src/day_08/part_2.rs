use crate::day_08::{
    part_1::{
        HEIGHT,
        PIXELS,
        WIDTH,
    },
    picture::Picture,
};

pub fn run() {
    let picture = Picture {
        width: WIDTH,
        height: HEIGHT,
        pixels: PIXELS,
    };

    let out = picture.render();

    println!("day_08 - part_2:\n{}", out);
}
