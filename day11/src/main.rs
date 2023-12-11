fn manhattan_distance((r1, c1): (usize, usize), (r2, c2): (usize, usize)) -> usize {
    r2.abs_diff(r1) + c2.abs_diff(c1)
}

#[derive(Debug, Clone)]
struct Image {
    galaxies: Vec<(usize, usize)>,
    width: usize,
    height: usize,
}
impl Image {
    fn print(&self) {
        for i in 0..self.width {
            print!("{}", i % 10);
        }
        println!();

        for row in 0..self.height {
            for col in 0..self.width {
                if self.galaxies.contains(&(row, col)) {
                    print!("#");
                } else {
                    print!(".");
                }
            }
            println!();
        }

        println!();
    }

    fn galaxy_pairs(&self) -> Vec<((usize, usize), (usize, usize))> {
        let mut result = Vec::new();

        for i in 0..self.galaxies.len() {
            for j in (i + 1)..self.galaxies.len() {
                result.push((self.galaxies[i], self.galaxies[j]));
            }
        }

        result
    }

    fn expand(&self, scale: usize) -> Image {
        let mut updated = self.clone();

        // expand rows
        for row in 0..self.height {
            if !self.galaxies.iter().any(|(test_row, _)| row == *test_row) {
                updated.height += scale;

                // increase row number of all galaxies after this row
                for (_, (updated_row, _)) in self
                    .galaxies
                    .iter()
                    .zip(updated.galaxies.iter_mut())
                    .filter(|((orig_row, _), _)| *orig_row > row)
                {
                    *updated_row += scale;
                }
            }
        }

        // expand columns
        for col in 0..self.width {
            if !self.galaxies.iter().any(|(_, test_col)| col == *test_col) {
                updated.width += scale;

                // increase column number of all galaxies after this row
                for (_, (_, updated_col)) in self
                    .galaxies
                    .iter()
                    .zip(updated.galaxies.iter_mut())
                    .filter(|((_, orig_col), _)| *orig_col > col)
                {
                    *updated_col += scale;
                }
            }
        }

        updated
    }
}

fn main() {
    let text = std::fs::read_to_string("input").unwrap();
    let galaxies = {
        text.lines()
            .enumerate()
            .map(|(row, line)| {
                line.chars().enumerate().filter_map(move |(col, char)| {
                    if char == '#' {
                        Some((row.clone(), col))
                    } else {
                        None
                    }
                })
            })
            .flatten()
            .collect::<Vec<_>>()
    };

    let image = Image {
        galaxies,
        width: text.lines().next().unwrap().len(),
        height: text.lines().count(),
    };

    image.print();
    let result1: usize = image
        .expand(1)
        .galaxy_pairs()
        .into_iter()
        .map(|(g1, g2)| manhattan_distance(g1, g2))
        .sum();
    println!("{result1}");

    let result2: usize = image
        .expand(999_999)
        .galaxy_pairs()
        .into_iter()
        .map(|(g1, g2)| manhattan_distance(g1, g2))
        .sum();
    println!("{result2}");
}
