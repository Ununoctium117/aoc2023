#[derive(Debug)]
struct History {
    derivatives: Vec<Vec<i64>>,
}
impl History {
    fn predict(&self) -> i64 {
        let mut predictions = vec![0i64; self.derivatives.len()];
        for i in (1..predictions.len()).rev() {
            predictions[i - 1] = self.derivatives[i - 1].last().unwrap() + predictions[i];
        }

        *predictions.first().unwrap()
    }

    fn predict_back(&self) -> i64 {
        let mut predictions = vec![0i64; self.derivatives.len()];
        for i in (1..predictions.len()).rev() {
            predictions[i - 1] = self.derivatives[i - 1].first().unwrap() - predictions[i];
        }

        *predictions.first().unwrap()
    }
}

fn main() {
    let (pred1, pred2): (Vec<_>, Vec<_>) = std::fs::read_to_string("input")
        .unwrap()
        .lines()
        .map(|line| {
            let values = line
                .split_ascii_whitespace()
                .map(|x| x.parse::<i64>().unwrap())
                .collect::<Vec<i64>>();
            let mut history = History {
                derivatives: vec![values],
            };

            while history.derivatives.last().unwrap().iter().any(|x| *x != 0) {
                history.derivatives.push(
                    history
                        .derivatives
                        .last()
                        .unwrap()
                        .windows(2)
                        .map(|x| x[1] - x[0])
                        .collect(),
                );
            }

            (history.predict(), history.predict_back())
        })
        .unzip();

    println!("{}", pred1.into_iter().sum::<i64>());
    println!("{}", pred2.into_iter().sum::<i64>());
}
