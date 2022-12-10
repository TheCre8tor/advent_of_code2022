mod iterator_version;

fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;

    // On windows operating systems "\n" is "\r\n"
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

    iterator_version::run();

    Ok(())
}
