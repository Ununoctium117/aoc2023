use std::{collections::HashSet, fmt::Display, str::FromStr};

#[derive(Debug, Clone, Copy)]
enum Tile {
    ForwardsMirror,
    BackwardsMirror,
    HorizSplitter,
    VertSplitter,
    Nothing,
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}
impl Default for Direction {
    fn default() -> Self {
        Direction::Right
    }
}
impl Direction {
    fn try_move(&self, r: usize, c: usize, w: usize, h: usize) -> Option<(usize, usize)> {
        let update = match self {
            Direction::Up => (r.checked_sub(1)?, c),
            Direction::Down => (r.checked_add(1)?, c),
            Direction::Left => (r, c.checked_sub(1)?),
            Direction::Right => (r, c.checked_add(1)?),
        };

        if update.0 >= h || update.1 >= w {
            None
        } else {
            Some(update)
        }
    }
}

fn next_direction(tile: Tile, direction: Direction) -> (Direction, Option<Direction>) {
    match (tile, direction) {
        (Tile::ForwardsMirror, Direction::Up) => (Direction::Right, None),
        (Tile::ForwardsMirror, Direction::Down) => (Direction::Left, None),
        (Tile::ForwardsMirror, Direction::Right) => (Direction::Up, None),
        (Tile::ForwardsMirror, Direction::Left) => (Direction::Down, None),
        (Tile::BackwardsMirror, Direction::Up) => (Direction::Left, None),
        (Tile::BackwardsMirror, Direction::Down) => (Direction::Right, None),
        (Tile::BackwardsMirror, Direction::Left) => (Direction::Up, None),
        (Tile::BackwardsMirror, Direction::Right) => (Direction::Down, None),
        (Tile::HorizSplitter, Direction::Up | Direction::Down) => {
            (Direction::Left, Some(Direction::Right))
        }
        (Tile::HorizSplitter, Direction::Left | Direction::Right) => (direction, None),
        (Tile::VertSplitter, Direction::Up | Direction::Down) => (direction, None),
        (Tile::VertSplitter, Direction::Left | Direction::Right) => {
            (Direction::Up, Some(Direction::Down))
        }
        (Tile::Nothing, _) => (direction, None),
    }
}

#[derive(Debug, Clone)]
struct Beam {
    r: usize,
    c: usize,
    dir: Direction,
}

#[derive(Debug, Clone)]
struct Map {
    tiles: Box<[Box<[Tile]>]>,
    w: usize,
    h: usize,

    energized: HashSet<(usize, usize)>,
    visited: HashSet<(usize, usize, Direction)>,
    beams: Vec<Beam>,
}
impl FromStr for Map {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let tiles: Box<[Box<[Tile]>]> = s
            .lines()
            .map(|line| {
                line.chars()
                    .map(|ch| match ch {
                        '.' => Tile::Nothing,
                        '|' => Tile::VertSplitter,
                        '-' => Tile::HorizSplitter,
                        '/' => Tile::ForwardsMirror,
                        '\\' => Tile::BackwardsMirror,
                        _ => panic!(),
                    })
                    .collect()
            })
            .collect();

        let w = tiles[0].len();
        let h = tiles.len();

        Ok(Map {
            tiles,
            w,
            h,
            energized: HashSet::new(),
            visited: HashSet::new(),
            beams: Vec::new(),
        })
    }
}
impl Display for Map {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for (r, row) in self.tiles.iter().enumerate() {
            for (c, tile) in row.iter().enumerate() {
                match tile {
                    Tile::ForwardsMirror => write!(f, "╱")?,
                    Tile::BackwardsMirror => write!(f, "╲")?,
                    Tile::HorizSplitter => write!(f, "━")?,
                    Tile::VertSplitter => write!(f, "│")?,
                    Tile::Nothing => {
                        if self.energized.contains(&(r, c)) {
                            write!(f, "#")?;
                        } else {
                            write!(f, ".")?;
                        }
                    }
                }
            }
            writeln!(f)?;
        }

        Ok(())
    }
}
impl Map {
    fn clone_with_clear_tiles(&self) -> Self {
        let mut result = self.clone();
        for row in result.tiles.iter_mut() {
            for tile in row.iter_mut() {
                *tile = Tile::Nothing;
            }
        }
        result
    }

    fn add_initial_beam(&mut self, r: usize, c: usize, dir: Direction) {
        self.energized.insert((r, c));

        let (dir, dir2) = next_direction(self.tiles[r][c], dir);
        self.beams.push(Beam { r, c, dir });
        if let Some(dir2) = dir2 {
            self.beams.push(Beam { r, c, dir: dir2 });
        }
    }

    fn all_edges(&self) -> Vec<(usize, usize, Direction)> {
        let mut result = Vec::new();
        // top
        result.extend((0..self.w).map(|c| (0, c, Direction::Down)));
        // bottom
        result.extend((0..self.w).map(|c| (self.h - 1, c, Direction::Up)));
        // left
        result.extend((0..self.h).map(|r| (r, 0, Direction::Right)));
        // right
        result.extend((0..self.h).map(|r| (r, self.w - 1, Direction::Left)));

        result
    }

    // true when all beams are done
    fn tick(&mut self) -> bool {
        let mut finished_beams = Vec::new();
        let mut new_beams = Vec::<Beam>::new();

        for (idx, beam) in self.beams.iter_mut().enumerate() {
            if let Some((nr, nc)) = beam.dir.try_move(beam.r, beam.c, self.w, self.h) {
                beam.r = nr;
                beam.c = nc;

                self.energized.insert((nr, nc));

                let (new_dir, new_dir2) = next_direction(self.tiles[nr][nc], beam.dir);
                beam.dir = new_dir;

                let already_seen_new_pos = !self.visited.insert((beam.r, beam.c, beam.dir));
                if already_seen_new_pos {
                    finished_beams.push(idx);
                }

                if let Some(new_dir2) = new_dir2 {
                    let mut new_beam = beam.clone();
                    new_beam.dir = new_dir2;
                    let already_seen_new_beam =
                        !self.visited.insert((new_beam.r, new_beam.c, new_beam.dir));

                    if !already_seen_new_beam {
                        new_beams.push(new_beam);
                    }
                }
            } else {
                finished_beams.push(idx);
            }
        }

        // remove the beams in opposite order
        for idx in finished_beams.into_iter().rev() {
            self.beams.swap_remove(idx);
        }
        for new_beam in new_beams {
            self.beams.push(new_beam);
        }

        self.beams.is_empty()
    }
}

fn main() {
    let map: Map = std::fs::read_to_string("input")
        .unwrap()
        .parse()
        .unwrap();

    let mut p1_map = map.clone();
    p1_map.add_initial_beam(0, 0, Direction::Right);

    println!("{p1_map}");
    while !p1_map.tick() {}

    // println!("{}", p1_map.clone_with_clear_tiles());
    println!("{}", p1_map.energized.len());

    let (p2, (_r, _c, _dir)) = map
        .all_edges()
        .into_iter()
        .map(|(r, c, dir)| {
            let mut map = map.clone();
            map.add_initial_beam(r, c, dir);

            while !map.tick() {}

            (map.energized.len(), (r, c, dir))
        })
        .max()
        .unwrap();

    // let mut test_map = map.clone();
    // test_map.add_initial_beam(r, c, dir);
    // while !test_map.tick() {}
    // println!("{}", test_map.clone_with_clear_tiles());

    println!("{p2:?}");
}
