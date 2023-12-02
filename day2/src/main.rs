use std::{iter::Sum, str::FromStr};

#[derive(Debug, Clone, Default)]
struct Pull {
    red: u32,
    green: u32,
    blue: u32,
}
impl Pull {
    fn power(&self) -> u32 {
        self.red * self.green * self.blue
    }
}
impl std::ops::Add for Pull {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Pull {
            red: self.red + rhs.red,
            green: self.green + rhs.green,
            blue: self.blue + rhs.blue,
        }
    }
}
impl std::ops::BitAnd for Pull {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self::Output {
        Self {
            red: self.red.max(rhs.red),
            green: self.green.max(rhs.green),
            blue: self.blue.max(rhs.blue),
        }
    }
}
impl Sum for Pull {
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        let mut result = Default::default();
        for i in iter {
            result = result + i;
        }
        result
    }
}
impl FromStr for Pull {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut iter = s.trim().split_whitespace();
        let num = iter.next().unwrap().parse().unwrap();
        Ok(match iter.next().unwrap() {
            "blue" => Self {
                blue: num,
                ..Default::default()
            },
            "red" => Self {
                red: num,
                ..Default::default()
            },
            "green" => Self {
                green: num,
                ..Default::default()
            },
            color => panic!("unknown color {color}"),
        })
    }
}

fn main() {
    let result1: u32 = std::fs::read_to_string("input")
        .unwrap()
        .lines()
        .enumerate()
        .filter_map(|(idx, line)| {
            let (_, game) = line.split_at(line.find(":").unwrap());
            let possible = game[1..]
                .trim()
                .split("; ")
                .map(|pull| {
                    pull.split(", ")
                        .map(|pull| pull.parse::<Pull>().unwrap())
                        .sum::<Pull>()
                })
                .all(|pull| pull.red <= 12 && pull.green <= 13 && pull.blue <= 14);

            if possible {
                Some(idx as u32 + 1)
            } else {
                None
            }
        })
        .sum();

    println!("{result1}");

    let result2: u32 = std::fs::read_to_string("input")
        .unwrap()
        .lines()
        .map(|line| {
            let (_, game) = line.split_at(line.find(":").unwrap());
            let min_cubes = game[1..]
                .trim()
                .split("; ")
                .map(|pull| {
                    pull.split(", ")
                        .map(|pull| pull.parse::<Pull>().unwrap())
                        .reduce(|acc, pull| acc & pull)
                        .unwrap()
                })
                .reduce(|acc, pull| acc & pull)
                .unwrap();

            min_cubes.power()
        })
        .sum();

    println!("{result2}");
}
