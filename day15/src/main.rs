use std::collections::HashMap;

fn hash(s: &[u8]) -> u32 {
    let mut hash = 0u32;
    for c in s {
        hash += *c as u32;
        hash *= 17;
        hash %= 256;
    }

    hash
}

#[test]
fn test_hash() {
    assert_eq!(52, hash("HASH".as_bytes()));
}

type FocalLength = u8;
type Label = Box<str>;
type LensBox = Vec<(Label, FocalLength)>;
type Boxes = HashMap<u32, LensBox>;

#[derive(Debug)]
enum Command {
    Remove(u32, Label),
    InsertOrReplace(u32, Label, FocalLength),
}
impl Command {
    fn apply(&self, boxes: &mut Boxes) {
        match self {
            Command::Remove(box_idx, label) => {
                if let Some(lens_box) = boxes.get_mut(box_idx) {
                    if let Some(idx) = lens_box.iter().position(|x| x.0 == *label) {
                        lens_box.remove(idx);
                    }
                }
            }
            Command::InsertOrReplace(box_idx, label, focal_length) => {
                let lens_box = boxes.entry(*box_idx).or_default();
                if let Some(idx) = lens_box.iter().position(|x| x.0 == *label) {
                    lens_box[idx] = (label.clone(), *focal_length);
                } else {
                    lens_box.push((label.clone(), *focal_length));
                }
            }
        }
    }
}

fn focusing_power(boxes: &Boxes) -> u32 {
    boxes
        .iter()
        .map(|(box_idx, lenses)| {
            lenses
                .iter()
                .enumerate()
                .map(|(lens_idx, (_, focal_length))| {
                    (*box_idx + 1) * ((lens_idx + 1) as u32) * (*focal_length as u32)
                })
                .sum::<u32>()
        })
        .sum()
}

fn main() {
    let res1 = std::fs::read_to_string("input")
        .unwrap()
        .replace('\n', "")
        .split(',')
        .map(|s| hash(s.as_bytes()))
        .sum::<u32>();

    println!("{res1}");

    let commands = std::fs::read_to_string("input")
        .unwrap()
        .replace('\n', "")
        .split(',')
        .map(|s| {
            if let Some(end_label) = s.find('=') {
                let label = s[0..end_label].to_string().into_boxed_str();
                let box_idx = hash(label.as_bytes());
                let (_, focal_length) = s.split_at(end_label + 1);

                Command::InsertOrReplace(box_idx, label, focal_length.parse().unwrap())
            } else {
                let label = s[0..s.len() - 1].to_string().into_boxed_str();
                let box_idx = hash(label.as_bytes());
                Command::Remove(box_idx, label)
            }
        })
        .collect::<Vec<_>>();

    let mut boxes = Boxes::default();
    for command in commands {
        command.apply(&mut boxes);
        // dbg!(command, &boxes);
    }

    println!("{}", focusing_power(&boxes));
}
