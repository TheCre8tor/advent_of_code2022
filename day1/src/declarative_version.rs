pub fn run() {
    let file = include_str!("input.txt");
    let lines = file
        .lines()
        .map(|v| v.parse::<u64>().ok())
        .collect::<Vec<_>>();

    let elven_lead = lines
        .split(|line| line.is_none())
        .map(|group| group.iter().map(|v| v.unwrap()).sum::<u64>())
        .max();

    println!("{elven_lead:?}");
}
