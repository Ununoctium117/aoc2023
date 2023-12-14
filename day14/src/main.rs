use std::{cmp::Ordering, collections::HashMap, fmt::Display, str::FromStr};

#[derive(Hash, PartialEq, Eq, Clone)]
struct Map {
    round_rock_pos: Vec<usize>,
    cube_pos: Vec<usize>,
    width: usize,
    height: usize,
    bound: usize,
}
impl Map {
    // offset: how to move the rock
    // comparator: comparison function that will order "earlier" moving rocks before "later" ones
    // ok_move: given (old rock pos, new rock pos), check if new rock pos is acceptable
    fn roll(
        &mut self,
        offset: isize,
        comparator: impl Fn(&usize, &usize) -> Ordering,
        ok_move: impl Fn(usize, usize) -> bool,
    ) -> bool {
        let mut map_changed = false;

        self.round_rock_pos.sort_unstable_by(&comparator);

        for rock_idx in 0..self.round_rock_pos.len() {
            let mut rock_changed = false;

            let mut rock_pos = self.round_rock_pos[rock_idx];
            while let Some(new_pos) = rock_pos.checked_add_signed(offset) {
                if ok_move(rock_pos, new_pos) && !self.is_blocked(new_pos, &comparator) {
                    rock_pos = new_pos;
                    rock_changed = true;
                } else {
                    break;
                }
            }

            if rock_changed {
                map_changed = true;
                self.round_rock_pos[rock_idx] = rock_pos;
                self.round_rock_pos.sort_unstable_by(&comparator);
            }
        }

        map_changed
    }

    fn roll_north(&mut self) {
        while self.roll(-(self.width as isize), |x, y| x.cmp(y), |_, _| true) {}
    }

    fn roll_south(&mut self) {
        while self.roll(self.width as isize, |x, y| y.cmp(x), |_, _| true) {}
    }

    fn roll_west(&mut self) {
        let width = self.width;
        while self.roll(
            -1,
            |x, y| {
                // column first comparison
                let (xr, xc) = (x / width, x % width);
                let (yr, yc) = (y / width, y % width);

                xc.cmp(&yc).then(xr.cmp(&yr))
            },
            |old_pos, new_pos| (old_pos / width) == (new_pos / width),
        ) {}
    }

    fn roll_east(&mut self) {
        let width = self.width;
        while self.roll(
            1,
            |x, y| {
                // column first comparison
                let (xr, xc) = (x / width, x % width);
                let (yr, yc) = (y / width, y % width);

                yc.cmp(&xc).then(yr.cmp(&xr))
            },
            |old_pos, new_pos| (old_pos / width) == (new_pos / width),
        ) {}
    }

    fn cycle(&mut self) {
        self.roll_north();
        self.roll_west();
        self.roll_south();
        self.roll_east();
    }

    fn is_blocked(&self, idx: usize, comparator: impl Fn(&usize, &usize) -> Ordering) -> bool {
        if idx >= self.bound {
            true
        } else {
            self.round_rock_pos
                .binary_search_by(|probe| comparator(probe, &idx))
                .is_ok()
                || self.cube_pos.binary_search(&idx).is_ok() // cube positions aren't reordered
        }
    }

    fn get_load(&self) -> usize {
        self.round_rock_pos
            .iter()
            .map(|pos| {
                let row = *pos / self.width;
                self.height - row
            })
            .sum()
    }
}
impl FromStr for Map {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut round_rock_pos = Vec::new();
        let mut cube_pos = Vec::new();
        let width = s.find('\n').unwrap();
        let height = s.lines().count();

        for (r, line) in s.lines().enumerate() {
            for (c, ch) in line.chars().enumerate() {
                match ch {
                    'O' | '0' => round_rock_pos.push(r * width + c),
                    '#' => cube_pos.push(r * width + c),
                    '.' => {}
                    _ => panic!(),
                }
            }
        }

        Ok(Map {
            round_rock_pos,
            cube_pos,
            width,
            height,
            bound: width * height,
        })
    }
}
impl Display for Map {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut sorted_rock_pos = self.round_rock_pos.clone();
        sorted_rock_pos.sort();

        for r in 0..self.height {
            for c in 0..self.width {
                let idx = r * self.width + c;
                if sorted_rock_pos.binary_search(&idx).is_ok() {
                    write!(f, "O")?;
                } else if self.cube_pos.binary_search(&idx).is_ok() {
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
    let mut map: Map = std::fs::read_to_string("input").unwrap().parse().unwrap();

    let mut p1_map = map.clone();
    p1_map.roll_north();
    println!("{}", p1_map.get_load());

    // Find a cycle in the cycles
    let (cycle_cycle_length, mut current_cycle_count) = {
        let mut maps = HashMap::new();
        let mut cycles = 0usize;
        loop {
            map.cycle();
            cycles += 1;

            if cycles % 10 == 0 {
                dbg!(cycles);
            }

            if let Some(old_cycle_count) = maps.insert(map.clone(), cycles) {
                // value was already in the map
                let cycle_length = cycles - old_cycle_count;
                break (cycle_length, cycles);
            }
        }
    };

    // println!("map after {current_cycle_count} cycles:\n{map}");

    for _ in 0..cycle_cycle_length {
        map.cycle();
        current_cycle_count += 1;
    }
    // println!("identical map after {current_cycle_count} cycles:\n{map}");

    // println!("skipping ahead by {cycle_cycle_length} at a time...");
    while current_cycle_count < 1_000_000_000 - cycle_cycle_length {
        // skip the next set of rounds
        current_cycle_count += cycle_cycle_length;
    }

    // dbg!(current_cycle_count);

    while current_cycle_count != 1_000_000_000 {
        map.cycle();
        current_cycle_count += 1;
    }

    // println!("map after {current_cycle_count} cycles:\n{map}");
    println!("{}", map.get_load());
}
