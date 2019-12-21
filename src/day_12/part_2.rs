use super::field::Field;
use num::Integer;

pub fn run() {
    let start_field: Field = super::part_1::INPUT.parse().unwrap();

    let (steps_x, _) =
        std::iter::successors(Some((2usize, start_field.step())), |(steps, field)| {
            Some((steps + 1, field.step()))
        })
        .take_while(|(_, field)| field.objects_x() != start_field.objects_x())
        .last()
        .unwrap();

    let (steps_y, _) =
        std::iter::successors(Some((2usize, start_field.step())), |(steps, field)| {
            Some((steps + 1, field.step()))
        })
        .take_while(|(_, field)| field.objects_y() != start_field.objects_y())
        .last()
        .unwrap();

    let (steps_z, _) =
        std::iter::successors(Some((2usize, start_field.step())), |(steps, field)| {
            Some((steps + 1, field.step()))
        })
        .take_while(|(_, field)| field.objects_z() != start_field.objects_z())
        .last()
        .unwrap();

    dbg!(steps_x.lcm(&steps_y).lcm(&steps_z));
}
