struct GroupSumIter<I> {
    inner: I,
}

impl<I> Iterator for GroupSumIter<I>
where
    I: Iterator<Item = Option<u64>>,
{
    type Item = u64;

    fn next(&mut self) -> Option<Self::Item> {
        let mut sum = loop {
            match self.inner.next() {
                Some(Some(v)) => break v,
                Some(None) => {
                    // huh, weird, didn't expect a separator there
                    // but let's just skip it
                }
                // we've reached the end of the inner iterator
                None => return None,
            }
        };

        loop {
            match self.inner.next() {
                Some(Some(v)) => sum += v,
                Some(None) | None => {
                    // reached a separator or the end of the iterator
                    break Some(sum);
                }
            }
        }
    }
}

pub fn run() {
    let lines = include_str!("input.txt").lines();
    let iter_lines = lines.map(|value| value.parse::<u64>().ok());

    // here! 👋
    let elven_lead = GroupSumIter { inner: iter_lines }.max();

    println!("{elven_lead:?}");
}
