mod declarative_version;
mod iterator_version;
mod itertools_version;

fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;

    declarative_version::run();
    iterator_version::run();
    itertools_version::run();

    Ok(())
}
