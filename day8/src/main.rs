use std::collections::{HashMap, HashSet};

enum Move {
    Left,
    Right,
}

fn lcm(nums: &[u64]) -> u64 {
    if nums.len() == 1 {
        nums[0]
    } else {
        let others = lcm(&nums[1..]);
        nums[0] * others / gcd(nums[0], others)
    }
}
fn gcd(a: u64, b: u64) -> u64 {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}

fn main() {
    let inp = std::fs::read_to_string("input").unwrap();
    let mut parts = inp.split("\n\n");
    let mut moves = parts
        .next()
        .unwrap()
        .trim()
        .chars()
        .map(|c| match c {
            'L' => Move::Left,
            'R' => Move::Right,
            _ => panic!(),
        })
        .cycle();

    let mut name_to_id = HashMap::new();
    let mut unresolved_lines = Vec::new();
    let mut start_id = None;
    let mut ghost_ids = Vec::new();
    let mut goal_id = None;
    let mut ghost_goals = HashSet::new();
    for (id, line) in parts.next().unwrap().lines().enumerate() {
        let (name, neighbors) = line.split_at(line.find(" = ").unwrap());
        let neighbors = neighbors
            .strip_prefix(" = (")
            .unwrap()
            .strip_suffix(")")
            .unwrap();
        let (left, right) = neighbors.split_at(neighbors.find(", ").unwrap());
        let right = right.strip_prefix(", ").unwrap();

        name_to_id.insert(name.to_string(), id);
        unresolved_lines.push((left, right));
        assert_eq!(id, unresolved_lines.len() - 1);

        if name == "AAA" {
            start_id = Some(id);
        }
        if name.ends_with('A') {
            ghost_ids.push(id);
        }

        if name == "ZZZ" {
            goal_id = Some(id);
        }
        if name.ends_with('Z') {
            ghost_goals.insert(id);
        }
    }

    let mut current_id = start_id.unwrap();
    let goal_id = goal_id.unwrap();

    let mut left_paths = Vec::new();
    let mut right_paths = Vec::new();
    for (left, right) in unresolved_lines {
        left_paths.push(*name_to_id.get(left).expect("unresolved name"));
        right_paths.push(*name_to_id.get(right).expect("unresolved name"));
    }

    let mut steps = 0u64;
    let mut normal_path_steps = None;
    let mut ghost_path_steps = ghost_ids.iter().map(|_| None).collect::<Vec<Option<u64>>>();
    while normal_path_steps.is_none() || ghost_path_steps.iter().any(|x| x.is_none()) {
        let next_move = moves.next().unwrap();
        steps += 1;

        current_id = match next_move {
            Move::Left => left_paths[current_id],
            Move::Right => right_paths[current_id],
        };
        if current_id == goal_id && normal_path_steps.is_none() {
            normal_path_steps = Some(steps);
        }

        for (ghost_id, ghost_steps) in ghost_ids.iter_mut().zip(ghost_path_steps.iter_mut()) {
            *ghost_id = match next_move {
                Move::Left => left_paths[*ghost_id],
                Move::Right => right_paths[*ghost_id],
            };
            if ghost_goals.contains(ghost_id) && ghost_steps.is_none() {
                *ghost_steps = Some(steps);
            }
        }
    }

    println!("{}", normal_path_steps.unwrap());

    let ghost_steps = ghost_path_steps
        .into_iter()
        .collect::<Option<Vec<u64>>>()
        .unwrap();
    dbg!(&ghost_steps);
    let result = lcm(&ghost_steps[..]);
    println!("{result}");
}
