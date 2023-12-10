use std::{marker::PhantomData, ops::Range};

use synonym::Synonym;

#[derive(Synonym)]
struct Seed(i64);
#[derive(Synonym)]
struct Soil(i64);
#[derive(Synonym)]
struct Fertilizer(i64);
#[derive(Synonym)]
struct Water(i64);
#[derive(Synonym)]
struct Light(i64);
#[derive(Synonym)]
struct Temperature(i64);
#[derive(Synonym)]
struct Humidity(i64);
#[derive(Synonym)]
struct Location(i64);

#[derive(Debug, Clone)]
struct MappingRange {
    src_start: i64,
    offset: i64,
    len: i64,
}

#[derive(Debug)]
struct Mapping<K, V> {
    // (source start, offset, length)
    ranges: Vec<MappingRange>,
    _phantom: PhantomData<(K, V)>,
}
impl<K, V> Mapping<K, V>
where
    K: AsRef<i64> + Clone + PartialEq,
    V: From<i64>,
{
    fn apply(&self, input: &K) -> V {
        let input = *input.as_ref();

        if let Some(range) = self.find_range(input) {
            (input + range.offset).into()
        } else {
            input.into()
        }
    }

    fn apply_ranges(&self, input: &[Range<K>]) -> Vec<Range<V>> {
        let mut result = Vec::new();

        for range in input {
            let mut start = *range.start.as_ref();
            let end = *range.end.as_ref();

            while start != end {
                if let Some(mapping) = self.find_range(start) {
                    // we're covered up to the end of this mapping
                    let end_of_cover = end.min(mapping.src_start + mapping.len);
                    result.push((start + mapping.offset)..(end_of_cover + mapping.offset));
                    start = end_of_cover;
                } else if let Some(next_mapping) = self.find_next_range(start) {
                    // we're not covered, up to the start of the next mapping
                    let end_of_uncover = end.min(next_mapping.src_start);
                    result.push(start..end_of_uncover);
                    start = end_of_uncover;
                } else {
                    // we're not covered and there are no mappings after the input
                    result.push(start..end);
                    start = end;
                }
            }
        }

        result.sort_by_key(|r| r.start);
        // could merge/simplify ranges here
        result.into_iter()
            .map(|range| range.start.into()..range.end.into())
            .collect()
    }

    fn find_range(&self, input: i64) -> Option<MappingRange> {
        for range in &self.ranges {
            if range.src_start <= input && range.src_start + range.len > input {
                return Some(range.clone());
            }
        }

        None
    }

    // finds the range, or the next range after the input
    fn find_next_range(&self, input: i64) -> Option<MappingRange> {
        for range in &self.ranges {
            if range.src_start + range.len > input {
                return Some(range.clone());
            }
        }

        None
    }
}

fn parse_map<K, V>(data: &str) -> Mapping<K, V> {
    let mut ranges: Vec<_> = data
        .lines()
        .skip(1)
        .map(|line| {
            let mut parts = line.split_ascii_whitespace();
            let dest_start: i64 = parts.next().unwrap().parse().unwrap();
            let src_start: i64 = parts.next().unwrap().parse().unwrap();
            let len = parts.next().unwrap().parse().unwrap();
            let offset = dest_start - src_start;
            MappingRange {
                src_start,
                offset,
                len,
            }
        })
        .collect();

    ranges.sort_by_key(|m| m.src_start);

    Mapping {
        ranges,
        _phantom: PhantomData,
    }
}

fn main() {
    let input = std::fs::read_to_string("input").unwrap();
    let mut data = input.split("\n\n");

    let seeds = data.next().unwrap();
    let (_, seeds) = seeds.split_at(seeds.find(":").unwrap());
    let seeds_p1 = seeds[1..]
        .trim()
        .split_ascii_whitespace()
        .map(|x| Seed(x.parse().unwrap()));

    let seed_ranges = seeds[1..]
        .trim()
        .split_ascii_whitespace()
        .collect::<Vec<_>>()
        .chunks_exact(2)
        .map(|chunk| {
            let start: i64 = chunk[0].parse().unwrap();
            let len: i64 = chunk[1].parse().unwrap();
            Seed(start)..Seed(start + len)
        })
        .collect::<Vec<_>>();

    let seed_to_soil = parse_map::<Seed, Soil>(data.next().unwrap());
    let soil_to_fertilizer = parse_map::<Soil, Fertilizer>(data.next().unwrap());
    let fertilizer_to_water = parse_map::<Fertilizer, Water>(data.next().unwrap());
    let water_to_light = parse_map::<Water, Light>(data.next().unwrap());
    let light_to_temp = parse_map::<Light, Temperature>(data.next().unwrap());
    let temp_to_humidity = parse_map::<Temperature, Humidity>(data.next().unwrap());
    let humidity_to_location = parse_map::<Humidity, Location>(data.next().unwrap());

    let result1 = seeds_p1
        .map(|seed| {
            humidity_to_location.apply(
                &temp_to_humidity.apply(
                    &light_to_temp.apply(
                        &water_to_light.apply(
                            &fertilizer_to_water
                                .apply(&soil_to_fertilizer.apply(&seed_to_soil.apply(&seed))),
                        ),
                    ),
                ),
            )
        })
        .min()
        .unwrap();

    println!("{result1:?}");

    let location_ranges = humidity_to_location.apply_ranges(&temp_to_humidity.apply_ranges(
        &light_to_temp.apply_ranges(&water_to_light.apply_ranges(
            &fertilizer_to_water.apply_ranges(
                &soil_to_fertilizer.apply_ranges(&seed_to_soil.apply_ranges(&seed_ranges)),
            ),
        )),
    ));
    dbg!(location_ranges.len());
    println!("{:?}", location_ranges[0].start);
}
