use std::{
    collections::{HashMap, HashSet},
    str::FromStr,
};

#[derive(Debug, Clone, Copy)]
struct Brick {
    x: u64,
    l: u64,

    y: u64,
    w: u64,

    z: u64,
    h: u64,
}
impl Brick {
    fn occupied_locations(&self) -> impl Iterator<Item = (u64, u64, u64)> {
        let clone = self.clone();
        (clone.x..clone.x + clone.l).flat_map(move |x| {
            (clone.y..clone.y + clone.w)
                .flat_map(move |y| (clone.z..clone.z + clone.h).map(move |z| (x, y, z)))
        })
    }
}
impl FromStr for Brick {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (c1, c2) = s.split_at(s.find('~').unwrap());
        let c2 = c2.strip_prefix('~').unwrap();

        let c1: Vec<u64> = c1.split(',').map(|num| num.parse().unwrap()).collect();
        let c2: Vec<u64> = c2.split(',').map(|num| num.parse().unwrap()).collect();

        Ok(Self {
            x: c1[0].min(c2[0]),
            l: c1[0].abs_diff(c2[0]) + 1,

            y: c1[1].min(c2[1]),
            w: c1[1].abs_diff(c2[1]) + 1,

            z: c1[2].min(c2[2]),
            h: c1[2].abs_diff(c2[2]) + 1,
        })
    }
}

#[derive(Debug, Default, Clone)]
struct World {
    bricks: Vec<Brick>,
    occupied_locations: HashMap<(u64, u64, u64), usize>,
    max_x: u64,
    max_y: u64,
    max_z: u64,
}
impl World {
    fn add_brick(&mut self, brick: Brick) {
        let idx = self.bricks.len();
        for occupied_loc in brick.occupied_locations() {
            self.max_x = self.max_x.max(occupied_loc.0);
            self.max_y = self.max_y.max(occupied_loc.1);
            self.max_z = self.max_z.max(occupied_loc.2);

            assert!(self.occupied_locations.insert(occupied_loc, idx).is_none());
        }
        self.bricks.push(brick);
    }

    fn is_supported(&self, brick_idx: usize, ignoring: Option<usize>) -> bool {
        self.bricks[brick_idx].occupied_locations().any(|loc| {
            let lower_loc = (loc.0, loc.1, loc.2 - 1);

            if lower_loc.2 == 0 {
                true
            } else if let Some(other_brick) = self.occupied_locations.get(&lower_loc) {
                if *other_brick == brick_idx {
                    false
                } else if Some(*other_brick) == ignoring {
                    false
                } else {
                    true
                }
            } else {
                false
            }
        })
    }

    fn get_supported(&self, brick_idx: usize) -> HashSet<usize> {
        self.bricks[brick_idx]
            .occupied_locations()
            .filter_map(|loc| {
                let upper_loc = (loc.0, loc.1, loc.2 + 1);

                if let Some(other_brick) = self.occupied_locations.get(&upper_loc) {
                    if *other_brick != brick_idx {
                        Some(*other_brick)
                    } else {
                        None
                    }
                } else {
                    None
                }
            })
            .collect()
    }

    fn settle(&mut self) -> usize {
        // walk all the bricks, trying to make them fall one-by-one, until no more move
        let mut all_moved_bricks = HashSet::new();
        loop {
            let mut bricks_that_can_fall = Vec::new();

            for idx in 0..self.bricks.len() {
                if !self.is_supported(idx, None) {
                    bricks_that_can_fall.push(idx);
                }
            }

            if bricks_that_can_fall.is_empty() {
                break;
            } else {
                for brick_idx in bricks_that_can_fall {
                    all_moved_bricks.insert(brick_idx);
                    self.move_brick_down(brick_idx);
                }
            }
        }

        all_moved_bricks.len()
    }

    fn num_settling_without(&self, ignoring: usize) -> usize {
        let mut dummy = self.clone();
        for location in dummy.bricks[ignoring].occupied_locations() {
            dummy.occupied_locations.remove(&location).unwrap();
        }

        dummy.settle()
    }

    fn move_brick_down(&mut self, idx: usize) {
        // "unoccupy" all of the brick's spots
        for occupied_loc in self.bricks[idx].occupied_locations() {
            self.occupied_locations.remove(&occupied_loc).unwrap();
        }

        self.bricks[idx].z -= 1;

        for occupied_loc in self.bricks[idx].occupied_locations() {
            assert!(self.occupied_locations.insert(occupied_loc, idx).is_none());
        }
    }

    fn count_non_structural_bricks(&self) -> usize {
        (0..self.bricks.len())
            .filter(|idx| self.num_settling_without(*idx) == 0)
            .count()
    }

    fn print_xz(&self) {
        let mut buf = "xz map:\n".to_string();
        for z in (1..=self.max_z).rev() {
            for x in 0..=self.max_x {
                let occupied_by = (0..=self.max_y)
                    .filter_map(|y| self.occupied_locations.get(&(x, y, z)))
                    .collect::<Vec<_>>();

                match occupied_by.len() {
                    0 => buf.push('.'),
                    _ => buf.push_str(&occupied_by[0].to_string()),
                    // _ => buf.push('?'),
                }
            }
            buf.push('\n');
        }

        for _ in 0..=self.max_x {
            buf.push('-');
        }

        println!("{buf}");
    }

    fn print_yz(&self) {
        let mut buf = "yz map:\n".to_string();
        for z in (1..=self.max_z).rev() {
            for y in 0..=self.max_y {
                let occupied_by = (0..=self.max_x)
                    .filter_map(|x| self.occupied_locations.get(&(x, y, z)))
                    .collect::<Vec<_>>();

                match occupied_by.len() {
                    0 => buf.push('.'),
                    _ => buf.push_str(&occupied_by[0].to_string()),
                    // _ => buf.push('?'),
                }
            }
            buf.push('\n');
        }

        for _ in 0..=self.max_y {
            buf.push('-');
        }

        println!("{buf}");
    }
}

fn main() {
    let bricks: Vec<Brick> = std::fs::read_to_string("input")
        .unwrap()
        .lines()
        .map(|line| line.parse().unwrap())
        .collect();

    let mut world = World::default();

    for brick in bricks {
        world.add_brick(brick);
    }

    world.settle();

    dbg!(world.count_non_structural_bricks());

    let result2: usize = (0..world.bricks.len())
        .map(|idx| world.num_settling_without(idx))
        .sum();

    dbg!(result2);
}
