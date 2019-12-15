use std::iter::FromIterator;

#[derive(Debug, Copy, Clone)]
pub struct Picture<'a> {
    pub width: usize,
    pub height: usize,

    pub pixels: &'a str,
}

impl<'a> Picture<'a> {
    pub fn layers(self) -> Vec<Vec<Vec<char>>> {
        let chars = self.pixels.chars().collect::<Vec<char>>();

        let line_chunks = chars
            .chunks(self.width)
            .map(|chunk| chunk.to_vec())
            .collect::<Vec<_>>();

        line_chunks
            .chunks(self.height)
            .map(|chunk| chunk.to_vec())
            .collect::<Vec<_>>()
    }
}

impl<'a> std::fmt::Display for Picture<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for (index, layer) in self.layers().iter().enumerate() {
            writeln!(f, "layer {}:", index)?;

            for line in layer {
                writeln!(f, "{}", String::from_iter(line))?;
            }

            writeln!(f)?;
        }

        unimplemented!()
    }
}

#[cfg(test)]
mod tests {
    use super::Picture;
    use pretty_assertions::assert_eq;

    #[test]
    fn layers_day_08_part_01_example_01() {
        let width = 3;
        let height = 2;
        let pixels = "123456789012";

        let input = Picture {
            width,
            height,
            pixels,
        };

        let expected = vec![
            vec![vec!['1', '2', '3'], vec!['4', '5', '6']],
            vec![vec!['7', '8', '9'], vec!['0', '1', '2']],
        ];

        let got = input.layers();

        assert_eq!(expected, got);
    }
}
