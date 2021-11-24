use std::env;

use rand::{Rng, thread_rng};

struct Lotto {
    take: usize,
    from: usize,
    numbers: Vec<usize>,
}

impl Lotto {
    fn new(take: usize, from: usize) -> Self {
        let mut rng = thread_rng();
        let mut numbers = vec![0; take];
        // sample without repetition
        let mut k = 0;
        while k < numbers.len() {
            let draw = rng.gen_range(1..=from);
            if numbers.contains(&draw) {
                continue; // re-draw
            }
            numbers[k] = draw;
            k += 1;
        }
        Lotto { take, from, numbers }
    }

    fn get_numbers(&self) -> Vec<usize> {
        self.numbers.clone()
    }
}

fn format_lotto_results(lotto: &Lotto) -> String {
    // Tip: Use the format macro
    format!("{} of {}: {:?}", lotto.take, lotto.from, &lotto.get_numbers())
}

fn main() {
    let args: Vec<String> = env::args().collect();
    // first argument (at position 0) is the program name
    if args.len() < 3 || args.len() % 2 == 0 {
        println!("invalid arguments!");
        println!("please specify how many numbers should be drawn and the maximum value per draw.");
        println!("usage: lotto TAKE FROM [TAKE FROM]...");
        return;
    }
    let mut results = Vec::new();
    for i in (1..args.len()).step_by(2) {
        let take = &args[i];
        let from = &args[i + 1];
        let take = take.parse().expect("error parsing 'take' argument");
        let from = from.parse().expect("error parsing 'from' argument");
        let result = Lotto::new(take, from);
        results.push(result);
    }
    for result in results {
        println!("{}", format_lotto_results(&result));
    }
}

#[test]
fn test_format_lotto_results() {
    let lotto = Lotto {
        take: 6,
        from: 45,
        numbers: vec![2, 3, 10, 25, 30, 40],
    };

    assert_eq!(
        "6 of 45: [2, 3, 10, 25, 30, 40]",
        format_lotto_results(&lotto)
    );
}

#[test]
fn test_lotto_constructor() {
    let lotto = Lotto::new(6, 45);

    let numbers = lotto.get_numbers();

    assert_eq!(numbers.len(), 6);
}

#[test]
fn test_lotto_constructor_uniques() {
    use std::collections::HashSet;
    let lotto = Lotto::new(6, 45);

    let numbers = lotto.get_numbers();
    let set: HashSet<usize> = numbers.into_iter().collect();

    assert_eq!(set.len(), 6);
}
