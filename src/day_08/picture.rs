use std::iter::FromIterator;

#[derive(Debug, Copy, Clone)]
pub struct Picture<'a> {
    pub width: usize,
    pub height: usize,

    pub pixels: &'a str,
}

#[derive(Debug)]
pub struct Render {
    pub pixels: Vec<Vec<char>>,
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

    pub fn render(self) -> Render {
        let mut pixels = vec![vec!['2'; self.width]; self.height];

        for layer in self.layers() {
            for (height, line) in layer.iter().enumerate() {
                for (width, pixel) in line.iter().enumerate() {
                    if *pixel == '2' {
                        continue;
                    }

                    let current = pixels[height][width];
                    if current == '2' {
                        pixels[height][width] = *pixel
                    }
                }
            }
        }

        Render { pixels }
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

        Ok(())
    }
}

impl<'a> std::fmt::Display for Render {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f)?;

        for line in &self.pixels {
            for _ in line {
                write!(f, "█")?;
            }
            write!(f, "█")?;
            write!(f, "█")?;
            writeln!(f)?;
            break;
        }

        for line in &self.pixels {
            write!(f, "█")?;

            for pixel in line {
                let c = match *pixel {
                    '1' => " ",
                    '0' => "█",
                    '2' => " ",
                    _ => unreachable!(),
                };

                write!(f, "{}", c)?;
            }

            write!(f, "█")?;

            writeln!(f)?;
        }

        for line in &self.pixels {
            for _ in line {
                write!(f, "█")?;
            }
            write!(f, "█")?;
            write!(f, "█")?;
            writeln!(f)?;
            break;
        }

        Ok(())
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

    #[test]
    fn renderd_day_08_part_02_example_01() {
        let width = 2;
        let height = 2;
        let pixels = "0222112222120000";

        let input = Picture {
            width,
            height,
            pixels,
        };

        let expected = vec![vec!['0', '1'], vec!['1', '0']];

        let got = input.render().pixels;

        assert_eq!(expected, got);
    }
}
