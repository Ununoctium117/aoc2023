use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashMap, HashSet},
    fmt::Display,
    rc::Rc,
    str::FromStr,
};

enum Either4<T1, T2, T3, T4> {
    A(T1),
    B(T2),
    C(T3),
    D(T4),
}
impl<I, T1, T2, T3, T4> Iterator for Either4<T1, T2, T3, T4>
where
    T1: Iterator<Item = I>,
    T2: Iterator<Item = I>,
    T3: Iterator<Item = I>,
    T4: Iterator<Item = I>,
{
    type Item = I;

    fn next(&mut self) -> Option<Self::Item> {
        use Either4::*;
        match self {
            A(a) => a.next(),
            B(b) => b.next(),
            C(c) => c.next(),
            D(d) => d.next(),
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}
impl Direction {
    fn all() -> impl Iterator<Item = Direction> {
        use Direction::*;
        [Up, Down, Left, Right].into_iter()
    }

    fn all_except_reverse(&self) -> impl Iterator<Item = Direction> {
        let copy = self.clone();
        Self::all().filter(move |x| (*x != copy.reverse()))
    }

    fn others_except_reverse(&self) -> impl Iterator<Item = Direction> {
        let copy = self.clone();
        self.all_except_reverse().filter(move |x| (*x != copy))
    }

    fn reverse(&self) -> Direction {
        use Direction::*;
        match self {
            Up => Down,
            Down => Up,
            Left => Right,
            Right => Left,
        }
    }

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

// ordering and equality only check current_cost
#[derive(Clone, Debug)]
struct State {
    tiles: Rc<Vec<Vec<u32>>>,
    cost: u32,
    position: (usize, usize),
    direction: Option<Direction>,
    moves_in_direction: u32,
}
impl PartialEq for State {
    fn eq(&self, other: &Self) -> bool {
        self.cost == other.cost
    }
}
impl Eq for State {}
impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.cost.partial_cmp(&other.cost)
    }
}
impl Ord for State {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.partial_cmp(other).unwrap()
    }
}
impl State {
    fn next_states(
        self,
        min_moves_before_turn: u32,
        max_moves_before_turn: u32,
    ) -> impl Iterator<Item = State> {
        let (cur_r, cur_c) = self.position;
        let width = self.tiles[0].len();
        let height = self.tiles.len();

        match (self.direction, self.moves_in_direction) {
            (None, _) => Either4::A(Direction::all()),
            (Some(dir), num) => {
                if num < min_moves_before_turn {
                    Either4::B(std::iter::once(dir))
                } else if num < max_moves_before_turn {
                    Either4::C(dir.all_except_reverse())
                } else {
                    Either4::D(dir.others_except_reverse())
                }
            }
        }
        .filter_map(move |possible_dir| {
            Some((
                possible_dir,
                possible_dir.try_move(cur_r, cur_c, width, height)?,
            ))
        })
        .map(move |(new_dir, (new_r, new_c))| {
            let mut new_state = self.clone();

            new_state.cost += new_state.tiles[new_r][new_c];
            new_state.position = (new_r, new_c);
            new_state.direction = Some(new_dir);
            new_state.moves_in_direction = if Some(new_dir) == self.direction {
                self.moves_in_direction + 1
            } else {
                1
            };

            new_state
        })
    }
}
impl FromStr for State {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let tiles = Rc::new(
            s.lines()
                .map(|line| line.chars().map(|ch| ch.to_digit(10).unwrap()).collect())
                .collect(),
        );

        Ok(State {
            tiles,
            cost: 0,
            position: (0, 0),
            direction: None,
            moves_in_direction: 0,
        })
    }
}
impl Display for State {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in self.tiles.iter() {
            for cost in row {
                write!(f, "{}", *cost)?;
            }
            writeln!(f)?;
        }

        Ok(())
    }
}

fn find_cheapest_path(
    initial_state: State,
    dest: (usize, usize),
    min_moves_before_turn: u32,
    max_moves_before_turn: u32,
) -> u32 {
    let mut positions_to_costs = HashMap::new();
    positions_to_costs.insert(initial_state.position, 0);

    let mut states_to_search = BinaryHeap::new();
    states_to_search.push(Reverse(initial_state)); // reverse since this is a max-heap

    let mut visited = HashSet::new();

    // dijkstra: queue is the inverse of "visited"
    while let Some(current_state) = states_to_search.pop() {
        let cur_pos = (
            current_state.0.position,
            current_state.0.direction,
            current_state.0.moves_in_direction,
        );

        // don't check the same position more than once
        if !visited.insert(cur_pos) {
            continue;
        }

        if cur_pos.0 == dest && cur_pos.2 >= min_moves_before_turn {
            return *positions_to_costs.get(&dest).unwrap();
        }

        for new_state in current_state
            .0
            .next_states(min_moves_before_turn, max_moves_before_turn)
        {
            match positions_to_costs.get_mut(&new_state.position) {
                Some(existing_cost) if new_state.cost < *existing_cost => {
                    // update
                    *existing_cost = new_state.cost;
                }
                None => {
                    // update
                    positions_to_costs.insert(new_state.position, new_state.cost);
                }
                _ => {}
            }

            if !visited.contains(&(
                new_state.position,
                new_state.direction,
                new_state.moves_in_direction,
            )) {
                states_to_search.push(Reverse(new_state));
            }
        }
    }

    panic!()
}

fn main() {
    let initial_state: State = std::fs::read_to_string("input")
        .unwrap()
        .parse()
        .unwrap();

    let dest = (
        initial_state.tiles[0].len() - 1,
        initial_state.tiles.len() - 1,
    );

    println!("{}", find_cheapest_path(initial_state.clone(), dest, 0, 3));
    println!("{}", find_cheapest_path(initial_state, dest, 4, 10));
}
