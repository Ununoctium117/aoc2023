use std::{fmt::Display, str::FromStr};

struct Map {
    rock_locations: Vec<usize>,
    width: usize,
    height: usize,
}
impl Map {
    // part 1, part 2
    fn score(&self) -> (usize, usize) {
        let mut perfect_reflection_score = 0;
        let mut off_by_one_reflection_score = 0;

        let (perfect, off_by_one) = self.scan_reflect_horizontal();
        perfect_reflection_score += perfect.map(|x| x * 100).unwrap_or(0);
        off_by_one_reflection_score += off_by_one.map(|x| x * 100).unwrap_or(0);

        let (perfect, off_by_one) = self.scan_reflect_vertical();
        perfect_reflection_score += perfect.unwrap_or(0);
        off_by_one_reflection_score += off_by_one.unwrap_or(0);

        (perfect_reflection_score, off_by_one_reflection_score)
    }

    // (line of reflection, line of reflection that's off by one)
    fn scan_reflect_vertical(&self) -> (Option<usize>, Option<usize>) {
        let mut actual_reflection = None;
        let mut off_by_one_reflection = None;

        for before_this_col in 1..self.width {
            let right_align_offset = (before_this_col > self.width / 2)
                .then(|| self.width - 2 * (self.width - before_this_col));

            let before_cols = if let Some(right_align_offset) = right_align_offset {
                right_align_offset..before_this_col
            } else {
                0..before_this_col
            };

            let after_cols = if right_align_offset.is_some() {
                before_this_col..self.width
            } else {
                before_this_col..(2 * before_this_col)
            };

            let misreflections = before_cols
                .zip(after_cols.rev())
                .map(|(col_a, col_b)| {
                    (0..self.height)
                        .filter(|r| self.at(*r, col_a) != self.at(*r, col_b))
                        .count()
                })
                .sum();

            match misreflections {
                0 => actual_reflection = Some(before_this_col),
                1 => off_by_one_reflection = Some(before_this_col),
                _ => {}
            }

            if actual_reflection.is_some() && off_by_one_reflection.is_some() {
                break;
            }
        }

        (actual_reflection, off_by_one_reflection)
    }

    fn scan_reflect_horizontal(&self) -> (Option<usize>, Option<usize>) {
        let mut actual_reflection = None;
        let mut off_by_one_reflection = None;

        for before_this_row in 1..self.height {
            let bottom_align_offset = (before_this_row > self.height / 2)
                .then(|| self.height - 2 * (self.height - before_this_row));

            let before_rows = if let Some(bottom_align_offset) = bottom_align_offset {
                bottom_align_offset..before_this_row
            } else {
                0..before_this_row
            };

            let after_rows = if bottom_align_offset.is_some() {
                before_this_row..self.height
            } else {
                before_this_row..(2 * before_this_row)
            };
            let misreflections = before_rows
                .zip(after_rows.rev())
                .map(|(row_a, row_b)| {
                    (0..self.width)
                        .filter(|c| self.at(row_a, *c) != self.at(row_b, *c))
                        .count()
                })
                .sum();

            match misreflections {
                0 => actual_reflection = Some(before_this_row),
                1 => off_by_one_reflection = Some(before_this_row),
                _ => {}
            }

            if actual_reflection.is_some() && off_by_one_reflection.is_some() {
                break;
            }
        }

        (actual_reflection, off_by_one_reflection)
    }

    fn at(&self, row: usize, col: usize) -> bool {
        self.rock_locations
            .binary_search(&(row * self.width + col))
            .is_ok()
    }
}
impl FromStr for Map {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let width = s.find('\n').unwrap();
        let height = s.lines().count();
        let rock_locations = s
            .lines()
            .enumerate()
            .map(|(row, line)| {
                line.chars().enumerate().filter_map(move |(col, char)| {
                    if char == '#' {
                        Some(row * width + col)
                    } else {
                        None
                    }
                })
            })
            .flatten()
            .collect::<Vec<_>>();

        Ok(Self {
            rock_locations,
            width,
            height,
        })
    }
}
impl Display for Map {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for r in 0..self.height {
            for c in 0..self.width {
                if self
                    .rock_locations
                    .binary_search(&(r * self.width + c))
                    .is_ok()
                {
                    write!(f, "#")?;
                } else {
                    write!(f, ".")?;
                }
            }
            writeln!(f)?;
        }

        Ok(())
    }
}

fn main() {
    let (result1, result2): (Vec<_>, Vec<_>) = std::fs::read_to_string("input")
        .unwrap()
        .split("\n\n")
        .map(|map| {
            let map = map.parse::<Map>().unwrap();
            println!("{map}");
            // dbg!(map.width);
            // dbg!(map.scan_reflect_vertical());
            // dbg!(map.scan_reflect_horizontal());
            dbg!(map.score())
        })
        .unzip();

    println!("{}", result1.iter().sum::<usize>());
    println!("{}", result2.iter().sum::<usize>());
}
