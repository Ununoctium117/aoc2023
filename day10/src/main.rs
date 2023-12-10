#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Tile {
    Vertical,
    Horizontal,
    BottomLeft,
    BottomRight,
    TopLeft,
    TopRight,
    Nothing,
    UnkStarting,
}
impl Tile {
    fn can_link(self, direction: Direction, other: Tile) -> bool {
        self.connects_in_direction(direction) && other.connects_in_direction(direction.opposite())
    }

    // direction from self to possible other
    fn connects_in_direction(self, direction: Direction) -> bool {
        use Direction::*;
        use Tile::*;

        match (self, direction) {
            (Vertical, Up) => true,
            (Vertical, Down) => true,
            (Horizontal, Left) => true,
            (Horizontal, Right) => true,
            (BottomLeft, Up) => true,
            (BottomLeft, Right) => true,
            (BottomRight, Up) => true,
            (BottomRight, Left) => true,
            (TopLeft, Down) => true,
            (TopLeft, Right) => true,
            (TopRight, Down) => true,
            (TopRight, Left) => true,
            (UnkStarting, _) => true,
            (_, _) => false,
        }
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}
impl Direction {
    fn opposite(self) -> Self {
        use Direction::*;

        match self {
            Up => Down,
            Down => Up,
            Left => Right,
            Right => Left,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum EncloseResult {
    OnMainLoop,
    Enclosed,
    NotEnclosed,
}

#[derive(Debug)]
struct TileMap {
    rows: Vec<Tile>,
    width: usize,
    height: usize,
    start: usize,
}
impl TileMap {
    fn print(&self) {
        let mut main_loop = self.get_loop();
        main_loop.sort();

        for r in 0..self.height {
            for c in 0..self.width {
                let pos = r * self.width + c;

                if main_loop.binary_search(&pos).is_ok() {
                    print!(
                        "{}",
                        match self.rows[pos] {
                            Tile::Vertical => '│',
                            Tile::Horizontal => '─',
                            Tile::BottomLeft => '└',
                            Tile::BottomRight => '┘',
                            Tile::TopLeft => '┌',
                            Tile::TopRight => '┐',
                            Tile::Nothing => panic!(),
                            Tile::UnkStarting => 'S',
                        }
                    );
                } else {
                    print!(
                        "{}",
                        if self.is_enclosed(pos, &main_loop) == EncloseResult::Enclosed {
                            '*'
                        } else {
                            ' '
                        }
                    );
                }
            }
            println!();
        }
    }

    fn resolve_start(&mut self) {
        use Direction::*;
        let real_start =
            match &[Up, Down, Left, Right].map(|d| self.try_follow_link(self.start, d).is_some()) {
                &[true, true, false, false] => Tile::Vertical,
                &[true, false, true, false] => Tile::BottomRight,
                &[true, false, false, true] => Tile::BottomLeft,
                &[false, true, true, false] => Tile::TopRight,
                &[false, true, false, true] => Tile::TopLeft,
                &[false, false, true, true] => Tile::Horizontal,
                _ => panic!(),
            };

        self.rows[self.start] = real_start;
    }

    fn get_loop(&self) -> Vec<usize> {
        use Direction::*;

        let mut result = Vec::new();
        let mut cur_pos = self.start;
        let mut last_dir = Direction::Up;
        'find_next: while result.is_empty() || cur_pos != self.start {
            for dir in &[Up, Down, Left, Right] {
                if *dir != last_dir.opposite() {
                    if let Some(new_pos) = self.try_follow_link(cur_pos, *dir) {
                        result.push(cur_pos);
                        last_dir = *dir;
                        cur_pos = new_pos;
                        continue 'find_next;
                    }
                }
            }

            panic!("loop broken: {cur_pos} {last_dir:?}");
        }

        result
    }

    fn count_enclosed_tiles(&self, ordered_main_loop: &[usize]) -> usize {
        let mut result = 0;

        for i in 0..self.rows.len() {
            if ordered_main_loop.binary_search(&i).is_ok() {
                continue;
            }

            if self.is_enclosed(i, ordered_main_loop) == EncloseResult::Enclosed {
                result += 1;
            }
        }

        result
    }

    fn is_enclosed(&self, pos: usize, ordered_main_loop: &[usize]) -> EncloseResult {
        if ordered_main_loop.binary_search(&pos).is_ok() {
            return EncloseResult::OnMainLoop;
        }

        let mut is_enclosed = false;
        let mut cur_pos = pos;
        while let Some(new_pos) = self.move_in_dir(cur_pos, Direction::Left) {
            if ordered_main_loop.binary_search(&new_pos).is_ok() {
                let tile = self.rows[new_pos];
                if tile == Tile::Vertical || tile == Tile::BottomLeft || tile == Tile::BottomRight {
                    is_enclosed = !is_enclosed;
                }
            }
            cur_pos = new_pos;
        }

        if is_enclosed {
            EncloseResult::Enclosed
        } else {
            EncloseResult::NotEnclosed
        }
    }

    fn move_in_dir(&self, pos: usize, dir: Direction) -> Option<usize> {
        let (r, c) = (pos / self.width, pos % self.width);
        let (r, c) = match dir {
            Direction::Up => (r.checked_sub(1)?, c),
            Direction::Down => (r.checked_add(1)?, c),
            Direction::Left => (r, c.checked_sub(1)?),
            Direction::Right => (r, c.checked_add(1)?),
        };
        if r >= self.height || c >= self.width {
            None
        } else {
            Some(r * self.width + c)
        }
    }

    fn try_follow_link(&self, pos: usize, dir: Direction) -> Option<usize> {
        let new_pos = self.move_in_dir(pos, dir)?;
        self.rows[pos]
            .can_link(dir, self.rows[new_pos])
            .then_some(new_pos)
    }
}

fn main() {
    let mut start = None;
    let mut width = None;
    let rows = std::fs::read_to_string("input")
        .unwrap()
        .lines()
        .enumerate()
        .map(|(row, line)| {
            let result: Vec<_> = line
                .chars()
                .enumerate()
                .map(|(col, ch)| match ch {
                    '|' => Tile::Vertical,
                    '-' => Tile::Horizontal,
                    'L' => Tile::BottomLeft,
                    'J' => Tile::BottomRight,
                    '7' => Tile::TopRight,
                    'F' => Tile::TopLeft,
                    '.' => Tile::Nothing,
                    'S' => {
                        start = Some(row * width.unwrap_or(0) + col);
                        Tile::UnkStarting
                    }
                    x => panic!("{row} {col} {x}"),
                })
                .collect();

            width = Some(result.len());

            result
        })
        .flatten()
        .collect::<Vec<Tile>>();

    let rows_len = rows.len();
    let mut map = TileMap {
        rows,
        width: width.unwrap(),
        height: rows_len / width.unwrap(),
        start: start.unwrap(),
    };

    map.resolve_start();
    map.print();

    let mut main_loop = map.get_loop();
    println!("{}", main_loop.len() / 2);

    main_loop.sort();
    println!("{}", map.count_enclosed_tiles(&main_loop));
}
