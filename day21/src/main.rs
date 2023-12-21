use std::{collections::HashSet, ops::Index, rc::Rc, str::FromStr};

use nalgebra::{Matrix3, Vector3};

#[derive(Debug, Clone, Copy)]
enum Tile {
    Garden,
    Rock,
}

#[derive(Clone, Debug)]
struct Map {
    tiles: Rc<[Box<[Tile]>]>,
    starting_tile: (usize, usize),
}
impl Map {
    fn adjacent_garden_tiles(&self, r: isize, c: isize) -> impl Iterator<Item = (isize, isize)> {
        let map_clone = self.clone();
        [(-1, 0), (1, 0), (0, -1), (0, 1)].into_iter().filter_map(
            move |(dr, dc)| -> Option<(isize, isize)> {
                let r = r.checked_add(dr)?;
                let c = c.checked_add(dc)?;

                match map_clone[(r, c)] {
                    Tile::Garden => Some((r, c)),
                    Tile::Rock => None,
                }
            },
        )
    }

    fn normalize(&self, (r, c): (isize, isize)) -> (usize, usize) {
        (
            r.rem_euclid(self.tiles.len() as isize) as usize,
            c.rem_euclid(self.tiles[0].len() as isize) as usize,
        )
    }
}
impl Index<(isize, isize)> for Map {
    type Output = Tile;

    fn index(&self, coords: (isize, isize)) -> &Self::Output {
        let (r, c) = self.normalize(coords);
        &self.tiles[r][c]
    }
}
impl FromStr for Map {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut starting_tile = None;

        let tiles = s
            .lines()
            .enumerate()
            .map(|(row, line)| {
                line.chars()
                    .enumerate()
                    .map(|(col, ch)| match ch {
                        '.' => Tile::Garden,
                        '#' => Tile::Rock,
                        'S' => {
                            starting_tile = Some((row, col));
                            Tile::Garden
                        }
                        _ => panic!(),
                    })
                    .collect()
            })
            .collect();

        Ok(Map {
            tiles,
            starting_tile: starting_tile.unwrap(),
        })
    }
}

fn bounded_bfs(
    map: &Map,
    limit: usize,
    mut current_queue: HashSet<(isize, isize)>,
) -> HashSet<(isize, isize)> {
    // let mut visited = HashSet::new();
    let mut next_queue = HashSet::new();

    for i in 0..limit {
        for (r, c) in current_queue.iter() {
            next_queue.extend(map.adjacent_garden_tiles(*r, *c));
        }

        if i == limit - 1 {
            return next_queue;
        }

        current_queue.clear();
        std::mem::swap(&mut current_queue, &mut next_queue);
    }

    unreachable!()
}

fn main() {
    let p1_cycles = 64;
    let p2_cycles = 26501365; // 481843 * 11 * 5

    let map: Map = std::fs::read_to_string("input").unwrap().parse().unwrap();

    let mut p1_starting_queue = HashSet::new();
    p1_starting_queue.insert((map.starting_tile.0 as isize, map.starting_tile.1 as isize));
    dbg!(bounded_bfs(&map, p1_cycles, p1_starting_queue).len());

    // entire starting row/col is all gardens
    // total area covered by n boards grows quadratically
    // we cover some subset of the boards, so we should be quadratic(?)
    let board_height = map.tiles.len();
    let mut p2_queue = HashSet::new();
    p2_queue.insert((map.starting_tile.0 as isize, map.starting_tile.1 as isize));
    let offset = p2_cycles % board_height;
    let p2_queue = bounded_bfs(&map, dbg!(offset), p2_queue);

    let p2_queue = bounded_bfs(&map, board_height, p2_queue);
    let sample_1 = p2_queue.len();
    let p2_queue = bounded_bfs(&map, board_height, p2_queue);
    let sample_2 = p2_queue.len();
    let p2_queue = bounded_bfs(&map, board_height, p2_queue);
    let sample_3 = p2_queue.len();

    dbg!(sample_1, sample_2, sample_3);

    // Construct a quadratic from f(x), where x = the offset + number_of_board_heights

    // sample_1 = 1a + 1b + c // (x = 1)
    // sample_2 = 4a + 2b + c // (x = 2)
    // sample_3 = 9a + 3b + c // (x = 3)
    // in matrix form:
    // [1, 1, 1]   [a]   [sample_1]
    // [4, 2, 1] * [b] = [sample_2]
    // [9, 3, 1]   [c]   [sample_3]
    #[cfg_attr(rustfmt, rustfmt_skip)]
    let coeffs = Matrix3::<f64>::new(
        1.0, 1.0, 1.0,
        4.0, 2.0, 1.0,
        9.0, 3.0, 1.0,
    );
    let decomp = coeffs.lu();
    let result = Vector3::new(sample_1 as f64, sample_2 as f64, sample_3 as f64);
    let solution = decomp.solve(&result).unwrap();
    let &[a, b, c] = solution.as_slice() else {
        panic!()
    };

    let a = a.round() as usize;
    let b = b.round() as usize;
    let c = c.round() as usize;

    let x = (p2_cycles - offset) / board_height;
    dbg!(a, b, c, x, x * board_height + offset, p2_cycles);
    dbg!(a * x * x + b * x + c);
}
