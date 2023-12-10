fn main() {
    let result1: u32 = std::fs::read_to_string("input")
        .unwrap()
        .lines()
        .map(|line| {
            let digits = line
                .chars()
                .filter_map(|c| c.to_digit(10))
                .collect::<Vec<_>>();
            (digits.first().unwrap() * 10) + digits.last().unwrap()
        })
        .sum();

    println!("{result1}");

    let result2: u32 = std::fs::read_to_string("input")
        .unwrap()
        .lines()
        .map(|line| {
            let mut first_digit = None;
            for substr_end in 1..line.len() + 1 {
                let slice = &line[..substr_end];
                if let Some(first) = slice
                    .replace("one", "1")
                    .replace("two", "2")
                    .replace("three", "3")
                    .replace("four", "4")
                    .replace("five", "5")
                    .replace("six", "6")
                    .replace("seven", "7")
                    .replace("eight", "8")
                    .replace("nine", "9")
                    .chars()
                    .filter_map(|c| c.to_digit(10))
                    .next()
                {
                    first_digit = Some(first);
                    break;
                }
            }

            let mut last_digit = None;
            for substr_start in (0..line.len()).rev() {
                let slice = &line[substr_start..];
                if let Some(last) = slice
                    .replace("one", "1")
                    .replace("two", "2")
                    .replace("three", "3")
                    .replace("four", "4")
                    .replace("five", "5")
                    .replace("six", "6")
                    .replace("seven", "7")
                    .replace("eight", "8")
                    .replace("nine", "9")
                    .chars()
                    .filter_map(|c| c.to_digit(10))
                    .next()
                {
                    last_digit = Some(last);
                    break;
                }
            }

            (first_digit.unwrap() * 10) + last_digit.unwrap()
        })
        .sum();

    println!("{result2}");
}
