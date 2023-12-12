#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
enum Pump {
    Working,
    Broken,
    Unknown,
}

// unknown is treated as working
fn produce_sequence(pumps: &[Pump], cutoff: usize) -> Vec<usize> {
    let mut counter = None;
    let mut sequence = Vec::new();

    for pump in &pumps[..cutoff] {
        match pump {
            Pump::Working | Pump::Unknown => {
                if let Some(counter) = counter.take() {
                    sequence.push(counter);
                }
            }
            Pump::Broken => *counter.get_or_insert(0) += 1,
        }
    }

    if let Some(counter) = counter.take() {
        sequence.push(counter);
    }

    sequence
}

fn get_combinations(pump_list: &[Pump], broken_pump_sequence: &[usize]) -> usize {
    let num_broken_pumps: usize = broken_pump_sequence.iter().sum();
    let known_broken_pumps = pump_list.iter().filter(|x| **x == Pump::Broken).count();
    let mut num_combinations = 0;
    let mut working_pump_list = Vec::from_iter(pump_list.iter().copied());
    let mut selected_indices = vec![0; num_broken_pumps.checked_sub(known_broken_pumps).unwrap()];
    let unknown_indices = pump_list
        .iter()
        .enumerate()
        .filter_map(|(idx, pump)| (*pump == Pump::Unknown).then_some(idx))
        .collect::<Vec<_>>();

    dbg!(selected_indices.len());

    get_larger_combinations(
        &mut working_pump_list,
        &broken_pump_sequence,
        &mut selected_indices,
        0,
        &unknown_indices,
        0,
        &mut num_combinations,
    );

    num_combinations
}

fn get_larger_combinations(
    working_pump_list: &mut Vec<Pump>,
    broken_pump_sequence: &[usize],

    selected_broken_indices: &mut [usize],
    selection_size_so_far: usize,

    unknown_indices: &[usize],
    next_unknown: usize,

    num_combinations: &mut usize,
) {
    if selection_size_so_far == selected_broken_indices.len() {
        if produce_sequence(&working_pump_list, working_pump_list.len()) == broken_pump_sequence {
            *num_combinations += 1;
        }
    } else {
        for j in next_unknown..unknown_indices.len() {
            selected_broken_indices[selection_size_so_far] = unknown_indices[j];
            let updated_selection_size = selection_size_so_far + 1;

            assert_eq!(working_pump_list[unknown_indices[j]], Pump::Unknown);
            working_pump_list[unknown_indices[j]] = Pump::Broken;

            let could_be_correct = if j + 1 >= unknown_indices.len() {
                let wip_sequence = produce_sequence(&working_pump_list, working_pump_list.len());
                wip_sequence == broken_pump_sequence
            } else {
                let wip_sequence = produce_sequence(&working_pump_list, unknown_indices[j + 1]);
                broken_pump_sequence
                    .iter()
                    .zip(wip_sequence.iter())
                    .all(|(expected, actual)| *expected >= *actual)
            };

            if could_be_correct {
                get_larger_combinations(
                    working_pump_list,
                    broken_pump_sequence,
                    selected_broken_indices,
                    updated_selection_size,
                    unknown_indices,
                    j + 1,
                    num_combinations,
                );
            }

            working_pump_list[unknown_indices[j]] = Pump::Unknown;
        }
    }
}

fn main() {
    let result1: usize = std::fs::read_to_string("input")
        .unwrap()
        .lines()
        .map(|line| {
            let mut parts = line.split_ascii_whitespace();
            let input = parts
                .next()
                .unwrap()
                .chars()
                .map(|ch| match ch {
                    '#' => Pump::Broken,
                    '.' => Pump::Working,
                    '?' => Pump::Unknown,
                    _ => panic!(),
                })
                .collect::<Vec<_>>();
            let damaged_sequence = parts
                .next()
                .unwrap()
                .split(",")
                .map(|x| x.parse::<usize>().unwrap())
                .collect::<Vec<_>>();

            get_combinations(&input, &damaged_sequence)
        })
        .sum();

    println!("{result1}");

    let result2: usize = std::fs::read_to_string("input")
        .unwrap()
        .lines()
        .enumerate()
        .map(|(line_num, line)| {
            let mut parts = line.split_ascii_whitespace();
            let input = parts
                .next()
                .unwrap()
                .chars()
                .map(|ch| match ch {
                    '#' => Pump::Broken,
                    '.' => Pump::Working,
                    '?' => Pump::Unknown,
                    _ => panic!(),
                })
                .collect::<Vec<_>>();

            let input: Vec<Pump> = itertools::Itertools::intersperse(
                [
                    input.clone(),
                    input.clone(),
                    input.clone(),
                    input.clone(),
                    input.clone(),
                ]
                .into_iter(),
                vec![Pump::Unknown],
            )
            .flatten()
            .collect();

            let damaged_sequence = parts
                .next()
                .unwrap()
                .split(",")
                .map(|x| x.parse::<usize>().unwrap())
                .collect::<Vec<_>>();

            let damaged_sequence: Vec<usize> = [
                damaged_sequence.clone(),
                damaged_sequence.clone(),
                damaged_sequence.clone(),
                damaged_sequence.clone(),
                damaged_sequence.clone(),
            ]
            .into_iter()
            .flatten()
            .collect();

            dbg!(line_num);
            dbg!(get_combinations(&input, &damaged_sequence))
        })
        .sum();

    println!("{result2}");
}
