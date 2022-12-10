use itertools::Itertools;

pub fn run() {
    let max = include_str!("input.txt")
        .lines()
        .map(|value| value.parse::<u64>().ok())
        .batching(|it| {
            let mut sum = None;

            while let Some(Some(value)) = it.next() {
                sum = Some(sum.unwrap_or(0) + value);
            }

            sum
        })
        .max();

    println!("{max:?}");
}
