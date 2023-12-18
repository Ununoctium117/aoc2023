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

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
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

    fn dir_from_move((old_r, old_c): (usize, usize), (new_r, new_c): (usize, usize)) -> Self {
        Self::all()
            .filter(|possible| {
                possible.try_move(old_r, old_c, usize::MAX, usize::MAX) == Some((new_r, new_c))
            })
            .next()
            .unwrap()
    }

    fn char(&self) -> &'static str {
        match self {
            Direction::Up => "↑",
            Direction::Down => "↓",
            Direction::Left => "←",
            Direction::Right => "→",
        }
    }
}

struct Map {
    tiles: Vec<Vec<u32>>,
}
impl FromStr for Map {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let tiles = s
            .lines()
            .map(|line| line.chars().map(|ch| ch.to_digit(10).unwrap()).collect())
            .collect();

        Ok(Map { tiles })
    }
}
impl Display for Map {
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

// ordering and equality only check current_cost
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct State {
    position: (usize, usize),
    direction: Option<Direction>,
    moves_in_direction: u32,
}
impl State {
    fn next_states(
        &self,
        map: &Rc<Map>,
        current_cost: u32,
        min_moves_before_turn: u32,
        max_moves_before_turn: u32,
    ) -> impl Iterator<Item = (State, u32)> {
        let (cur_r, cur_c) = self.position;
        let map_clone = Rc::clone(map);
        let self_clone = self.clone();
        let width = map.tiles[0].len();
        let height = map.tiles.len();

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
            let mut new_state = self_clone.clone();

            new_state.position = (new_r, new_c);
            new_state.direction = Some(new_dir);
            new_state.moves_in_direction = if Some(new_dir) == self_clone.direction {
                self_clone.moves_in_direction + 1
            } else {
                1
            };

            (new_state, current_cost + map_clone.tiles[new_r][new_c])
        })
    }
}

fn find_cheapest_path(
    map: &Rc<Map>,
    initial_state: State,
    dest: (usize, usize),
    min_moves_before_turn: u32,
    max_moves_before_turn: u32,
) -> u32 {
    let width = map.tiles[0].len();

    let mut state_to_cost = HashMap::new();
    state_to_cost.insert(initial_state.clone(), 0);

    let mut state_queue = BinaryHeap::new();
    state_queue.push(Reverse((0, initial_state)));

    let mut previous_states = HashMap::new();
    let mut visited = HashSet::new();

    let print_path_to = |state: &State, previous_states: &HashMap<State, State>| {
        let mut dbg_string = std::iter::repeat((0..width).map(|_| " ").collect::<String>())
            .take(map.tiles.len())
            .collect::<Vec<_>>()
            .join("\n");
        let mut state = state.clone();
        while state.position != (0, 0) {
            let prev_state = previous_states.get(&state).unwrap();

            let idx = state.position.0 * (width + 1) + state.position.1;
            let (idx1, _) = dbg_string.char_indices().nth(idx).unwrap();
            let (idx2, _) = dbg_string
                .char_indices()
                .nth(idx + 1)
                .unwrap_or_else(|| (dbg_string.len(), ' '));
            dbg_string.replace_range(
                idx1..idx2,
                Direction::dir_from_move(prev_state.position, state.position).char(),
            );

            state = prev_state.clone();
        }
        println!("{}", dbg_string);
    };

    // dijkstra: queue is the inverse of "visited"
    while let Some(Reverse((current_cost, current_state))) = state_queue.pop() {
        // don't check the same position more than once
        if !visited.insert(current_state.clone()) {
            continue;
        }

        if current_state.position == dest
            && current_state.moves_in_direction >= min_moves_before_turn
        {
            print_path_to(&current_state, &previous_states);
            return current_cost;
        }

        for (new_state, new_state_cost) in current_state.next_states(
            map,
            current_cost,
            min_moves_before_turn,
            max_moves_before_turn,
        ) {
            match state_to_cost.get_mut(&new_state) {
                Some(existing_cost) if new_state_cost < *existing_cost => {
                    // update
                    *existing_cost = new_state_cost;
                    previous_states.insert(new_state.clone(), current_state.clone());
                }
                None => {
                    // update
                    state_to_cost.insert(new_state.clone(), new_state_cost);
                    previous_states.insert(new_state.clone(), current_state.clone());
                }
                _ => {}
            }

            if !visited.contains(&new_state) {
                state_queue.push(Reverse((new_state_cost, new_state)));
            }
        }
    }

    panic!()
}

fn main() {
    let map = Rc::<Map>::new(
        std::fs::read_to_string("input")
            .unwrap()
            .parse()
            .unwrap(),
    );
    let initial_state = State {
        position: (0, 0),
        direction: None,
        moves_in_direction: 0,
    };

    println!("{map}");

    let dest = (map.tiles[0].len() - 1, map.tiles.len() - 1);

    println!(
        "{}",
        find_cheapest_path(&map, initial_state.clone(), dest, 0, 3)
    );
    println!("{}", find_cheapest_path(&map, initial_state, dest, 4, 10));
}
