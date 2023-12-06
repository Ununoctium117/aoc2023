fn p2() {
    let input = std::fs::read_to_string("input").unwrap();
    let mut data = input.lines();
    let time = data
        .next()
        .unwrap()
        .split(":")
        .nth(1)
        .unwrap()
        .trim()
        .split_ascii_whitespace()
        .collect::<String>()
        .parse::<i64>()
        .unwrap();

    let distance = data
        .next()
        .unwrap()
        .split(":")
        .nth(1)
        .unwrap()
        .trim()
        .split_ascii_whitespace()
        .collect::<String>()
        .parse::<i64>()
        .unwrap();

    let result: i64 = std::iter::once(time)
        .zip(std::iter::once(distance))
        .map(|(max_time, max_distance)| {
            let mut num_ways = 0;
            for time_charged in 1..max_time {
                let time_remaining = max_time - time_charged;
                let speed = time_charged;
                let total_distance = speed * time_remaining;
                if total_distance > max_distance {
                    num_ways += 1;
                }
            }

            num_ways
        })
        .product();

    println!("{result}");
}

fn p1() {
    let input = std::fs::read_to_string("input").unwrap();
    let mut data = input.lines();
    let times = data
        .next()
        .unwrap()
        .split(":")
        .nth(1)
        .unwrap()
        .trim()
        .split_ascii_whitespace()
        .map(|x| x.parse::<i64>().unwrap());

    let distances = data
        .next()
        .unwrap()
        .split(":")
        .nth(1)
        .unwrap()
        .trim()
        .split_ascii_whitespace()
        .map(|x| x.parse::<i64>().unwrap());

    let result1: i64 = times
        .zip(distances)
        .map(|(max_time, max_distance)| {
            let mut num_ways = 0;
            for time_charged in 1..max_time {
                let time_remaining = max_time - time_charged;
                let speed = time_charged;
                let total_distance = speed * time_remaining;
                if total_distance > max_distance {
                    num_ways += 1;
                }
            }

            num_ways
        })
        .product();

    println!("{result1}");
}

fn main() {
    p1();
    p2();
}
