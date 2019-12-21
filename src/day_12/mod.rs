pub mod field;
pub mod object;
pub mod part_1;
pub mod part_2;
pub mod position;
pub mod velocity;

pub fn parse_values(s: &str) -> Result<(isize, isize, isize), ParseValueError> {
    let mut x = 0;
    let mut y = 0;
    let mut z = 0;

    for split_comma in s.split(',') {
        let split_equal = split_comma.trim().split('=').collect::<Vec<_>>();

        match split_equal.as_slice() {
            ["x", value] => x = value.trim().parse()?,
            ["y", value] => y = value.trim().parse()?,
            ["z", value] => z = value.trim().parse()?,
            [name, _] => return Err(ParseValueError::UnkownCoordinateType((*name).to_string())),
            _ => unreachable!(),
        };
    }

    Ok((x, y, z))
}

#[derive(Debug)]
pub enum ParseValueError {
    UnkownCoordinateType(String),
    IntError(std::num::ParseIntError),
}

impl From<std::num::ParseIntError> for ParseValueError {
    fn from(err: std::num::ParseIntError) -> Self {
        Self::IntError(err)
    }
}
